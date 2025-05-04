use std::io;
use std::time::{Duration, Instant};

use anyhow::anyhow;
use futures::{FutureExt, StreamExt};
use ratatui::prelude::CrosstermBackend;

use crate::{
    event::{Event, EventSender, TerminalEvent},
    state::State,
    FumResult,
};

/// Manages the terminal input and rendering.
pub struct Terminal {
    /// Ratatui terminal.
    terminal: ratatui::Terminal<CrosstermBackend<io::Stdout>>,

    /// The rate the tick event will be sent out.
    tick_rate: Duration,
}

impl Terminal {
    pub fn new(fps: u64) -> FumResult<Self> {
        let terminal = ratatui::Terminal::new(CrosstermBackend::new(io::stdout()))?;

        // Switch to alternative screen.
        crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

        // Enables mouse capture.
        crossterm::execute!(io::stdout(), crossterm::event::EnableMouseCapture)?;

        // Enables raw mode.
        crossterm::terminal::enable_raw_mode()?;

        // Restores the original state of the terminal when a panic happen.
        let hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic| {
            let _ = Terminal::restore();
            hook(panic);
        }));

        // Converts the fps into Duration.
        let tick_rate = Duration::from_millis(1000 / fps);

        Ok(Self { terminal, tick_rate })
    }

    /// Sends events into the centalized event thingy.
    pub fn send_events(&self, event_sender: EventSender) {
        let tick_rate = self.tick_rate.clone();

        tokio::spawn(async move {
            let mut term_event_stream = crossterm::event::EventStream::new();

            let mut tick_interval = tokio::time::interval(tick_rate);
            let mut last_tick = Instant::now();

            loop {
                let term_event = term_event_stream.next().fuse();

                tokio::select! {
                    // Sends out term events.
                    Some(term_event) = term_event => {
                        match term_event {
                            // If Ok, sends out term event..
                            Ok(event) => {
                                event_sender.send(Ok(Event::Terminal(TerminalEvent::Term(event))))
                                    .expect("Failed to send out event: TerminalEvent::Term");
                            },

                            // If Err, Sends out err event.
                            Err(err) => {
                                event_sender.send(Err(anyhow!("Error on watching terminal events: {err}")))
                                    .expect("Failed to send out err event");
                            }
                        }
                    },

                    // Sends out tick events.
                    _ = tick_interval.tick() => {
                        // Calculates fps.
                        let now = Instant::now();
                        let delta = now.duration_since(last_tick);
                        let fps = 1.0 / delta.as_secs_f64();

                        // Sets the last tick to now.
                        last_tick = now;

                        event_sender.send(Ok(Event::Terminal(TerminalEvent::Tick(fps as u64))))
                            .expect("Failed to send out event: TerminalEvent::Tick");
                    }
                }
            }
        });
    }

    /// Handle the terminal events.
    pub async fn handle(&mut self, state: &mut State, event: TerminalEvent) -> FumResult<()> {
        match event {
            TerminalEvent::Term(event) => match event {
                crossterm::event::Event::Key(key) => self.handle_key_input(state, key)?,

                _ => {}
            },
            TerminalEvent::Tick(fps) => self.handle_tick(state, fps)?,
        }

        Ok(())
    }

    /// Handles keyboard input.
    fn handle_key_input(
        &self,
        state: &mut State,
        key: crossterm::event::KeyEvent,
    ) -> FumResult<()> {
        match key.code {
            crossterm::event::KeyCode::Char('q') => {
                state.set_exit();
            }

            _ => {}
        }

        Ok(())
    }

    /// Handles TerminalEvent::Tick event.
    fn handle_tick(&mut self, state: &mut State, fps: u64) -> FumResult<()> {
        let config = state.config();
        let terminal = self.ratatui_terminal_mut();

        terminal.draw(|frame| {
            frame.render_widget(
                ratatui::text::Text::from(format!("Fps Set: {}. Current Fps: {fps}", config.fps)),
                frame.area(),
            );
        })?;

        Ok(())
    }

    /// Restores the terminal's original state.
    pub fn restore() -> FumResult<()> {
        // Switch out of alternate screen.
        crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;

        // Disables mouse capture.
        crossterm::execute!(io::stdout(), crossterm::event::DisableMouseCapture)?;

        // Disables raw mode.
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }

    /// Gets the reference to ratatui::Terminal.
    #[allow(dead_code)]
    pub fn ratatui_terminal(&self) -> &ratatui::Terminal<CrosstermBackend<io::Stdout>> {
        &self.terminal
    }

    /// Gets the mutable reference to ratatui::Terminal.
    pub fn ratatui_terminal_mut(&mut self) -> &mut ratatui::Terminal<CrosstermBackend<io::Stdout>> {
        &mut self.terminal
    }
}
