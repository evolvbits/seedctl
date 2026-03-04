//! Solana JSON-RPC balance query client.
//!
//! Provides a single [`get_balance`] function that queries a Solana RPC node
//! using the `getBalance` JSON-RPC method and returns the confirmed SOL
//! balance for a given base58 wallet address.
//!
//! This module is only active when [`seedctl_core::constants::RPC_URL_ENABLE`]
//! is `true`. When disabled, callers receive an empty RPC URL and skip all
//! balance queries, keeping the tool fully offline-capable.

use serde_json::json;

/// Queries a Solana RPC node for the confirmed SOL balance of `address`.
///
/// Uses the `getBalance` JSON-RPC method, which returns the balance in
/// **lamports** (1 SOL = 1 000 000 000 lamports = 1e9 lamports).
///
/// # Parameters
///
/// - `rpc_url` — full URL to the Solana JSON-RPC endpoint, e.g.
///   `"https://api.mainnet-beta.solana.com"`.
/// - `address` — base58-encoded Solana wallet address (32-byte Ed25519
///   verifying key), e.g. the value produced by [`crate::utils::pubkey_to_address`].
///
/// # Returns
///
/// - `Some(balance_sol)` — confirmed balance in SOL as a floating-point value
///   (lamports ÷ 1e9).
/// - `None` — if the HTTP request failed, the response could not be parsed,
///   or the `result.value` field was absent.
///
/// # Notes
///
/// The public Solana mainnet RPC endpoint is rate-limited. For production or
/// high-frequency use, consider a dedicated provider (Helius, QuickNode,
/// Triton, etc.) or a self-hosted validator node.
pub fn get_balance(rpc_url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();
  let body = json!({
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getBalance",
    "params": [address],
  });

  let response = client.post(rpc_url).json(&body).send().ok()?;
  let payload: serde_json::Value = response.json().ok()?;

  // `result.value` contains the balance in lamports as a u64.
  let lamports = payload.get("result")?.get("value")?.as_u64()?;

  // Convert lamports to SOL (1 SOL = 1e9 lamports).
  Some(lamports as f64 / 1e9)
}
