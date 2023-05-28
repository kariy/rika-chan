use crate::opts::account::utils::read_json_file;

use std::fs::DirBuilder;
use std::path::Path;
use std::str::FromStr;

use eyre::Result;
use rand::thread_rng;
use starknet::accounts::SingleOwnerAccount;
use starknet::core::types::FieldElement;
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::providers::Provider;
use starknet::signers::{LocalWallet, SigningKey};
use starknet_keystore::Keystore;

#[derive(Debug)]
pub struct SimpleWallet {
    pub signing_key: SigningKey,
    pub account: FieldElement,
    pub chain_id: Option<FieldElement>,
}

impl SimpleWallet {
    pub fn new(
        account: FieldElement,
        signing_key: FieldElement,
        chain_id: Option<FieldElement>,
    ) -> Self {
        Self {
            chain_id,
            account,
            signing_key: SigningKey::from_secret_scalar(signing_key),
        }
    }

    pub async fn account(
        self,
        provider: JsonRpcClient<HttpTransport>,
    ) -> Result<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>> {
        let chain_id = match self.chain_id {
            Some(chain_id) => chain_id,
            None => provider.chain_id().await?,
        };

        print!("hi");

        Ok(SingleOwnerAccount::new(
            provider,
            LocalWallet::from_signing_key(self.signing_key.clone()),
            self.account,
            chain_id,
        ))
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
        if self.chain_id.is_some() {
            path = path.join(self.chain_id.as_ref().unwrap().to_string());
        }
        DirBuilder::new().recursive(true).create(&path)?;

        let mut filename = format!("{:#x}", self.account);
        if let Some(tag) = tag {
            filename.push_str(format!("-{tag}").as_str());
        }
        filename.push_str(".json");

        // check if a keystore with that filename already exists
        if path.join(&filename).exists() {
            eprintln!("keystore already exists `{filename}`.");
            std::process::exit(1)
        }

        let chain: Option<String> = self.chain_id.as_ref().map(|chain| chain.to_string());

        let mut rng = thread_rng();
        starknet_keystore::encrypt_key(
            path,
            &mut rng,
            self.signing_key.secret_scalar().to_bytes_be(),
            password.as_ref().as_bytes(),
            Some(&filename),
            Some(format!("{:#x}", self.account)),
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
            FieldElement::from_str(&c).ok()
        } else {
            None
        };

        Ok(SimpleWallet::new(
            FieldElement::from_str(&keystore.address.unwrap())?,
            priv_key,
            chain,
        ))
    }
}
