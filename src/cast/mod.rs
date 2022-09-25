pub mod utils;

use eyre::{eyre, Report, Result};
use reqwest::Url;
use starknet::providers::jsonrpc::models::BlockId;
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient};
use starknet::{
    core::{
        crypto::{ecdsa_sign, ecdsa_verify, pedersen_hash, Signature},
        types::FieldElement,
        utils::{cairo_short_string_to_felt, parse_cairo_short_string, starknet_keccak},
    },
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

        Ok(utils::hex_encode(res.to_bytes_be()))
    }
}

pub struct SimpleCast;

impl SimpleCast {
    pub fn address_zero() -> String {
        utils::hex_encode(FieldElement::ZERO.to_bytes_be())
    }

    pub fn to_hex(dec: &FieldElement) -> Result<String> {
        Ok(utils::hex_encode(dec.to_bytes_be()))
    }

    pub fn to_dec(hex: &FieldElement) -> Result<String> {
        Ok(hex.to_string())
    }

    pub fn keccak(data: &str) -> Result<String> {
        let hash = match data.as_bytes() {
            // 0x prefix => read as hex data
            [b'0', b'x', rest @ ..] => starknet_keccak(&hex::decode(rest)?),
            // No 0x prefix => read as text
            _ => starknet_keccak(data.as_bytes()),
        };

        Ok(utils::hex_encode(hash.to_bytes_be()))
    }

    pub fn pedersen(x: &str, y: &str) -> Result<String> {
        let x = utils::parse_hex_or_str_as_felt(x)?;
        let y = utils::parse_hex_or_str_as_felt(y)?;
        let hash = pedersen_hash(&x, &y);

        Ok(utils::hex_encode(hash.to_bytes_be()))
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
        Ok(utils::hex_encode(felt.to_bytes_be()))
    }

    pub fn from_utf8(felt: &FieldElement) -> Result<String> {
        parse_cairo_short_string(&felt).map_err(|e| Report::new(e))
    }

    pub fn ecdsa_sign(private_key: &FieldElement, message_hash: &str) -> Result<Signature> {
        ecdsa_sign(private_key, &FieldElement::from_hex_be(message_hash)?)
            .map_err(|e| Report::new(e))
    }

    pub fn ecdsa_verify(
        public_key: &FieldElement,
        message_hash: &str,
        signature_r: &FieldElement,
        signature_s: &FieldElement,
    ) -> Result<bool> {
        ecdsa_verify(
            public_key,
            &FieldElement::from_hex_be(message_hash)?,
            &Signature {
                r: signature_r.to_owned(),
                s: signature_s.to_owned(),
            },
        )
        .map_err(|e| Report::new(e))
    }
}
