use dialoguer::{Input, Select};
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

#[derive(Clone, Copy)]
pub enum LtcNetwork {
  Mainnet,
  Testnet,
}

/// Returns (network, coin_type): 0 = Mainnet, 1 = Testnet.
pub fn select_network() -> Result<(LtcNetwork, u32), Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Litecoin network:")
    .items(["Mainnet", "Testnet"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => (LtcNetwork::Mainnet, 2),
    1 => (LtcNetwork::Testnet, 1),
    _ => unreachable!(),
  })
}

pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  if !RPC_URL_ENABLE {
    return Ok(String::new());
  }

  let url: String = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("Litecoin RPC URL (enter to skip balance check)")
    .allow_empty(true)
    .interact_text()?;

  Ok(url)
}
