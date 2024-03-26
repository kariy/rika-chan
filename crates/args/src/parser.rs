use std::str::FromStr;

use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::{Error, ErrorKind};
use starknet::core::types::FieldElement;
use starknet::core::types::{BlockId, BlockTag};
use starknet::core::utils::get_selector_from_name;

use crate::opts::starknet::ChainId;

#[derive(Debug, Clone, Copy)]
pub struct BlockIdParser;

#[allow(unused_variables)]
impl TypedValueParser for BlockIdParser {
    type Value = BlockId;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
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

    #[allow(unused_variables)]
    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
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
