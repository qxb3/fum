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
            eprintln!("[CONFIG ERR] -> {}.", err);
            process::exit(1);
        }
    };

    println!("{:#?}", config);

    // let mut fum = match Fum::new(&config, &term_config) {
    //     Ok(fum) => fum,
    //     Err(err) => {
    //         eprintln!("[CONFIG ERR] -> {}.", err);
    //         process::exit(1);
    //     }
    // };
    //
    // fum.run();
}
