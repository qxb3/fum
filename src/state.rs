use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{cover::Cover, player::Player, track::Track};

/// Type alias for current player state.
pub type CurrentPlayerState = Arc<Mutex<Option<Box<dyn Player>>>>;

/// Type alias for current track state.
pub type CurrentTrackState = Arc<Mutex<Track>>;

/// Type alias for current track state.
pub type CurrentCoverState = Arc<Mutex<Option<Cover>>>;

/// Contains the application states.
pub struct State {
    /// The current player.
    pub current_player: CurrentPlayerState,

    /// The current track.
    pub current_track: CurrentTrackState,

    /// The current cover art.
    pub current_cover: CurrentCoverState,

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
