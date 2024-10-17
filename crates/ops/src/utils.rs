use std::future::Future;

use alloy_primitives::U256;
use color_eyre::eyre::Context;
use color_eyre::Result;
use starknet::core::types::FieldElement;

/// Blocks on a future, returning the output.
pub fn block_on<F, T>(future: F) -> T
where
    F: Future<Output = T>,
{
    use tokio::runtime::Builder;
    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build async runtime")
        .block_on(future)
}

pub fn to_u256(low: FieldElement, high: FieldElement) -> Result<U256> {
    let low: u128 = low.try_into().context("parsing low")?;
    let high: u128 = high.try_into().context("parsing high")?;
    Ok(U256::from(high) << 128 | U256::from(low))
}
