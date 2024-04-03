use clap::Parser;
use starknet::core::types::{BlockId, FieldElement};

use crate::opts::display::DisplayOptions;
use crate::opts::starknet::StarknetOptions;
use crate::parser::BlockIdParser;

#[derive(Debug, Parser)]
pub struct TxArgs {
    #[arg(value_name = "TX_HASH")]
    pub hash: FieldElement,

    #[command(flatten)]
    #[command(next_help_heading = "Display options")]
    pub display: DisplayOptions,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,
}

#[derive(Debug, Parser)]
pub struct TxCountArgs {
    #[arg(next_line_help = true)]
    #[arg(default_value = "pending")]
    #[arg(value_parser = BlockIdParser)]
    #[arg(
        help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
    )]
    pub block_id: BlockId,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,
}

#[derive(Debug, Parser)]
pub struct TxStatusArgs {
    #[arg(value_name = "TX_HASH")]
    pub hash: FieldElement,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,
}

#[derive(Debug, Parser)]
pub struct ReceiptArgs {
    #[arg(value_name = "TX_HASH")]
    pub hash: FieldElement,

    #[command(flatten)]
    #[command(next_help_heading = "Display options")]
    pub display: DisplayOptions,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    pub starknet: StarknetOptions,
}
