mod utils;
mod wallet;

use crate::opts::account::utils::get_main_keystore_dir;

use self::{utils::get_from_keystore, wallet::Account};
use super::starknet::StarknetChain;
use crate::opts::parser::PathParser;

use std::{path::PathBuf, str::FromStr};

use clap::{ArgGroup, Parser, Subcommand};
use eyre::Result;
use inquire::{CustomType, Password, Select, Text};
use starknet::core::types::FieldElement;

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

    #[clap(about = "Edit an entry in the keystore.")]
    Edit {
        #[clap(value_name = "PATH")]
        #[clap(value_parser(PathParser))]
        #[clap(help = "The path to the JSON keystore to be edited.")]
        path: Option<PathBuf>,
    },

    #[clap(about = "Sign a message.")]
    Sign {},
}

impl WalletCommands {
    pub fn run(self) -> Result<()> {
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

                    let account = Account::new(account.unwrap(), privatekey.unwrap(), chain);
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

            Self::Edit { path } => {
                if let Some(path) = path {
                    // check that path must be a file
                    todo!("wallet edit with path");
                } else {
                    // show list of chains
                    // show list of keystores inside of the selected chain dir
                    // show list of editable entries
                    todo!("wallet edit interactive");
                };
            }

            Self::Sign {} => todo!("wallet sign"),
        }
    }
}

#[derive(Debug, Clone, Parser, Default)]
pub struct WalletOptions {
    #[clap(short, long)]
    #[clap(exclusive = true)]
    #[clap(help_heading = "WALLET OPTIONS - RAW")]
    #[clap(help = "Open an interactive prompt to enter your wallet details.")]
    pub interactive: bool,

    #[clap(long)]
    #[clap(value_name = "PRIVATE_KEY")]
    #[clap(help_heading = "WALLET OPTIONS - RAW")]
    #[clap(help = "The raw private key associated with the account contract.")]
    pub private_key: Option<FieldElement>,

    #[clap(long)]
    #[clap(value_name = "ACCOUNT_ADDRESS")]
    #[clap(help_heading = "WALLET OPTIONS - RAW")]
    #[clap(help = "Account contract to initiate the transaction from.")]
    pub from: Option<FieldElement>,

    #[clap(long = "keystore")]
    #[clap(value_name = "PATH")]
    #[clap(env = "STARKNET_KEYSTORE")]
    #[clap(help_heading = "WALLET OPTIONS - KEYSTORE")]
    #[clap(help = "Use the keystore in the given folder or file.")]
    pub keystore_path: Option<PathBuf>,

    #[clap(long = "password")]
    #[clap(value_name = "PASSWORD")]
    #[clap(requires = "keystore_path")]
    #[clap(help_heading = "WALLET OPTIONS - KEYSTORE")]
    #[clap(help = "The keystore password. Used with --keystore.")]
    pub keystore_password: Option<String>,

    #[clap(env = "STARKNET_KEYSTORE_PASSWORD")]
    #[clap(long = "password-file")]
    #[clap(requires = "keystore_path")]
    #[clap(value_name = "PASSWORD_FILE")]
    #[clap(help_heading = "WALLET OPTIONS - KEYSTORE")]
    #[clap(help = "The keystore password file path. Used with --keystore.")]
    pub keystore_password_file: Option<PathBuf>,
}

impl WalletOptions {
    pub fn interactive(&self) -> Result<Option<Account>> {
        Ok(if self.interactive {
            let felt_prompter = |message: &'static str| {
                CustomType::new(message)
                    .with_parser(&|input| FieldElement::from_str(input).map_err(|_| ()))
                    .with_error_message("Invalid field element value.")
                    .with_help_message(
                        "Value must be smaller than 251 bits. Can be a hex or decimal number.",
                    )
            };

            let account = felt_prompter("Enter account address : ").prompt()?;
            let private_key = felt_prompter("Enter private key : ").prompt()?;

            let options = vec!["mainnet", "testnet", "testnet2"];
            let chain =
                Select::new("Please select the chain for this account.", options).prompt()?;

            Some(Account::new(
                account,
                private_key,
                StarknetChain::from_str(chain).ok(),
            ))
        } else {
            None
        })
    }

    pub fn raw(&self) -> Option<Account> {
        match (self.from, self.private_key) {
            (Some(from), Some(pk)) => Some(Account::new(from, pk, None)),
            _ => None,
        }
    }

    pub fn keystore(&self) -> Result<Option<Account>> {
        get_from_keystore(
            self.from.unwrap().to_string().as_ref(),
            self.keystore_path.as_ref(),
            self.keystore_password.as_ref(),
            self.keystore_password_file.as_ref(),
        )
        // todo!("walletopts: create account from keystore")
    }
}
