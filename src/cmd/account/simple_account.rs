use crate::opts::{account::utils::read_json_file, starknet::StarknetChain};
use crate::probe::SimpleProbe;

use std::fs::DirBuilder;
use std::str::FromStr;
use std::{convert::Infallible, path::Path};

use async_trait::async_trait;
use eyre::Result;
use rand::thread_rng;
use starknet::accounts::Call;
use starknet::core::crypto::compute_hash_on_elements;
use starknet::core::{crypto::Signature, types::FieldElement};
use starknet::providers::jsonrpc::models::BroadcastedInvokeTransaction;
use starknet::providers::jsonrpc::models::BroadcastedInvokeTransactionV1;
use starknet::providers::jsonrpc::models::{BlockId, BlockTag};
use starknet::providers::jsonrpc::models::{BroadcastedTransaction, InvokeTransactionResult};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient, JsonRpcClientError};
use starknet::signers::{local_wallet::SignError, Signer, SigningKey, VerifyingKey};
use starknet_keystore::Keystore;

const PREFIX_INVOKE: FieldElement = FieldElement::from_mont([
    18443034532770911073,
    18446744073709551615,
    18446744073709551615,
    513398556346534256,
]);

#[derive(Debug, thiserror::Error)]
pub enum AccountError {
    #[error(transparent)]
    ProviderError(JsonRpcClientError<reqwest::Error>),

    #[error("provider is not set for this account")]
    MissingProvider,

    #[error(transparent)]
    SignError(SignError),
}

#[derive(Debug)]
pub struct SimpleAccount {
    signing_key: SigningKey,
    pub account: FieldElement,
    pub chain: Option<StarknetChain>,
    pub provider: Option<JsonRpcClient<HttpTransport>>,
}

impl SimpleAccount {
    pub fn new(
        provider: Option<JsonRpcClient<HttpTransport>>,
        account: FieldElement,
        signing_key: FieldElement,
        chain: Option<StarknetChain>,
    ) -> Self {
        Self {
            chain,
            account,
            provider,
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

        let mut filename = format!("{:#x}", self.account);
        if let Some(tag) = tag {
            filename.push_str(format!("-{}", tag).as_str());
        }
        filename.push_str(".json");

        // check if a keystore with that filename already exists
        if path.join(&filename).exists() {
            eprintln!("keystore already exists `{}` .", filename);
            std::process::exit(1)
        }

        let chain: Option<String> = self.chain.as_ref().map(|chain| chain.to_string());

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
            StarknetChain::from_str(&c).ok()
        } else {
            None
        };

        Ok(SimpleAccount::new(
            None,
            FieldElement::from_str(&keystore.address.unwrap())?,
            priv_key,
            chain,
        ))
    }
}

#[async_trait]
pub trait Account: Signer {
    type Error: std::error::Error;

    fn get_provider(&self) -> Result<&JsonRpcClient<HttpTransport>, Self::Error>;

    async fn get_max_fee(&self, request: &BroadcastedTransaction) -> Result<u64, Self::Error>;

    async fn get_nonce(&self) -> Result<FieldElement, Self::Error>;

    async fn send_invoke_transaction(
        &self,
        request: &BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, Self::Error>;

    async fn prepare_invoke_transaction(
        &self,
        calls: &[Call],
        nonce: FieldElement,
        max_fee: FieldElement,
    ) -> Result<BroadcastedInvokeTransaction, Self::Error>;
}

#[async_trait]
impl Account for SimpleAccount {
    type Error = AccountError;

    fn get_provider(&self) -> Result<&JsonRpcClient<HttpTransport>, Self::Error> {
        self.provider.as_ref().ok_or(AccountError::MissingProvider)
    }

    async fn get_nonce(&self) -> Result<FieldElement, Self::Error> {
        let provider = self.get_provider()?;

        provider
            .get_nonce(&BlockId::Tag(BlockTag::Latest), self.account)
            .await
            .map_err(AccountError::ProviderError)
    }

    async fn prepare_invoke_transaction(
        &self,
        calls: &[Call],
        nonce: FieldElement,
        max_fee: FieldElement,
    ) -> Result<BroadcastedInvokeTransaction, Self::Error> {
        let provider = self.get_provider()?;
        let chain = provider
            .chain_id()
            .await
            .map_err(AccountError::ProviderError)?;

        let calldata = SimpleProbe::generate_calldata_for_multicall_account(calls);

        let tx_hash = compute_hash_on_elements(&[
            PREFIX_INVOKE,
            FieldElement::ONE, // version
            self.account,
            FieldElement::ZERO, // entry_point_selector
            compute_hash_on_elements(&calldata),
            max_fee,
            chain,
            nonce,
        ]);

        let signature = self
            .sign_hash(&tx_hash)
            .await
            .map_err(AccountError::SignError)?;

        Ok(BroadcastedInvokeTransaction::V1(
            BroadcastedInvokeTransactionV1 {
                calldata,
                nonce,
                sender_address: self.account,
                max_fee,
                signature: vec![signature.r, signature.s],
            },
        ))
    }

    // must be called after prepare_transaction
    async fn get_max_fee(&self, request: &BroadcastedTransaction) -> Result<u64, Self::Error> {
        let provider = self.get_provider()?;
        let res = provider
            .estimate_fee(request, &BlockId::Tag(BlockTag::Latest))
            .await
            .map_err(AccountError::ProviderError)?;

        Ok(res.overall_fee)
    }

    // must be called after setting the max fee
    async fn send_invoke_transaction(
        &self,
        request: &BroadcastedInvokeTransaction,
    ) -> Result<InvokeTransactionResult, Self::Error> {
        let provider = self.get_provider()?;

        provider
            .add_invoke_transaction(request)
            .await
            .map_err(AccountError::ProviderError)
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

        let wallet = SimpleAccount::new(None, account, priv_key, None);

        assert!(wallet
            .encrypt_keystore(get_main_keystore_dir(), "yohallo", Some("kari".to_string()))
            .is_ok());
    }
}
