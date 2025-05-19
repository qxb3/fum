use anyhow::anyhow;

use crate::{
    config::Config,
    event::{Event, EventSender, ScriptEvent},
};

/// CONFIG().
pub fn config_function(event_sender: EventSender) -> impl Fn(rhai::Map) {
    move |opts: rhai::Map| {
        // Extract `fps` from opts.
        let fps = match opts
            .get("fps")
            .unwrap_or(&rhai::Dynamic::from_int(10))
            .as_int()
        {
            Ok(fps) => fps as u64,
            Err(_) => {
                event_sender
                    .send(Err(anyhow!("Config `fps` needs to be a valid number")))
                    .unwrap();

                return;
            }
        };

        // Extract `players` from opts.
        let players_opt = match opts.get("players").cloned() {
            Some(players_opt) => players_opt,
            None => {
                event_sender
                    .send(Err(anyhow!("Config needs to have `players`")))
                    .unwrap();

                return;
            }
        };

        // Make sure that `players` is an array.
        let players = match players_opt.try_cast_result::<rhai::Array>() {
            Ok(players) => players,
            Err(_) => {
                event_sender
                    .send(Err(anyhow!("Config `players` needs to be a array")))
                    .unwrap();

                return;
            }
        };

        // Parse players.
        let mut filter_players = Vec::new();
        for player in players {
            let player = match player.try_cast_result::<String>() {
                Ok(player) => player,
                Err(_) => {
                event_sender
                    .send(Err(anyhow!("Config `players` needs to be a array of string")))
                    .unwrap();

                    return;
                }
            };

            filter_players.push(player);
        }

        // New config with passed in opts.
        let config = Config {
            fps: fps as u64,
            filter_players
        };

        event_sender
            .send(Ok(Event::Script(ScriptEvent::ConfigUpdated(config))))
            .unwrap();
    }
}
