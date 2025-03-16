use std::sync::Arc;

use ratatui_image::protocol::StatefulProtocol;
use tokio::sync::Mutex;

use crate::{mpris::Player, track::Track};

/// Contains the application states.
pub struct State {
    /// The current player.
    pub current_player: Arc<Mutex<Option<Player>>>,

    /// The current track.
    pub current_track: Arc<Mutex<Track>>,

    /// The current cover art.
    pub current_cover: Arc<Mutex<Option<StatefulProtocol>>>,

    /// Exit state.
    pub exit: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_player: Arc::new(Mutex::new(None)),
            current_track: Arc::new(Mutex::new(Track::new())),
            current_cover: Arc::new(Mutex::new(None)),
            exit: false,
        }
    }
}

impl State {
    /// Creates new state.
    pub fn new() -> Self {
        Self::default()
    }
}
