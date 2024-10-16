use crate::opts::account::utils::read_json_file;
use crate::opts::starknet::ChainId;

use std::fs::DirBuilder;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use color_eyre::{
    eyre::{bail, eyre},
    Result,
};
use rand::thread_rng;
use starknet::accounts::{ExecutionEncoding, SingleOwnerAccount};
use starknet::core::types::{BlockId, BlockTag, ContractClass, FieldElement};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::providers::Provider;
use starknet::signers::{LocalWallet, SigningKey};
use starknet_keystore::Keystore;

#[derive(Debug)]
pub struct SimpleWallet {
    pub signing_key: SigningKey,
    pub account: FieldElement,
    pub chain: Option<ChainId>,
}

impl SimpleWallet {
    pub fn new(account: FieldElement, signing_key: FieldElement, chain: Option<ChainId>) -> Self {
        Self {
            chain,
            account,
            signing_key: SigningKey::from_secret_scalar(signing_key),
        }
    }

    pub async fn account(
        self,
        provider: JsonRpcClient<HttpTransport>,
    ) -> Result<SingleOwnerAccount<JsonRpcClient<HttpTransport>, LocalWallet>> {
        let chain_id = match self.chain {
            Some(chain_id) => chain_id.id(),
            None => provider.chain_id().await?,
        };

        // fetch the account class definition to make sure it exists
        let class = provider
            .get_class_at(BlockId::Tag(BlockTag::Pending), self.account)
            .await
            .map_err(|e| eyre!("Account {:#x} doesn't exists: {e}", self.account))?;

        let execution_encoding = match class {
            ContractClass::Legacy(_) => ExecutionEncoding::Legacy,
            ContractClass::Sierra(_) => ExecutionEncoding::New,
        };

        Ok(SingleOwnerAccount::new(
            provider,
            LocalWallet::from_signing_key(self.signing_key.clone()),
            self.account,
            chain_id,
            execution_encoding,
        ))
    }

    pub fn encrypt_keystore<T, U>(
        &self,
        path: T,
        password: U,
        tag: Option<String>,
    ) -> Result<PathBuf>
    where
        T: AsRef<Path>,
        U: AsRef<str>,
    {
        let mut path = path.as_ref().to_path_buf();

        path = match &self.chain {
            Some(chain) => path.join(chain.to_string()),
            None => path.join("OTHER"),
        };

        DirBuilder::new().recursive(true).create(&path)?;

        let mut filename = format!("{:#x}", self.account);
        if let Some(tag) = tag {
            filename.push_str(format!("-{tag}").as_str());
        }
        filename.push_str(".json");

        // check if a keystore with that filename already exists
        if path.join(&filename).exists() {
            bail!("keystore already exists `{filename}`.")
        }

        let mut rng = thread_rng();
        starknet_keystore::encrypt_key(
            &path,
            &mut rng,
            self.signing_key.secret_scalar().to_bytes_be(),
            password.as_ref().as_bytes(),
            Some(&filename),
            Some(format!("{:#x}", self.account)),
            self.chain.as_ref().map(|c| c.to_string()),
        )?;

        Ok(path.join(&filename))
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
            Some(ChainId::from_str(&c)?)
        } else {
            None
        };

        Ok(SimpleWallet::new(
            FieldElement::from_str(&keystore.address.ok_or(eyre!("Missing account address."))?)?,
            priv_key,
            chain,
        ))
    }
}
