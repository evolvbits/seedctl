//! Cardano balance query client using the Koios REST API.
//!
//! Provides a single [`get_balance`] function that POSTs to the Koios
//! `/address_info` endpoint and returns the confirmed ADA balance for a
//! given Shelley bech32 address.
//!
//! This module is only active when [`seedctl_core::constants::RPC_URL_ENABLE`]
//! is `true`. When disabled, callers receive an empty RPC URL and skip all
//! balance queries, keeping the tool fully offline-capable.

use serde_json::json;

/// Queries the Koios REST API for the confirmed ADA balance of `address`.
///
/// Sends a POST request to `api_url` with a JSON body of the form:
///
/// ```json
/// { "_addresses": ["<address>"] }
/// ```
///
/// and extracts the `balance` field (in lovelace) from the first element of
/// the response array, converting it to ADA (1 ADA = 1 000 000 lovelace).
///
/// # Parameters
///
/// - `api_url` — full URL to the Koios `/address_info` endpoint, e.g.
///   `"https://api.koios.rest/api/v1/address_info"`.
/// - `address` — bech32-encoded Shelley base address to look up
///   (e.g. `"addr1q…"`).
///
/// # Returns
///
/// - `Some(balance_ada)` — confirmed balance in ADA as a floating-point value.
/// - `None` — if the HTTP request failed, the response could not be parsed,
///   or the address was not found in the response array.
///
/// # Notes
///
/// Koios is a decentralised, community-driven Cardano query layer. For
/// production use, consider running your own Koios instance or using a
/// dedicated indexer such as db-sync.
pub fn get_balance(api_url: &str, address: &str) -> Option<f64> {
  let client = reqwest::blocking::Client::new();
  let body = json!({
    "_addresses": [address],
  });

  let response = client.post(api_url).json(&body).send().ok()?;
  let payload: serde_json::Value = response.json().ok()?;

  // Koios returns an array; take the first element for the queried address.
  let first = payload.as_array()?.first()?;

  // `balance` is returned as a decimal string in lovelace (1 ADA = 1e6 lovelace).
  let lovelace = first.get("balance")?.as_str()?.parse::<f64>().ok()?;
  Some(lovelace / 1e6)
}
