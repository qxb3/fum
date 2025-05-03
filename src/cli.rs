use clap::{command, Parser, Subcommand};
use mprizzle::{Mpris, MprisEvent};
use std::path::PathBuf;

use crate::FumResult;

#[derive(Debug)]
pub enum RunMode {
    Player,
    Mpris,
}

#[derive(Debug, Parser)]
struct Cli {
    /// The config path.
    #[arg(short, long, value_name = "config path", default_value = "~/.config/fum/config.rhai")]
    pub config: PathBuf,

    /// Command to execute.
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Start fum in mp3 player mode. (alias: pl)
    #[command(alias = "pl")]
    Player,

    /// Start fum in mpris mode. (alias: mp)
    #[command(alias = "mp")]
    Mpris,

    /// List out active players. (alias: ls)
    #[command(alias = "ls")]
    ListPlayers,
}

/// Runs the cli.
pub async fn run() -> FumResult<Option<(PathBuf, RunMode)>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Player => Ok(Some((cli.config, RunMode::Player))),
        Command::Mpris => Ok(Some((cli.config, RunMode::Mpris))),
        Command::ListPlayers => {
            let mut mpris = Mpris::new_without_options().await?;
            mpris.watch();

            while let Ok(event) = mpris.recv().await? {
                match event {
                    MprisEvent::PlayerAttached(identity) => {
                        println!("* {} ~> {}", identity.short(), identity.bus());
                    }
                    _ => {}
                }
            }

            Ok(None)
        }
    }
}
