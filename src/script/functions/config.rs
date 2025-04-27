use crate::script::ScriptConfig;

use super::ScriptFnResult;

/// CONFIG() function to set the config.
pub fn config(config_state: ScriptConfig) -> impl Fn(rhai::Map) -> ScriptFnResult<()> {
    move |opts: rhai::Map| -> ScriptFnResult<()> {
        // Extract fps from opts.
        let fps = opts
            .get("fps")
            .unwrap_or(&rhai::Dynamic::from_int(10))
            .as_int()
            .map_err(|_| "Config `fps` needs to be a valid number")?;

        // Acquire lock for the config state.
        let mut config = config_state
            .lock()
            .map_err(|err| format!("Failed to acquire lock for the config state: {err}"))?;

        // Update config values.
        config.fps = fps as u64;

        Ok(())
    }
}
