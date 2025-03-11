use std::sync::Arc;

use tokio::sync::Mutex;

use crate::track::Track;

/// Contains the application states.
#[derive(Debug)]
pub struct State {
    /// The current track.
    pub current_track: Arc<Mutex<Track>>,

    /// Exit state.
    pub exit: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
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
