use bip32::XPrv;
use std::error::Error;

pub fn scan_common_paths(master: XPrv) -> Result<(), Box<dyn Error>> {
  seedctl_core::evm::scan_common_paths(master, &seedctl_core::evm::POLYGON_PROFILE)
}
