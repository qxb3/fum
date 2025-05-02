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
    mpris.watch();

    loop {
        let event_result = mpris.recv().await?;

        match event_result {
            Ok(event) => match event {
                MprisEvent::PlayerAttached(identity) => println!("ATTACHED = {:?}", identity),
                MprisEvent::PlayerDetached(identity) => println!("DETACHED = {:?}", identity),
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
