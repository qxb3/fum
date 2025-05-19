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
    if let Some((config_path, run_mode)) = cli::run().await? {
        let mut fum = Fum::new(config_path).await?;
        fum.run(run_mode).await?;
    }

    Ok(())
}
