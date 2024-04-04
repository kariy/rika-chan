pub mod simple_account;

use super::account::simple_account::SimpleWallet;
use crate::opts::account::{utils::get_main_keystore_dir, WalletOptions};
use crate::opts::starknet::ChainId;
use crate::utils::canonicalize_path;
use crate::utils::parse_hex_or_str_as_felt;

use std::path::PathBuf;
use std::str::FromStr;

use clap::{ArgGroup, Subcommand};
use eyre::{bail, Context, Result};
use inquire::{Password, Select, Text};
use starknet::core::types::FieldElement;

#[derive(Debug, Subcommand)]
pub enum WalletCommands {
    #[command(about = "Create a keystore for a StarkNet account.")]
    #[command(group(ArgGroup::new("new-raw").args(&["path"]).requires_all(&["account", "privatekey", "password", "chain"])))]
    New {
        #[arg(long)]
        #[arg(value_name = "PATH")]
        #[arg(value_parser = canonicalize_path)]
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
        chain: Option<ChainId>,

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
        #[arg(value_parser = canonicalize_path)]
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
                let (path, account_address, chain) = if let Some(path) = path {
                    if !path.is_dir() {
                        // we require path to be an existing directory
                        bail!("'{}' is not a directory.", path.display())
                    }

                    let wallet = SimpleWallet::new(account.unwrap(), privatekey.unwrap(), chain);
                    let path = wallet.encrypt_keystore(&path, password.unwrap(), name)?;

                    (
                        path.display().to_string(),
                        wallet.account,
                        wallet
                            .chain
                            .map_or_else(|| "other".to_string(), |c| c.to_string()),
                    )
                } else {
                    let wallet = WalletOptions {
                        interactive: true,
                        ..Default::default()
                    };

                    let mut wallet = wallet.interactive()?.unwrap();

                    let chain = Select::new(
                        "Please select the chain for this account.",
                        [ChainId::options(), &["other"]].concat(),
                    )
                    .prompt()
                    .map(|chain| ChainId::from_str(chain).ok())?;

                    wallet.chain = chain;

                    let name = Text::new("Enter account name : ").prompt()?;
                    let password = Password::new("Enter keystore password : ").prompt()?;

                    let path = wallet.encrypt_keystore(
                        get_main_keystore_dir(),
                        password,
                        if name.is_empty() { None } else { Some(name) },
                    )?;

                    (
                        path.display().to_string(),
                        wallet.account,
                        wallet
                            .chain
                            .map_or_else(|| "other".to_string(), |c| c.to_string()),
                    )
                };

                println!(
                    "🎉 Successfully created new encrypted keystore at {path}\n\nAccount address: {account_address:#x}\nChain: {chain}",
                );

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
                    let wallet = WalletOptions::default()
                        .interactive()
                        .wrap_err_with(|| "Failed to open keystore".to_string())?
                        .expect("Must create wallet from keystore");

                    let message = Text::new("Enter message to sign : ").with_help_message("Message with 0x prefix is treated as hex value otherwise literal string").prompt()?;
                    let hash = parse_hex_or_str_as_felt(&message)?;
                    let sig = wallet.signing_key.sign(&hash)?;

                    println!("\n{:#x} {:#x}", sig.r, sig.s);
                }

                Ok(())
            }
        }
    }
}
