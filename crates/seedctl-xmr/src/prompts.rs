//! Interactive prompts for the Monero (XMR) chain crate.
//!
//! Provides user-facing [`dialoguer`] prompts used by [`crate::run`] to
//! collect Monero-specific configuration: network selection, address count,
//! and an optional Monero wallet-RPC URL for balance queries.

use dialoguer::{Input, Select};
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

/// Monero key-derivation mode used from an imported BIP-39 mnemonic.
#[derive(Clone, Copy)]
pub enum XmrDerivationMode {
  /// Native Monero-style mapping: `spend = Hs(bip39_seed_64)`.
  Native,
  /// Compatibility mode aligned with WalletCore / Trust Wallet imports.
  ///
  /// Derives a 32-byte key at `m/44'/128'/0'/0'/0'` via SLIP-0010 Ed25519 and
  /// uses it directly as the spend scalar basis.
  WalletCore,
}

/// Prompts the user to choose between generating addresses and scanning common
/// Monero derivation profiles.
pub fn select_operation_mode() -> Result<usize, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select derivation mode (Monero):")
    .items(["Generate addresses", "Scan common derivation paths"])
    .default(0)
    .interact()?;

  Ok(choice)
}

/// Monero network variant — determines the address prefix bytes used during
/// encoding.
#[derive(Clone, Copy)]
pub enum XmrNetwork {
  /// Monero Mainnet — standard addresses start with `4`, subaddresses with `8`.
  Mainnet,

  /// Monero Stagenet/Testnet — standard addresses start with `9`, subaddresses
  /// with `c`.
  Testnet,
}

impl XmrNetwork {
  /// Returns the prefix byte for standard (primary) Monero addresses on this
  /// network.
  ///
  /// | Network  | Prefix byte | First char |
  /// |:---------|:-----------:|:----------:|
  /// | Mainnet  | `18`        | `4`        |
  /// | Testnet  | `53`        | `9`        |
  pub fn standard_prefix(self) -> u8 {
    match self {
      Self::Mainnet => 18,
      Self::Testnet => 53,
    }
  }

  /// Returns the prefix byte for Monero subaddresses on this network.
  ///
  /// | Network  | Prefix byte | First char |
  /// |:---------|:-----------:|:----------:|
  /// | Mainnet  | `42`        | `8`        |
  /// | Testnet  | `63`        | `c`        |
  pub fn subaddress_prefix(self) -> u8 {
    match self {
      Self::Mainnet => 42,
      Self::Testnet => 63,
    }
  }

  /// Returns the network identifier string written into the watch-only export
  /// JSON.
  ///
  /// - Mainnet → `"monero-mainnet"`
  /// - Testnet → `"monero-testnet"`
  pub fn export_network(self) -> &'static str {
    match self {
      Self::Mainnet => "monero-mainnet",
      Self::Testnet => "monero-testnet",
    }
  }
}

/// Prompts the user to choose between Monero Mainnet and Testnet.
///
/// Returns the selected [`XmrNetwork`] variant.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn select_network() -> Result<XmrNetwork, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select Monero network:")
    .items(["Mainnet (4...)", "Testnet (9...)"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => XmrNetwork::Mainnet,
    1 => XmrNetwork::Testnet,
    _ => unreachable!(),
  })
}

/// Prompts the user to choose how Monero keys should be derived from BIP-39.
pub fn select_derivation_mode() -> Result<XmrDerivationMode, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select Monero derivation mode:")
    .items([
      "Native Monero from BIP39 seed (legacy seedctl default)",
      "WalletCore / Trust Wallet compatibility",
    ])
    .default(1)
    .interact()?;

  Ok(match choice {
    0 => XmrDerivationMode::Native,
    1 => XmrDerivationMode::WalletCore,
    _ => unreachable!(),
  })
}

/// Prompts the user for the number of Monero addresses / subaddresses to
/// generate.
///
/// Index `0` produces the standard (primary) address; indices `1..n` produce
/// subaddresses under account 0.
///
/// Defaults to `10` if the user presses Enter without typing a value.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  let count: u32 = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("How many Monero addresses/subaddresses to generate?")
    .default(10)
    .interact_text()?;

  Ok(count)
}

/// Prompts the user for a Monero wallet-RPC URL used for balance queries.
///
/// Returns an empty string immediately when [`RPC_URL_ENABLE`] is `false`
/// (the default), keeping the tool fully offline-capable out of the box.
///
/// When enabled, the user may press Enter without typing anything to skip
/// the balance check for this session.
///
/// The expected endpoint is the Monero `wallet-rpc` JSON-RPC interface, e.g.:
/// `http://127.0.0.1:18088/json_rpc`
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  if !RPC_URL_ENABLE {
    return Ok(String::new());
  }

  let s: String = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("Monero wallet-rpc URL (enter to skip balance check)")
    .allow_empty(true)
    .interact_text()?;
  Ok(s)
}
