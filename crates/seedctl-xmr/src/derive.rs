use curve25519_dalek::{constants::ED25519_BASEPOINT_TABLE, edwards::EdwardsPoint, scalar::Scalar};
use sha3::{Digest, Keccak256};

use crate::prompts::XmrNetwork;

const MONERO_B58_ALPHABET: &[u8; 58] =
  b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const ENCODED_BLOCK_SIZES: [usize; 9] = [0, 2, 3, 5, 6, 7, 9, 10, 11];

pub struct XmrWallet {
  spend_private: Scalar,
  view_private: Scalar,
  spend_public: EdwardsPoint,
  view_public: EdwardsPoint,
}

pub struct DerivedAddress {
  pub path: String,
  pub address: String,
}

impl XmrWallet {
  pub fn spend_private_hex(&self) -> String {
    hex::encode(self.spend_private.to_bytes())
  }

  pub fn spend_public_hex(&self) -> String {
    hex::encode(self.spend_public.compress().to_bytes())
  }
}

pub fn wallet_from_bip39_seed(seed: &[u8]) -> XmrWallet {
  // Monero-compatible key flow:
  // spend = Hs(seed), view = Hs(spend), public = scalar * G.
  let spend_private = scalar_from_hash(seed);
  let view_private = scalar_from_hash(&spend_private.to_bytes());

  let spend_public = &spend_private * ED25519_BASEPOINT_TABLE;
  let view_public = &view_private * ED25519_BASEPOINT_TABLE;

  XmrWallet {
    spend_private,
    view_private,
    spend_public,
    view_public,
  }
}

pub fn derivation_path(index: u32) -> String {
  format!("xmr(major=0,minor={index})")
}

pub fn derive_address(wallet: &XmrWallet, network: XmrNetwork, index: u32) -> DerivedAddress {
  let address = if index == 0 {
    encode_address(
      network.standard_prefix(),
      &wallet.spend_public.compress().to_bytes(),
      &wallet.view_public.compress().to_bytes(),
    )
  } else {
    // Subaddress for account=0, minor=index.
    let m = subaddress_secret(&wallet.view_private, 0, index);
    let d = wallet.spend_public + (&m * ED25519_BASEPOINT_TABLE);
    let c = d * wallet.view_private;

    encode_address(
      network.subaddress_prefix(),
      &d.compress().to_bytes(),
      &c.compress().to_bytes(),
    )
  };

  DerivedAddress {
    path: derivation_path(index),
    address,
  }
}

fn subaddress_secret(view_private: &Scalar, major: u32, minor: u32) -> Scalar {
  let mut data = Vec::with_capacity(8 + 32 + 4 + 4);
  data.extend_from_slice(b"SubAddr\0");
  data.extend_from_slice(&view_private.to_bytes());
  data.extend_from_slice(&major.to_le_bytes());
  data.extend_from_slice(&minor.to_le_bytes());

  scalar_from_hash(&data)
}

fn encode_address(prefix: u8, spend_public: &[u8; 32], view_public: &[u8; 32]) -> String {
  let mut payload = Vec::with_capacity(1 + 32 + 32);
  payload.push(prefix);
  payload.extend_from_slice(spend_public);
  payload.extend_from_slice(view_public);

  let checksum = keccak256(&payload);
  payload.extend_from_slice(&checksum[..4]);

  monero_base58_encode(&payload)
}

fn scalar_from_hash(data: &[u8]) -> Scalar {
  let hash = keccak256(data);
  Scalar::from_bytes_mod_order(hash)
}

fn keccak256(data: &[u8]) -> [u8; 32] {
  let mut hasher = Keccak256::new();
  hasher.update(data);
  let out = hasher.finalize();

  let mut bytes = [0u8; 32];
  bytes.copy_from_slice(&out);
  bytes
}

fn monero_base58_encode(data: &[u8]) -> String {
  let mut out = String::new();

  for chunk in data.chunks(8) {
    let encoded_size = ENCODED_BLOCK_SIZES[chunk.len()];
    out.push_str(&encode_block(chunk, encoded_size));
  }

  out
}

fn encode_block(block: &[u8], encoded_size: usize) -> String {
  let mut num: u128 = 0;
  for &byte in block {
    num = (num << 8) | byte as u128;
  }

  let mut out = vec!['1'; encoded_size];
  let mut idx = encoded_size;

  while num > 0 {
    let rem = (num % 58) as usize;
    num /= 58;

    if idx == 0 {
      break;
    }

    idx -= 1;
    out[idx] = MONERO_B58_ALPHABET[rem] as char;
  }

  out.into_iter().collect()
}
