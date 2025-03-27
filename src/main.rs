use std::error::Error;

use fum::Fum;
use script::Script;

mod cli;
mod event;
mod fum;
mod mode;
mod mpris;
mod script;
mod state;
mod track;
mod ui;
mod widget;

/// Type alias for Result.
type FumResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> FumResult<()> {
    // None variant signifies that we shouldn't start fum tui.
    if let Some(args) = cli::run().await? {
        let script = Script::new(&args.config_path)?;
        script.get_ui_widgets()?;

        // let mut fum = Fum::new(&args.config_path).await?;
        // fum.start(args.mode).await?;
    }

    Ok(())
}
