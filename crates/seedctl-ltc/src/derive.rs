//! Litecoin address derivation using BIP-84 (Native SegWit P2WPKH).
//!
//! Provides [`derive_account`] for deriving the account-level key pair from a
//! BIP-32 master key, and [`receive_addresses`] for generating a batch of
//! bech32-encoded Litecoin receive addresses (`ltc1…` on Mainnet, `tltc1…`
//! on Testnet).

use bech32::{Bech32, Hrp};
use bip32::{ChildNumber, DerivationPath, XPrv, XPub};
use ripemd::Ripemd160;
use sha2::{Digest as ShaDigest, Sha256};
use std::error::Error;

use crate::prompts::LtcNetwork;

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
) -> Result<(XPrv, XPub, [u8; 4]), Box<dyn Error>> {
  let account_path: DerivationPath = format!("m/84'/{}'/0'", coin_type).parse()?;
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
    let addr = ltc_bech32_p2wpkh(pubkey.as_bytes(), network)?;

    out.push((format!("m/84'/{}'/0'/0/{}", coin_type, i), addr));
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
/// program, prepends the SegWit version byte (`0x00`), and encodes the result
/// as a bech32 string using the network-appropriate HRP.
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

  // Prepend SegWit version byte 0x00 before the 20-byte key hash.
  let mut program = Vec::with_capacity(1 + rip.len());
  program.push(0u8);
  program.extend_from_slice(&rip);

  let hrp = match network {
    LtcNetwork::Mainnet => "ltc",
    LtcNetwork::Testnet => "tltc",
  };

  Ok(bech32::encode::<Bech32>(Hrp::parse(hrp)?, &program)?)
}
