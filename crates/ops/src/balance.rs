use std::future::join;

use alloy_primitives::U256;
use eyre::{eyre, ContextCompat, Report, Result};
use rika_args::commands::balance::BalanceArgs;
use starknet::{
    core::{
        types::{BlockId, FieldElement, StarknetError},
        utils::parse_cairo_short_string,
    },
    providers::{Provider, ProviderError},
};

use crate::{
    call::{contract_call, ContractCallError},
    utils,
};

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
    fn handle_error(err: ContractCallError) -> Report {
        match err {
            ContractCallError::Provider(ProviderError::StarknetError(
                StarknetError::ContractNotFound,
            )) => {
                eyre!("token contract not found")
            }
            e => eyre!(e),
        }
    }

    let retdata = contract_call(
        provider,
        contract_address,
        "balanceOf",
        vec![address],
        block_id,
    )
    .await
    .map_err(handle_error)?;

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

async fn get_decimals(
    provider: impl Provider,
    block_id: BlockId,
    contract_address: FieldElement,
) -> Result<u8> {
    let retdata =
        contract_call(provider, contract_address, "decimals", Vec::new(), block_id).await?;
    let dec = retdata.first().context("missing value in call retdata")?;
    Ok((*dec).try_into()?)
}

async fn get_symbol(
    provider: impl Provider,
    block_id: BlockId,
    contract_address: FieldElement,
) -> Result<String> {
    let retdata = contract_call(provider, contract_address, "symbol", Vec::new(), block_id).await?;
    let symbol = retdata.first().context("missing value in call retdata")?;
    Ok(parse_cairo_short_string(symbol)?)
}
