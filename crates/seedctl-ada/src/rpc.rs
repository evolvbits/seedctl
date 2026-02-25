use serde_json::json;

pub fn get_balance(api_url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();
  let body = json!({
    "_addresses": [address],
  });

  let response = client.post(api_url).json(&body).send().ok()?;
  let payload: serde_json::Value = response.json().ok()?;

  let first = payload.as_array()?.first()?;
  let lovelace = first.get("balance")?.as_str()?.parse::<f64>().ok()?;
  Some(lovelace / 1e6)
}
