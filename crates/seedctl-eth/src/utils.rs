//! Ethereum (ETH) BIP-32 derivation utilities.
//!
//! Thin wrappers around [`seedctl_core::evm`] helpers bound to the
//! [`seedctl_core::evm::ETHEREUM_PROFILE`] so that the rest of the crate never
//! references the profile constant directly.
//!
//! All functions delegate to the shared EVM derivation logic in
//! [`seedctl_core::evm`], which handles path parsing, child-key derivation,
//! and the EIP-55 address encoding algorithm.

use bip32::XPrv;
use std::error::Error;

/// Re-exported derivation style for Ethereum.
///
/// Ethereum uses the same EVM derivation styles as the other EVM chains:
/// - [`DerivationStyle::Standard`] — `m/44'/60'/0'/0/x` (MetaMask-compatible).
/// - [`DerivationStyle::Ledger`]   — `m/44'/60'/0'` (Ledger Ethereum app).
/// - [`DerivationStyle::Custom`]   — user-supplied base path.
pub use seedctl_core::evm::DerivationStyle;

/// Prompts the user to choose an Ethereum derivation style.
///
/// Delegates to [`seedctl_core::evm::select_derivation_style`] with the
/// [`seedctl_core::evm::ETHEREUM_PROFILE`], which supplies the Ethereum-specific
/// prompt strings.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  seedctl_core::evm::select_derivation_style(&seedctl_core::evm::ETHEREUM_PROFILE)
}

/// Converts a [`DerivationStyle`] to its canonical base path string for
/// Ethereum.
///
/// Returns:
/// - `Standard`   → `"m/44'/60'/0'/0"`
/// - `Ledger`     → `"m/44'/60'/0'"`
/// - `Custom(s)`  → `s` verbatim
pub fn style_to_string(style: &DerivationStyle) -> String {
  seedctl_core::evm::style_to_string(style)
}

/// Derives a child [`XPrv`] from `master` by parsing and following `path`.
///
/// Parses `path` as a [`bip32::DerivationPath`] and applies each component
/// in sequence to produce the descendant key.
///
/// # Parameters
///
/// - `master` — BIP-32 master extended private key.
/// - `path`   — BIP-32 derivation path string, e.g. `"m/44'/60'/0'/0"`.
///
/// # Errors
///
/// Returns a boxed error if `path` cannot be parsed or if any child
/// derivation step fails (e.g. an invalid hardened index at an unhardened
/// position).
pub fn derive_from_path(master: XPrv, path: &str) -> Result<XPrv, Box<dyn Error>> {
  seedctl_core::evm::derive_from_path(master, path)
}

/// Derives the address-level [`XPrv`] and its full derivation path string for
/// the given address `index` using the specified `style`.
///
/// # Strategy per style
///
/// - `Standard`: appends a single non-hardened child `index` to
///   `account_xprv` (already at `m/44'/60'/0'/0`), avoiding a full
///   re-derivation from the master key.
/// - `Ledger` / `Custom`: derives the complete path from `master` to handle
///   hardened components that cannot be derived from a public key.
///
/// # Parameters
///
/// - `master`       — BIP-32 master extended private key.
/// - `account_xprv` — Account-level key used for the fast Standard single-step
///   child derivation.
/// - `style`        — Derivation style determining the full path structure.
/// - `index`        — Address index within the style's path layout.
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
  seedctl_core::evm::derive_address_key(master, account_xprv, style, index)
}
