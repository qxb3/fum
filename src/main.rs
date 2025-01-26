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

fn main() -> FumResult<()> {
    let config = cli::run()?;
    let mut fum = Fum::new(&config)?;

    fum.run()?;

    Ok(())
}
