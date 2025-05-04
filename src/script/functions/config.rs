use crate::{
    config::Config,
    event::{Event, EventSender, ScriptEvent},
};

use super::ScriptFnResult;

/// CONFIG().
pub fn config_function(event_sender: EventSender) -> impl Fn(rhai::Map) -> ScriptFnResult<()> {
    move |opts: rhai::Map| -> ScriptFnResult<()> {
        // Extract `fps` from opts.
        let fps = opts
            .get("fps")
            .unwrap_or(&rhai::Dynamic::from_int(10))
            .as_int()
            .map_err(|_| "Config `fps` needs to be a valid number")?;

        // New config with passed in opts.
        let config = Config { fps: fps as u64 };

        event_sender
            .send(Ok(Event::Script(ScriptEvent::ConfigUpdated(config))))
            .expect("Failed to send out event: ScriptEvent::ConfigUpdated");

        Ok(())
    }
}
