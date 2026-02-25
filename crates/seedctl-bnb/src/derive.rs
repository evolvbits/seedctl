use bip32::XPrv;
use std::error::Error;

pub fn address_from_xprv(xprv: XPrv) -> Result<String, Box<dyn Error>> {
  seedctl_core::evm::address_from_xprv(xprv)
}
