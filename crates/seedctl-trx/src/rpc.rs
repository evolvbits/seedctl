use serde_json::json;

fn endpoint(url: &str) -> String {
  format!("{}/wallet/getaccount", url.trim_end_matches('/'))
}

pub fn get_balance(node_url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();
  let body = json!({
    "address": address,
    "visible": true,
  });

  let response = client.post(endpoint(node_url)).json(&body).send().ok()?;
  let payload: serde_json::Value = response.json().ok()?;

  let sun = payload.get("balance").and_then(|v| v.as_u64()).unwrap_or(0);
  Some(sun as f64 / 1e6)
}
