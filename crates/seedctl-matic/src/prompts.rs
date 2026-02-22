use dialoguer::{Input, Select};
use seedctl_core::ui::dialoguer_theme;
use std::error::Error;

use crate::utils::DerivationStyle;

/// 0 = Generate addresses, 1 = Scan common derivation paths.
pub fn select_derivation_mode() -> Result<usize, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select derivation mode (Polygon):")
    .items(["Generate addresses", "Scan common derivation paths"])
    .default(0)
    .interact()?;
  Ok(choice)
}

pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  let n: u32 = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("How many Polygon addresses generate?")
    .default(10)
    .interact_text()?;
  Ok(n)
}

pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  crate::utils::select_derivation_style()
}

pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  let s: String = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("Polygon RPC URL (enter to skip balance check)")
    .allow_empty(true)
    .interact_text()?;
  Ok(s)
}
