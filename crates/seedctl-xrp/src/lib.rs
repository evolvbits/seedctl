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

pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  let network = prompts::select_network()?;
  let passphrase = prompt_passphrase()?;
  let master = master_from_mnemonic(mnemonic, &passphrase)?;

  let addr_count = prompts::prompt_address_count()?;
  let rpc_url = prompts::prompt_rpc_url()?;
  let show_privkeys = true;

  let base_path = "m/44'/144'/0'/0";
  let account_xprv = seedctl_core::evm::derive_from_path(master, base_path)?;
  let account_xpub = account_xprv.public_key();

  let account_xprv_hex = hex::encode(account_xprv.to_bytes());
  let account_xpub_hex = hex::encode(account_xpub.to_bytes());

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

  output::print_wallet_output(&output::WalletOutput {
    purpose: BIP44,
    coin_type: XRP_COIN_TYPE,
    account_xprv: &account_xprv_hex,
    account_xpub: &account_xpub_hex,
    show_privkeys,
    addresses: &addresses,
  });

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
