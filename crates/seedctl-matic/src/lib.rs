//! Polygon (MATIC/POL) wallet derivation crate for `seedctl`.
//!
//! Polygon is EVM-compatible and shares the same derivation logic as Ethereum
//! (`m/44'/60'/0'/0/x`). All heavy lifting is delegated to the shared
//! [`seedctl_core::evm`] module via thin wrapper functions so that only the
//! chain profile differs across EVM crates.
//!
//! Orchestrates the full interactive workflow:
//!
//! 1. Optional BIP-39 passphrase prompt.
//! 2. Derivation-mode selection (generate addresses / scan common paths).
//! 3. Address count, derivation style, and optional RPC URL prompts.
//! 4. Account key derivation and address generation with optional balances.
//! 5. Wallet display and optional watch-only JSON export.

mod derive;
mod output;
mod prompts;
mod rpc;
mod scanner;
mod utils;
mod wallet;

use bip39::Mnemonic;
use console::style;
use seedctl_core::{
  constants::{BIP44, ETHEREUM_COIN_TYPE},
  ui::{prompt_confirm_options, prompt_export_watch_only, prompt_passphrase},
  userprofile,
  utils::{master_from_mnemonic, print_mnemonic},
};
use serde_json::to_string_pretty;
use std::{error::Error, fs, process::exit};

/// Runs the interactive Polygon wallet workflow.
///
/// Called by `seedctl`'s main dispatch loop after a BIP-39 mnemonic has been
/// obtained (either freshly generated or imported by the user).
///
/// # Parameters
///
/// - `coin_name` — human-readable chain label shown in the wallet header
///   (e.g. `"Polygon (POL)"`).
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

  // Step 1 — choose between generating addresses and scanning common paths.
  let mode = prompts::select_derivation_mode()?;
  if mode == 1 {
    scanner::scan_common_paths(master)?;
    return Ok(());
  }

  let addr_count = prompts::prompt_address_count()?;
  let derivation_style = prompts::select_derivation_style()?;
  let base_path = utils::style_to_string(&derivation_style);

  let account_xprv = utils::derive_from_path(master.clone(), &base_path)?;
  let account_xpub = account_xprv.public_key();

  let export = wallet::build_export(info, &base_path, account_xpub.clone())?;

  let rpc_url = prompts::prompt_rpc_url()?;
  let rpc_client = if rpc_url.is_empty() {
    None
  } else {
    Some(rpc::RpcClient::new(rpc_url.clone()))
  };
  // Private keys are shown unconditionally; add a prompt here to make this configurable.
  let show_privkeys = true;

  let go = prompt_confirm_options()?;
  if go == 1 {
    exit(0);
  }

  seedctl_core::ui::print_wallet_header(coin_name);
  print_mnemonic(
    mnemonic,
    &format!("BIP39 MNEMONIC ({} words):", mnemonic.word_count()),
  );

  let mut addresses: Vec<(String, String, Option<f64>)> = Vec::with_capacity(addr_count as usize);

  for i in 0..addr_count {
    let (child, path_str) =
      utils::derive_address_key(&master, &account_xprv, &derivation_style, i)?;
    let addr = derive::address_from_xprv(child)?;
    let balance = rpc_client
      .as_ref()
      .and_then(|client| client.get_balance(&addr));
    addresses.push((path_str, addr, balance));
  }

  output::print_wallet_output(&output::WalletOutput {
    purpose: BIP44,
    coin_type: ETHEREUM_COIN_TYPE,
    account_xprv: &hex::encode(account_xprv.to_bytes()),
    account_xpub: &hex::encode(account_xpub.to_bytes()),
    show_privkeys,
    addresses: &addresses,
  });

  let export_watch_only = prompt_export_watch_only()?;
  if export_watch_only == 0 {
    let xpub_part = &export.keys.account_xpub[0..7];
    let filename = userprofile!(format!("wallet-matic-{}-watch-only.json", xpub_part));
    fs::write(&filename, to_string_pretty(&export)?)?;
    println!(
      "{} {}",
      style("Wallet exported to:").bold().yellow(),
      style(filename.to_string_lossy()).bold()
    );
  }

  Ok(())
}
