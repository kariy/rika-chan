use clap::Parser;
use eyre::Result;
use rika_args::commands::utility;
use rika_ops as ops;

#[derive(Parser, Debug)]
#[command(name = "rika", version, about, long_about = None)]
pub enum App {
    #[command(flatten)]
    Utilities(utility::UtilityCommands),

    #[cfg(feature = "rpc")]
    #[command(flatten)]
    Rpc(rpc::RpcCommands),
}

pub mod utilities {
    use super::*;
    use rika_args::commands::utility;

    pub fn execute(command: utility::UtilityCommands) -> Result<()> {
        match command {
            utility::UtilityCommands::Index(args) => ops::utility::storage_address(args)?,
            _ => unimplemented!("This command is not implemented yet"),
        }

        Ok(())
    }
}

#[cfg(feature = "rpc")]
pub mod rpc {
    use super::*;
    pub use rika_args::commands::rpc::RpcCommands;

    pub fn execute(command: RpcCommands) -> Result<()> {
        match command {
            // RpcCommands::Call(args) => ops::rpc::call::call(args)?,
            // RpcCommands::Balance(args) => ops::rpc::balance::get(args)?,
            // RpcCommands::Tx(args) => ops::rpc::transaction::get(args)?,
            // RpcCommands::TxCount(args) => ops::rpc::transaction::count(args)?,
            // RpcCommands::TxStatus(args) => ops::rpc::transaction::status(args)?,
            RpcCommands::Receipt(args) => ops::rpc::transaction::receipt(args)?,
            // RpcCommands::Rpc(args) => ops::rpc::raw::send(args)?,
            // RpcCommands::Block(args) => ops::rpc::block::get(args)?,
            // RpcCommands::Age(args) => ops::rpc::block::age(args)?,
            // RpcCommands::BlockNumber(args) => ops::rpc::block::number(args)?,
            // RpcCommands::ChainId(args) => ops::rpc::chain::id(args)?,
            // RpcCommands::Syncing(args) => ops::rpc::chain::syncing(args)?,
            _ => {
                unimplemented!("This command is not implemented yet")
            }
        }

        Ok(())
    }
}

#[cfg(feature = "account-mgmt")]
pub mod account {}

#[cfg(feature = "dojo")]
pub mod dojo {}

#[cfg(feature = "katana")]
pub mod katana {}
