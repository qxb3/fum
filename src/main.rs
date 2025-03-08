use std::error::Error;

mod mpris;

/// Type alias for Result.
type FumResult<T> = Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> FumResult<()> {
    Ok(())
}
