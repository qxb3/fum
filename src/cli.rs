use clap::Parser;
use expanduser::expanduser;
use crate::config::Config;

#[derive(Parser)]
#[command(name = "fum", version, about)]
struct FumCli {
    #[arg(short, long, value_name = "json file", default_value = "~/.config/fum/config.json")]
    config: Option<String>,

    #[arg(short, long, value_name = "string[]", value_delimiter = ',')]
    players: Option<Vec<String>>,

    #[arg(long, value_name = "boolean")]
    use_active_player: Option<bool>,

    #[arg(short, long, value_name = "center,top,left,bottom,right,top-left,top-right,bottom-left,bottom-right")]
    align: Option<String>
}

pub fn run() -> Result<Config, String> {
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

    if let Some(align) = fum_cli.align.as_ref() {
        config.align = align.to_string();
    }

    if !matches!(
        config.align.as_str(),
        "center" | "top" | "left" |
        "bottom" | "right" | "top-left" |
        "top-right" | "bottom-left" | "bottom-right"
    ) {
        return Err("Invalid value for 'align'".to_string())
    }

    Ok(config)
}
