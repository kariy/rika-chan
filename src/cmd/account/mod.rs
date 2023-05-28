pub mod simple_account;

use super::{account::simple_account::SimpleWallet, parser::PathParser};
use crate::opts::account::{utils::get_main_keystore_dir, WalletOptions};
use crate::probe::utils::parse_hex_or_str_as_felt;

use std::path::PathBuf;
use std::str::FromStr;

use clap::{ArgGroup, Subcommand};
use eyre::Result;
use inquire::{Password, Select, Text};
use starknet::core::types::FieldElement;
use walkdir::WalkDir;

#[derive(Debug, Subcommand)]
pub enum WalletCommands {
    #[command(about = "Create a keystore for a StarkNet account.")]
    #[command(group(ArgGroup::new("new-raw").args(&["path"]).requires_all(&["account", "privatekey", "password", "chain"])))]
    New {
        #[arg(long)]
        #[arg(value_name = "PATH")]
        #[arg(value_parser(PathParser))]
        #[arg(help = "If provided, then keypair will be written to an encrypted JSON keystore.")]
        path: Option<PathBuf>,

        #[arg(long)]
        #[arg(value_name = "ACCOUNT_ADDRESS")]
        #[arg(
            help = "Address of the StarkNet account contract you want to create a keystore for."
        )]
        account: Option<FieldElement>,

        #[arg(long)]
        #[arg(requires = "path")]
        #[arg(value_name = "PRIVATE_KEY")]
        #[arg(help = "The raw private key associated with the account contract.")]
        privatekey: Option<FieldElement>,

        #[arg(long)]
        #[arg(requires = "path")]
        #[arg(value_name = "CHAIN")]
        chain: Option<FieldElement>,

        #[arg(long)]
        #[arg(requires = "path")]
        #[arg(value_name = "KEYSTORE_NAME")]
        #[arg(help = "A name to identify the keystore with.")]
        name: Option<String>,

        #[arg(long)]
        #[arg(requires = "path")]
        #[arg(value_name = "KEYSTORE_PASSWORD")]
        #[arg(help = "Provide the password for the JSON keystore in cleartext.")]
        password: Option<String>,
    },

    #[command(about = "Sign a message using an account's signing key.")]
    #[command(group(ArgGroup::new("sign-raw").args(&["keystore"]).requires_all(&["password", "message"])))]
    Sign {
        #[arg(short, long)]
        #[arg(value_name = "PATH")]
        #[arg(value_parser(PathParser))]
        keystore: Option<PathBuf>,

        #[arg(short, long)]
        #[arg(requires = "keystore")]
        #[arg(value_name = "KEYSTORE_PASSWORD")]
        #[arg(help = "Provide the password for the JSON keystore in cleartext.")]
        password: Option<String>,

        #[arg(short, long)]
        #[arg(requires = "keystore")]
        #[arg(value_name = "MESSAGE_HASH")]
        #[arg(help = "The hash of the message you want to sign.")]
        message: Option<String>,
    },
}

impl WalletCommands {
    pub async fn run(self) -> Result<()> {
        match self {
            Self::New {
                path,
                account,
                privatekey,
                chain,
                name,
                password,
            } => {
                if let Some(path) = path {
                    if !path.is_dir() {
                        // we require path to be an existing directory
                        eprintln!("`{}` is not a directory.", path.display());
                        std::process::exit(1)
                    }

                    let wallet = SimpleWallet::new(account.unwrap(), privatekey.unwrap(), chain);
                    wallet.encrypt_keystore(&path, password.unwrap(), name)?;

                    println!(
                        "\n🎉 Successfully created new encrypted keystore at {}.\n\nAccount: {:#X}\nPrivate key: {:#X}\nChain: {}",
                        path.display(),
                        wallet.account,
                        wallet.signing_key.secret_scalar(),
                        wallet.chain_id.map_or_else(|| "".to_string(), |c| c.to_string())
                    );
                } else {
                    let wallet = WalletOptions {
                        interactive: true,
                        ..Default::default()
                    };

                    let mut wallet = wallet.interactive()?.unwrap();
                    let options = vec!["mainnet", "testnet", "testnet2"];
                    let chain = Select::new("Please select the chain for this account.", options)
                        .prompt()?;

                    wallet.chain_id = Some(FieldElement::from_str(chain)?);

                    let name = Text::new("Enter account name : ").prompt()?;
                    let password = Password::new("Enter keystore password : ").prompt()?;

                    wallet.encrypt_keystore(get_main_keystore_dir(), password, Some(name))?;

                    println!(
                        "\n🎉 Created new encrypted keystore.\n\nAccount: {:#X}\nPrivate key: {:#X}\nChain: {}",
                        wallet.account,
                        wallet.signing_key.secret_scalar(),
                        wallet.chain_id.map_or_else(|| "".to_string(), |c| c.to_string())
                    );
                }

                Ok(())
            }

            Self::Sign {
                keystore: path,
                password,
                message,
            } => {
                // construct a SimpleAccount from the keystore
                // `path` must be the encrypted keystore json file
                if let Some(path) = path {
                    let wallet = SimpleWallet::decrypt_keystore(path, password.unwrap())?;
                    let hash = parse_hex_or_str_as_felt(message.as_ref().unwrap())?;
                    let sig = wallet.signing_key.sign(&hash)?;
                    println!("{:#x} {:#x}", sig.r, sig.s);
                } else {
                    let chain = Select::new("Select chain", vec!["mainnet", "testnet", "testnet2"])
                        .prompt()?;

                    let mut keystores_path: Vec<String> = Vec::new();

                    let path = format!("~/.starknet/keystore/{chain}");
                    let path = shellexpand::tilde(&path);

                    for entry in WalkDir::new(path.as_ref()) {
                        let file = entry?;
                        if file.file_type().is_file() {
                            keystores_path.push(file.into_path().to_str().unwrap().to_string());
                        }
                    }

                    let keystore = Select::new("Select keystore", keystores_path).prompt()?;
                    let password = Password::new("Enter keystore password :").prompt()?;
                    let account = SimpleWallet::decrypt_keystore(keystore, password)?;

                    let message = Text::new("Enter message to sign : ").with_help_message("Message with 0x prefix is treated as hex value otherwise literal string").prompt()?;

                    let hash = parse_hex_or_str_as_felt(&message)?;
                    let sig = account.signing_key.sign(&hash)?;

                    println!("\n{:#x} {:#x}", sig.r, sig.s);
                }

                Ok(())
            }
        }
    }
}
