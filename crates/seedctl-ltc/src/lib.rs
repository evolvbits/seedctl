use bech32::{self, Bech32, Hrp};
use bip32::{DerivationPath, XPrv, XPub};
use bip39::Mnemonic;
use console::style;
use ripemd::Ripemd160;
use seedctl_core::{
  export,
  traits::chain::Chain,
  types::address::AddressRow,
  ui::{print_address_table, prompt_confirm_options, prompt_export_watch_only, prompt_passphrase},
  userprofile,
  utils::{format_fingerprint_hex, print_mnemonic},
};
use serde_json::to_string_pretty;
use sha2::{Digest as ShaDigest, Sha256};
use std::{error::Error, fs, process::exit};

#[derive(Clone, Copy)]
enum LtcNetwork {
  Mainnet,
  Testnet,
}

/// Concrete implementation of the generic `Chain` trait for Litecoin.
struct LtcChain;

impl Chain for LtcChain {
  type Address = AddressRow;
  type PublicKey = XPub;
  type PrivateKey = XPrv;

  fn name() -> &'static str {
    "Litecoin (LTC)"
  }

  fn symbol() -> &'static str {
    "LTC"
  }

  // SLIP‑44: 2' for Litecoin mainnet
  fn coin_type() -> u32 {
    2
  }

  fn derive_account(seed: &[u8], account: u32) -> Self::PrivateKey {
    let path_str = format!("m/84'/{}'/{}'", Self::coin_type(), account);
    let account_path: DerivationPath = path_str
      .parse()
      .expect("invalid derivation path for Litecoin account");
    XPrv::derive_from_path(seed, &account_path)
      .expect("failed to derive Litecoin account private key")
  }

  fn public_from_private(privkey: &Self::PrivateKey) -> Self::PublicKey {
    privkey.public_key()
  }

  fn derive_addresses(_pubkey: &Self::PublicKey, count: u32) -> Vec<Self::Address> {
    // For Litecoin, addresses are derived from child private keys, so we only
    // have access to the account‑level xpub here. To keep the behaviour
    // consistent with the previous implementation (which derived from seed),
    // we treat the provided public key as the account‑level key and only
    // construct address rows with the standard BIP84 path; the actual address
    // string must still be computed from child keys elsewhere.
    //
    // For now, we just build placeholder rows based on the canonical paths,
    // and let the existing receiver logic fill them concretely.
    (0..count)
      .map(|i| {
        let path = format!("m/84'/{}'/0'/0/{}", Self::coin_type(), i);
        AddressRow::new(path, "") // address will be filled by caller if needed
      })
      .collect()
  }
}

fn encode_bech32(hrp: &str, data: &[u8]) -> Result<String, bech32::EncodeError> {
  bech32::encode::<Bech32>(Hrp::parse(hrp).unwrap(), data)
}

pub fn run(coin_name: &str, mnemonic: &Mnemonic, info: &[&str]) -> Result<(), Box<dyn Error>> {
  // 1) Escolher mainnet/testnet da Litecoin
  let (network, coin_type) = select_network()?;

  // 2) Passphrase opcional
  let passphrase = prompt_passphrase()?;
  let seed = mnemonic.to_seed(&passphrase);

  // 3) Derivação padrão BIP84: m/84'/coin_type'/0' usando o trait Chain.
  let account_path_str = format!("m/84'/{}'/0'", coin_type);
  // let account_path: DerivationPath = account_path_str.parse()?;
  let master_xprv = XPrv::new(&seed)?;
  let account_xprv = LtcChain::derive_account(&seed, 0);
  let account_xpub = LtcChain::public_from_private(&account_xprv);
  let fingerprint = XPub::from(&master_xprv).fingerprint();

  let script_type = "bip84";
  let derivation_path = account_path_str.clone();

  let go_continue = prompt_confirm_options()?;
  if go_continue == 1 {
    exit(0);
  }

  seedctl_core::ui::print_wallet_header(coin_name);

  print_mnemonic(
    mnemonic,
    &format!("BIP39 MNEMONIC ({} words):", mnemonic.word_count()),
  );

  let addresses = receive_addresses(&seed, &account_path_str, network, coin_type, 10)?;

  print_wallet_output(
    coin_type,
    &fingerprint,
    &account_xprv,
    &account_xpub,
    &addresses,
  );

  // Export watch-only (xpub + descriptors simples)
  let export = build_export(
    info,
    network,
    script_type,
    &derivation_path,
    &fingerprint,
    &account_xpub,
  );

  let json = to_string_pretty(&export).unwrap();
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

fn select_network() -> Result<(LtcNetwork, u32), Box<dyn Error>> {
  use dialoguer::Select;
  use seedctl_core::ui::dialoguer_theme;

  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Litecoin network:")
    .items(["Mainnet", "Testnet"])
    .default(0)
    .interact()?;

  Ok(match choice {
    // SLIP-44: 2' para Litecoin mainnet, 1' para testnets
    0 => (LtcNetwork::Mainnet, 2),
    1 => (LtcNetwork::Testnet, 1),
    _ => unreachable!(),
  })
}

fn receive_addresses(
  seed: &[u8],
  account_path_str: &str,
  network: LtcNetwork,
  coin_type: u32,
  count: u32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
  let mut out = Vec::with_capacity(count as usize);

  for i in 0..count {
    let full_path_str = format!("{}/0/{}", account_path_str, i);
    let full_path: DerivationPath = full_path_str.parse()?;
    let xprv = XPrv::derive_from_path(seed, &full_path)?;
    let privkey = xprv.private_key();

    // Deriva chave pública secp256k1 comprimida via k256
    let signing = k256::ecdsa::SigningKey::from_bytes(&privkey.to_bytes())?;
    let pubkey = signing.verifying_key().to_encoded_point(true);
    let pub_bytes = pubkey.as_bytes();

    let addr = ltc_bech32_p2wpkh(pub_bytes, network)?;

    out.push((format!("m/84'/{}'/0'/0/{}", coin_type, i), addr));
  }

  Ok(out)
}

fn ltc_bech32_p2wpkh(
  pubkey_compressed: &[u8],
  network: LtcNetwork,
) -> Result<String, Box<dyn Error>> {
  // HASH160(pubkey)
  let sha = Sha256::digest(pubkey_compressed);
  let rip = Ripemd160::digest(&sha);

  let mut program = Vec::with_capacity(1 + rip.len());
  // Versão 0 para P2WPKH
  program.push(0u8);
  program.extend_from_slice(&rip);

  let hrp = match network {
    LtcNetwork::Mainnet => "ltc",
    LtcNetwork::Testnet => "tltc",
  };

  let resul = encode_bech32(hrp, &program)?;

  Ok(resul)
}

fn build_export(
  info: &[&str],
  network: LtcNetwork,
  script_type: &str,
  derivation_path: &str,
  fingerprint: &[u8; 4],
  account_xpub: &XPub,
) -> export::WalletExport {
  let network_str = match network {
    LtcNetwork::Mainnet => "litecoin",
    LtcNetwork::Testnet => "litecoin-testnet",
  };

  export::WalletExport {
    software: export::SoftwareInfo {
      name: info[0].to_string(),
      version: info[1].to_string(),
      repository: info[2].to_string(),
    },
    network: network_str.to_string(),
    script_type: script_type.to_string(),
    key_origin: export::KeyOrigin {
      fingerprint: format_fingerprint_hex(fingerprint),
      derivation_path: derivation_path.to_string(),
    },
    watch_only: true,
    keys: export::Keys {
      // exporta xpub em hex, como no ETH
      account_xpub: hex::encode(account_xpub.to_bytes()),
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      receive: "ltc-bip84".into(),
      change: "ltc-bip84".into(),
    },
  }
}
fn print_wallet_output(
  coin_type: u32,
  fingerprint: &[u8; 4],
  account_xprv: &XPrv,
  account_xpub: &XPub,
  addresses: &[(String, String)],
) {

  println!(
    "\n{} {} {}",
    style("[PUBLIC] →").yellow().bold(),
    style("Fingerprint:").cyan().bold(),
    format_fingerprint_hex(fingerprint)
  );

  println!(
    "\n{} {} m/84'/{}'/0'",
    style("[PUBLIC] → ").bold().yellow(),
    style("Derivation path:").bold().cyan(),
    coin_type
  );

  println!(
    "\n{} {}\n{}",
    style("[SECRET] →").red().bold(),
    style("Account Private Key:").cyan().bold(),
    hex::encode(account_xprv.to_bytes())
  );

  println!(
    "\n{} {}\n{}",
    style("[PUBLIC] →").yellow().bold(),
    style("Account Public Key:").cyan().bold(),
    hex::encode(account_xpub.to_bytes())
  );

  println!(
    "\n{} {}",
    style("[PUBLIC] → ").bold().yellow(),
    style("Showing first 10 receive addresses: ").bold().cyan()
  );

  print_address_table(addresses);
}
