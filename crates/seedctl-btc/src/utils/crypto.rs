//! Bitcoin extended key prefix conversion utilities (SLIP-132).
//!
//! Provides helpers to re-encode `xprv` / `xpub` keys with alternative
//! version bytes so that wallets display them with the script-type-aware
//! prefixes defined by SLIP-132:
//!
//! | Prefix | Script type     | BIP |
//! |:------:|:----------------|:---:|
//! | `xpub` / `xprv` | Legacy P2PKH   | 44  |
//! | `ypub` / `yprv` | Nested SegWit  | 49  |
//! | `zpub` / `zprv` | Native SegWit  | 84  |
//!
//! All functions operate on the Base58Check-encoded form of the key,
//! replacing only the 4-byte version prefix before re-encoding.

use bitcoin::{
  base58,
  bip32::{Xpriv, Xpub},
  hashes::{Hash, sha256d::Hash as Sha256dHash},
};

/// Re-encodes an `xprv` extended private key with the `zprv` version prefix.
///
/// `zprv` is the SLIP-132 private-key prefix for BIP-84 (Native SegWit /
/// P2WPKH) wallets on Bitcoin Mainnet.
///
/// # Version bytes
///
/// | Prefix | Hex version   |
/// |:------:|:-------------|
/// | `zprv` | `0x04B2430C` |
pub fn xprv_to_zprv(xprv: &Xpriv) -> String {
  let mut data = xprv.encode();
  data[0..4].copy_from_slice(&[0x04, 0xB2, 0x43, 0x0C]);
  base58::encode_check(&data)
}

/// Re-encodes an `xpub` extended public key with the `zpub` version prefix.
///
/// `zpub` is the SLIP-132 public-key prefix for BIP-84 (Native SegWit /
/// P2WPKH) wallets on Bitcoin Mainnet.
///
/// # Version bytes
///
/// | Prefix | Hex version   |
/// |:------:|:-------------|
/// | `zpub` | `0x04B24746` |
pub fn xpub_to_zpub(xpub: &Xpub) -> String {
  let mut data = xpub.encode();
  data[0..4].copy_from_slice(&[0x04, 0xB2, 0x47, 0x46]);
  base58::encode_check(&data)
}

/// Re-encodes an `xprv` extended private key with the `yprv` version prefix.
///
/// `yprv` is the SLIP-132 private-key prefix for BIP-49 (Nested SegWit /
/// P2SH-P2WPKH) wallets on Bitcoin Mainnet.
///
/// # Version bytes
///
/// | Prefix | Hex version   |
/// |:------:|:-------------|
/// | `yprv` | `0x049D7878` |
pub fn xprv_to_yprv(xprv: &Xpriv) -> String {
  let mut data = xprv.encode();
  data[0..4].copy_from_slice(&[0x04, 0x9D, 0x78, 0x78]);
  base58::encode_check(&data)
}

/// Converts an `xpub` to any SLIP-132 public-key prefix by replacing the
/// 4-byte version field with the supplied `version` value (big-endian).
///
/// This is the generic helper used by [`xpub_to_ypub`] and [`xpub_to_zpub`].
///
/// # Parameters
///
/// - `xpub`    — source extended public key in any Base58Check-encoded form.
/// - `version` — target 4-byte version as a `u32` (big-endian byte order).
///
/// # Panics
///
/// Panics if `xpub.to_string()` is not a valid Base58Check-encoded string,
/// which should never happen for keys produced by the `bitcoin` crate.
pub fn convert_xpub_prefix(xpub: &Xpub, version: u32) -> String {
  // Decode Base58Check into raw bytes.
  let mut data = base58::decode_check(&xpub.to_string()).expect("Invalid Base58Check xpub");

  // Replace the 4-byte version prefix.
  data[0..4].copy_from_slice(&version.to_be_bytes());

  // Recompute the double-SHA256 checksum over the new payload.
  let checksum: Sha256dHash = Hash::hash(&data[..data.len() - 4]);
  let len = data.len();
  data[len - 4..len].copy_from_slice(&checksum[..4]);

  base58::encode_check(&data)
}

/// Re-encodes an `xpub` extended public key with the `ypub` version prefix.
///
/// `ypub` is the SLIP-132 public-key prefix for BIP-49 (Nested SegWit /
/// P2SH-P2WPKH) wallets on Bitcoin Mainnet.
///
/// # Version bytes
///
/// | Prefix | Hex version   |
/// |:------:|:-------------|
/// | `ypub` | `0x049D7CB2` |
pub fn xpub_to_ypub(xpub: &Xpub) -> String {
  convert_xpub_prefix(xpub, 0x049d7cb2)
}
