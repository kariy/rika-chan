use alloy_primitives::U256;

/// Format the ERC20 token balance into a human-readable string. The resulting string will be
/// rounded to 2 decimal places.
///
/// # Example
///
/// ```
/// let bal = U256::from(12345);
/// let symbol = "ETH";
/// let decimals = 4;
///
/// let formatted = format_erc20_balance(bal, symbol, decimals);
/// println!("{formatted}")); // 1.23 ETH
/// ```
pub fn format_erc20_balance(balance: U256, symbol: &str, decimals: u8) -> String {
    use bigdecimal::num_bigint::{BigInt, Sign};
    use bigdecimal::BigDecimal;

    // Round the balance to 2 decimal places
    let value = BigInt::from_bytes_be(Sign::Plus, &balance.to_be_bytes::<{ U256::BYTES }>());
    let decimal = BigDecimal::new(value, decimals as i64).round(2);

    format!("{decimal} {symbol}")
}
