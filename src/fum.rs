use std::panic;

use ratatui::{prelude::CrosstermBackend, text::Text, Terminal};

use crate::{
    event::{EventHandler, FumEvent},
    state::State,
    FumResult,
};

/// Fum TUI App.
#[derive(Debug)]
pub struct Fum {
    /// ratatui terminal.
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,

    /// Event handler.
    event_handler: EventHandler,

    /// Application state.
    state: State,
}

impl Fum {
    /// Creates new Fum TUI.
    pub fn new() -> FumResult<Self> {
        // Hook into panics to properly restore the terminal
        // when a panic happened.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            ratatui::restore();
            panic_hook(panic);
        }));

        // Enables mouse capture.
        crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

        Ok(Self {
            terminal: ratatui::init(),
            event_handler: EventHandler::new(10),
            state: State::new(),
        })
    }

    /// Start Fum.
    pub async fn start(&mut self) -> FumResult<()> {
        // Start event handler.
        self.event_handler.handle();

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
    pub async fn tick(&mut self) -> FumResult<()> {
        self.terminal.draw(|frame| {
            frame.render_widget(Text::from("Fum"), frame.area());
        })?;

        Ok(())
    }

    /// Handle keypress event.
    pub async fn keypress(&mut self, key: crossterm::event::KeyEvent) -> FumResult<()> {
        match key.code {
            crossterm::event::KeyCode::Char('q') => self.exit(),

            _ => {}
        }

        Ok(())
    }

    /// Handle mouse click event.
    pub async fn mouse_click(
        &mut self,
        _mouse: crossterm::event::MouseEvent,
        _button: crossterm::event::MouseButton,
    ) -> FumResult<()> {
        Ok(())
    }

    /// Exits out of fum.
    pub fn exit(&mut self) {
        ratatui::restore();
        self.state.exit = true;
    }
}
