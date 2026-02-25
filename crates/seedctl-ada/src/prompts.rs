use dialoguer::{Input, Select};
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

#[derive(Clone, Copy)]
pub enum AdaNetwork {
  Mainnet,
  Testnet,
}

impl AdaNetwork {
  pub fn hrp(self) -> &'static str {
    match self {
      Self::Mainnet => "addr",
      Self::Testnet => "addr_test",
    }
  }

  pub fn network_id(self) -> u8 {
    match self {
      Self::Mainnet => 1,
      Self::Testnet => 0,
    }
  }

  pub fn base_header(self) -> u8 {
    // Shelley Base Address: 0000 + network_id
    self.network_id()
  }

  pub fn export_network(self) -> &'static str {
    match self {
      Self::Mainnet => "cardano-mainnet",
      Self::Testnet => "cardano-testnet",
    }
  }
}

pub fn select_network() -> Result<AdaNetwork, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select Cardano network:")
    .items(["Mainnet (addr...)", "Testnet (addr_test...)"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => AdaNetwork::Mainnet,
    1 => AdaNetwork::Testnet,
    _ => unreachable!(),
  })
}

pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  let count: u32 = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("How many Cardano addresses to generate?")
    .default(10)
    .interact_text()?;

  Ok(count)
}

pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  if !RPC_URL_ENABLE {
    return Ok(String::new());
  }

  let s: String = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("Cardano address API URL (Koios /address_info, enter to skip)")
    .allow_empty(true)
    .interact_text()?;
  Ok(s)
}
