pub mod utils;

use self::utils::get_from_keystore;
use crate::cmd::account::simple_account::SimpleWallet;

use std::{path::PathBuf, str::FromStr};

use clap::{ArgGroup, Args};
use eyre::Result;
use inquire::CustomType;
use starknet::core::types::FieldElement;

#[derive(Debug, Clone, Args, Default)]
#[command(group(ArgGroup::new("wallet-method").args(["private_key", "keystore_path"])))]
#[command(group(ArgGroup::new("password-method").args(["keystore_password", "keystore_password_file"]).requires("keystore_path")))]
#[command(group(ArgGroup::new("wallet-interactive").args(["interactive"]).conflicts_with_all(["private_key", "account", "keystore_path", "keystore_password", "keystore_password_file"])))]
pub struct WalletOptions {
    #[arg(short, long)]
    #[arg(help_heading = "WALLET OPTIONS - RAW")]
    #[arg(help = "Open an interactive prompt to enter your wallet details.")]
    pub interactive: bool,

    #[arg(long)]
    #[arg(requires = "account")]
    #[arg(value_name = "PRIVATE_KEY")]
    #[arg(help_heading = "WALLET OPTIONS - RAW")]
    #[arg(help = "The raw private key associated with the account contract.")]
    pub private_key: Option<FieldElement>,

    #[arg(long)]
    #[arg(value_name = "FROM")]
    #[arg(requires = "wallet-method")]
    #[arg(help_heading = "WALLET OPTIONS - RAW")]
    #[arg(help = "Account contract to initiate the transaction from.")]
    pub account: Option<FieldElement>,

    #[arg(long = "keystore")]
    #[arg(value_name = "PATH")]
    #[arg(env = "STARKNET_KEYSTORE")]
    #[arg(help_heading = "WALLET OPTIONS - KEYSTORE")]
    #[arg(help = "Use the keystore in the given folder or file.")]
    pub keystore_path: Option<PathBuf>,

    #[arg(long = "password")]
    #[arg(value_name = "PASSWORD")]
    #[arg(help_heading = "WALLET OPTIONS - KEYSTORE")]
    #[arg(help = "The keystore password. Used with --keystore.")]
    pub keystore_password: Option<String>,

    #[arg(long = "password-file")]
    #[arg(value_name = "PASSWORD_FILE")]
    #[arg(env = "STARKNET_KEYSTORE_PASSWORD")]
    #[arg(conflicts_with = "keystore_password")]
    #[arg(help_heading = "WALLET OPTIONS - KEYSTORE")]
    #[arg(help = "The keystore password file path. Used with --keystore.")]
    pub keystore_password_file: Option<PathBuf>,
}

impl WalletOptions {
    pub fn build_wallet(&self) -> Result<Option<SimpleWallet>> {
        match self.keystore()?.or_else(|| self.raw()) {
            Some(account) => Ok(Some(account)),
            None => self.interactive(),
        }
    }

    pub fn interactive(&self) -> Result<Option<SimpleWallet>> {
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

            Some(SimpleWallet::new(account, private_key, None))
        } else {
            None
        })
    }

    pub fn raw(&self) -> Option<SimpleWallet> {
        match (self.account, self.private_key) {
            (Some(account), Some(pk)) => Some(SimpleWallet::new(account, pk, None)),
            _ => None,
        }
    }

    pub fn keystore(&self) -> Result<Option<SimpleWallet>> {
        get_from_keystore(
            self.account,
            self.keystore_path.as_ref(),
            self.keystore_password.as_ref(),
            self.keystore_password_file.as_ref(),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn account_from_keystore() {
        let account_addr = FieldElement::from_hex_be(
            "0x148A764E88277F972B6E1517A60CD6EF5FC11FF3DBC686EA932451552D0641B",
        )
        .unwrap();

        let file = Path::new("./tests/test-keys/test-key1.json");
        let opts = WalletOptions {
            account: Some(account_addr),
            keystore_path: Some(file.to_path_buf()),
            keystore_password: Some("12345".to_string()),
            ..Default::default()
        };

        let wallet = opts.keystore().unwrap().unwrap();

        assert_eq!(
            wallet.account,
            FieldElement::from_hex_be(
                "0x148A764E88277F972B6E1517A60CD6EF5FC11FF3DBC686EA932451552D0641B"
            )
            .unwrap()
        );

        assert_eq!(wallet.chain_id.unwrap().to_string(), "mainnet");
    }

    #[test]
    fn account_from_keystore_and_password_file() {
        let account_addr = FieldElement::from_hex_be(
            "0x148A764E88277F972B6E1517A60CD6EF5FC11FF3DBC686EA932451552D0641B",
        )
        .unwrap();

        let file = Path::new("./tests/test-keys/test-key1.json");
        let password_file = Path::new("./tests/test-keys/password1");

        let opts = WalletOptions {
            account: Some(account_addr),
            keystore_path: Some(file.to_path_buf()),
            keystore_password_file: Some(password_file.to_path_buf()),
            ..Default::default()
        };

        let wallet = opts.keystore().unwrap().unwrap();

        assert_eq!(
            wallet.account,
            FieldElement::from_hex_be(
                "0x148A764E88277F972B6E1517A60CD6EF5FC11FF3DBC686EA932451552D0641B"
            )
            .unwrap()
        );

        assert_eq!(wallet.chain_id.unwrap().to_string(), "mainnet");
    }

    #[test]
    fn account_from_raw() {
        let from = FieldElement::from_hex_be("").unwrap();
        let private_key = FieldElement::from_hex_be("").unwrap();

        let opts = WalletOptions {
            account: Some(from),
            private_key: Some(private_key),
            ..Default::default()
        };

        let wallet = opts.raw().unwrap();

        assert!(wallet.chain_id.is_none());
        assert_eq!(wallet.account, from);
    }
}
