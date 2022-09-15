use clap::{ArgAction, ArgGroup, Parser, Subcommand};

/// A test tool to try out clap
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct App {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[clap(name = "--address-zero", about = "Get StarkNet zero address.")]
    AddressZero,

    #[clap(name = "--max-felt", about = "Get the maximum felt value.")]
    UnsignedFeltMax,

    #[clap(
        name = "--max-signed-felt",
        about = "Get the maximum signed felt value."
    )]
    SignedFeltMax,

    #[clap(
        name = "--min-signed-felt",
        about = "Get the minimum signed felt value."
    )]
    SignedFeltMin,

    #[clap(
        name = "--str-to-felt",
        about = "Convert short string to felt decimal."
    )]
    StrToFelt {
        #[clap(value_name = "SHORTSTRING")]
        str: String,
    },

    #[clap(name = "--to-utf8", about = "Convert felt to utf-8 short string.")]
    FeltToStr {
        #[clap(value_name = "FELT")]
        felt: String,
    },

    #[clap(name = "--hex-to-dec", about = "Convert hexadecimal felt to decimal.")]
    HexToDec {
        #[clap(value_name = "HEXDATA")]
        hex: String,
    },

    #[clap(name = "--dec-to-hex", about = "Convert decimal felt to hexadecimal.")]
    DecToHex {
        #[clap(value_name = "DECIMAL")]
        dec: String,
        #[clap(short, long, action = ArgAction::SetTrue)]
        padding: bool,
        #[clap(short, long, action = ArgAction::SetTrue)]
        upper: bool,
    },

    #[clap(about = "Get the function selector from its name.")]
    Selector {
        #[clap(value_name = "FUNCTION_NAME")]
        function_name: String,
    },

    #[clap(about = "Calculate the StarkNet keccak of the given data.")]
    Keccak {
        #[clap(value_name = "DATA")]
        data: String,
    },
    // Uint256ToCairoUint256 {

    // }
    // #[clap(about = "Get StarkNet block based on its hash or block number.")]
    // #[clap(group(ArgGroup::new("block_id").required(true).args(&["hash", "number"])))]
    // GetBlock {
    //     #[clap(long, value_name = "HASH")]
    //     hash: Option<String>,
    //     #[clap(long, value_name = "NUMBER")]
    //     number: Option<String>,
    // },
}
