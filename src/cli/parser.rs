use std::str::FromStr;

use clap::{builder::TypedValueParser, PossibleValue};
use starknet::{
    core::{types::FieldElement, utils::cairo_short_string_to_felt, chain_id::{MAINNET, TESTNET}},
    providers::jsonrpc::models::{BlockId, BlockTag},
};

#[derive(Debug, Clone, Copy)]
pub struct FieldElementParser;

#[allow(unused_variables)]
impl TypedValueParser for FieldElementParser {
    type Value = FieldElement;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value = value.to_str().ok_or(clap::Error::raw(
            clap::ErrorKind::InvalidUtf8,
            "Invalid utf-8",
        ))?;

        if value.starts_with("0x") {
            FieldElement::from_hex_be(value)
                .map_err(|e| clap::Error::raw(clap::ErrorKind::InvalidValue, e.to_string()))
        } else {
            FieldElement::from_dec_str(value)
                .map_err(|e| clap::Error::raw(clap::ErrorKind::InvalidValue, e.to_string()))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Ether,
    Dai,
    Other(FieldElement),
}

#[derive(Debug, Clone, Copy)]
pub struct TokenValueParser;

#[allow(unused_variables)]
impl TypedValueParser for TokenValueParser {
    type Value = TokenKind;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value = value.to_str().ok_or(clap::Error::raw(
            clap::ErrorKind::InvalidUtf8,
            "Invalid utf-8",
        ))?;

        let value = value.to_lowercase();
        match value.as_str() {
            "ether" => Ok(TokenKind::Ether),

            "dai" => Ok(TokenKind::Dai),

            _ => Ok(TokenKind::Other(
                FieldElement::from_hex_be(&value)
                    .map_err(|e| clap::Error::raw(clap::ErrorKind::InvalidValue, e.to_string()))?,
            )),
        }
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::PossibleValue<'static>> + '_>> {
        let possible_values: Vec<PossibleValue<'static>> =
            vec![PossibleValue::new("ether"), PossibleValue::new("dai")];
        Some(Box::new(possible_values.into_iter()))
    }
}

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
    ) -> Result<Self::Value, clap::Error> {
        let value = value.to_str().ok_or(clap::Error::raw(
            clap::ErrorKind::InvalidUtf8,
            "Invalid utf-8",
        ))?;

        // There must be a more idiomatic way of doing this.
        if value.starts_with("0x") {
            let hash = FieldElement::from_hex_be(value)
                .map_err(|e| clap::Error::raw(clap::ErrorKind::InvalidValue, e))?;

            Ok(BlockId::Hash(hash))
        } else {
            if let Ok(number) = value.parse::<u64>() {
                Ok(BlockId::Number(number))
            } else {
                match value.to_lowercase().as_str() {
                    "latest" => Ok(BlockId::Tag(BlockTag::Latest)),

                    "pending" => Ok(BlockId::Tag(BlockTag::Pending)),

                    _ => Err(clap::Error::raw(
                        clap::ErrorKind::InvalidValue,
                        "Invalid value",
                    )),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChainParser;

impl TypedValueParser for ChainParser {
    type Value = FieldElement;

    fn parse_ref(
        &self,
        cmd: &clap::Command,
        arg: Option<&clap::Arg>,
        value: &std::ffi::OsStr,
    ) -> Result<Self::Value, clap::Error> {
        let value = value.to_str().ok_or(clap::Error::raw(
            clap::ErrorKind::InvalidUtf8,
            "Invalid utf-8",
        ))?;

        if value.parse::<u128>().is_ok() {
            FieldElement::from_str(value).map_err(|e|
                clap::Error::raw(clap::ErrorKind::InvalidValue, e.to_string()))
        } else {
            match value {
                "SN_MAIN" => Ok(MAINNET),
                "SN_GOERLI" => Ok(TESTNET),
                _ => Err(clap::Error::raw(clap::ErrorKind::InvalidValue, "Invalid chain id"))
            }
        }
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::PossibleValue<'static>> + '_>> {
        let possible_values: Vec<PossibleValue<'static>> =
            vec![PossibleValue::new("SN_MAIN"), PossibleValue::new("SN_GOERLI")];
        Some(Box::new(possible_values.into_iter()))
    }
}

