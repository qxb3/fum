use std::{collections::HashMap, sync::Arc};

use anyhow::anyhow;
use tokio::sync::Mutex;
use zbus::{zvariant, Connection, Proxy};

use crate::FumResult;

use super::{identity::PlayerIdentity, metadata::PlayerMetadata, proxies};

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
        let connection = shared_conn.lock().await;

        let player_proxy = proxies::create_player_proxy(&*connection, identity.bus()).await?;

        Ok(Self {
            connection: shared_connection,
            player_proxy,
            identity,
        })
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

    /// Gets the identity of the player.
    pub fn identity(&self) -> &PlayerIdentity {
        &self.identity
    }
}
