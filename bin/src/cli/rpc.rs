use color_eyre::Result;
pub use rika_args::commands::rpc::RpcCommands;
use rika_ops as ops;

pub fn execute(command: RpcCommands) -> Result<()> {
    match command {
        RpcCommands::Balance(args) => ops::rpc::balance::get(args)?,
        // RpcCommands::Call(args) => ops::rpc::call::call(args)?,
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
