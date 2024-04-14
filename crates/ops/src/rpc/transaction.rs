use eyre::Result;
use rika_args::{
    commands::rpc::{ReceiptArgs, TxArgs, TxCountArgs, TxStatusArgs},
    fmt::Pretty,
};
use starknet::providers::Provider;

use super::utils;

pub fn get(args: TxArgs) -> Result<()> {
    let TxArgs {
        hash,
        display,
        starknet,
    } = args;

    let provider = starknet.provider();
    let result = utils::do_call_with_mapped_rpc_err(provider.get_transaction_by_hash(hash))?;
    display.display(result)?;

    Ok(())
}

pub fn count(args: TxCountArgs) -> Result<()> {
    let TxCountArgs { block_id, starknet } = args;

    let provider = starknet.provider();
    let count = utils::do_call_with_mapped_rpc_err(provider.get_block_transaction_count(block_id))?;
    println!("{count}");

    Ok(())
}

pub fn status(args: TxStatusArgs) -> Result<()> {
    let TxStatusArgs { hash, starknet } = args;

    let provider = starknet.provider();
    let status = utils::do_call_with_mapped_rpc_err(provider.get_transaction_status(hash))?;
    println!("{}", status.prettify());

    Ok(())
}

pub fn receipt(args: ReceiptArgs) -> Result<()> {
    let ReceiptArgs {
        hash,
        display,
        starknet,
    } = args;

    let provider = starknet.provider();
    let receipt = utils::do_call_with_mapped_rpc_err(provider.get_transaction_receipt(hash))?;
    display.display(receipt)?;

    Ok(())
}
