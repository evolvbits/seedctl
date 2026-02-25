mod derive;
mod output;
mod prompts;
mod rpc;
mod utils;
mod wallet;

use bip39::Mnemonic;
use bitcoin::key::Secp256k1;
use console::style;
use seedctl_core::{
  ui::{print_wallet_header, prompt_confirm_options, prompt_export_watch_only, prompt_passphrase},
  userprofile,
  utils::{format_fingerprint_hex, print_mnemonic},
};
use serde_json::to_string_pretty;
use std::{error::Error, fs, process::exit};

pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  // 3) Escolher mainnet/testnet da moeda
  let (btc_network, coin_type) = prompts::select_network()?;

  // 4) Passphrase opcional
  let passphrase = prompt_passphrase()?;
  let master = wallet::master_from_mnemonic(mnemonic, &passphrase, btc_network)?;

  // 5) Escolher derivation path (BIP84/49/44)
  let (purpose, address_type) = prompts::select_address_type()?;
  let secp = Secp256k1::new();
  let (acc_xprv, acc_xpub, fingerprint) =
    derive::derive_account(&master, &secp, purpose, coin_type)?;

  let (account_xprv, account_xpub) =
    wallet::account_key_strings(&acc_xprv, &acc_xpub, address_type);

  let (_, desc_receive, desc_change) =
    wallet::key_origin_and_descriptors(fingerprint, purpose, coin_type, &account_xpub);
  let derivation_path = format!("m/{}'/{}'/0'", purpose, coin_type);
  let script_type = match purpose {
    84 => "bip84",
    49 => "bip49",
    44 => "bip44",
    _ => unreachable!(),
  };

  let rpc_url = prompts::prompt_rpc_url()?;

  let go_continue = prompt_confirm_options()?;
  if go_continue == 1 {
    exit(0);
  }

  print_wallet_header(coin_name);
  print_mnemonic(
    mnemonic,
    &format!("BIP39 MNEMONIC ({} words):", mnemonic.word_count()),
  );

  let receive_addresses = derive::receive_addresses(
    &acc_xpub,
    &secp,
    btc_network,
    address_type,
    purpose,
    coin_type,
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

  output::print_wallet_output(&output::WalletOutput {
    purpose,
    coin_type,
    fingerprint: &fingerprint,
    account_xprv: &account_xprv,
    account_xpub: &account_xpub,
    desc_receive: &desc_receive,
    desc_change: &desc_change,
    addresses: &addresses,
  });

  // Watch-only wallet
  let watch_only = true;
  let export = wallet::build_export(&wallet::BuildExport {
    info,
    network: btc_network,
    script_type,
    derivation_path: &derivation_path,
    fingerprint: &fingerprint,
    account_xpub: &account_xpub,
    account_xprv: if watch_only {
      None
    } else {
      Some(&account_xprv)
    },
    desc_receive: &desc_receive,
    desc_change: &desc_change,
  });

  let json = to_string_pretty(&export).unwrap();
  let export_watch_only = prompt_export_watch_only()?;

  if export_watch_only == 0 {
    let filename = userprofile!(format!(
      "wallet-btc-{}-watch-only.json",
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
