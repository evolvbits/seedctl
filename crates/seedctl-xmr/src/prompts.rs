use dialoguer::{Input, Select};
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

#[derive(Clone, Copy)]
pub enum XmrNetwork {
  Mainnet,
  Testnet,
}

impl XmrNetwork {
  pub fn standard_prefix(self) -> u8 {
    match self {
      Self::Mainnet => 18,
      Self::Testnet => 53,
    }
  }

  pub fn subaddress_prefix(self) -> u8 {
    match self {
      Self::Mainnet => 42,
      Self::Testnet => 63,
    }
  }

  pub fn export_network(self) -> &'static str {
    match self {
      Self::Mainnet => "monero-mainnet",
      Self::Testnet => "monero-testnet",
    }
  }
}

pub fn select_network() -> Result<XmrNetwork, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select Monero network:")
    .items(["Mainnet (4...)", "Testnet (9...)"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => XmrNetwork::Mainnet,
    1 => XmrNetwork::Testnet,
    _ => unreachable!(),
  })
}

pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  let count: u32 = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("How many Monero addresses/subaddresses to generate?")
    .default(10)
    .interact_text()?;

  Ok(count)
}

pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  if !RPC_URL_ENABLE {
    return Ok(String::new());
  }

  let s: String = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("Monero wallet-rpc URL (enter to skip balance check)")
    .allow_empty(true)
    .interact_text()?;
  Ok(s)
}
