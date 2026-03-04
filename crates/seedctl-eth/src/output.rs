//! Ethereum wallet output rendering.
//!
//! Thin wrapper around [`seedctl_core::evm::print_wallet_output`] that binds
//! the [`seedctl_core::evm::ETHEREUM_PROFILE`] so the rest of the crate never
//! references the profile constant directly.

/// All data required to render an Ethereum wallet section to stdout.
///
/// Re-exported from [`seedctl_core::evm::WalletOutput`] so call sites in this
/// crate can use the local path `output::WalletOutput` without importing from
/// `seedctl_core` directly.
pub type WalletOutput<'a> = seedctl_core::evm::WalletOutput<'a>;

/// Renders the full Ethereum wallet section to stdout.
///
/// Delegates to [`seedctl_core::evm::print_wallet_output`] with the
/// [`seedctl_core::evm::ETHEREUM_PROFILE`], which supplies the wallet title,
/// export network identifier, and scan paths specific to Ethereum.
///
/// The private key is conditionally included based on
/// [`WalletOutput::show_privkeys`].
pub fn print_wallet_output(output: &WalletOutput<'_>) {
  seedctl_core::evm::print_wallet_output(&seedctl_core::evm::ETHEREUM_PROFILE, output);
}
