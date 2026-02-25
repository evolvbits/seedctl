use serde_json::json;

pub fn get_balance(rpc_url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();
  let scan_arg = format!("addr({address})");

  let body = json!({
    "jsonrpc": "1.0",
    "id": "seedctl",
    "method": "scantxoutset",
    "params": ["start", [scan_arg]],
  });

  let response = client.post(rpc_url).json(&body).send().ok()?;
  let payload: serde_json::Value = response.json().ok()?;

  if !payload.get("error").is_none_or(|v| v.is_null()) {
    return None;
  }

  payload
    .get("result")?
    .get("total_amount")?
    .as_f64()
    .or_else(|| {
      payload
        .get("result")?
        .get("total_amount")?
        .as_str()?
        .parse::<f64>()
        .ok()
    })
}
