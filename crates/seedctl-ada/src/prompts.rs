//! Interactive prompts for the Cardano (ADA) chain crate.
//!
//! Provides user-facing [`dialoguer`] prompts used by [`crate::run`] to
//! collect Cardano-specific configuration: network selection, address count,
//! and an optional Koios REST API URL for balance queries.

use dialoguer::{Input, Select};
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

/// Cardano network variant — determines the bech32 HRP and address header byte.
#[derive(Clone, Copy)]
pub enum AdaNetwork {
  /// Cardano Mainnet — addresses start with `addr1…`.
  Mainnet,
  /// Cardano Testnet (Preview / Preprod) — addresses start with `addr_test1…`.
  Testnet,
}

impl AdaNetwork {
  /// Returns the bech32 human-readable part (HRP) for this network.
  ///
  /// - Mainnet → `"addr"`
  /// - Testnet → `"addr_test"`
  pub fn hrp(self) -> &'static str {
    match self {
      Self::Mainnet => "addr",
      Self::Testnet => "addr_test",
    }
  }

  /// Returns the 4-bit network ID encoded in the low nibble of the address
  /// header byte.
  ///
  /// - Mainnet → `1`
  /// - Testnet → `0`
  pub fn network_id(self) -> u8 {
    match self {
      Self::Mainnet => 1,
      Self::Testnet => 0,
    }
  }

  /// Returns the Shelley base-address header byte (`0b0000_NNNN` where
  /// `NNNN` is the network ID).
  pub fn base_header(self) -> u8 {
    // Shelley base address type: upper nibble 0x00, lower nibble = network_id.
    self.network_id()
  }

  /// Returns the network identifier string written into the watch-only export
  /// JSON.
  ///
  /// - Mainnet → `"cardano-mainnet"`
  /// - Testnet → `"cardano-testnet"`
  pub fn export_network(self) -> &'static str {
    match self {
      Self::Mainnet => "cardano-mainnet",
      Self::Testnet => "cardano-testnet",
    }
  }
}

/// Prompts the user to choose between generating addresses and scanning common
/// Cardano derivation paths.
pub fn select_derivation_mode() -> Result<usize, Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let choice = Select::with_theme(&theme)
    .with_prompt("Select derivation mode (Cardano):")
    .items(["Generate addresses", "Scan common derivation paths"])
    .default(0)
    .interact()?;

  Ok(choice)
}

/// Prompts the user to choose between Cardano Mainnet and Testnet.
///
/// Returns the selected [`AdaNetwork`] variant.
pub fn select_network() -> Result<AdaNetwork, Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let choice = Select::with_theme(&theme)
    .with_prompt("Select Cardano network:")
    .items(["Mainnet (addr...)", "Testnet (addr_test...)"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => AdaNetwork::Mainnet,
    1 => AdaNetwork::Testnet,
    _ => unreachable!(),
  })
}

/// Prompts the user for the number of Cardano Shelley base addresses to derive.
///
/// Defaults to `10` if the user presses Enter without typing a value.
pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let count: u32 = Input::with_theme(&theme)
    .with_prompt("How many Cardano addresses to generate?")
    .default(10)
    .interact_text()?;

  Ok(count)
}

/// Prompts the user for a Koios REST API URL used to query address balances.
///
/// Returns an empty string immediately when [`RPC_URL_ENABLE`] is `false`
/// (the default), keeping the tool fully offline-capable out of the box.
///
/// When enabled, the user may press Enter without typing anything to skip
/// the balance check for this session.
///
/// The expected endpoint is the Koios `/address_info` route, e.g.:
/// `https://api.koios.rest/api/v1/address_info`
pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  if !RPC_URL_ENABLE {
    return Ok(String::new());
  }

  let theme = dialoguer_theme("►");
  let s: String = Input::with_theme(&theme)
    .with_prompt("Cardano address API URL (Koios /address_info, enter to skip)")
    .allow_empty(true)
    .interact_text()?;

  Ok(s)
}
