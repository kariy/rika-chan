use std::path::PathBuf;

use clap::{Parser, Subcommand};
use starknet::core::types::FieldElement;

#[derive(Subcommand, Debug)]
pub enum UtilityCommands {
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

    #[command(visible_alias = "ec")]
    #[command(about = "Perform ECDSA operations over the STARK-friendly elliptic curve.")]
    Ecdsa {
        #[command(subcommand)]
        commands: EcdsaCommand,
    },

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

    #[command(visible_alias = "kck")]
    #[command(about = "Hash abritrary data using Starknet keccak.")]
    Keccak {
        #[arg(value_name = "DATA")]
        data: String,
    },

    #[command(visible_alias = "ped")]
    #[command(about = "Calculate the Pedersen hash on an array of elements.")]
    Pedersen {
        #[arg(help = "List of elements to compute the hash on.")]
        elements: Vec<FieldElement>,
    },

    #[command(visible_alias = "pos")]
    #[command(about = "Calculate the Poseidon hash on an array of elements.")]
    Poseidon {
        #[arg(help = "List of elements to compute the hash on.")]
        elements: Vec<FieldElement>,
    },

    #[command(visible_alias = "idx")]
    #[command(about = "Compute the address of a storage variable.")]
    Index(IndexArgs),

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

#[derive(Debug, Parser)]
pub struct IndexArgs {
    #[arg(value_name = "VARIABLE_NAME")]
    #[arg(help = "The storage variable name.")]
    pub var_name: String,

    #[arg(help = "The storage keys")]
    pub keys: Vec<FieldElement>,
}
