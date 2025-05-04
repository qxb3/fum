mod config;
pub use config::*;

use rhai::EvalAltResult;

/// Shorthand for Result<T, Box<EvalAltResult>>
type ScriptFnResult<T> = Result<T, Box<EvalAltResult>>;
