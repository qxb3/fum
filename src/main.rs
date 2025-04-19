mod cli;
mod event;
mod fum;
mod mode;
mod mpris;
mod player;
mod script;
mod state;
mod track;
mod ui;
mod utils;
mod widget;

use std::error::Error;

use fum::Fum;

/// Type alias for Result.
type FumResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[tokio::main]
async fn main() -> FumResult<()> {
    // None variant signifies that we shouldn't start fum tui.
    if let Some(args) = cli::run().await? {
        let mut fum = Fum::new(&args).await?;
        fum.start(args.mode).await?;
    }

    Ok(())
}
