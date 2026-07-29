//! Offline EVM transaction signing.
//!
//! Accepts an unsigned EIP-2718 transaction hex payload, derives the selected
//! EVM private key from the user's BIP-39 seed, signs locally, and prints the
//! signed raw transaction hex.

use alloy::{
  consensus::{SignableTransaction, TxEnvelope, TypedTransaction},
  eips::eip2718::Encodable2718,
  signers::{SignerSync, local::PrivateKeySigner},
};
use bip39::Mnemonic;
use console::style;
use dialoguer::Input;
use seedctl_core::ui::{dialoguer_theme, prompt_passphrase};
use std::error::Error;

/// Runs the interactive offline EVM transaction signing workflow.
pub fn sign_offline(mnemonic: &Mnemonic) -> Result<(), Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let passphrase = prompt_passphrase()?;

  let derivation_path: String = Input::with_theme(&theme)
    .with_prompt("EVM derivation path")
    .default("m/44'/60'/0'/0/0".to_string())
    .interact_text()?;

  let chain_id: u64 = Input::with_theme(&theme)
    .with_prompt("EVM chain ID (Ethereum=1, BSC=56, Polygon=137)")
    .default(1)
    .interact_text()?;

  let unsigned_hex: String = Input::with_theme(&theme)
    .with_prompt("Unsigned transaction hex from Rabby")
    .interact_text()?;

  let signed_hex = sign_unsigned_hex(
    mnemonic,
    &passphrase,
    &derivation_path,
    chain_id,
    &unsigned_hex,
  )?;

  println!();
  println!("{}", style("SIGNED EVM TRANSACTION HEX:").bold().green());
  println!("{signed_hex}");

  Ok(())
}

/// Signs an unsigned EVM transaction hex using a key derived from `mnemonic`.
pub fn sign_unsigned_hex(
  mnemonic: &Mnemonic,
  passphrase: &str,
  derivation_path: &str,
  chain_id: u64,
  unsigned_hex: &str,
) -> Result<String, Box<dyn Error>> {
  let master = seedctl_core::utils::master_from_mnemonic(mnemonic, passphrase)?;
  let leaf = crate::utils::derive_from_path(master, derivation_path)?;
  sign_unsigned_hex_with_key(&leaf.to_bytes(), chain_id, unsigned_hex)
}

fn sign_unsigned_hex_with_key(
  private_key: &[u8; 32],
  chain_id: u64,
  unsigned_hex: &str,
) -> Result<String, Box<dyn Error>> {
  let mut tx_bytes = decode_hex(unsigned_hex)?;
  let mut tx_slice = tx_bytes.as_slice();
  let mut tx = TypedTransaction::decode_unsigned(&mut tx_slice)?;

  if !tx_slice.is_empty() {
    return Err("unsigned transaction hex contains trailing bytes".into());
  }

  if !tx.set_chain_id_checked(chain_id) {
    return Err(
      format!("transaction chain ID does not match requested chain ID {chain_id}").into(),
    );
  }

  let signer = PrivateKeySigner::from_slice(private_key)?;
  let signature = signer.sign_hash_sync(&tx.signature_hash())?;
  let envelope: TxEnvelope = tx.into_signed(signature).into();
  let signed_bytes = envelope.encoded_2718();

  tx_bytes.clear();

  Ok(format!("0x{}", alloy::hex::encode(signed_bytes)))
}

fn decode_hex(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
  let cleaned = input
    .trim()
    .trim_start_matches("0x")
    .trim_start_matches("0X");
  Ok(alloy::hex::decode(cleaned)?)
}
