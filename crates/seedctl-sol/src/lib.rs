//! Solana (SOL) wallet derivation crate for `seedctl`.
//!
//! Solana uses the Ed25519 curve with BIP-32 SLIP-0010 derivation, producing
//! base58-encoded 32-byte public keys as addresses. The derivation path
//! follows `m/44'/501'/<index>'/0'`, which is compatible with Phantom,
//! Solflare, and the Solana CLI.
//!
//! Orchestrates the full interactive workflow:
//!
//! 1. Optional BIP-39 passphrase prompt.
//! 2. Address count and optional RPC URL prompts.
//! 3. Ed25519 key derivation and address generation with optional balances.
//! 4. Wallet display and optional watch-only JSON export.

mod derive;
mod output;
mod rpc;
mod utils;
mod wallet;

use bip39::Mnemonic;
use console::style;
use seedctl_core::{
  constants::{BIP44, ETHEREUM_COIN_TYPE},
  ui::{prompt_confirm_options, prompt_export_watch_only, prompt_passphrase},
  userprofile,
  utils::print_mnemonic,
};
use serde_json::to_string_pretty;
use std::{error::Error, fs, process::exit};

/// Runs the interactive Solana wallet workflow.
///
/// Called by `seedctl`'s main dispatch loop after a BIP-39 mnemonic has been
/// obtained (either freshly generated or imported by the user).
///
/// # Parameters
///
/// - `coin_name` — human-readable chain label shown in the wallet header
///   (e.g. `"Solana (SOL)"`).
/// - `mnemonic`  — validated BIP-39 mnemonic to derive keys from.
/// - `info`      — software metadata slice `[name, version, repository]`
///   written into the watch-only export JSON.
///
/// # Errors
///
/// Propagates any `dialoguer`, `ed25519`, or filesystem error encountered
/// during the interactive session.
pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  let passphrase = prompt_passphrase()?;
  let seed = mnemonic.to_seed(&passphrase);

  // Derive the BIP-39 64-byte seed; individual address seeds are derived inside `derive`.
  let addr_count = utils::prompt_address_count()?;
  let rpc_url = utils::prompt_rpc_url()?;
  // Private keys are shown unconditionally; add a prompt here to make this configurable.
  let show_privkeys = true;

  let go = prompt_confirm_options()?;
  if go == 1 {
    exit(0);
  }

  // Display the wallet header and mnemonic table.
  seedctl_core::ui::print_wallet_header(coin_name);
  print_mnemonic(
    mnemonic,
    &format!("BIP39 MNEMONIC ({} words):", mnemonic.word_count()),
  );

  // Derive the first key pair to use as the "account" keys in the display.
  let (first_secret_hex, first_pubkey_hex) = {
    let (kp0, _) = derive::keypair_and_address(&seed, 0)?;
    let secret_hex = hex::encode(kp0.to_bytes());
    let pubkey_hex = hex::encode(kp0.verifying_key().as_bytes());
    (secret_hex, pubkey_hex)
  };

  let mut addresses: Vec<(String, String, Option<f64>)> = Vec::with_capacity(addr_count as usize);

  for i in 0..addr_count {
    let (_, addr) = derive::keypair_and_address(&seed, i)?;
    let path = format!("m/44'/501'/{}'/0'", i);
    let balance = if rpc_url.is_empty() {
      None
    } else {
      rpc::get_balance(&rpc_url, &addr)
    };
    addresses.push((path, addr, balance));
  }

  let export = wallet::build_export(info, &first_pubkey_hex)?;

  output::print_wallet_output(&output::WalletOutput {
    purpose: ETHEREUM_COIN_TYPE,
    coin_type: BIP44,
    account_xprv: &first_secret_hex,
    account_xpub: &first_pubkey_hex,
    show_privkeys,
    addresses: &addresses,
  });

  let export_watch_only = prompt_export_watch_only()?;
  if export_watch_only == 0 {
    let xpub_part = &export.keys.account_xpub[0..7];
    let filename = userprofile!(format!("wallet-sol-{}-watch-only.json", xpub_part));
    fs::write(&filename, to_string_pretty(&export)?)?;
    println!(
      "{} {}",
      style("Wallet exported to:").bold().yellow(),
      style(filename.to_string_lossy()).bold()
    );
  }

  Ok(())
}
