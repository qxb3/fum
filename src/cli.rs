use clap::{Parser, Subcommand};

use crate::{mpris::Mpris, FumResult};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Start fum in mp3 player mode.
    Player {},

    /// Start fum in mpris mode.
    Mpris {
        #[arg(short, long, value_name = "string[]", value_delimiter = ',')]
        players: Option<Vec<String>>,

        #[arg(long, value_name = "number")]
        fps: Option<u64>,
    },

    /// List out active players (alias: ls)
    #[command(alias = "ls")]
    ListPlayers,
}

/// Run the cli.
pub async fn run() -> FumResult<Option<()>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Player {} => {
            return Ok(Some(()));
        }

        Commands::Mpris {
            ..
        } => {
            return Ok(Some(()));
        }

        Commands::ListPlayers => {
            let mpris = Mpris::new().await?;
            let players = mpris.players().await?;

            println!("Active Players:");

            for (bus_name, _) in players.iter() {
                println!("* {bus_name}");
            }

            return Ok(None);
        }
    }
}
