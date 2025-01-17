use std::time::Duration;

use mpris::{Metadata, PlaybackStatus};
use ratatui_image::protocol::StatefulProtocol;

use crate::utils;

#[derive(Clone)]
pub struct CoverArt {
    pub url: String,
    pub image: StatefulProtocol
}

#[derive(Clone)]
pub struct Meta<'a> {
    pub metadata: Metadata,
    pub title: String,
    pub artists: Vec<String>,
    pub album: String,
    pub status: PlaybackStatus,
    pub status_icon: &'a str,
    pub position: Duration,
    pub length: Duration,
    pub cover_art: Option<CoverArt>
}

impl<'a> Default for Meta<'a> {
    fn default() -> Self {
        Self {
            metadata: Metadata::default(),
            title: "No Music".to_string(),
            artists: vec!["Artist".to_string()],
            album: "Album".to_string(),
            status: PlaybackStatus::Stopped,
            status_icon: utils::player::get_status_icon(&PlaybackStatus::Stopped),
            position: Duration::from_secs(0),
            length: Duration::from_secs(0),
            cover_art: None
        }
    }
}
