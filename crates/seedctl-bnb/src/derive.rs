//! BNB Smart Chain address derivation from an EVM `XPrv`.
//!
//! Thin wrapper around [`seedctl_core::evm::address_from_xprv`] so that the
//! BNB crate never duplicates EVM address-encoding logic.

use bip32::XPrv;
use std::error::Error;

/// Derives a BNB Smart Chain (EVM-compatible) address from an extended private key.
///
/// Produces an EIP-55 checksum-encoded `0x…` address identical to the
/// Ethereum address for the same key — BNB Smart Chain shares the secp256k1
/// curve and Keccak-256 hashing scheme with Ethereum.
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
