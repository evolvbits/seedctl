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

pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  let network = prompts::select_network()?;
  let passphrase = prompt_passphrase()?;
  let seed = mnemonic.to_seed(&passphrase);

  let wallet = derive::wallet_from_bip39_seed(&seed);

  let addr_count = prompts::prompt_address_count()?;
  let rpc_url = prompts::prompt_rpc_url()?;
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
