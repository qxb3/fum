use std::time::Duration;

use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

use crate::FumResult;

/// Fum events.
#[derive(Debug)]
pub enum FumEvent {
    /// Terminal render / update.
    Tick,

    /// Keyboard press.
    KeyPress(crossterm::event::KeyEvent),

    /// Mouse event.
    MouseClick(crossterm::event::MouseEvent, crossterm::event::MouseButton),

    /// Mouse drag event.
    MouseDrag(crossterm::event::MouseEvent, crossterm::event::MouseButton),

    /// Resize event.
    Resize(u16, u16),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event channel sender.
    sender: mpsc::UnboundedSender<FumEvent>,

    /// Event channel receiver.
    receiver: mpsc::UnboundedReceiver<FumEvent>,

    /// Tick rate.
    tick_rate: Duration,
}

impl EventHandler {
    pub fn new(fps: u64) -> Self {
        // Create a new unbounded channel.
        let (sender, receiver) = mpsc::unbounded_channel();

        // Create tick_rate Duration that will be converted from fps to millis.
        let tick_rate = Duration::from_millis(1000 / fps);

        Self {
            sender,
            receiver,
            tick_rate,
        }
    }

    /// Starts listening and sending out events.
    pub fn handle(&self) {
        let sender = self.sender.clone();
        let tick_rate = self.tick_rate.clone();

        tokio::spawn(async move {
            let mut event_stream = crossterm::event::EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_rate);

            loop {
                let term_event = event_stream.next().fuse();

                tokio::select! {
                    // If the event channel has been closed, exit out of this loop.
                    _ = sender.closed() => break,

                    // Send tick event every tick.
                    _ = tick_interval.tick() => sender.send(FumEvent::Tick).unwrap(),

                    // Terminal events.
                    Some(Ok(event)) = term_event => {
                        match event {
                            // Send keypress event.
                            crossterm::event::Event::Key(key) if key.kind == crossterm::event::KeyEventKind::Press => {
                                sender.send(FumEvent::KeyPress(key)).unwrap();
                            },

                            // Handle mouse events.
                            crossterm::event::Event::Mouse(mouse) => {
                                match mouse.kind {
                                    // Send mouse click event.
                                    crossterm::event::MouseEventKind::Down(button) => {
                                        sender.send(FumEvent::MouseClick(mouse, button)).unwrap();
                                    },

                                    crossterm::event::MouseEventKind::Drag(button) => {
                                        sender.send(FumEvent::MouseDrag(mouse, button)).unwrap();
                                    },

                                    _ => {}
                                }
                            },

                            // Handle resize event.
                            crossterm::event::Event::Resize(width, height) => {
                                sender.send(FumEvent::Resize(width, height)).unwrap();
                            },

                            // Ignore the rest of the terminal events.
                            _ => {}
                        }
                    }
                }
            }
        });
    }

    /// Receive the terminal events..
    pub async fn recv(&mut self) -> FumResult<FumEvent> {
        self.receiver
            .recv()
            .await
            .ok_or(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to receive an event from EventHandler",
            )))
    }
}
