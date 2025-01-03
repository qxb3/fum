use clap::Parser;
use expanduser::expanduser;
use crate::config::Config;

#[derive(Parser)]
#[command(name = "fum", version, about)]
struct FumCli {
    #[arg(short, long, value_name = "json file", default_value = "~/.config/fum/config.json")]
    config: Option<String>,

    #[arg(short, long, value_name = "string", value_delimiter = ',')]
    players: Option<Vec<String>>,

    #[arg(long, value_name = "number")]
    width: Option<u16>,

    #[arg(long, value_name = "number")]
    height: Option<u16>,
}

pub fn run() -> Result<Config, String> {
    let fum_cli = FumCli::parse();

    let config_path = expanduser(&fum_cli.config.unwrap())
        .map_err(|err| format!("Failed to expand path: {err}"))?;

    let mut config = Config::load(&config_path)?;

    if let Some(players) = fum_cli.players.as_ref() {
        config.players = players.to_owned();
    }

    if let Some(width) = fum_cli.width.as_ref() {
        config.width = width.to_owned();
    }

    if let Some(height) = fum_cli.height.as_ref() {
        config.height = height.to_owned();
    }

    Ok(config)
}
