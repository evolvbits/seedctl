//! Interactive prompts for the Polygon (MATIC/POL) chain crate.
//!
//! Thin wrappers around [`seedctl_core::evm`] prompt helpers bound to the
//! [`seedctl_core::evm::POLYGON_PROFILE`] so that the rest of the crate never
//! references the profile constant directly.

use std::error::Error;

use crate::utils::DerivationStyle;

/// Prompts the user to choose between generating addresses and scanning common
/// Polygon derivation paths.
///
/// Returns:
/// - `0` — generate addresses (continue to address-count / style prompts).
/// - `1` — scan common paths (prints a table then returns early).
pub fn select_derivation_mode() -> Result<usize, Box<dyn Error>> {
  seedctl_core::evm::select_derivation_mode(&seedctl_core::evm::POLYGON_PROFILE)
}

/// Prompts the user for the number of Polygon addresses to generate.
///
/// Defaults to `10` if the user presses Enter without typing a value.
pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  seedctl_core::evm::prompt_address_count(&seedctl_core::evm::POLYGON_PROFILE)
}

/// Prompts the user to choose a Polygon derivation style.
///
/// Delegates to [`crate::utils::select_derivation_style`] which in turn
/// calls [`seedctl_core::evm::select_derivation_style`] with the Polygon
/// profile.
pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  crate::utils::select_derivation_style()
}

/// Prompts the user for a Polygon RPC URL used for balance queries.
///
/// Returns an empty string immediately when
/// [`seedctl_core::constants::RPC_URL_ENABLE`] is `false` (the default),
/// keeping the tool fully offline-capable out of the box.
///
/// When enabled, the user may press Enter without typing anything to skip
/// the balance check for this session.
pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  seedctl_core::evm::prompt_rpc_url(&seedctl_core::evm::POLYGON_PROFILE)
}
