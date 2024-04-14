use clap::Parser;
use starknet::core::types::BlockId;

use crate::opts::display::DisplayOptions;
use crate::opts::starknet::StarknetOptions;
use crate::parser::BlockIdParser;

#[derive(Debug, Parser)]
pub struct StateUpdateArgs {
    #[arg(next_line_help = true)]
    #[arg(default_value = "pending")]
    #[arg(value_parser = BlockIdParser)]
    #[arg(
        help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
    )]
    block_id: BlockId,

    // #[command(flatten)]
    // #[command(next_help_heading = "Display options")]
    // display: DisplayOptions,
    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
}
