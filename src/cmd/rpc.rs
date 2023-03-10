use clap::Args;
use eyre::Result;
use reqwest::Url;
use serde_json::json;

#[derive(Debug, Clone, Args)]
pub struct RpcArgs {
    #[arg(help = "RPC method name")]
    method: String,

    #[arg(short, long)]
    #[arg(help = r#"Pass the "params" as is"#)]
    #[arg(long_help = r#"Pass the "params" as is
If --raw is passed the first PARAM will be taken as the value of "params". If no params are given, stdin will be used. For example:
rpc starknet_getStorageAt '["0x123", "0x69420", "latest"]' --raw
    => {"method": "eth_getBlockByNumber", "params": ["0x123", false] ... }"#)]
    raw: bool,

    #[arg(value_name = "PARAMS")]
    #[arg(help = "RPC parameters")]
    #[arg(long_help = r#"RPC parameters

    Parameters are interpreted as JSON and then fall back to string. For example:

    rpc starknet_getStorageAt 0x123 0x69420 latest
    => {"method": "starknet_getStorageAt", "params": ["0x123", "0x69420", "latest"] ... }"#)]
    params: Vec<String>,

    #[arg(long)]
    #[arg(value_name = "URL")]
    #[arg(help = "The RPC endpoint")]
    #[arg(env = "STARKNET_RPC_URL")]
    #[arg(default_value = "http://localhost:5050/rpc")]
    rpc_url: Url,
}

impl RpcArgs {
    pub async fn run(self) -> Result<String> {
        let Self {
            method,
            raw,
            params,
            rpc_url,
        } = self;

        let mut vec = Vec::new();
        if raw {
            if let Some(p) = params.get(0) {
                vec.push(serde_json::from_str(p)?)
            }
        } else {
            for value in params.into_iter() {
                vec.push(serde_json::from_str(&value).unwrap_or(serde_json::Value::String(value)))
            }
        }

        let params = serde_json::Value::Array(vec);
        let res = reqwest::Client::new()
            .post(rpc_url)
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

#[cfg(test)]
mod tests {
    use crate::cmd::probe::{App, Commands};

    use clap::Parser;
    use serde_json::json;

    #[tokio::test]
    async fn parse_rpc_params() {
        let p = json!({
            "contract_address": "0x050225ec8d27d8d34c2a5dfd97f01bcd8d55b521fe34ac1db5ba9f544b99af01",
            "entry_point_selector": "0x025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918",
            "calldata": [
                "0x12314",
                "0x42069"
            ]
        })
        .to_string();

        let args: App = App::parse_from(["probe", "rpc", "starknet_call", &p, "latest"]);

        match args.command {
            Commands::Rpc(args) => {
                let params = args.params;
                assert_eq!(params, vec![p, "latest".to_string()])
            }
            _ => {
                unreachable!()
            }
        };
    }
}
