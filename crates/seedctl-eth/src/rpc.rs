use serde_json::json;

pub fn get_balance(url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();

  let body = json!({
    "jsonrpc":"2.0",
    "id":1,
    "method":"eth_getBalance",
    "params":[address,"latest"]
  });

  let res = client.post(url).json(&body).send().ok()?;
  let v: serde_json::Value = res.json().ok()?;
  let hex = v.get("result")?.as_str()?;

  let wei = u128::from_str_radix(hex.trim_start_matches("0x"), 16).ok()?;
  Some(wei as f64 / 1e18)
}
