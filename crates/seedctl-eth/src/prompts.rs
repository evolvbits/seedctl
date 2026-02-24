use std::error::Error;

use crate::utils::DerivationStyle;

/// 0 = Generate addresses, 1 = Scan common derivation paths.
pub fn select_derivation_mode() -> Result<usize, Box<dyn Error>> {
  seedctl_core::evm::select_derivation_mode(&seedctl_core::evm::ETHEREUM_PROFILE)
}

pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  seedctl_core::evm::prompt_address_count(&seedctl_core::evm::ETHEREUM_PROFILE)
}

pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  crate::utils::select_derivation_style()
}

pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  seedctl_core::evm::prompt_rpc_url(&seedctl_core::evm::ETHEREUM_PROFILE)
}
