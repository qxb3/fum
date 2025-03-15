use std::error::Error;

use fum::Fum;

mod cli;
mod event;
mod fum;
mod mode;
mod mpris;
mod state;
mod track;
mod ui;

/// Type alias for Result.
type FumResult<T> = Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> FumResult<()> {
    // None variant signifies that we shouldn't start fum tui.
    if let Some((mode,)) = cli::run().await? {
        let mut fum = Fum::new().await?;
        fum.start(mode).await?;
    }

    Ok(())
}
