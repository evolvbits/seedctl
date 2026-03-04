//! Error types for the `seedctl-core` library.
//!
//! All public APIs that can fail return either a [`SeedCtlError`] directly
//! or a `Box<dyn std::error::Error>` that may wrap one.

use std::fmt;

/// Enumeration of all domain errors that can occur inside `seedctl-core`.
#[derive(Debug)]
pub enum SeedCtlError {
  /// A BIP-32 derivation path string was malformed or contained invalid segments.
  InvalidPath,

  /// A cryptographic key was invalid, out of range, or could not be parsed.
  InvalidKey,

  /// A cryptographic operation failed (e.g. hashing, signing, or key derivation).
  CryptoError,

  /// An I/O operation failed (e.g. reading from stdin or writing a file).
  IoError,

  /// The requested feature is not supported for this chain or configuration.
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
