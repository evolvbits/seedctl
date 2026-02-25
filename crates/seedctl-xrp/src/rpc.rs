use serde_json::json;

pub fn get_balance(rpc_url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();
  let body = json!({
    "method": "account_info",
    "params": [{
      "account": address,
      "ledger_index": "validated",
      "strict": true,
    }]
  });

  let response = client.post(rpc_url).json(&body).send().ok()?;
  let payload: serde_json::Value = response.json().ok()?;

  let balance_drops = payload
    .get("result")?
    .get("account_data")?
    .get("Balance")?
    .as_str()?;

  let drops = balance_drops.parse::<f64>().ok()?;
  Some(drops / 1e6)
}
