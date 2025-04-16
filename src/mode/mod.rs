mod mpris;
pub use mpris::*;

/// Which mode fum will be.
#[derive(Debug, PartialEq)]
pub enum FumMode {
    /// Fum will be an mp3 music player.
    Player,

    /// Fum will be a client for mpris.
    Mpris,
}
