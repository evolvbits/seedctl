//! Ethereum address derivation from an EVM `XPrv`.
//!
//! Thin wrapper around [`seedctl_core::evm::address_from_xprv`] so that the
//! Ethereum crate never duplicates EVM address-encoding logic.

use bip32::XPrv;
use std::error::Error;

/// Derives an Ethereum (EVM-compatible) address from an extended private key.
///
/// Produces an EIP-55 checksum-encoded `0x…` address by:
///
/// 1. Extracting the 32-byte private scalar from `xprv`.
/// 2. Deriving the uncompressed secp256k1 public key (65 bytes).
/// 3. Applying Keccak-256 to the last 64 bytes (public key without the `0x04`
///    prefix) and taking the last 20 bytes as the account ID.
/// 4. Applying EIP-55 mixed-case checksum encoding to the 40-hex-char address.
///
/// # Parameters
///
/// - `xprv` — leaf extended private key at the full derivation path
///   (e.g. `m/44'/60'/0'/0/0`).
///
/// # Errors
///
/// Returns a boxed error if the private-key bytes cannot be loaded into a
/// [`k256::ecdsa::SigningKey`], which should never happen for keys produced
/// by the `bip32` crate.
pub fn address_from_xprv(xprv: XPrv) -> Result<String, Box<dyn Error>> {
  seedctl_core::evm::address_from_xprv(xprv)
}
