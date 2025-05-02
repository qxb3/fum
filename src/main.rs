#[allow(dead_code)]
mod mpris;
mod status;

/// Type alias for global Result.
type FumResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> FumResult<()> {
    println!("HAHAHAHAHAHAHAHAHAHAH");

    Ok(())
}
