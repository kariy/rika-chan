use crate::cmd::account::simple_account::SimpleAccount;

use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use eyre::{bail, eyre, Result, WrapErr};
use inquire::Password;
use starknet::core::types::FieldElement;
use starknet_keystore::Keystore;

pub const KEYSTORE_DIR: &str = ".starknet/keystore";

pub fn read_json_file<T>(path: impl AsRef<Path>) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let content = fs::read_to_string(path)?;
    let file: T = serde_json::from_str(&content)?;
    Ok(file)
}

pub fn get_main_keystore_dir() -> PathBuf {
    home::home_dir().unwrap().join(KEYSTORE_DIR)
}

/// Attempts to find the actual path of the keystore file.
///
/// If the path is a directory then we try to find the first keystore file with the correct
/// sender address
pub fn find_keystore_file(
    account: Option<FieldElement>,
    path: impl AsRef<Path>,
) -> Result<PathBuf> {
    let path = path.as_ref();
    if !path.exists() {
        bail!("keystore file `{path:?}` does not exist")
    }

    if path.is_dir() {
        let Some(account) = account else {
            return Err(eyre!("unable to find the keystore with unknown account address"))
        };

        let (_, file) = walkdir::WalkDir::new(path)
            .max_depth(2)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter_map(|e| {
                read_json_file::<Keystore>(e.path())
                    .map(|keystore| (keystore, e.path().to_path_buf()))
                    .ok()
            })
            .find(|(keystore, _)| {
                FieldElement::from_str(keystore.address.as_ref().unwrap()).ok() == Some(account)
            })
            .ok_or_else(|| eyre!("no matching keystore file found for {account:?} in {path:?}"))?;

        return Ok(file);
    }

    Ok(path.to_path_buf())
}

pub fn get_from_keystore(
    account: Option<FieldElement>,
    keystore_path: Option<&PathBuf>,
    keystore_password: Option<&String>,
    keystore_password_file: Option<&PathBuf>,
) -> Result<Option<SimpleAccount>> {
    Ok(
        match (keystore_path, keystore_password, keystore_password_file) {
            (Some(path), Some(password), _) => {
                let path = find_keystore_file(account, path)?;
                Some(
                    SimpleAccount::decrypt_keystore(&path, password)
                        .wrap_err_with(|| format!("Failed to decrypt keystore {path:?}"))?,
                )
            }
            (Some(path), _, Some(password_file)) => {
                let path = find_keystore_file(account, path)?;
                Some(
                SimpleAccount::decrypt_keystore(&path, password_from_file(password_file)?)
                    .wrap_err_with(|| format!("Failed to decrypt keystore {path:?} with password file {password_file:?}"))?,
            )
            }
            (Some(path), None, None) => {
                let path = find_keystore_file(account, path)?;
                let password = Password::new("Enter keystore password:").prompt()?;
                Some(SimpleAccount::decrypt_keystore(path, password)?)
            }
            (None, _, _) => None,
        },
    )
}

/// Attempts to read the keystore password from the password file.
fn password_from_file(password_file: impl AsRef<Path>) -> Result<String> {
    let password_file = password_file.as_ref();
    if !password_file.is_file() {
        bail!("Keystore password file `{password_file:?}` does not exist")
    }

    Ok(fs::read_to_string(password_file)?.trim_end().to_string())
}
