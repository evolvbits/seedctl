//! Solana key derivation using BIP-32 SLIP-0010 with Ed25519.
//!
//! Provides [`keypair_and_address`] which derives a SLIP-0010 Ed25519 signing
//! key from a BIP-39 seed for the given account index, and encodes the
//! corresponding verifying key as a base58 Solana address.

use ed25519_dalek::SigningKey;
use std::error::Error;

use crate::utils::{derive_seed_from_path, pubkey_to_address};

/// Derives an Ed25519 signing key and the corresponding Solana address for
/// the given BIP-39 seed and derivation path.
///
/// # Parameters
///
/// - `seed`  — 64-byte BIP-39 seed produced by `Mnemonic::to_seed(passphrase)`.
/// - `path`  — full derivation path, e.g. `"m/44'/501'/0'/0'"`.
///
/// # Returns
///
/// A tuple of `(SigningKey, base58_address)` where:
/// - `SigningKey` is the 32-byte Ed25519 private key for the derived account.
/// - `base58_address` is the base58-encoded 32-byte Ed25519 verifying (public)
///   key, which serves as the Solana wallet address.
///
/// # Errors
///
/// Returns a boxed error if the SLIP-0010 path derivation fails (e.g. the path
/// string is malformed), which should never happen given the hardcoded path
/// template.
pub fn keypair_and_address(
  seed: &[u8],
  path: &str,
) -> Result<(SigningKey, String), Box<dyn Error>> {
  let seed_32 = derive_seed_from_path(seed, path)?;
  let signing_key = SigningKey::from_bytes(&seed_32);
  let verifying = signing_key.verifying_key();
  let addr = pubkey_to_address(verifying.as_bytes());
  Ok((signing_key, addr))
}
