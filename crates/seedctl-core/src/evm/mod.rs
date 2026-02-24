use crate::{
  export,
  ui::{AddressRows, dialoguer_theme, print_standard_wallet},
};
use bip32::{ChildNumber, DerivationPath, XPrv};
use console::style;
use dialoguer::{Input, Select};
use k256::ecdsa::{SigningKey, VerifyingKey};
use sha3::{Digest, Keccak256};
use std::error::Error;

pub type EvmAccountXpub = bip32::ExtendedPublicKey<VerifyingKey>;

const EVM_STANDARD_PATH: &str = "m/44'/60'/0'/0";
const EVM_LEDGER_PATH: &str = "m/44'/60'/0'";

const ETHEREUM_SCAN_PATHS: &[&str] = &[
  "m/44'/60'/0'/0/0",
  "m/44'/60'/0'/0/1",
  "m/44'/60'/1'/0/0",
  "m/44'/60'/0'/1/0",
  "m/44'/60'/0'/0/5",
  "m/44'/60'/0'",
  "m/44'/60'/1'",
];

const POLYGON_SCAN_PATHS: &[&str] = &[
  "m/44'/60'/0'/0/0",
  "m/44'/60'/0'/0/1",
  "m/44'/60'/0'/1/0",
  "m/44'/60'/1'/0/0",
];

#[derive(Clone, Copy)]
pub struct EvmProfile {
  pub name: &'static str,
  pub wallet_title: &'static str,
  pub derivation_mode_prompt: &'static str,
  pub derivation_style_prompt: &'static str,
  pub address_count_prompt: &'static str,
  pub rpc_prompt: &'static str,
  pub scan_title: &'static str,
  pub scan_tip: Option<&'static str>,
  pub export_network: &'static str,
  pub export_script_type: &'static str,
  pub export_descriptor: &'static str,
  pub export_file_prefix: &'static str,
  pub scan_paths: &'static [&'static str],
}

pub const ETHEREUM_PROFILE: EvmProfile = EvmProfile {
  name: "Ethereum",
  wallet_title: "Ethereum Wallet",
  derivation_mode_prompt: "Select derivation mode:",
  derivation_style_prompt: "Select derivation style:",
  address_count_prompt: "How many addresses generate?",
  rpc_prompt: "RPC URL (enter to skip balance check)",
  scan_title: "🔎 Automatic derivation path scanner",
  scan_tip: Some("Tip: compare with your wallet known address to find correct path."),
  export_network: "ethereum",
  export_script_type: "ethereum-bip44",
  export_descriptor: "ethereum-address",
  export_file_prefix: "eth",
  scan_paths: ETHEREUM_SCAN_PATHS,
};

pub const POLYGON_PROFILE: EvmProfile = EvmProfile {
  name: "Polygon",
  wallet_title: "Polygon Wallet",
  derivation_mode_prompt: "Select derivation mode (Polygon):",
  derivation_style_prompt: "Select derivation style (Polygon):",
  address_count_prompt: "How many Polygon addresses generate?",
  rpc_prompt: "Polygon RPC URL (enter to skip balance check)",
  scan_title: "Scanning common Polygon derivation paths:",
  scan_tip: None,
  export_network: "polygon",
  export_script_type: "polygon-evm-bip44",
  export_descriptor: "polygon-address",
  export_file_prefix: "matic",
  scan_paths: POLYGON_SCAN_PATHS,
};

#[derive(Clone)]
pub enum DerivationStyle {
  Standard,
  Ledger,
  Custom(String),
}

pub struct WalletOutput<'a> {
  pub purpose: u32,
  pub coin_type: u32,
  pub account_xprv: &'a str,
  pub account_xpub: &'a str,
  pub show_privkeys: bool,
  pub addresses: &'a [(String, String, Option<f64>)],
}

pub struct RpcClient {
  url: String,
  client: reqwest::blocking::Client,
}

impl RpcClient {
  pub fn new(url: impl Into<String>) -> Self {
    Self {
      url: url.into(),
      client: reqwest::blocking::Client::new(),
    }
  }

  pub fn get_balance(&self, address: &str) -> Option<f64> {
    let body = serde_json::json!({
      "jsonrpc": "2.0",
      "method": "eth_getBalance",
      "params": [address, "latest"],
      "id": 1,
    });

    let res = self.client.post(&self.url).json(&body).send().ok()?;
    let payload: serde_json::Value = res.json().ok()?;
    let hex = payload.get("result")?.as_str()?;

    let wei = u128::from_str_radix(hex.trim_start_matches("0x"), 16).ok()?;
    Some(wei as f64 / 1e18)
  }
}

pub fn get_balance(url: &str, address: &str) -> Option<f64> {
  RpcClient::new(url).get_balance(address)
}

pub fn select_derivation_mode(profile: &EvmProfile) -> Result<usize, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt(profile.derivation_mode_prompt)
    .items(["Generate addresses", "Scan common derivation paths"])
    .default(0)
    .interact()?;

  Ok(choice)
}

pub fn prompt_address_count(profile: &EvmProfile) -> Result<u32, Box<dyn Error>> {
  let count: u32 = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt(profile.address_count_prompt)
    .default(10)
    .interact_text()?;

  Ok(count)
}

pub fn prompt_rpc_url(profile: &EvmProfile) -> Result<String, Box<dyn Error>> {
  let url: String = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt(profile.rpc_prompt)
    .allow_empty(true)
    .interact_text()?;

  Ok(url)
}

pub fn select_derivation_style(profile: &EvmProfile) -> Result<DerivationStyle, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt(profile.derivation_style_prompt)
    .items(["Standard (m/44'/60'/0'/0/x)", "Ledger style", "Custom path"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => DerivationStyle::Standard,
    1 => DerivationStyle::Ledger,
    2 => {
      let input: String = Input::with_theme(&dialoguer_theme("►"))
        .with_prompt("Enter custom derivation base path")
        .default(EVM_STANDARD_PATH.into())
        .interact_text()?;
      DerivationStyle::Custom(input)
    }
    _ => unreachable!(),
  })
}

pub fn style_to_string(style: &DerivationStyle) -> String {
  match style {
    DerivationStyle::Standard => EVM_STANDARD_PATH.into(),
    DerivationStyle::Ledger => EVM_LEDGER_PATH.into(),
    DerivationStyle::Custom(custom) => custom.clone(),
  }
}

pub fn build_path(style: &DerivationStyle, index: u32) -> Result<DerivationPath, Box<dyn Error>> {
  let path_str = match style {
    DerivationStyle::Standard => format!("{}/{}", EVM_STANDARD_PATH, index),
    DerivationStyle::Ledger => format!("m/44'/60'/{}'/0/0", index),
    DerivationStyle::Custom(template) => {
      if template.contains("{index}") {
        template.replace("{index}", &index.to_string())
      } else if template.ends_with('/') {
        format!("{}{}", template, index)
      } else {
        template.clone()
      }
    }
  };

  Ok(path_str.parse()?)
}

pub fn derive_path(mut key: XPrv, path: &DerivationPath) -> Result<XPrv, Box<dyn Error>> {
  for child in path.iter() {
    key = key.derive_child(child)?;
  }

  Ok(key)
}

pub fn derive_from_path(master: XPrv, path: &str) -> Result<XPrv, Box<dyn Error>> {
  let derivation_path: DerivationPath = path.parse()?;
  derive_path(master, &derivation_path)
}

pub fn derive_address_key(
  master: &XPrv,
  account_xprv: &XPrv,
  style: &DerivationStyle,
  index: u32,
) -> Result<(XPrv, String), Box<dyn Error>> {
  let path = build_path(style, index)?;
  let path_str = path.to_string();

  let key = match style {
    DerivationStyle::Standard => {
      let child = ChildNumber::new(index, false)?;
      account_xprv.clone().derive_child(child)?
    }
    DerivationStyle::Ledger | DerivationStyle::Custom(_) => derive_path(master.clone(), &path)?,
  };

  Ok((key, path_str))
}

pub fn to_checksum_address(addr: &[u8]) -> String {
  let hex_addr = hex::encode(addr);
  let hash = Keccak256::digest(hex_addr.as_bytes());

  let mut out = String::from("0x");

  for (idx, ch) in hex_addr.chars().enumerate() {
    let hash_byte = hash[idx / 2];
    let nibble = if idx % 2 == 0 {
      hash_byte >> 4
    } else {
      hash_byte & 0x0f
    };

    if ch.is_ascii_digit() {
      out.push(ch);
    } else if nibble >= 8 {
      out.push(ch.to_ascii_uppercase());
    } else {
      out.push(ch);
    }
  }

  out
}

pub fn address_from_xprv(xprv: XPrv) -> Result<String, Box<dyn Error>> {
  let pk = xprv.private_key().to_bytes();
  let signing = SigningKey::from_bytes(&pk)?;
  let pubkey = signing.verifying_key().to_encoded_point(false);

  let hash = Keccak256::digest(&pubkey.as_bytes()[1..]);
  let addr_bytes = &hash[12..];

  Ok(to_checksum_address(addr_bytes))
}

pub fn scan_common_paths(master: XPrv, profile: &EvmProfile) -> Result<(), Box<dyn Error>> {
  println!(
    "\n{} {}\n",
    style("🔎").cyan().bold(),
    style(profile.scan_title).cyan().bold(),
  );

  for path_str in profile.scan_paths.iter() {
    let path: DerivationPath = path_str.parse()?;
    let child = derive_path(master.clone(), &path)?;
    let address = address_from_xprv(child)?;
    println!("{:<22} → {}", path_str, address);
  }

  if let Some(tip) = profile.scan_tip {
    println!("\n{}", style(tip).yellow().bold());
  }

  Ok(())
}

pub fn print_wallet_output(profile: &EvmProfile, output: &WalletOutput<'_>) {
  let account_xprv_opt = if output.show_privkeys {
    Some(output.account_xprv)
  } else {
    None
  };

  print_standard_wallet(
    profile.wallet_title,
    output.purpose,
    output.coin_type,
    None,
    account_xprv_opt,
    output.account_xpub,
    AddressRows::WithBalance(output.addresses),
    vec![],
  );
}

pub fn build_watch_only_export(
  profile: &EvmProfile,
  info: &[&str],
  base_path: &str,
  xpub: EvmAccountXpub,
) -> export::WalletExport {
  let xpub_bytes = xpub.to_bytes();

  export::WalletExport {
    software: export::SoftwareInfo {
      name: info.get(0).copied().unwrap_or("seedctl").to_string(),
      version: info.get(1).copied().unwrap_or("unknown").to_string(),
      repository: info.get(2).copied().unwrap_or("unknown").to_string(),
    },
    network: profile.export_network.into(),
    script_type: profile.export_script_type.into(),
    key_origin: export::KeyOrigin {
      fingerprint: hex::encode(&xpub_bytes[0..4]),
      derivation_path: base_path.into(),
    },
    watch_only: true,
    keys: export::Keys {
      account_xpub: hex::encode(&xpub_bytes),
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      receive: profile.export_descriptor.into(),
      change: profile.export_descriptor.into(),
    },
  }
}
