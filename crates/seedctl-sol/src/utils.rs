//! Solana address encoding and interactive prompt utilities.
//!
//! Provides:
//! - [`derive_seed`]         — SLIP-0010 Ed25519 key derivation for a given account index.
//! - [`pubkey_to_address`]   — encodes a 32-byte Ed25519 public key as a base58 Solana address.
//! - [`prompt_address_count`] — asks the user how many addresses to generate.
//! - [`prompt_rpc_url`]      — optionally asks for a Solana RPC URL for balance queries.
//! - [`prompt_show_privkeys`] — optionally asks whether to display private keys (currently unused).

use dialoguer::{Input, Select};
use ed25519_hd_key::derive_from_path;
use seedctl_core::{constants::RPC_URL_ENABLE, ui::dialoguer_theme};
use std::error::Error;

/// Standard Solana path used by Phantom, Solflare, and Solana CLI.
const SOL_STANDARD_PATH: &str = "m/44'/501'/{index}'/0'";

/// WalletCore-style path where account index is the last hardened component.
const SOL_WALLETCORE_PATH: &str = "m/44'/501'/0'/0'/{index}'";

/// Legacy shorter path used by some old Solana tooling.
const SOL_LEGACY_PATH: &str = "m/44'/501'/{index}'";

/// Common Solana derivation path candidates for scanner mode.
const SOL_SCAN_PATHS: &[&str] = &[
  "m/44'/501'/0'/0'",
  "m/44'/501'/1'/0'",
  "m/44'/501'/0'",
  "m/44'/501'/1'",
  "m/44'/501'/0'/0'/0'",
  "m/44'/501'/0'/0'/1'",
  "m/44'/501'/0'/1'",
];

/// Supported derivation path styles for Solana wallets.
#[derive(Clone)]
pub enum DerivationStyle {
  /// `m/44'/501'/<index>'/0'` (Phantom / Solflare / Solana CLI).
  Standard,
  /// `m/44'/501'/0'/0'/<index>'` (WalletCore-compatible profile).
  WalletCore,
  /// `m/44'/501'/<index>'` (legacy profile).
  Legacy,
  /// Custom path template. `{index}` placeholder is supported.
  Custom(String),
}

/// Prompts the user to choose between generating addresses and scanning common
/// derivation paths.
pub fn select_derivation_mode() -> Result<usize, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select derivation mode (Solana):")
    .items(["Generate addresses", "Scan common derivation paths"])
    .default(0)
    .interact()?;
  Ok(choice)
}

/// Prompts the user to choose a Solana derivation style.
pub fn select_derivation_style() -> Result<DerivationStyle, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Select Solana derivation style:")
    .items([
      "Standard (m/44'/501'/<index>'/0')",
      "WalletCore style (m/44'/501'/0'/0'/<index>')",
      "Legacy (m/44'/501'/<index>')",
      "Custom path",
    ])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => DerivationStyle::Standard,
    1 => DerivationStyle::WalletCore,
    2 => DerivationStyle::Legacy,
    3 => {
      let input: String = Input::with_theme(&dialoguer_theme("►"))
        .with_prompt("Enter custom derivation path template")
        .default(SOL_STANDARD_PATH.into())
        .interact_text()?;
      DerivationStyle::Custom(input)
    }
    _ => unreachable!(),
  })
}

/// Builds the full derivation path string for `index` and `style`.
pub fn path_for_index(style: &DerivationStyle, index: u32) -> String {
  let template = match style {
    DerivationStyle::Standard => SOL_STANDARD_PATH,
    DerivationStyle::WalletCore => SOL_WALLETCORE_PATH,
    DerivationStyle::Legacy => SOL_LEGACY_PATH,
    DerivationStyle::Custom(custom) => custom.as_str(),
  };

  if template.contains("{index}") {
    template.replace("{index}", &index.to_string())
  } else {
    template.to_string()
  }
}

/// Returns the list of common path candidates used by scanner mode.
pub fn common_scan_paths() -> &'static [&'static str] {
  SOL_SCAN_PATHS
}

/// Derives a 32-byte Ed25519 private scalar for `path`.
pub fn derive_seed_from_path(seed: &[u8], path: &str) -> Result<[u8; 32], Box<dyn Error>> {
  // SLIP-0010 Ed25519 supports hardened derivation only.
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
