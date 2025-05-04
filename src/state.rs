use crate::config::Config;

/// A struct that will contains all the state for fum.
#[derive(Debug, Default)]
pub struct State {
    /// The config that controls behavior of fum.
    config: Config,

    /// The layout of widgets.
    layout: (),

    /// Exit state.
    exit: bool,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the config state.
    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    /// Gets the config state.
    pub fn config(&self) -> &Config {
        &self.config
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
