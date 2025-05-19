use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Context};
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

    /// All of the mpris players.
    players: HashMap<mprizzle::PlayerIdentity, mprizzle::MprisPlayer>,

    /// The current player identity to manage.
    current_player_id: Option<mprizzle::PlayerIdentity>,

    /// A list of player that should only be managed.
    filter_players: Vec<String>,

    /// The centralize event manager sender.
    event_sender: EventSender,
}

impl Mpris {
    pub async fn new(event_manager: &EventManager) -> FumResult<Self> {
        let mpris = mprizzle::Mpris::new().await?;

        Ok(Self {
            mpris: Arc::new(Mutex::new(mpris)),
            players: HashMap::new(),
            current_player_id: None,
            filter_players: Vec::new(),
            event_sender: event_manager.sender(),
        })
    }

    /// Sends events into the centalized event thingy.
    pub async fn send_events(&mut self) {
        let mpris = Arc::clone(&self.mpris);
        let event_sender = self.event_sender.clone();

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

                    // Receive mpris events.
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
                                mprizzle::MprisEvent::PlayerAttached(player) => {
                                    event_sender.send(Ok(Event::Mpris(MprisEvent::PlayerAttached(player)))).unwrap();
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
            // Insert the attached player into players.
            MprisEvent::PlayerAttached(attached_player) => {
                let attached_player_id = attached_player.identity().clone();

                if !self.check_is_filtered(&attached_player_id) {
                    return Ok(());
                }

                if self.current_player_id.is_none() {
                    self.current_player_id = Some(attached_player_id.clone());

                    self.event_sender
                        .send(Ok(Event::Mpris(MprisEvent::PlayerPropertiesChanged(attached_player_id.clone()))))
                        .unwrap();
                }

                self.players.insert(attached_player_id, attached_player);
            }

            // Removes the detached player into players.
            MprisEvent::PlayerDetached(detached_id) => {
                if let Some(detached_player) = self.players.remove(&detached_id) {
                    if let Some(current_player_id) = &self.current_player_id {
                        if current_player_id != detached_player.identity() {
                            return Ok(());
                        }

                        if let Some((next_player_id, _)) = self.players.iter().next() {
                            if !self.check_is_filtered(&next_player_id) {
                                return Ok(());
                            }

                            self.current_player_id = Some(next_player_id.clone());

                            self.event_sender
                                .send(Ok(Event::Mpris(MprisEvent::PlayerPropertiesChanged(next_player_id.clone()))))
                                .unwrap();
                        } else {
                            self.current_player_id = None;

                            let default_track = Track::default();
                            state.set_current_track(default_track);
                        }
                    }
                }
            }

            // Updates the current track entire metadata.
            MprisEvent::PlayerPropertiesChanged(props_changed_id) => {
                if let Some(current_player_id) = &self.current_player_id {
                    if *current_player_id != props_changed_id {
                        return Ok(());
                    }

                    if let Some(current_player) = self.players.get(current_player_id) {
                        let new_track = Track::from_mpris_player(current_player).await?;
                        state.set_current_track(new_track);
                    }
                }
            }

            // Updates the current track position.
            MprisEvent::PlayerSeeked(seeked_id) => {
                if let Some(current_player_id) = &self.current_player_id {
                    if *current_player_id != seeked_id {
                        return Ok(());
                    }

                    if let Some(current_player) = self.players.get(current_player_id) {
                        let seeked_position = current_player.position().await?;

                        let current_track = state.get_current_track_mut();
                        current_track.position = seeked_position;
                    }
                }
            }

            // Updates the current track position.
            MprisEvent::PlayerPosition(pos_changed_id, new_position) => {
                if let Some(current_player_id) = &self.current_player_id {
                    if *current_player_id == pos_changed_id {
                        let current_track = state.get_current_track_mut();
                        current_track.position = new_position;
                    }
                }
            }

            MprisEvent::PlayerFilterUpdated(new_player_filters) => {
                self.filter_players = new_player_filters;
            }
        }

        // Sends out a TrackUpdated event to the script everytime we receive a mpris event.
        self.event_sender
            .send(Ok(Event::Script(ScriptEvent::TrackUpdated)))
            .unwrap();

        Ok(())
    }

    /// Checks if the identity is included in the filter.
    fn check_is_filtered(&self, identity: &mprizzle::PlayerIdentity) -> bool {
        self.filter_players
            .iter()
            .any(|f| identity.matches_either(f))
    }
}
