mod cli;
mod fum;
mod meta;
mod ui;
mod utils;
mod config;
mod term_config;

use std::process;

use fum::Fum;
use term_config::TermConfig;

fn main() {
    let config = match cli::run() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("[CONFIG ERR] -> {}.", err);
            process::exit(1);
        }
    };

    let term_config = TermConfig::from_config(&config);

    let mut fum = match Fum::new(&config, &term_config) {
        Ok(fum) => fum,
        Err(err) => {
            eprintln!("[CONFIG ERR] -> {}.", err);
            process::exit(1);
        }
    };

    fum.run();
}
