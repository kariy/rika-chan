pub mod simple_account;

use super::{account::simple_account::SimpleAccount, parser::PathParser};
use crate::opts::account::{utils::get_main_keystore_dir, WalletOptions};
use crate::opts::starknet::StarknetChain;
use crate::probe::utils::parse_hex_or_str_as_felt;

use std::path::PathBuf;

use clap::{ArgGroup, Subcommand};
use eyre::Result;
use inquire::{Password, Select, Text};
use starknet::{core::types::FieldElement, signers::Signer};
use walkdir::WalkDir;

#[derive(Debug, Subcommand)]
pub enum WalletCommands {
    #[clap(about = "Create a new keystore.")]
    #[clap(group(ArgGroup::new("new-raw").args(&["path"]).requires_all(&["account", "privatekey", "password", "chain"])))]
    New {
        #[clap(long)]
        #[clap(value_name = "PATH")]
        #[clap(value_parser(PathParser))]
        #[clap(help = "If provided, then keypair will be written to an encrypted JSON keystore.")]
        path: Option<PathBuf>,

        #[clap(long)]
        #[clap(value_name = "ACCOUNT_ADDRESS")]
        #[clap(
            help = "Address of the StarkNet account contract you want to create a keystore for."
        )]
        account: Option<FieldElement>,

        #[clap(long)]
        #[clap(requires = "path")]
        #[clap(value_name = "PRIVATE_KEY")]
        #[clap(help = "The raw private key associated with the account contract.")]
        privatekey: Option<FieldElement>,

        #[clap(long)]
        #[clap(requires = "path")]
        #[clap(value_name = "CHAIN")]
        chain: Option<StarknetChain>,

        #[clap(long)]
        #[clap(requires = "path")]
        #[clap(value_name = "KEYSTORE_NAME")]
        #[clap(help = "A name to identify the keystore with.")]
        name: Option<String>,

        #[clap(long)]
        #[clap(requires = "path")]
        #[clap(value_name = "KEYSTORE_PASSWORD")]
        #[clap(help = "Provide the password for the JSON keystore in cleartext.")]
        password: Option<String>,
    },

    #[clap(about = "Sign a message using an account's signing key.")]
    #[clap(group(ArgGroup::new("sign-raw").args(&["keystore"]).requires_all(&["password", "message"])))]
    Sign {
        #[clap(short, long)]
        #[clap(value_name = "PATH")]
        #[clap(value_parser(PathParser))]
        keystore: Option<PathBuf>,

        #[clap(short, long)]
        #[clap(requires = "keystore")]
        #[clap(value_name = "KEYSTORE_PASSWORD")]
        #[clap(help = "Provide the password for the JSON keystore in cleartext.")]
        password: Option<String>,

        #[clap(short, long)]
        #[clap(requires = "keystore")]
        #[clap(value_name = "MESSAGE_HASH")]
        #[clap(help = "The hash of the message you want to sign.")]
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

                    let account = SimpleAccount::new(account.unwrap(), privatekey.unwrap(), chain);
                    account.encrypt_keystore(&path, password.unwrap(), name)?;

                    println!(
                        "\n🎉 Successfully created new encrypted keystore at {}.\n\nAccount: {:#X}\nPrivate key: {:#X}\nChain: {}",
                        path.display(),
                        account.account,
                        account.get_signing_key(),
                        account.chain.map_or_else(|| "".to_string(), |c| c.to_string())
                    );
                } else {
                    let wallet = WalletOptions {
                        interactive: true,
                        ..Default::default()
                    };

                    let account = wallet.interactive()?.unwrap();

                    let name = Text::new("Enter account name : ").prompt()?;
                    let password = Password::new("Enter keystore password : ").prompt()?;

                    account.encrypt_keystore(get_main_keystore_dir(), password, Some(name))?;

                    println!(
                        "\n🎉 Created new encrypted keystore.\n\nAccount: {:#X}\nPrivate key: {:#X}\nChain: {}",
                        account.account,
                        account.get_signing_key(),
                        account.chain.map_or_else(|| "".to_string(), |c| c.to_string())
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
                    let account = SimpleAccount::decrypt_keystore(path, password.unwrap())?;
                    let hash = parse_hex_or_str_as_felt(message.as_ref().unwrap())?;
                    let sig = account.sign_hash(&hash).await?;
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
                    let account = SimpleAccount::decrypt_keystore(keystore, password)?;

                    let message = Text::new("Enter message to sign : ").with_help_message("Message with 0x prefix is treated as hex value otherwise literal string").prompt()?;

                    let hash = parse_hex_or_str_as_felt(&message)?;
                    let sig = account.sign_hash(&hash).await?;

                    println!("\n{:#x} {:#x}", sig.r, sig.s);
                }

                Ok(())
            }
        }
    }
}
