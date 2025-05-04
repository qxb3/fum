/// Config of fum.
/// This will be coming from the script CONFIG() function.
#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    /// How many frames should fum render.
    pub fps: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self { fps: 10 }
    }
}
