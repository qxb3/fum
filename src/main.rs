mod cli;
mod fum;
mod meta;
mod ui;
mod utils;
mod config;

use std::process;

use fum::Fum;

fn main() {
    let config = match cli::run() {
        Ok(config) => config,
        Err(err) => {
            eprintln!("[ERR] -> {}.", err);
            process::exit(1);
        }
    };

    let mut fum = match Fum::new(&config) {
        Ok(fum) => fum,
        Err(err) => {
            eprintln!("[ERR] -> {}.", err);
            process::exit(1);
        }
    };

    fum.run();
}
