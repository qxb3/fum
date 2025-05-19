use std::time::Duration;

use anyhow::anyhow;
use tokio::sync::{broadcast, mpsc};

use crate::{
    config::Config,
    widget::{FumWidget, SendSyncFnPtr},
    FumResult,
};

/// Script events.
#[derive(Debug)]
pub enum ScriptEvent {
    /// When the script calls the CONFIG() function.
    ConfigUpdated(Config),

    /// When the script calls the LAYOUT() function.
    LayoutUpdated(Vec<FumWidget>),

    /// When the config script file has been changed.
    ConfigModified,

    /// When a button widget has been clicked.
    ButtonClicked(SendSyncFnPtr),

    /// When the track has been updated.
    TrackUpdated,
}

/// Terminal events.
#[derive(Debug)]
pub enum TerminalEvent {
    /// All crossterm events.
    Term(crossterm::event::Event),

    /// Its tickler time!.
    Tick(u64),
}

/// Mpris events.
#[derive(Debug)]
pub enum MprisEvent {
    /// When there is a new player attached.
    PlayerAttached(mprizzle::MprisPlayer),

    /// When there is a detached player.
    PlayerDetached(mprizzle::PlayerIdentity),

    /// When a player's properties changed.
    PlayerPropertiesChanged(mprizzle::PlayerIdentity),

    /// When a player's has been seeked!.
    PlayerSeeked(mprizzle::PlayerIdentity),

    /// When a player's position updated.
    PlayerPosition(mprizzle::PlayerIdentity, Duration),

    /// When the `players` from the config script updated.
    PlayerFilterUpdated(Vec<String>),
}

/// All events.
#[derive(Debug)]
pub enum Event {
    Script(ScriptEvent),
    Terminal(TerminalEvent),
    Mpris(MprisEvent),
}

/// A side update events.
#[derive(Debug, Clone)]
pub enum UpdateEvent {
    /// When the config fps updated.
    FpsUpdated(u64),
}

pub type EventResult = FumResult<Event>;
pub type EventSender = mpsc::UnboundedSender<EventResult>;
pub type EventReceiver = mpsc::UnboundedReceiver<EventResult>;

pub type UpdateChannel = broadcast::Sender<UpdateEvent>;

/// A centralized event kind of system.
/// All events will be sent here and read here.
pub struct EventManager {
    /// Main event sender.
    sender: EventSender,

    /// Main event receiver
    receiver: EventReceiver,

    /// A side update channel,
    /// This is typically used in a tokio async task context.
    update_channel: UpdateChannel,
}

impl EventManager {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let (update_channel, _) = broadcast::channel(69);

        Self {
            sender,
            receiver,
            update_channel,
        }
    }

    /// Receive events.
    pub async fn recv(&mut self) -> FumResult<EventResult> {
        self.receiver
            .recv()
            .await
            .ok_or(anyhow!("EventManager event channel has been closed"))
    }

    /// Gets the cloned main event manager sender.
    pub fn sender(&self) -> EventSender {
        self.sender.clone()
    }

    /// Gets the cloned side update channel.
    pub fn update_channel(&self) -> UpdateChannel {
        self.update_channel.clone()
    }
}
