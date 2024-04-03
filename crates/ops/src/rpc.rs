use eyre::Result;
use reqwest::IntoUrl;
use rika_args::commands::rpc::RpcArgs;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::utils;

pub fn rpc(args: RpcArgs) -> Result<()> {
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

fn build_payload(method: &str, params: Vec<String>, raw_params: bool) -> Result<Value> {
    let mut json_params = Vec::with_capacity(params.len());
    if raw_params {
        if let Some(p) = params.first() {
            json_params.push(serde_json::from_str(p)?)
        }
    } else {
        for value in params {
            let value = serde_json::from_str(&value).unwrap_or(Value::String(value));
            json_params.push(value)
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
