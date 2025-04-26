use std::{fs, str::FromStr, sync::Arc};

use base64::{prelude::BASE64_STANDARD, Engine};
use ratatui_image::picker::Picker;
use reqwest::Url;

use crate::{
    cover::Cover,
    mpris::{Mpris, MprisEvent, MprisPlayer, MprisPlayerEvent},
    player::Player,
    state::FumState,
    track::Track,
    FumResult,
};

use super::{FumMode, FumModeEvent};

/// Mpris mode events.
#[derive(Debug)]
pub enum MprisModeEvent {
    /// Triggers when the general track metadata changed.
    PlayerTrackMetaChanged,

    /// Triggers when the player position changes. This includes the seeked event.
    PlayerPositionChanged,

    /// Triggers when the cover changes.
    CoverChanged,
}

/// MprisMode.
///
/// How will this works is that, modes will have their own whatever stuff
/// they need to play around with and a reference to the application state.
/// The main work that a mode needs to do is to mutate the necessary state
/// like the current_track & current_player. and the ui will just pull the
/// states out for rendering.
///
/// Fum Starts -> Figure out which mode -> Runs that mode
/// -> Mode do async stuff to mutate the state. -> Ui render those state.
pub struct MprisMode {
    /// Mpris D-Bus connection.
    mpris: Arc<Mpris<'static>>,

    /// Picker for image rendering.
    picker: Arc<Picker>,

    /// Handle to the shared state.
    state: FumState,

    /// Sender for mpris mode event.
    sender: tokio::sync::mpsc::Sender<FumModeEvent>,

    /// Receiver for mpris mode event.
    receiver: tokio::sync::mpsc::Receiver<FumModeEvent>,
}

impl MprisMode {
    /// Creates new MprisMode.
    pub async fn new(state: FumState) -> FumResult<Self> {
        let mpris = Arc::new(Mpris::new().await?);
        let picker = Arc::new(Picker::from_query_stdio()?);

        let (sender, receiver) = tokio::sync::mpsc::channel(20);

        Ok(Self {
            mpris,
            picker,
            state,
            sender,
            receiver,
        })
    }
}

#[async_trait::async_trait]
impl FumMode for MprisMode {
    async fn start(&mut self) -> FumResult<()> {
        let mpris = Arc::clone(&self.mpris);
        let picker = Arc::clone(&self.picker);
        let mode_tx = self.sender.clone();

        let state = Arc::clone(&self.state);

        // A specific channel for broadcasting to the current player thread that its
        // detached and to stop watching for events.
        let (detached_tx, _) = tokio::sync::broadcast::channel::<String>(20);

        tokio::spawn(async move {
            // Channel for mpris events.
            let (mpris_tx, mut mpris_rx) = tokio::sync::mpsc::channel(20);

            // Watch mpris events.
            mpris
                .watch(mpris_tx.clone())
                .await
                .expect("Failed to watch mpris events");

            while let Some(event) = mpris_rx.recv().await {
                match event {
                    MprisEvent::PlayerAttached(player) => {
                        let mut detached_rx = detached_tx.subscribe();
                        let picker = Arc::clone(&picker);
                        let mode_tx = mode_tx.clone();

                        let shared_state = Arc::clone(&state);

                        // We update the current_player
                        {
                            let mut state = shared_state.lock().await;
                            state.set_player(Box::new(player));
                        }

                        // Update the track metadata as soon as the player attached.
                        {
                            let mut state = shared_state.lock().await;
                            let current_player = downcast_player(state.get_player());

                            // Creates a track metadata of player.
                            let track = Track::from_mpris_player(current_player)
                                .await
                                .expect(&format!(
                                    "Failed to create track for: {}",
                                    current_player.bus_name
                                ));

                            // Update current cover.
                            let current_cover = state.get_cover_mut();
                            if let Some(art_url) = &track.art_url {
                                MprisMode::update_cover(
                                    art_url.to_string(),
                                    picker.clone(),
                                    Arc::clone(&shared_state),
                                    mode_tx.clone(),
                                );
                            }

                            // Update the track metadata.
                            state.set_track(track);

                            // Sends out both the PlayerTrackMetaChanged & PlayerPositionChanged event.
                            mode_tx
                                .send(FumModeEvent::MprisEvent(
                                    MprisModeEvent::PlayerTrackMetaChanged,
                                ))
                                .await
                                .unwrap();

                            mode_tx
                                .send(FumModeEvent::MprisEvent(
                                    MprisModeEvent::PlayerPositionChanged,
                                ))
                                .await
                                .unwrap();
                        }

                        tokio::spawn(async move {
                            // Channel for player events.
                            let (player_tx, mut player_rx) =
                                tokio::sync::mpsc::channel::<MprisPlayerEvent>(20);

                            // For watching the events of current player.
                            {
                                let state = shared_state.lock().await;
                                let current_player = downcast_player(state.get_player());

                                // Watch player events.
                                current_player.watch(player_tx.clone()).await.expect(
                                    &format!(
                                        "Failed to watch player: {} events",
                                        current_player.bus_name
                                    ),
                                );
                            }

                            loop {
                                tokio::select! {
                                    // If received an detached event and if the bus_name matched to the player
                                    // then we break out of this loop.
                                    Ok(bus_name) = detached_rx.recv() => {
                                        let mut state = shared_state.lock().await;
                                        let current_player = downcast_player(state.get_player());

                                        if bus_name == current_player.bus_name {
                                            // Remove the current player.
                                            state.clear_player();

                                            // Resets the current track metadata to their default values.
                                            state.set_track(Track::default());

                                            // Remove the current cover.
                                            state.clear_cover();

                                            // Sends out both the PlayerTrackMetaChanged & PlayerPositionChanged event.
                                            mode_tx
                                                .send(FumModeEvent::MprisEvent(MprisModeEvent::PlayerTrackMetaChanged))
                                                .await
                                                .unwrap();

                                            mode_tx
                                                .send(FumModeEvent::MprisEvent(MprisModeEvent::PlayerPositionChanged))
                                                .await
                                                .unwrap();

                                            break;
                                        }
                                    },

                                    // Receive player events.
                                    Some(event) = player_rx.recv() => {
                                        match event {
                                            // Update the track metadata when the player properties changed.
                                            MprisPlayerEvent::PropertiesChanged => {
                                                let mut state = shared_state.lock().await;
                                                let current_player = downcast_player(state.get_player());
                                                let current_track = state.get_track();

                                                // Creates a track metadata of player.
                                                let track = Track::from_mpris_player(current_player)
                                                    .await
                                                    .expect(&format!("Failed to create track for: {}", current_player.bus_name));

                                                // If the new art_url doesn't match the current art url means that its been changed,
                                                if let Some(current_art_url) = &current_track.art_url {
                                                    if let Some(track_art_url) = &track.art_url {
                                                        if current_art_url != track_art_url {
                                                            // Update current cover.
                                                            MprisMode::update_cover(
                                                                track_art_url.to_string(),
                                                                picker.clone(),
                                                                Arc::clone(&shared_state),
                                                                mode_tx.clone()
                                                            );
                                                        }
                                                    }
                                                }

                                                // Update the track state.
                                                state.set_track(track);

                                                // Sends out the PlayerTrackMetaChanged event.
                                                mode_tx
                                                    .send(FumModeEvent::MprisEvent(MprisModeEvent::PlayerTrackMetaChanged))
                                                    .await
                                                    .unwrap();
                                            },

                                            // Update the position the current track when seeked.
                                            MprisPlayerEvent::Seeked => {
                                                let mut state = shared_state.lock().await;
                                                let current_player = downcast_player(state.get_player());

                                                // Get the updated recent position.
                                                let position = current_player
                                                    .position()
                                                    .await
                                                    .expect(&format!("Failed to get the player position for: {}", current_player.bus_name));

                                                // Updates the current track position.
                                                let current_track = state.get_track_mut();
                                                current_track.position = position;

                                                // Sends out the PlayerPositionChanged event.
                                                mode_tx
                                                    .send(FumModeEvent::MprisEvent(MprisModeEvent::PlayerPositionChanged))
                                                    .await
                                                    .unwrap();
                                            },

                                            // Update the position the current track when the the track position progress.
                                            MprisPlayerEvent::Position(position) => {
                                                let mut state = shared_state.lock().await;
                                                let current_track = state.get_track_mut();

                                                current_track.position = position;

                                                // Sends out the PlayerPositionChanged event.
                                                mode_tx
                                                    .send(FumModeEvent::MprisEvent(MprisModeEvent::PlayerPositionChanged))
                                                    .await
                                                    .unwrap();
                                            },
                                        }
                                    }
                                }
                            }
                        });
                    }

                    // Send the event along with bus_name to the detached channel.
                    MprisEvent::PlayerDetached(bus_name) => {
                        detached_tx
                            .send(bus_name)
                            .expect("Failed to send detached event to player");
                    }
                }
            }
        });

        Ok(())
    }

    async fn recv(&mut self) -> FumResult<FumModeEvent> {
        self.receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to receive an event on MprisMode",
            )))
    }
}

impl MprisMode {
    /// Updates the current cover.
    fn update_cover(
        art_url: String,
        picker: Arc<Picker>,
        state: FumState,
        mpris_tx: tokio::sync::mpsc::Sender<FumModeEvent>,
    ) {
        tokio::spawn(async move {
            // Handle if art url is file:// scheme.
            if art_url.starts_with("file://") {
                // Parse the path of the url.
                let path = Url::from_str(&art_url)
                    .expect("Failed to parse cover art url")
                    .to_file_path()
                    .expect("Failed to covert cover art url to Path");

                // Get bytes of cover image.
                let bytes = fs::read(&path).expect("Failed to get cover art image bytes");

                // Creates the cover.
                let cover = Cover::new(bytes, &*picker);

                // Updates the current cover.
                let mut state = state.lock().await;
                state.set_cover(cover);

                // Sends the CoverChanged event.
                mpris_tx
                    .send(FumModeEvent::MprisEvent(MprisModeEvent::CoverChanged))
                    .await
                    .unwrap();
            }
            // Handle if the art url is base64.
            else if art_url.starts_with("data:") {
                // Only get the data of the image.
                let image_data = art_url
                    .split_once("base64,")
                    .expect("Invalid base64 cover url format")
                    .1;

                // Get bytes of base64 cover art.
                let bytes = BASE64_STANDARD
                    .decode(image_data)
                    .expect("Failed to get cover art image bytes");

                // Creates the cover.
                let cover = Cover::new(bytes, &*picker);

                // Updates the current cover.
                let mut state = state.lock().await;
                state.set_cover(cover);

                // Sends the CoverChanged event.
                mpris_tx
                    .send(FumModeEvent::MprisEvent(MprisModeEvent::CoverChanged))
                    .await
                    .unwrap();
            }
            // Handle normal cover art url.
            else if art_url.starts_with("http://") || art_url.starts_with("https://") {
                // Request to get the cover image.
                let client = reqwest::Client::new();
                let response = client
                    .get(art_url)
                    // Only fetch 1mb of bytes for perf reason (its being rendered in the terminal u wouldn't know).
                    .header(reqwest::header::RANGE, "bytes=0-1048576")
                    .send()
                    .await
                    .expect("Failed to fetch cover art");

                // Get bytes of cover image.
                let bytes = response
                    .bytes()
                    .await
                    .expect("Failed to get cover art image bytes");

                // Creates the cover.
                let cover = Cover::new(bytes.to_vec(), &*picker);

                // Updates the current cover.
                let mut state = state.lock().await;
                state.set_cover(cover);

                // Sends the CoverChanged event.
                mpris_tx
                    .send(FumModeEvent::MprisEvent(MprisModeEvent::CoverChanged))
                    .await
                    .unwrap();
            }
        });
    }
}

/// A helper function to downcast the current player to MprisPlayer.
fn downcast_player<'a>(player: Option<&'a (dyn Player)>) -> &'a MprisPlayer {
    player
        .as_ref()
        .expect("Tried to update track metadata for the player but current player is None somehow")
        .as_any()
        .downcast_ref::<MprisPlayer>()
        .expect("Expected an mpris player to be on the mpris mode but got a different player somehow")
}
