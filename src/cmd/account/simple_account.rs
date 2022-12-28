use crate::opts::{account::utils::read_json_file, starknet::StarknetChain};

use std::fs::DirBuilder;
use std::str::FromStr;
use std::{convert::Infallible, path::Path};

use async_trait::async_trait;
use eyre::Result;
use rand::thread_rng;
use starknet::{
    core::{crypto::Signature, types::FieldElement},
    signers::{local_wallet::SignError, Signer, SigningKey, VerifyingKey},
};
use starknet_keystore::Keystore;

#[derive(Debug, Clone)]
pub struct SimpleAccount {
    signing_key: SigningKey,
    pub account: FieldElement,
    pub chain: Option<StarknetChain>,
}

impl SimpleAccount {
    pub fn new(
        account: FieldElement,
        signing_key: FieldElement,
        chain: Option<StarknetChain>,
    ) -> Self {
        Self {
            chain,
            account,
            signing_key: SigningKey::from_secret_scalar(signing_key),
        }
    }

    pub fn get_signing_key(&self) -> FieldElement {
        self.signing_key.secret_scalar()
    }

    pub fn encrypt_keystore<T, U>(
        &self,
        path: T,
        password: U,
        tag: Option<String>,
    ) -> Result<String>
    where
        T: AsRef<Path>,
        U: AsRef<str>,
    {
        let mut path = path.as_ref().to_path_buf();
        if self.chain.is_some() {
            path = path.join(self.chain.as_ref().unwrap().to_string());
        }
        DirBuilder::new().recursive(true).create(&path)?;

        let mut filename = format!("{:X}", self.account);
        if let Some(tag) = tag {
            filename.push_str(format!("-{}", tag).as_str());
        }
        filename.push_str(".json");

        // check if a keystore with that filename already exists
        if path.join(&filename).exists() {
            eprintln!("keystore already exists `{}` .", filename);
            std::process::exit(1)
        }

        let chain: Option<String> = if let Some(chain) = &self.chain {
            Some(chain.to_string())
        } else {
            None
        };

        let mut rng = thread_rng();
        starknet_keystore::encrypt_key(
            path,
            &mut rng,
            self.signing_key.secret_scalar().to_bytes_be(),
            password.as_ref().as_bytes(),
            Some(&filename),
            Some(format!("{:X}", self.account)),
            chain,
        )?;

        Ok(filename)
    }

    pub fn decrypt_keystore<P, S>(path: P, password: S) -> Result<Self>
    where
        P: AsRef<Path>,
        S: AsRef<[u8]>,
    {
        let keystore: Keystore = read_json_file(path.as_ref())?;
        let v = starknet_keystore::decrypt_key(path, password)?;
        let priv_key = unsafe { FieldElement::from_bytes_be(&*(v.as_ptr() as *const [u8; 32]))? };
        let chain = if let Some(c) = keystore.chain {
            StarknetChain::from_str(&c).ok()
        } else {
            None
        };

        Ok(SimpleAccount::new(
            FieldElement::from_str(&keystore.address.unwrap())?,
            priv_key,
            chain,
        ))
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl Signer for SimpleAccount {
    type GetPublicKeyError = Infallible;
    type SignError = SignError;

    async fn get_public_key(&self) -> Result<VerifyingKey, Self::GetPublicKeyError> {
        Ok(self.signing_key.verifying_key())
    }

    async fn sign_hash(&self, hash: &FieldElement) -> Result<Signature, Self::SignError> {
        Ok(self.signing_key.sign(hash)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::opts::account::utils::get_main_keystore_dir;

    use super::*;
    use rand::Rng;

    #[test]
    fn test_encrypt_wallet() {
        let mut rng = thread_rng();

        let account = {
            let mut arr = [0u64; 4];
            rng.fill(&mut arr);
            FieldElement::from_mont(arr)
        };
        let priv_key = {
            let mut arr = [0u64; 4];
            rng.fill(&mut arr);
            FieldElement::from_mont(arr)
        };

        let wallet = SimpleAccount::new(account, priv_key, None);

        assert!(wallet
            .encrypt_keystore(get_main_keystore_dir(), "yohallo", Some("kari".to_string()))
            .is_ok());
    }
}
