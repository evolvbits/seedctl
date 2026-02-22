use bip39::Mnemonic;
use console::style;
use seedctl_core::ui::{prompt_confirm_options, prompt_passphrase};
use seedctl_core::utils::print_mnemonic;
use std::{error::Error, process::exit};

/// Monero (XMR) ainda não está totalmente implementado aqui.
/// Esta crate apenas exibe o mnemonic e o seed em hex como base
/// para futuras integrações com bibliotecas específicas de Monero.
pub fn run(coin_name: &str, mnemonic: &Mnemonic, _info: &[&str]) -> Result<(), Box<dyn Error>> {
  let passphrase = prompt_passphrase()?;
  let seed = mnemonic.to_seed(&passphrase);

  let go = prompt_confirm_options()?;
  if go == 1 {
    exit(0);
  }

  seedctl_core::ui::print_wallet_header(coin_name);
  print_mnemonic(
    mnemonic,
    &format!("BIP39 MNEMONIC ({} words):", mnemonic.word_count()),
  );

  println!(
    "\n{} {}",
    style("[INFO] → ").bold().yellow(),
    style("Monero support is experimental / TODO.")
      .bold()
      .cyan()
  );
  println!(
    "{}",
    style("Below is the raw BIP39 seed (hex) that can be used with external Monero tooling:").dim()
  );
  println!(
    "\n{} {}",
    style("Seed (hex):").bold().cyan(),
    hex::encode(seed)
  );

  Ok(())
}
