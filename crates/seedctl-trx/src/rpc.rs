//! Tron (TRX) balance query client using the Tron full-node HTTP API.
//!
//! Provides a single [`get_balance`] function that POSTs to the
//! `/wallet/getaccount` endpoint and returns the confirmed TRX balance
//! (in TRX, not sun) for a given Base58Check-encoded Tron address.
//!
//! This module is only active when [`seedctl_core::constants::RPC_URL_ENABLE`]
//! is `true`. When disabled, callers receive an empty RPC URL and skip all
//! balance queries, keeping the tool fully offline-capable.

use serde_json::json;

/// Builds the `/wallet/getaccount` endpoint URL from a base node URL.
///
/// Strips any trailing slash from `url` before appending the path so that
/// double-slash URLs are never produced.
fn endpoint(url: &str) -> String {
  format!("{}/wallet/getaccount", url.trim_end_matches('/'))
}

/// Queries a Tron full node for the confirmed TRX balance of `address`.
///
/// Posts a JSON body of the form:
///
/// ```json
/// { "address": "<address>", "visible": true }
/// ```
///
/// to the `/wallet/getaccount` endpoint and reads the `balance` field from
/// the response, which is expressed in **sun** (1 TRX = 1 000 000 sun).
///
/// # Parameters
///
/// - `node_url` — base URL of the Tron full-node HTTP API, e.g.
///   `"https://api.trongrid.io"`. A trailing slash is stripped automatically.
/// - `address`  — Base58Check-encoded Tron address to query
///   (e.g. `"TYour…"`, produced by [`crate::derive::address_from_xprv`]).
///
/// # Returns
///
/// - `Some(balance_trx)` — confirmed balance in TRX as a floating-point value
///   (sun ÷ 1e6).
/// - `None` — if the HTTP request failed, the response could not be parsed,
///   or the `balance` field was absent (which Tron nodes omit for zero-balance
///   accounts).
///
/// # Notes
///
/// Tron nodes return an empty object `{}` for accounts that have never
/// been activated (zero balance, no transactions). In that case `balance`
/// will be absent and this function returns `None`, which the caller
/// renders as `"-"` in the address table.
pub fn get_balance(node_url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();
  let body = json!({
    "address": address,
    // `visible: true` tells the node to accept/return Base58Check addresses
    // instead of hex-encoded addresses.
    "visible": true,
  });

  let response = client.post(endpoint(node_url)).json(&body).send().ok()?;
  let payload: serde_json::Value = response.json().ok()?;

  // `balance` is in sun (integer). Absent for unactivated / zero-balance accounts.
  let sun = payload.get("balance").and_then(|v| v.as_u64()).unwrap_or(0);

  // Convert sun to TRX (1 TRX = 1e6 sun).
  Some(sun as f64 / 1e6)
}
