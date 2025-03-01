use std::collections::HashMap;

use zbus::{zvariant, Connection, Proxy};

use crate::FumResult;

use super::Metadata;

#[derive(Debug)]
pub struct Player<'a> {
    player_proxy: Proxy<'a>,
    properties_proxy: Proxy<'a>,

    pub bus_name: String,
}

impl<'a> Player<'a> {
    /// Creates a new player struct.
    pub async fn new(connection: &'a Connection, bus_name: String) -> FumResult<Self> {
        // Proxy for "org.mpris.MediaPlayer2.Player" interface.
        let player_proxy = Proxy::new(
            connection,
            bus_name.to_string(),
            "/org/mpris/MediaPlayer2",
            "org.mpris.MediaPlayer2.Player"
        ).await?;

        // Proxy for "org.freedesktop.DBus.Properties" interface.
        let properties_proxy = Proxy::new(
            connection,
            bus_name.to_string(),
            "/org/mpris/MediaPlayer2",
            "org.freedesktop.DBus.Properties"
        ).await?;

        Ok(Self {
            player_proxy,
            properties_proxy,
            bus_name
        })
    }

    /// Get the player metadata.
    pub async fn metadata(&self) -> FumResult<Metadata> {
        let metadata: HashMap<String, zvariant::Value> =
            self.player_proxy.get_property("Metadata").await?;

        Ok(Metadata::new(metadata)?)
    }
}
