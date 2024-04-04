use clap::Parser;
use reqwest::Url;
use serde_json::Value;

#[derive(Debug, Clone, Parser)]
pub struct RpcArgs {
    #[arg(help = "RPC method name")]
    pub method: String,

    #[arg(short, long)]
    #[arg(help = r#"Pass the "params" as is"#)]
    #[arg(long_help = r#"Pass the "params" as is
If --raw is passed the first PARAM will be taken as the value of "params". If no params are given, stdin will be used. For example:
rpc starknet_getStorageAt '[123, "0x69420", "latest"]' --raw
    => {"method": "eth_getBlockByNumber", "params": [123, "0x69420", false] ... }"#)]
    pub raw: bool,

    #[arg(value_name = "PARAMS")]
    #[arg(help = "RPC parameters")]
    #[arg(value_parser = params_value_parser)]
    #[arg(long_help = r#"RPC parameters

    Parameters are interpreted as JSON and then fall back to string. For example:

    rpc starknet_getStorageAt 123 0x69420 latest
    => {"method": "starknet_getStorageAt", "params": [123, "0x69420", "latest"] ... }"#)]
    pub params: Vec<Value>,

    #[arg(long)]
    #[arg(value_name = "URL")]
    #[arg(help = "The RPC endpoint")]
    pub url: Url,
}

fn params_value_parser(value: &str) -> Result<Value, serde_json::Error> {
    // parse as number if possible
    if let Ok(num) = value.parse::<i64>() {
        Ok(Value::Number(serde_json::Number::from(num)))
    } else {
        // otherwise parse normally
        serde_json::from_str(value)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn parse_params() {
        let json = serde_json::json!({
            "key": "value"
        });

        let args = RpcArgs::parse_from(&[
            "rpc",
            "starknet_getStorageAt",
            "123",
            "69420",
            "\"latest\"",
            &json.to_string(),
            "--url",
            "http://localhost:8545",
        ]);

        let expected_params = [json!(123), json!(69420), json!("latest"), json];
        similar_asserts::assert_eq!(args.params, expected_params);
    }
}
