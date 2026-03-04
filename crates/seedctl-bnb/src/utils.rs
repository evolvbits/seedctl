//! BNB Smart Chain EVM derivation utilities.
//!
//! Thin wrappers around [`seedctl_core::evm`] helpers bound to the
//! [`seedctl_core::evm::BNB_PROFILE`] so that the rest of the crate never
//! references the profile constant directly.

use bip32::XPrv;
use std::error::Error;

/// Re-exported derivation style for BNB Smart Chain.
///
/// BNB Smart Chain uses the same EVM derivation styles as Ethereum:
/// Standard (`m/44'/60'/0'/0/x`), Ledger, or Custom.
pub use seedctl_core::evm::DerivationStyle;

/// Prompts the user to choose a BNB Smart Chain derivation style.
///
/// Delegates to [`seedctl_core::evm::select_derivation_style`] with the
/// [`seedctl_core::evm::BNB_PROFILE`], which supplies the BNB-specific
/// prompt strings.
pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  seedctl_core::evm::select_derivation_style(&seedctl_core::evm::BNB_PROFILE)
}

/// Converts a [`DerivationStyle`] to its base path string for BNB Smart Chain.
///
/// Returns the canonical path string:
/// - `Standard` → `"m/44'/60'/0'/0"`
/// - `Ledger`   → `"m/44'/60'/0'"`
/// - `Custom(s)` → `s` verbatim
pub fn style_to_string(style: &DerivationStyle) -> String {
  seedctl_core::evm::style_to_string(style)
}

/// Derives a child `XPrv` from `master` by following `path`.
///
/// Parses `path` as a [`bip32::DerivationPath`] and applies each component
/// in sequence to produce the descendant key.
///
/// # Errors
///
/// Returns a boxed error if `path` cannot be parsed or if any child
/// derivation step fails.
pub fn derive_from_path(master: XPrv, path: &str) -> Result<XPrv, Box<dyn Error>> {
  seedctl_core::evm::derive_from_path(master, path)
}

/// Derives the address-level `XPrv` and its derivation path string for the
/// given `index` using the specified `style`.
///
/// # Parameters
///
/// - `master`      — BIP-32 master extended private key.
/// - `account_xprv`— Account-level extended private key (used for Standard
///   style child derivation).
/// - `style`       — Derivation style determining the full path structure.
/// - `index`       — Address index within the derivation style's layout.
///
/// # Returns
///
/// A tuple of `(child_XPrv, derivation_path_string)`.
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
  seedctl_core::evm::derive_address_key(master, account_xprv, style, index)
}
