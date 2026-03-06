//! Cardano (ADA) wallet derivation crate for `seedctl`.
//!
//! Implements the Icarus master-key derivation scheme (CIP-0003) on top of
//! Byron-era derivation paths and CIP-1852 Shelley paths, producing
//! bech32-encoded base addresses compatible with modern Cardano wallets
//! (e.g. Yoroi, Eternl, Daedalus).
//!
//! Orchestrates the full interactive workflow:
//!
//! 1. Network selection (Mainnet / Testnet).
//! 2. Optional BIP-39 passphrase prompt.
//! 3. Icarus master key derivation from the mnemonic entropy.
//! 4. Account key derivation at `m/1852'/1815'/0'` (CIP-1852).
//! 5. Address generation with optional on-chain balance queries via Koios.
//! 6. Wallet display and optional watch-only JSON export.

mod derive;
mod output;
mod prompts;
mod rpc;
mod wallet;

use bip39::Mnemonic;
use console::style;
use seedctl_core::{
  constants::{CARDANO_COIN_TYPE, CARDANO_PURPOSE},
  ui::{print_wallet_header, prompt_confirm_options, prompt_export_watch_only, prompt_passphrase},
  userprofile,
  utils::print_mnemonic,
};
use serde_json::to_string_pretty;
use std::{error::Error, fs, process::exit};

/// Runs the interactive Cardano wallet workflow.
///
/// Called by `seedctl`'s main dispatch loop after a BIP-39 mnemonic has been
/// obtained (either freshly generated or imported by the user).
///
/// # Parameters
///
/// - `coin_name` — human-readable chain label shown in the wallet header
///   (e.g. `"Cardano (ADA)"`).
/// - `mnemonic`  — validated BIP-39 mnemonic to derive keys from.
/// - `info`      — software metadata slice `[name, version, repository]`
///   written into the watch-only export JSON.
///
/// # Errors
///
/// Propagates any `dialoguer`, `bech32`, or filesystem error encountered
/// during the interactive session.
pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  let network = prompts::select_network()?;
  let passphrase = prompt_passphrase()?;
  let mode = prompts::select_derivation_mode()?;
  if mode == 1 {
    scan_common_paths(mnemonic, &passphrase, network)?;
    return Ok(());
  }

  // Account 0 is the conventional default for Cardano wallets.
  let account_index = 0u32;
  let master = derive::master_from_mnemonic_icarus(mnemonic, &passphrase);
  let account = derive::derive_account(&master, account_index);

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

  // Derive the first payment key to use as the "account xprv" in the display.
  let first_payment_xprv = derive::derive_payment_xprv(&account.account_xprv, 0);
  let first_secret_hex = hex::encode(first_payment_xprv.as_ref());
  let account_xpub_hex = hex::encode(account.account_xpub.as_ref());

  let mut addresses: Vec<(String, String, Option<f64>)> = Vec::with_capacity(addr_count as usize);

  for i in 0..addr_count {
    let (_, address) = derive::keypair_and_address(&account, i, network)?;
    let balance = if rpc_url.is_empty() {
      None
    } else {
      rpc::get_balance(&rpc_url, &address)
    };
    addresses.push((derive::payment_path(account_index, i), address, balance));
  }

  output::print_wallet_output(&output::WalletOutput {
    purpose: CARDANO_PURPOSE,
    coin_type: CARDANO_COIN_TYPE,
    account_xprv: &first_secret_hex,
    account_xpub: &account_xpub_hex,
    show_privkeys,
    addresses: &addresses,
  });

  let export = wallet::build_export(info, network, account_index, &account_xpub_hex);

  let export_watch_only = prompt_export_watch_only()?;
  if export_watch_only == 0 {
    let xpub_part = &export.keys.account_xpub[0..7];
    let filename = userprofile!(format!("wallet-ada-{}-watch-only.json", xpub_part));
    fs::write(&filename, to_string_pretty(&export)?)?;
    println!(
      "{} {}",
      style("Wallet exported to:").bold().yellow(),
      style(filename.to_string_lossy()).bold()
    );
  }

  Ok(())
}

fn scan_common_paths(
  mnemonic: &Mnemonic,
  passphrase: &str,
  network: prompts::AdaNetwork,
) -> Result<(), Box<dyn Error>> {
  let common = [
    (0u32, 0u32),
    (0u32, 1u32),
    (1u32, 0u32),
    (1u32, 1u32),
    (2u32, 0u32),
  ];

  println!("\n🔎 Scanning common Cardano derivation paths:\n");
  let master = derive::master_from_mnemonic_icarus(mnemonic, passphrase);
  for (account_idx, addr_idx) in common {
    let account = derive::derive_account(&master, account_idx);
    let (_, address) = derive::keypair_and_address(&account, addr_idx, network)?;
    let path = derive::payment_path(account_idx, addr_idx);
    println!("{:<24} → {}", path, address);
  }
  println!("\nTip: compare with your known wallet address to find the right path.");
  Ok(())
}
