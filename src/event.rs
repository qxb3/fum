use anyhow::anyhow;
use tokio::sync::mpsc;

use crate::{config::Config, widget::FumWidget, FumResult};

/// Script events.
#[derive(Debug)]
pub enum ScriptEvent {
    /// When the script calls the CONFIG() function.
    ConfigUpdated(Config),

    /// When the script calls the LAYOUT() function.
    LayoutUpdated(Vec<FumWidget>),

    /// When the config script file has been changed.
    ConfigModified,
}

/// Terminal events.
#[derive(Debug)]
pub enum TerminalEvent {
    /// All crossterm events.
    Term(crossterm::event::Event),

    /// Its tick time.
    Tick(u64),
}

/// All events.
#[derive(Debug)]
pub enum Event {
    Script(ScriptEvent),
    Terminal(TerminalEvent),
}

pub type EventResult = FumResult<Event>;
pub type EventSender = mpsc::UnboundedSender<EventResult>;
pub type EventReceiver = mpsc::UnboundedReceiver<EventResult>;

/// A centralized event kind of system.
/// All events will be sent here and read here.
pub struct EventManager {
    /// Main event sender.
    sender: EventSender,

    /// Main event receiver
    receiver: EventReceiver,
}

impl EventManager {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        Self { sender, receiver }
    }

    /// Receive events.
    pub async fn recv(&mut self) -> FumResult<EventResult> {
        self.receiver
            .recv()
            .await
            .ok_or(anyhow!("EventManager event channel has been closed"))
    }

    /// Gets the cloned event manager sender.
    pub fn sender(&self) -> EventSender {
        self.sender.clone()
    }
}
