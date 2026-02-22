use dialoguer::{Input, Select};
use seedctl_core::ui::dialoguer_theme;
use std::error::Error;

use crate::utils::DerivationStyle;

pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  let n: u32 = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("How many addresses to generate?")
    .default(10)
    .interact_text()?;
  Ok(n)
}

pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  crate::utils::select_derivation_style()
}

#[allow(dead_code)]
pub fn prompt_show_privkeys() -> Result<bool, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Show private keys?")
    .items(["No (recommended)", "Yes (dangerous)"])
    .default(0)
    .interact()?;
  Ok(choice == 1)
}
