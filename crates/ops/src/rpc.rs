use eyre::Result;
use reqwest::IntoUrl;
use rika_args::commands::rpc::RpcArgs;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::utils;

pub fn send(args: RpcArgs) -> Result<()> {
    let RpcArgs {
        method,
        params,
        raw,
        url,
    } = args;

    let payload = build_payload(&method, params, raw)?;
    let res = utils::block_on(send_request::<Value>(url, payload))?;
    println!("{}", serde_json::to_string_pretty(&res)?);

    Ok(())
}

fn build_payload(method: &str, params: Vec<String>, raw: bool) -> Result<Value> {
    let mut json_params = Vec::with_capacity(params.len());

    if raw {
        if let Some(p) = params.first() {
            json_params.push(serde_json::from_str(p)?)
        }
    } else {
        for value in params {
            json_params.push(serde_json::from_str(&value)?)
        }
    }

    Ok(json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": method,
        "params": Value::Array(json_params)
    }))
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

    use super::*;
    use serde_json::json;

    #[test]
    fn parse_rpc_params() {
        let method = "foo";
        let params = vec![json!({
            "contract_address": "0x050225ec8d27d8d34c2a5dfd97f01bcd8d55b521fe34ac1db5ba9f544b99af01",
            "entry_point_selector": "0x025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918",
            "calldata": [
                "0x12314",
                "0x42069"
            ]
        }).to_string(),"918".to_string(), "\"latest\"".to_string()];

        // prepare the expected payload
        let expected_payload = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "foo",
            "params": [
                {
                    "contract_address": "0x050225ec8d27d8d34c2a5dfd97f01bcd8d55b521fe34ac1db5ba9f544b99af01",
                    "entry_point_selector": "0x025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918",
                    "calldata": [
                        "0x12314",
                        "0x42069"
                    ]
                },
                918,
                "latest"
            ]
        });

        // test as non-raw payload
        let payload = build_payload(method, params.clone(), false).unwrap();
        similar_asserts::assert_eq!(payload, expected_payload);

        // prepare the expected payload, only the first param is parsed
        let expected_payload = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "foo",
            "params": [
                {
                    "contract_address": "0x050225ec8d27d8d34c2a5dfd97f01bcd8d55b521fe34ac1db5ba9f544b99af01",
                    "entry_point_selector": "0x025ec026985a3bf9d0cc1fe17326b245dfdc3ff89b8fde106542a3ea56c5a918",
                    "calldata": [
                        "0x12314",
                        "0x42069"
                    ]
                },
            ]
        });

        // test as raw payload
        let payload = build_payload(method, params, true).unwrap();
        similar_asserts::assert_eq!(payload, expected_payload);
    }
}
