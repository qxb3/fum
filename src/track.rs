use std::time::Duration;

use crate::mpris::PlaybackStatus;

/// Contains the metadata of the current track / song.
#[derive(Debug)]
pub struct Track {
    pub track_id: Option<String>,
    pub title: String,
    pub album: String,
    pub artists: Vec<String>,
    pub length: Duration,
    pub art_url: Option<String>,
    pub playback_status: PlaybackStatus,
    pub shuffle: bool,
    pub volume: f64,
    pub position: Duration,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            track_id: None,
            title: "No Music".into(),
            album: "Album".into(),
            artists: vec!["Artist".into()],
            length: Duration::from_secs(0),
            art_url: None,
            playback_status: PlaybackStatus::Stopped,
            shuffle: false,
            volume: 0.0,
            position: Duration::from_secs(0),
        }
    }
}

impl Track {
    /// Creates a new track.
    pub fn new() -> Self {
        Self::default()
    }
}
