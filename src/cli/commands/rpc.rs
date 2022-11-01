use crate::cli::commands::opts::StarkNetOptions;

use std::fs;
use std::path::Path;

use clap::{ArgGroup, Parser};
use eyre::Result;
use serde_json::json;

#[derive(Debug, Clone, Parser)]
#[clap(group(ArgGroup::new("params-src").required(true).args(&["params", "file"])))]
pub struct RpcArgs {
    #[clap(help = "RPC method name")]
    method: String,

    #[clap(long)]
    #[clap(group = "params-src")]
    #[clap(help = "RPC parameters")]
    params: Option<Vec<String>>,

    #[clap(long)]
    #[clap(group = "params-src")]
    #[clap(help = "Get RPC parameters from a file")]
    file: Option<String>,

    #[clap(flatten)]
    starknet: StarkNetOptions,
}

impl RpcArgs {
    pub async fn run(self) -> Result<String> {
        let Self {
            method,
            params,
            file,
            starknet,
        } = self;

        let params = if let Some(path) = file {
            let content = fs::read_to_string(Path::new(&path))?;
            serde_json::from_str(&content)?
        } else {
            let params = params.unwrap();
            serde_json::Value::Array(
                params
                    .into_iter()
                    .map(|value| {
                        serde_json::from_str(&value)
                            .unwrap_or(serde_json::Value::String(value.to_owned()))
                    })
                    .collect(),
            )
        };

        let res = reqwest::Client::new()
            .post(starknet.rpc_url)
            .json(&json!({
                "id": 1,
                "jsonrpc": "2.0",
                "method": method,
                "params": params
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(serde_json::to_string_pretty(&res)?)
    }
}
