use std::{path::PathBuf, str::FromStr};

use eyre::Result;
use starknet::core::{
    types::{FieldElement, FromStrError},
    utils::cairo_short_string_to_felt,
};

pub fn parse_hex_or_str_as_felt(data: &str) -> Result<FieldElement> {
    if data.starts_with("0x") {
        Ok(FieldElement::from_hex_be(data)?)
    } else {
        Ok(cairo_short_string_to_felt(data)?)
    }
}

pub fn canonicalize_path(path: &str) -> Result<PathBuf> {
    let path = PathBuf::from(shellexpand::full(path)?.into_owned());
    Ok(dunce::canonicalize(path)?)
}

// Expected format for keys : 0x124123,0x14123,0x1342
// where each array is a key
pub fn parse_event_keys(value: &str) -> std::result::Result<Vec<FieldElement>, FromStrError> {
    value.split(',').map(FieldElement::from_str).collect()
}
