use std::str::FromStr;

use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::{Error, ErrorKind};
use starknet::core::types::FieldElement;
use starknet::core::types::{BlockId, BlockTag};
use starknet::core::utils::get_selector_from_name;

use crate::opts::starknet::ChainId;

#[derive(Debug, Clone, Copy)]
pub struct BlockIdParser;

impl TypedValueParser for BlockIdParser {
    type Value = BlockId;

    fn parse_ref(
        &self,
        _: &clap::Command,
        _: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, Error> {
        let value = value
            .to_str()
            .ok_or_else(|| Error::raw(ErrorKind::InvalidUtf8, "invalid utf-8"))?;

        // There must be a more idiomatic way of doing this.
        if value.starts_with("0x") {
            let hash = FieldElement::from_hex_be(value).map_err(|e| {
                Error::raw(ErrorKind::InvalidValue, format!("invalid block id: {e}"))
            })?;

            Ok(BlockId::Hash(hash))
        } else if let Ok(number) = value.parse::<u64>() {
            Ok(BlockId::Number(number))
        } else {
            match value.to_lowercase().as_str() {
                "latest" => Ok(BlockId::Tag(BlockTag::Latest)),
                "pending" => Ok(BlockId::Tag(BlockTag::Pending)),
                _ => Err(Error::raw(ErrorKind::InvalidValue, "invalid block tag")),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChainIdParser;

impl TypedValueParser for ChainIdParser {
    type Value = ChainId;

    fn parse_ref(
        &self,
        _: &clap::Command,
        _: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, Error> {
        let value = value
            .to_str()
            .ok_or_else(|| Error::raw(ErrorKind::InvalidUtf8, "invalid utf-8"))?;

        match ChainId::from_str(value) {
            Ok(chain_id) => Ok(chain_id),
            Err(_) => {
                let felt = FieldElement::from_str(value)
                    .map_err(|_| Error::raw(ErrorKind::InvalidValue, "invalid felt value"))?;

                Ok(ChainId::try_from(felt)
                    .map_err(|_| Error::raw(ErrorKind::InvalidValue, "invalid chain id"))?)
            }
        }
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        Some(Box::new(
            [PossibleValue::new("mainnet"), PossibleValue::new("sepolia")].into_iter(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct TokenAddressParser;

impl TypedValueParser for TokenAddressParser {
    type Value = FieldElement;

    fn parse_ref(
        &self,
        _: &clap::Command,
        _: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, Error> {
        let value = value
            .to_str()
            .ok_or(Error::raw(ErrorKind::InvalidUtf8, "invalid utf-8"))?;

        let address = match value.to_lowercase().as_str() {
            // 0x049D36570D4e46f48e99674bd3fcc84644DdD6b96F7C741B1562B82f9e004dC7
            "eth" => FieldElement::from_mont([
                4380532846569209554,
                17839402928228694863,
                17240401758547432026,
                418961398025637529,
            ]),
            // 0x04718f5a0Fc34cC1AF16A1cdee98fFB20C31f5cD61D6Ab07201858f4287c938D
            "strk" => FieldElement::from_mont([
                16432072983745651214,
                1325769094487018516,
                5134018303144032807,
                468300854463065062,
            ]),
            // 0x053C91253BC9682c04929cA02ED00b3E423f6710D2ee7e0D5EBB06F3eCF368A8
            "usdc" => FieldElement::from_mont([
                5808361013446951402,
                13558485962494585092,
                9528015766451344574,
                198270530439797869,
            ]),

            _ => {
                return Err(Error::raw(
                    ErrorKind::InvalidValue,
                    format!("invalid token: {value}"),
                ))
            }
        };

        Ok(address)
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        Some(Box::new(
            [
                PossibleValue::new("ETH"),
                PossibleValue::new("STRK"),
                PossibleValue::new("USDC"),
            ]
            .into_iter(),
        ))
    }
}

/// Used as clap's value parser for `selector` field in `InvokeArgs`.
pub fn selector_parser(selector: &str) -> eyre::Result<FieldElement> {
    let value = FieldElement::from_str(selector);
    match value {
        Ok(selector) => Ok(selector),
        Err(_) => Ok(get_selector_from_name(selector)?),
    }
}

pub fn calldata_parser(calldata: &str) -> eyre::Result<Vec<FieldElement>> {
    todo!()
}
