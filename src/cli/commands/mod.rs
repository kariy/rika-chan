pub mod opts;
mod rpc;

pub use rpc::RpcArgs;

use self::opts::StarkNetOptions;
use crate::cli::parser::{BlockIdParser, FieldElementParser};

use clap::{Parser, Subcommand};
use starknet::{core::types::FieldElement, providers::jsonrpc::models::BlockId};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct App {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(name = "--to-hex")]
    #[clap(about = "Convert decimal felt to hexadecimal.")]
    DecToHex {
        #[clap(value_name = "DECIMAL")]
        #[clap(value_parser(FieldElementParser))]
        dec: FieldElement,
    },

    #[clap(name = "--from-ascii")]
    #[clap(about = "Convert from ASCII to Cairo short string.")]
    FromAscii {
        #[clap(value_name = "ASCII")]
        #[clap(value_parser(FieldElementParser))]
        ascii: FieldElement,
    },

    #[clap(name = "--to-dec")]
    #[clap(about = "Convert hexadecimal felt to decimal.")]
    HexToDec {
        #[clap(value_name = "HEX")]
        #[clap(value_parser(FieldElementParser))]
        hex: FieldElement,
    },

    #[clap(name = "--max-felt")]
    #[clap(about = "Get the maximum felt value.")]
    MaxUnsignedFelt,

    #[clap(name = "--max-sfelt")]
    #[clap(about = "Get the maximum signed felt value.")]
    MaxSignedFelt,

    #[clap(name = "--min-sfelt")]
    #[clap(about = "Get the minimum signed felt value.")]
    MinSignedFelt,

    #[clap(name = "--to-ascii")]
    #[clap(about = "Convert Cairo short string to its ASCII format.")]
    ToAscii {
        #[clap(value_name = "SHORT_STRING")]
        short_str: String,
    },

    #[clap(about = "Perform ECDSA operations over the STARK-friendly elliptic curve.")]
    Ecdsa {
        #[clap(subcommand)]
        commands: EcdsaCommand,
    },

    #[clap(about = "Hash abritrary data using StarkNet keccak.")]
    Keccak {
        #[clap(value_name = "DATA")]
        data: String,
    },

    #[clap(about = "Calculate the Pedersen hash on two field elements.")]
    Pedersen {
        #[clap(value_name = "X")]
        x: String,
        #[clap(value_name = "Y")]
        y: String,
    },

    #[clap(name = "tx")]
    #[clap(about = "Get information about a transaction.")]
    Transaction {
        #[clap(value_name = "TX_HASH")]
        #[clap(value_parser(FieldElementParser))]
        hash: FieldElement,

        #[clap(long)]
        field: Option<String>,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(name = "tx-status")]
    #[clap(about = "Get the status of a transaction.")]
    TransactionStatus {
        #[clap(value_name = "TX_HASH")]
        #[clap(value_parser(FieldElementParser))]
        hash: FieldElement,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(name = "receipt")]
    #[clap(about = "Get the receipt of a transaction.")]
    TransactionReceipt {
        #[clap(value_name = "TX_HASH")]
        #[clap(value_parser(FieldElementParser))]
        hash: FieldElement,

        #[clap(long)]
        field: Option<String>,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(about = "Get the StarkNet chain ID.")]
    ChainId {
        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(about = "Get information about a block.")]
    Block {
        #[clap(value_name = "BLOCK_ID")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(help = "Can be a hash (0x...), number (1, 2), or tags (latest, pending).")]
        id: BlockId,

        #[clap(long)]
        #[clap(action(clap::ArgAction::SetTrue))]
        #[clap(help = "Get the full information (incl. transactions) of the block.")]
        full: bool,

        #[clap(long)]
        field: Option<String>,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(about = "Get the latest block number.")]
    BlockNumber {
        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(about = "Get the timestamp of a block.")]
    Age {
        #[clap(value_name = "BLOCK_ID")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(help = "Can be a hash (0x...), number (1, 2), or tags (latest, pending).")]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(about = "Get the latest nonce associated with the address.")]
    Nonce {
        #[clap(value_parser(FieldElementParser))]
        contract_address: FieldElement,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(name = "tx-pending")]
    #[clap(about = "Get the transactions in the transaction pool, recognized by the sequencer.")]
    PendingTransactions {
        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(name = "tx-count")]
    #[clap(about = "Get the number of transactions in a block.")]
    CountTransactions {
        #[clap(value_name = "BLOCK_ID")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(help = "Can be a hash (0x...), number (1, 2), or tags (latest, pending).")]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(about = "Get the value of a contract's storage at the given index")]
    Storage {
        #[clap(value_parser(FieldElementParser))]
        contract_address: FieldElement,

        #[clap(value_parser(FieldElementParser))]
        index: FieldElement,

        #[clap(short, long = "block")]
        #[clap(value_name = "BLOCK_ID")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(help = "Can be a hash (0x...), number (1, 2), or tags (latest, pending)")]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(about = "Perform a raw JSON-RPC request.")]
    Rpc(RpcArgs),

    #[clap(about = "Call a StarkNet function without creating a transaction.")]
    Call {
        #[clap(short = 'C', long)]
        #[clap(display_order = 1)]
        contract_address: FieldElement,

        #[clap(short, long)]
        #[clap(display_order = 2)]
        #[clap(help = "The name of the function to be called")]
        #[clap(value_name = "FUNCTION_NAME")]
        function: String,

        #[clap(short, long)]
        #[clap(multiple_values = true)]
        #[clap(display_order = 3)]
        inputs: Vec<FieldElement>,

        #[clap(short, long)]
        #[clap(display_order = 4)]
        #[clap(help = "Path to the contract's abi file to validate the call inputs")]
        abi: Option<String>,

        #[clap(display_order = 5)]
        #[clap(short, long = "block")]
        #[clap(value_name = "BLOCK_ID")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(help = "Can be a hash (0x...), number (1, 2), or tags (latest, pending)")]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(about = "Get the information about the result of executing the requested block")]
    StateUpdate {
        #[clap(short, long = "block")]
        #[clap(value_name = "BLOCK_ID")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(help = "Can be a hash (0x...), number (1, 2), or tags (latest, pending)")]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(about = "Compute the address of a storage variable.")]
    Index {
        #[clap(value_name = "VAR_NAME")]
        variable_name: String,

        keys: Vec<FieldElement>,
    },

    #[clap(about = "Compute the hash of a StarkNet contract.")]
    ContractHash {
        #[clap(help = "The compiled contract file")]
        contract: String,
    },

    #[clap(about = "Estimate the fee for a given StarkNet transaction.")]
    #[clap(
        long_about = "Estimates the resources required by a transaction relative to a given state."
    )]
    Estimate {
        #[clap(short = 'C', long)]
        #[clap(display_order = 1)]
        contract_address: FieldElement,

        #[clap(short, long)]
        #[clap(display_order = 2)]
        function_name: String,

        #[clap(short, long)]
        #[clap(display_order = 3)]
        #[clap(multiple_values = true)]
        calldata: Vec<FieldElement>,

        #[clap(short, long = "block")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(help = "Can be a hash (0x...), number (1, 2), or tags (latest, pending)")]
        #[clap(display_order = 4)]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(
        about = "Get the contract class definition in the given block associated with the given hash"
    )]
    Class {
        #[clap(value_name = "CLASS_HASH")]
        #[clap(help = "The hash of the requested contract class")]
        hash: FieldElement,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },

    #[clap(
        about = "Get the contract class hash in the given block for the contract deployed at the given address"
    )]
    ContractClass {
        #[clap(help = "The address of the contract whose class hash will be returned")]
        contract_address: FieldElement,

        #[clap(short, long = "block")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(help = "Can be a hash (0x...), number (1, 2), or tags (latest, pending)")]
        block_id: BlockId,

        #[clap(flatten)]
        #[clap(next_help_heading = "STARKNET OPTIONS")]
        starknet: StarkNetOptions,
    },
}

#[derive(Subcommand, Debug)]
pub enum EcdsaCommand {
    #[clap(about = "Sign a message.")]
    Sign {
        #[clap(short, long)]
        #[clap(value_name = "MESSAGE_HASH")]
        #[clap(value_parser(FieldElementParser))]
        #[clap(help = "Message hash to be signed.")]
        message: FieldElement,

        #[clap(short, long)]
        #[clap(value_name = "PRIVATE_KEY")]
        #[clap(value_parser(FieldElementParser))]
        #[clap(help = "The private key for signing.")]
        private_key: FieldElement,
    },

    #[clap(about = "Verify the signature of a message.")]
    Verify {
        #[clap(short, long)]
        #[clap(value_name = "MESSAGE_HASH")]
        #[clap(value_parser(FieldElementParser))]
        #[clap(help = "Message hash used in the signature.")]
        message: FieldElement,

        #[clap(short, long)]
        #[clap(required = true)]
        #[clap(number_of_values = 2)]
        #[clap(value_names = &["SIGNATURE_R", "SIGNATURE_S"])]
        #[clap(value_parser(FieldElementParser))]
        signature: Vec<FieldElement>,

        #[clap(short, long)]
        #[clap(value_name = "VERIFYING_KEY")]
        #[clap(value_parser(FieldElementParser))]
        #[clap(help = "The key for verification.")]
        verifying_key: FieldElement,
    },
}
