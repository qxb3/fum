use std::time::Duration;

use image::DynamicImage;
use mpris::PlaybackStatus;

#[derive(Debug, Clone)]
pub struct Meta {
    pub title: String,
    pub artists: Vec<String>,
    pub status: PlaybackStatus,
    pub position: Duration,
    pub length: Duration,
    pub cover_art: DynamicImage
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            title: "No Music".to_string(),
            artists: vec!["Artist".to_string()],
            status: PlaybackStatus::Stopped,
            position: Duration::from_secs(0),
            length: Duration::from_secs(0),
            cover_art: DynamicImage::default()
        }
    }
}
