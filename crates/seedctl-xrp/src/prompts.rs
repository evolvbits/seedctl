//! Interactive prompts for the XRP Ledger (XRP) chain crate.
//!
//! Provides user-facing [`dialoguer`] prompts used by [`crate::run`] to
//! collect XRP-specific configuration: network selection, address count,
//! and an optional XRPL RPC URL for balance queries.

use dialoguer::{Input, Select};
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

/// XRP Ledger network variant — determines the RPC endpoint and export label.
#[derive(Clone, Copy)]
pub enum XrpNetwork {
  /// XRPL Mainnet — live network with real XRP funds.
  Mainnet,
  /// XRPL Testnet — test network with valueless XRP tokens.
  Testnet,
}

impl XrpNetwork {
  /// Returns the network identifier string written into the watch-only export
  /// JSON.
  ///
  /// - Mainnet → `"xrpl-mainnet"`
  /// - Testnet → `"xrpl-testnet"`
  pub fn export_network(self) -> &'static str {
    match self {
      Self::Mainnet => "xrpl-mainnet",
      Self::Testnet => "xrpl-testnet",
    }
  }
}

/// Prompts the user to choose between XRPL Mainnet and Testnet.
///
/// Returns the selected [`XrpNetwork`] variant.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn select_network() -> Result<XrpNetwork, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select XRP network:")
    .items(["Mainnet", "Testnet"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => XrpNetwork::Mainnet,
    1 => XrpNetwork::Testnet,
    _ => unreachable!(),
  })
}

/// Prompts the user for the number of XRP Ledger addresses to generate.
///
/// Defaults to `10` if the user presses Enter without typing a value.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  let n: u32 = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("How many XRP addresses to generate?")
    .default(10)
    .interact_text()?;

  Ok(n)
}

/// Prompts the user for an XRPL JSON-RPC URL used for balance queries.
///
/// Returns an empty string immediately when [`RPC_URL_ENABLE`] is `false`
/// (the default), keeping the tool fully offline-capable out of the box.
///
/// When enabled, the user may press Enter without typing anything to skip
/// the balance check for this session.
///
/// The expected endpoint is a standard XRPL JSON-RPC node, e.g.:
/// `https://s1.ripple.com:51234/`
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  if !RPC_URL_ENABLE {
    return Ok(String::new());
  }

  let s: String = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("XRPL RPC URL (enter to skip balance check)")
    .allow_empty(true)
    .interact_text()?;
  Ok(s)
}
