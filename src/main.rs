mod cli;
mod doctor;
mod error;
mod generator;
mod templates;

use clap::Parser;
use cli::Cli;
use std::process;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = cli.execute() {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
