use clap::Parser;
use starknet::core::types::{BlockId, FieldElement};

use crate::opts::starknet::StarknetOptions;
use crate::parser::BlockIdParser;

#[derive(Debug, Parser)]
pub struct ContractClassArgs {
    #[arg(help = "The address of the contract whose class hash will be returned")]
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

#[derive(Debug, Parser)]
pub struct StorageArgs {
    contract_address: FieldElement,

    index: FieldElement,

    #[arg(next_line_help = true)]
    #[arg(short, long = "block")]
    #[arg(default_value = "pending")]
    #[arg(value_parser = BlockIdParser)]
    #[arg(help = "The hash of the requested block, or number (height) of the requested block, \
                  or a block tag (e.g. pending, pending).")]
    block_id: BlockId,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
}

#[derive(Debug, Parser)]
pub struct NonceArgs {
    contract_address: FieldElement,

    #[arg(next_line_help = true)]
    #[arg(default_value = "pending")]
    #[arg(value_parser = BlockIdParser)]
    #[arg(help = "The hash of the requested block, or number (height) of the requested block, \
                  or a block tag (e.g. pending, pending).")]
    block_id: BlockId,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
}
