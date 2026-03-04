//! XRP Ledger JSON-RPC balance query client.
//!
//! Provides a single [`get_balance`] function that queries an XRPL JSON-RPC
//! node using the `account_info` method and returns the confirmed XRP balance
//! for a given classic address.
//!
//! This module is only active when [`seedctl_core::constants::RPC_URL_ENABLE`]
//! is `true`. When disabled, callers receive an empty RPC URL and skip all
//! balance queries, keeping the tool fully offline-capable.

use serde_json::json;

/// Queries an XRPL JSON-RPC node for the confirmed XRP balance of `address`.
///
/// Sends the `account_info` method with `ledger_index: "validated"` so that
/// only finalized ledger data is returned, avoiding unconfirmed transactions.
///
/// The balance is reported in **drops** by the XRPL protocol
/// (1 XRP = 1 000 000 drops = 1e6 drops).
///
/// # Parameters
///
/// - `rpc_url` — full URL to the XRPL JSON-RPC endpoint, e.g.
///   `"https://s1.ripple.com:51234/"`.
/// - `address` — Base58-encoded XRPL classic address to query
///   (e.g. `"rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh"`), produced by
///   [`crate::derive::address_from_xprv`].
///
/// # Returns
///
/// - `Some(balance_xrp)` — confirmed balance in XRP as a floating-point value
///   (drops ÷ 1e6).
/// - `None` — if the HTTP request failed, the response could not be parsed,
///   or the account was not found on the ledger (never activated / no XRP).
///
/// # Notes
///
/// XRPL accounts must hold a minimum reserve (currently 10 XRP) to be
/// activated. Newly derived addresses that have never received XRP will not
/// appear in `account_info` and this function will return `None` for them.
pub fn get_balance(rpc_url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();

  let body = json!({
    "method": "account_info",
    "params": [{
      "account": address,
      // Request data from the most recently validated (finalised) ledger only.
      "ledger_index": "validated",
      // `strict: true` enforces that `account` is a valid classic address.
      "strict": true,
    }]
  });

  let response = client.post(rpc_url).json(&body).send().ok()?;
  let payload: serde_json::Value = response.json().ok()?;

  // Navigate to `result.account_data.Balance` (a decimal string in drops).
  let balance_drops = payload
    .get("result")?
    .get("account_data")?
    .get("Balance")?
    .as_str()?;

  // Convert drops (integer string) to XRP (1 XRP = 1e6 drops).
  let drops = balance_drops.parse::<f64>().ok()?;
  Some(drops / 1e6)
}
