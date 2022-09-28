use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::{io::Error, path::Path};

use eyre::Result;
use starknet::core::{types::FieldElement, utils::cairo_short_string_to_felt};

// const STARKNET_ACCOUNT_FILEPATH: &'static str = "~/.starknet_accounts";
pub const SIGNED_FELT_MIN: &'static str =
    "-1809251394333065606848661391547535052811553607665798349986546028067936010240";

pub const SIGNED_FELT_MAX: &'static str =
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

// istg this code could be better
// placeholder - will change
pub fn count_function_inputs_from_abi(abi: &str, function_name: &str) -> Result<u8> {
    let abi_str = fs::read_to_string(Path::new(abi))?;
    let abi = {
        let abi = serde_json::Value::from_str(&abi_str)?
            .as_array()
            .ok_or(eyre::eyre!("invalid abi format"))?
            .to_owned();
        parse_abi_as_map(abi)?
    };

    if let Some(value) = abi.get(function_name) {
        let mut count: u8 = 0;

        for e in value["inputs"].as_array().unwrap().iter() {
            let is_type = e["type"].as_str().unwrap();

            if is_type.eq("felt") {
                count += 1;
            } else {
                let elem = abi
                    .get(is_type)
                    .ok_or(eyre::eyre!("no `{}` found in the abi", is_type))?;
                count += elem.get("size").unwrap().as_u64().unwrap() as u8;
            }
        }

        Ok(count)
    } else {
        return Err(eyre::eyre!("no `{}` found in the abi", function_name));
    }
}

pub fn parse_abi_as_map(abi: Vec<serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
    let mut map = HashMap::new();

    for elem in abi.into_iter() {
        if let Some(key) = elem.get("name") {
            let key = key.as_str().unwrap();
            map.insert(String::from(key), elem);
        }
    }

    Ok(map)
}
