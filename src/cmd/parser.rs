use crate::cast::utils::canonicalize_path;

use std::{path::PathBuf, str::FromStr};

use clap::{builder::TypedValueParser, PossibleValue};
use starknet::{
    core::{
        chain_id::{MAINNET, TESTNET},
        types::FieldElement,
    },
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
        let value = value
            .to_str()
            .ok_or_else(|| clap::Error::raw(clap::ErrorKind::InvalidUtf8, "Invalid utf-8"))?;

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
    USDC,
    USDT,
    Other(FieldElement),
}

impl TokenKind {
    pub fn get_token_address(self) -> FieldElement {
        match self {
            Self::Ether => FieldElement::from_mont([
                4380532846569209554u64,
                17839402928228694863u64,
                17240401758547432026u64,
                418961398025637529u64,
            ]),

            Self::Dai => FieldElement::from_mont([
                9111736349608482743u64,
                12835366815636321047u64,
                4097671348364524325u64,
                173241921963463696u64,
            ]),

            Self::USDC => FieldElement::from_mont([
                5808361013446951402u64,
                13558485962494585092u64,
                9528015766451344574u64,
                198270530439797869u64,
            ]),

            Self::USDT => FieldElement::from_mont([
                12825534675109051809u64,
                4047367891602102096u64,
                7552378060304297298u64,
                570026286552673382u64,
            ]),

            Self::Other(addr) => addr,
        }
    }
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
        let value = value
            .to_str()
            .ok_or_else(|| clap::Error::raw(clap::ErrorKind::InvalidUtf8, "Invalid utf-8"))?;

        let value = value.to_lowercase();
        match value.as_str() {
            "ether" => Ok(TokenKind::Ether),
            "dai" => Ok(TokenKind::Dai),
            "usdc" => Ok(TokenKind::USDC),
            "usdt" => Ok(TokenKind::USDT),
            _ => Ok(TokenKind::Other(
                FieldElement::from_hex_be(&value)
                    .map_err(|e| clap::Error::raw(clap::ErrorKind::InvalidValue, e.to_string()))?,
            )),
        }
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::PossibleValue<'static>> + '_>> {
        let possible_values: Vec<PossibleValue<'static>> = vec![
            PossibleValue::new("ether"),
            PossibleValue::new("dai"),
            PossibleValue::new("usdc"),
            PossibleValue::new("usdt"),
        ];
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
        let value = value
            .to_str()
            .ok_or_else(|| clap::Error::raw(clap::ErrorKind::InvalidUtf8, "Invalid utf-8"))?;

        // There must be a more idiomatic way of doing this.
        if value.starts_with("0x") {
            let hash = FieldElement::from_hex_be(value)
                .map_err(|e| clap::Error::raw(clap::ErrorKind::InvalidValue, e))?;

            Ok(BlockId::Hash(hash))
        } else if let Ok(number) = value.parse::<u64>() {
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
    ) -> Result<Self::Value, clap::Error> {
        let value = value
            .to_str()
            .ok_or_else(|| clap::Error::raw(clap::ErrorKind::InvalidUtf8, "Invalid utf-8"))?;

        if value.parse::<u128>().is_ok() {
            FieldElement::from_str(value)
                .map_err(|e| clap::Error::raw(clap::ErrorKind::InvalidValue, e.to_string()))
        } else {
            match value {
                "SN_MAIN" => Ok(MAINNET),
                "SN_GOERLI" => Ok(TESTNET),
                _ => Err(clap::Error::raw(
                    clap::ErrorKind::InvalidValue,
                    "Invalid chain id",
                )),
            }
        }
    }

    fn possible_values(
        &self,
    ) -> Option<Box<dyn Iterator<Item = clap::PossibleValue<'static>> + '_>> {
        let possible_values: Vec<PossibleValue<'static>> = vec![
            PossibleValue::new("SN_MAIN"),
            PossibleValue::new("SN_GOERLI"),
        ];
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
    ) -> Result<Self::Value, clap::Error> {
        let value = value
            .to_str()
            .ok_or_else(|| clap::Error::raw(clap::ErrorKind::InvalidUtf8, "Invalid utf-8"))?;

        canonicalize_path(value).map_err(|e| clap::Error::raw(clap::ErrorKind::ValueValidation, e))
    }
}
