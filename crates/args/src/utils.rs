use std::path::PathBuf;

use eyre::{Context, Result};
use starknet::core::{types::FieldElement, utils::cairo_short_string_to_felt};

pub fn parse_hex_or_str_as_felt(data: &str) -> Result<FieldElement> {
    if data.starts_with("0x") {
        Ok(FieldElement::from_hex_be(data)?)
    } else {
        Ok(cairo_short_string_to_felt(data)?)
    }
}

/// Canonicalizes a path and performs both tilde and environment expansions in the default system context.
pub fn canonicalize_path(path: &str) -> Result<PathBuf> {
    let expanded = shellexpand::full(path).context(format!("failed to expand path {path}"))?;
    let path = PathBuf::from(expanded.into_owned());
    Ok(dunce::canonicalize(path)?)
}
