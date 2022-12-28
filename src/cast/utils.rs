use std::fs;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};

use eyre::{eyre, Result};
use starknet::core::{
    types::{AbiEntry, FieldElement},
    utils::cairo_short_string_to_felt,
};

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

/// `file` path to the abi file
pub fn count_function_inputs<P>(abi_file: P, function_name: &str) -> Result<u64>
where
    P: AsRef<Path>,
{
    let abi = fs::read_to_string(abi_file)?;
    let abi = parse_abi_into_map(&abi)?;

    if let Some(AbiEntry::Function(function)) = abi.get(function_name) {
        let mut count = 0;

        for input in function.inputs.iter() {
            if input.r#type.eq("felt") {
                count += 1;
            } else {
                match abi.get(input.r#type.as_str()) {
                    Some(AbiEntry::Struct(s)) => count += s.size,
                    _ => return Err(eyre::eyre!("no type `{}` found in the abi", input.r#type)),
                }
            }
        }

        Ok(count)
    } else {
        Err(eyre!("no function `{}` found in the abi", function_name))
    }
}

pub fn parse_abi_into_map(abi_str: &str) -> Result<HashMap<String, AbiEntry>> {
    let abi: Vec<AbiEntry> = serde_json::from_str(abi_str)?;
    let mut map = HashMap::new();

    for elem in abi.into_iter() {
        let key = match &elem {
            AbiEntry::Event(v) => v.name.to_owned(),
            AbiEntry::Struct(v) => v.name.to_owned(),
            AbiEntry::Function(v) => v.name.to_owned(),
            AbiEntry::L1Handler(v) => v.name.to_owned(),
            AbiEntry::Constructor(v) => v.name.to_owned(),
        };
        map.insert(key, elem);
    }

    Ok(map)
}

pub fn canonicalize_path(path: impl AsRef<str>) -> Result<PathBuf> {
    let path = shellexpand::tilde(path.as_ref());
    Ok(dunce::canonicalize(path.to_string().as_str())?)
}
