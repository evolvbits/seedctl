use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct EthRpcResponse {
  result: Option<String>,
}

/// Obtém saldo em MATIC (convertido de wei) via RPC compatível com Ethereum.
pub fn get_balance(rpc_url: &str, address: &str) -> Option<f64> {
  // Formato JSON-RPC padrão do Ethereum.
  let body = serde_json::json!({
    "jsonrpc": "2.0",
    "method": "eth_getBalance",
    "params": [address, "latest"],
    "id": 1,
  });

  match do_post(rpc_url, &body) {
    Ok(resp) => {
      if let Some(result) = resp.result {
        if let Ok(wei) = u128::from_str_radix(result.trim_start_matches("0x"), 16) {
          // 1 MATIC = 10^18 wei (mesma escala do ETH).
          let matic = wei as f64 / 1e18;
          return Some(matic);
        }
      }
      None
    }
    Err(_) => None,
  }
}

fn do_post(url: &str, body: &serde_json::Value) -> Result<EthRpcResponse> {
  let client = reqwest::blocking::Client::new();
  let resp = client.post(url).json(body).send()?;
  let parsed: EthRpcResponse = resp.json()?;
  Ok(parsed)
}
