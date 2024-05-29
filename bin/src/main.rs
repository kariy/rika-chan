#![cfg_attr(not(test), warn(unused_crate_dependencies))]

mod cli;

use self::cli::App;
use clap::Parser;
use eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = App::parse();

    match execute(args) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

fn execute(commands: App) -> Result<()> {
    match commands {
        App::Utilities(cmd) => cli::utilities::execute(cmd)?,

        #[cfg(feature = "rpc")]
        App::Rpc(rpc) => cli::rpc::execute(rpc)?,
    }

    Ok(())
}
