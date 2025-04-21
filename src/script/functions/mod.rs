use rhai::EvalAltResult;

/// Type alias for Result at script function calls.
pub type ScriptFnResult<T> = Result<T, Box<EvalAltResult>>;

mod button;
mod config;
mod container;
mod cover_image;
mod label;
mod player_controls;
mod slider;
mod ui;

pub use button::*;
pub use config::*;
pub use container::*;
pub use cover_image::*;
pub use label::*;
pub use player_controls::*;
pub use slider::*;
pub use ui::*;
