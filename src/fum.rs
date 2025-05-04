use std::{path::PathBuf, process};

use crate::{
    cli::RunMode,
    event::{Event, EventManager},
    script::Script,
    state::State,
    terminal::Terminal,
    FumResult,
};

/// Main fum tui app.
pub struct Fum<'a> {
    /// Centralized event manager.
    event_manager: EventManager,

    /// Manages the script.
    script: Script<'a>,

    /// Terminal manager.
    terminal: Terminal,

    /// Shared state.
    state: State,
}

impl<'a> Fum<'a> {
    pub fn new(config_path: PathBuf) -> FumResult<Self> {
        let event_manager = EventManager::new();
        let script = Script::new(event_manager.sender(), config_path)?;
        let terminal = Terminal::new(10)?;
        let state = State::new();

        Ok(Self {
            event_manager,
            script,
            terminal,
            state,
        })
    }

    /// Runs fum.
    pub async fn run(&mut self, _run_mode: RunMode) -> FumResult<()> {
        // Executes the script at start.
        self.script.execute()?;

        // Watches the config script file for changes.
        self.script.watch_config()?;

        // Sends events to event manager.
        self.terminal.send_events(self.event_manager.sender());

        // Main loop.
        while !self.state.exit() {
            let event_res = self.event_manager.recv().await?;

            match event_res {
                Ok(event) => match event {
                    Event::Script(event) => self.script.handle(&mut self.state, event).await?,
                    Event::Terminal(event) => self.terminal.handle(&mut self.state, event).await?,
                },
                Err(err) => {
                    eprintln!("ERROR: {err}");
                    process::exit(1);
                }
            }
        }

        // Restores the terminal after exiting.
        Terminal::restore()?;

        Ok(())
    }
}
