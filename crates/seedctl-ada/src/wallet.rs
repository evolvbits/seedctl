//! Cardano watch-only wallet export builder.
//!
//! Provides [`build_export`] which assembles a [`export::WalletExport`]
//! document from Cardano-specific metadata and the derived account public key,
//! ready to be serialised to a JSON file on disk.

use seedctl_core::export;

use crate::derive;
use crate::prompts::AdaNetwork;

/// Builds a [`export::WalletExport`] document for a Cardano watch-only wallet.
///
/// Encodes the CIP-1852 derivation path (`m/1852'/1815'/<account>'`) as the
/// key origin, and uses the hex-encoded account public key as the primary
/// identifier. No private key is included — this is always a watch-only export.
///
/// # Parameters
///
/// - `info`             — software metadata slice `[name, version, repository]`.
/// - `network`          — target Cardano network; determines the `"network"`
///   field in the JSON output.
/// - `account`          — account index used to reconstruct the derivation path
///   label (typically `0`).
/// - `account_xpub_hex` — hex-encoded account-level extended public key at
///   `m/1852'/1815'/<account>'`.
///
/// # Returns
///
/// A fully populated [`export::WalletExport`] with:
///
/// - `script_type` = `"cardano-cip1852-shelley-base"`
/// - `watch_only`  = `true`
/// - `descriptors` both set to `"cardano-base-address"` (Cardano does not use
///   Bitcoin-style output descriptors).
pub fn build_export(
  info: &[&str],
  network: AdaNetwork,
  account: u32,
  account_xpub_hex: &str,
) -> export::WalletExport {
  export::WalletExport {
    software: export::SoftwareInfo {
      name: info.first().copied().unwrap_or("seedctl").to_string(),
      version: info.get(1).copied().unwrap_or("unknown").to_string(),
      repository: info.get(2).copied().unwrap_or("unknown").to_string(),
    },
    network: network.export_network().into(),
    script_type: "cardano-cip1852-shelley-base".into(),
    key_origin: export::KeyOrigin {
      // Use the first 8 hex chars of the xpub as a fingerprint substitute.
      fingerprint: account_xpub_hex.get(..8).unwrap_or("").to_string(),
      derivation_path: derive::account_path(account),
    },
    watch_only: true,
    keys: export::Keys {
      account_xpub: account_xpub_hex.to_string(),
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      // Cardano uses bech32 base addresses, not Bitcoin-style descriptors.
      receive: "cardano-base-address".into(),
      change: "cardano-base-address".into(),
    },
  }
}
