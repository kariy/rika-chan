use clap::Parser;

use crate::opts::starknet::StarknetOptions;

#[derive(Debug, Parser)]
pub struct ChainIdArgs {
    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
}

#[derive(Debug, Parser)]
pub struct SyncingArgs {
    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
}
