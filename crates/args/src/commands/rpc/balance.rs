use clap::Parser;
use starknet::core::types::{BlockId, FieldElement};

use crate::opts::starknet::StarknetOptions;
use crate::parser::{BlockIdParser, TokenAddressParser};

#[derive(Debug, Parser)]
pub struct BalanceArgs {
    /// The address whose balance you want to query.
    #[arg(value_name = "ADDRESS")]
    pub address: FieldElement,

    /// The token you want to query the balance of.
    #[arg(value_parser = TokenAddressParser)]
    #[arg(default_value = "STRK")]
    pub token: FieldElement,

    /// Return the balance as a raw integer value in hexadecimal form.
    #[arg(long)]
    pub raw: bool,

    /// The hash of the requested block, or number (height) of the requested block, or a block tag
    /// (e.g. latest, pending).
    #[arg(next_line_help = true)]
    #[arg(short, long = "block")]
    #[arg(default_value = "pending")]
    #[arg(value_parser = BlockIdParser)]
    pub block_id: BlockId,

    #[command(flatten)]
    pub starknet: StarknetOptions,
}
