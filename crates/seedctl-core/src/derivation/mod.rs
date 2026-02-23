use crate::error::SeedCtlError;
use bip32::DerivationPath;

/// Parses a BIP‑32 style path like `m/84'/0'/0'/0/0` into its numeric components.
/// Returns `(index, hardened)` for each segment.
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

/// Helper that validates and converts a BIP‑32 path string into a `bip32::DerivationPath`,
/// reusing `parse_path` for normalization/validation.
pub fn bip32_from_path(path: &str) -> Result<DerivationPath, SeedCtlError> {
  // Validate structure and components first.
  let _ = parse_path(path)?;
  // Then delegate to the bip32 parser; if this fails, treat as invalid path.
  path.parse().map_err(|_| SeedCtlError::InvalidPath)
}

/// Generic wallet/key derivation interface working on a BIP‑39 seed and path.
pub trait WalletGenerator {
  type Output;

  fn derive(seed: &[u8], path: &str) -> Result<Self::Output, SeedCtlError>;
}
