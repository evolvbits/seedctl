//! Litecoin watch-only wallet export builder.
//!
//! Provides [`BuildExport`] and [`build_export`] used by [`crate::run`] to
//! assemble a [`seedctl_core::export::WalletExport`] document from Litecoin-
//! specific metadata and the derived account public key, ready to be
//! serialised to a JSON file on disk.

use seedctl_core::{export, utils::format_fingerprint_hex};

use crate::prompts::LtcNetwork;

/// Parameters for [`build_export`].
///
/// Bundles all data needed to assemble a [`export::WalletExport`] document so
/// that the call site does not need to pass a large number of positional
/// arguments.
pub struct BuildExport<'a> {
  /// Software metadata: `[name, version, repository]`.
  pub info: &'a [&'a str],

  /// Target Litecoin network (Mainnet or Testnet).
  pub network: LtcNetwork,

  /// Script type label written into the export, e.g. `"bip84"`.
  pub script_type: &'a str,

  /// Full BIP-32 derivation path, e.g. `"m/84'/2'/0'"`.
  pub derivation_path: &'a str,

  /// 4-byte master fingerprint produced by [`crate::derive::derive_account`].
  pub fingerprint: &'a [u8; 4],

  /// Hex-encoded account-level extended public key.
  pub account_xpub: &'a str,
}

/// Builds a [`export::WalletExport`] document for a Litecoin watch-only wallet.
///
/// No private key is included — this is always a watch-only export.
///
/// # Network string mapping
///
/// | [`LtcNetwork`] variant  | JSON `"network"` value      |
/// |:------------------------|:----------------------------|
/// | `LtcNetwork::Mainnet`   | `"litecoin"`                |
/// | `LtcNetwork::Testnet`   | `"litecoin-testnet"`        |
///
/// # Returns
///
/// A fully populated [`export::WalletExport`] with:
///
/// - `watch_only`  = `true`
/// - No private key (`account_xprv` = `None`).
/// - `descriptors` both set to `"ltc-bip84"` (Litecoin BIP-84 Native SegWit).
pub fn build_export(output: &BuildExport<'_>) -> export::WalletExport {
  // Map the network variant to the JSON string used in the export document.
  let network_str = match output.network {
    LtcNetwork::Mainnet => "litecoin",
    LtcNetwork::Testnet => "litecoin-testnet",
  };

  let descriptor = match output.script_type {
    "bip44" => "ltc-bip44",
    _ => "ltc-bip84",
  };

  export::WalletExport {
    software: export::SoftwareInfo {
      name: output.info[0].to_string(),
      version: output.info[1].to_string(),
      repository: output.info[2].to_string(),
    },
    network: network_str.to_string(),
    script_type: output.script_type.to_string(),
    key_origin: export::KeyOrigin {
      // Format the 4-byte fingerprint as a lowercase hex string.
      fingerprint: format_fingerprint_hex(output.fingerprint),
      derivation_path: output.derivation_path.to_string(),
    },
    // This is always a watch-only export; the private key is intentionally absent.
    watch_only: true,
    keys: export::Keys {
      account_xpub: output.account_xpub.to_string(),
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      receive: descriptor.into(),
      change: descriptor.into(),
    },
  }
}
