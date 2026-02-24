mod derive;
mod output;
mod prompts;
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

pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  // 3) (Opcional) no Tron consideramos apenas mainnet por enquanto,
  // mas poderíamos adicionar um select_network aqui no futuro.

  // 4) Passphrase opcional
  let passphrase = prompt_passphrase()?;
  let master = master_from_mnemonic(mnemonic, &passphrase)?;

  // 5) Derivation path (Standard / Ledger / Custom)
  let addr_count = prompts::prompt_address_count()?;
  let derivation_style = prompts::select_derivation_style()?;
  let base_path = utils::style_to_string(&derivation_style);

  // 6) Geração de chaves e endereços
  let account_xprv = utils::derive_from_path(master.clone(), &base_path)?;
  let account_xpub = account_xprv.public_key();

  let export = wallet::build_export(info, &base_path, account_xpub.clone())?;
  // let show_privkeys = prompts::prompt_show_privkeys()?; // It asks if you want to show the private key.
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

  let mut addresses: Vec<(String, String)> = Vec::with_capacity(addr_count as usize);
  for i in 0..addr_count {
    let (child, path_str) =
      utils::derive_address_key(&master, &account_xprv, &derivation_style, i)?;
    let addr = derive::address_from_xprv(child)?;
    addresses.push((path_str.clone(), addr.clone()));
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
