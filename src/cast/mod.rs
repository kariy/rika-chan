pub mod utils;

use eyre::{Report, Result};
use starknet::core::{
    crypto::{ecdsa_sign, ecdsa_verify, pedersen_hash, Signature},
    types::FieldElement,
    utils::{cairo_short_string_to_felt, parse_cairo_short_string, starknet_keccak},
};

pub struct Cast {}

pub struct SimpleCast;

impl SimpleCast {
    pub fn address_zero() -> String {
        utils::hex_encode(FieldElement::ZERO.to_bytes_be())
    }

    pub fn to_hex(dec: &FieldElement) -> Result<String> {
        Ok(utils::hex_encode(dec.to_bytes_be()))
    }

    pub fn to_dec(hex: &FieldElement) -> Result<String> {
        Ok(hex.to_string())
    }

    pub fn keccak(data: &str) -> Result<String> {
        let hash = match data.as_bytes() {
            // 0x prefix => read as hex data
            [b'0', b'x', rest @ ..] => starknet_keccak(&hex::decode(rest)?),
            // No 0x prefix => read as text
            _ => starknet_keccak(data.as_bytes()),
        };

        Ok(utils::hex_encode(hash.to_bytes_be()))
    }

    pub fn pedersen(x: &str, y: &str) -> Result<String> {
        let x = utils::parse_hex_or_str_as_felt(x)?;
        let y = utils::parse_hex_or_str_as_felt(y)?;
        let hash = pedersen_hash(&x, &y);

        Ok(utils::hex_encode(hash.to_bytes_be()))
    }

    pub fn max_felt() -> String {
        FieldElement::MAX.to_string()
    }

    pub fn max_signed_felt() -> &'static str {
        utils::SIGNED_FELT_MAX
    }

    pub fn min_signed_felt() -> &'static str {
        utils::SIGNED_FELT_MIN
    }

    pub fn str_to_felt(short_str: &str) -> Result<String> {
        let felt = cairo_short_string_to_felt(short_str)?;
        Ok(utils::hex_encode(felt.to_bytes_be()))
    }

    pub fn from_utf8(felt: &FieldElement) -> Result<String> {
        parse_cairo_short_string(&felt).map_err(|e| Report::new(e))
    }

    pub fn ecdsa_sign(private_key: &FieldElement, message_hash: &str) -> Result<Signature> {
        ecdsa_sign(private_key, &FieldElement::from_hex_be(message_hash)?)
            .map_err(|e| Report::new(e))
    }

    pub fn ecdsa_verify(
        public_key: &FieldElement,
        message_hash: &str,
        signature_r: &FieldElement,
        signature_s: &FieldElement,
    ) -> Result<bool> {
        ecdsa_verify(
            public_key,
            &FieldElement::from_hex_be(message_hash)?,
            &Signature {
                r: signature_r.to_owned(),
                s: signature_s.to_owned(),
            },
        )
        .map_err(|e| Report::new(e))
    }
}
