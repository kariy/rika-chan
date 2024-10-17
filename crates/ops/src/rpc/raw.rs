use color_eyre::Result;
use probe_args::commands::rpc::RawRpcArgs;
use reqwest::IntoUrl;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::utils;

pub fn send(args: RawRpcArgs) -> Result<()> {
    let RawRpcArgs { method, params, url } = args;

    let payload = build_payload(&method, params);
    let res = utils::block_on(send_request::<Value>(url, payload))?;
    println!("{}", colored_json::to_colored_json_auto(&res)?);

    Ok(())
}

fn build_payload(method: &str, params: Vec<Value>) -> Value {
    json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": method,
        "params": Value::Array(params)
    })
}

async fn send_request<T>(url: impl IntoUrl, payload: Value) -> Result<T>
where
    T: DeserializeOwned,
{
    let client = reqwest::Client::new().post(url);
    let res = client.json(&payload).send().await?.json::<T>().await?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::build_payload;

    #[test]
    fn test_build_payload() {
        let method = "starknet_getStorageAt";
        let params = vec![json!(123), json!(69420), json!("latest"), json!({ "key": "value" })];

        let expected = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "starknet_getStorageAt",
            "params": [123, 69420, "latest", { "key": "value" }]
        });

        let result = build_payload(method, params);
        similar_asserts::assert_eq!(result, expected);
    }
}
