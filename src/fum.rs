use std::path::PathBuf;

use crate::{
    cli::RunMode,
    event::{Event, EventManager},
    mpris::Mpris,
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

    /// Mpris manager.
    mpris: Mpris,
}

impl<'a> Fum<'a> {
    pub async fn new(config_path: PathBuf) -> FumResult<Self> {
        let event_manager = EventManager::new();
        let script = Script::new(&event_manager, config_path)?;
        let terminal = Terminal::new(&event_manager, 10)?;
        let state = State::new();

        let mpris = Mpris::new(&event_manager, vec![""]).await?;

        Ok(Self {
            event_manager,
            script,
            terminal,
            state,
            mpris,
        })
    }

    /// Runs fum.
    pub async fn run(&mut self, run_mode: RunMode) -> FumResult<()> {
        // Sends events to event manager.
        // WARNING: the world will be destroyed if this function
        // has been called after the guy below. You've been warned!.
        self.terminal.send_events();

        // Executes the script.
        // Please dont let the guy above be called after me.
        self.script.execute()?;

        // Watches the config script file for changes.
        // The two guys above me is morons.
        self.script.watch_config()?;

        match run_mode {
            RunMode::Player => {}
            RunMode::Mpris => self.mpris.send_events().await,
        }

        // Main loop.
        while !self.state.exit() {
            let event_res = self.event_manager.recv().await?;

            match event_res {
                Ok(event) => match event {
                    Event::Script(event) => self.script.handle(&mut self.state, event).await?,
                    Event::Terminal(event) => self.terminal.handle(&mut self.state, event).await?,
                    Event::Mpris(event) => self.mpris.handle(&mut self.state, event).await?,
                },
                Err(err) => {
                    // Sets the error instead of crashing.
                    // Good shit.
                    self.state.set_error(Some(err));
                }
            }
        }

        // Restores the terminal after exiting.
        Terminal::restore()?;

        Ok(())
    }
}
