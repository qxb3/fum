/// Global Result type.
type FumResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> FumResult<()> {
    Ok(())
}
