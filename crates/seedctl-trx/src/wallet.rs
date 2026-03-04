//! Tron (TRX) watch-only wallet export builder.
//!
//! Provides [`build_export`] which assembles a
//! [`seedctl_core::export::WalletExport`] document from the account-level
//! extended public key and Tron-specific metadata, ready to be serialised
//! to a JSON file on disk.

use seedctl_core::export;
use std::error::Error;

/// Builds a [`export::WalletExport`] document for a Tron watch-only wallet.
///
/// Uses the hex-encoded account-level extended public key as the canonical
/// identifier and derives a short fingerprint from its first 8 characters.
/// No private key is included — this is always a watch-only export.
///
/// # Parameters
///
/// - `info`      — software metadata slice `[name, version, repository]`.
/// - `base_path` — account-level BIP-32 derivation path string, e.g.
///   `"m/44'/195'/0'/0"`.
/// - `xpub`      — account-level BIP-32 extended public key whose bytes are
///   hex-encoded to form the `account_xpub` field and the fingerprint.
///
/// # Returns
///
/// A fully populated [`export::WalletExport`] with:
///
/// - `network`     = `"tron"`
/// - `script_type` = `"tron-bip44"`
/// - `watch_only`  = `true`
/// - No private key (`account_xprv` = `None`).
/// - `descriptors` both set to `"tron-address"` (Tron does not use
///   Bitcoin-style output descriptors).
///
/// # Errors
///
/// This function is infallible in practice — the `Result` wrapper exists only
/// to keep the call-site signature consistent with other chain crates.
pub fn build_export(
  info: &[&str],
  base_path: &str,
  xpub: bip32::ExtendedPublicKey<k256::ecdsa::VerifyingKey>,
) -> Result<export::WalletExport, Box<dyn Error>> {
  let xpub_bytes = xpub.to_bytes();
  let xpub_hex = hex::encode(xpub_bytes);

  Ok(export::WalletExport {
    software: export::SoftwareInfo {
      name: info.first().copied().unwrap_or("seedctl").to_string(),
      version: info.get(1).copied().unwrap_or("unknown").to_string(),
      repository: info.get(2).copied().unwrap_or("unknown").to_string(),
    },
    network: "tron".into(),
    script_type: "tron-bip44".into(),
    key_origin: export::KeyOrigin {
      // Use the first 8 hex chars of the public key as a fingerprint substitute.
      fingerprint: xpub_hex.get(..8).unwrap_or("").to_string(),
      derivation_path: base_path.into(),
    },
    watch_only: true,
    keys: export::Keys {
      account_xpub: xpub_hex,
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      // Tron uses Base58Check addresses, not Bitcoin-style output descriptors.
      receive: "tron-address".into(),
      change: "tron-address".into(),
    },
  })
}
