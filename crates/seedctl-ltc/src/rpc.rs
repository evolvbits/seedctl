//! Litecoin RPC client for balance queries via Litecoin Core's `scantxoutset`.
//!
//! Provides a single [`get_balance`] function that queries a Litecoin Core node
//! using the `scantxoutset` JSON-RPC method to retrieve the confirmed UTXO
//! balance for a given address.
//!
//! This module is only active when [`seedctl_core::constants::RPC_URL_ENABLE`]
//! is `true`. When disabled, callers receive an empty RPC URL and skip all
//! balance queries, keeping the tool fully offline-capable.

use serde_json::json;

/// Queries a Litecoin Core node for the confirmed UTXO balance of `address`.
///
/// Uses the `scantxoutset` JSON-RPC method, which scans the UTXO set for all
/// outputs matching the given address descriptor (`addr(<address>)`).
///
/// # Parameters
///
/// - `rpc_url` — full URL to the Litecoin Core RPC endpoint, including
///   credentials if required (e.g. `"http://user:pass@127.0.0.1:9332"`).
/// - `address` — encoded Litecoin address to look up (bech32 `ltc1…`,
///   P2SH `M…`, or legacy Base58Check `L…`).
///
/// # Returns
///
/// - `Some(balance)` — total confirmed balance in LTC as a floating-point value.
/// - `None` — if the HTTP request failed, the response could not be parsed,
///   or the RPC node returned a non-null error field.
///
/// # Notes
///
/// `scantxoutset` performs a full UTXO scan and can be slow on nodes without
/// a UTXO index. For production use, consider a dedicated indexer (e.g.
/// Electrum-LTC or an Esplora-compatible backend) instead.
pub fn get_balance(rpc_url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();

  // Build the `addr()` descriptor for the target address.
  let scan_arg = format!("addr({address})");

  let body = json!({
    "jsonrpc": "1.0",
    "id": "seedctl",
    "method": "scantxoutset",
    // "start" initiates a new scan; the second element is the descriptor list.
    "params": ["start", [scan_arg]],
  });

  let response = client.post(rpc_url).json(&body).send().ok()?;
  let payload: serde_json::Value = response.json().ok()?;

  // Abort if the RPC node reported an application-level error.
  if !payload.get("error").is_none_or(|v| v.is_null()) {
    return None;
  }

  // `result.total_amount` is the confirmed UTXO balance in LTC.
  // Try to parse it as an f64 directly; fall back to string parsing in case
  // the node serialises the amount as a decimal string.
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
