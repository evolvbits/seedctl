use bech32::{Bech32, Hrp};
use bip39::Mnemonic;
use cryptoxide::{blake2b::Blake2b, digest::Digest};
use ed25519_bip32::{DerivationIndex, DerivationScheme, XPrv, XPub};
use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha512;
use std::error::Error;

use crate::prompts::AdaNetwork;

const HARDENED: u32 = 0x8000_0000;
const PURPOSE_CIP1852: u32 = 1852;
const COIN_TYPE_ADA: u32 = 1815;

pub struct AdaAccount {
  pub account_xprv: XPrv,
  pub account_xpub: XPub,
  pub stake_xpub: XPub,
}

pub fn account_path(account: u32) -> String {
  format!("m/1852'/1815'/{}'", account)
}

pub fn payment_path(account: u32, index: u32) -> String {
  format!("m/1852'/1815'/{}'/0/{}", account, index)
}

pub fn master_from_mnemonic_icarus(mnemonic: &Mnemonic, passphrase: &str) -> XPrv {
  // CIP-0003 (Icarus): master key from entropy + passphrase via PBKDF2-HMAC-SHA512.
  let entropy = mnemonic.to_entropy();
  let mut data = pbkdf2_hmac_array::<Sha512, 96>(passphrase.as_bytes(), &entropy, 4096);

  data[0] &= 0b1111_1000;
  data[31] &= 0b0001_1111;
  data[31] |= 0b0100_0000;

  let mut extended = [0u8; 64];
  extended.copy_from_slice(&data[..64]);

  let mut chain_code = [0u8; 32];
  chain_code.copy_from_slice(&data[64..]);

  XPrv::from_extended_and_chaincode(&extended, &chain_code)
}

pub fn derive_account(master: &XPrv, account: u32) -> AdaAccount {
  let account_xprv = derive_path(
    master.clone(),
    &[
      hardened(PURPOSE_CIP1852),
      hardened(COIN_TYPE_ADA),
      hardened(account),
    ],
  );

  let account_xpub = account_xprv.public();
  let stake_xprv = derive_path(account_xprv.clone(), &[2, 0]);
  let stake_xpub = stake_xprv.public();

  AdaAccount {
    account_xprv,
    account_xpub,
    stake_xpub,
  }
}

pub fn derive_payment_xprv(account_xprv: &XPrv, index: u32) -> XPrv {
  derive_path(account_xprv.clone(), &[0, index])
}

pub fn address_from_payment_key(
  payment_xpub: &XPub,
  stake_xpub: &XPub,
  network: AdaNetwork,
) -> Result<String, Box<dyn Error>> {
  let payment_hash = blake2b224(&payment_xpub.public_key());
  let stake_hash = blake2b224(&stake_xpub.public_key());

  // Shelley base address: header(4-bit type + 4-bit network) + payment_hash + stake_hash.
  let header = network.base_header();
  let mut bytes = Vec::with_capacity(1 + 28 + 28);
  bytes.push(header);
  bytes.extend_from_slice(&payment_hash);
  bytes.extend_from_slice(&stake_hash);

  Ok(bech32::encode::<Bech32>(
    Hrp::parse(network.hrp())?,
    &bytes,
  )?)
}

pub fn keypair_and_address(
  account: &AdaAccount,
  index: u32,
  network: AdaNetwork,
) -> Result<(XPrv, String), Box<dyn Error>> {
  let payment_xprv = derive_payment_xprv(&account.account_xprv, index);
  let payment_xpub = payment_xprv.public();
  let address = address_from_payment_key(&payment_xpub, &account.stake_xpub, network)?;

  Ok((payment_xprv, address))
}

fn derive_path(mut key: XPrv, segments: &[DerivationIndex]) -> XPrv {
  for segment in segments {
    key = key.derive(DerivationScheme::V2, *segment);
  }

  key
}

fn hardened(index: u32) -> DerivationIndex {
  index | HARDENED
}

fn blake2b224(data: &[u8]) -> [u8; 28] {
  let mut hasher = Blake2b::new(28);
  hasher.input(data);

  let mut out = [0u8; 28];
  hasher.result(&mut out);
  out
}
