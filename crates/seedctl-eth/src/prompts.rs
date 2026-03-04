//! Interactive prompts for the Ethereum (ETH) chain crate.
//!
//! Thin wrappers around [`seedctl_core::evm`] prompt helpers bound to the
//! [`seedctl_core::evm::ETHEREUM_PROFILE`] so that the rest of the crate never
//! references the profile constant directly.
//!
//! Each function delegates to the shared EVM prompt logic in
//! [`seedctl_core::evm`], which reads the prompt strings and default values
//! from the profile, keeping all chain-specific text in one place.

use std::error::Error;

use crate::utils::DerivationStyle;

/// Prompts the user to choose between generating addresses and scanning common
/// Ethereum derivation paths.
///
/// Delegates to [`seedctl_core::evm::select_derivation_mode`] with the
/// [`seedctl_core::evm::ETHEREUM_PROFILE`].
///
/// # Returns
///
/// - `0` — generate addresses (continue to address-count / style prompts).
/// - `1` — scan common paths (prints a formatted table then returns early from
///   [`crate::run`]).
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn select_derivation_mode() -> Result<usize, Box<dyn Error>> {
  seedctl_core::evm::select_derivation_mode(&seedctl_core::evm::ETHEREUM_PROFILE)
}

/// Prompts the user for the number of Ethereum addresses to generate.
///
/// Delegates to [`seedctl_core::evm::prompt_address_count`] with the
/// [`seedctl_core::evm::ETHEREUM_PROFILE`].
///
/// Defaults to `10` if the user presses Enter without typing a value.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  seedctl_core::evm::prompt_address_count(&seedctl_core::evm::ETHEREUM_PROFILE)
}

/// Prompts the user to choose an Ethereum BIP-32 derivation style.
///
/// Delegates to [`crate::utils::select_derivation_style`] which in turn calls
/// [`seedctl_core::evm::select_derivation_style`] with the
/// [`seedctl_core::evm::ETHEREUM_PROFILE`].
///
/// Available options:
/// - `Standard (m/44'/60'/0'/0/x)` — MetaMask-compatible (default).
/// - `Ledger style`                 — Ledger Ethereum app layout.
/// - `Custom path`                  — user-supplied base path string.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  crate::utils::select_derivation_style()
}

/// Prompts the user for an Ethereum JSON-RPC URL used for `eth_getBalance`
/// queries.
///
/// Delegates to [`seedctl_core::evm::prompt_rpc_url`] with the
/// [`seedctl_core::evm::ETHEREUM_PROFILE`].
///
/// Returns an empty string immediately when
/// [`seedctl_core::constants::RPC_URL_ENABLE`] is `false` (the default),
/// keeping the tool fully offline-capable out of the box.
///
/// When enabled, the user may press Enter without typing anything to skip
/// the balance check for this session.
///
/// Compatible endpoints (EIP-1474 JSON-RPC):
/// - `https://cloudflare-eth.com/v1/mainnet`
/// - `https://mainnet.infura.io/v3/<api_key>`
/// - Local Geth / Reth node: `http://127.0.0.1:8545`
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  seedctl_core::evm::prompt_rpc_url(&seedctl_core::evm::ETHEREUM_PROFILE)
}
