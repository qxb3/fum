mod cli;
mod fum;
mod state;
mod meta;
mod ui;
mod utils;
mod text;
mod widget;
mod config;
mod action;
mod regexes;

use fum::{Fum, FumResult};
use mpris::PlayerFinder;

fn main() -> FumResult<()> {
    let (cli, config) = cli::run()?;

    if let Some(command) = &cli.command {
        match command {
            cli::Commands::ListPlayers => {
                let player_finder = PlayerFinder::new()
                    .map_err(|err| format!("Failed to connect to D-Bus: {err}."))?;

                let players = player_finder
                    .find_all()
                    .map_err(|err| format!("There is no any active players: {err}."))?;

                println!("Active Players:");
                for player in players {
                    let identity = player.identity().to_lowercase();
                    let bus_name = player.bus_name();

                    println!("* {identity} ~> {bus_name}");
                }

                return Ok(());
            }
        }
    }

    Fum::new(&config)?
        .run()?;

    Ok(())
}
