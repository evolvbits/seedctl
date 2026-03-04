//! Interactive prompts for the Tron (TRX) chain crate.
//!
//! Provides user-facing [`dialoguer`] prompts used by [`crate::run`] to
//! collect Tron-specific configuration: address count, derivation style
//! selection, and an optional Tron node URL for balance queries.

use dialoguer::{Input, Select};
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

use crate::utils::DerivationStyle;

/// Prompts the user for the number of Tron addresses to generate.
///
/// Defaults to `10` if the user presses Enter without typing a value.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let n: u32 = Input::with_theme(&theme)
    .with_prompt("How many addresses to generate?")
    .default(10)
    .interact_text()?;
  Ok(n)
}

/// Prompts the user to choose a Tron derivation style.
///
/// Delegates to [`crate::utils::select_derivation_style`] which presents
/// three options:
///
/// - `Standard` — `m/44'/195'/0'/0/x` (compatible with TronLink)
/// - `Ledger`   — `m/44'/195'/0'/x/0` (Ledger Tron app)
/// - `Custom`   — user-supplied base path
pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  crate::utils::select_derivation_style()
}

/// Prompts the user for a Tron full-node API URL used for balance queries.
///
/// Returns an empty string immediately when [`RPC_URL_ENABLE`] is `false`
/// (the default), keeping the tool fully offline-capable out of the box.
///
/// When enabled, the user may press Enter without typing anything to skip
/// the balance check for this session.
///
/// The expected endpoint is the Tron full-node HTTP API, e.g.:
/// `https://api.trongrid.io`
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn prompt_rpc_url() -> Result<String, Box<dyn Error>> {
  if !RPC_URL_ENABLE {
    return Ok(String::new());
  }

  let theme = dialoguer_theme("►");
  let s: String = Input::with_theme(&theme)
    .with_prompt("Tron node URL (enter to skip balance check)")
    .allow_empty(true)
    .interact_text()?;
  Ok(s)
}

/// Prompts the user whether to display private keys in the wallet output.
///
/// Returns `true` if the user selects "Yes (dangerous)", `false` for the
/// default "No (recommended)" option.
///
/// This function is intentionally marked `#[allow(dead_code)]` because it
/// is currently disabled in [`crate::run`] — private keys are always shown
/// until a future release adds a configurable toggle.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
#[allow(dead_code)]
pub fn prompt_show_privkeys() -> Result<bool, Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let choice = Select::with_theme(&theme)
    .with_prompt("Show private keys?")
    .items(["No (recommended)", "Yes (dangerous)"])
    .default(0)
    .interact()?;
  Ok(choice == 1)
}
