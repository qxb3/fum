use std::sync::Arc;

use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use tokio::sync::Mutex;

use crate::{
    mpris::{Mpris, MprisEvent, PlayerEvent},
    state::State,
    track::Track,
    FumResult,
};

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
pub struct MprisMode<'a> {
    /// Mpris D-Bus connection.
    mpris: Arc<Mpris<'static>>,

    /// Picker for image rendering.
    picker: Arc<Picker>,

    /// A reference to the application state.
    state: &'a mut State,
}

impl<'a> MprisMode<'a> {
    /// Creates new MprisMode.
    pub async fn new(state: &'a mut State) -> FumResult<Self> {
        let mpris = Arc::new(Mpris::new().await?);
        let picker = Arc::new(Picker::from_query_stdio()?);

        Ok(Self {
            mpris,
            picker,
            state,
        })
    }

    /// Handle mpris mode.
    pub async fn handle(&mut self) -> FumResult<()> {
        let mpris = Arc::clone(&self.mpris);
        let picker = Arc::clone(&self.picker);

        let current_player = Arc::clone(&self.state.current_player);
        let current_track = Arc::clone(&self.state.current_track);
        let current_cover = Arc::clone(&self.state.current_cover);

        // A specific channel for broadcasting to the current player thread that its
        // detached and to stop watching for events.
        let (detached_tx, _) = tokio::sync::broadcast::channel::<String>(10);

        tokio::spawn(async move {
            // Channel for mpris events.
            let (mpris_tx, mut mpris_rx) = tokio::sync::mpsc::channel(10);

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

                        let current_player = Arc::clone(&current_player);
                        let current_track = Arc::clone(&current_track);
                        let current_cover = Arc::clone(&current_cover);

                        // We update the current_player to the player
                        {
                            let mut current_player = current_player.lock().await;
                            *current_player = Some(player);
                        }

                        // Update the track metadata as soon as the player attached.
                        {
                            let current_player = current_player.lock().await;
                            let current_player = current_player
                                .as_ref()
                                .expect("Tried to update track metadata for the player but current player is None somehow");

                            // Creates a track metadata of player.
                            let track = Track::from_player(current_player).await.expect(
                                &format!(
                                    "Failed to create track for: {}",
                                    current_player.bus_name
                                ),
                            );

                            // Update current cover.
                            if let Some(art_url) = &track.art_url {
                                MprisMode::update_cover(
                                    art_url.to_string(),
                                    picker.clone(),
                                    current_cover.clone(),
                                );
                            }

                            // Update the track metadata.
                            let mut current_track = current_track.lock().await;
                            *current_track = track;
                        }

                        tokio::spawn(async move {
                            // Channel for player events.
                            let (player_tx, mut player_rx) =
                                tokio::sync::mpsc::channel::<PlayerEvent>(10);

                            // For watching the events of current player.
                            {
                                let current_player = current_player.lock().await;
                                let current_player = current_player
                                    .as_ref()
                                    .expect("Tried to watch event for player but current player is somehow None");

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
                                        let mut current_player = current_player.lock().await;
                                        let mut current_cover = current_cover.lock().await;
                                        let curr_player = current_player
                                            .as_ref()
                                            .expect("Tried to reset the track metadata for the player but current player is None somehow");

                                        if bus_name == curr_player.bus_name {
                                            // Set the current player to None.
                                            *current_player = None;

                                            // Resets the current track metadata to their default values.
                                            let mut current_track = current_track.lock().await;
                                            let track = Track::default();
                                            *current_track = track;

                                            // Set the current cover to None.
                                            *current_cover = None;

                                            break;
                                        }
                                    },

                                    // Receive player events.
                                    Some(event) = player_rx.recv() => {
                                        match event {
                                            // Update the track metadata when the player properties changed.
                                            PlayerEvent::PropertiesChanged => {
                                                let current_player = current_player.lock().await;
                                                let current_player = current_player
                                                    .as_ref()
                                                    .expect("Tried to update track metadata for the player but current player is None somehow");

                                                // Creates a track metadata of player.
                                                let track = Track::from_player(current_player)
                                                    .await
                                                    .expect(&format!("Failed to create track for: {}", current_player.bus_name));

                                                let mut current_track = current_track.lock().await;

                                                // If the new art_url doesn't match the current art url means that its been changed,
                                                if let Some(current_art_url) = &current_track.art_url {
                                                    if let Some(track_art_url) = &track.art_url {
                                                        if current_art_url != track_art_url {
                                                            // Update current cover.
                                                            MprisMode::update_cover(
                                                                track_art_url.to_string(),
                                                                picker.clone(),
                                                                current_cover.clone()
                                                            );
                                                        }
                                                    }
                                                }

                                                // Update the track metadata.
                                                *current_track = track;
                                            },

                                            // Update the position the current track when seeked.
                                            PlayerEvent::Seeked => {
                                                let current_player = current_player.lock().await;
                                                let current_player = current_player
                                                    .as_ref()
                                                    .expect("Tried to update track position for the player but current player is None somehow");

                                                // Get the updated recent position.
                                                let position = current_player
                                                    .position()
                                                    .await
                                                    .expect(&format!("Failed to get the player position for: {}", current_player.bus_name));

                                                let mut current_track = current_track.lock().await;
                                                current_track.position = position;
                                            },

                                            // Update the position the current track when the the track position progress.
                                            PlayerEvent::Position(position) => {
                                                let mut current_track = current_track.lock().await;
                                                current_track.position = position;
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

    /// Updates the current cover.
    fn update_cover(
        art_url: String,
        picker: Arc<Picker>,
        current_cover: Arc<Mutex<Option<StatefulProtocol>>>,
    ) {
        tokio::spawn(async move {
            // Request to get the cover image.
            let client = reqwest::Client::new();
            let response = client
                .get(art_url)
                .header(reqwest::header::RANGE, "bytes=0-1048576") // Only fetch 1mb of bytes for perf reason.
                .send()
                .await
                .expect("Failed to fetch cover art");

            // Get bytes of cover image.
            let bytes = response
                .bytes()
                .await
                .expect("Failed to get cover art image bytes");

            // Decode cover image to image.
            let cover = image::ImageReader::new(std::io::Cursor::new(bytes))
                .with_guessed_format()
                .expect("Unknown cover art image file type")
                .decode()
                .expect("Failed to decode cover art image");

            // Creates a image StatefulProtocol.
            let protocol = picker.new_resize_protocol(cover);

            // Updates the current cover.
            let mut current_cover = current_cover.lock().await;
            *current_cover = Some(protocol);
        });
    }
}
