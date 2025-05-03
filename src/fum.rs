use std::{process, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    event::{Event, EventManager, TerminalEvent},
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
    state: Arc<Mutex<State>>,
}

impl Fum {
    pub fn new() -> FumResult<Self> {
        let event_manager = EventManager::new();
        let terminal = Terminal::new(10)?;

        let state = Arc::new(Mutex::new(State::default()));

        Ok(Self {
            event_manager,
            terminal,
            state,
        })
    }

    pub async fn run(&mut self) -> FumResult<()> {
        // Sends events to event manager.
        self.terminal.send_events(self.event_manager.sender());

        while !self.should_exit()? {
            let event_res = self.event_manager.recv().await?;

            match event_res {
                Ok(event) => match event {
                    Event::Terminal(term_event) => self.handle_term_event(term_event)?,
                },
                Err(err) => {
                    eprintln!("ERROR: {err}");
                    process::exit(1);
                }
            }
        }

        Terminal::restore()?;

        Ok(())
    }

    /// Handles TerminalEvent events.
    fn handle_term_event(&mut self, event: TerminalEvent) -> FumResult<()> {
        match event {
            TerminalEvent::Term(event) => match event {
                crossterm::event::Event::Key(key) => self.handle_key_input(key)?,

                _ => {}
            },
            TerminalEvent::Tick(fps) => self.handle_tick(fps)?,
        }

        Ok(())
    }

    /// Handles keyboard input.
    fn handle_key_input(&mut self, key: crossterm::event::KeyEvent) -> FumResult<()> {
        match key.code {
            crossterm::event::KeyCode::Char('q') => {
                let mut state = self.state.try_lock()?;
                state.set_exit();
            }

            _ => {}
        }

        Ok(())
    }

    /// Handles TerminalEvent::Tick event.
    fn handle_tick(&mut self, fps: u64) -> FumResult<()> {
        let terminal = self.terminal.ratatui_terminal_mut();
        terminal.draw(|frame| {
            frame.render_widget(ratatui::text::Text::from(fps.to_string().as_str()), frame.area());
        })?;

        Ok(())
    }

    /// Checks the exit state to see if we should exit.
    fn should_exit(&self) -> FumResult<bool> {
        let state = self.state.try_lock()?;
        let exit = state.exit();

        Ok(exit)
    }
}
