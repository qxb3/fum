mod cli;
mod cover;
mod event;
mod fum;
mod mode;
mod mpris;
mod player;
mod script;
mod state;
mod status;
mod track;
mod ui;
mod utils;
mod widget;

use fum::Fum;

/// Type alias for global Result.
type FumResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> FumResult<()> {
    // None variant signifies that we shouldn't start fum tui.
    if let Some(args) = cli::run().await? {
        let mut fum = Fum::new(&args).await?;
        fum.start(args.mode).await?;
    }

    Ok(())
}
