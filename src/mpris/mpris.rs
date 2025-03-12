#![allow(dead_code)]

use std::{collections::HashMap, sync::Arc};

use futures::{stream::FuturesUnordered, StreamExt};
use tokio::sync::Mutex;
use zbus::{Connection, Proxy};

use crate::{fum::Fum, FumResult};

use super::{player, Player};

/// Mpris Events.
pub enum MprisEvent {
    /// When there is a new player.
    PlayerAttached(Player),

    /// When a player de-attach or quits..
    PlayerDetached(String)
}

/// Represents an MPRIS connection.
///
/// This struct provides access to an MPRIS-compatible media player using D-Bus.
/// It allows sending commands and retrieving properties via the D-Bus connection.
///
/// # Example
///
/// ```no_run
/// use mpris::Mpris;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mpris = Mpris::new().await?;
///
///     let players = mpris.players().await?;
///     println!("{:#?}", players);
///
///     if let Some(spotify) = players.get("org.mpris.MediaPlayer2.spotify") {
///         let metadata = spotify.metadata().await?;
///
///         let title = metadata.title()?.unwrap_or("No Title".into());
///         println("Current song: {title}");
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Mpris<'a> {
    pub connection: Arc<Mutex<Connection>>,
    pub dbus_proxy: Proxy<'a>,
}

impl<'a> Mpris<'a> {
    /// Creates a new D-Bus connection.
    pub async fn new() -> FumResult<Self> {
        let connection = Connection::session().await?;
        let dbus_proxy = Mpris::create_dbus_proxy(&connection).await?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            dbus_proxy,
        })
    }

    /// Creates a new Mpris based on existing Connection.
    pub async fn from_connection(connection: Arc<Mutex<Connection>>) -> FumResult<Self> {
        let dbus_proxy = Mpris::create_dbus_proxy(&*connection.lock().await).await?;

        Ok(Self {
            connection,
            dbus_proxy,
        })
    }

    /// Gets all the active players.
    pub async fn players(&self) -> FumResult<HashMap<String, Player>> {
        let bus_names = self.bus_names().await?;

        // Creates a new Player based on bus names.
        let mut players: HashMap<String, Player> = HashMap::new();

        for bus_name in bus_names {
            let player =
                Player::new(self.connection.clone(), bus_name.to_string()).await?;
            players.insert(bus_name, player);
        }

        Ok(players)
    }

    /// List out mpris D-Bus bus names.
    pub async fn bus_names(&self) -> FumResult<Vec<String>> {
        // Call "ListNames" to get all of active D-Bus services.
        let bus_names: Vec<String> = self.dbus_proxy.call("ListNames", &()).await?;

        // Return the filtered mpris only service bus names.
        Ok(bus_names
            .into_iter()
            .filter(|bus_name| bus_name.starts_with("org.mpris.MediaPlayer2."))
            .collect::<Vec<String>>())
    }

    /// Watch for mpris events.
    pub async fn watch(
        &self,
        tx: tokio::sync::mpsc::Sender<MprisEvent>,
    ) -> FumResult<()> {
        let connection_arc = Arc::clone(&self.connection);

        tokio::spawn(async move {
            // Creates a new mpris based on existing connection.
            let mpris = Mpris::from_connection(connection_arc.clone())
                .await
                .expect("Failed to create mpris based on existing connection");

            // Get already active players to send the PlayerAttached event.
            let players = mpris
                .players()
                .await
                .expect("Failed to get active players.");

            // Send PlayerAttached event for existing players.
            for (_, player) in players {
                tx.send(MprisEvent::PlayerAttached(player)).await.unwrap();
            }

            // NameOwnerChanged signal event stream.
            let mut name_owner_stream = mpris
                .dbus_proxy
                .receive_signal("NameOwnerChanged")
                .await
                .expect("Failed to create stream for NameOwnerChanged signal");

            loop {
                tokio::select! {
                    // Break out of this loop if the channel has been closed.
                    _ = tx.closed() => break,

                    Some(signal) = name_owner_stream.next() => {
                        if let Ok((name, old_owner, new_owner)) = signal.body().deserialize::<(String, String, String)>() {
                            if name.starts_with("org.mpris.MediaPlayer2.") {
                                // Get active players.
                                let players = mpris
                                    .players()
                                    .await
                                    .expect("Failed to get active players.");

                                if old_owner.is_empty() && !new_owner.is_empty() {
                                    // Send PlayerAttached event if name matched to players.
                                    if let Some(player) = players.get(&name).cloned() {
                                        tx.send(MprisEvent::PlayerAttached(player)).await.unwrap();
                                    }
                                }

                                if !old_owner.is_empty() && new_owner.is_empty() {
                                    tx.send(MprisEvent::PlayerDetached(name)).await.unwrap();
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(())
    }

    /// Creates a D-Bus proxy.
    pub async fn create_dbus_proxy(connection: &Connection) -> FumResult<Proxy<'a>> {
        let dbus_proxy = Proxy::new(
            connection,
            "org.freedesktop.DBus",
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus",
        )
        .await?;

        Ok(dbus_proxy)
    }
}
