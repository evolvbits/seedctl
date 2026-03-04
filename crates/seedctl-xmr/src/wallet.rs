//! Monero (XMR) watch-only wallet export builder.
//!
//! Provides [`build_export`] which assembles a
//! [`seedctl_core::export::WalletExport`] document from Monero-specific
//! metadata and the derived spend public key, ready to be serialised to a
//! JSON file on disk.
//!
//! # Note on Monero key representation
//!
//! Unlike Bitcoin/EVM chains, Monero does not use BIP-32 extended keys.
//! The "account public key" stored in the export is the **spend public key**
//! (32-byte compressed Ed25519 point, hex-encoded), which uniquely identifies
//! the wallet's primary address on the chosen network.

use seedctl_core::export;

use crate::prompts::XmrNetwork;

/// Builds a [`export::WalletExport`] document for a Monero watch-only wallet.
///
/// Uses the hex-encoded spend public key as the primary wallet identifier and
/// derives a short fingerprint from its first 8 characters. No private key is
/// included — this is always a watch-only export.
///
/// # Parameters
///
/// - `info`              — software metadata slice `[name, version, repository]`.
/// - `network`           — target Monero network; determines the `"network"` field
///   in the JSON output (`"monero-mainnet"` or `"monero-testnet"`).
/// - `first_public_hex`  — hex-encoded 32-byte spend public key for the primary
///   address (index 0), produced by [`crate::derive::XmrWallet::spend_public_hex`].
///
/// # Returns
///
/// A fully populated [`export::WalletExport`] with:
///
/// - `script_type` = `"monero-standard-subaddress"`
/// - `watch_only`  = `true`
/// - `key_origin.derivation_path` = `"m/44'/128'/0'/0/0"` (conventional label
///   only — Monero does not actually follow BIP-32/BIP-44 derivation).
/// - No private key (`account_xprv` = `None`).
/// - `descriptors` both set to `"monero-subaddress"` (Monero uses its own
///   address encoding, not Bitcoin-style output descriptors).
pub fn build_export(
  info: &[&str],
  network: XmrNetwork,
  first_public_hex: &str,
) -> export::WalletExport {
  export::WalletExport {
    software: export::SoftwareInfo {
      name: info.first().copied().unwrap_or("seedctl").to_string(),
      version: info.get(1).copied().unwrap_or("unknown").to_string(),
      repository: info.get(2).copied().unwrap_or("unknown").to_string(),
    },
    network: network.export_network().into(),
    script_type: "monero-standard-subaddress".into(),
    key_origin: export::KeyOrigin {
      // Use the first 8 hex chars of the spend public key as a fingerprint
      // substitute (Monero has no BIP-32 fingerprint concept).
      fingerprint: first_public_hex.get(..8).unwrap_or("").to_string(),
      // Conventional BIP-44-style label; not an actual Monero derivation path.
      derivation_path: "m/44'/128'/0'/0/0".into(),
    },
    // Watch-only export: the spend private key is intentionally omitted.
    watch_only: true,
    keys: export::Keys {
      account_xpub: first_public_hex.to_string(),
      // Private key is never included in watch-only exports.
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      // Monero uses Base58-encoded addresses with custom prefix bytes, not
      // Bitcoin-style output descriptors.
      receive: "monero-subaddress".into(),
      change: "monero-subaddress".into(),
    },
  }
}
