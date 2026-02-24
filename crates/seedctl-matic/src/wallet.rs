use seedctl_core::export;
use std::error::Error;

pub fn build_export(
  info: &[&str],
  base_path: &str,
  xpub: seedctl_core::evm::EvmAccountXpub,
) -> Result<export::WalletExport, Box<dyn Error>> {
  Ok(seedctl_core::evm::build_watch_only_export(
    &seedctl_core::evm::POLYGON_PROFILE,
    info,
    base_path,
    xpub,
  ))
}
