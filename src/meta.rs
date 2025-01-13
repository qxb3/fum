use std::time::Duration;

use mpris::PlaybackStatus;
use ratatui_image::protocol::StatefulProtocol;

#[derive(Clone)]
pub struct CoverArt {
    pub url: String,
    pub image: StatefulProtocol
}

#[derive(Clone)]
pub struct Meta {
    pub title: String,
    pub artists: Vec<String>,
    pub status: PlaybackStatus,
    pub position: Duration,
    pub length: Duration,
    pub cover_art: Option<CoverArt>
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            title: "No Music".to_string(),
            artists: vec!["Artist".to_string()],
            status: PlaybackStatus::Stopped,
            position: Duration::from_secs(0),
            length: Duration::from_secs(0),
            cover_art: None
        }
    }
}
