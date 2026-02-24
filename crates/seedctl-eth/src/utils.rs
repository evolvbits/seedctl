use bip32::XPrv;
use std::error::Error;

pub use seedctl_core::evm::DerivationStyle;

pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  seedctl_core::evm::select_derivation_style(&seedctl_core::evm::ETHEREUM_PROFILE)
}

pub fn style_to_string(style: &DerivationStyle) -> String {
  seedctl_core::evm::style_to_string(style)
}

pub fn derive_from_path(master: XPrv, path: &str) -> Result<XPrv, Box<dyn Error>> {
  seedctl_core::evm::derive_from_path(master, path)
}

pub fn derive_address_key(
  master: &XPrv,
  account_xprv: &XPrv,
  style: &DerivationStyle,
  index: u32,
) -> Result<(XPrv, String), Box<dyn Error>> {
  seedctl_core::evm::derive_address_key(master, account_xprv, style, index)
}
