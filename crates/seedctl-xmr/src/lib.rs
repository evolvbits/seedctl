//! Monero (XMR) wallet derivation crate for `seedctl`.
//!
//! Monero uses a custom key scheme derived from a BIP-39 seed rather than
//! standard BIP-32 / secp256k1. The derivation flow:
//!
//! 1. The BIP-39 64-byte seed is passed through `Hs()` (Keccak-256 mod order)
//!    to produce the spend private scalar.
//! 2. The view private scalar is derived by hashing the spend scalar again.
//! 3. The spend and view public keys are computed as scalar multiples of the
//!    Ed25519 base point.
//! 4. Addresses are encoded using the Monero-specific Base58 alphabet with
//!    8-byte block encoding (not standard Base58Check).
//!
//! Subaddresses (index > 0) are derived using the official Monero subaddress
//! scheme (`"SubAddr\0" || view_private || major || minor`).
//!
//! Orchestrates the full interactive workflow:
//!
//! 1. Network selection (Mainnet / Testnet).
//! 2. Optional BIP-39 passphrase prompt.
//! 3. Monero master wallet derivation from the BIP-39 seed.
//! 4. Address generation (standard + subaddresses) with optional balances.
//! 5. Wallet display and optional watch-only JSON export.

mod derive;
mod output;
mod prompts;
mod rpc;
mod wallet;

use bip39::Mnemonic;
use console::style;
use seedctl_core::{
  constants::{BIP44, MONERO_COIN_TYPE},
  ui::{print_wallet_header, prompt_confirm_options, prompt_export_watch_only, prompt_passphrase},
  userprofile,
  utils::print_mnemonic,
};
use serde_json::to_string_pretty;
use std::{error::Error, fs, process::exit};

/// Runs the interactive Monero wallet workflow.
///
/// Called by `seedctl`'s main dispatch loop after a BIP-39 mnemonic has been
/// obtained (either freshly generated or imported by the user).
///
/// # Parameters
///
/// - `coin_name` — human-readable chain label shown in the wallet header
///   (e.g. `"Monero (XMR)"`).
/// - `mnemonic`  — validated BIP-39 mnemonic to derive keys from.
/// - `info`      — software metadata slice `[name, version, repository]`
///   written into the watch-only export JSON.
///
/// # Errors
///
/// Propagates any `dialoguer`, cryptographic, or filesystem error encountered
/// during the interactive session.
pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  let network = prompts::select_network()?;
  let passphrase = prompt_passphrase()?;
  let seed = mnemonic.to_seed(&passphrase);

  // Derive the Monero wallet keys from the BIP-39 seed.
  let wallet = derive::wallet_from_bip39_seed(&seed);

  let addr_count = prompts::prompt_address_count()?;
  let rpc_url = prompts::prompt_rpc_url()?;
  // Private keys are shown unconditionally; add a prompt here to make this configurable.
  let show_privkeys = true;

  let go = prompt_confirm_options()?;
  if go == 1 {
    exit(0);
  }

  print_wallet_header(coin_name);
  print_mnemonic(
    mnemonic,
    &format!("BIP39 MNEMONIC ({} words):", mnemonic.word_count()),
  );

  let mut addresses: Vec<(String, String, Option<f64>)> = Vec::with_capacity(addr_count as usize);

  for i in 0..addr_count {
    let derived = derive::derive_address(&wallet, network, i);
    let balance = if rpc_url.is_empty() {
      None
    } else {
      rpc::get_balance(&rpc_url, &derived.address)
    };
    addresses.push((derived.path, derived.address, balance));
  }

  let spend_private_hex = wallet.spend_private_hex();
  let spend_public_hex = wallet.spend_public_hex();

  output::print_wallet_output(&output::WalletOutput {
    purpose: BIP44,
    coin_type: MONERO_COIN_TYPE,
    account_xprv: &spend_private_hex,
    account_xpub: &spend_public_hex,
    show_privkeys,
    addresses: &addresses,
  });

  let export = wallet::build_export(info, network, &spend_public_hex);

  let export_watch_only = prompt_export_watch_only()?;
  if export_watch_only == 0 {
    let xpub_part = &export.keys.account_xpub[0..7];
    let filename = userprofile!(format!("wallet-xmr-{}-watch-only.json", xpub_part));
    fs::write(&filename, to_string_pretty(&export)?)?;
    println!(
      "{} {}",
      style("Wallet exported to:").bold().yellow(),
      style(filename.to_string_lossy()).bold()
    );
  }

  Ok(())
}
