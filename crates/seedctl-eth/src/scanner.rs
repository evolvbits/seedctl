use bip32::{DerivationPath, XPrv};
use console::style;
use std::error::Error;

pub fn scan_common_paths(master: XPrv) -> Result<(), Box<dyn Error>> {
  println!(
    "\n{} {}\n",
    style("🔎 Automatic derivation path scanner").cyan().bold(),
    style("(common wallet standards)").dim()
  );

  let paths = vec![
    "m/44'/60'/0'/0/0",
    "m/44'/60'/0'/0/1",
    "m/44'/60'/1'/0/0",
    "m/44'/60'/0'/1/0",
    "m/44'/60'/0'/0/5",
    "m/44'/60'/0'",
    "m/44'/60'/1'",
  ];

  for p in paths {
    let path: DerivationPath = p.parse()?;
    let child = crate::utils::derive_path(master.clone(), &path)?;
    let addr = crate::derive::address_from_xprv(child)?;
    println!("{:<22} → {}", p, addr);
  }

  println!(
    "\n{}",
    style("Tip: compare with your wallet known address to find correct path.")
      .yellow()
      .bold()
  );

  Ok(())
}
