//! Monero wallet-RPC balance query client.
//!
//! Provides a single [`get_balance`] function that queries a running
//! `monero-wallet-rpc` daemon using two JSON-RPC calls:
//!
//! 1. `get_address_index` — resolves the address to its `(major, minor)`
//!    account/subaddress index pair.
//! 2. `get_balance`       — retrieves the balance for that specific
//!    subaddress index within the account.
//!
//! This module is only active when [`seedctl_core::constants::RPC_URL_ENABLE`]
//! is `true`. When disabled, callers receive an empty RPC URL and skip all
//! balance queries, keeping the tool fully offline-capable.
//!
//! # Prerequisites
//!
//! A `monero-wallet-rpc` instance must be running and have the wallet already
//! opened. The standard local endpoint is `http://127.0.0.1:18088/json_rpc`.

use serde_json::json;

/// Queries a Monero `wallet-rpc` daemon for the confirmed XMR balance of
/// `address`.
///
/// The query is performed in two steps because the Monero RPC API does not
/// expose a single call that accepts an address string and returns its balance
/// directly:
///
/// 1. **`get_address_index`** — resolves `address` to its `(major, minor)`
///    index pair within the currently open wallet.
/// 2. **`get_balance`** — fetches the balance for `account_index = major`
///    filtered to `address_indices = [minor]`, returning the atomic unit
///    (piconero) value for that specific subaddress.
///
/// The balance is reported in **atomic units** (piconeros) by the Monero RPC
/// (1 XMR = 1 000 000 000 000 piconeros = 1e12 piconeros).
///
/// # Parameters
///
/// - `wallet_rpc_url` — full URL to the `monero-wallet-rpc` JSON-RPC endpoint,
///   e.g. `"http://127.0.0.1:18088/json_rpc"`.
/// - `address`        — Base58-encoded Monero address to query (standard or
///   subaddress), produced by [`crate::derive::derive_address`].
///
/// # Returns
///
/// - `Some(balance_xmr)` — confirmed balance in XMR as a floating-point value
///   (atomic units ÷ 1e12).
/// - `None` — if either HTTP request fails, a response cannot be parsed,
///   the address is not found in the open wallet, or any expected JSON field
///   is missing.
///
/// # Notes
///
/// - The address must belong to the wallet currently open in `wallet-rpc`.
///   If the wallet does not contain the address, `get_address_index` will
///   return an error and this function will return `None`.
/// - Only **confirmed** (unlocked) balance is returned. Pending / locked funds
///   are not included in the `balance` field of `per_subaddress`.
/// - For air-gapped operation, leave [`seedctl_core::constants::RPC_URL_ENABLE`]
///   set to `false` (the default); this function is never called in that mode.
pub fn get_balance(wallet_rpc_url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();

  // ── Step 1: resolve the address to its (major, minor) index ──────────────
  let idx_body = json!({
    "jsonrpc": "2.0",
    "id": "seedctl",
    "method": "get_address_index",
    "params": {
      // The address whose account/subaddress index we want to look up.
      "address": address,
    },
  });

  let idx_resp = client.post(wallet_rpc_url).json(&idx_body).send().ok()?;
  let idx_payload: serde_json::Value = idx_resp.json().ok()?;

  // Navigate to `result.index.{major, minor}`.
  let index = idx_payload.get("result")?.get("index")?;
  let major = index.get("major")?.as_u64()?;
  let minor = index.get("minor")?.as_u64()?;

  // ── Step 2: fetch the balance for this specific subaddress ────────────────
  let bal_body = json!({
    "jsonrpc": "2.0",
    "id": "seedctl",
    "method": "get_balance",
    "params": {
      // Account index (major) within the wallet.
      "account_index": major,
      // Filter to only the specific subaddress (minor) we care about.
      "address_indices": [minor],
    },
  });

  let bal_resp = client.post(wallet_rpc_url).json(&bal_body).send().ok()?;
  let bal_payload: serde_json::Value = bal_resp.json().ok()?;

  // Navigate to `result.per_subaddress[0].balance` (atomic units / piconeros).
  let atomic = bal_payload
    .get("result")?
    .get("per_subaddress")?
    .as_array()?
    .first()?
    .get("balance")?
    .as_u64()?;

  // Convert piconeros to XMR (1 XMR = 1e12 piconeros).
  Some(atomic as f64 / 1e12)
}
