#![allow(dead_code)]

use std::{collections::HashMap, ops::Deref, str::FromStr, sync::Arc, time::Duration};

use futures::StreamExt;
use tokio::sync::Mutex;
use zbus::{zvariant::ObjectPath, Connection, Proxy};

use crate::{track::Track, FumResult};

use super::{LoopStatus, Metadata, MetadataValue, PlaybackStatus};

/// Player Events.
#[derive(Debug)]
pub enum PlayerEvent {
    /// When the metadata or properties of the player changed.
    PropertiesChanged,

    /// When a user manually changes the position of the player.
    Seeked,

    /// When the current position of the player changed.
    Position(Duration),
}

/// Represents an MPRIS media player instance.
///
/// This struct provides an interface to control and retrieve information from an MPRIS-compatible media player.
/// It uses a D-Bus proxy to communicate with the player and manage playback.
///
/// # Example
///
/// ```no_run
/// use mpris::{Mpris, Player};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mpris = Mpris::new().await?;
///
///     // Ideally you should use the one provided by mpris.players()
///     // but you can just create a player yourself.
///     let spotify = Player::new(&mpris.connection, "org.mpris.MediaPlayer2.spotify").await?;
///
///     let metadata = spotify.metadata().await?;
///
///     let title = metadata.title()?.unwrap_or("No Title".into());
///     println("Current song: {title}");
///
///     Ok(())
/// }
/// ```
///
/// # Fields
///
/// * `connection` - A reference to the D-Bus connection.
/// * `bus_name` - The D-Bus name of the media player.
#[derive(Debug, Clone)]
pub struct Player {
    connection: Arc<Mutex<Connection>>,
    player_proxy: Proxy<'static>,

    pub bus_name: String,
}

impl Player {
    /// Creates a new Player.
    pub async fn new(
        connection: Arc<Mutex<Connection>>,
        bus_name: String,
    ) -> FumResult<Self> {
        let player_proxy =
            Player::create_player_proxy(connection.clone(), bus_name.to_string()).await?;

        Ok(Self {
            connection,
            player_proxy,
            bus_name,
        })
    }

    /// Metadata of player.
    pub async fn metadata(&self) -> FumResult<Metadata> {
        let metadata: HashMap<String, MetadataValue> =
            self.player_proxy.get_property("Metadata").await?;

        Ok(Metadata::new(metadata)?)
    }

    /// Skips to the next track in the tracklist.
    pub async fn next(&self) -> FumResult<()> {
        self.player_proxy.call_method("Next", &()).await?;

        Ok(())
    }

    /// Skips to the previous track in the tracklist.
    pub async fn previous(&self) -> FumResult<()> {
        self.player_proxy.call_method("Previous", &()).await?;

        Ok(())
    }

    /// Pauses playback.
    pub async fn pause(&self) -> FumResult<()> {
        self.player_proxy.call_method("Pause", &()).await?;

        Ok(())
    }

    /// Play Pause playback.
    pub async fn play_pause(&self) -> FumResult<()> {
        self.player_proxy.call_method("PlayPause", &()).await?;

        Ok(())
    }

    /// Stop playback.
    pub async fn stop(&self) -> FumResult<()> {
        self.player_proxy.call_method("Stop", &()).await?;

        Ok(())
    }

    /// Play playback.
    pub async fn play(&self) -> FumResult<()> {
        self.player_proxy.call_method("Play", &()).await?;

        Ok(())
    }

    /// Seek forward.
    pub async fn seek_forward(&self, offset: Duration) -> FumResult<()> {
        self.player_proxy
            .call_method("Seek", &(offset.as_micros() as i64))
            .await?;

        Ok(())
    }

    /// Seek backward.
    pub async fn seek_backward(&self, offset: Duration) -> FumResult<()> {
        self.player_proxy
            .call_method("Seek", &(-(offset.as_micros() as i64)))
            .await?;

        Ok(())
    }

    /// Set playback position.
    pub async fn set_position(&self, trackid: &str, position: Duration) -> FumResult<()> {
        let trackid = ObjectPath::try_from(trackid)?;

        self.player_proxy
            .call_method("SetPosition", &(trackid, position.as_micros() as i64))
            .await?;

        Ok(())
    }

    /// PlaybackStatus of player.
    pub async fn playback_status(&self) -> FumResult<PlaybackStatus> {
        let playback_status: String =
            self.player_proxy.get_property("PlaybackStatus").await?;

        Ok(PlaybackStatus::from_str(&playback_status)?)
    }

    /// LoopStatus of player.
    pub async fn loop_status(&self) -> FumResult<LoopStatus> {
        let loop_status: String = self.player_proxy.get_property("LoopStatus").await?;

        Ok(LoopStatus::from_str(&loop_status)?)
    }

    /// Set LoopStatus of player.
    pub async fn set_loop_status(&self, loop_status: LoopStatus) -> FumResult<()> {
        if !self.can_control().await? {
            return Err("Cannot set the LoopStatus as CanControl is false".into());
        }

        self.player_proxy
            .set_property("LoopStatus", loop_status.to_string())
            .await?;

        Ok(())
    }

    /// Playback Rate of player.
    pub async fn playback_rate(&self) -> FumResult<f64> {
        let rate: f64 = self.player_proxy.get_property("Rate").await?;

        Ok(rate)
    }

    /// Set Playback Rate of player.
    pub async fn set_playback_rate(&self, rate: f64) -> FumResult<()> {
        if !self.can_control().await? {
            return Err("Cannot set the Rate as CanControl is false".into());
        }

        let min_rate = self.min_playback_rate().await?;
        let max_rate = self.max_playback_rate().await?;

        if rate < min_rate || rate > max_rate {
            return Err(
                "Cannot set the Rate as its passed on MinimumRate or MaximumRate bounds"
                    .into(),
            );
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

    /// Shuffle status of player.
    pub async fn shuffle(&self) -> FumResult<bool> {
        let shuffle: bool = self.player_proxy.get_property("Shuffle").await?;

        Ok(shuffle)
    }

    /// Set Shuffle status of player.
    pub async fn set_shuffle(&self, shuffle: bool) -> FumResult<()> {
        if !self.can_control().await? {
            return Err("Cannot set the Shuffle as CanControl is false".into());
        }

        self.player_proxy.set_property("Shuffle", shuffle).await?;

        Ok(())
    }

    /// Volume of player.
    pub async fn volume(&self) -> FumResult<f64> {
        let volume: f64 = self.player_proxy.get_property("Volume").await?;

        Ok(volume)
    }

    /// Set the Volume of player.
    pub async fn set_volume(&self, volume: f64) -> FumResult<()> {
        if !self.can_control().await? {
            return Err("Cannot set the Volume as CanControl is false".into());
        }

        self.player_proxy.set_property("Volume", volume).await?;

        Ok(())
    }

    /// Position of player.
    pub async fn position(&self) -> FumResult<Duration> {
        let position: i64 = self.player_proxy.get_property("Position").await?;

        Ok(Duration::from_micros(position as u64))
    }

    /// Can the player go next.
    pub async fn can_next(&self) -> FumResult<bool> {
        let can_go_next: bool = self.player_proxy.get_property("CanGoNext").await?;

        Ok(can_go_next)
    }

    /// Can the player go previous.
    pub async fn can_previous(&self) -> FumResult<bool> {
        let can_go_previous: bool =
            self.player_proxy.get_property("CanGoPrevious").await?;

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

    /// Watch events of player.
    pub async fn watch(
        &self,
        tx: tokio::sync::mpsc::Sender<PlayerEvent>,
    ) -> FumResult<()> {
        let connection_arc = Arc::clone(&self.connection);
        let bus_name = self.bus_name.to_string();

        tokio::spawn(async move {
            // Properties proxy.
            let properties_proxy = Player::create_properties_proxy(
                connection_arc.clone(),
                bus_name.to_string(),
            )
            .await
            .expect("Failed to create properties proxy");

            // Player proxy.
            let player_proxy =
                Player::create_player_proxy(connection_arc.clone(), bus_name.to_string())
                    .await
                    .expect("Failed to create player proxy");

            // Creates event stream for PropertiesChanged interface.
            let mut properties_event_stream = properties_proxy
                .receive_signal("PropertiesChanged")
                .await
                .expect("Failed to create stream for PropertiesChanged interface");

            let mut seeked_event_stream = player_proxy
                .receive_signal("Seeked")
                .await
                .expect("Failed to create stream for Seeked signal");

            // Handling of PropertiesChanged event.
            let ptx = tx.clone();
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        // Break out of this loop if the channel has been closed.
                        _ = ptx.closed() => break,

                        // Receive PropertiesChanged events.
                        Some(_) = properties_event_stream.next() => {
                            ptx.send(PlayerEvent::PropertiesChanged).await.unwrap();
                        }
                    }
                }
            });

            // Handling of Seeked event.
            let stx = tx.clone();
            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        // Break out of this loop if the channel has been closed.
                        _ = stx.closed() => break,

                        // Receive Seeked signal events.
                        Some(_) = seeked_event_stream.next() => {
                            stx.send(PlayerEvent::Seeked).await.unwrap();
                        }
                    }
                }
            });

            // Handling of position progress event.
            let potx = tx.clone();
            tokio::spawn(async move {
                // Creates a new player based on the player proxy connection above.
                let player = Player::new(connection_arc.clone(), bus_name.to_string())
                    .await
                    .expect("Failed to create Player in handling of position event");

                // Create a ticker that tick each seconds.
                let mut ticker = tokio::time::interval(Duration::from_secs(1));

                loop {
                    tokio::select! {
                        // Break out of this loop if the channel has been closed.
                        _ = potx.closed() => break,

                        // Tick that tickler!
                        _ = ticker.tick() => {
                            let playback = player
                                .playback_status()
                                .await
                                .expect("Failed to get playback status of player in position event");

                            if playback == PlaybackStatus::Playing {
                                // Gets the new player position.
                                let position = player
                                    .position()
                                    .await
                                    .expect("Failed to get position of player in position event");

                                potx.send(PlayerEvent::Position(position)).await.unwrap();
                            }
                        }
                    }
                }
            });
        });

        Ok(())
    }

    /// Creates a proxy for "org.freedesktop.DBus.Properties".
    async fn create_properties_proxy(
        connection: Arc<Mutex<Connection>>,
        bus_name: String,
    ) -> FumResult<Proxy<'static>> {
        let connection = connection.lock().await;

        let properties_proxy = Proxy::new(
            connection.deref(),
            bus_name,
            "/org/mpris/MediaPlayer2",
            "org.freedesktop.DBus.Properties",
        )
        .await?;

        Ok(properties_proxy)
    }

    /// Proxy for "org.mpris.MediaPlayer2.Player" interface.
    async fn create_player_proxy(
        connection: Arc<Mutex<Connection>>,
        bus_name: String,
    ) -> FumResult<Proxy<'static>> {
        let connection = connection.lock().await;

        let player_proxy: Proxy = zbus::proxy::Builder::new(connection.deref())
            .destination(bus_name.to_string())?
            .path("/org/mpris/MediaPlayer2")?
            .interface("org.mpris.MediaPlayer2.Player")?
            .cache_properties(zbus::proxy::CacheProperties::No)
            .build()
            .await?;

        Ok(player_proxy)
    }
}
