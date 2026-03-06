//! Litecoin (LTC) wallet derivation crate for `seedctl`.
//!
//! Litecoin uses the same secp256k1 curve as Bitcoin but with its own
//! SLIP-44 coin type (`2` for Mainnet, `1` for Testnet) and bech32 address
//! encoding with the `ltc` / `tltc` HRP. The derivation path follows
//! `m/84'/<coin_type>'/0'` (BIP-84 Native SegWit P2WPKH), producing
//! `ltc1…` addresses compatible with Litecoin Core and most LTC wallets.
//!
//! Orchestrates the full interactive workflow:
//!
//! 1. Network selection (Mainnet / Testnet).
//! 2. Optional BIP-39 passphrase prompt.
//! 3. BIP-32 master key derivation from the mnemonic.
//! 4. Account key derivation at `m/84'/<coin_type>'/0'`.
//! 5. Receive address generation with optional on-chain balance queries.
//! 6. Wallet display and optional watch-only JSON export.

mod derive;
mod output;
mod prompts;
mod rpc;
mod wallet;

use bip39::Mnemonic;
use console::style;
use seedctl_core::{
  ui::{print_wallet_header, prompt_confirm_options, prompt_export_watch_only, prompt_passphrase},
  userprofile,
  utils::{format_fingerprint_hex, master_from_mnemonic_bip32, print_mnemonic},
};
use serde_json::to_string_pretty;
use std::{error::Error, fs, process::exit};

/// Runs the interactive Litecoin wallet workflow.
///
/// Called by `seedctl`'s main dispatch loop after a BIP-39 mnemonic has been
/// obtained (either freshly generated or imported by the user).
///
/// # Parameters
///
/// - `coin_name` — human-readable chain label shown in the wallet header
///   (e.g. `"Litecoin (LTC)"`).
/// - `mnemonic`  — validated BIP-39 mnemonic to derive keys from.
/// - `info`      — software metadata slice `[name, version, repository]`
///   written into the watch-only export JSON.
///
/// # Errors
///
/// Propagates any `dialoguer`, `bip32`, or filesystem error encountered
/// during the interactive session.
pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  // Step 1 — choose Mainnet or Testnet; coin_type follows SLIP-44.
  let (network, coin_type) = prompts::select_network()?;
  let derivation_style = prompts::select_derivation_style()?;
  let purpose = derivation_style.purpose();

  // Step 2 — optional BIP-39 passphrase.
  let passphrase = prompt_passphrase()?;

  // Step 3 — derive the BIP-32 master key from the mnemonic + passphrase.
  let master = master_from_mnemonic_bip32(mnemonic, &passphrase)?;

  // Step 4 — derive the account-level key pair at m/<purpose>'/<coin_type>'/0'.
  let (account_xprv, account_xpub, fingerprint) =
    derive::derive_account(&master, coin_type, purpose)?;
  let account_xprv_hex = hex::encode(account_xprv.to_bytes());
  let account_xpub_hex = hex::encode(account_xpub.to_bytes());
  let derivation_path = format!("m/{purpose}'/{coin_type}'/0'");

  let go_continue = prompt_confirm_options()?;
  if go_continue == 1 {
    exit(0);
  }

  // Step 5 — optionally prompt for an RPC URL for balance queries.
  let rpc_url = prompts::prompt_rpc_url()?;

  // Step 6 — display wallet header and mnemonic table.
  print_wallet_header(coin_name);
  print_mnemonic(
    mnemonic,
    &format!("BIP39 MNEMONIC ({} words):", mnemonic.word_count()),
  );

  // Step 7 — derive receive addresses and optionally query balances.
  let receive_addresses = derive::receive_addresses(
    &account_xprv,
    network,
    coin_type,
    purpose,
    derivation_style,
    10,
  )?;
  let mut addresses: Vec<(String, String, Option<f64>)> =
    Vec::with_capacity(receive_addresses.len());

  for (path, addr) in receive_addresses {
    let balance = if rpc_url.is_empty() {
      None
    } else {
      rpc::get_balance(&rpc_url, &addr)
    };
    addresses.push((path, addr, balance));
  }

  // Step 8 — render the wallet section to the terminal.
  output::print_wallet_output(&output::WalletOutput {
    purpose,
    coin_type,
    fingerprint: &fingerprint,
    account_xprv: &account_xprv_hex,
    account_xpub: &account_xpub_hex,
    addresses: &addresses,
    descriptor: derivation_style.descriptor(),
  });

  // Step 9 — assemble the watch-only export document.
  let export = wallet::build_export(&wallet::BuildExport {
    info,
    network,
    script_type: derivation_style.script_type(),
    derivation_path: &derivation_path,
    fingerprint: &fingerprint,
    account_xpub: &account_xpub_hex,
  });

  // Step 10 — optionally write the watch-only JSON file to the home directory.
  let json = to_string_pretty(&export)?;
  let export_watch_only = prompt_export_watch_only()?;

  if export_watch_only == 0 {
    let filename = userprofile!(format!(
      "wallet-ltc-{}-watch-only.json",
      format_fingerprint_hex(&fingerprint)
    ));
    fs::write(&filename, json)?;
    println!(
      "{} {}",
      style("Wallet exported to:").bold().yellow(),
      style(filename.to_string_lossy()).bold()
    );
  }

  Ok(())
}
