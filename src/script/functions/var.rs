use crate::script::{ScriptEvent, ScriptVars};

use super::ScriptFnResult;

// DEF_VAR() function to define a persistent variable.
pub fn define_var(
    vars: ScriptVars,
) -> impl Fn(rhai::Dynamic, rhai::Dynamic) -> ScriptFnResult<()> {
    move |var_name: rhai::Dynamic, value: rhai::Dynamic| -> ScriptFnResult<()> {
        // Turn the var_name dynamic into String.
        let var_name = var_name
            .into_string()
            .map_err(|_| "SET_VAR() variable name needs to be a string")?;

        // Acquire lock for vars state.
        let mut vars = vars
            .lock()
            .map_err(|err| format!("Failed to acquire lock for vars: {err}"))?;

        // Update the vars state.
        if !vars.contains_key(&var_name) {
            vars.insert(var_name, value);
        }

        Ok(())
    }
}

// SET_VAR() function to set a persistent variable.
pub fn set_var(
    vars: ScriptVars,
    script_tx: tokio::sync::mpsc::UnboundedSender<ScriptEvent>,
) -> impl Fn(rhai::Dynamic, rhai::Dynamic) -> ScriptFnResult<()> {
    move |var_name: rhai::Dynamic, value: rhai::Dynamic| -> ScriptFnResult<()> {
        // Turn the var_name dynamic into String.
        let var_name = var_name
            .into_string()
            .map_err(|_| "SET_VAR() variable name needs to be a string")?;

        // Acquire lock for vars state.
        let mut vars = vars
            .lock()
            .map_err(|err| format!("Failed to acquire lock for vars: {err}"))?;

        // Update the vars state.
        vars.insert(var_name, value);

        // Send out the SetVar script event.
        script_tx.send(ScriptEvent::SetVar).unwrap();

        Ok(())
    }
}

// GET_VAR() function get a variable from persistent vars state.
pub fn get_var(
    vars: ScriptVars,
) -> impl Fn(rhai::Dynamic) -> ScriptFnResult<rhai::Dynamic> {
    move |var_name: rhai::Dynamic| -> ScriptFnResult<rhai::Dynamic> {
        // Turn the var_name dynamic into String.
        let var_name = var_name
            .into_string()
            .map_err(|_| "SET_VAR() variable name needs to be a string")?;

        // Acquire lock for vars state.
        let vars = vars
            .lock()
            .map_err(|err| format!("Failed to acquire lock for vars: {err}"))?;

        // Get the value.
        let value = vars
            .get(&var_name)
            .cloned()
            .ok_or(format!("There is no persistent variable named: {var_name}"))?;

        Ok(value)
    }
}
