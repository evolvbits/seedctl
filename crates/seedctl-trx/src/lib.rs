//! Tron (TRX) wallet derivation crate for `seedctl`.
//!
//! Tron uses secp256k1 with Keccak-256 address encoding, prefixed with `0x41`
//! and Base58Check-encoded to produce addresses starting with `T`. The
//! derivation path follows `m/44'/195'/0'/0/x` (Standard) or
//! `m/44'/195'/0'` (Ledger style).
//!
//! Orchestrates the full interactive workflow:
//!
//! 1. Optional BIP-39 passphrase prompt.
//! 2. Address count, derivation style, and optional RPC URL prompts.
//! 3. Account key derivation and address generation with optional balances.
//! 4. Wallet display and optional watch-only JSON export.

mod derive;
mod output;
mod prompts;
mod rpc;
mod utils;
mod wallet;

use bip39::Mnemonic;
use console::style;
use seedctl_core::{
  ui::{prompt_confirm_options, prompt_export_watch_only, prompt_passphrase},
  userprofile,
  utils::{master_from_mnemonic, print_mnemonic},
};
use serde_json::to_string_pretty;
use std::{error::Error, fs, process::exit};

/// Runs the interactive Tron wallet workflow.
///
/// Called by `seedctl`'s main dispatch loop after a BIP-39 mnemonic has been
/// obtained (either freshly generated or imported by the user).
///
/// # Parameters
///
/// - `coin_name` — human-readable chain label shown in the wallet header
///   (e.g. `"TRON (TRX)"`).
/// - `mnemonic`  — validated BIP-39 mnemonic to derive keys from.
/// - `info`      — software metadata slice `[name, version, repository]`
///   written into the watch-only export JSON.
///
/// # Errors
///
/// Propagates any `dialoguer`, `bip32`, or filesystem error encountered
/// during the interactive session.
pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  let passphrase = prompt_passphrase()?;
  let master = master_from_mnemonic(mnemonic, &passphrase)?;

  let mode = prompts::select_derivation_mode()?;
  if mode == 1 {
    scan_common_paths(&master)?;
    return Ok(());
  }

  // Step 1 — choose derivation style and collect configuration.
  let addr_count = prompts::prompt_address_count()?;
  let derivation_style = prompts::select_derivation_style()?;
  let rpc_url = prompts::prompt_rpc_url()?;
  let base_path = utils::style_to_string(&derivation_style);

  // Step 2 — derive the account-level key pair.
  let account_xprv = utils::derive_from_path(master.clone(), &base_path)?;
  let account_xpub = account_xprv.public_key();

  let export = wallet::build_export(info, &base_path, account_xpub.clone())?;
  // Private keys are shown unconditionally; add a prompt here to make this configurable.
  let show_privkeys = true;

  let go = prompt_confirm_options()?;
  if go == 1 {
    exit(0);
  }

  // Step 3 — display wallet header and mnemonic table.
  seedctl_core::ui::print_wallet_header(coin_name);
  print_mnemonic(
    mnemonic,
    &format!("BIP39 MNEMONIC ({} words):", mnemonic.word_count()),
  );

  // Step 4 — derive addresses and optionally query balances.
  let mut addresses: Vec<(String, String, Option<f64>)> = Vec::with_capacity(addr_count as usize);
  for i in 0..addr_count {
    let (child, path_str) =
      utils::derive_address_key(&master, &account_xprv, &derivation_style, i)?;
    let addr = derive::address_from_xprv(child)?;
    let balance = if rpc_url.is_empty() {
      None
    } else {
      rpc::get_balance(&rpc_url, &addr)
    };
    addresses.push((path_str, addr, balance));
  }

  output::print_account_and_addresses(
    &hex::encode(account_xprv.to_bytes()),
    &hex::encode(account_xpub.to_bytes()),
    show_privkeys,
    addr_count,
    &addresses,
  )?;

  let export_watch_only = prompt_export_watch_only()?;
  if export_watch_only == 0 {
    let xpub_part = &export.keys.account_xpub[0..7];
    let filename = userprofile!(format!("wallet-trx-{}-watch-only.json", xpub_part));
    fs::write(&filename, to_string_pretty(&export)?)?;
    println!(
      "{} {}",
      style("Wallet exported to:").bold().yellow(),
      style(filename.to_string_lossy()).bold()
    );
  }

  Ok(())
}

fn scan_common_paths(master: &bip32::XPrv) -> Result<(), Box<dyn Error>> {
  let common_paths = [
    "m/44'/195'/0'/0/0",
    "m/44'/195'/0'/0/1",
    "m/44'/195'/1'/0/0",
    "m/44'/195'/0'/0'/0/0",
    "m/44'/195'/0'/1/0",
  ];

  println!("\n🔎 Scanning common Tron derivation paths:\n");
  for path in common_paths {
    let child = utils::derive_from_path(master.clone(), path)?;
    let addr = derive::address_from_xprv(child)?;
    println!("{:<24} → {}", path, addr);
  }
  println!("\nTip: compare with your known wallet address to find the right path.");
  Ok(())
}
