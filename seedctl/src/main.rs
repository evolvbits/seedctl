mod utils;

use bip39::Mnemonic;
use dialoguer::{Input, Select};
use seedctl_core::{
  args,
  entropy::{print_entropy_mode, resolve_final_entropy},
  options::entropy_type,
  security::Security,
  ui::{dialoguer_theme, exit_confirm},
  utils::dice_hash,
};
use std::error::Error;

use crate::utils::{connection::Connection, copyright_phrase, meta};
use console::style;

fn main() -> Result<(), Box<dyn Error>> {
  match args::parse_args() {
    args::CliAction::Version => {
      utils::args::print_version();
      return Ok(());
    }
    args::CliAction::About => {
      utils::args::print_about();
      return Ok(());
    }
    args::CliAction::Run => {
      // Clear screen
      let term = console::Term::stdout();
      term.clear_screen()?;

      // Check connection internet. If connection = closed
      // Connection::check();

      // Show warning security
      // let security = Security;
      // security.warning("I UNDERSTOOD")?;

      // Show slogan
      println!("{}", style("\n:: Welcome to").bold());
      utils::slogan::slogan_view(true, true);

      // 0) Choose between generating a new wallet or importing an existing seed.
      let action = Select::with_theme(&dialoguer_theme("►"))
        .with_prompt("Choose action:")
        .items(["Create new wallet", "Import existing wallet"])
        .default(0)
        .interact()
        .unwrap();

      // 1) Get the mnemonic (new or imported)
      let mnemonic = match action {
        0 => {
          // Generate a new universal BIP39 seed from entropy.
          let entropy_type = entropy_type()?;
          let dice_entropy = dice_hash(&entropy_type.1);
          let dice_mode = entropy_type.2;
          print_entropy_mode(dice_mode);
          let final_entropy = resolve_final_entropy(entropy_type, dice_entropy);
          Mnemonic::from_entropy(&final_entropy).expect("failed to build mnemonic from entropy")
        }
        1 => loop {
          let phrase: String = Input::with_theme(&dialoguer_theme("►"))
            .with_prompt("Enter existing BIP39 seed phrase (12/24 words)")
            .interact_text()?;

          match Mnemonic::parse_normalized(phrase.trim()) {
            Ok(m) => break m,
            Err(e) => {
              eprintln!("Invalid mnemonic: {e}. Please try again.\n");
            }
          }
        },
        _ => unreachable!(),
      };

      // 2) Choose the network/currency
      let network = Select::with_theme(&dialoguer_theme("►"))
        .with_prompt("Select network:")
        .items([
          "Bitcoin",
          "Ethereum (ETH + ERC20 tokens)",
          "Tron (TRX + TRC20 tokens)",
          "Solana (SOL + SPL tokens)",
          "Litecoin (LTC)",
          "Polygon (MATIC, EVM)",
          // "Cardano (ADA) [experimental]",
          // "Monero (XMR) [experimental]",
        ])
        .default(0)
        .interact()
        .unwrap();

      let info = &[meta::PROJECT_NAME, meta::VERSION, meta::PROJECT_REPOSITORY];

      match network {
        0 => seedctl_btc::run("Bitcoin (BTC)", &mnemonic, info)?,
        1 => seedctl_eth::run("Ethereum (ETH)", &mnemonic, info)?,
        2 => seedctl_trx::run("TRON (TRX)", &mnemonic, info)?,
        3 => seedctl_sol::run("Solana (SOL)", &mnemonic, info)?,
        4 => seedctl_ltc::run("Litecoin (LTC)", &mnemonic, info)?,
        5 => seedctl_matic::run("Polygon (POL)", &mnemonic, info)?,
        // 6 => seedctl_ada::run("Cardano (ADA)", &mnemonic, info)?,
        // 7 => seedctl_xmr::run("Monero (XMR)", &mnemonic, info)?,
        _ => unreachable!(),
      };
    }
  }

  copyright_phrase();
  exit_confirm();

  Ok(())
}
