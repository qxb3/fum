use std::sync::Arc;

use crate::{
    mpris::{Mpris, MprisEvent, PlayerEvent},
    state::State,
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
                                            // Update the current_track metadata to default / none values.
                                            let mut current_track = current_track.lock().await;
                                            current_track.track_id = None;
                                            current_track.title = "No Music".into();

                                            break;
                                        }
                                    },

                                    // Receive player events.
                                    Some(event) = player_rx.recv() => {
                                        match event {
                                            // Update the track metadata when the player properties changed.
                                            PlayerEvent::PropertiesChanged => {
                                                let metadata = player.metadata()
                                                    .await
                                                    .expect(&format!("Failed to get metadata of: {}", &player.bus_name));

                                                let trackid = metadata
                                                    .trackid()
                                                    .expect(&format!("Failed to get the trackid of: {}", &player.bus_name));

                                                let title = metadata
                                                    .title()
                                                    .expect(&format!("Failed to get the title of: {}", &player.bus_name))
                                                    .unwrap_or("No Music".into());

                                                // Update current_track.
                                                let mut current_track = current_track.lock().await;
                                                current_track.track_id = trackid;
                                                current_track.title = title;
                                            },

                                            _ => {}
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
