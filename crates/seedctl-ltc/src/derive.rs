//! Litecoin address derivation using BIP-84 (Native SegWit P2WPKH).
//!
//! Provides [`derive_account`] for deriving the account-level key pair from a
//! BIP-32 master key, and [`receive_addresses`] for generating a batch of
//! bech32-encoded Litecoin receive addresses (`ltc1…` on Mainnet, `tltc1…`
//! on Testnet).

use bech32::Hrp;
use bip32::{ChildNumber, DerivationPath, XPrv, XPub};
use ripemd::Ripemd160;
use sha2::{Digest as ShaDigest, Sha256};
use std::error::Error;

use crate::prompts::{LtcDerivationStyle, LtcNetwork};

/// Derives the account-level key pair at `m/84'/<coin_type>'/0'`.
///
/// Returns a tuple of:
/// - `XPrv`     — account-level extended private key.
/// - `XPub`     — corresponding account-level extended public key.
/// - `[u8; 4]`  — master fingerprint (first 4 bytes of the master public key
///   hash), used in export documents and watch-only wallet descriptors.
///
/// # Parameters
///
/// - `master`     — BIP-32 master extended private key derived from the
///   BIP-39 seed.
/// - `coin_type`  — SLIP-44 coin type: `2` for Litecoin Mainnet, `1` for
///   Testnet.
///
/// # Errors
///
/// Returns a boxed error if the derivation path cannot be parsed or if any
/// hardened child derivation step fails.
pub fn derive_account(
  master: &XPrv,
  coin_type: u32,
  purpose: u32,
) -> Result<(XPrv, XPub, [u8; 4]), Box<dyn Error>> {
  let account_path: DerivationPath = format!("m/{purpose}'/{coin_type}'/0'").parse()?;
  let account_xprv = derive_path(master.clone(), &account_path)?;
  let account_xpub = account_xprv.public_key();
  let fingerprint = XPub::from(master).fingerprint();

  Ok((account_xprv, account_xpub, fingerprint))
}

/// Generates the first `count` receive addresses from an account-level private key.
///
/// Derives child keys along the external (receive) chain `0/i` and encodes
/// each compressed public key as a bech32 P2WPKH address.
///
/// # Parameters
///
/// - `account_xprv` — account-level extended private key at `m/84'/<coin_type>'/0'`.
/// - `network`      — [`LtcNetwork::Mainnet`] (`ltc1…`) or
///   [`LtcNetwork::Testnet`] (`tltc1…`).
/// - `coin_type`    — SLIP-44 coin type used to build the derivation path label.
/// - `count`        — number of addresses to generate.
///
/// # Returns
///
/// A `Vec` of `(derivation_path_string, bech32_address)` pairs, one per index
/// `0..count`.
///
/// # Errors
///
/// Returns a boxed error if child key derivation, signing key construction,
/// or bech32 encoding fails for any index.
pub fn receive_addresses(
  account_xprv: &XPrv,
  network: LtcNetwork,
  coin_type: u32,
  purpose: u32,
  style: LtcDerivationStyle,
  count: u32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
  let mut out = Vec::with_capacity(count as usize);
  let receive = ChildNumber::new(0, false)?;

  for i in 0..count {
    let index = ChildNumber::new(i, false)?;
    let xprv = account_xprv
      .clone()
      .derive_child(receive)?
      .derive_child(index)?;

    let signing = k256::ecdsa::SigningKey::from_bytes(&xprv.private_key().to_bytes())?;
    let pubkey = signing.verifying_key().to_encoded_point(true);
    let addr = match style {
      LtcDerivationStyle::BIP84 => ltc_bech32_p2wpkh(pubkey.as_bytes(), network)?,
      LtcDerivationStyle::BIP44 => ltc_legacy_p2pkh(pubkey.as_bytes(), network),
    };

    out.push((format!("m/{purpose}'/{coin_type}'/0'/0/{i}"), addr));
  }

  Ok(out)
}

/// Derives a child key by applying each component of `path` in sequence.
///
/// Consumes the `key` by value and returns the fully derived descendant key.
///
/// # Errors
///
/// Returns a boxed error if any child derivation step fails (e.g. attempting
/// a hardened derivation from a public key).
fn derive_path(mut key: XPrv, path: &DerivationPath) -> Result<XPrv, Box<dyn Error>> {
  for child in path.iter() {
    key = key.derive_child(child)?;
  }
  Ok(key)
}

/// Encodes a compressed public key as a bech32 P2WPKH address for Litecoin.
///
/// Applies SHA-256 followed by RIPEMD-160 to produce the 20-byte witness
/// program and encodes it as a SegWit v0 bech32 string using the
/// network-appropriate HRP.
///
/// # Parameters
///
/// - `pubkey_compressed` — 33-byte compressed SEC public key.
/// - `network`           — target network; determines the HRP (`ltc` or `tltc`).
///
/// # Errors
///
/// Returns a boxed error if the HRP string is invalid or bech32 encoding fails.
fn ltc_bech32_p2wpkh(
  pubkey_compressed: &[u8],
  network: LtcNetwork,
) -> Result<String, Box<dyn Error>> {
  // SHA-256 → RIPEMD-160 (standard P2WPKH witness program construction).
  let sha = Sha256::digest(pubkey_compressed);
  let rip = Ripemd160::digest(sha);

  let hrp = match network {
    LtcNetwork::Mainnet => "ltc",
    LtcNetwork::Testnet => "tltc",
  };

  // Use the segwit-specific encoder for witness-v0 addresses.
  Ok(bech32::segwit::encode_v0(Hrp::parse(hrp)?, &rip)?)
}

/// Encodes a compressed public key as a legacy Litecoin P2PKH Base58Check
/// address (`L...` on mainnet).
fn ltc_legacy_p2pkh(pubkey_compressed: &[u8], network: LtcNetwork) -> String {
  let sha = Sha256::digest(pubkey_compressed);
  let rip = Ripemd160::digest(sha);

  // Version byte for P2PKH.
  let version = match network {
    LtcNetwork::Mainnet => 0x30u8, // L...
    LtcNetwork::Testnet => 0x6fu8, // m... / n...
  };

  let mut payload = Vec::with_capacity(1 + rip.len() + 4);
  payload.push(version);
  payload.extend_from_slice(&rip);

  // Base58Check checksum = first 4 bytes of SHA256(SHA256(version||hash160)).
  let checksum = Sha256::digest(Sha256::digest(&payload));
  payload.extend_from_slice(&checksum[..4]);

  bs58::encode(payload).into_string()
}

#[cfg(test)]
mod tests {
  use super::{derive_account, ltc_bech32_p2wpkh, receive_addresses};
  use crate::prompts::{LtcDerivationStyle, LtcNetwork};
  use bip39::Mnemonic;
  use seedctl_core::utils::master_from_mnemonic_bip32;

  #[test]
  fn bech32_address_shape_is_valid() {
    // Compressed secp256k1 pubkey.
    let pubkey = hex::decode("0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798")
      .expect("valid hex");
    let addr = ltc_bech32_p2wpkh(&pubkey, LtcNetwork::Mainnet).expect("encode bech32");
    assert!(addr.starts_with("ltc1q"));
    assert!(addr.len() >= 40 && addr.len() <= 62);
  }

  #[test]
  fn trust_wallet_vector_matches_bip84() {
    let phrase = "lift find file planet grow remain steel rude arrive curve congress outer";
    let mnemonic = Mnemonic::parse(phrase).expect("valid mnemonic");
    let master = master_from_mnemonic_bip32(&mnemonic, "").expect("master key");
    let (account_xprv, _, _) = derive_account(&master, 2, 84).expect("account derivation");

    let addresses = receive_addresses(
      &account_xprv,
      LtcNetwork::Mainnet,
      2,
      84,
      LtcDerivationStyle::BIP84,
      1,
    )
    .expect("receive addresses");

    assert_eq!(
      addresses[0].1,
      "ltc1q0emsz6raagvzstd3tww0hz0wm3w50cpc52rvv7"
    );
  }
}
