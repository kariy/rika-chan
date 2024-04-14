use eyre::{Context, Result};
use rika_args::commands::rpc::{ChainIdArgs, SyncingArgs};
use starknet::{
    core::{
        types::{SyncStatus, SyncStatusType},
        utils::parse_cairo_short_string,
    },
    providers::Provider,
};

use crate::utils;

pub fn id(args: ChainIdArgs) -> Result<()> {
    let ChainIdArgs { starknet } = args;
    let id = utils::block_on(starknet.provider().chain_id())?;
    let parsed_id = parse_cairo_short_string(&id).context("failed to parse chain id")?;
    println!("{id:#x} ({parsed_id})",);
    Ok(())
}

pub fn syncing(args: SyncingArgs) -> Result<()> {
    // let SyncingArgs { display, starknet } = args;
    // let status = utils::block_on(starknet.provider().syncing())?;
    Ok(())
}
