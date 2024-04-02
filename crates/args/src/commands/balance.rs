use clap::Parser;
use starknet::core::types::{BlockId, FieldElement};

use crate::opts::starknet::StarknetOptions;
use crate::parser::{BlockIdParser, TokenAddressParser};

#[derive(Debug, Parser)]
pub struct BalanceArgs {
    #[arg(value_name = "ADDRESS")]
    #[arg(help = "The address whose balance you want to query.")]
    address: FieldElement,

    #[arg(help = "The token you want to query the balance of.")]
    #[arg(value_parser(TokenAddressParser))]
    token: FieldElement,

    #[arg(next_line_help = true)]
    #[arg(short, long = "block")]
    #[arg(default_value = "pending")]
    #[arg(value_parser(BlockIdParser))]
    #[arg(
        help = "The hash of the requested block, or number (height) of the requested block, or a block tag (e.g. latest, pending)."
    )]
    block_id: BlockId,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
}
