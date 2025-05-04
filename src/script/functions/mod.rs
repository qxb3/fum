mod config;
pub use config::*;

mod layout;
pub use layout::*;

mod label;
pub use label::*;

use rhai::EvalAltResult;

/// Shorthand for Result<T, Box<EvalAltResult>>
type ScriptFnResult<T> = Result<T, Box<EvalAltResult>>;
