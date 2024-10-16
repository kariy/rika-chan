#![cfg_attr(not(test), warn(unused_crate_dependencies))]

mod cli;

use clap::Parser;
use cli::Cli;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    match cli.execute() {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
