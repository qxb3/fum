use std::time::Duration;

use crate::{
    mpris::{PlaybackStatus, Player},
    FumResult,
};

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

    /// Creates a new track based on player.
    pub async fn from_player(player: &Player) -> FumResult<Self> {
        let metadata = player.metadata().await?;

        // Comes from the metadata.
        let track_id = metadata.trackid()?;
        let title = metadata.title()?.unwrap_or("No Music".into());
        let album = metadata.album()?.unwrap_or("Album".into());
        let artists = metadata.artists()?.unwrap_or(vec!["Artist".into()]);
        let length = metadata.length()?.unwrap_or(Duration::from_secs(0));
        let art_url = metadata.art_url()?;

        // Comes from the player.
        let playback_status = player
            .playback_status()
            .await
            .unwrap_or(PlaybackStatus::Stopped);

        let shuffle = player.shuffle().await.unwrap_or(false);
        let volume = player.volume().await.unwrap_or(0.0);
        let position = player.position().await.unwrap_or(Duration::from_secs(0));

        Ok(Track {
            track_id,
            title,
            album,
            artists,
            length,
            art_url,
            playback_status,
            shuffle,
            volume,
            position,
        })
    }
}
