//! Tron (TRX) BIP-32 derivation utilities and address-encoding helpers.
//!
//! Provides:
//! - [`DerivationStyle`]       — enum of supported Tron derivation path variants.
//! - [`select_derivation_style`] — interactive prompt for choosing a style.
//! - [`style_to_string`]       — converts a style to its canonical base path.
//! - [`to_tron_address`]       — encodes a 20-byte account ID as a `T…` address.
//! - [`derive_path`]           — iteratively applies a full derivation path.
//! - [`derive_from_path`]      — parses and applies a BIP-32 path string.
//! - [`derive_address_key`]    — derives the leaf `XPrv` and path string for a given index.
//! - [`build_path`]            — constructs the full `DerivationPath` for a given index.

use bip32::{ChildNumber, DerivationPath, XPrv};
use dialoguer::{Input, Select};
use seedctl_core::ui::dialoguer_theme;
use std::error::Error;

/// Standard Tron derivation base path (TronLink, Trust Wallet).
const TRON_PATH_STANDARD: &str = "m/44'/195'/0'/0";

/// Ledger Tron app derivation base path.
const TRON_PATH_LEDGER: &str = "m/44'/195'/0'";

/// Supported derivation path styles for Tron wallets.
///
/// Each variant determines how the full per-address BIP-32 path is constructed
/// given an address `index`.
#[derive(Clone)]
pub enum DerivationStyle {
  /// Standard path: `m/44'/195'/0'/0/<index>`.
  ///
  /// Compatible with TronLink and Trust Wallet — the most common layout.
  Standard,

  /// Ledger-style path: `m/44'/195'/0'/<index>'/0/0`.
  ///
  /// Used by the Ledger Tron app's account-index layout.
  Ledger,

  /// Fully custom base path supplied by the user.
  ///
  /// The index is appended as `/<index>` unless the template ends with `/`
  /// (in which case the separator is omitted) or contains `{index}` (which
  /// is replaced verbatim).
  Custom(String),
}

/// Prompts the user to choose a Tron derivation style.
///
/// Presents three options in an interactive [`dialoguer::Select`] prompt:
/// - `Standard (m/44'/195'/0'/0/x)` — default; TronLink-compatible.
/// - `Ledger style (m/44'/195'/0'/x/0)` — Ledger Tron app layout.
/// - `Custom path` — allows the user to enter any base path.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let choice = Select::with_theme(&theme)
    .with_prompt("Select derivation style:")
    .items([
      "Standard (m/44'/195'/0'/0/x)",
      "Ledger style (m/44'/195'/0'/x/0)",
      "Custom path",
    ])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => DerivationStyle::Standard,
    1 => DerivationStyle::Ledger,
    2 => {
      let theme = dialoguer_theme("►");
      let input: String = Input::with_theme(&theme)
        .with_prompt("Enter custom derivation base path")
        .default(TRON_PATH_STANDARD.into())
        .interact_text()?;
      DerivationStyle::Custom(input)
    }
    _ => unreachable!(),
  })
}

/// Converts a [`DerivationStyle`] to its canonical base path string.
///
/// Returns:
/// - `Standard` → `"m/44'/195'/0'/0"`
/// - `Ledger`   → `"m/44'/195'/0'"`
/// - `Custom(s)` → `s` verbatim
pub fn style_to_string(style: &DerivationStyle) -> String {
  match style {
    DerivationStyle::Standard => TRON_PATH_STANDARD.into(),
    DerivationStyle::Ledger => TRON_PATH_LEDGER.into(),
    DerivationStyle::Custom(s) => s.clone(),
  }
}

/// Encodes a 20-byte Tron account ID as a Base58Check address (prefix `T`).
///
/// Prepends the Tron version byte `0x41` to `addr_20` and applies
/// Base58Check encoding, producing the canonical `T…` address format
/// used on Tron Mainnet.
///
/// # Parameters
///
/// - `addr_20` — 20-byte raw account ID derived from Keccak-256 of the
///   uncompressed public key (last 20 bytes of the 32-byte hash).
///
/// # Returns
///
/// A 34-character Base58Check-encoded Tron address string starting with `T`.
pub fn to_tron_address(addr_20: &[u8]) -> String {
  // Tron version byte 0x41 identifies Mainnet base addresses.
  let mut payload = Vec::with_capacity(1 + addr_20.len());
  payload.push(0x41u8);
  payload.extend_from_slice(addr_20);
  bs58::encode(payload).with_check().into_string()
}

/// Iteratively applies each component of `path` to `key`, returning the
/// fully derived descendant `XPrv`.
///
/// Consumes `key` by value; clone before calling if the original is needed.
///
/// # Errors
///
/// Returns a boxed error if any child derivation step fails (e.g. attempting
/// a hardened derivation from a public key, which is impossible by design).
pub fn derive_path(mut key: XPrv, path: &DerivationPath) -> Result<XPrv, Box<dyn Error>> {
  for c in path.iter() {
    key = key.derive_child(c)?;
  }
  Ok(key)
}

/// Parses `path` as a [`DerivationPath`] and derives the descendant `XPrv`
/// from `master`.
///
/// # Parameters
///
/// - `master` — BIP-32 master extended private key.
/// - `path`   — BIP-32 derivation path string, e.g. `"m/44'/195'/0'/0"`.
///
/// # Errors
///
/// Returns a boxed error if `path` cannot be parsed or if any child
/// derivation step fails.
pub fn derive_from_path(master: XPrv, path: &str) -> Result<XPrv, Box<dyn Error>> {
  let dp: DerivationPath = path.parse()?;
  derive_path(master, &dp)
}

/// Derives the address-level `XPrv` and its full derivation path string for
/// the given `index` using the specified `style`.
///
/// # Strategy per style
///
/// - `Standard`: derives only the non-hardened child `index` from
///   `account_xprv` (the key already at `m/44'/195'/0'/0`).
/// - `Ledger` / `Custom`: re-derives the complete path from `master`.
///
/// # Parameters
///
/// - `master`       — BIP-32 master extended private key.
/// - `account_xprv` — Account-level key used for Standard single-child
///   derivation.
/// - `style`        — Derivation style determining the full path layout.
/// - `index`        — Address index within the style's path structure.
///
/// # Returns
///
/// A tuple of `(leaf_XPrv, derivation_path_string)`.
///
/// # Errors
///
/// Returns a boxed error if path construction or any derivation step fails.
pub fn derive_address_key(
  master: &XPrv,
  account_xprv: &XPrv,
  style: &DerivationStyle,
  index: u32,
) -> Result<(XPrv, String), Box<dyn Error>> {
  let path = build_path(style, index)?;
  let path_str = path.to_string();

  let key = match style {
    DerivationStyle::Standard => {
      // Standard: account key is already at m/44'/195'/0'/0; append index directly.
      let child = ChildNumber::new(index, false)?;
      account_xprv.clone().derive_child(child)?
    }
    // Ledger and Custom: derive the full path from the master key.
    DerivationStyle::Ledger | DerivationStyle::Custom(_) => derive_path(master.clone(), &path)?,
  };

  Ok((key, path_str))
}

/// Constructs the full [`DerivationPath`] for the given `index` and `style`.
///
/// # Path construction rules
///
/// | Style      | Pattern                                    |
/// |:-----------|:-------------------------------------------|
/// | Standard   | `m/44'/195'/0'/0/<index>`                  |
/// | Ledger     | `m/44'/195'/0'/<index>'/0/0`               |
/// | Custom     | template with `{index}` replaced, or `/<index>` appended |
///
/// # Errors
///
/// Returns a boxed error if the constructed path string cannot be parsed as
/// a valid [`DerivationPath`].
pub fn build_path(style: &DerivationStyle, index: u32) -> Result<DerivationPath, Box<dyn Error>> {
  let path_str = match style {
    DerivationStyle::Standard => format!("{}/{}", TRON_PATH_STANDARD, index),
    DerivationStyle::Ledger => format!("{}/{}'/0/0", TRON_PATH_LEDGER, index),
    DerivationStyle::Custom(template) => {
      if template.contains("{index}") {
        template.replace("{index}", &index.to_string())
      } else if template.ends_with('/') {
        format!("{}{}", template, index)
      } else {
        format!("{}/{}", template, index)
      }
    }
  };

  Ok(path_str.parse()?)
}
