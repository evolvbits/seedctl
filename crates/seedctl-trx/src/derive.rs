//! Tron (TRX) address derivation from a BIP-32 `XPrv`.
//!
//! Produces Base58Check-encoded Tron addresses (starting with `T`) from a
//! leaf extended private key by applying secp256k1 signing, Keccak-256
//! hashing, and the Tron-specific `0x41` version byte prefix.

use bip32::XPrv;
use k256::ecdsa::SigningKey;
use sha3::{Digest, Keccak256};
use std::error::Error;

use crate::utils::to_tron_address;

/// Derives a Tron address from an extended private key.
///
/// # Algorithm
///
/// 1. Extract the 32-byte private scalar from `xprv`.
/// 2. Derive the uncompressed secp256k1 public key (65 bytes).
/// 3. Apply Keccak-256 to the last 64 bytes (public key without the `0x04`
///    prefix).
/// 4. Take the last 20 bytes of the hash as the raw account ID.
/// 5. Prepend the Tron version byte `0x41` and Base58Check-encode the
///    resulting 21-byte payload to produce the final `T…` address.
///
/// # Parameters
///
/// - `xprv` — leaf extended private key at the full derivation path
///   (e.g. `m/44'/195'/0'/0/0`).
///
/// # Returns
///
/// A Base58Check-encoded Tron address string (44 characters, starting with
/// `T` on Mainnet).
///
/// # Errors
///
/// Returns a boxed error if the private-key bytes cannot be loaded into a
/// [`k256::ecdsa::SigningKey`], which should never happen for keys produced
/// by the `bip32` crate.
pub fn address_from_xprv(xprv: XPrv) -> Result<String, Box<dyn Error>> {
  let pk = xprv.private_key().to_bytes();
  let signing = SigningKey::from_bytes(&pk)?;
  let pubkey = signing.verifying_key().to_encoded_point(false);

  // Skip the 0x04 uncompressed prefix byte before hashing.
  let hash = Keccak256::digest(&pubkey.as_bytes()[1..]);

  // The Tron account ID is the last 20 bytes of the Keccak-256 hash.
  let addr_20 = &hash[12..];

  Ok(to_tron_address(addr_20))
}
