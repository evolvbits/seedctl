//! Chain-agnostic derivation trait and shared derivation context.
//!
//! Defines the [`Chain`] trait that each chain crate can implement to expose
//! a uniform address-derivation interface, and the [`ChainContext`] helper
//! that bundles the common parameters required for any derivation call.

use crate::error::SeedCtlError;

/// Core abstraction for a BIP-39 / BIP-32 compatible blockchain.
///
/// Implement this trait in a chain crate to expose address derivation through
/// a consistent interface, independent of the underlying cryptographic library
/// used by that chain.
pub trait Chain {
  /// Human-readable chain name, e.g. `"Bitcoin (BTC)"`.
  fn name(&self) -> &'static str;

  /// SLIP-44 coin type for this chain, e.g. `0` for Bitcoin, `60` for Ethereum.
  fn coin_type(&self) -> u32;

  /// Derives a single address for the given account and address index.
  ///
  /// # Parameters
  ///
  /// - `mnemonic`   — BIP-39 mnemonic phrase (space-separated words).
  /// - `passphrase` — optional BIP-39 passphrase; `None` is treated as `""`.
  /// - `account`    — account index (hardened), typically `0`.
  /// - `index`      — address index (non-hardened) within the account.
  ///
  /// # Errors
  ///
  /// Returns [`SeedCtlError`] if the mnemonic is invalid, the derivation path
  /// is malformed, or any cryptographic operation fails.
  fn derive_address(
    &self,
    mnemonic: &str,
    passphrase: Option<&str>,
    account: u32,
    index: u32,
  ) -> Result<String, SeedCtlError>;
}

/// Shared context bundling the common parameters needed for any derivation call.
///
/// Passed to chain implementations to avoid repeating the same arguments
/// across every `derive_*` function signature.
pub struct ChainContext<'a> {
  /// BIP-39 mnemonic phrase (space-separated words).
  pub mnemonic: &'a str,

  /// Optional BIP-39 passphrase; `None` is treated as an empty string.
  pub passphrase: Option<&'a str>,

  /// Account index (hardened), typically `0`.
  pub account: u32,

  /// Address index (non-hardened) within the account.
  pub index: u32,
}
