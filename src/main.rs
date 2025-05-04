mod cli;
mod event;
mod fum;
mod script;
mod state;
mod terminal;

use fum::Fum;

/// Global Result type.
pub type FumResult<T> = anyhow::Result<T>;
pub type FumErr = anyhow::Error;

#[tokio::main]
async fn main() -> FumResult<()> {
    if let Some((config_path, run_mode)) = cli::run().await? {
        let mut fum = Fum::new()?;
        fum.run(config_path, run_mode).await?;
    }

    Ok(())
}
