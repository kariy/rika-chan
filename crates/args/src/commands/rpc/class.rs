use clap::Parser;
use starknet::core::types::{BlockId, FieldElement};

use crate::opts::starknet::StarknetOptions;
use crate::parser::BlockIdParser;

#[derive(Debug, Parser)]
pub struct ClassArgs {
    #[arg(value_name = "CLASS_HASH")]
    #[arg(help = "The hash of the requested contract class")]
    hash: FieldElement,

    #[arg(next_line_help = true)]
    #[arg(default_value = "latest")]
    #[arg(value_parser = BlockIdParser)]
    #[arg(help = "The hash of the requested block, or number (height) of the requested block, \
                  or a block tag (e.g. latest, pending).")]
    block_id: BlockId,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
}

#[derive(Debug, Parser)]
pub struct CodeArgs {
    #[arg(help = "The address of the contract whose class definition will be returned")]
    contract_address: FieldElement,

    #[arg(next_line_help = true)]
    #[arg(short, long = "block")]
    #[arg(default_value = "pending")]
    #[arg(value_parser = BlockIdParser)]
    #[arg(help = "The hash of the requested block, or number (height) of the requested block, \
                  or a block tag (e.g. latest, pending).")]
    block_id: BlockId,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
}
