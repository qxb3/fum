use clap::Parser;
use expanduser::expanduser;

use crate::{config::{Align, Config}, fum::FumResult};

#[derive(Parser)]
#[command(name = "fum", version, about)]
struct FumCli {
    #[arg(short, long, value_name = "config file", default_value = "~/.config/fum/config.jsonc")]
    config: Option<String>,

    #[arg(short, long, value_name = "string[]", value_delimiter = ',')]
    players: Option<Vec<String>>,

    #[arg(long, value_name = "boolean")]
    use_active_player: Option<bool>,

    #[arg(long, value_name = "number")]
    fps: Option<i32>,

    #[arg(short, long, value_name = "center,top,left,bottom,right,top-left,top-right,bottom-left,bottom-right")]
    align: Option<String>
}

pub fn run() -> FumResult<Config> {
    let fum_cli = FumCli::parse();

    let config_path = expanduser(&fum_cli.config.unwrap())
        .map_err(|err| format!("Failed to expand path: {err}"))?;

    let mut config = Config::load(&config_path)?;

    if let Some(players) = fum_cli.players.as_ref() {
        config.players = players.to_owned();
    }

    if let Some(use_active_player) = fum_cli.use_active_player.as_ref() {
        config.use_active_player = use_active_player.to_owned();
    }

    if let Some(fps) = fum_cli.fps.as_ref() {
        config.fps = fps.to_owned();
    }

    if let Some(align) = fum_cli.align.as_ref() {
        let align = Align::from_str(align.as_str())
            .ok_or("Invalid value for 'align'".to_string())?;

        config.align = align;
    }

    Ok(config)
}
