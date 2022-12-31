pub mod utils;

use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

use crypto_bigint::U256;
use eyre::{eyre, Report, Result};
use reqwest::Url;
use starknet::core::utils::get_selector_from_name;
use starknet::providers::jsonrpc::models::{
    BlockId, BroadcastedTransaction, EventFilter, FunctionCall,
};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::{
    core::{
        crypto::{ecdsa_sign, ecdsa_verify, pedersen_hash, Signature},
        types::{ContractArtifact, FieldElement, FromStrError},
        utils::{
            cairo_short_string_to_felt, get_contract_address, get_storage_var_address,
            parse_cairo_short_string, starknet_keccak,
        },
    },
    providers::jsonrpc::models::MaybePendingBlockWithTxs,
};

use crate::cmd::parser::TokenKind;

pub struct Probe {
    client: JsonRpcClient<HttpTransport>,
}

impl Probe {
    pub fn new(url: Url) -> Self {
        Self {
            client: JsonRpcClient::new(HttpTransport::new(url)),
        }
    }

    pub async fn block(
        &self,
        block_id: BlockId,
        full: bool,
        field: Option<String>,
    ) -> Result<String> {
        let block = self.client.get_block_with_txs(&block_id).await?;
        let mut block_json = match block {
            MaybePendingBlockWithTxs::Block(block) => serde_json::to_value(block)?,
            MaybePendingBlockWithTxs::PendingBlock(block) => serde_json::to_value(block)?,
        };

        if !full {
            block_json
                .as_object_mut()
                .unwrap()
                .remove_entry("transactions");
        }

        if let Some(field) = field {
            block_json = block_json
                .get(&field)
                .ok_or_else(|| eyre!("`{}` is not a valid block field.", field))?
                .to_owned();
        }

        Ok(serde_json::to_string_pretty(&block_json)?)
    }

    pub async fn get_block_transaction_count(&self, block_id: BlockId) -> Result<u64> {
        let total = self.client.get_block_transaction_count(&block_id).await?;
        Ok(total)
    }

    pub async fn block_number(&self) -> Result<u64> {
        Ok(self.client.block_number().await?)
    }

    pub async fn chain_id(&self) -> Result<String> {
        Ok(self.client.chain_id().await?.to_string())
    }

    pub async fn get_transaction_by_hash(
        &self,
        transaction_hash: FieldElement,
        field: Option<String>,
    ) -> Result<String> {
        let tx = self
            .client
            .get_transaction_by_hash(transaction_hash)
            .await?;

        let mut tx_json = serde_json::to_value(tx)?;

        if let Some(field) = field {
            tx_json = tx_json
                .get(&field)
                .ok_or_else(|| eyre!("`{}` is not a valid transaction field.", field))?
                .to_owned();
        }

        Ok(serde_json::to_string_pretty(&tx_json)?)
    }

    pub async fn get_transaction_receipt(
        &self,
        transaction_hash: FieldElement,
        field: Option<String>,
    ) -> Result<String> {
        let tx = self
            .client
            .get_transaction_receipt(transaction_hash)
            .await?;

        let mut tx_json = serde_json::to_value(tx)?;

        if let Some(field) = field {
            tx_json = tx_json
                .get(&field)
                .ok_or_else(|| eyre!("`{}` is not a valid transaction receipt field.", field))?
                .to_owned();
        }

        Ok(serde_json::to_string_pretty(&tx_json)?)
    }

    pub async fn pending_transactions(&self) -> Result<String> {
        let res = self.client.pending_transactions().await?;
        Ok(serde_json::to_string_pretty(&res)?)
    }

    pub async fn get_nonce(
        &self,
        contract_address: FieldElement,
        block_id: &BlockId,
    ) -> Result<String> {
        let nonce = self.client.get_nonce(block_id, contract_address).await?;
        let nonce = format!("{:#x}", nonce.to_string().parse::<u128>()?);
        Ok(nonce)
    }

    pub async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_id: &BlockId,
    ) -> Result<String> {
        let res = self
            .client
            .get_storage_at(contract_address, key, block_id)
            .await?;

        Ok(format!("{:#x}", res))
    }

    pub async fn call(
        &self,
        contract_address: &FieldElement,
        function_name: &str,
        calldata: &Vec<FieldElement>,
        block_id: &BlockId,
        abi: &Option<PathBuf>,
    ) -> Result<String> {
        if let Some(abi) = abi {
            let expected_input_count = utils::count_function_inputs(abi, function_name)?;
            if expected_input_count != calldata.len() as u64 {
                return Err(eyre!(
                    "expected {} input(s) but got {}",
                    expected_input_count,
                    calldata.len()
                ));
            }
        }

        let res = self
            .client
            .call(
                FunctionCall {
                    calldata: calldata.to_owned(),
                    contract_address: contract_address.to_owned(),
                    entry_point_selector: get_selector_from_name(function_name)?,
                },
                block_id,
            )
            .await?;

        let res = res
            .into_iter()
            .map(|value| format!("{:#x}", value))
            .collect::<Vec<String>>();

        Ok(res.join(" "))
    }

    pub async fn get_state_update(&self, block_id: &BlockId) -> Result<String> {
        let res = self.client.get_state_update(block_id).await?;
        let res = serde_json::to_value(res)?;
        Ok(serde_json::to_string_pretty(&res)?)
    }

    pub async fn estimate_fee<R>(&self, call: R, block_id: &BlockId) -> Result<String>
    where
        R: AsRef<BroadcastedTransaction>,
    {
        let res = self.client.estimate_fee(call, block_id).await?;
        let value = serde_json::to_value(res)?;
        Ok(serde_json::to_string_pretty(&value)?)
    }

    pub async fn get_class_code(
        &self,
        class_hash: FieldElement,
        block_id: &BlockId,
    ) -> Result<String> {
        let res = self.client.get_class(block_id, class_hash).await?;
        let res = serde_json::to_value(res)?;
        Ok(serde_json::to_string_pretty(&res)?)
    }

    pub async fn get_contract_code(
        &self,
        contract_address: FieldElement,
        block_id: &BlockId,
    ) -> Result<String> {
        let res = self.client.get_class_at(block_id, contract_address).await?;
        let res = serde_json::to_value(res)?;
        Ok(serde_json::to_string_pretty(&res)?)
    }

    pub async fn get_contract_class(
        &self,
        contract_address: FieldElement,
        block_id: &BlockId,
    ) -> Result<String> {
        let res = self
            .client
            .get_class_hash_at(block_id, contract_address)
            .await?;
        Ok(format!("{:#x}", res))
    }

    pub async fn get_events(
        &self,
        filter: EventFilter,
        chunk_size: u64,
        continuation_token: Option<String>,
    ) -> Result<String> {
        let res = self
            .client
            .get_events(filter, continuation_token, chunk_size)
            .await?;
        let value = serde_json::to_value(res)?;
        Ok(serde_json::to_string_pretty(&value)?)
    }

    pub async fn get_balance(
        &self,
        account: FieldElement,
        token: TokenKind,
        block_id: BlockId,
    ) -> Result<String> {
        // value is a Uint256(low,high)
        let res = self
            .client
            .call(
                &FunctionCall {
                    calldata: vec![account],
                    contract_address: token.get_token_address(),
                    // keccak hash of the string 'balanceOf'
                    entry_point_selector: FieldElement::from_mont([
                        8914400797191611589u64,
                        3817639149632004388u64,
                        9799122768618501063u64,
                        186492163330788704u64,
                    ]),
                },
                &block_id,
            )
            .await?;

        Ok(format!("{:#x}{:x}", &res[1], &res[0]))
    }
}

pub struct SimpleProbe;

impl SimpleProbe {
    pub fn to_hex(dec: &FieldElement) -> String {
        format!("{:#x}", dec)
    }

    pub fn to_dec(hex: &FieldElement) -> String {
        hex.to_string()
    }

    pub fn keccak(data: &str) -> Result<String> {
        let hash = match data.as_bytes() {
            // 0x prefix => read as hex data
            [b'0', b'x', rest @ ..] => starknet_keccak(&hex::decode(rest)?),
            // No 0x prefix => read as text
            _ => starknet_keccak(data.as_bytes()),
        };

        Ok(format!("{:#x}", hash))
    }

    pub fn pedersen(x: &str, y: &str) -> Result<String> {
        let x = utils::parse_hex_or_str_as_felt(x)?;
        let y = utils::parse_hex_or_str_as_felt(y)?;
        let hash = pedersen_hash(&x, &y);

        Ok(format!("{:#x}", hash))
    }

    pub fn max_felt() -> String {
        FieldElement::MAX.to_string()
    }

    pub fn max_signed_felt() -> &'static str {
        utils::SIGNED_FELT_MAX
    }

    pub fn min_signed_felt() -> &'static str {
        utils::SIGNED_FELT_MIN
    }

    pub fn str_to_felt(short_str: &str) -> Result<String> {
        let felt = cairo_short_string_to_felt(short_str)?;
        Ok(format!("{:#x}", felt))
    }

    pub fn from_utf8(felt: &FieldElement) -> Result<String> {
        parse_cairo_short_string(felt).map_err(Report::new)
    }

    pub fn ecdsa_sign(
        private_key: &FieldElement,
        message_hash: &FieldElement,
    ) -> Result<Signature> {
        ecdsa_sign(private_key, message_hash).map_err(Report::new)
    }

    pub fn ecdsa_verify(
        public_key: &FieldElement,
        message_hash: &FieldElement,
        signature_r: &FieldElement,
        signature_s: &FieldElement,
    ) -> Result<bool> {
        ecdsa_verify(
            public_key,
            message_hash,
            &Signature {
                r: signature_r.to_owned(),
                s: signature_s.to_owned(),
            },
        )
        .map_err(Report::new)
    }

    pub fn get_storage_index(var_name: &str, keys: &[FieldElement]) -> Result<FieldElement> {
        get_storage_var_address(var_name, keys).map_err(Report::new)
    }

    pub fn compute_contract_hash<P>(compiled_contract: P) -> Result<FieldElement>
    where
        P: AsRef<Path>,
    {
        let res = fs::read_to_string(compiled_contract)?;
        let contract: ContractArtifact = serde_json::from_str(&res)?;
        contract.class_hash().map_err(Report::new)
    }

    pub fn compute_contract_address(
        caller_address: FieldElement,
        salt: FieldElement,
        class_hash: FieldElement,
        calldata: &[FieldElement],
    ) -> String {
        let address = get_contract_address(salt, class_hash, calldata, caller_address);
        format!("{:#x}", address)
    }

    pub fn split_u256(hex: &str) -> Result<(String, String)> {
        let hex = hex.trim_start_matches("0x");
        let hex_chars_len = hex.len();

        let padded_hex = match hex_chars_len.cmp(&64) {
            Ordering::Equal => hex::decode(hex)?,

            Ordering::Less => {
                let mut padded_hex = str::repeat("0", 64 - hex_chars_len);
                padded_hex.push_str(hex);
                hex::decode(padded_hex)?
            }

            Ordering::Greater => return Err(eyre!(FromStrError::OutOfRange)),
        };

        let value = U256::from_be_slice(&padded_hex);
        let (high, low) = value.split();

        Ok((format!("0x{:x}", high), format!("0x{:x}", low)))
    }
}
