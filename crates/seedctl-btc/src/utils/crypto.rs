use bitcoin::{
  base58,
  bip32::{Xpriv, Xpub},
  hashes::{Hash, sha256d::Hash as Sha256dHash},
};

pub fn xprv_to_zprv(xprv: &Xpriv) -> String {
  let mut data = xprv.encode();
  data[0..4].copy_from_slice(&[0x04, 0xB2, 0x43, 0x0C]);
  base58::encode_check(&data)
}

pub fn xpub_to_zpub(xpub: &Xpub) -> String {
  let mut data = xpub.encode();
  data[0..4].copy_from_slice(&[0x04, 0xB2, 0x47, 0x46]);
  base58::encode_check(&data)
}

pub fn xprv_to_yprv(xprv: &Xpriv) -> String {
  let mut data = xprv.encode();
  // yprv prefix
  data[0..4].copy_from_slice(&[0x04, 0x9D, 0x78, 0x78]);
  base58::encode_check(&data)
}

/// Converte xpub → ypub / zpub (SLIP-132)
pub fn convert_xpub_prefix(xpub: &Xpub, version: u32) -> String {
  // Decode Base58Check
  let mut data = base58::decode_check(&xpub.to_string()).expect("Invalid Base58Check xpub");

  // Substitui version bytes
  data[0..4].copy_from_slice(&version.to_be_bytes());

  // Recalcula checksum
  let checksum: Sha256dHash = Hash::hash(&data[..data.len() - 4]);

  let len = data.len();
  data[len - 4..len].copy_from_slice(&checksum[..4]);

  // Encode Base58Check
  base58::encode_check(&data)
}

pub fn xpub_to_ypub(xpub: &Xpub) -> String {
  convert_xpub_prefix(xpub, 0x049d7cb2) // ypub
}
