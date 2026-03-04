//! XRP Ledger address derivation from a BIP-32 `XPrv`.
//!
//! Produces classic XRPL addresses (starting with `r`) from a leaf extended
//! private key by applying secp256k1 signing, SHA-256 → RIPEMD-160 hashing,
//! and Base58Check encoding with the XRPL-specific alphabet.

use bip32::XPrv;
use k256::ecdsa::SigningKey;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use std::error::Error;

/// XRPL classic address version byte prepended before Base58Check encoding.
///
/// The value `0x00` identifies a standard XRPL account address (as opposed to
/// node public keys or other XRPL data types that use different prefixes).
const XRPL_CLASSIC_VERSION: u8 = 0x00;

/// The XRPL Base58 alphabet used for address encoding.
///
/// XRPL uses a custom alphabet that differs from Bitcoin's Base58Check
/// alphabet. The alphabet is identical to the one specified in the
/// XRPL documentation.
const XRPL_ALPHABET: &[u8; 58] = b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz";

/// Derives an XRPL classic address from a BIP-32 extended private key.
///
/// # Algorithm
///
/// 1. Extract the 32-byte secp256k1 private scalar from `xprv`.
/// 2. Derive the compressed 33-byte public key.
/// 3. Compute `hash160` (SHA-256 followed by RIPEMD-160) over the public key.
/// 4. Prepend the XRPL version byte (`0x00`) to the 20-byte account ID.
/// 5. Compute the 4-byte checksum via double SHA-256 and append it.
/// 6. Base58-encode the full payload using the XRPL alphabet.
///
/// # Parameters
///
/// - `xprv` — leaf extended private key at the full derivation path
///   (e.g. `m/44'/144'/0'/0/0`).
///
/// # Returns
///
/// A Base58-encoded XRPL classic address string starting with `r`.
///
/// # Errors
///
/// Returns a boxed error if the private-key bytes cannot be loaded into a
/// [`k256::ecdsa::SigningKey`], which should never happen for keys produced
/// by the `bip32` crate.
pub fn address_from_xprv(xprv: XPrv) -> Result<String, Box<dyn Error>> {
  let private = xprv.private_key().to_bytes();
  let signing = SigningKey::from_bytes(&private)?;
  // Use the compressed public key (33 bytes) for address derivation.
  let pubkey = signing.verifying_key().to_encoded_point(true);

  Ok(classic_address_from_pubkey(pubkey.as_bytes()))
}

/// Encodes a compressed secp256k1 public key as an XRPL classic address.
///
/// Applies `hash160` to the public key bytes to produce the 20-byte account
/// ID, then constructs the full Base58Check payload using the XRPL alphabet.
///
/// # Parameters
///
/// - `pubkey_compressed` — 33-byte compressed SEC public key.
///
/// # Returns
///
/// A Base58-encoded XRPL classic address string starting with `r`.
fn classic_address_from_pubkey(pubkey_compressed: &[u8]) -> String {
  // Derive the 20-byte account ID via SHA-256 + RIPEMD-160.
  let account_id = hash160(pubkey_compressed);

  // Prepend the XRPL version byte to the account ID.
  let mut payload = Vec::with_capacity(1 + account_id.len());
  payload.push(XRPL_CLASSIC_VERSION);
  payload.extend_from_slice(&account_id);

  // Compute the 4-byte checksum (first 4 bytes of double SHA-256).
  let checksum = double_sha256(&payload);

  // Append the checksum to form the final Base58Check payload.
  let mut full = payload;
  full.extend_from_slice(&checksum[..4]);

  // Encode with the XRPL-specific Base58 alphabet.
  let alphabet = bs58::Alphabet::new(XRPL_ALPHABET).expect("valid XRPL alphabet");
  bs58::encode(full).with_alphabet(&alphabet).into_string()
}

/// Computes SHA-256 followed by RIPEMD-160 over `data`, returning a 20-byte
/// account ID.
///
/// This is the standard `hash160` (also called `HASH160` in Bitcoin script
/// terminology) used by both Bitcoin and XRPL for public-key hashing.
///
/// # Parameters
///
/// - `data` — byte slice to hash (typically a compressed public key).
///
/// # Returns
///
/// A 20-byte array containing the RIPEMD-160 digest of the SHA-256 digest
/// of `data`.
fn hash160(data: &[u8]) -> [u8; 20] {
  // First pass: SHA-256.
  let sha = Sha256::digest(data);
  // Second pass: RIPEMD-160 over the SHA-256 output.
  let rip = Ripemd160::digest(sha);

  let mut out = [0u8; 20];
  out.copy_from_slice(&rip);
  out
}

/// Computes double SHA-256 (SHA-256 applied twice) over `data`, returning a
/// 32-byte digest.
///
/// Used to compute the 4-byte checksum appended at the end of the Base58Check
/// payload. Only the first 4 bytes of the result are used as the checksum.
///
/// # Parameters
///
/// - `data` — byte slice to hash (typically the versioned payload without the
///   checksum).
///
/// # Returns
///
/// A 32-byte array containing `SHA256(SHA256(data))`.
fn double_sha256(data: &[u8]) -> [u8; 32] {
  // First SHA-256 pass.
  let first = Sha256::digest(data);
  // Second SHA-256 pass over the first digest.
  let second = Sha256::digest(first);

  let mut out = [0u8; 32];
  out.copy_from_slice(&second);
  out
}
