extern crate clap;
extern crate failure;
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate url;

mod cli;
mod commands;

fn main() {
    match cli::main() {
        Ok(_) => {}
        Err(e) => panic!("error: {:?}", e)
    };
}
