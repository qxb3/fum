use anyhow::Context;

use crate::FumResult;

/// The player identity the player can have.
#[derive(Debug, Clone)]
pub struct PlayerIdentity {
    /// The short name of player.
    short: String,

    /// The full long bus name player.
    bus: String,
}

impl PlayerIdentity {
    /// Creates a player identity.
    /// Can error when the bus_name passed isn't properly formatted.
    pub fn new(bus: String) -> FumResult<Self> {
        let short = bus
            .split('.')
            .nth(3)
            .context("Failed to get player identity, bus_name might not be formatted correctly")?
            .to_lowercase();

        Ok(Self {
            short,
            bus: bus.to_lowercase(),
        })
    }

    /// Checks if the short identity matches the other string.
    pub fn check_short(&self, other: &str) -> bool {
        self.short() == other.to_lowercase()
    }

    /// Checks if the bus identity starts witht the other string.
    pub fn check_bus(&self, other: &str) -> bool {
        self.bus().starts_with(&other.to_lowercase())
    }

    /// Calls both `check_short()` and `check_bus()` and returns true if even one is true.
    pub fn check_both_or(&self, other: &str) -> bool {
        self.check_short(other) || self.check_bus(other)
    }

    /// Gets the short name.
    pub fn short(&self) -> &str {
        &self.short
    }

    /// Gets the bus name.
    pub fn bus(&self) -> &str {
        &self.bus
    }
}
