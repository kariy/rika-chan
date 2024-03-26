mod utils;

use clap::Parser;
use eyre::Result;
use rika_args::commands::{App, Commands};
use starknet::providers::Provider;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = App::parse();
    execute(args)
}

fn execute(args: App) -> Result<()> {
    match args.command {
        Commands::Receipt {
            hash,
            display,
            starknet,
        } => {
            let provider = starknet.provider();
            let result = utils::block_on(provider.get_transaction_receipt(hash))?;
            display.display(result)?;
        }

        Commands::StateUpdate {
            block_id,
            display,
            starknet,
        } => {
            let provider = starknet.provider();
            let result = utils::block_on(provider.get_state_update(block_id))?;
            // display.display(result)?;
        }

        Commands::Transaction {
            hash,
            display,
            starknet,
        } => {
            let provider = starknet.provider();
            let result = utils::block_on(provider.get_transaction_by_hash(hash))?;
            display.display(result)?;
        }

        _ => {
            unimplemented!("This command is not implemented yet")
        }
    }

    Ok(())
}
