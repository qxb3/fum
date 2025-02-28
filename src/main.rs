use std::error::Error;

use mpris::Mpris;

mod mpris;

type FumResult<T> = Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> FumResult<()> {
    let mpris = Mpris::new().await?;

    let players = mpris.players().await?;

    if let Some(spotify) = players.get("org.mpris.MediaPlayer2.spotify") {
        println!("{:#?}", spotify.bus_name);
    }

    Ok(())
}
