use serde_json::json;

pub fn get_balance(rpc_url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();
  let body = json!({
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getBalance",
    "params": [address],
  });

  let response = client.post(rpc_url).json(&body).send().ok()?;
  let payload: serde_json::Value = response.json().ok()?;

  let lamports = payload.get("result")?.get("value")?.as_u64()?;
  Some(lamports as f64 / 1e9)
}
