use dialoguer::{Input, Select};
use seedctl_core::ui::dialoguer_theme;
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
