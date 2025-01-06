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

    #[arg(short, long, value_name = "center,top,left,bottom,right,top-left,top-right,bottom-left,bottom-right")]
    align: Option<String>,

    #[arg(short, long, value_name = "top-to-bottom,left-to-right,bottom-to-top,right-to-left")]
    layout: Option<String>,

    #[arg(long, value_name = "string[]", value_delimiter = ',')]
    hidden: Option<Vec<String>>,

    #[arg(long, value_name = "char")]
    progress: Option<char>,

    #[arg(long, value_name = "char")]
    empty: Option<char>,
}

pub fn run() -> Result<Config, String> {
    let fum_cli = FumCli::parse();

    let config_path = expanduser(&fum_cli.config.unwrap())
        .map_err(|err| format!("Failed to expand path: {err}"))?;

    let mut config = Config::load(&config_path)?;

    if let Some(players) = fum_cli.players.as_ref() {
        config.players = players.to_owned();
    }

    if let Some(align) = fum_cli.align.as_ref() {
        config.align = align.to_string();
    }

    if let Some(layout) = fum_cli.layout.as_ref() {
        config.layout = layout.to_string();
    }

    if let Some(hidden) = fum_cli.hidden.as_ref() {
        config.hidden = hidden.to_owned();
    }

    if let Some(progress) = fum_cli.progress.as_ref() {
        config.progress = progress.to_owned();
    }

    if let Some(empty) = fum_cli.empty.as_ref() {
        config.empty = empty.to_owned();
    }

    if !matches!(
        config.align.as_str(),
        "center" | "top" | "left" |
        "bottom" | "right" | "top-left" |
        "top-right" | "bottom-left" | "bottom-right"
    ) {
        return Err("Invalid value for 'align'".to_string())
    }

    if !matches!(
        config.layout.as_str(),
        "top-to-bottom" | "bottom-to-top" |
        "left-to-right" | "right-to-left"
    ) {
        return Err("Invalid value for 'layout'".to_string())
    }

    for hidden in &config.hidden {
        if !matches!(
            hidden.as_str(),
            "title" | "artists" | "buttons" |
            "progress-bar" | "progress-text"
        ) {
            return Err(format!("Invalid values for 'hidden'. value: '{hidden}' is not allowed"))
        }
    }

    Ok(config)
}
