pub mod account;
pub mod balance;
pub mod block;
pub mod call;
pub mod chain;
pub mod class;
pub mod contract;
pub mod events;
pub mod raw;
pub mod send;
pub mod state_update;
pub mod transaction;

pub use account::*;
pub use balance::*;
pub use block::*;
pub use call::*;
pub use chain::*;
pub use class::*;
pub use contract::*;
pub use events::*;
pub use raw::*;
pub use send::*;
pub use state_update::*;
pub use transaction::*;

use clap::Subcommand;
use clap_complete::Shell;

#[derive(Subcommand, Debug)]
pub enum RpcCommands {
    /// Generate command completion script for a specific shell.
    #[command(name = "completions", visible_alias = "com")]
    ShellCompletions { shell: Option<Shell> },

    /// Get the timestamp of a block.
    Age(AgeArgs),

    /// Get an ERC20 token balance of an address.
    #[command(visible_alias = "bal")]
    Balance(BalanceArgs),

    /// Get information about a block.
    #[command(visible_alias = "b")]
    Block(BlockArgs),

    /// Get the latest block number.
    #[command(visible_alias = "bn")]
    BlockNumber(BlockNumberArgs),

    /// Call a StarkNet function without creating a transaction.
    Call(CallArgs),

    /// Get the StarkNet chain ID.
    #[command(visible_alias = "ci")]
    ChainId(ChainIdArgs),

    /// Get the contract class definition in the given block associated with the given hash
    #[command(visible_alias = "cl")]
    Class(ClassArgs),

    /// Get the contract class definition in the given block at the given address
    #[command(visible_alias = "cd")]
    Code(CodeArgs),

    /// Get the contract class hash in the given block for the contract deployed at the given address
    #[command(visible_alias = "cc")]
    ContractClass(ContractClassArgs),

    /// Returns all events matching the given filter
    ///
    /// Returns all event objects matching the conditions in the provided filter
    #[command(visible_alias = "ev")]
    Events(EventsArgs),

    /// Get the latest nonce associated with the address.
    #[command(visible_alias = "n1")]
    Nonce(NonceArgs),

    /// Perform a raw JSON-RPC request.
    Rpc(RawRpcArgs),

    /// Get the information about the result of executing the requested block
    StateUpdate(StateUpdateArgs),

    /// Get the value of a contract's storage at the given index
    #[command(visible_alias = "str")]
    Storage(StorageArgs),

    /// Get the synchronization status of the StarkNet node
    #[command(visible_alias = "sync")]
    Syncing(SyncingArgs),

    /// Get information about a transaction.
    #[command(name = "tx")]
    Tx(TxArgs),

    /// Get the number of transactions in a block.
    #[command(visible_alias = "txc")]
    #[command(name = "tx-count")]
    TxCount(TxCountArgs),

    /// Get the status of a transaction.
    #[command(visible_alias = "txs")]
    #[command(name = "tx-status")]
    TxStatus(TxStatusArgs),

    /// Get the receipt of a transaction.
    #[command(visible_alias = "rct")]
    #[command(name = "receipt")]
    Receipt(ReceiptArgs),
}
