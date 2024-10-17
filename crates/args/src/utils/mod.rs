pub mod fs;
pub mod json;

use color_eyre::Result;
use starknet::core::types::FieldElement;
use starknet::core::utils::cairo_short_string_to_felt;

pub fn parse_hex_or_str_as_felt(data: &str) -> Result<FieldElement> {
    if data.starts_with("0x") {
        Ok(FieldElement::from_hex_be(data)?)
    } else {
        Ok(cairo_short_string_to_felt(data)?)
    }
}
