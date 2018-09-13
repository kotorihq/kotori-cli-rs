extern crate clap;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate url;

use std::process::exit;

mod config;
mod cli;
mod commands;

fn main() {
    match cli::main() {
        Ok(_) => {}

        Err(e) => {
            if cfg!(debug_assertions) {
                panic!("error: {:?}", e)
            } else {
                eprintln!("error: {}", e);
                exit(1)
            }
        }
    };
}
