use bip32::XPrv;
use k256::ecdsa::SigningKey;
use sha3::{Digest, Keccak256};
use std::error::Error;

use crate::utils::to_tron_address;

/// Deriva endereço Tron (base58check, prefixo T) a partir de XPrv.
pub fn address_from_xprv(xprv: XPrv) -> Result<String, Box<dyn Error>> {
  let pk = xprv.private_key().to_bytes();
  let signing = SigningKey::from_bytes(&pk)?;
  let pubkey = signing.verifying_key().to_encoded_point(false);
  let hash = Keccak256::digest(&pubkey.as_bytes()[1..]);
  let addr_20 = &hash[12..];
  Ok(to_tron_address(addr_20))
}
