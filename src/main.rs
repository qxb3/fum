mod event;
mod fum;
mod state;
mod terminal;

use fum::Fum;

/// Global Result type.
pub type FumResult<T> = anyhow::Result<T>;
pub type FumErr = anyhow::Error;

#[tokio::main]
async fn main() -> FumResult<()> {
    let mut fum = Fum::new()?;
    fum.run().await?;

    Ok(())
}
