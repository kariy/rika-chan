use eyre::Result;
use rika_args::{commands::rpc::CallArgs, fmt::Pretty};
use starknet::{
    core::{
        types::{BlockId, FieldElement, FunctionCall},
        utils::{get_selector_from_name, NonAsciiNameError},
    },
    providers::{Provider, ProviderError},
};

use crate::utils;

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
    let retdata = utils::block_on(contract_call(
        provider,
        contract_address,
        &function,
        input,
        block_id,
    ))?;
    println!("{}", retdata.prettify());

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum ContractCallError {
    #[error("invalid contract entrypoint name: {0}")]
    Selector(#[from] NonAsciiNameError),
    #[error(transparent)]
    Provider(#[from] ProviderError),
}

pub(crate) async fn contract_call<P: Provider>(
    provider: P,
    contract_address: FieldElement,
    entrypoint: &str,
    calldata: Vec<FieldElement>,
    block: BlockId,
) -> Result<Vec<FieldElement>, ContractCallError> {
    let entrypoint = get_selector_from_name(entrypoint)?;
    let request = FunctionCall {
        calldata,
        contract_address,
        entry_point_selector: entrypoint,
    };

    let retdata = provider.call(request, block).await?;
    Ok(retdata)
}
