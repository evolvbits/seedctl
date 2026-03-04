//! Ethereum derivation-path scanner.
//!
//! Thin wrapper around [`seedctl_core::evm::scan_common_paths`] bound to the
//! [`seedctl_core::evm::ETHEREUM_PROFILE`] so that the rest of the crate never
//! references the profile constant directly.

use bip32::XPrv;
use std::error::Error;

/// Scans and prints addresses for the most common Ethereum derivation paths
/// derived from `master`.
///
/// Iterates over the paths defined in
/// [`seedctl_core::evm::ETHEREUM_PROFILE`] (`scan_paths` field), derives an
/// EVM address for each one, and prints a formatted table to stdout so the
/// user can identify which path their existing wallet uses.
///
/// # Parameters
///
/// - `master` — BIP-32 master extended private key derived from the BIP-39
///   seed + passphrase.
///
/// # Errors
///
/// Returns a boxed error if any derivation path cannot be parsed or if key
/// derivation fails for any path.
pub fn scan_common_paths(master: XPrv) -> Result<(), Box<dyn Error>> {
  seedctl_core::evm::scan_common_paths(master, &seedctl_core::evm::ETHEREUM_PROFILE)
}
