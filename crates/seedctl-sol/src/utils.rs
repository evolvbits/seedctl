//! Solana address encoding and interactive prompt utilities.
//!
//! Provides:
//! - [`derive_seed`]         — SLIP-0010 Ed25519 key derivation for a given account index.
//! - [`pubkey_to_address`]   — encodes a 32-byte Ed25519 public key as a base58 Solana address.
//! - [`prompt_address_count`] — asks the user how many addresses to generate.
//! - [`prompt_rpc_url`]      — optionally asks for a Solana RPC URL for balance queries.
//! - [`prompt_show_privkeys`] — optionally asks whether to display private keys (currently unused).

use dialoguer::Input;
use ed25519_hd_key::derive_from_path;
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

/// Derives a 32-byte Ed25519 private scalar for the given account `index`
/// using BIP-32 SLIP-0010 with the path `m/44'/501'/<index>'/0'`.
///
/// This path is compatible with Phantom, Solflare, and the Solana CLI.
///
/// # Parameters
///
/// - `seed`  — 64-byte BIP-39 seed produced by `Mnemonic::to_seed(passphrase)`.
/// - `index` — account index that maps to the third path component (`<index>'`).
///
/// # Returns
///
/// A 32-byte array containing the derived Ed25519 private scalar.
///
/// # Errors
///
/// Returns a boxed error if the path string is malformed (should never happen
/// given the hardcoded path template) or if the underlying SLIP-0010
/// derivation fails.
pub fn derive_seed(seed: &[u8], index: u32) -> Result<[u8; 32], Box<dyn Error>> {
  // Path m/44'/501'/<index>'/0' is fully hardened, as required by SLIP-0010
  // for Ed25519 keys (unhardened Ed25519 derivation is not defined).
  let path = format!("m/44'/501'/{}'/0'", index);
  let (private_key, _chain_code) = derive_from_path(&path, seed);
  Ok(private_key)
}

/// Encodes a 32-byte Ed25519 verifying (public) key as a Solana base58 address.
///
/// In Solana, a wallet address IS the base58 representation of the 32-byte
/// Ed25519 public key — there is no additional hashing or versioning step.
///
/// # Parameters
///
/// - `pubkey` — 32-byte Ed25519 verifying key produced by
///   [`ed25519_dalek::SigningKey::verifying_key`].
///
/// # Returns
///
/// The base58-encoded address string (typically 43–44 characters long).
pub fn pubkey_to_address(pubkey: &[u8; 32]) -> String {
  bs58::encode(pubkey).into_string()
}

/// Prompts the user for the number of Solana addresses to generate.
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

/// Prompts the user for a Solana RPC URL used to query address balances.
///
/// Returns an empty string immediately when [`RPC_URL_ENABLE`] is `false`
/// (the default), keeping the tool fully offline-capable out of the box.
///
/// When enabled, the user may press Enter without typing anything to skip
/// the balance check for this session.
///
/// The expected endpoint is a standard Solana JSON-RPC node, e.g.:
/// `https://api.mainnet-beta.solana.com`
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
    .with_prompt("Solana RPC URL (enter to skip balance check)")
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
  use dialoguer::Select;
  let theme = dialoguer_theme("►");
  let choice = Select::with_theme(&theme)
    .with_prompt("Show private keys?")
    .items(["No (recommended)", "Yes (dangerous)"])
    .default(0)
    .interact()?;
  Ok(choice == 1)
}
