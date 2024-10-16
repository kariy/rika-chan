pub mod utils;

use std::cmp::Ordering;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

use cairo_lang_starknet::casm_contract_class::CasmContractClass;
use cairo_lang_starknet::contract_class::ContractClass;
use crypto_bigint::U256;
use color_eyre::{eyre::eyre, Report, Result};
use reqwest::Url;
use rika_fmt::Pretty;
use starknet::accounts::Call;
use starknet::core::crypto::{compute_hash_on_elements, ExtendedSignature};
use starknet::core::types::contract::CompiledClass;
use starknet::core::types::{
    BlockId, ContractArtifact, EventFilter, FunctionCall, MaybePendingTransactionReceipt,
};
use starknet::core::utils::get_selector_from_name;
use starknet::core::{
    crypto::{ecdsa_sign, ecdsa_verify, Signature},
    types::{FieldElement, FromStrError, MaybePendingBlockWithTxs},
    utils::{
        cairo_short_string_to_felt, get_contract_address, get_storage_var_address,
        parse_cairo_short_string, starknet_keccak,
    },
};
use starknet::providers::jsonrpc::{HttpTransport, JsonRpcClient, JsonRpcTransport};
use starknet::providers::Provider;

pub struct Rika<P> {
    provider: P,
}

impl<T: JsonRpcTransport> Rika<JsonRpcClient<T>> {
    pub fn new_with_transport(transport: T) -> Self {
        Self {
            provider: JsonRpcClient::new(transport),
        }
    }
}

impl Rika<JsonRpcClient<HttpTransport>> {
    pub fn new_with_http(url: Url) -> Self {
        Self::new_with_transport(HttpTransport::new(url))
    }
}

impl<P: Provider> Rika<P> {
    pub async fn block(
        &self,
        block_id: BlockId,
        full: bool,
        field: Option<String>,
        to_json: bool,
    ) -> Result<String> {
        let block = self.provider.get_block_with_txs(block_id).await?;

        if to_json || field.is_some() {
            let mut json = match block {
                MaybePendingBlockWithTxs::Block(block) => serde_json::to_value(block)?,
                MaybePendingBlockWithTxs::PendingBlock(block) => serde_json::to_value(block)?,
            };

            if let Some(field) = field {
                json = json
                    .get(&field)
                    .ok_or_else(|| eyre!("`{field}` is not a valid block field."))?
                    .to_owned();
            } else if !full {
                json.as_object_mut().unwrap().remove("transactions");
            }

            Ok(serde_json::to_string_pretty(&json)?)
        } else {
            Ok(format!("\n{}", {
                if full {
                    block.prettify()
                } else {
                    pretty_block_without_txs(&block)
                }
            }))
        }
    }

    pub async fn get_block_transaction_count(&self, block_id: BlockId) -> Result<u64> {
        let total = self.provider.get_block_transaction_count(block_id).await?;
        Ok(total)
    }

    pub async fn block_number(&self) -> Result<u64> {
        Ok(self.provider.block_number().await?)
    }

    pub async fn chain_id(&self) -> Result<String> {
        Ok(self.provider.chain_id().await?.to_string())
    }

    pub async fn get_transaction_by_hash(
        &self,
        transaction_hash: FieldElement,
        field: Option<String>,
        to_json: bool,
    ) -> Result<String> {
        let tx = self
            .provider
            .get_transaction_by_hash(transaction_hash)
            .await?;

        if to_json || field.is_some() {
            let mut value = serde_json::to_value(tx)?;

            if let Some(field) = field {
                value = value
                    .get(&field)
                    .ok_or_else(|| eyre!("`{}` is not a valid transaction field.", field))?
                    .to_owned();
            }

            Ok(serde_json::to_string_pretty(&value)?)
        } else {
            Ok(format!("\n{}", tx.prettify()))
        }
    }

    pub async fn get_transaction_receipt(
        &self,
        transaction_hash: FieldElement,
        field: Option<&str>,
        json: bool,
    ) -> Result<String> {
        let receipt = self
            .provider
            .get_transaction_receipt(transaction_hash)
            .await?;

        if json || field.is_some() {
            let mut json = serde_json::to_value(&receipt)?;

            if let Some(field) = field {
                json = json
                    .get(field)
                    .ok_or_else(|| eyre!("`{field}` is not a valid transaction receipt field."))?
                    .to_owned();
            }

            Ok(serde_json::to_string_pretty(&json)?)
        } else {
            Ok(format!("\n{}", receipt.prettify()))
        }
    }

    pub async fn get_transaction_status(&self, transaction_hash: FieldElement) -> Result<String> {
        let receipt = self
            .provider
            .get_transaction_receipt(transaction_hash)
            .await?;

        match receipt {
            MaybePendingTransactionReceipt::Receipt(receipt) => {
                let json = serde_json::to_value(receipt)?;
                Ok(json
                    .get("status")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .replace('_', " "))
            }
            MaybePendingTransactionReceipt::PendingReceipt(_) => Ok("PENDING".into()),
        }
    }

    pub async fn get_nonce(
        &self,
        contract_address: FieldElement,
        block_id: &BlockId,
    ) -> Result<String> {
        let nonce = self.provider.get_nonce(block_id, contract_address).await?;
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
            .provider
            .get_storage_at(contract_address, key, block_id)
            .await?;

        Ok(format!("{res:#x}"))
    }

    pub async fn call(
        &self,
        contract_address: &FieldElement,
        function_name: &str,
        calldata: &Vec<FieldElement>,
        block_id: &BlockId,
    ) -> Result<String> {
        let res = self
            .provider
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
            .map(|value| format!("{value:#x}"))
            .collect::<Vec<String>>();

        Ok(res.join(" "))
    }

    pub async fn get_state_update(&self, block_id: &BlockId) -> Result<String> {
        let res = self.provider.get_state_update(block_id).await?;
        let res = serde_json::to_value(res)?;
        Ok(serde_json::to_string_pretty(&res)?)
    }

    pub async fn get_class_code(
        &self,
        class_hash: FieldElement,
        block_id: &BlockId,
    ) -> Result<String> {
        let res = self.provider.get_class(block_id, class_hash).await?;
        let res = serde_json::to_value(res)?;
        Ok(serde_json::to_string_pretty(&res)?)
    }

    pub async fn get_contract_code(
        &self,
        contract_address: FieldElement,
        block_id: &BlockId,
    ) -> Result<String> {
        let res = self
            .provider
            .get_class_at(block_id, contract_address)
            .await?;
        let res = serde_json::to_value(res)?;
        Ok(serde_json::to_string_pretty(&res)?)
    }

    pub async fn get_contract_class(
        &self,
        contract_address: FieldElement,
        block_id: &BlockId,
    ) -> Result<String> {
        let res = self
            .provider
            .get_class_hash_at(block_id, contract_address)
            .await?;
        Ok(format!("{res:#x}"))
    }

    pub async fn get_events(
        &self,
        filter: EventFilter,
        chunk_size: u64,
        continuation_token: Option<String>,
    ) -> Result<String> {
        let res = self
            .provider
            .get_events(filter, continuation_token, chunk_size)
            .await?;
        let value = serde_json::to_value(res)?;
        Ok(serde_json::to_string_pretty(&value)?)
    }

    pub async fn get_eth_balance(
        &self,
        account: FieldElement,
        block_id: BlockId,
    ) -> Result<String> {
        // value is a Uint256(low,high)
        let res = self
            .provider
            .call(
                &FunctionCall {
                    calldata: vec![account],
                    // ETH contract address on mainnet, testnet, testnet2
                    contract_address: FieldElement::from_mont([
                        4380532846569209554u64,
                        17839402928228694863u64,
                        17240401758547432026u64,
                        418961398025637529u64,
                    ]),
                    // keccak hash of the string 'balanceOf'
                    entry_point_selector: FieldElement::from_mont([
                        8914400797191611589u64,
                        3817639149632004388u64,
                        9799122768618501063u64,
                        186492163330788704u64,
                    ]),
                },
                block_id,
            )
            .await?;
        Ok(format!("{:#x}{:x}", res[1], res[0]))
    }

    pub async fn syncing(&self) -> Result<String> {
        let res = self.provider.syncing().await?;
        Ok(serde_json::to_string_pretty(&res)?)
    }
}

pub struct SimpleRika;

impl SimpleRika {
    pub fn to_hex(dec: &FieldElement, pad: bool) -> String {
        if pad {
            format!("{dec:#064x}")
        } else {
            format!("{dec:#x}")
        }
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

        Ok(format!("{hash:#x}"))
    }

    pub fn pedersen(data: &[FieldElement]) -> FieldElement {
        compute_hash_on_elements(data)
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
        Ok(format!("{felt:#x}"))
    }

    pub fn from_utf8(felt: &FieldElement) -> Result<String> {
        parse_cairo_short_string(felt).map_err(Report::new)
    }

    pub fn ecdsa_sign(
        private_key: &FieldElement,
        message_hash: &FieldElement,
    ) -> Result<ExtendedSignature> {
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

    pub fn compute_contract_hash<P>(path: P) -> Result<FieldElement>
    where
        P: AsRef<Path>,
    {
        let file = BufReader::new(File::open(path)?);
        let contract_artifact: ContractArtifact = serde_json::from_reader(file)?;

        Ok(match contract_artifact {
            ContractArtifact::SierraClass(class) => class.class_hash()?,
            ContractArtifact::CompiledClass(class) => class.class_hash()?,
            ContractArtifact::LegacyClass(class) => class.class_hash()?,
        })
    }

    pub fn compute_compiled_contract_hash<P>(path: P) -> Result<FieldElement>
    where
        P: AsRef<Path>,
    {
        let casm_contract_class: ContractClass = serde_json::from_reader(File::open(path)?)?;

        let casm_contract = CasmContractClass::from_contract_class(casm_contract_class, true)?;
        let compiled_class = serde_json::to_string_pretty(&casm_contract)
            .and_then(|c| serde_json::from_str::<CompiledClass>(&c))?;

        compiled_class.class_hash().map_err(|e| eyre!(e))
    }

    pub fn compute_contract_address(
        caller_address: FieldElement,
        salt: FieldElement,
        class_hash: FieldElement,
        calldata: &[FieldElement],
    ) -> String {
        let address = get_contract_address(salt, class_hash, calldata, caller_address);
        format!("{address:#x}")
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

        Ok((format!("{high:#x}"), format!("{low:#x}")))
    }

    pub fn generate_multicall_calldata(args: &str) -> Result<Vec<FieldElement>> {
        let mut calls = Vec::new();

        for (idx, call_str) in args.split('-').enumerate() {
            let mut data = call_str.trim().split(' ');

            let to = data
                .next()
                .ok_or_else(|| eyre!("missing contract address for call {}", idx + 1))?;

            let selector = data
                .next()
                .ok_or_else(|| eyre!("missing function name for call {}", idx + 1))?;

            let mut calldata: Vec<FieldElement> = Vec::new();
            for i in data {
                calldata.push(
                    FieldElement::from_str(i)
                        .map_err(|e| eyre!("{e} in calldata for call {}", idx + 1))?,
                )
            }

            let call = Call {
                to: FieldElement::from_str(to)
                    .map_err(|e| eyre!("{e} for call {} contract address ", idx + 1))?,

                selector: get_selector_from_name(selector)
                    .map_err(|e| eyre!("{e} for call {} selector ", idx + 1))?,

                calldata,
            };

            calls.push(call);
        }

        let calldata = Self::generate_calldata_for_multicall_account(&calls);

        Ok(calldata)
    }

    pub fn generate_calldata_for_multicall_account(calls: &[Call]) -> Vec<FieldElement> {
        let mut concated_calldata: Vec<FieldElement> = vec![];
        let mut execute_calldata: Vec<FieldElement> = vec![calls.len().into()];
        for call in calls.iter() {
            execute_calldata.push(call.to); // to
            execute_calldata.push(call.selector); // selector
            execute_calldata.push(concated_calldata.len().into()); // data_offset
            execute_calldata.push(call.calldata.len().into()); // data_len

            for item in call.calldata.iter() {
                concated_calldata.push(*item);
            }
        }
        execute_calldata.push(concated_calldata.len().into()); // calldata_len
        for item in concated_calldata.into_iter() {
            execute_calldata.push(item); // calldata
        }

        execute_calldata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_multicall_str() {
        let arg = "0x123456789 balanceOf 0x987654321 - 0xabc298498723 get_the_owner_of_something 0x1abdf988 0x9872349 0x19831".to_string();
        let calls = SimpleRika::generate_multicall_calldata(&arg).unwrap();

        assert_eq!(
            calls,
            vec![
                FieldElement::from_dec_str("2").unwrap(),
                FieldElement::from_str("0x123456789").unwrap(),
                get_selector_from_name("balanceOf").unwrap(),
                FieldElement::ZERO,
                FieldElement::ONE,
                FieldElement::from_str("0xabc298498723").unwrap(),
                get_selector_from_name("get_the_owner_of_something").unwrap(),
                FieldElement::ONE,
                FieldElement::THREE,
                FieldElement::from_dec_str("4").unwrap(),
                FieldElement::from_str("0x987654321").unwrap(),
                FieldElement::from_str("0x1abdf988").unwrap(),
                FieldElement::from_str("0x9872349").unwrap(),
                FieldElement::from_str("0x19831").unwrap(),
            ]
        );
    }
}
