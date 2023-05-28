pub mod fmt;

use std::path::PathBuf;

use eyre::Result;
use starknet::core::{types::FieldElement, utils::cairo_short_string_to_felt};

// const STARKNET_ACCOUNT_FILEPATH: &'static str = "~/.starknet_accounts";
pub const SIGNED_FELT_MIN: &str =
    "-1809251394333065606848661391547535052811553607665798349986546028067936010240";

pub const SIGNED_FELT_MAX: &str =
    "1809251394333065606848661391547535052811553607665798349986546028067936010240";

pub fn parse_hex_or_str_as_felt(data: &str) -> Result<FieldElement> {
    let felt = match data.as_bytes() {
        // 0x prefix => read as hex data
        [b'0', b'x', restx @ ..] => {
            // Make sure is valid hex string
            FieldElement::from_hex_be(&hex::encode(hex::decode(restx)?))?
        }
        // No 0x prefix => read as text
        _ => cairo_short_string_to_felt(data)?,
    };

    Ok(felt)
}

pub fn canonicalize_path(path: impl AsRef<str>) -> Result<PathBuf> {
    let path = shellexpand::tilde(path.as_ref());
    Ok(dunce::canonicalize(path.to_string().as_str())?)
}
