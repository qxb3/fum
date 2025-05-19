/// Config of fum.
/// This will be coming from the script CONFIG() function.
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    /// How many frames should fum render.
    pub fps: u64,

    /// A list of player that should only be managed.
    pub filter_players: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fps: 10,
            filter_players: Vec::new(),
        }
    }
}
