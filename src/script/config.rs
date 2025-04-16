/// A struct containing the config behavior of fum.
/// This comes from the CONFIG() function in the script.
pub struct FumConfig {
    /// How many frames should fum render.
    pub fps: u64,
}

impl Default for FumConfig {
    fn default() -> Self {
        Self { fps: 10 }
    }
}

impl FumConfig {
    pub fn new() -> Self {
        Self::default()
    }
}
