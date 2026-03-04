//! Interactive prompts for the Bitcoin chain crate.
//!
//! Provides user-facing [`dialoguer`] prompts used by [`crate::run`] to
//! collect Bitcoin-specific configuration: network selection, BIP purpose /
//! address type, and an optional RPC URL for balance queries.

use bitcoin::Network;
use dialoguer::{Input, Select};
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

/// Prompts the user to choose between Bitcoin Mainnet and Testnet.
///
/// Returns `(Network, coin_type)` where `coin_type` follows SLIP-44:
/// - Mainnet → `(Network::Bitcoin, 0)`
/// - Testnet → `(Network::Testnet, 1)`
pub fn select_network() -> Result<(Network, u32), Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let choice = Select::with_theme(&theme)
    .with_prompt("Bitcoin network:")
    .items(["Mainnet", "Testnet (test address)"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => (Network::Bitcoin, 0),
    1 => (Network::Testnet, 1),
    _ => unreachable!(),
  })
}

/// Prompts the user to choose a BIP derivation purpose and address type.
///
/// Returns `(purpose, address_type_index)`:
/// - BIP-84 → `(84, 0)` — Native SegWit (bech32, recommended)
/// - BIP-49 → `(49, 1)` — Nested SegWit (P2SH-P2WPKH)
/// - BIP-44 → `(44, 2)` — Legacy (P2PKH)
pub fn select_address_type() -> Result<(u32, usize), Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let address_type = Select::with_theme(&theme)
    .with_prompt("Bitcoin derivation path (BIP purpose):")
    .items([
      "BIP84 (Native SegWit, recommended)",
      "BIP49 (Nested SegWit)",
      "BIP44 (Legacy)",
    ])
    .default(0)
    .interact()?;

  let purpose = match address_type {
    0 => 84,
    1 => 49,
    2 => 44,
    _ => unreachable!(),
  };

  Ok((purpose, address_type))
}

/// Prompts the user for a Bitcoin Core RPC URL used for balance queries.
///
/// Returns an empty string immediately when [`RPC_URL_ENABLE`] is `false`
/// (the default), keeping the tool fully offline-capable out of the box.
///
/// When enabled, the user may press Enter without typing anything to skip
/// the balance check for this session.
pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  if !RPC_URL_ENABLE {
    return Ok(String::new());
  }

  let theme = dialoguer_theme("►");
  let url: String = Input::with_theme(&theme)
    .with_prompt("Bitcoin RPC URL (enter to skip balance check)")
    .allow_empty(true)
    .interact_text()?;

  Ok(url)
}
