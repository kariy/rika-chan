use clap::ArgAction;
use clap::Parser;
use starknet::core::types::BlockId;

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
    block_id: BlockId,

    #[arg(short = 'r', long)]
    #[arg(help_heading = "Display options")]
    human_readable: bool,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
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
    id: BlockId,

    #[arg(long)]
    #[arg(action(ArgAction::SetTrue))]
    #[arg(help = "Get the full information (incl. transactions) of the block.")]
    full: bool,

    #[arg(long)]
    field: Option<String>,

    #[arg(short = 'j', long = "json")]
    #[arg(help_heading = "Display options")]
    to_json: bool,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
}

#[derive(Debug, Parser)]
pub struct BlockNumberArgs {
    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
}
