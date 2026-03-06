//! Solana watch-only wallet export builder.
//!
//! Provides [`build_export`] which assembles a [`seedctl_core::export::WalletExport`]
//! document from the first account's public key hex string, ready to be
//! serialised to a JSON file on disk.

use seedctl_core::export;
use std::error::Error;

/// Builds a [`export::WalletExport`] document for a Solana watch-only wallet.
///
/// Uses the first account's public key (index 0) as the canonical identifier
/// and fingerprint source, since Solana Ed25519 keys have no BIP-32 extended
/// key hierarchy — each account is an independent SLIP-0010 derivation.
///
/// # Parameters
///
/// - `info`            — software metadata slice `[name, version, repository]`.
/// - `first_pubkey_hex`— hex-encoded 32-byte Ed25519 verifying key for account
///   index 0.
/// - `first_path`      — derivation path used for account index 0.
///
/// # Returns
///
/// A fully populated [`export::WalletExport`] with:
///
/// - `network`     = `"solana"`
/// - `script_type` = `"solana-ed25519"`
/// - `watch_only`  = `true`
/// - `key_origin.derivation_path` = path used for the first derived account
/// - No private key (`account_xprv` = `None`).
/// - `descriptors` both set to `"solana-address"` (Solana does not use
///   Bitcoin-style output descriptors).
///
/// # Errors
///
/// This function is infallible in practice — the `Result` wrapper exists only
/// to keep the call-site signature consistent with other chain crates.
pub fn build_export(
  info: &[&str],
  first_pubkey_hex: &str,
  first_path: &str,
) -> Result<export::WalletExport, Box<dyn Error>> {
  Ok(export::WalletExport {
    software: export::SoftwareInfo {
      name: info.first().copied().unwrap_or("seedctl").to_string(),
      version: info.get(1).copied().unwrap_or("unknown").to_string(),
      repository: info.get(2).copied().unwrap_or("unknown").to_string(),
    },
    network: "solana".into(),
    script_type: "solana-ed25519".into(),
    key_origin: export::KeyOrigin {
      // Use the first 8 hex chars of the public key as a fingerprint substitute.
      fingerprint: first_pubkey_hex.get(..8).unwrap_or("").to_string(),
      derivation_path: first_path.into(),
    },
    watch_only: true,
    keys: export::Keys {
      account_xpub: first_pubkey_hex.to_string(),
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      // Solana uses base58 public keys as addresses, not Bitcoin-style descriptors.
      receive: "solana-address".into(),
      change: "solana-address".into(),
    },
  })
}
