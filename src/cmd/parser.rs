use crate::rika::utils::canonicalize_path;

use std::{path::PathBuf, str::FromStr};

use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::{Error, ErrorKind};
use starknet::core::types::{BlockId, BlockTag};
use starknet::core::{
    chain_id::{MAINNET, TESTNET},
    types::FieldElement,
};

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
            .ok_or_else(|| Error::raw(ErrorKind::InvalidUtf8, "Invalid utf-8"))?;

        // There must be a more idiomatic way of doing this.
        if value.starts_with("0x") {
            let hash = FieldElement::from_hex_be(value)
                .map_err(|e| Error::raw(ErrorKind::InvalidValue, e))?;

            Ok(BlockId::Hash(hash))
        } else if let Ok(number) = value.parse::<u64>() {
            Ok(BlockId::Number(number))
        } else {
            match value.to_lowercase().as_str() {
                "latest" => Ok(BlockId::Tag(BlockTag::Latest)),

                "pending" => Ok(BlockId::Tag(BlockTag::Pending)),

                _ => Err(Error::raw(ErrorKind::InvalidValue, "Invalid value")),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChainParser;

impl TypedValueParser for ChainParser {
    type Value = FieldElement;

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

        if value.parse::<u128>().is_ok() {
            FieldElement::from_str(value)
                .map_err(|e| Error::raw(ErrorKind::InvalidValue, e.to_string()))
        } else {
            match value {
                "mainnet" => Ok(MAINNET),
                "goerli" => Ok(TESTNET),
                _ => Err(Error::raw(ErrorKind::InvalidValue, "invalid chain id")),
            }
        }
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        let possible_values: Vec<PossibleValue> =
            vec![PossibleValue::new("mainnet"), PossibleValue::new("goerli")];
        Some(Box::new(possible_values.into_iter()))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PathParser;

// write the clap parser impl for pathbuf
impl TypedValueParser for PathParser {
    type Value = PathBuf;

    #[allow(unused_variables)]
    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, Error> {
        let value = value
            .to_str()
            .ok_or_else(|| Error::raw(ErrorKind::InvalidUtf8, "Invalid utf-8"))?;

        canonicalize_path(value).map_err(|e| Error::raw(ErrorKind::ValueValidation, e))
    }
}
