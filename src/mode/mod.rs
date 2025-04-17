mod mpris;
mod player;

pub use mpris::*;
pub use player::*;

use crate::FumResult;

/// Enum of modes events.
#[derive(Debug)]
pub enum FumModeEvent {
    PlayerEvent(()),
    MprisEvent(MprisModeEvent),
}

/// Enum to tell which mode user wants.
#[derive(Debug)]
pub enum FumModes {
    /// Dedicated music mp3 player.
    Player,

    /// Pull music from mpris.
    Mpris,
}

/// Trait to define a mode.
#[async_trait::async_trait]
pub trait FumMode {
    /// Starts up the mode.
    async fn start(&mut self) -> FumResult<()>;

    /// Recieve events.
    async fn recv(&mut self) -> FumResult<FumModeEvent>;
}
