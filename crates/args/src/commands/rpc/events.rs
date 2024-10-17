use clap::Parser;
use starknet::core::types::{BlockId, FieldElement};

use crate::opts::starknet::StarknetOptions;
use crate::parser::{parse_event_keys, BlockIdParser};

#[derive(Debug, Parser)]
pub struct EventsArgs {
    #[arg(num_args(0..))]
    #[arg(help = r"The values used to filter the events.
Example: 0x12,0x23 0x34,0x45 - Which will be parsed as [[0x12,0x23], [0x34,0x45]]")]
    #[arg(value_parser = parse_event_keys)]
    keys: Option<Vec<Vec<FieldElement>>>,

    #[arg(required = true)]
    #[arg(short = 's', long)]
    #[arg(help = "The number of events to return in each page")]
    chunk_size: u64,

    #[arg(short = 'C', long)]
    #[arg(value_name = "CONTRACT_ADDRESS")]
    #[arg(help = "Address of the contract emitting the events")]
    from: Option<FieldElement>,

    #[arg(short, long)]
    #[arg(value_parser(BlockIdParser))]
    from_block: Option<BlockId>,

    #[arg(short, long)]
    #[arg(value_parser(BlockIdParser))]
    to_block: Option<BlockId>,

    #[arg(short = 'c', long)]
    #[arg(help = "A pointer to the last element of the delivered page, use this token in a \
                  subsequent query to obtain the next page")]
    continuation_token: Option<String>,

    #[command(flatten)]
    #[command(next_help_heading = "Starknet options")]
    starknet: StarknetOptions,
}
