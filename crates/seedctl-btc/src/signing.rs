//! Offline Bitcoin PSBT signing.
//!
//! Uses `rust-bitcoin`'s PSBT signer with the BIP-32 master private key derived
//! from the seed. The incoming PSBT must include key origin metadata such as
//! `bip32_derivation`/Taproot origins so matching inputs can be signed.

use bip39::Mnemonic;
use bitcoin::{key::Secp256k1, psbt::Psbt};
use console::style;
use dialoguer::{Input, Select};
use seedctl_core::{
  ui::{dialoguer_theme, prompt_passphrase},
  utils::format_fingerprint_hex,
};
use std::{error::Error, fs, path::Path, str::FromStr};

/// Runs the interactive offline Bitcoin PSBT signing workflow.
pub fn sign_offline(mnemonic: &Mnemonic) -> Result<(), Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let (network, _) = crate::prompts::select_network()?;
  let passphrase = prompt_passphrase()?;
  let master = crate::wallet::master_from_mnemonic(mnemonic, &passphrase, network)?;

  let source = Select::with_theme(&theme)
    .with_prompt("PSBT input source:")
    .items(["Paste PSBT text (base64 or hex)", "Read .psbt file"])
    .default(0)
    .interact()?;

  let psbt_text = match source {
    0 => Input::<String>::with_theme(&theme)
      .with_prompt("Unsigned PSBT")
      .interact_text()?,
    1 => {
      let path: String = Input::with_theme(&theme)
        .with_prompt("Path to .psbt file")
        .interact_text()?;
      read_psbt_file(&path)?
    }
    _ => unreachable!(),
  };

  let signed = sign_psbt_text(&psbt_text, &master)?;
  let secp = Secp256k1::new();
  let fingerprint = master.fingerprint(&secp);

  println!();
  println!(
    "{} {}",
    style("Master fingerprint:").bold().yellow(),
    style(format_fingerprint_hex(&[
      fingerprint[0],
      fingerprint[1],
      fingerprint[2],
      fingerprint[3]
    ]))
    .bold()
  );
  println!("{}", style("SIGNED PSBT (base64):").bold().green());
  println!("{}", signed);
  println!();
  println!("{}", style("SIGNED PSBT (hex):").bold().green());
  println!("{}", signed.serialize_hex());

  Ok(())
}

/// Signs a PSBT text payload and returns the updated PSBT.
pub fn sign_psbt_text(
  psbt_text: &str,
  master: &bitcoin::bip32::Xpriv,
) -> Result<Psbt, Box<dyn Error>> {
  let mut psbt = parse_psbt_text(psbt_text)?;
  let secp = Secp256k1::new();

  match psbt.sign(master, &secp) {
    Ok(used) => {
      if used.is_empty() {
        return Err("PSBT was parsed, but no matching key origins were found to sign".into());
      }
    }
    Err((used, errors)) => {
      if used.is_empty() {
        return Err(format!("unable to sign PSBT inputs: {errors:?}").into());
      }

      eprintln!(
        "{} {:?}",
        style("PSBT partially signed; some inputs returned errors:")
          .bold()
          .yellow(),
        errors
      );
    }
  }

  Ok(psbt)
}

fn parse_psbt_text(psbt_text: &str) -> Result<Psbt, Box<dyn Error>> {
  let cleaned = psbt_text.trim();

  if cleaned.starts_with("70736274ff") || cleaned.starts_with("0x70736274ff") {
    let hex = cleaned.trim_start_matches("0x").trim_start_matches("0X");
    return Ok(Psbt::deserialize(&hex::decode(hex)?)?);
  }

  Ok(Psbt::from_str(cleaned)?)
}

fn read_psbt_file(path: &str) -> Result<String, Box<dyn Error>> {
  let bytes = fs::read(Path::new(path))?;

  match String::from_utf8(bytes.clone()) {
    Ok(text) if !text.trim().is_empty() => Ok(text),
    _ => Ok(format!("0x{}", hex::encode(bytes))),
  }
}
