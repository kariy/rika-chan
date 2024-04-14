use clap::Parser;

use crate::opts::{display::DisplayOptions, starknet::StarknetOptions};

#[derive(Debug, Parser)]
pub struct ChainIdArgs {
    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,
}

#[derive(Debug, Parser)]
pub struct SyncingArgs {
    // #[command(flatten)]
    // #[command(next_help_heading = "Display options")]
    // pub display: DisplayOptions,
    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,
}
