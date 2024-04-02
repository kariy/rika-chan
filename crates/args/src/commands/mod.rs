pub mod account;
pub mod balance;
pub mod block;
pub mod call;
pub mod chain;
pub mod class;
pub mod contract;
pub mod events;
pub mod rpc;
pub mod send;
pub mod state_update;
pub mod transaction;
pub mod utility;

use self::account::AccountArgs;
use self::balance::BalanceArgs;
use self::block::{AgeArgs, BlockArgs, BlockNumberArgs};
use self::call::CallArgs;
use self::chain::{ChainIdArgs, SyncingArgs};
use self::class::{ClassArgs, CodeArgs};
use self::contract::{ContractClassArgs, NonceArgs, StorageArgs};
use self::events::EventsArgs;
use self::rpc::RpcArgs;
use self::state_update::StateUpdateArgs;
use self::transaction::{ReceiptArgs, TxArgs, TxCountArgs, TxStatusArgs};
use self::utility::UtilityCommands;

use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(name = "rika", version, about, long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(name = "completions")]
    #[command(visible_alias = "com")]
    #[command(about = "Generate command completion script for a specific shell.")]
    ShellCompletions { shell: Option<Shell> },

    #[command(flatten)]
    Utilities(UtilityCommands),

    #[command(visible_alias = "acc")]
    #[command(about = "Account management utilities")]
    Account(AccountArgs),

    #[command(about = "Get the timestamp of a block.")]
    Age(AgeArgs),

    #[command(visible_alias = "bal")]
    #[command(about = "Get an ERC20 token balance of an address.")]
    Balance(BalanceArgs),

    #[command(visible_alias = "b")]
    #[command(about = "Get information about a block.")]
    Block(BlockArgs),

    #[command(visible_alias = "bn")]
    #[command(about = "Get the latest block number.")]
    BlockNumber(BlockNumberArgs),

    #[command(about = "Call a StarkNet function without creating a transaction.")]
    Call(CallArgs),

    #[command(visible_alias = "ci")]
    #[command(about = "Get the StarkNet chain ID.")]
    ChainId(ChainIdArgs),

    #[command(visible_alias = "cl")]
    #[command(
        about = "Get the contract class definition in the given block associated with the given hash"
    )]
    Class(ClassArgs),

    #[command(visible_alias = "cd")]
    #[command(about = "Get the contract class definition in the given block at the given address")]
    Code(CodeArgs),

    #[command(visible_alias = "cc")]
    #[command(
        about = "Get the contract class hash in the given block for the contract deployed at the given address"
    )]
    ContractClass(ContractClassArgs),

    #[command(visible_alias = "ev")]
    #[command(about = "Returns all events matching the given filter")]
    #[command(
        long_about = "Returns all event objects matching the conditions in the provided filter"
    )]
    Events(EventsArgs),

    #[command(visible_alias = "n1")]
    #[command(about = "Get the latest nonce associated with the address.")]
    Nonce(NonceArgs),

    #[command(about = "Perform a raw JSON-RPC request.")]
    Rpc(RpcArgs),

    #[command(about = "Get the information about the result of executing the requested block")]
    StateUpdate(StateUpdateArgs),

    #[command(visible_alias = "str")]
    #[command(about = "Get the value of a contract's storage at the given index")]
    Storage(StorageArgs),

    #[command(visible_alias = "sync")]
    #[command(about = "Get the synchronization status of the StarkNet node")]
    Syncing(SyncingArgs),

    #[command(name = "tx")]
    #[command(about = "Get information about a transaction.")]
    Tx(TxArgs),

    #[command(visible_alias = "txc")]
    #[command(name = "tx-count")]
    #[command(about = "Get the number of transactions in a block.")]
    TxCount(TxCountArgs),

    #[command(visible_alias = "txs")]
    #[command(name = "tx-status")]
    #[command(about = "Get the status of a transaction.")]
    TxStatus(TxStatusArgs),

    #[command(visible_alias = "rct")]
    #[command(name = "receipt")]
    #[command(about = "Get the receipt of a transaction.")]
    Receipt(ReceiptArgs),
}
