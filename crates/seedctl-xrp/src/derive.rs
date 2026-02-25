use bip32::XPrv;
use k256::ecdsa::SigningKey;
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use std::error::Error;

const XRPL_CLASSIC_VERSION: u8 = 0x00;
const XRPL_ALPHABET: &[u8; 58] = b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz";

pub fn address_from_xprv(xprv: XPrv) -> Result<String, Box<dyn Error>> {
  let private = xprv.private_key().to_bytes();
  let signing = SigningKey::from_bytes(&private)?;
  let pubkey = signing.verifying_key().to_encoded_point(true);

  Ok(classic_address_from_pubkey(pubkey.as_bytes()))
}

fn classic_address_from_pubkey(pubkey_compressed: &[u8]) -> String {
  let account_id = hash160(pubkey_compressed);

  let mut payload = Vec::with_capacity(1 + account_id.len());
  payload.push(XRPL_CLASSIC_VERSION);
  payload.extend_from_slice(&account_id);

  let checksum = double_sha256(&payload);

  let mut full = payload;
  full.extend_from_slice(&checksum[..4]);

  let alphabet = bs58::Alphabet::new(XRPL_ALPHABET).expect("valid XRPL alphabet");
  bs58::encode(full).with_alphabet(&alphabet).into_string()
}

fn hash160(data: &[u8]) -> [u8; 20] {
  let sha = Sha256::digest(data);
  let rip = Ripemd160::digest(sha);

  let mut out = [0u8; 20];
  out.copy_from_slice(&rip);
  out
}

fn double_sha256(data: &[u8]) -> [u8; 32] {
  let first = Sha256::digest(data);
  let second = Sha256::digest(first);

  let mut out = [0u8; 32];
  out.copy_from_slice(&second);
  out
}
