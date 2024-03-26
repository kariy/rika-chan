use super::account::WalletCommands;
use super::parser::BlockIdParser;
use super::rpc::RpcArgs;
use crate::opts::{display::DisplayOptions, starknet::StarknetOptions};
use crate::utils::parse_event_keys;

use clap::{Parser, Subcommand};
use clap_complete::Shell;
use starknet::core::types::{BlockId, FieldElement};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rika", version, about, long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(visible_alias = "th")]
    #[command(name = "--to-hex")]
    #[command(about = "Convert decimal felt to hexadecimal.")]
    DecToHex {
        #[arg(value_name = "DECIMAL")]
        dec: FieldElement,

        #[arg(long)]
        #[arg(help = "Pad the resulting hex value to 32 bytes.")]
        pad: bool,
    },

    #[command(visible_alias = "td")]
    #[command(name = "--to-dec")]
    #[command(about = "Convert hexadecimal felt to decimal.")]
    HexToDec {
        #[arg(value_name = "HEX")]
        hex: FieldElement,
    },

    #[command(visible_alias = "mxf")]
    #[command(name = "--max-felt")]
    #[command(about = "Get the maximum felt value.")]
    MaxUnsignedFelt,

    #[command(visible_alias = "mxsf")]
    #[command(name = "--max-sfelt")]
    #[command(about = "Get the maximum signed felt value.")]
    MaxSignedFelt,

    #[command(visible_alias = "mnsf")]
    #[command(name = "--min-sfelt")]
    #[command(about = "Get the minimum signed felt value.")]
    MinSignedFelt,

    #[command(visible_alias = "fa")]
    #[command(name = "--from-ascii")]
    #[command(about = "Convert from ASCII to Cairo short string.")]
    FromAscii {
        #[arg(value_name = "ASCII")]
        ascii: FieldElement,
    },

    #[command(visible_alias = "ta")]
    #[command(name = "--to-ascii")]
    #[command(about = "Convert Cairo short string to its ASCII format.")]
    ToAscii {
        #[arg(value_name = "SHORT_STRING")]
        short_str: String,
    },

    #[command(visible_alias = "su")]
    #[command(name = "--split-u256")]
    #[command(about = "Split a uint256 into its low and high components.")]
    SplitU256 { value: String },

    #[command(visible_alias = "acc")]
    #[command(about = "Account management utilities")]
    Account {
        #[command(subcommand)]
        commands: WalletCommands,
    },

    #[command(about = "Get the timestamp of a block.")]
    Age {
        #[arg(next_line_help = true)]
        #[arg(default_value = "latest")]
        #[arg(value_parser(BlockIdParser))]
        #[arg(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[arg(short = 'r', long)]
        #[arg(help_heading = "Display options")]
        human_readable: bool,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "bal")]
    #[command(about = "Get the ETH balance of an address.")]
    Balance {
        #[arg(value_name = "ADDRESS")]
        #[arg(help = "The address whose balance you want to query.")]
        address: FieldElement,

        #[arg(next_line_help = true)]
        #[arg(short, long = "block")]
        #[arg(default_value = "latest")]
        #[arg(value_parser(BlockIdParser))]
        #[arg(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "b")]
    #[command(about = "Get information about a block.")]
    Block {
        #[arg(next_line_help = true)]
        #[arg(value_name = "BLOCK_ID")]
        #[arg(default_value = "latest")]
        #[arg(value_parser(BlockIdParser))]
        #[arg(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        id: BlockId,

        #[arg(long)]
        #[arg(action(clap::ArgAction::SetTrue))]
        #[arg(help = "Get the full information (incl. transactions) of the block.")]
        full: bool,

        #[arg(long)]
        field: Option<String>,

        #[arg(short = 'j', long = "json")]
        #[arg(help_heading = "Display options")]
        to_json: bool,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "bn")]
    #[command(about = "Get the latest block number.")]
    BlockNumber {
        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(about = "Call a StarkNet function without creating a transaction.")]
    Call {
        #[arg(display_order = 1)]
        contract_address: FieldElement,

        #[arg(display_order = 2)]
        #[arg(help = "The name of the function to be called")]
        #[arg(value_name = "FUNCTION_NAME")]
        function: String,

        #[arg(short, long)]
        #[arg(display_order = 3)]
        #[arg(value_delimiter = ',')]
        #[arg(help = "Comma seperated values e.g., 0x12345,0x69420,...")]
        input: Vec<FieldElement>,

        #[arg(next_line_help = true)]
        #[arg(display_order = 5)]
        #[arg(short, long = "block")]
        #[arg(default_value = "latest")]
        #[arg(value_parser(BlockIdParser))]
        #[arg(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "ci")]
    #[command(about = "Get the StarkNet chain ID.")]
    ChainId {
        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "cl")]
    #[command(
        about = "Get the contract class definition in the given block associated with the given hash"
    )]
    Class {
        #[arg(value_name = "CLASS_HASH")]
        #[arg(help = "The hash of the requested contract class")]
        hash: FieldElement,

        #[arg(next_line_help = true)]
        #[arg(default_value = "latest")]
        #[arg(value_parser(BlockIdParser))]
        #[arg(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "cd")]
    #[command(about = "Get the contract class definition in the given block at the given address")]
    Code {
        #[arg(help = "The address of the contract whose class definition will be returned")]
        contract_address: FieldElement,

        #[arg(next_line_help = true)]
        #[arg(short, long = "block")]
        #[arg(default_value = "latest")]
        #[arg(value_parser(BlockIdParser))]
        #[arg(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "ca")]
    #[command(about = "Compute the contract address from the given information")]
    ComputeAddress {
        #[arg(help = "The address of the deploying account contract (currently always zero)")]
        caller_address: FieldElement,

        #[arg(help = "The salt used in the deploy transaction")]
        salt: FieldElement,

        #[arg(help = "The hash of the class to instantiate a new contract from")]
        class_hash: FieldElement,

        #[arg(help = "The inputs passed to the constructor")]
        calldata: Vec<FieldElement>,
    },

    #[command(visible_alias = "cc")]
    #[command(
        about = "Get the contract class hash in the given block for the contract deployed at the given address"
    )]
    ContractClass {
        #[arg(help = "The address of the contract whose class hash will be returned")]
        contract_address: FieldElement,

        #[arg(next_line_help = true)]
        #[arg(short, long = "block")]
        #[arg(default_value = "latest")]
        #[arg(value_parser(BlockIdParser))]
        #[arg(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "ch")]
    #[command(about = "Compute the hash of a contract class.")]
    ClassHash {
        #[arg(help = "Path to the contract artifact file")]
        contract: PathBuf,
    },

    #[command(visible_alias = "cch")]
    #[command(about = "Compute the compiled class hash of a Sierra contract class.")]
    CompiledClassHash {
        #[arg(help = "Path to the Sierra contract artifact file")]
        contract: PathBuf,
    },

    #[command(visible_alias = "ec")]
    #[command(about = "Perform ECDSA operations over the STARK-friendly elliptic curve.")]
    Ecdsa {
        #[command(subcommand)]
        commands: EcdsaCommand,
    },

    #[command(visible_alias = "ev")]
    #[command(about = "Returns all events matching the given filter")]
    #[command(
        long_about = "Returns all event objects matching the conditions in the provided filter"
    )]
    Events {
        #[arg(num_args(0..))]
        #[arg(help = r"The values used to filter the events.
Example: 0x12,0x23 0x34,0x45 - Which will be parsed as [[0x12,0x23], [0x34,0x45]]")]
        #[arg(value_parser = parse_event_keys)]
        keys: Option<Vec<Vec<FieldElement>>>,

        #[arg(required = true)]
        #[arg(short = 's', long)]
        #[arg(help = "The number of events to return in each page")]
        chunk_size: u64,

        #[arg(short = 'C', long)]
        #[arg(value_name = "CONTRACT_ADDRESS")]
        #[arg(help = "Address of the contract emitting the events")]
        from: Option<FieldElement>,

        #[arg(short, long)]
        #[arg(value_parser(BlockIdParser))]
        from_block: Option<BlockId>,

        #[arg(short, long)]
        #[arg(value_parser(BlockIdParser))]
        to_block: Option<BlockId>,

        #[arg(short = 'c', long)]
        #[arg(
            help = "A pointer to the last element of the delivered page, use this token in a subsequent query to obtain the next page"
        )]
        continuation_token: Option<String>,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "idx")]
    #[command(about = "Compute the address of a storage variable.")]
    Index {
        #[arg(value_name = "VAR_NAME")]
        variable_name: String,

        keys: Vec<FieldElement>,
    },

    #[command(visible_alias = "kck")]
    #[command(about = "Hash abritrary data using StarkNet keccak.")]
    Keccak {
        #[arg(value_name = "DATA")]
        data: String,
    },

    #[command(visible_alias = "n1")]
    #[command(about = "Get the latest nonce associated with the address.")]
    Nonce {
        contract_address: FieldElement,

        #[arg(next_line_help = true)]
        #[arg(default_value = "latest")]
        #[arg(value_parser(BlockIdParser))]
        #[arg(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "ped")]
    #[command(about = "Calculate the Pedersen hash on two field elements.")]
    Pedersen {
        #[arg(help = "List of elements to compute the hash on.")]
        elements: Vec<FieldElement>,
    },

    #[command(about = "Perform a raw JSON-RPC request.")]
    Rpc(RpcArgs),

    #[command(name = "completions")]
    #[command(visible_alias = "com")]
    #[command(about = "Generate command completion script for a specific shell.")]
    ShellCompletions { shell: Option<Shell> },

    #[command(about = "Get the information about the result of executing the requested block")]
    StateUpdate {
        #[arg(next_line_help = true)]
        #[arg(short, long = "block")]
        #[arg(default_value = "latest")]
        #[arg(value_parser(BlockIdParser))]
        #[arg(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "str")]
    #[command(about = "Get the value of a contract's storage at the given index")]
    Storage {
        contract_address: FieldElement,

        index: FieldElement,

        #[arg(next_line_help = true)]
        #[arg(short, long = "block")]
        #[arg(default_value = "latest")]
        #[arg(value_parser(BlockIdParser))]
        #[arg(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "sync")]
    #[command(about = "Get the synchronization status of the StarkNet node")]
    Syncing {
        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(name = "tx")]
    #[command(about = "Get information about a transaction.")]
    Transaction {
        #[arg(value_name = "TX_HASH")]
        hash: FieldElement,

        #[arg(long)]
        field: Option<String>,

        #[arg(short = 'j', long = "json")]
        #[arg(help_heading = "Display options")]
        to_json: bool,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "txc")]
    #[command(name = "tx-count")]
    #[command(about = "Get the number of transactions in a block.")]
    TransactionCount {
        #[arg(next_line_help = true)]
        #[arg(default_value = "latest")]
        #[arg(value_parser(BlockIdParser))]
        #[arg(
            help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
        )]
        block_id: BlockId,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "txs")]
    #[command(name = "tx-status")]
    #[command(about = "Get the status of a transaction.")]
    TransactionStatus {
        #[arg(value_name = "TX_HASH")]
        hash: FieldElement,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "rct")]
    #[command(name = "receipt")]
    #[command(about = "Get the receipt of a transaction.")]
    TransactionReceipt {
        #[arg(value_name = "TX_HASH")]
        hash: FieldElement,

        #[command(flatten)]
        #[command(next_help_heading = "Display options")]
        display: DisplayOptions,

        #[command(flatten)]
        #[command(next_help_heading = "Starknet options")]
        starknet: StarknetOptions,
    },

    #[command(visible_alias = "gca")]
    #[command(about = "Generate call array calldata")]
    CallArray {
        #[arg(required = true)]
        #[arg(value_delimiter = ' ')]
        #[arg(help = r#"List of calls seperated with a hyphen, -
        example : <contract address> <function name> [<calldata> ...] - ..."#)]
        calls: Vec<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum EcdsaCommand {
    #[command(about = "Sign a message.")]
    Sign {
        #[arg(short, long)]
        #[arg(value_name = "MESSAGE_HASH")]
        #[arg(help = "Message hash to be signed.")]
        message: FieldElement,

        #[arg(short, long)]
        #[arg(value_name = "PRIVATE_KEY")]
        #[arg(help = "The private key for signing.")]
        private_key: FieldElement,
    },

    #[command(about = "Verify the signature of a message.")]
    Verify {
        #[arg(short, long)]
        #[arg(value_name = "MESSAGE_HASH")]
        #[arg(help = "Message hash used in the signature.")]
        message: FieldElement,

        #[arg(short, long)]
        #[arg(required = true)]
        #[arg(number_of_values = 2)]
        #[arg(value_names = &["SIGNATURE_R", "SIGNATURE_S"])]
        signature: Vec<FieldElement>,

        #[arg(short, long)]
        #[arg(value_name = "VERIFYING_KEY")]
        #[arg(help = "The key for verification.")]
        verifying_key: FieldElement,
    },
}

#[cfg(test)]
mod tests {
    use crate::commands::Commands;

    use super::App;
    use clap::{CommandFactory, Parser};
    use starknet::core::types::FieldElement;

    #[test]
    fn verify_cli() {
        App::command().debug_assert()
    }

    #[test]
    fn parse_event_keys() {
        let expected_keys = vec![
            vec![FieldElement::from(0x1234u64), FieldElement::from(0x12u64)],
            vec![FieldElement::from(0x6666u64), FieldElement::from(0x7777u64)],
        ];

        let app = App::parse_from([
            "rika",
            "events",
            "--chunk-size",
            "2",
            "0x1234,0x12",
            "0x6666,0x7777",
        ]);

        match app.command {
            Commands::Events { keys, .. } => {
                assert_eq!(keys, Some(expected_keys));
            }
            _ => panic!("wrong command"),
        }
    }
}
