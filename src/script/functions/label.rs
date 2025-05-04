use super::ScriptFnResult;

/// raw function of LABEL().
pub fn label_function_raw() -> impl Fn(rhai::Map) -> ScriptFnResult<()> {
    move |opts: rhai::Map| -> ScriptFnResult<()> { todo!() }
}
