mod parser;

use self::parser::{BlockIdParser, FieldElementParser};

use clap::{ArgGroup, Parser, Subcommand};
use starknet::{core::types::FieldElement, providers::jsonrpc::models::BlockId};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct App {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(name = "--address-zero")]
    #[clap(about = "Get StarkNet zero address.")]
    AddressZero,

    #[clap(name = "--to-hex")]
    #[clap(about = "Convert decimal felt to hexadecimal.")]
    DecToHex {
        #[clap(value_name = "DECIMAL")]
        #[clap(value_parser(FieldElementParser))]
        dec: FieldElement,
    },

    #[clap(name = "--from-utf8")]
    #[clap(about = "Convert felt to utf-8 short string.")]
    FromUtf8 {
        #[clap(value_name = "FELT")]
        #[clap(value_parser(FieldElementParser))]
        felt: FieldElement,
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

    #[clap(name = "--max-signed-felt")]
    #[clap(about = "Get the maximum signed felt value.")]
    MaxSignedFelt,

    #[clap(name = "--min-signed-felt")]
    #[clap(about = "Get the minimum signed felt value.")]
    MinSignedFelt,

    #[clap(name = "--str-to-felt")]
    #[clap(about = "Convert short string to felt decimal. (String whose length is < 31)")]
    StrToFelt {
        #[clap(value_name = "SHORTSTRING")]
        str: String,
    },

    #[clap(about = "Perform ECDSA related operations.")]
    Ecdsa {
        #[clap(subcommand)]
        commands: EcdsaCommand,
    },

    #[clap(about = "Hash abritrary data using StarkNet keccak.")]
    Keccak {
        #[clap(value_name = "DATA")]
        data: String,
    },

    #[clap(about = "Calculate the Pedersen hash on two field elements, (x,y)")]
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

        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
    },

    #[clap(name = "tx-status")]
    #[clap(about = "Get the status of a transaction.")]
    TransactionStatus {
        #[clap(value_name = "TX_HASH")]
        #[clap(value_parser(FieldElementParser))]
        hash: FieldElement,

        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
    },

    #[clap(name = "receipt")]
    #[clap(about = "Get the receipt of a transaction.")]
    TransactionReceipt {
        #[clap(value_name = "TX_HASH")]
        #[clap(value_parser(FieldElementParser))]
        hash: FieldElement,

        #[clap(long)]
        field: Option<String>,

        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
    },

    #[clap(about = "Get the StarkNet chain ID.")]
    ChainId {
        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
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

        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
    },

    #[clap(about = "Get the latest block number.")]
    BlockNumber {
        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
    },

    #[clap(about = "Get the timestamp of a block.")]
    Age {
        #[clap(value_name = "BLOCK_ID")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(help = "Can be a hash (0x...), number (1, 2), or tags (latest, pending).")]
        id: BlockId,

        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
    },

    #[clap(about = "Get the latest nonce associated with the address.")]
    Nonce {
        #[clap(value_parser(FieldElementParser))]
        contract_address: FieldElement,

        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
    },

    #[clap(name = "tx-pending")]
    #[clap(about = "Get the transactions in the transaction pool, recognized by the sequencer.")]
    PendingTransactions {
        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
    },

    #[clap(name = "tx-count")]
    #[clap(about = "Get the number of transactions in a block.")]
    CountTransactions {
        #[clap(value_name = "BLOCK_ID")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(help = "Can be a hash (0x...), number (1, 2), or tags (latest, pending).")]
        id: BlockId,

        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
    },

    #[clap(about = "Get the value of the storage at the given address and key")]
    Storage {
        #[clap(value_parser(FieldElementParser))]
        contract_address: FieldElement,

        #[clap(value_parser(FieldElementParser))]
        key: FieldElement,

        #[clap(long)]
        #[clap(value_name = "BLOCK_HASH")]
        #[clap(conflicts_with = "number")]
        #[clap(value_parser(FieldElementParser))]
        hash: Option<FieldElement>,

        #[clap(long)]
        #[clap(value_name = "BLOCK_NUMBER")]
        number: Option<u64>,

        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
    },

    #[clap(about = "Perform a raw JSON-RPC request.")]
    #[clap(group(ArgGroup::new("params-src").required(true).args(&["params", "file"])))]
    Rpc {
        #[clap(help = "RPC method name")]
        method: String,

        #[clap(long)]
        #[clap(group = "params-src")]
        #[clap(help = "RPC parameters")]
        params: Option<Vec<String>>,

        #[clap(long)]
        #[clap(group = "params-src")]
        #[clap(help = "Get RPC parameters from a file")]
        file: Option<String>,

        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
    },

    Call {
        #[clap(value_name = "FUNCTION_NAME")]
        function_name: String,

        #[clap(short, long)]
        #[clap(help = "Path to the contract's abi file")]
        abi: String,

        #[clap(short, long)]
        #[clap(multiple_values = true)]
        inputs: Vec<FieldElement>,

        #[clap(short, long)]
        contract_address: FieldElement,

        #[clap(short, long)]
        #[clap(value_name = "BLOCK_ID")]
        #[clap(default_value = "latest")]
        #[clap(value_parser(BlockIdParser))]
        #[clap(help = "Can be a hash (0x...), number (1, 2), or tags (latest, pending)")]
        block_id: BlockId,

        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        #[clap(default_value = "http://localhost:5050/rpc")]
        rpc_url: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum EcdsaCommand {
    #[clap(about = "Sign a message.")]
    Sign {
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
        #[clap(value_name = "MESSAGE_HASH")]
        #[clap(value_parser(FieldElementParser))]
        #[clap(help = "Message hash used in the signature.")]
        message: FieldElement,

        #[clap(value_parser(FieldElementParser))]
        signature: Vec<FieldElement>,

        #[clap(short, long)]
        #[clap(value_name = "VERIFYING_KEY")]
        #[clap(value_parser(FieldElementParser))]
        #[clap(help = "The key for verification.")]
        verifying_key: FieldElement,
    },
}
