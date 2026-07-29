//! Cross-platform path construction helpers for `seedctl`.
//!
//! Prefer [`crate::utils::user_profile_path`] for new code.

/// Builds a [`std::path::PathBuf`] rooted at the current user's home directory.
///
/// Deprecated compatibility wrapper around [`crate::utils::user_profile_path`].
///
/// # Examples
///
/// ```rust
/// use seedctl_core::userprofile;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Resolves to e.g. "/home/alice/wallet-btc-abc1234-watch-only.json"
/// let path = userprofile!("wallet-btc-abc1234-watch-only.json")?;
///
/// // Resolves to e.g. "/home/alice/seedctl/exports/wallet.json"
/// let nested = userprofile!("seedctl", "exports", "wallet.json")?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! userprofile {
  ($($part:expr),*) => {
    $crate::utils::user_profile_path([$($part),*])
  };
}
