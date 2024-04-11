use clap::Parser;
use starknet::core::types::BlockId;

use crate::opts::display::DisplayOptions;
use crate::opts::starknet::StarknetOptions;
use crate::parser::BlockIdParser;

#[derive(Debug, Parser)]
pub struct AgeArgs {
    #[arg(next_line_help = true)]
    #[arg(default_value = "latest")]
    #[arg(value_parser = BlockIdParser)]
    #[arg(
        help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
    )]
    pub block_id: BlockId,

    #[arg(short = 'r', long)]
    #[arg(help_heading = "Display options")]
    pub human_readable: bool,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,
}

#[derive(Debug, Parser)]
pub struct BlockArgs {
    #[arg(next_line_help = true)]
    #[arg(value_name = "BLOCK_ID")]
    #[arg(default_value = "latest")]
    #[arg(value_parser = BlockIdParser)]
    #[arg(
        help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
    )]
    pub id: BlockId,

    #[arg(long)]
    #[arg(conflicts_with = "compact")]
    #[arg(help = "Get the full information (incl. transactions) of the block.")]
    pub full: bool,

    #[arg(long)]
    #[arg(help = "Get the block with the transaction hashes only, not the full transactions.")]
    pub compact: bool,

    #[command(flatten)]
    #[command(next_help_heading = "Display options")]
    pub display: DisplayOptions,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,
}

#[derive(Debug, Parser)]
pub struct BlockNumberArgs {
    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,
}
