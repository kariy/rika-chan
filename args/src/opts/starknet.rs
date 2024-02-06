use crate::parser::ChainParser;

use std::{fmt, str::FromStr};

use clap::Args;
use reqwest::Url;
use starknet::core::chain_id::{MAINNET, TESTNET, TESTNET2};
use starknet::core::types::FieldElement;
use starknet::providers::{jsonrpc::HttpTransport, JsonRpcClient};

#[derive(Debug, Clone, Args)]
pub struct StarknetOptions {
    #[arg(long)]
    #[arg(value_name = "URL")]
    #[arg(help = "The RPC endpoint")]
    #[arg(env = "STARKNET_RPC_URL")]
    #[arg(default_value = "http://localhost:5050/rpc")]
    pub rpc_url: Url,

    #[arg(long)]
    #[arg(env = "STARKNET_CHAIN")]
    #[arg(value_name = "CHAIN_ID")]
    #[arg(value_parser(ChainParser))]
    pub chain: Option<FieldElement>,
}

impl StarknetOptions {
    pub fn provider(&self) -> JsonRpcClient<HttpTransport> {
        JsonRpcClient::new(HttpTransport::new(self.rpc_url.clone()))
    }
}

#[derive(Debug, Clone)]
pub enum StarknetChain {
    Mainnet,
    Testnet,
    Testnet2,
}

impl StarknetChain {
    pub fn id(&self) -> FieldElement {
        match self {
            Self::Mainnet => MAINNET,
            Self::Testnet => TESTNET,
            Self::Testnet2 => TESTNET2,
        }
    }

    pub fn options() -> Vec<String> {
        vec![
            Self::Mainnet.to_string(),
            Self::Testnet.to_string(),
            Self::Testnet2.to_string(),
        ]
    }
}

impl From<FieldElement> for StarknetChain {
    fn from(chain_id: FieldElement) -> Self {
        if chain_id == MAINNET {
            Self::Mainnet
        } else if chain_id == TESTNET {
            Self::Testnet
        } else if chain_id == TESTNET2 {
            Self::Testnet2
        } else {
            panic!("{}", InvalidStarknetChain(format!("{chain_id:#x}")))
        }
    }
}

impl fmt::Display for StarknetChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mainnet => write!(f, "SN_MAIN"),
            Self::Testnet => write!(f, "SN_GOERLI"),
            Self::Testnet2 => write!(f, "SN_GOERLI2"),
        }
    }
}

impl FromStr for StarknetChain {
    type Err = InvalidStarknetChain;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_uppercase();
        match s.as_str() {
            "SN_MAIN" => Ok(Self::Mainnet),
            "SN_GOERLI" => Ok(Self::Testnet),
            "SN_GOERLI2" => Ok(Self::Testnet2),
            _ => Err(InvalidStarknetChain(s)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid chain id: {0}")]
pub struct InvalidStarknetChain(String);
