use std::sync::Arc;

use crate::{
    mpris::{Mpris, MprisEvent, PlayerEvent}, state::State, track::Track, FumResult
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

    /// A reference to the application state.
    state: &'a mut State,
}

impl<'a> MprisMode<'a> {
    /// Creates new MprisMode.
    pub async fn new(state: &'a mut State) -> FumResult<Self> {
        let mpris = Arc::new(Mpris::new().await?);

        Ok(Self {
            mpris,
            state,
        })
    }

    /// Handle mpris mode.
    pub async fn handle(&mut self) -> FumResult<()> {
        let mpris = Arc::clone(&self.mpris);

        let current_player = Arc::clone(&self.state.current_player);
        let current_track = Arc::clone(&self.state.current_track);

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

                        let current_player = Arc::clone(&current_player);
                        let current_track = Arc::clone(&current_track);

                        // Update the track metadata as soon as the player attached.
                        {
                            // Creates a track metadata of player.
                            let track = player
                                .track()
                                .await
                                .expect(&format!("Failed to create track for: {}", &player.bus_name));

                            // Update the track metadata.
                            let mut current_track = current_track.lock().await;
                            *current_track = track;
                        }

                        tokio::spawn(async move {
                            // Channel for player events.
                            let (player_tx, mut player_rx) =
                                tokio::sync::mpsc::channel::<PlayerEvent>(10);

                            // Watch player events.
                            player.watch(player_tx.clone()).await.expect(&format!(
                                "Failed to watch player: {} events",
                                &player.bus_name
                            ));

                            loop {
                                tokio::select! {
                                    // If received an detached event and if the bus_name matched to the player
                                    // then we break out of this loop.
                                    Ok(bus_name) = detached_rx.recv() => {
                                        if bus_name == player.bus_name {
                                            // Resets the current_track metadata to their default values.
                                            let mut current_track = current_track.lock().await;

                                            let track = Track::default();
                                            *current_track = track;

                                            break;
                                        }
                                    },

                                    // Receive player events.
                                    Some(event) = player_rx.recv() => {
                                        match event {
                                            // Update the track metadata when the player properties changed.
                                            PlayerEvent::PropertiesChanged => {
                                                // Creates a track metadata of player.
                                                let track = player
                                                    .track()
                                                    .await
                                                    .expect(&format!("Failed to create track for: {}", &player.bus_name));

                                                // Update the track metadata.
                                                let mut current_track = current_track.lock().await;
                                                *current_track = track;
                                            },

                                            // Update the position the current track when seeked.
                                            PlayerEvent::Seeked => {
                                                let mut current_track = current_track.lock().await;

                                                // Get the updated recent position.
                                                let position = player
                                                    .position()
                                                    .await
                                                    .expect(&format!("Failed to get the player position for: {}", &player.bus_name));

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

                            // We update the current_player to the player
                            // in-case we needed it somewhere like rendering the player name or something.
                            let mut current_player = current_player.lock().await;
                            *current_player = Some(player);
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
}
