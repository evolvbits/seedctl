use bip32::{DerivationPath, XPrv};
use console::style;
use std::error::Error;

use crate::derive::address_from_xprv;
use crate::utils::derive_path;

/// Faz um scan simples em alguns paths comuns para Polygon (mesma lógica do Ethereum).
pub fn scan_common_paths(master: XPrv) -> Result<(), Box<dyn Error>> {
  let paths = [
    "m/44'/60'/0'/0/0",
    "m/44'/60'/0'/0/1",
    "m/44'/60'/0'/1/0",
    "m/44'/60'/1'/0/0",
  ];

  println!(
    "{} {}",
    style("[SCAN] →").bold().yellow(),
    style("Scanning common Polygon derivation paths:")
      .bold()
      .cyan()
  );

  for p in paths.iter() {
    let dp: DerivationPath = p.parse()?;
    let xprv = derive_path(master.clone(), &dp)?;
    let addr = address_from_xprv(xprv)?;
    println!("{} → {}", p, addr);
  }

  Ok(())
}
