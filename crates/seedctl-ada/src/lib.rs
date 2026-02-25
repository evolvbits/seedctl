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

pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  let network = prompts::select_network()?;
  let passphrase = prompt_passphrase()?;

  let account_index = 0u32;
  let master = derive::master_from_mnemonic_icarus(mnemonic, &passphrase);
  let account = derive::derive_account(&master, account_index);

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
