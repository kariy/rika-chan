use eyre::{Context, Result};
use rika_args::commands::rpc::CallArgs;
use rika_fmt::Pretty;
use starknet::{
    core::{
        types::{BlockId, FieldElement, FunctionCall},
        utils::get_selector_from_name,
    },
    providers::{Provider, ProviderError},
};

use super::utils;

// TODO: parse the return data according to the ABI?
pub fn call(args: CallArgs) -> Result<()> {
    let CallArgs {
        contract_address,
        function,
        input,
        block_id,
        starknet,
    } = args;

    let provider = starknet.provider();

    let selector = get_selector_from_name(&function)
        .with_context(|| format!("invalid contract entrypoint name '{function}'"))?;
    let retdata = utils::do_call_with_mapped_rpc_err(contract_call(
        provider,
        contract_address,
        selector,
        input,
        block_id,
    ))?;

    println!("{}", retdata.prettify());

    Ok(())
}

pub(crate) async fn contract_call<P: Provider>(
    provider: P,
    contract_address: FieldElement,
    entry_point_selector: FieldElement,
    calldata: Vec<FieldElement>,
    block: BlockId,
) -> Result<Vec<FieldElement>, ProviderError> {
    Ok(provider
        .call(
            FunctionCall {
                calldata,
                contract_address,
                entry_point_selector,
            },
            block,
        )
        .await?)
}
