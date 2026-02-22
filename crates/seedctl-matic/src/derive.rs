use bip32::XPrv;
use k256::ecdsa::SigningKey;
use sha3::{Digest, Keccak256};
use std::error::Error;

pub fn address_from_xprv(xprv: XPrv) -> Result<String, Box<dyn Error>> {
  let pk = xprv.private_key().to_bytes();

  let signing = SigningKey::from_bytes(&pk)?;
  let pubkey = signing.verifying_key().to_encoded_point(false);

  let hash = Keccak256::digest(&pubkey.as_bytes()[1..]);
  let addr_bytes = &hash[12..];

  Ok(crate::utils::to_checksum_address(addr_bytes))
}
