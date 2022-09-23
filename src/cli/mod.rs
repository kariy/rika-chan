pub mod commands;

use self::commands::ecdsa;

use clap::{Parser, Subcommand};

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
        dec: String,
    },

    #[clap(name = "--from-utf8")]
    #[clap(about = "Convert felt to utf-8 short string.")]
    FromUtf8 {
        #[clap(value_name = "FELT")]
        felt: String,
    },

    #[clap(name = "--to-dec")]
    #[clap(about = "Convert hexadecimal felt to decimal.")]
    HexToDec {
        #[clap(value_name = "HEX")]
        hex: String,
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
        commands: ecdsa::EcdsaCommand,
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
}
