use clap::Parser;
use starknet::core::types::{BlockId, FieldElement};

use crate::opts::starknet::StarknetOptions;
use crate::parser::BlockIdParser;

#[derive(Debug, Parser)]
pub struct CallArgs {
    #[arg(display_order = 1)]
    contract_address: FieldElement,

    #[arg(display_order = 2)]
    #[arg(help = "The name of the function to be called")]
    #[arg(value_name = "FUNCTION_NAME")]
    function: String,

    #[arg(short, long)]
    #[arg(display_order = 3)]
    #[arg(value_delimiter = ',')]
    #[arg(help = "Comma seperated values e.g., 0x12345,0x69420,...")]
    input: Vec<FieldElement>,

    #[arg(next_line_help = true)]
    #[arg(display_order = 5)]
    #[arg(short, long = "block")]
    #[arg(default_value = "pending")]
    #[arg(value_parser = BlockIdParser)]
    #[arg(
        help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
    )]
    block_id: BlockId,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
}
