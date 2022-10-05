pub mod utils;

use std::str::FromStr;

use eyre::{eyre, Report, Result};
use reqwest::Url;
use serde::Serialize;
use serde_json::{json, Value};
use starknet::providers::jsonrpc::models::{BlockId, BlockTag, FunctionCall};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::{
    core::{
        crypto::{ecdsa_sign, ecdsa_verify, pedersen_hash, Signature},
        types::FieldElement,
        utils::{cairo_short_string_to_felt, parse_cairo_short_string, starknet_keccak},
    },
    accounts::Call,
    providers::jsonrpc::models::MaybePendingBlockWithTxs,
};

pub struct Cast {
    client: JsonRpcClient<HttpTransport>,
}

impl Cast {
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
            MaybePendingBlockWithTxs::Block(block) => serde_json::to_value(&block)?,
            MaybePendingBlockWithTxs::PendingBlock(block) => serde_json::to_value(&block)?,
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
                .ok_or(eyre!("`{}` is not a valid block field.", field))?
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
                .ok_or(eyre!("`{}` is not a valid transaction field.", field))?
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
                .ok_or(eyre!(
                    "`{}` is not a valid transaction receipt field.",
                    field
                ))?
                .to_owned();
        }

        Ok(serde_json::to_string_pretty(&tx_json)?)
    }

    pub async fn pending_transactions(&self) -> Result<String> {
        let res = self.client.pending_transactions().await?;
        Ok(serde_json::to_string_pretty(&res)?)
    }

    pub async fn get_nonce(&self, contract_address: FieldElement) -> Result<String> {
        let nonce = self.client.get_nonce(contract_address).await?;
        let nonce = format!("{:x}", nonce.to_string().parse::<u128>()?);
        Ok(nonce)
    }

    pub async fn get_storage_at(
        &self,
        contract_address: FieldElement,
        key: FieldElement,
        block_id: BlockId,
    ) -> Result<String> {
        let res = self
            .client
            .get_storage_at(contract_address, key, &block_id)
            .await?;

        Ok(format!("{:#x}", res))
    }

    pub async fn call(
        &self,
        contract_address: &FieldElement,
        selector: &str,
        calldata: Vec<FieldElement>,
        block_id: &BlockId,
    ) -> Result<String> {
        let entry_point_selector = FieldElement::from_hex_be(&SimpleCast::keccak(selector)?)?;
        let res = self
            .client
            .call(
                FunctionCall {
                    contract_address: contract_address.to_owned(),
                    entry_point_selector,
                    calldata,
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

    pub async fn invoke(&self, 
        contract_address: &FieldElement,
        calls: &[Call]
    ) {
        // create a signer
        // create function call object, call
        // hash function call object, t_h = hash(call)
        // sign transaction hash s = sign(t_h)
        // send transaction
    }

    pub async fn get_state_update(
        &self, block_id: &BlockId
    ) -> Result<String> {
        let res = self.client.get_state_update(block_id).await?;
        let res = serde_json::to_value(res)?;
        Ok(serde_json::to_string_pretty(&res)?)
    }
}

pub struct SimpleCast;

impl SimpleCast {
    pub fn address_zero() -> String {
        format!("{:#x}", FieldElement::ZERO)
    }

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
        parse_cairo_short_string(&felt).map_err(|e| Report::new(e))
    }

    pub fn ecdsa_sign(
        private_key: &FieldElement,
        message_hash: &FieldElement,
    ) -> Result<Signature> {
        ecdsa_sign(private_key, message_hash).map_err(|e| Report::new(e))
    }

    pub fn ecdsa_verify(
        public_key: &FieldElement,
        message_hash: &FieldElement,
        signature_r: &FieldElement,
        signature_s: &FieldElement,
    ) -> Result<bool> {
        ecdsa_verify(
            public_key,
            &message_hash,
            &Signature {
                r: signature_r.to_owned(),
                s: signature_s.to_owned(),
            },
        )
        .map_err(|e| Report::new(e))
    }
}
