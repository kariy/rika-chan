use crate::cmd::parser::ChainParser;

use std::{fmt, str::FromStr};

use clap::Parser;
use reqwest::Url;
use starknet::core::{
    chain_id::{MAINNET, TESTNET, TESTNET2},
    types::FieldElement,
};

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

#[derive(Debug, Clone)]
pub enum StarknetChain {
    Mainnet,
    Testnet,
    Testnet2,
}

#[allow(unused)]
impl StarknetChain {
    pub fn get_id(&self) -> FieldElement {
        match self {
            Self::Mainnet => MAINNET,
            Self::Testnet => TESTNET,
            Self::Testnet2 => TESTNET2,
        }
    }
}

impl fmt::Display for StarknetChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mainnet => write!(f, "mainnet"),
            Self::Testnet => write!(f, "testnet"),
            Self::Testnet2 => write!(f, "testnet2"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid chain id")]
pub struct InvalidStarknetChain;

impl FromStr for StarknetChain {
    type Err = InvalidStarknetChain;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "mainnet" => Ok(Self::Mainnet),
            "testnet" => Ok(Self::Testnet),
            "testnet2" => Ok(Self::Testnet2),
            _ => Err(InvalidStarknetChain),
        }
    }
}
