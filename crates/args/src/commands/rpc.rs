use clap::Parser;
use reqwest::Url;

#[derive(Debug, Clone, Parser)]
pub struct RpcArgs {
    #[arg(help = "RPC method name")]
    pub method: String,

    #[arg(short, long)]
    #[arg(help = r#"Pass the "params" as is"#)]
    #[arg(long_help = r#"Pass the "params" as is
If --raw is passed the first PARAM will be taken as the value of "params". If no params are given, stdin will be used. For example:
rpc starknet_getStorageAt '["0x123", "0x69420", "latest"]' --raw
    => {"method": "eth_getBlockByNumber", "params": ["0x123", false] ... }"#)]
    pub raw: bool,

    #[arg(value_name = "PARAMS")]
    #[arg(help = "RPC parameters")]
    #[arg(long_help = r#"RPC parameters

    Parameters are interpreted as JSON and then fall back to string. For example:

    rpc starknet_getStorageAt 0x123 0x69420 latest
    => {"method": "starknet_getStorageAt", "params": ["0x123", "0x69420", "latest"] ... }"#)]
    pub params: Vec<String>,

    #[arg(long)]
    #[arg(value_name = "URL")]
    #[arg(help = "The RPC endpoint")]
    pub url: Url,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_params() {
        let json = serde_json::json!({
            "key": "value"
        })
        .to_string();

        let args = RpcArgs::parse_from(&[
            "rpc",
            "starknet_getStorageAt",
            "0x123",
            "0x69420",
            "\"latest\"",
            &json,
            "--url",
            "http://localhost:8545",
        ]);

        let actual_params = args
            .params
            .as_slice()
            .into_iter()
            .map(String::as_str)
            .collect::<Vec<&str>>();

        let expected_params = ["0x123", "0x69420", "\"latest\"", &json];
        similar_asserts::assert_eq!(actual_params.as_slice(), expected_params);
    }
}
