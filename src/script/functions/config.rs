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

        // New config with passed in opts.
        let config = Config { fps: fps as u64 };

        event_sender
            .send(Ok(Event::Script(ScriptEvent::ConfigUpdated(config))))
            .unwrap();
    }
}
