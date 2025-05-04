use std::sync::Arc;

use anyhow::anyhow;
use futures::lock::Mutex;

use crate::{event::EventSender, FumResult};

/// Manages the mpris events.
pub struct Mpris<T: IntoIterator<Item = &'static str> + Sync + Send + Clone + 'static> {
    /// Internal mprizzle Mpris.
    mpris: Arc<Mutex<mprizzle::Mpris<T>>>,

    /// The centralize event manager sender.
    event_sender: EventSender,
}

impl<T: IntoIterator<Item = &'static str> + Sync + Send + Clone + 'static> Mpris<T> {
    pub async fn new(event_sender: EventSender, filter_players: T) -> FumResult<Self> {
        let mpris = mprizzle::Mpris::new(Some(mprizzle::MprisOptions { filter_players })).await?;

        Ok(Self {
            mpris: Arc::new(Mutex::new(mpris)),
            event_sender,
        })
    }

    /// Sends events into the centalized event thingy.
    pub fn send_events(&self) {
        let mpris = Arc::clone(&self.mpris);
        let event_sender = self.event_sender.clone();

        tokio::spawn(async move {
            let mut mpris = mpris.lock().await;
            mpris.watch();

            loop {
                let event = match mpris.recv().await {
                    Ok(event) => event,
                    Err(err) => {
                        event_sender
                            .send(Err(anyhow!("Failed to receive mpris event: {err}")))
                            .unwrap();

                        return;
                    }
                };

                match event {
                    Ok(event) => match event {
                        mprizzle::MprisEvent::PlayerAttached(identity) => {}
                        mprizzle::MprisEvent::PlayerDetached(identity) => {}
                        mprizzle::MprisEvent::PlayerPropertiesChanged(identity) => {}
                        mprizzle::MprisEvent::PlayerSeeked(identity) => {}
                        mprizzle::MprisEvent::PlayerPosition(identity, position) => {}
                    },
                    Err(err) => {
                        event_sender
                            .send(Err(anyhow!("Received a error event in mpris: {err}")))
                            .unwrap();

                        return;
                    }
                }
            }
        });
    }
}
