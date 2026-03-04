//! XRP Ledger watch-only wallet export builder.
//!
//! Provides [`build_export`] which assembles a [`seedctl_core::export::WalletExport`]
//! document from XRP-specific metadata and the derived account public key,
//! ready to be serialised to a JSON file on disk.

use seedctl_core::export;

use crate::prompts::XrpNetwork;

/// Builds a [`export::WalletExport`] document for an XRP Ledger watch-only wallet.
///
/// Uses the hex-encoded account-level extended public key as the primary
/// identifier and derives a short fingerprint from its first 8 characters.
/// No private key is included — this is always a watch-only export.
///
/// # Parameters
///
/// - `info`         — software metadata slice `[name, version, repository]`.
/// - `network`      — target XRPL network; determines the `"network"` field in
///   the JSON output.
/// - `base_path`    — account-level BIP-32 derivation path string, e.g.
///   `"m/44'/144'/0'/0"`.
/// - `account_xpub` — hex-encoded account-level extended public key used as the
///   canonical wallet identifier and fingerprint source.
///
/// # Returns
///
/// A fully populated [`export::WalletExport`] with:
///
/// - `script_type` = `"xrpl-bip44-secp256k1"`
/// - `watch_only`  = `true`
/// - No private key (`account_xprv` = `None`).
/// - `descriptors` both set to `"xrp-classic-address"` (XRPL does not use
///   Bitcoin-style output descriptors).
pub fn build_export(
  info: &[&str],
  network: XrpNetwork,
  base_path: &str,
  account_xpub: &str,
) -> export::WalletExport {
  export::WalletExport {
    software: export::SoftwareInfo {
      name: info.first().copied().unwrap_or("seedctl").to_string(),
      version: info.get(1).copied().unwrap_or("unknown").to_string(),
      repository: info.get(2).copied().unwrap_or("unknown").to_string(),
    },
    network: network.export_network().into(),
    script_type: "xrpl-bip44-secp256k1".into(),
    key_origin: export::KeyOrigin {
      // Use the first 8 hex chars of the public key as a fingerprint substitute.
      fingerprint: account_xpub.get(..8).unwrap_or("").to_string(),
      derivation_path: base_path.into(),
    },
    watch_only: true,
    keys: export::Keys {
      account_xpub: account_xpub.to_string(),
      // Watch-only export: private key is intentionally omitted.
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      // XRPL uses classic addresses (Base58 + custom alphabet), not Bitcoin
      // output descriptors.
      receive: "xrp-classic-address".into(),
      change: "xrp-classic-address".into(),
    },
  }
}
