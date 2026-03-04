//! Bitcoin wallet key management and watch-only export builder.
//!
//! Provides [`master_from_mnemonic`] for deriving the BIP-32 master key from
//! a BIP-39 mnemonic, [`account_key_strings`] for encoding the account keys
//! with the correct SLIP-132 prefix, [`key_origin_and_descriptors`] for
//! assembling output descriptors, and [`build_export`] for constructing the
//! watch-only JSON export document.

use bip39::Mnemonic;
use bitcoin::{Network, bip32::Xpriv};
use seedctl_core::{export, utils::format_fingerprint_hex};
use std::error::Error;

use crate::utils::{
  crypto::{xprv_to_yprv, xprv_to_zprv, xpub_to_ypub, xpub_to_zpub},
  format::{format_key_origin, output_descriptor},
};

/// Derives a BIP-32 master extended private key from a BIP-39 mnemonic and passphrase.
///
/// The master key is produced by passing the 64-byte BIP-39 seed (derived from
/// the mnemonic + passphrase) into [`Xpriv::new_master`] with the specified
/// Bitcoin network.
///
/// # Parameters
///
/// - `mnemonic`   — validated BIP-39 mnemonic object.
/// - `passphrase` — optional BIP-39 passphrase; pass `""` for no passphrase.
/// - `network`    — `Network::Bitcoin` for Mainnet or `Network::Testnet`.
///
/// # Errors
///
/// Returns a boxed error if [`Xpriv::new_master`] fails (e.g. the derived seed
/// is invalid for secp256k1, which is astronomically unlikely in practice).
pub fn master_from_mnemonic(
  mnemonic: &Mnemonic,
  passphrase: &str,
  network: Network,
) -> Result<Xpriv, Box<dyn Error>> {
  let seed = mnemonic.to_seed(passphrase);
  Ok(Xpriv::new_master(network, &seed)?)
}

/// Parameters for [`build_export`].
///
/// Bundles all the data needed to assemble a [`export::WalletExport`] document
/// so that the call site does not need to pass a large number of positional
/// arguments.
pub struct BuildExport<'a> {
  /// Software metadata: `[name, version, repository]`.
  pub info: &'a [&'a str],

  /// Target Bitcoin network (Mainnet or Testnet).
  pub network: Network,

  /// Script type label written into the export, e.g. `"bip84"`.
  pub script_type: &'a str,

  /// Full BIP-32 derivation path, e.g. `"m/84'/0'/0'"`.
  pub derivation_path: &'a str,

  /// 4-byte master fingerprint produced by [`crate::derive::derive_account`].
  pub fingerprint: &'a [u8; 4],

  /// Account-level extended public key string (xpub / ypub / zpub).
  pub account_xpub: &'a str,

  /// Account-level extended private key string — `None` for watch-only exports.
  pub account_xprv: Option<&'a str>,

  /// Output descriptor for the external (receive) chain.
  pub desc_receive: &'a str,

  /// Output descriptor for the internal (change) chain.
  pub desc_change: &'a str,
}

/// Builds a [`export::WalletExport`] document from the provided parameters.
///
/// The `watch_only` flag is derived automatically: when `account_xprv` is
/// `None`, the export is considered watch-only and the private key field is
/// omitted from the JSON output.
///
/// # Network string mapping
///
/// | [`Network`] variant  | JSON `"network"` value |
/// |:---------------------|:-----------------------|
/// | `Network::Bitcoin`   | `"bitcoin"`            |
/// | `Network::Testnet`   | `"testnet"`            |
/// | anything else        | `"unknown"`            |
pub fn build_export(output: &BuildExport) -> export::WalletExport {
  let network_str = match output.network {
    Network::Bitcoin => "bitcoin",
    Network::Testnet => "testnet",
    _ => "unknown",
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
      fingerprint: format_fingerprint_hex(output.fingerprint),
      derivation_path: output.derivation_path.to_string(),
    },
    watch_only: output.account_xprv.is_none(),
    keys: export::Keys {
      account_xpub: output.account_xpub.to_string(),
      account_xprv: output.account_xprv.map(String::from),
    },
    descriptors: export::Descriptors {
      receive: output.desc_receive.to_string(),
      change: output.desc_change.to_string(),
    },
  }
}

/// Returns the account-level `(xprv_string, xpub_string)` pair encoded with
/// the correct SLIP-132 prefix for the given address type.
///
/// | `address_type` | xpub prefix | xprv prefix | BIP |
/// |:-:|:--|:--|:-:|
/// | `0` | `zpub` | `zprv` | 84 — Native SegWit   |
/// | `1` | `ypub` | `yprv` | 49 — Nested SegWit   |
/// | `2` | `xpub` | `xprv` | 44 — Legacy P2PKH    |
///
/// # Returns
///
/// `(account_xprv_string, account_xpub_string)` — note the order: private key
/// first, public key second, matching the pattern used in [`crate::run`].
///
/// # Panics
///
/// Panics (via `unreachable!`) if `address_type` is outside `0..=2`.
pub fn account_key_strings(
  acc_xprv: &Xpriv,
  acc_xpub: &bitcoin::bip32::Xpub,
  address_type: usize,
) -> (String, String) {
  let account_xpub = match address_type {
    0 => xpub_to_zpub(acc_xpub),
    1 => xpub_to_ypub(acc_xpub),
    2 => acc_xpub.to_string(),
    _ => unreachable!(),
  };
  let account_xprv = match address_type {
    0 => xprv_to_zprv(acc_xprv),
    1 => xprv_to_yprv(acc_xprv),
    2 => acc_xprv.to_string(),
    _ => unreachable!(),
  };

  (account_xprv, account_xpub)
}

/// Builds the key-origin string and the receive / change output descriptors.
///
/// Delegates to [`format_key_origin`] and [`output_descriptor`] to assemble
/// the three strings needed for the wallet display and the export document.
///
/// # Returns
///
/// `(key_origin, desc_receive, desc_change)` — all three strings in a tuple.
pub fn key_origin_and_descriptors(
  fingerprint: [u8; 4],
  purpose: u32,
  btc_coin_type: u32,
  account_xpub: &str,
) -> (String, String, String) {
  let key_origin = format_key_origin(fingerprint, purpose, btc_coin_type);
  let desc_receive = output_descriptor(purpose, &key_origin, account_xpub, 0);
  let desc_change = output_descriptor(purpose, &key_origin, account_xpub, 1);
  (key_origin, desc_receive, desc_change)
}
