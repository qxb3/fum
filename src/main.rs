mod cli;
mod config;
mod event;
mod fum;
mod mpris;
mod script;
mod state;
mod terminal;
mod track;
mod utils;
mod widget;

use fum::Fum;

/// Global Result type.
pub type FumResult<T> = anyhow::Result<T>;
pub type FumErr = anyhow::Error;

#[tokio::main]
async fn main() -> FumResult<()> {
    if let Some(config) = cli::run().await? {
        Fum::new(config).await?.run().await?;
    }

    Ok(())
}
