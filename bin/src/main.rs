#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use clap::Parser;
use eyre::Result;
use rika_args::commands::{App, Commands};
use rika_ops as ops;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = App::parse();
    execute(args)
}

fn execute(args: App) -> Result<()> {
    match args.command {
        Commands::Tx(args) => ops::transaction::get(args)?,
        Commands::TxCount(args) => ops::transaction::count(args)?,
        Commands::TxStatus(args) => ops::transaction::status(args)?,
        Commands::Receipt(args) => ops::transaction::receipt(args)?,
        Commands::Rpc(args) => ops::rpc::send(args)?,

        _ => {
            unimplemented!("This command is not implemented yet")
        }
    }

    Ok(())
}
