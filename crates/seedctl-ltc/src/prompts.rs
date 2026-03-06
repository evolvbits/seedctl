//! Interactive prompts for the Litecoin (LTC) chain crate.
//!
//! Provides user-facing [`dialoguer`] prompts used by [`crate::run`] to
//! collect Litecoin-specific configuration: network selection and an optional
//! Litecoin Core RPC URL for balance queries via `scantxoutset`.

use dialoguer::{Input, Select};
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

/// Supported Litecoin derivation/address styles.
#[derive(Clone, Copy)]
pub enum LtcDerivationStyle {
  /// Native SegWit bech32 `ltc1...` (BIP-84).
  BIP84,
  /// Legacy Base58 `L...` / `m...` (BIP-44).
  BIP44,
}

impl LtcDerivationStyle {
  /// Returns BIP purpose for the selected style.
  pub fn purpose(self) -> u32 {
    match self {
      Self::BIP84 => 84,
      Self::BIP44 => 44,
    }
  }

  /// Returns export script type label.
  pub fn script_type(self) -> &'static str {
    match self {
      Self::BIP84 => "bip84",
      Self::BIP44 => "bip44",
    }
  }

  /// Returns descriptor label used in display/export.
  pub fn descriptor(self) -> &'static str {
    match self {
      Self::BIP84 => "ltc-bip84",
      Self::BIP44 => "ltc-bip44",
    }
  }
}

/// Litecoin network variant — determines the bech32 HRP and SLIP-44 coin type.
#[derive(Clone, Copy)]
pub enum LtcNetwork {
  /// Litecoin Mainnet — addresses start with `ltc1…` (bech32) or `L…` / `M…`
  /// (legacy).
  Mainnet,

  /// Litecoin Testnet — addresses start with `tltc1…` (bech32) or `m…` / `n…`
  /// (legacy).
  Testnet,
}

/// Prompts the user to choose between Litecoin Mainnet and Testnet.
///
/// Returns `(LtcNetwork, coin_type)` where `coin_type` follows SLIP-44:
/// - Mainnet → `(LtcNetwork::Mainnet, 2)`
/// - Testnet → `(LtcNetwork::Testnet, 1)`
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn select_network() -> Result<(LtcNetwork, u32), Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Litecoin network:")
    .items(["Mainnet", "Testnet"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => (LtcNetwork::Mainnet, 2),
    1 => (LtcNetwork::Testnet, 1),
    _ => unreachable!(),
  })
}

/// Prompts the user to choose Litecoin derivation/address style.
pub fn select_derivation_style() -> Result<LtcDerivationStyle, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select Litecoin derivation style:")
    .items([
      "Native SegWit BIP84 (m/84'/2'/0'/0/x, ltc1...)",
      "Legacy BIP44 (m/44'/2'/0'/0/x, L...)",
    ])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => LtcDerivationStyle::BIP84,
    1 => LtcDerivationStyle::BIP44,
    _ => unreachable!(),
  })
}

/// Prompts the user for a Litecoin Core RPC URL used for balance queries via
/// `scantxoutset`.
///
/// Returns an empty string immediately when [`RPC_URL_ENABLE`] is `false`
/// (the default), keeping the tool fully offline-capable out of the box.
///
/// When enabled, the user may press Enter without typing anything to skip
/// the balance check for this session.
///
/// The expected endpoint is a Litecoin Core JSON-RPC node with authentication,
/// e.g.: `http://user:pass@127.0.0.1:9332`
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  if !RPC_URL_ENABLE {
    return Ok(String::new());
  }

  let url: String = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("Litecoin RPC URL (enter to skip balance check)")
    .allow_empty(true)
    .interact_text()?;

  Ok(url)
}
