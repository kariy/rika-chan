use super::parser::ChainParser;

use clap::Parser;
use reqwest::Url;
use starknet::core::types::FieldElement;

#[derive(Debug, Clone, Parser)]
pub struct StarkNetOptions {
    #[clap(long)]
    #[clap(value_name = "URL")]
    #[clap(help = "The RPC endpoint")]
    #[clap(env = "STARKNET_RPC_URL")]
    #[clap(default_value = "http://localhost:5050/rpc")]
    pub rpc_url: Url,

    #[clap(long)]
    #[clap(env = "STARKNET_CHAIN")]
    #[clap(value_name = "CHAIN_ID")]
    #[clap(value_parser(ChainParser))]
    pub chain: Option<FieldElement>,
}
