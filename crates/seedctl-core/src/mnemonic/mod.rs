//! BIP-39 mnemonic generation and validation.
//!
//! Provides [`MnemonicGenerator`], a thin wrapper around the `bip39` crate
//! that centralises mnemonic construction and parsing so all chain crates
//! share a single, well-documented entry point.

use bip39::Mnemonic;

use crate::error::SeedCtlError;

/// Stateless factory for BIP-39 mnemonics.
///
/// All methods are associated functions (no `self`) so callers never need
/// to instantiate the struct — it acts purely as a namespace.
pub struct MnemonicGenerator;

impl MnemonicGenerator {
  /// Builds a [`Mnemonic`] from raw entropy bytes.
  ///
  /// The length of `entropy` determines the mnemonic word count:
  ///
  /// | Entropy bytes | Word count | Entropy bits |
  /// |:---:|:---:|:---:|
  /// | 16 | 12 | 128 |
  /// | 32 | 24 | 256 |
  ///
  /// # Errors
  ///
  /// Returns [`SeedCtlError::CryptoError`] if the entropy length is not one
  /// of the values supported by BIP-39 (multiples of 4 bytes, 16–32 bytes).
  pub fn from_entropy(entropy: &[u8]) -> Result<Mnemonic, SeedCtlError> {
    Mnemonic::from_entropy(entropy).map_err(|_| SeedCtlError::CryptoError)
  }

  /// Parses and validates a user-supplied BIP-39 phrase.
  ///
  /// Leading/trailing whitespace is stripped and internal whitespace is
  /// normalised before validation, so phrases copied from documents or
  /// typed with extra spaces are handled gracefully.
  ///
  /// # Errors
  ///
  /// Returns [`SeedCtlError::CryptoError`] if any word in the phrase is not
  /// in the BIP-39 English word list, or if the embedded checksum is invalid.
  pub fn parse(phrase: &str) -> Result<Mnemonic, SeedCtlError> {
    Mnemonic::parse_normalized(phrase.trim()).map_err(|_| SeedCtlError::CryptoError)
  }
}
