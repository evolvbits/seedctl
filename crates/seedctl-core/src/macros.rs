//! Cross-platform path construction macros for `seedctl`.
//!
//! Provides the [`userprofile!`] macro that resolves the current user's home
//! directory on both Unix (`$HOME`) and Windows (`%USERPROFILE%`) and appends
//! one or more path components to it.

/// Builds a [`std::path::PathBuf`] rooted at the current user's home directory.
///
/// Accepts one or more string expressions that are joined with the
/// platform-native separator (`/` on Unix, `\` on Windows) and appended to the
/// home directory path.
///
/// # Panics
///
/// Panics if the `HOME` (Unix) or `USERPROFILE` (Windows) environment variable
/// is not set.
///
/// # Examples
///
/// ```rust
/// use seedctl_core::userprofile;
///
/// // Resolves to e.g. "/home/alice/wallet-btc-abc1234-watch-only.json"
/// let path = userprofile!("wallet-btc-abc1234-watch-only.json");
///
/// // Resolves to e.g. "/home/alice/seedctl/exports/wallet.json"
/// let nested = userprofile!("seedctl", "exports", "wallet.json");
/// ```
#[macro_export]
macro_rules! userprofile {
  ($($part:expr),*) => {
    {
      let base_path = if cfg!(windows) {
          std::path::PathBuf::from(std::env::var("USERPROFILE").unwrap())
      } else {
          std::path::PathBuf::from(std::env::var("HOME").unwrap())
      };
      let path_parts = vec![$($part),*];
      let separator = if cfg!(windows) { r"\" } else { "/" };
      let path_str = path_parts.join(separator);
      base_path.join(path_str)
    }
  };
}
