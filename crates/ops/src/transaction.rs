use eyre::Result;
use rika_args::{
    commands::transaction::{ReceiptArgs, TxArgs, TxCountArgs, TxStatusArgs},
    fmt::Pretty,
};
use starknet::providers::Provider;

use crate::utils;

pub fn get(args: TxArgs) -> Result<()> {
    let TxArgs {
        hash,
        display,
        starknet,
    } = args;

    let provider = starknet.provider();
    let result = utils::block_on(provider.get_transaction_by_hash(hash))?;
    display.display(result)?;

    Ok(())
}

pub fn count(args: TxCountArgs) -> Result<()> {
    let TxCountArgs { block_id, starknet } = args;

    let provider = starknet.provider();
    let count = utils::block_on(provider.get_block_transaction_count(block_id))?;
    println!("{count}");

    Ok(())
}

pub fn status(args: TxStatusArgs) -> Result<()> {
    let TxStatusArgs { hash, starknet } = args;

    let provider = starknet.provider();
    let status = utils::block_on(provider.get_transaction_status(hash))?;
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
    let receipt = utils::block_on(provider.get_transaction_receipt(hash))?;
    display.display(receipt)?;

    Ok(())
}
