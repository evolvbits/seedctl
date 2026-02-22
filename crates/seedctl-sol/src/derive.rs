use ed25519_dalek::SigningKey;
use std::error::Error;

use crate::utils::{derive_seed, pubkey_to_address};

/// Gera keypair Ed25519 e endereço Solana (base58) para o seed BIP39 e índice dados.
/// Path: m/44'/501'/index'/0' (compatível com Phantom/Solana CLI).
pub fn keypair_and_address(
  seed: &[u8],
  index: u32,
) -> Result<(SigningKey, String), Box<dyn Error>> {
  let seed_32 = derive_seed(seed, index)?;
  let signing_key = SigningKey::from_bytes(&seed_32);
  let verifying = signing_key.verifying_key();
  let addr = pubkey_to_address(verifying.as_bytes());
  Ok((signing_key, addr))
}
