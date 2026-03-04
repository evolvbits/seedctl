//! Watch-only wallet export structures serialised to JSON.
//!
//! [`WalletExport`] is the top-level document written to disk when the user
//! chooses to export a watch-only wallet. All fields are serialised with
//! [`serde`] so the output can be loaded by compatible wallet software.

use serde::Serialize;

/// Top-level watch-only wallet export document.
///
/// Serialised to a pretty-printed JSON file in the user's home directory.
#[derive(Serialize)]
pub struct WalletExport {
  /// Metadata about the software that generated this export.
  pub software: SoftwareInfo,

  /// Target network identifier, e.g. `"bitcoin"`, `"ethereum"`, `"testnet"`.
  pub network: String,

  /// Script / address type, e.g. `"bip84"`, `"bip44"`, `"ethereum-bip44"`.
  pub script_type: String,

  /// Key origin: master fingerprint + derivation path used to derive the
  /// account-level key.
  pub key_origin: KeyOrigin,

  /// `true` when the export omits the private key (watch-only mode).
  pub watch_only: bool,

  /// Account-level key pair (public key always present; private key optional).
  pub keys: Keys,

  /// Output descriptors for the receive and change chains.
  pub descriptors: Descriptors,
}

/// Identifies the `seedctl` release that produced this export.
#[derive(Serialize)]
pub struct SoftwareInfo {
  /// Crate / binary name, e.g. `"seedctl"`.
  pub name: String,

  /// Semantic version string, e.g. `"0.2.1"`.
  pub version: String,

  /// Source-code repository URL.
  pub repository: String,
}

/// Records where in the BIP-32 tree the exported key was derived from.
#[derive(Serialize)]
pub struct KeyOrigin {
  /// Lowercase hex master fingerprint (4 bytes), e.g. `"a1b2c3d4"`.
  pub fingerprint: String,

  /// Full BIP-32 derivation path, e.g. `"m/84'/0'/0'"`.
  pub derivation_path: String,
}

/// Account-level extended keys.
///
/// `account_xprv` is `None` in watch-only exports to avoid leaking the
/// private key.
#[derive(Serialize)]
pub struct Keys {
  /// Account-level extended public key (xpub / zpub / ypub / hex).
  pub account_xpub: String,

  /// Account-level extended private key — omitted (`None`) in watch-only mode.
  pub account_xprv: Option<String>,
}

/// BIP-380 / miniscript output descriptors for the two standard chains.
#[derive(Serialize)]
pub struct Descriptors {
  /// Descriptor for the external (receive) chain, e.g.
  /// `wpkh([a1b2c3d4/84'/0'/0']zpub…/0/*)`.
  pub receive: String,

  /// Descriptor for the internal (change) chain, e.g.
  /// `wpkh([a1b2c3d4/84'/0'/0']zpub…/1/*)`.
  pub change: String,
}
