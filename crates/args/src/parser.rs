use std::str::FromStr;

use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::{Error, ErrorKind};
use starknet::core::types::{BlockId, BlockTag};
use starknet::core::types::{FieldElement, FromStrError};
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

/// Parser for known ERC20 token name to its contract address.
#[derive(Debug, Clone)]
pub struct TokenAddressParser;

impl TokenAddressParser {
    // 0x049D36570D4e46f48e99674bd3fcc84644DdD6b96F7C741B1562B82f9e004dC7
    pub const ETH: FieldElement = FieldElement::from_mont([
        4380532846569209554,
        17839402928228694863,
        17240401758547432026,
        418961398025637529,
    ]);

    // 0x04718f5a0Fc34cC1AF16A1cdee98fFB20C31f5cD61D6Ab07201858f4287c938D
    pub const STRK: FieldElement = FieldElement::from_mont([
        16432072983745651214,
        1325769094487018516,
        5134018303144032807,
        468300854463065062,
    ]);

    // 0x053C91253BC9682c04929cA02ED00b3E423f6710D2ee7e0D5EBB06F3eCF368A8
    pub const USDC: FieldElement = FieldElement::from_mont([
        5808361013446951402,
        13558485962494585092,
        9528015766451344574,
        198270530439797869,
    ]);
}

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
            "eth" => Self::ETH,
            "usdc" => Self::USDC,
            "strk" => Self::STRK,

            value => FieldElement::from_str(value).map_err(|e| match e {
                FromStrError::InvalidCharacter => Error::raw(
                    ErrorKind::InvalidValue,
                    format!("value must be an address or one of the known tokens"),
                ),
                FromStrError::OutOfRange => Error::raw(
                    ErrorKind::InvalidValue,
                    format!("unknown token address '{e}'"),
                ),
            })?,
        };

        Ok(address)
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        Some(Box::new(
            [
                PossibleValue::new("ETH"),
                PossibleValue::new("USDC"),
                PossibleValue::new("STRK"),
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

#[cfg(test)]
mod tests {
    use clap::builder::TypedValueParser;
    use starknet::core::types::FieldElement;

    use crate::parser::TokenAddressParser;

    #[test]
    fn test_parse_token_name() -> eyre::Result<()> {
        let parser = super::TokenAddressParser;
        let eth = parser.parse_ref(&clap::Command::new("test"), None, "ETH".as_ref())?;
        let usdc = parser.parse_ref(&clap::Command::new("test"), None, "USDC".as_ref())?;
        let strk = parser.parse_ref(&clap::Command::new("test"), None, "STRK".as_ref())?;
        let address = parser.parse_ref(&clap::Command::new("test"), None, "0x123".as_ref())?;
        let random = parser.parse_ref(&clap::Command::new("test"), None, "DOGE".as_ref());

        assert_eq!(eth, TokenAddressParser::ETH);
        assert_eq!(usdc, TokenAddressParser::USDC);
        assert_eq!(strk, TokenAddressParser::STRK);
        assert_eq!(address, FieldElement::from(0x123u16));
        assert!(random
            .unwrap_err()
            .to_string()
            .contains("value must be an address or one of the known tokens"));

        Ok(())
    }
}
