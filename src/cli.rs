use anyhow::Context;
use clap::{command, Parser, Subcommand};
use mprizzle::{Mpris, MprisEvent};

use std::{env, path::PathBuf};

use crate::FumResult;

#[derive(Debug, Parser)]
pub struct Cli {
    /// The config path.
    #[arg(short, long, value_name = "config path", default_value = "~/.config/fum/config.rhai")]
    pub config: PathBuf,

    /// Command to execute.
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// List out active players. (alias: ls)
    #[command(alias = "ls")]
    ListPlayers,
}

/// Runs the cli.
pub async fn run() -> FumResult<Option<Cli>> {
    let mut cli = Cli::parse();

    // Expands the tilde if found one.
    // Shells already do this for you but if there is no
    // --config <path> that is specified it will use the `default_value`
    // and that won't expand.
    if cli.config.starts_with("~") {
        // Gets the $HOME path from envs.
        let home_path = env::var("HOME").context("Missing $HOME variable")?;

        // Strip the ~ from the config path.
        let stripped_path = cli
            .config
            .strip_prefix("~")
            .context("Expected that ~ to be stripped")?;

        // Updates the config from cli.
        cli.config = PathBuf::from(home_path).join(stripped_path);
    }

    if let Some(command) = &cli.command {
        match command {
            Command::ListPlayers => {
                let mut mpris = Mpris::new().await?;
                mpris.watch();

                while let Ok(event) = mpris.recv().await? {
                    match event {
                        MprisEvent::PlayerAttached(attached_player) => {
                            let attached_player_identity = attached_player.identity();

                            println!(
                                "* {} ~> {}",
                                attached_player_identity.short(),
                                attached_player_identity.bus()
                            );
                        }

                        _ => {}
                    }
                }

                return Ok(None);
            }
        }
    }

    Ok(Some(cli))
}
