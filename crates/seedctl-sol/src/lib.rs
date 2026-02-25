mod derive;
mod output;
mod rpc;
mod utils;
mod wallet;

use bip39::Mnemonic;
use console::style;
use seedctl_core::{
  constants::{BIP44, ETHEREUM_COIN_TYPE},
  ui::{prompt_confirm_options, prompt_export_watch_only, prompt_passphrase},
  userprofile,
  utils::print_mnemonic,
};
use serde_json::to_string_pretty;
use std::{error::Error, fs, process::exit};

pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  // 3) (Opcional) Solana: por enquanto assumimos mainnet; etiquetas podem ser ajustadas no export.

  // 4) Passphrase opcional
  let passphrase = prompt_passphrase()?;
  let seed = mnemonic.to_seed(&passphrase);

  // 5) Derivation path (fixo m/44'/501'/0'/0' + índice)
  let addr_count = utils::prompt_address_count()?;
  let rpc_url = utils::prompt_rpc_url()?;
  // let show_privkeys = utils::prompt_show_privkeys()?; // It asks if you want to show the private key.
  let show_privkeys = true;

  let go = prompt_confirm_options()?;
  if go == 1 {
    exit(0);
  }

  // 6) Geração de chaves e endereços
  seedctl_core::ui::print_wallet_header(coin_name);
  print_mnemonic(
    mnemonic,
    &format!("BIP39 MNEMONIC ({} words):", mnemonic.word_count()),
  );

  let (first_secret_hex, first_pubkey_hex) = {
    let (kp0, _) = derive::keypair_and_address(&seed, 0)?;
    let secret_hex = hex::encode(kp0.to_bytes());
    let pubkey_hex = hex::encode(kp0.verifying_key().as_bytes());
    (secret_hex, pubkey_hex)
  };

  let mut addresses: Vec<(String, String, Option<f64>)> = Vec::with_capacity(addr_count as usize);
  for i in 0..addr_count {
    let (_, addr) = derive::keypair_and_address(&seed, i)?;
    let path = format!("m/44'/501'/{}'/0'", i);
    let balance = if rpc_url.is_empty() {
      None
    } else {
      rpc::get_balance(&rpc_url, &addr)
    };
    addresses.push((path, addr, balance));
  }

  let export = wallet::build_export(info, &first_pubkey_hex)?;

  output::print_wallet_output(&output::WalletOutput {
    purpose: ETHEREUM_COIN_TYPE,
    coin_type: BIP44,
    account_xprv: &first_secret_hex,
    account_xpub: &first_pubkey_hex,
    show_privkeys,
    addresses: &addresses,
  });

  let export_watch_only = prompt_export_watch_only()?;
  if export_watch_only == 0 {
    let xpub_part = &export.keys.account_xpub[0..7];
    let filename = userprofile!(format!("wallet-sol-{}-watch-only.json", xpub_part));
    fs::write(&filename, to_string_pretty(&export)?)?;
    println!(
      "{} {}",
      style("Wallet exported to:").bold().yellow(),
      style(filename.to_string_lossy()).bold()
    );
  }

  Ok(())
}
