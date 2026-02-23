use bip39::Mnemonic;

use crate::error::SeedCtlError;

// TODO: Nao USADO
// Simple wrapper responsible for generating/validating BIP39 mnemonics.
pub struct MnemonicGenerator;

impl MnemonicGenerator {
  /// Build a mnemonic from raw entropy bytes.
  pub fn from_entropy(entropy: &[u8]) -> Result<Mnemonic, SeedCtlError> {
    Mnemonic::from_entropy(entropy).map_err(|_| SeedCtlError::CryptoError)
  }

  /// Parse a user-provided phrase, normalizing whitespace.
  pub fn parse(phrase: &str) -> Result<Mnemonic, SeedCtlError> {
    Mnemonic::parse_normalized(phrase.trim()).map_err(|_| SeedCtlError::CryptoError)
  }
}
