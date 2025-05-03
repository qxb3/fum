use std::time::{Duration, Instant};

use anyhow::anyhow;
use futures::{FutureExt, StreamExt};
use ratatui::prelude::CrosstermBackend;

use crate::{
    event::{Event, EventSender, TerminalEvent},
    FumResult,
};

/// Manages the terminal input and rendering.
pub struct Terminal {
    /// Ratatui terminal.
    terminal: ratatui::Terminal<CrosstermBackend<std::io::Stdout>>,

    /// The rate the tick event will be sent out.
    tick_rate: Duration,
}

impl Terminal {
    pub fn new(fps: u64) -> FumResult<Self> {
        let terminal = ratatui::Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

        // Switch to alternative screen.
        crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;

        // Enables mouse capture.
        crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

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

    /// Restores the terminal's original state.
    pub fn restore() -> FumResult<()> {
        // Switch out of alternate screen.
        crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;

        // Disables mouse capture.
        crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;

        // Disables raw mode.
        crossterm::terminal::disable_raw_mode()?;

        Ok(())
    }

    /// Gets the reference to ratatui::Terminal.
    #[allow(dead_code)]
    pub fn ratatui_terminal(&self) -> &ratatui::Terminal<CrosstermBackend<std::io::Stdout>> {
        &self.terminal
    }

    /// Gets the mutable reference to ratatui::Terminal.
    pub fn ratatui_terminal_mut(
        &mut self,
    ) -> &mut ratatui::Terminal<CrosstermBackend<std::io::Stdout>> {
        &mut self.terminal
    }
}
