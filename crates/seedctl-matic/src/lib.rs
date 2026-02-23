//! Polygon
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
  types::address::EthAddress,
  ui::{prompt_confirm_options, prompt_export_watch_only, prompt_passphrase, table::print_table},
  userprofile,
  utils::{master_from_mnemonic, print_mnemonic},
};
use serde_json::to_string_pretty;
use std::{error::Error, fs, process::exit};

pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  // 3) (Opcional) Polygon (EVM) usa a mesma estrutura de chaves do Ethereum.

  // 4) Passphrase opcional
  let passphrase = prompt_passphrase()?;
  let master = master_from_mnemonic(mnemonic, &passphrase)?;

  // 5) Derivation mode/style
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
  // It asks if you want to show the private key (mantemos true por padrão, como no ETH).
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
    let balance = if rpc_url.is_empty() {
      None
    } else {
      rpc::get_balance(&rpc_url, &addr)
    };
    addresses.push((path_str.clone(), addr.clone(), balance));
  }

  let purpose = 44u32; // BIP44
  let coin = 60u32; // Ethereum coin type
  let addr_rows: Vec<EthAddress> = addresses
    .iter()
    .map(|(path, addr, balance)| EthAddress {
      path: path.clone(),
      address: addr.clone(),
      balance: *balance,
    })
    .collect();

  output::print_wallet_output(&output::WalletOutput {
    purpose,
    coin_type: coin,
    account_xprv: &hex::encode(account_xprv.to_bytes()),
    account_xpub: &hex::encode(account_xpub.to_bytes()),
    show_privkeys,
    addresses: &addresses,
  });

  print_table(&addr_rows);

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
