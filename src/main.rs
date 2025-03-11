use std::error::Error;

use fum::Fum;

mod event;
mod fum;
mod mpris;
mod state;
mod track;

/// Type alias for Result.
type FumResult<T> = Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> FumResult<()> {
    let mut fum = Fum::new()?;
    fum.start().await?;

    Ok(())
}
