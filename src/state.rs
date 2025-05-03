pub struct State {
    exit: bool,
}

impl Default for State {
    fn default() -> Self {
        Self { exit: false }
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
