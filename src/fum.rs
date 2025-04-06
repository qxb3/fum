use std::{ops::Deref, panic, path::PathBuf, sync::Arc};

use ratatui::{layout::Rect, prelude::CrosstermBackend, Terminal};
use taffy::prelude::TaffyMinContent;

use crate::{
    event::{EventHandler, FumEvent},
    mode::{FumMode, MprisMode},
    script::Script,
    state::State,
    ui,
    widget::FumWidget,
    FumResult,
};

/// Fum TUI App.
pub struct Fum<'a> {
    /// ratatui terminal.
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,

    /// Event handler.
    event_handler: EventHandler,

    /// Application state.
    state: State,

    /// Config script.
    script: Script<'a>,
}

impl<'a> Fum<'a> {
    /// Creates new Fum TUI.
    pub async fn new(config_path: &PathBuf) -> FumResult<Self> {
        // Hook into panics to properly restore the terminal
        // when a panic happened.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            let _ = Fum::restore();

            panic_hook(panic);
        }));

        // Enables mouse capture.
        crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

        let terminal = ratatui::init();
        let event_handler = EventHandler::new(10);
        let state = State::new();
        let script = Script::from_file(config_path)?;

        Ok(Self {
            terminal,
            event_handler,
            state,
            script,
        })
    }

    /// Start Fum.
    pub async fn start(&mut self, mode: FumMode) -> FumResult<()> {
        // Start event handler.
        self.event_handler.handle();

        // Execute the script at start.
        self.script.execute()?;

        // Handle the corresponding mode.
        match mode {
            FumMode::Player => {}
            FumMode::Mpris => {
                let mut mpris_mode = MprisMode::new(&mut self.state).await?;
                mpris_mode.handle().await?;
            }
        }

        // Read events and execute while we running.
        while !self.state.exit {
            match self.event_handler.next().await? {
                FumEvent::Tick => self.tick().await?,
                FumEvent::KeyPress(key) => self.keypress(key).await?,
                FumEvent::MouseClick(mouse, button) => {
                    self.mouse_click(mouse, button).await?
                }
            }
        }

        Ok(())
    }

    /// Handle tick event.
    async fn tick(&mut self) -> FumResult<()> {
        let ui_arc = Arc::clone(&self.script.ui);
        let ui = ui_arc
            .lock()
            .map_err(|err| format!("Failed to acquire lock for ui: {err}"))?;

        // Draws the ui.
        ui::draw(&mut self.terminal, &mut self.state, ui.deref()).await?;

        Ok(())
    }

    /// Handle keypress event.
    async fn keypress(&mut self, key: crossterm::event::KeyEvent) -> FumResult<()> {
        match key.code {
            crossterm::event::KeyCode::Char('q') => self.exit()?,

            _ => {}
        }

        Ok(())
    }

    /// Handle mouse click event.
    async fn mouse_click(
        &mut self,
        _mouse: crossterm::event::MouseEvent,
        _button: crossterm::event::MouseButton,
    ) -> FumResult<()> {
        Ok(())
    }

    /// Restore terminal state.
    fn restore() -> FumResult<()> {
        // Restore terminal.
        ratatui::restore();

        // Disables mouse capture.
        crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;

        Ok(())
    }

    /// Exits out of fum.
    fn exit(&mut self) -> FumResult<()> {
        Fum::restore()?;

        self.state.exit = true;

        Ok(())
    }
}
