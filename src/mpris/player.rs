use std::{collections::HashMap, time::Duration};

use zbus::{zvariant::{self, ObjectPath}, Connection, Proxy};

use crate::FumResult;

use super::Metadata;

/// Playback status of player.
#[derive(Debug)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped
}

/// Loop status of player.
#[derive(Debug)]
pub enum LoopStatus {
    None,
    Track,
    Playlist
}

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

        match playback_status.as_str() {
            "Playing" => Ok(PlaybackStatus::Playing),
            "Paused" => Ok(PlaybackStatus::Paused),
            "Stopped" => Ok(PlaybackStatus::Stopped),
            _ => Err("PlaybackStatus is not Playing,Paused or Stopped.".into())
        }
    }

    /// LoopStatus of player.
    pub async fn loop_status(&self) -> FumResult<LoopStatus> {
        let loop_status: String =
            self.player_proxy.get_property("LoopStatus").await?;

        match loop_status.as_str() {
            "None" => Ok(LoopStatus::None),
            "Track" => Ok(LoopStatus::Track),
            "Playlist" => Ok(LoopStatus::Playlist),
            _ => Err("PlaybackStatus is not None,Track or Playlist.".into())
        }
    }

    /// Playback Rate of player.
    pub async fn playback_rate(&self) -> FumResult<f64> {
        let rate: f64 = self.player_proxy.get_property("Rate").await?;

        Ok(rate)
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

    /// Volume of player.
    pub async fn volume(&self) -> FumResult<f64> {
        let volume: f64 = self.player_proxy.get_property("Volume").await?;

        Ok(volume)
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
