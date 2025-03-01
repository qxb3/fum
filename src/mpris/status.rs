use std::{fmt, str::FromStr};

/// Playback status of player.
#[derive(Debug)]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

impl FromStr for PlaybackStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Playing" => Ok(PlaybackStatus::Playing),
            "Paused" => Ok(PlaybackStatus::Paused),
            "Stopped" => Ok(PlaybackStatus::Stopped),
            _ => Err("PlaybackStatus is not Playing,Paused or Stopped.".into()),
        }
    }
}

impl AsRef<str> for PlaybackStatus {
    fn as_ref(&self) -> &str {
        match self {
            PlaybackStatus::Playing => "Playing",
            PlaybackStatus::Paused => "Paused",
            PlaybackStatus::Stopped => "Stopped",
        }
    }
}

impl fmt::Display for PlaybackStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

/// Loop status of player.
#[derive(Debug)]
pub enum LoopStatus {
    None,
    Track,
    Playlist,
}

impl FromStr for LoopStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(LoopStatus::None),
            "Track" => Ok(LoopStatus::Track),
            "Playlist" => Ok(LoopStatus::Playlist),
            _ => Err("LoopStatus is not None,Track or Playlist.".into()),
        }
    }
}

impl AsRef<str> for LoopStatus {
    fn as_ref(&self) -> &str {
        match self {
            LoopStatus::None => "None",
            LoopStatus::Track => "Track",
            LoopStatus::Playlist => "Playlist",
        }
    }
}

impl fmt::Display for LoopStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
