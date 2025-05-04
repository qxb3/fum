/// A struct that will contains all the state for fum.
pub struct State {
    /// The config.
    config: (),

    /// The layout of widgets.
    layout: (),

    /// Exit state.
    exit: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            config: (),
            layout: (),
            exit: false,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the config state.
    pub fn set_config(&mut self, config: ()) {
        self.config = config;
    }

    /// Gets the config state.
    pub fn config(&mut self) -> () {
        self.config
    }

    /// Sets the layout state.
    pub fn set_layout(&mut self, layout: ()) {
        self.layout = layout;
    }

    /// Gets the layout state.
    pub fn layout(&mut self) -> () {
        self.layout
    }

    /// Sets the exit state to true.
    pub fn set_exit(&mut self) {
        self.exit = true;
    }

    /// Gets the exit state.
    pub fn exit(&self) -> bool {
        self.exit
    }
}
