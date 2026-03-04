//! XRP Ledger (XRP) wallet derivation crate for `seedctl`.
//!
//! XRP Ledger uses secp256k1 with a custom Base58Check alphabet and a
//! `hash160` (SHA-256 → RIPEMD-160) address derivation scheme, producing
//! classic addresses that start with `r`. The derivation path follows
//! `m/44'/144'/0'/0/x`, compatible with XUMM, Ledger, and the XRPL CLI.
//!
//! Orchestrates the full interactive workflow:
//!
//! 1. Network selection (Mainnet / Testnet).
//! 2. Optional BIP-39 passphrase prompt.
//! 3. BIP-32 master key derivation from the mnemonic.
//! 4. Account key derivation at `m/44'/144'/0'/0`.
//! 5. Address generation with optional on-chain balance queries via XRPL RPC.
//! 6. Wallet display and optional watch-only JSON export.

mod derive;
mod output;
mod prompts;
mod rpc;
mod wallet;

use bip32::ChildNumber;
use bip39::Mnemonic;
use console::style;
use seedctl_core::{
  constants::{BIP44, XRP_COIN_TYPE},
  ui::{print_wallet_header, prompt_confirm_options, prompt_export_watch_only, prompt_passphrase},
  userprofile,
  utils::{master_from_mnemonic, print_mnemonic},
};
use serde_json::to_string_pretty;
use std::{error::Error, fs, process::exit};

/// Runs the interactive XRP Ledger wallet workflow.
///
/// Called by `seedctl`'s main dispatch loop after a BIP-39 mnemonic has been
/// obtained (either freshly generated or imported by the user).
///
/// # Parameters
///
/// - `coin_name` — human-readable chain label shown in the wallet header
///   (e.g. `"XRP Ledger (XRP)"`).
/// - `mnemonic`  — validated BIP-39 mnemonic to derive keys from.
/// - `info`      — software metadata slice `[name, version, repository]`
///   written into the watch-only export JSON.
///
/// # Errors
///
/// Propagates any `dialoguer`, `bip32`, or filesystem error encountered
/// during the interactive session.
pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  // Step 1 — choose Mainnet or Testnet.
  let network = prompts::select_network()?;

  // Step 2 — optional BIP-39 passphrase.
  let passphrase = prompt_passphrase()?;
  let master = master_from_mnemonic(mnemonic, &passphrase)?;

  // Step 3 — collect configuration (address count, optional RPC URL).
  let addr_count = prompts::prompt_address_count()?;
  let rpc_url = prompts::prompt_rpc_url()?;
  // Private keys are shown unconditionally; add a prompt here to make this configurable.
  let show_privkeys = true;

  // Step 4 — derive the account-level key pair at m/44'/144'/0'/0.
  let base_path = "m/44'/144'/0'/0";
  let account_xprv = seedctl_core::evm::derive_from_path(master, base_path)?;
  let account_xpub = account_xprv.public_key();

  let account_xprv_hex = hex::encode(account_xprv.to_bytes());
  let account_xpub_hex = hex::encode(account_xpub.to_bytes());

  let go = prompt_confirm_options()?;
  if go == 1 {
    exit(0);
  }

  // Step 5 — display wallet header and mnemonic table.
  print_wallet_header(coin_name);
  print_mnemonic(
    mnemonic,
    &format!("BIP39 MNEMONIC ({} words):", mnemonic.word_count()),
  );

  // Step 6 — derive addresses and optionally query on-chain balances.
  let mut addresses: Vec<(String, String, Option<f64>)> = Vec::with_capacity(addr_count as usize);
  for i in 0..addr_count {
    let child_num = ChildNumber::new(i, false)?;
    let child_xprv = account_xprv.clone().derive_child(child_num)?;
    let address = derive::address_from_xprv(child_xprv)?;
    let balance = if rpc_url.is_empty() {
      None
    } else {
      rpc::get_balance(&rpc_url, &address)
    };
    addresses.push((format!("{base_path}/{i}"), address, balance));
  }

  // Step 7 — render the wallet section.
  output::print_wallet_output(&output::WalletOutput {
    purpose: BIP44,
    coin_type: XRP_COIN_TYPE,
    account_xprv: &account_xprv_hex,
    account_xpub: &account_xpub_hex,
    show_privkeys,
    addresses: &addresses,
  });

  // Step 8 — optionally export a watch-only JSON file.
  let export = wallet::build_export(info, network, base_path, &account_xpub_hex);

  let export_watch_only = prompt_export_watch_only()?;
  if export_watch_only == 0 {
    let xpub_part = &export.keys.account_xpub[0..7];
    let filename = userprofile!(format!("wallet-xrp-{}-watch-only.json", xpub_part));
    fs::write(&filename, to_string_pretty(&export)?)?;
    println!(
      "{} {}",
      style("Wallet exported to:").bold().yellow(),
      style(filename.to_string_lossy()).bold()
    );
  }

  Ok(())
}
