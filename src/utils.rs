use eyre::Result;
use starknet_core::{
    crypto::{pedersen_hash, Signature},
    types::FieldElement,
    utils::{cairo_short_string_to_felt, starknet_keccak},
};

// const STARKNET_ACCOUNT_FILEPATH: &'static str = "~/.starknet_accounts";
pub const SIGNED_FELT_MIN: &'static str =
    "-1809251394333065606848661391547535052811553607665798349986546028067936010240";

pub const SIGNED_FELT_MAX: &'static str =
    "1809251394333065606848661391547535052811553607665798349986546028067936010240";

pub struct Ecdsa;

impl Ecdsa {
    pub fn sign() -> Result<Signature> {
        todo!()
    }
    pub fn verify() -> Result<bool> {
        todo!()
    }
}

pub struct Misc;

impl Misc {
    fn parse_as_felt(data: &str) -> Result<FieldElement> {
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

    pub fn hex_encode<T>(data: T) -> String
    where
        T: AsRef<[u8]>,
    {
        let hex_str = hex::encode(data);
        format!("0x{hex_str}")
    }

    pub fn keccak(data: &str) -> Result<String> {
        let hash = match data.as_bytes() {
            // 0x prefix => read as hex data
            [b'0', b'x', rest @ ..] => starknet_keccak(&hex::decode(rest)?),
            // No 0x prefix => read as text
            _ => starknet_keccak(data.as_bytes()),
        };

        Ok(Self::hex_encode(hash.to_bytes_be()))
    }

    pub fn pedersen(x: &str, y: &str) -> Result<String> {
        let x = Self::parse_as_felt(x)?;
        let y = Self::parse_as_felt(y)?;

        let hash = pedersen_hash(&x, &y);

        Ok(Self::hex_encode(hash.to_bytes_be()))
    }
}
