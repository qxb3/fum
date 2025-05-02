// mod cli;
// mod cover;
// mod event;
// mod fum;
// mod mode;
mod mpris;
// mod player;
// mod script;
// mod state;
mod status;
// mod track;
// mod ui;
// mod utils;
// mod widget;

use mpris::{Mpris, MprisEvent};

// use fum::Fum;

/// Type alias for global Result.
type FumResult<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> FumResult<()> {
    let mut mpris = Mpris::new_without_options().await?;
    let players = mpris.players();

    mpris.watch();

    loop {
        let event_result = mpris.recv().await?;

        match event_result {
            Ok(event) => match event {
                MprisEvent::PlayerAttached(identity) => println!("ATTACHED = {:?}", identity),
                MprisEvent::PlayerDetached(identity) => println!("DETACHED = {:?}", identity),
                MprisEvent::PlayerPropertiesChanged(identity) => {
                    let players = players.try_lock()?;
                    if let Some(player) = players.iter().find(|p| *p.identity() == identity) {
                        println!("PLAYER_PROP_CHANGED = {:?}", player.metadata().await?)
                    }
                }
                MprisEvent::PlayerSeeked(identity) => println!("PLAYER_SEEKED = {:?}", identity),
                MprisEvent::PlayerPosition(identity, position) => {
                    println!("PLAYER_POSITION = {:?} {:?}", identity, position.as_secs())
                }
            },
            Err(err) => panic!("{:?}", err),
        }
    }

    // // None variant signifies that we shouldn't start fum tui.
    // if let Some(args) = cli::run().await? {
    //     let mut fum = Fum::new(&args).await?;
    //     fum.start(args.mode).await?;
    // }

    Ok(())
}
