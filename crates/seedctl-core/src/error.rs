use std::fmt;

#[derive(Debug)]
pub enum SeedCtlError {
  InvalidPath,
  InvalidKey,
  CryptoError,
  IoError,
  Unsupported,
}

impl fmt::Display for SeedCtlError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::InvalidPath => write!(f, "invalid derivation path"),
      Self::InvalidKey => write!(f, "invalid key"),
      Self::CryptoError => write!(f, "cryptographic error"),
      Self::IoError => write!(f, "io error"),
      Self::Unsupported => write!(f, "unsupported feature"),
    }
  }
}

impl std::error::Error for SeedCtlError {}
