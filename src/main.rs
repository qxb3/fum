mod cli;
mod fum;
mod meta;
mod ui;
mod utils;
mod widget;
mod config;
mod action;

use fum::{Fum, FumResult};

fn main() -> FumResult<()> {
    let config = cli::run()?;
    let mut fum = Fum::new(&config)?;

    fum.run()?;

    Ok(())
}
