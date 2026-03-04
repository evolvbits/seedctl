//! Polygon (MATIC/POL) watch-only wallet export builder.
//!
//! Thin wrapper around [`seedctl_core::evm::build_watch_only_export`] bound
//! to the [`seedctl_core::evm::POLYGON_PROFILE`] so that the rest of the
//! crate never references the profile constant directly.

use seedctl_core::export;
use std::error::Error;

/// Builds a [`export::WalletExport`] document for a Polygon watch-only wallet.
///
/// Delegates to [`seedctl_core::evm::build_watch_only_export`] with the
/// [`seedctl_core::evm::POLYGON_PROFILE`], which fills in the Polygon-specific
/// `network`, `script_type`, and `descriptors` fields.
///
/// # Parameters
///
/// - `info`      — software metadata slice `[name, version, repository]`.
/// - `base_path` — account-level BIP-32 derivation path string, e.g.
///   `"m/44'/60'/0'/0"`.
/// - `xpub`      — account-level EVM extended public key used to derive the
///   watch-only descriptor and fingerprint.
///
/// # Returns
///
/// A fully populated [`export::WalletExport`] with:
///
/// - `network`     = `"polygon"`
/// - `script_type` = `"polygon-evm-bip44"`
/// - `watch_only`  = `true`
/// - No private key (`account_xprv` = `None`).
///
/// # Errors
///
/// This function is infallible in practice — the `Result` wrapper exists
/// only to keep the call-site signature uniform across all EVM crates.
pub fn build_export(
  info: &[&str],
  base_path: &str,
  xpub: seedctl_core::evm::EvmAccountXpub,
) -> Result<export::WalletExport, Box<dyn Error>> {
  Ok(seedctl_core::evm::build_watch_only_export(
    &seedctl_core::evm::POLYGON_PROFILE,
    info,
    base_path,
    xpub,
  ))
}
