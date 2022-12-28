pub mod simple_account;

use super::{account::simple_account::SimpleAccount, parser::PathParser};
use crate::opts::account::utils::get_main_keystore_dir;
use crate::opts::account::WalletOptions;
use crate::opts::starknet::StarknetChain;

use std::path::PathBuf;

use clap::{ArgGroup, Subcommand};
use eyre::Result;
use inquire::{Password, Text};
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
