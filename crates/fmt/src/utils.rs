use alloy_primitives::U256;

pub fn format_erc20_balance(balance: U256, symbol: &str, decimals: u8) -> String {
    use bigdecimal::{
        num_bigint::{BigInt, Sign},
        BigDecimal,
    };

    let decimal = BigDecimal::new(
        BigInt::from_bytes_be(Sign::Plus, &balance.to_be_bytes::<{ U256::BYTES }>()),
        decimals as i64,
    );

    format!("{decimal} {symbol}")
}
