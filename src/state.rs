/// A struct that will contains all the state for fum.
pub struct State {
    exit: bool,
}

impl Default for State {
    fn default() -> Self {
        Self { exit: false }
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }
}

impl State {
    /// Sets the exit to true.
    pub fn set_exit(&mut self) {
        self.exit = true;
    }

    pub fn exit(&self) -> bool {
        self.exit
    }
}
