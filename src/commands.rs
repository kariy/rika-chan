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

    #[clap(about = "Get information about a transaction.")]
    Transaction {
        hash: String,

        field: Option<String>,

        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        rpc_url: Option<String>,
    },

    #[clap(about = "Get the StarkNet chain ID.")]
    ChainId {
        #[clap(long)]
        #[clap(value_name = "URL")]
        #[clap(env = "STARKNET_RPC_URL")]
        rpc_url: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum EcdsaCommand {
    #[clap(about = "Sign a message.")]
    Sign {
        #[clap(long)]
        #[clap(value_name = "PRIVATEKEY")]
        // #[clap(conflicts_with = "account-dir")]
        #[clap(help = "Specify a private key for signing.")]
        private_key: Option<String>,

        #[clap(value_name = "MESSAGE")]
        #[clap(help = "Message hash to be signed.")]
        message: String,
        //

        // #[clap(long)]
        // #[clap(value_name = "NAME")]
        // #[clap(help = "Use an account from the StarkNet keystore.")]
        // account: Option<String>,

        // #[clap(long)]
        // #[clap(value_name = "DIR")]
        // #[clap(requires = "account")]
        // #[clap(default_value = "~/.starknet_accounts")]
        // #[clap(help = "The directory containing the StarkNet keystore.")]
        // account_dir: String,
    },

    #[clap(about = "Verify the signature of a message.")]
    Verify {
        #[clap(long)]
        #[clap(value_name = "PUBLICKEY")]
        // #[clap(conflicts_with = "account-dir")]
        #[clap(help = "Specify a public key for verification.")]
        public_key: Option<String>,

        #[clap(value_name = "MESSAGE")]
        #[clap(help = "Message hash used in the signature.")]
        message: String,

        #[clap(help = "R value of the signature.")]
        signature_r: String,

        #[clap(help = "S value of the signature.")]
        signature_s: String,
        //

        // #[clap(long)]
        // #[clap(value_name = "ACCOUNTNAME")]
        // #[clap(help = "Use an account from the StarkNet keystore.")]
        // account: Option<String>,

        // #[clap(long)]
        // #[clap(value_name = "DIR")]
        // #[clap(requires = "account")]
        // #[clap(default_value = "~/.starknet_accounts")]
        // #[clap(help = "The directory containing the StarkNet keystore.")]
        // account_dir: String,
        // #[clap(long)]
        // #[clap(required = true)]
        // #[clap(number_of_values = 2)]
        // #[clap(help = "ECDSA signature, r and s.")]
        // #[clap(value_names = &["R", "S"])]
        // signature: Vec<String>,
    },
}
