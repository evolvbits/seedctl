//! Build-time metadata constants for the `seedctl` binary.
//!
//! All constants are populated at compile time via `env!` macros, sourcing
//! values from `Cargo.toml` fields and from environment variables injected
//! by `build.rs` (git commit hash, build date, build profile, copyright year,
//! and maintainer name).
//!
//! Import this module wherever you need to display version information,
//! copyright notices, or repository links.

/// The crate / binary name as declared in `Cargo.toml` (`name` field).
///
/// Example: `"seedctl"`
pub const PROJECT_NAME: &str = env!("CARGO_PKG_NAME");

/// The crate version as declared in `Cargo.toml` (`version` field).
///
/// Example: `"0.2.1"`
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The repository URL as declared in `Cargo.toml` (`repository` field).
///
/// Example: `"https://github.com/yourname/seedctl"`
pub const PROJECT_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

/// The short description as declared in `Cargo.toml` (`description` field).
pub const PROJECT_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

/// The copyright year injected by `build.rs` from `package.metadata.copyright_year`.
///
/// Example: `"2026"`
pub const COPYRIGHT_YEAR: &str = env!("COPYRIGHT_YEAR");

/// The maintainer name injected by `build.rs` from `package.metadata.maintainer`.
///
/// Example: `"William C. Canin"`
pub const PROJECT_MAINTAINER: &str = env!("PROJECT_MAINTAINER");

/// The short Git commit hash injected by `build.rs` at build time.
///
/// Example: `"abc1234"` or `"unknown"` when the repository has no commits.
pub const GIT_COMMIT: &str = env!("GIT_COMMIT");

/// The date of the HEAD commit injected by `build.rs` at build time.
///
/// Format: `YYYY-MM-DD`.  Example: `"2025-06-15"` or `"unknown"`.
pub const GIT_DATE: &str = env!("GIT_DATE");

/// The Cargo build profile injected by `build.rs` at build time.
///
/// Typical values: `"debug"` or `"release"`.
pub const BUILD_PROFILE: &str = env!("BUILD_PROFILE");
