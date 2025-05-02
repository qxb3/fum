use std::{collections::HashMap, str::FromStr, sync::Arc, time::Duration};

use anyhow::anyhow;
use futures::StreamExt;
use tokio::sync::{broadcast, mpsc, Mutex};
use zbus::{zvariant, Connection, Proxy};

use crate::{status::PlaybackStatus, FumResult};

use super::{
    identity::PlayerIdentity,
    metadata::PlayerMetadata,
    proxies::{self, create_player_proxy, create_properties_proxy},
    MprisEvent,
};

/// Represents an MPRIS media player instance.
///
/// This struct provides an interface to control and retrieve information from an MPRIS-compatible media player.
/// It uses a D-Bus proxy to communicate with the player and manage playback.
///
/// # Example
///
/// ```no_run
/// use mpris::{Mpris, Player, PlayerIdentity};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mpris = Mpris::new().await?;
///
///     // Ideally you should never create your own player and just use the one from `mpris`
///     // but you can just create a player yourself.
///     let spotify = Player::new(&mpris.connection, PlayerIdentity::Bus("org.mpris.MediaPlayer2.spotify".into())).await?;
///
///     let metadata = spotify.metadata().await?;
///
///     let title = metadata.title()?.unwrap_or("No Title".into());
///     println("Current song: {title}");
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct MprisPlayer {
    /// A shared D-Bus connection.
    connection: Arc<Mutex<Connection>>,

    /// Player proxy.
    player_proxy: Proxy<'static>,

    /// The identity of this player.
    identity: PlayerIdentity,
}

impl MprisPlayer {
    pub async fn new(
        shared_connection: Arc<Mutex<Connection>>,
        identity: PlayerIdentity,
    ) -> FumResult<Self> {
        let shared_conn = Arc::clone(&shared_connection);
        let player_proxy = proxies::create_player_proxy(shared_conn, identity.bus()).await?;

        Ok(Self {
            connection: shared_connection,
            player_proxy,
            identity,
        })
    }

    /// Start watching for player events.
    pub fn watch(
        &self,
        event_sender: mpsc::UnboundedSender<FumResult<MprisEvent>>,
        mut close_rx: broadcast::Receiver<String>,
    ) {
        let shared_connection = self.connection();
        let identity = self.identity().clone();

        tokio::spawn(async move {
            // Creates a properties proxy.
            let shared_conn = Arc::clone(&shared_connection);
            let properties_proxy = match create_properties_proxy(shared_conn, identity.bus()).await
            {
                Ok(properties_proxy) => properties_proxy,
                Err(err) => {
                    event_sender.send(Err(err.into())).unwrap();
                    return;
                }
            };

            // Creates a player proxy.
            let shared_conn = Arc::clone(&shared_connection);
            let player_proxy = match create_player_proxy(shared_conn, identity.bus()).await {
                Ok(player_proxy) => player_proxy,
                Err(err) => {
                    event_sender.send(Err(err.into())).unwrap();
                    return;
                }
            };

            // Creates a PropertiesChanged signal stream.
            let mut prop_changed_stream =
                match properties_proxy.receive_signal("PropertiesChanged").await {
                    Ok(properties_changed) => properties_changed,
                    Err(err) => {
                        event_sender.send(Err(err.into())).unwrap();
                        return;
                    }
                };

            // Creates a Seeked signal stream.
            let mut seeked_stream = match player_proxy.receive_signal("Seeked").await {
                Ok(seeked_stream) => seeked_stream,
                Err(err) => {
                    event_sender.send(Err(err.into())).unwrap();
                    return;
                }
            };

            // Create a ticker that tick each seconds to tick me.
            let mut tickler = tokio::time::interval(Duration::from_secs(1));

            loop {
                tokio::select! {
                    // Tells tokio::select to check for the result chronologically.
                    // So it checks if event channel has been closed or
                    // if this player should stop receiving events first, then the rest.
                    biased;

                    // Break out of the loop if the event channel has been closed.
                    _ = event_sender.closed() => break,

                    // Break out of the loop if the close channel event bus matches the identity.
                    close_res = close_rx.recv() => {
                        let bus = match close_res {
                            Ok(bus) => bus,
                            Err(err) => {
                                event_sender.send(Err(anyhow!("Failed to receive close event: {err}"))).unwrap();
                                break;
                            }
                        };

                        // Break if it checks out.
                        if identity.check_bus(&bus) {
                            break
                        }
                    },

                    // Receive PropertiesChanged signal.
                    Some(_) = prop_changed_stream.next() => {
                        // Send out PlayerPropertiesChanged event.
                        event_sender.send(Ok(MprisEvent::PlayerPropertiesChanged(identity.clone()))).unwrap();
                    },

                    // Receive Seeked signal.
                    Some(_) = seeked_stream.next() => {
                        // Send out PlayerSeeked event.
                        event_sender.send(Ok(MprisEvent::PlayerSeeked(identity.clone()))).unwrap();
                    },

                    // Tick that tickler!
                    _ = tickler.tick() => {
                        // Gets the player playback status from D-Bus.
                        let playback_status: String = match player_proxy.get_property("PlaybackStatus").await {
                            Ok(playback_status) => playback_status,
                            Err(err) => {
                                event_sender.send(Err(anyhow!("Failed to get playback status: {err}"))).unwrap();
                                return;
                            }
                        };

                        // Converts the playback status into PlaybackStatus type.
                        let playback_status = match PlaybackStatus::from_str(&playback_status) {
                            Ok(playback_status) => playback_status,
                            Err(err) => {
                                event_sender.send(Err(anyhow!("Failed to parse playback status: {err}"))).unwrap();
                                return;
                            }
                        };

                        // Only send out the PlayerPosition event if the playback is Playing.
                        if playback_status == PlaybackStatus::Playing {
                            // Gets the player position from the D-Bus.
                            let position: i64 = match player_proxy.get_property("Position").await {
                                Ok(position) => position,
                                Err(err) => {
                                    event_sender.send(Err(anyhow!("Failed to get player Position: {err}"))).unwrap();
                                    return;
                                }
                            };

                            // Converts the player position into Duration type.
                            let position = Duration::from_micros(position as u64);

                            // Send out PlayerPosition event.
                            event_sender.send(Ok(MprisEvent::PlayerPosition(identity.clone(), position))).unwrap();
                        }
                    },
                }
            }
        });
    }

    /// Metadata of player.
    pub async fn metadata(&self) -> FumResult<PlayerMetadata> {
        let metadata: HashMap<String, zvariant::Value> =
            self.player_proxy.get_property("Metadata").await?;

        Ok(PlayerMetadata::new(metadata)?)
    }

    /// Playback Rate of player.
    pub async fn playback_rate(&self) -> FumResult<f64> {
        let rate: f64 = self.player_proxy.get_property("Rate").await?;

        Ok(rate)
    }

    /// Set Playback Rate of player.
    pub async fn set_playback_rate(&self, rate: f64) -> FumResult<()> {
        if !self.can_control().await? {
            return Err(anyhow!("Cannot set the Rate as CanControl is false"));
        }

        let min_rate = self.min_playback_rate().await?;
        let max_rate = self.max_playback_rate().await?;

        if rate < min_rate || rate > max_rate {
            return Err(anyhow!(
                "Cannot set the Rate as its passed on MinimumRate or MaximumRate bounds"
            ));
        }

        self.player_proxy.set_property("Rate", rate).await?;

        Ok(())
    }

    /// Minimum Playback Rate of player.
    pub async fn min_playback_rate(&self) -> FumResult<f64> {
        let min_rate: f64 = self.player_proxy.get_property("MinimumRate").await?;

        Ok(min_rate)
    }

    /// Maximum Playback Rate of player.
    pub async fn max_playback_rate(&self) -> FumResult<f64> {
        let max_rate: f64 = self.player_proxy.get_property("MaximumRate").await?;

        Ok(max_rate)
    }

    /// Can the player go next.
    pub async fn can_next(&self) -> FumResult<bool> {
        let can_go_next: bool = self.player_proxy.get_property("CanGoNext").await?;

        Ok(can_go_next)
    }

    /// Can the player go previous.
    pub async fn can_previous(&self) -> FumResult<bool> {
        let can_go_previous: bool = self.player_proxy.get_property("CanGoPrevious").await?;

        Ok(can_go_previous)
    }

    /// Can the player play.
    pub async fn can_play(&self) -> FumResult<bool> {
        let can_play: bool = self.player_proxy.get_property("CanPlay").await?;

        Ok(can_play)
    }

    /// Can the player pause.
    pub async fn can_pause(&self) -> FumResult<bool> {
        let can_pause: bool = self.player_proxy.get_property("CanPause").await?;

        Ok(can_pause)
    }

    /// Can the player seek.
    pub async fn can_seek(&self) -> FumResult<bool> {
        let can_seek: bool = self.player_proxy.get_property("CanSeek").await?;

        Ok(can_seek)
    }

    /// Can the player be controlled.
    pub async fn can_control(&self) -> FumResult<bool> {
        let can_control: bool = self.player_proxy.get_property("CanControl").await?;

        Ok(can_control)
    }

    /// Gets the shared mpris connection.
    fn connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.connection)
    }

    /// Gets the identity of the player.
    pub fn identity(&self) -> &PlayerIdentity {
        &self.identity
    }
}
