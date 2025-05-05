use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use tokio::sync::Mutex;

use crate::{
    event::{Event, EventManager, EventSender, MprisEvent, ScriptEvent},
    state::State,
    track::Track,
    FumResult,
};

/// Manages the mpris events.
pub struct Mpris {
    /// Internal mprizzle Mpris.
    mpris: Arc<Mutex<mprizzle::Mpris>>,

    shared_players: Option<Arc<Mutex<HashMap<mprizzle::PlayerIdentity, mprizzle::MprisPlayer>>>>,

    /// The current player identity.
    current_player_identity: Option<mprizzle::PlayerIdentity>,

    /// The centralize event manager sender.
    event_sender: EventSender,
}

impl Mpris {
    pub async fn new(
        event_manager: &EventManager,
        filter_players: Vec<&'static str>,
    ) -> FumResult<Self> {
        let mpris = mprizzle::Mpris::new(Some(mprizzle::MprisOptions { filter_players })).await?;

        Ok(Self {
            mpris: Arc::new(Mutex::new(mpris)),
            current_player_identity: None,
            shared_players: None,
            event_sender: event_manager.sender(),
        })
    }

    /// Sends events into the centalized event thingy.
    pub async fn send_events(&mut self) {
        let mpris = Arc::clone(&self.mpris);
        let event_sender = self.event_sender.clone();

        {
            let mpris = mpris.lock().await;
            self.shared_players = Some(mpris.players());
        }

        tokio::spawn(async move {
            let mut mpris = mpris.lock().await;
            mpris.watch();

            loop {
                tokio::select! {
                    // Checks the results chronologically,
                    // So it first first check if the channels has been closed,
                    // Then the rest.
                    biased;

                    // Break out of this loop if event_sender has been closed
                    _ = event_sender.closed() => break,

                    mpris_event_res = mpris.recv() => {
                        let event = match mpris_event_res {
                            Ok(event) => event,
                            Err(err) => {
                                event_sender
                                    .send(Err(anyhow!("Failed to receive mpris event: {err}")))
                                    .unwrap();

                                return;
                            }
                        };

                        // Just sents out the mpris events to the centralized event.
                        match event {
                            Ok(event) => match event {
                                mprizzle::MprisEvent::PlayerAttached(identity) => {
                                    event_sender.send(Ok(Event::Mpris(MprisEvent::PlayerAttached(identity)))).unwrap();
                                }
                                mprizzle::MprisEvent::PlayerDetached(identity) => {
                                    event_sender.send(Ok(Event::Mpris(MprisEvent::PlayerDetached(identity)))).unwrap();
                                }
                                mprizzle::MprisEvent::PlayerPropertiesChanged(identity) => {
                                    event_sender.send(Ok(Event::Mpris(MprisEvent::PlayerPropertiesChanged(identity)))).unwrap();
                                }
                                mprizzle::MprisEvent::PlayerSeeked(identity) => {
                                    event_sender.send(Ok(Event::Mpris(MprisEvent::PlayerSeeked(identity)))).unwrap();
                                }
                                mprizzle::MprisEvent::PlayerPosition(identity, position) => {
                                    event_sender.send(Ok(Event::Mpris(MprisEvent::PlayerPosition(identity, position)))).unwrap();
                                }
                            },
                            Err(err) => {
                                event_sender
                                    .send(Err(anyhow!("Received a error event in mpris event: {err}")))
                                    .unwrap();

                                return;
                            }
                        }
                    }
                }
            }
        });
    }

    /// Handle the mpris events.
    pub async fn handle(&mut self, state: &mut State, event: MprisEvent) -> FumResult<()> {
        match event {
            MprisEvent::PlayerAttached(identity) => {
                // When a new player has been attached and the current player is still None, make it Some! :D
                if self.current_player_identity.is_none() {
                    self.current_player_identity = Some(identity);
                }
            }
            MprisEvent::PlayerDetached(identity) => {
                // If the current player matches the detached player,
                // Try to set the existing player, If not just make it None.
                if let Some(current_player_identity) = &self.current_player_identity {
                    let players = self.shared_players.as_ref().unwrap().try_lock().unwrap();

                    if current_player_identity == &identity {
                        // Set the current player to the next existing player if there is one.
                        if let Some((next_player_identity, _)) = players.iter().next() {
                            self.current_player_identity = Some(next_player_identity.clone());
                        } else {
                            self.current_player_identity = None;
                        }
                    }
                }
            }
            MprisEvent::PlayerPropertiesChanged(identity) => {
                // If the current player matches the the player that properties has been changed,
                // Updates the state's current track.
                if let Some(current_player_identity) = &self.current_player_identity {
                    if current_player_identity == &identity {
                        let players = self.shared_players.as_ref().unwrap().try_lock().unwrap();

                        // Updates the state's current track.
                        if let Some(current_player) = players.get(current_player_identity) {
                            let track = Track::from_mpris_player(current_player).await?;
                            state.set_current_track(track);
                        }
                    }
                }
            }
            MprisEvent::PlayerSeeked(identity) => {
                // If the current player matches the player that has been seeked,
                // Updates the state's current track position.
                if let Some(current_player_identity) = &self.current_player_identity {
                    if current_player_identity == &identity {
                        let players = self.shared_players.as_ref().unwrap().try_lock().unwrap();

                        if let Some(_current_player) = players.get(current_player_identity) {
                            let _current_track = state.get_current_track_mut();
                            // TODO: update position
                        }
                    }
                }
            }
            MprisEvent::PlayerPosition(identity, position) => {
                // If the current player matches the player position has been changed,
                // Updates the state's current track position.
                if let Some(current_player_identity) = &self.current_player_identity {
                    if current_player_identity == &identity {
                        let current_track = state.get_current_track_mut();
                        current_track.position = position;
                    }
                }
            }
        }

        // Sends out a TrackUpdated event to the script everytime we receive a mpris event.
        self.event_sender
            .send(Ok(Event::Script(ScriptEvent::TrackUpdated)))
            .unwrap();

        Ok(())
    }
}
