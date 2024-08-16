use crate::cli::Cli;
use crate::cli::Commands;

use clap::Parser;

mod cli;

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Login) => println!("Performing login..."),
        None => {}
    }
}