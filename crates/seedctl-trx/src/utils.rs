use bip32::{ChildNumber, DerivationPath, XPrv};
use dialoguer::{Input, Select};
use seedctl_core::ui::dialoguer_theme;
use std::error::Error;

const TRON_PATH_STANDARD: &str = "m/44'/195'/0'/0";
const TRON_PATH_LEDGER: &str = "m/44'/195'/0'";

pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select derivation style:")
    .items([
      "Standard (m/44'/195'/0'/0/x)",
      "Ledger style (m/44'/195'/0'/x/0)",
      "Custom path",
    ])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => DerivationStyle::Standard,
    1 => DerivationStyle::Ledger,
    2 => {
      let input: String = Input::with_theme(&dialoguer_theme("►"))
        .with_prompt("Enter custom derivation base path")
        .default(TRON_PATH_STANDARD.into())
        .interact_text()?;
      DerivationStyle::Custom(input)
    }
    _ => unreachable!(),
  })
}

pub fn style_to_string(style: &DerivationStyle) -> String {
  match style {
    DerivationStyle::Standard => TRON_PATH_STANDARD.into(),
    DerivationStyle::Ledger => TRON_PATH_LEDGER.into(),
    DerivationStyle::Custom(s) => s.clone(),
  }
}

/// Codifica endereço Tron: 0x41 + 20 bytes -> base58check (prefixo T).
pub fn to_tron_address(addr_20: &[u8]) -> String {
  let mut payload = vec![0x41u8];
  payload.extend_from_slice(addr_20);
  bs58::encode(payload).with_check().into_string()
}

pub fn derive_path(mut key: XPrv, path: &DerivationPath) -> Result<XPrv, Box<dyn Error>> {
  for c in path.iter() {
    key = key.derive_child(c)?;
  }
  Ok(key)
}

pub fn derive_from_path(master: XPrv, path: &str) -> Result<XPrv, Box<dyn Error>> {
  let dp: DerivationPath = path.parse()?;
  derive_path(master, &dp)
}

// Deriva a chave do endereço no índice index.
// Standard: deriva só o filho `index` a partir da chave de conta (m/44'/195'/0'/0).
// Ledger: deriva o path completo m/44'/195'/0'/index'/0/0 a partir do master.
// Custom: deriva o path completo a partir do master.
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
    DerivationStyle::Ledger => derive_path(master.clone(), &path)?,
    DerivationStyle::Custom(_) => derive_path(master.clone(), &path)?,
  };
  Ok((key, path_str))
}

pub fn build_path(style: &DerivationStyle, index: u32) -> Result<DerivationPath, Box<dyn Error>> {
  let path_str = match style {
    DerivationStyle::Standard => format!("{}/{}", TRON_PATH_STANDARD, index),
    DerivationStyle::Ledger => format!("{}/{}'/0/0", TRON_PATH_LEDGER, index),
    DerivationStyle::Custom(template) => {
      if template.contains("{index}") {
        template.replace("{index}", &index.to_string())
      } else if template.ends_with('/') {
        format!("{}{}", template, index)
      } else {
        format!("{}/{}", template, index)
      }
    }
  };
  Ok(path_str.parse()?)
}

#[derive(Clone)]
pub enum DerivationStyle {
  Standard,
  Ledger,
  Custom(String),
}
