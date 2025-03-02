use std::collections::HashMap;

use futures::{stream::FuturesUnordered, StreamExt};
use zbus::{Connection, Proxy};

use crate::FumResult;

use super::Player;

#[derive(Debug)]
pub struct Mpris<'a> {
    pub connection: Connection,
    pub proxy: Proxy<'a>
}

impl<'a> Mpris<'a> {
    /// Creates a new D-Bus connection.
    pub async fn new() -> FumResult<Self> {
        let connection = Connection::session().await?;

        let proxy = Proxy::new(
            &connection,
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus",
        )
        .await?;

        Ok(Self {
            connection,
            proxy
        })
    }

    /// Gets all the active players.
    pub async fn players(&self) -> FumResult<HashMap<String, Player>> {
        let bus_names = self.bus_names().await?;

        // Creates a new Player based on bus names.
        let players: HashMap<String, Player> = bus_names
            .into_iter()
            .map(|bus_name| async move {
                Player::new(&self.connection, bus_name)
                    .await
                    .ok()
                    .map(|player| (player.bus_name.to_string(), player))
            })
            .collect::<FuturesUnordered<_>>()
            .filter_map(|p| async { p })
            .collect()
            .await;

        Ok(players)
    }

    /// List out mpris D-Bus bus names.
    pub async fn bus_names(&self) -> FumResult<Vec<String>> {
        // Call "ListNames" to get all of active D-Bus services.
        let bus_names: Vec<String> = self.proxy.call("ListNames", &()).await?;

        // Return the filtered mpris only service bus names.
        Ok(bus_names
            .into_iter()
            .filter(|bus_name| bus_name.starts_with("org.mpris.MediaPlayer2."))
            .collect::<Vec<String>>())
    }
}
