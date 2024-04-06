#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use clap::Parser;
use eyre::Result;
use rika_args::commands::{App, Commands};
use rika_ops as ops;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = App::parse();

    match execute(args.command) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}

fn execute(command: Commands) -> Result<()> {
    match command {
        Commands::Call(args) => ops::call::call(args)?,
        Commands::Balance(args) => ops::balance::get(args)?,
        Commands::Tx(args) => ops::transaction::get(args)?,
        Commands::TxCount(args) => ops::transaction::count(args)?,
        Commands::TxStatus(args) => ops::transaction::status(args)?,
        Commands::Receipt(args) => ops::transaction::receipt(args)?,
        Commands::Rpc(args) => ops::rpc::send(args)?,
        Commands::Block(args) => ops::block::get(args)?,
        Commands::Age(args) => ops::block::age(args)?,
        Commands::BlockNumber(args) => ops::block::number(args)?,

        _ => {
            unimplemented!("This command is not implemented yet")
        }
    }

    Ok(())
}
