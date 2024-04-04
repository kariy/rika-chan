use std::future::join;

use alloy_primitives::U256;
use eyre::{eyre, ContextCompat, Report, Result};
use rika_args::commands::balance::BalanceArgs;
use starknet::{
    core::{
        types::{BlockId, FieldElement, FunctionCall, StarknetError},
        utils::parse_cairo_short_string,
    },
    macros::selector,
    providers::{Provider, ProviderError},
};

use crate::utils;

pub fn get(args: BalanceArgs) -> Result<()> {
    let BalanceArgs {
        address,
        token,
        raw,
        block_id,
        starknet,
    } = args;

    let provider = starknet.provider();
    let (metadata, balance) = utils::block_on(join!(
        get_token_metadata(&provider, block_id, token),
        get_balance(&provider, block_id, token, address)
    ));

    let balance = balance?;
    let (symbol, decimals) = metadata?;

    if raw {
        println!("{:#x}", balance);
    } else {
        println!("{}", format_balance(balance, &symbol, decimals));
    }

    Ok(())
}

async fn get_balance<P>(
    provider: P,
    block_id: BlockId,
    contract_address: FieldElement,
    address: FieldElement,
) -> Result<U256>
where
    P: Provider,
{
    fn handle_error(err: ProviderError) -> Report {
        match err {
            ProviderError::StarknetError(StarknetError::ContractNotFound) => {
                eyre!("token contract not found")
            }
            e => eyre!(e),
        }
    }

    let call = FunctionCall {
        contract_address,
        calldata: vec![address],
        entry_point_selector: selector!("balanceOf"),
    };

    let retdata = provider.call(call, block_id).await.map_err(handle_error)?;
    // the convention is to return a u256, which means there are two felts
    let low = retdata.first().context("missing low value")?;
    let high = retdata.last().context("missing high value")?;

    utils::to_u256(*low, *high)
}

async fn get_token_metadata<P>(
    provider: P,
    block_id: BlockId,
    contract_address: FieldElement,
) -> Result<(String, u8)>
where
    P: Provider + Sync,
{
    async fn get_decimals(
        provider: impl Provider,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<u8> {
        let request = FunctionCall {
            contract_address,
            calldata: Vec::new(),
            entry_point_selector: selector!("decimals"),
        };

        let result = provider.call(request, block_id).await?;
        let decimals = result.first().context("missing decimals in call retdata")?;
        Ok((*decimals).try_into()?)
    }

    async fn get_symbol(
        provider: impl Provider,
        block_id: BlockId,
        contract_address: FieldElement,
    ) -> Result<String> {
        let request = FunctionCall {
            contract_address,
            calldata: Vec::new(),
            entry_point_selector: selector!("symbol"),
        };

        let result = provider.call(request, block_id).await?;
        let symbol = result.first().context("missing symbol in call retdata")?;
        Ok(parse_cairo_short_string(symbol)?)
    }

    let (symbol, decimals) = join!(
        get_symbol(&provider, block_id, contract_address),
        get_decimals(&provider, block_id, contract_address)
    )
    .await;

    Ok((symbol?, decimals?))
}

fn format_balance(balance: U256, symbol: &str, decimals: u8) -> String {
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
