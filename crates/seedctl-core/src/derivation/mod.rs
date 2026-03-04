//! BIP-32 derivation path utilities and wallet generator abstraction.
//!
//! Provides helpers for parsing and validating BIP-32 derivation path strings,
//! converting them to [`bip32::DerivationPath`] values, and the
//! [`WalletGenerator`] trait for chain-specific key derivation.

use crate::error::SeedCtlError;
use bip32::DerivationPath;

/// Parses a BIP-32 derivation path string into a list of `(index, hardened)` pairs.
///
/// The path must start with `"m/"` followed by slash-separated segments.
/// Each segment is an unsigned integer optionally suffixed with `'` to indicate
/// a hardened child derivation step.
///
/// # Examples
///
/// ```
/// use seedctl_core::derivation::parse_path;
///
/// let segs = parse_path("m/84'/0'/0'").unwrap();
/// assert_eq!(segs, vec![(84, true), (0, true), (0, true)]);
///
/// let segs2 = parse_path("m/44'/60'/0'/0/5").unwrap();
/// assert_eq!(segs2, vec![(44, true), (60, true), (0, true), (0, false), (5, false)]);
/// ```
///
/// # Errors
///
/// Returns [`SeedCtlError::InvalidPath`] if the path does not start with `"m/"`
/// or if any segment cannot be parsed as a `u32`.
pub fn parse_path(path: &str) -> Result<Vec<(u32, bool)>, SeedCtlError> {
  if !path.starts_with("m/") {
    return Err(SeedCtlError::InvalidPath);
  }

  path[2..]
    .split('/')
    .map(|p| {
      let hardened = p.ends_with('\'');
      let num = p
        .trim_end_matches('\'')
        .parse::<u32>()
        .map_err(|_| SeedCtlError::InvalidPath)?;

      Ok((num, hardened))
    })
    .collect()
}

/// Validates a BIP-32 path string and converts it to a [`bip32::DerivationPath`].
///
/// Internally calls [`parse_path`] to validate the structure and component
/// values, then delegates to the `bip32` crate's own parser for the final
/// conversion.
///
/// # Errors
///
/// Returns [`SeedCtlError::InvalidPath`] if:
/// - [`parse_path`] rejects the string (missing `"m/"` prefix or non-numeric segment).
/// - The `bip32` crate's parser rejects the string for any other reason.
pub fn bip32_from_path(path: &str) -> Result<DerivationPath, SeedCtlError> {
  // Validate structure and numeric components first.
  let _ = parse_path(path)?;
  // Delegate the final conversion to the bip32 crate's own parser.
  path.parse().map_err(|_| SeedCtlError::InvalidPath)
}

/// Trait for types that can derive a chain-specific output (key, wallet, address)
/// from a raw BIP-39 seed and a BIP-32 derivation path string.
///
/// Implement this trait in a chain crate to expose a uniform derivation entry
/// point that the higher-level CLI or test harness can call without knowing the
/// concrete cryptographic types involved.
///
/// # Type Parameters
///
/// - `Output` — the type produced by a successful derivation (e.g. an extended
///   private key, a wallet struct, or a plain address string).
pub trait WalletGenerator {
  /// The type produced by a successful derivation.
  type Output;

  /// Derives `Output` from `seed` (64-byte BIP-39 seed) following `path`.
  ///
  /// # Parameters
  ///
  /// - `seed` — 64-byte array produced by `Mnemonic::to_seed(passphrase)`.
  /// - `path` — BIP-32 derivation path string, e.g. `"m/84'/0'/0'"`.
  ///
  /// # Errors
  ///
  /// Returns [`SeedCtlError`] if the path is invalid or any cryptographic
  /// operation during derivation fails.
  fn derive(seed: &[u8], path: &str) -> Result<Self::Output, SeedCtlError>;
}
