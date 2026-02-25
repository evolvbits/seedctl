use bitcoin::Network;
use dialoguer::{Input, Select};
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

/// Retorna (Network, coin_type): 0 = Mainnet, 1 = Testnet.
pub fn select_network() -> Result<(Network, u32), Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Bitcoin network:")
    .items(["Mainnet", "Testnet (test address)"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => (Network::Bitcoin, 0),
    1 => (Network::Testnet, 1),
    _ => unreachable!(),
  })
}

/// Retorna (purpose, address_type index). purpose: 84, 49 ou 44.
pub fn select_address_type() -> Result<(u32, usize), Box<dyn Error>> {
  let address_type = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Bitcoin derivation path (BIP purpose):")
    .items([
      "BIP84 (Native SegWit, recommended)",
      "BIP49 (Nested SegWit)",
      "BIP44 (Legacy)",
    ])
    .default(0)
    .interact()?;

  let purpose = match address_type {
    0 => 84,
    1 => 49,
    2 => 44,
    _ => unreachable!(),
  };
  Ok((purpose, address_type))
}

pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  if !RPC_URL_ENABLE {
    return Ok(String::new());
  }

  let url: String = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("Bitcoin RPC URL (enter to skip balance check)")
    .allow_empty(true)
    .interact_text()?;

  Ok(url)
}
