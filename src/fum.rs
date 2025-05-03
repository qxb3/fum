use std::process;

use crate::{
    event::{Event, EventManager},
    state::State,
    terminal::Terminal,
    FumResult,
};

/// Main fum tui app.
pub struct Fum {
    /// Centralized event manager.
    event_manager: EventManager,

    /// Terminal manager.
    terminal: Terminal,

    /// Shared state.
    state: State,
}

impl Fum {
    pub fn new() -> FumResult<Self> {
        let event_manager = EventManager::new();
        let terminal = Terminal::new(10)?;
        let state = State::new();

        Ok(Self {
            event_manager,
            terminal,
            state,
        })
    }

    pub async fn run(&mut self) -> FumResult<()> {
        // Sends events to event manager.
        self.terminal.send_events(self.event_manager.sender());

        // Main loop.
        while !self.state.exit() {
            let event_res = self.event_manager.recv().await?;

            match event_res {
                Ok(event) => match event {
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
