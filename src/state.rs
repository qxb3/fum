use std::sync::Arc;

use ratatui::layout::Position;
use tokio::sync::Mutex;

use crate::{cover::Cover, player::Player, track::Track};

/// Type alias for the fum application state.
pub type FumState = Arc<Mutex<State>>;

/// Drag state.
#[derive(Debug, Default)]
pub struct DragState {
    /// Whether is currently dragging.
    pub is_dragging: bool,

    /// Where the starting position of the drag occur.
    pub start_drag: Option<Position>,

    /// Where the current position of drag is.
    pub current_drag: Option<Position>,
}

impl DragState {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Contains the application states.
pub struct State {
    /// The current player, If there is one.
    current_player: Option<Box<dyn Player>>,

    /// The current track.
    current_track: Track,

    /// The current cover art loaded, If there is one.
    current_cover: Option<Cover>,

    /// Drag state.
    drag: DragState,

    /// Exit state.
    exit: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_player: None,
            current_track: Track::default(),
            current_cover: None,
            drag: DragState::new(),
            exit: false,
        }
    }
}

impl State {
    /// Creates a new State wrapped in Arc<Mutex>.
    pub fn new_shared() -> FumState {
        Arc::new(Mutex::new(Self::default()))
    }

    /// Checks if we should exit.
    pub fn should_exit(&self) -> bool {
        self.exit
    }

    /// Exit.
    pub fn exit(&mut self) {
        self.exit = true;
    }
}

/// Current player state functions.
impl State {
    /// Sets the current player.
    pub fn set_player(&mut self, player: Box<dyn Player>) {
        self.current_player = Some(player);
    }

    /// Removes the current player.
    pub fn clear_player(&mut self) {
        self.current_player = None;
    }

    /// Gets a reference to the current player.
    pub fn get_player(&self) -> Option<&dyn Player> {
        self.current_player.as_deref()
    }

    /// Gets a mutable reference to the current player.
    pub fn get_player_mut(&mut self) -> Option<&mut dyn Player> {
        self.current_player.as_deref_mut()
    }
}

/// Current track state functions.
impl State {
    /// Sets the current track.
    pub fn set_track(&mut self, track: Track) {
        self.current_track = track;
    }

    /// Resets the current track to default metadata.
    pub fn reset_track(&mut self) {
        self.current_track = Track::default();
    }

    /// Gets a reference to the current track.
    pub fn get_track(&self) -> &Track {
        &self.current_track
    }

    /// Gets a mutable reference to the current track.
    pub fn get_track_mut(&mut self) -> &mut Track {
        &mut self.current_track
    }
}

/// Current cover state functions.
impl State {
    /// Sets the current cover.
    pub fn set_cover(&mut self, cover: Cover) {
        self.current_cover = Some(cover);
    }

    /// Removes the current cover.
    pub fn clear_cover(&mut self) {
        self.current_cover = None;
    }

    /// Gets a reference to the current cover.
    pub fn get_cover(&self) -> Option<&Cover> {
        self.current_cover.as_ref()
    }

    /// Gets a mutable reference to the current cover.
    pub fn get_cover_mut(&mut self) -> Option<&mut Cover> {
        self.current_cover.as_mut()
    }
}

/// Drag state functions.
impl State {
    /// Checks if currently dragging.
    pub fn is_dragging(&self) -> bool {
        self.drag.is_dragging
    }

    /// Initiates drag.
    pub fn start_drag(&mut self, start_drag: Position) {
        self.drag.is_dragging = true;
        self.drag.start_drag = Some(start_drag);
        self.drag.current_drag = Some(start_drag); // Sets the current drag to the start drag since its gonna be there anyway.
    }

    /// Updates the current drag position.
    pub fn update_current_drag(&mut self, current_position: Position) {
        // Only update the current drag if we draggin'.
        if self.is_dragging() {
            self.drag.current_drag = Some(current_position);
        }
    }

    /// Ends the drag.
    pub fn end_drag(&mut self) {
        self.drag.is_dragging = false;
        self.drag.start_drag = None;
        self.drag.current_drag = None;
    }

    /// Gets a reference of the drag state.
    pub fn get_drag(&self) -> &DragState {
        &self.drag
    }
}
