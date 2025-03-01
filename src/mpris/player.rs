use std::{collections::HashMap, str::FromStr, time::Duration};

use zbus::{zvariant::{self, ObjectPath}, Connection, Proxy};

use crate::FumResult;

use super::{LoopStatus, Metadata, PlaybackStatus};

#[derive(Debug)]
pub struct Player<'a> {
    player_proxy: Proxy<'a>,
    properties_proxy: Proxy<'a>,

    pub bus_name: String,
}

impl<'a> Player<'a> {
    /// Creates a new Player.
    pub async fn new(connection: &'a Connection, bus_name: String) -> FumResult<Self> {
        // Proxy for "org.mpris.MediaPlayer2.Player" interface.
        let player_proxy = Proxy::new(
            connection,
            bus_name.to_string(),
            "/org/mpris/MediaPlayer2",
            "org.mpris.MediaPlayer2.Player"
        ).await?;

        // Proxy for "org.freedesktop.DBus.Properties" interface.
        let properties_proxy = Proxy::new(
            connection,
            bus_name.to_string(),
            "/org/mpris/MediaPlayer2",
            "org.freedesktop.DBus.Properties"
        ).await?;

        Ok(Self {
            player_proxy,
            properties_proxy,
            bus_name
        })
    }

    /// Metadata of player.
    pub async fn metadata(&self) -> FumResult<Metadata> {
        let metadata: HashMap<String, zvariant::Value> =
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
        self.player_proxy.call_method("Seek", &(offset.as_micros() as i64)).await?;

        Ok(())
    }

    /// Seek backward.
    pub async fn seek_backward(&self, offset: Duration) -> FumResult<()> {
        self.player_proxy.call_method("Seek", &(-(offset.as_micros() as i64))).await?;

        Ok(())
    }

    /// Set playback position.
    pub async fn set_position(&self, trackid: &str, position: Duration) -> FumResult<()> {
        let trackid = ObjectPath::try_from(trackid)?;

        self.player_proxy.call_method("SetPosition", &(trackid, position.as_micros() as i64)).await?;

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
        let loop_status: String =
            self.player_proxy.get_property("LoopStatus").await?;

        Ok(LoopStatus::from_str(&loop_status)?)
    }

    /// Set LoopStatus of player.
    pub async fn set_loop_status(&self, loop_status: LoopStatus) -> FumResult<()> {
        if !self.can_control().await? {
            return Err("Cannot set the LoopStatus as CanControl is false".into());
        }

        self.player_proxy.set_property("LoopStatus", loop_status.to_string()).await?;

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
            return Err("Cannot set the Rate as its passed on MinimumRate or MaximumRate bounds".into());
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
}
