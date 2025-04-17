use std::{env, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::{mode::FumModes, mpris::Mpris, FumResult};

/// Fum cli.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Config path.
    #[arg(short, long, value_name = "path")]
    pub config: Option<PathBuf>,

    /// Executed command.
    #[command(subcommand)]
    command: Command,
}

/// Fum available commands.
#[derive(Subcommand, Debug)]
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

/// Cli arguments.
pub struct CliArgs {
    pub config_path: PathBuf,
    pub mode: FumModes,
}

/// Run the cli.
pub async fn run() -> FumResult<Option<CliArgs>> {
    let mut cli = Cli::parse();

    // If config path is not specified use default path.
    if cli.config.is_none() {
        let config_path = get_config_path()?;
        cli.config = Some(config_path);
    }

    // Its ok to unwrap here since the above.
    let config_path = cli.config.unwrap();

    match cli.command {
        Command::Player => Ok(Some(CliArgs {
            config_path,
            mode: FumModes::Player,
        })),

        Command::Mpris => Ok(Some(CliArgs {
            config_path,
            mode: FumModes::Mpris,
        })),

        Command::ListPlayers => {
            let mpris = Mpris::new().await?;
            let players = mpris.players().await?;

            println!("Active Players:");

            if players.is_empty() {
                println!("* No active players.");
            }

            for (_, player) in players.iter() {
                println!("* {} ~> {}", &player.identity, &player.bus_name);
            }

            Ok(None)
        }
    }
}

/// A utility function to get the config path of fum.
/// If `$XDG_CONFIG_HOME` exists it will use that, otherwise
/// it will fallback to using `~/.config/fum`.
fn get_config_path() -> FumResult<PathBuf> {
    if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
        Ok(PathBuf::from(xdg_config_home).join("fum/config.rhai"))
    } else if let Ok(home) = env::var("HOME") {
        Ok(PathBuf::from(home).join(".config/fum/config.rhai"))
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Could not determine fum config path",
        )))
    }
}
