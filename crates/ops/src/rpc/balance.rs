use std::future::join;

use alloy_primitives::U256;
use eyre::{eyre, ContextCompat, Report, Result};
use rika_args::commands::rpc::BalanceArgs;
use starknet::{
    core::{
        types::{BlockId, FieldElement, StarknetError},
        utils::parse_cairo_short_string,
    },
    macros::selector,
    providers::{Provider, ProviderError},
};

use super::call::contract_call;
use crate::utils::{self};

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
        println!("{}", rika_fmt::utils::format_erc20_balance(balance, &symbol, decimals));
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
    let handle_error = |err: ProviderError| -> Report {
        match err {
            ProviderError::StarknetError(StarknetError::ContractNotFound) => {
                eyre!("token with address '{address:#x}' is not found")
            }
            e => eyre!(e),
        }
    };

    let retdata = contract_call(
        provider,
        contract_address,
        selector!("balanceOf"),
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

async fn get_decimals(
    provider: impl Provider,
    block_id: BlockId,
    contract_address: FieldElement,
) -> Result<u8> {
    let retdata = contract_call(
        provider,
        contract_address,
        selector!("decimals"),
        Vec::new(),
        block_id,
    )
    .await?;
    let dec = retdata.first().context("missing value in call retdata")?;
    Ok((*dec).try_into()?)
}

async fn get_symbol(
    provider: impl Provider,
    block_id: BlockId,
    contract_address: FieldElement,
) -> Result<String> {
    let retdata = contract_call(
        provider,
        contract_address,
        selector!("symbol"),
        Vec::new(),
        block_id,
    )
    .await?;
    let symbol = retdata.first().context("missing value in call retdata")?;
    Ok(parse_cairo_short_string(symbol)?)
}
