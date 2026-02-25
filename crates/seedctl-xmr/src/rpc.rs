use serde_json::json;

pub fn get_balance(wallet_rpc_url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();

  let idx_body = json!({
    "jsonrpc": "2.0",
    "id": "seedctl",
    "method": "get_address_index",
    "params": { "address": address },
  });

  let idx_resp = client.post(wallet_rpc_url).json(&idx_body).send().ok()?;
  let idx_payload: serde_json::Value = idx_resp.json().ok()?;
  let index = idx_payload.get("result")?.get("index")?;
  let major = index.get("major")?.as_u64()?;
  let minor = index.get("minor")?.as_u64()?;

  let bal_body = json!({
    "jsonrpc": "2.0",
    "id": "seedctl",
    "method": "get_balance",
    "params": {
      "account_index": major,
      "address_indices": [minor],
    },
  });

  let bal_resp = client.post(wallet_rpc_url).json(&bal_body).send().ok()?;
  let bal_payload: serde_json::Value = bal_resp.json().ok()?;
  let atomic = bal_payload
    .get("result")?
    .get("per_subaddress")?
    .as_array()?
    .first()?
    .get("balance")?
    .as_u64()?;

  Some(atomic as f64 / 1e12)
}
