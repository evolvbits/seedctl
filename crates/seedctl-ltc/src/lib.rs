mod derive;
mod output;
mod prompts;
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

pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  let (network, coin_type) = prompts::select_network()?;
  let passphrase = prompt_passphrase()?;
  let master = master_from_mnemonic_bip32(mnemonic, &passphrase)?;
  let (account_xprv, account_xpub, fingerprint) = derive::derive_account(&master, coin_type)?;
  let account_xprv_hex = hex::encode(account_xprv.to_bytes());
  let account_xpub_hex = hex::encode(account_xpub.to_bytes());
  let derivation_path = format!("m/84'/{}'/0'", coin_type);

  let go_continue = prompt_confirm_options()?;
  if go_continue == 1 {
    exit(0);
  }

  print_wallet_header(coin_name);
  print_mnemonic(
    mnemonic,
    &format!("BIP39 MNEMONIC ({} words):", mnemonic.word_count()),
  );

  let addresses = derive::receive_addresses(&account_xprv, network, coin_type, 10)?;

  output::print_wallet_output(&output::WalletOutput {
    coin_type,
    fingerprint: &fingerprint,
    account_xprv: &account_xprv_hex,
    account_xpub: &account_xpub_hex,
    addresses: &addresses,
  });

  let export = wallet::build_export(&wallet::BuildExport {
    info,
    network,
    script_type: "bip84",
    derivation_path: &derivation_path,
    fingerprint: &fingerprint,
    account_xpub: &account_xpub_hex,
  });

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
