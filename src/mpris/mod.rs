mod identity;
mod metadata;
mod player;
mod proxies;

use std::sync::Arc;

use anyhow::{anyhow, Context};
use futures::StreamExt;
use identity::PlayerIdentity;
use player::MprisPlayer;
use tokio::sync::{mpsc, Mutex};
use zbus::Connection;

use crate::FumResult;

/// Mpris events.
pub enum MprisEvent {
    /// Triggers when a new player has been attached or added.
    PlayerAttached(PlayerIdentity),

    /// Triggers when an existing player has been detached or removed.
    PlayerDetached(PlayerIdentity),
}

/// Mpris options.
#[derive(Debug, Clone)]
pub struct MprisOptions<T: IntoIterator<Item = &'static str> + Clone + Send> {
    pub filter_players: T,
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
///     let mpris = Mpris::new_without_options().await?;
///     mpris.watch();
///     loop {
///         let event_result = mpris.recv().await?;
///
///         match event_result {
///             Ok(event) => match event {
///                 MprisEvent::PlayerAttached(identity) => println!("ATTACHED = {:?}", identity),
///                 MprisEvent::PlayerDetached(identity) => println!("DETACHED = {:?}", identity),
///             },
///             Err(err) => panic!("{:?}", err),
///         }
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct Mpris<T: IntoIterator<Item = &'static str> + Clone + Send> {
    /// The underlying connection to D-Bus.
    connection: Arc<Mutex<Connection>>,

    /// The current active players.
    players: Arc<Mutex<Vec<MprisPlayer>>>,

    /// Mpris options passed.
    options: Option<MprisOptions<T>>,

    /// Event sender.
    sender: mpsc::UnboundedSender<FumResult<MprisEvent>>,

    /// Event receiver.
    receiver: mpsc::UnboundedReceiver<FumResult<MprisEvent>>,
}

impl<T: IntoIterator<Item = &'static str> + Clone + Send + 'static> Mpris<T> {
    pub async fn new(options: Option<MprisOptions<T>>) -> FumResult<Self> {
        let connection = Arc::new(Mutex::new(Connection::session().await?));
        let players = Arc::new(Mutex::new(Vec::new()));
        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        Ok(Self {
            connection,
            players,
            options,
            sender: event_sender,
            receiver: event_receiver,
        })
    }

    /// Start watching for mpris events.
    pub fn watch(&self) {
        let shared_connection = self.connection();
        let shared_players = self.players();
        let options = self.options();

        let event_sender = self.sender.clone();

        tokio::spawn(async move {
            let connection = shared_connection.lock().await;

            // Creates a new dbus proxy.
            let dbus_proxy = match proxies::create_dbus_proxy(&*connection).await {
                Ok(dbus_proxy) => dbus_proxy,
                Err(err) => {
                    event_sender.send(Err(err)).unwrap();
                    return;
                }
            };

            // Creates a NameOwnerChanged signal stream.
            let mut noc_stream = match dbus_proxy.receive_signal("NameOwnerChanged").await {
                Ok(noc_stream) => noc_stream,
                Err(err) => {
                    event_sender.send(Err(err.into())).unwrap();
                    return;
                }
            };

            loop {
                tokio::select! {
                    _ = event_sender.closed() => break,

                    // Receive NameOwnerChanged signal.
                    Some(signal) = noc_stream.next() => {
                        if let Ok((name, old_owner, new_owner)) = signal.body().deserialize::<(String, String, String)>() {
                            // Only accepts mpris signals.
                            if !name.starts_with("org.mpris.MediaPlayer2.") {
                                continue;
                            }

                            // There has been a new mpris player.
                            if old_owner.is_empty() && !new_owner.is_empty() {
                                // Creates the player identity.
                                let identity = match PlayerIdentity::new(name.to_string()) {
                                    Ok(identity) => identity,
                                    Err(err) => {
                                        event_sender.send(Err(err.into())).unwrap();
                                        return;
                                    }
                                };

                                // Check if the name is on the filter list in the options.
                                if !options.as_ref().map_or(false, |f| f.filter_players.clone().into_iter().find(|f| identity.check_both_or(&f)).is_some()) {
                                    // Creates the player itself with the shared connection.
                                    let player = match MprisPlayer::new(Arc::clone(&shared_connection), identity.clone()).await {
                                        Ok(player) => player,
                                        Err(err) => {
                                            event_sender.send(Err(err.into())).unwrap();
                                            return;
                                        }
                                    };

                                    // Push the player in the shared players.
                                    let mut players = shared_players.lock().await;
                                    players.push(player);

                                    // Send out PlayerAttached event along with the identity.
                                    event_sender.send(Ok(MprisEvent::PlayerAttached(identity))).unwrap();
                                }
                            }

                            // There has been a mpris player detached.
                            if !old_owner.is_empty() && new_owner.is_empty() {
                                let mut players = shared_players.lock().await;

                                // Only send out the PlayerDetached event if its on the shared players only.
                                if let Some(index) = players.iter().position(|p| p.identity().check_both_or(&name)) {
                                    let player = match players.get(index) {
                                        Some(player) => player,
                                        None => {
                                            event_sender.send(Err(anyhow!("Expected a player at index {index} but got None"))).unwrap();
                                            return;
                                        }
                                    };

                                    // Gets the player identity.
                                    let identity = player.identity().clone();

                                    // Remove the player at the shared players.
                                    players.remove(index);

                                    // Send out the PlayerDetached event.
                                    event_sender.send(Ok(MprisEvent::PlayerDetached(identity))).unwrap();
                                }
                            }
                        }
                    }
                }
            }
        });
    }

    /// Recieve mpris events.
    pub async fn recv(&mut self) -> FumResult<FumResult<MprisEvent>> {
        self.receiver
            .recv()
            .await
            .context("Failed to receive mpris event")
    }

    /// Gets the shared mpris connection.
    fn connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.connection)
    }

    /// Gets the cloned mpris options.
    fn options(&self) -> Option<MprisOptions<T>> {
        self.options.clone()
    }

    /// Gets the shared active players.
    pub fn players(&self) -> Arc<Mutex<Vec<MprisPlayer>>> {
        Arc::clone(&self.players)
    }
}

/// My shitty fix of automatically inferring the T generic.
/// If u know a better way to this pls help.
/// Ill give u like the $2 on my bank account.
impl Mpris<Vec<&'static str>> {
    pub async fn new_without_options() -> FumResult<Self> {
        Self::new(None).await
    }
}
