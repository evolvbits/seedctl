use dialoguer::{Input, Select};
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

#[derive(Clone, Copy)]
pub enum XrpNetwork {
  Mainnet,
  Testnet,
}

impl XrpNetwork {
  pub fn export_network(self) -> &'static str {
    match self {
      Self::Mainnet => "xrpl-mainnet",
      Self::Testnet => "xrpl-testnet",
    }
  }
}

pub fn select_network() -> Result<XrpNetwork, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select XRP network:")
    .items(["Mainnet", "Testnet"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => XrpNetwork::Mainnet,
    1 => XrpNetwork::Testnet,
    _ => unreachable!(),
  })
}

pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  let n: u32 = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("How many XRP addresses to generate?")
    .default(10)
    .interact_text()?;

  Ok(n)
}

pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  if !RPC_URL_ENABLE {
    return Ok(String::new());
  }

  let s: String = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("XRPL RPC URL (enter to skip balance check)")
    .allow_empty(true)
    .interact_text()?;
  Ok(s)
}
