use bip32::{ChildNumber, DerivationPath, XPrv};
use dialoguer::{Input, Select};
use seedctl_core::ui::dialoguer_theme;
use sha3::{Digest, Keccak256};
use std::error::Error;

pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select derivation style (Polygon):")
    .items(["Standard (m/44'/60'/0'/0/x)", "Ledger style", "Custom path"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => DerivationStyle::Standard,
    1 => DerivationStyle::Ledger,
    2 => {
      let input: String = Input::with_theme(&dialoguer_theme("►"))
        .with_prompt("Enter custom derivation base path")
        .default("m/44'/60'/0'/0".into())
        .interact_text()?;
      DerivationStyle::Custom(input)
    }
    _ => unreachable!(),
  })
}

pub fn style_to_string(style: &DerivationStyle) -> String {
  match style {
    DerivationStyle::Standard => "m/44'/60'/0'/0".into(),
    DerivationStyle::Ledger => "m/44'/60'/0'".into(),
    DerivationStyle::Custom(s) => s.clone(),
  }
}

/// Converte endereço para formato checksum EIP-55
pub fn to_checksum_address(addr: &[u8]) -> String {
  let hex_addr = hex::encode(addr);
  let hash = Keccak256::digest(hex_addr.as_bytes());

  let mut result = String::from("0x");

  for (i, c) in hex_addr.chars().enumerate() {
    let hash_byte = hash[i / 2];
    let nibble = if i % 2 == 0 {
      hash_byte >> 4
    } else {
      hash_byte & 0x0f
    };

    if c.is_ascii_digit() {
      result.push(c);
    } else if nibble >= 8 {
      result.push(c.to_ascii_uppercase());
    } else {
      result.push(c);
    }
  }

  result
}

/// Deriva caminho BIP32 manualmente
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

/// Deriva a chave do endereço no índice 'index'.
/// Standard/Custom com base em m/44'/60'/0'/0: deriva só o filho 'index' a partir da chave de conta.
/// Ledger/Custom com conta por índice: deriva o path completo a partir do master.
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

/// Constrói derivation path baseado no estilo
pub fn build_path(style: &DerivationStyle, index: u32) -> Result<DerivationPath, Box<dyn Error>> {
  let path_str = match style {
    DerivationStyle::Standard => {
      format!("m/44'/60'/0'/0/{}", index)
    }

    DerivationStyle::Ledger => {
      format!("m/44'/60'/{}'/0/0", index)
    }

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

/// Estilos suportados
#[derive(Clone)]
pub enum DerivationStyle {
  Standard,
  Ledger,
  Custom(String),
}
