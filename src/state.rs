use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{mpris::Player, track::Track};

/// Contains the application states.
#[derive(Debug)]
pub struct State {
    /// The current player.
    pub current_player: Arc<Mutex<Option<Player>>>,

    /// The current track.
    pub current_track: Arc<Mutex<Track>>,

    /// Exit state.
    pub exit: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_player: Arc::new(Mutex::new(None)),
            current_track: Arc::new(Mutex::new(Track::new())),
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
