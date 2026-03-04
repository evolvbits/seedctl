//! Solana wallet output rendering.
//!
//! Provides [`WalletOutput`] and [`print_wallet_output`] used by [`crate::run`]
//! to display the complete Solana wallet section — account keys and the
//! base58 address table — in a consistent format on the terminal.

use seedctl_core::ui::{AddressRows, print_standard_wallet};

/// All data required to render a Solana wallet section to stdout.
///
/// Assembled in [`crate::run`] after SLIP-0010 Ed25519 key derivation and
/// address generation, then passed to [`print_wallet_output`] for display.
pub struct WalletOutput<'a> {
  /// Coin type used in the derivation path label (BIP-44 = `44`).
  ///
  /// Note: this field is repurposed here — Solana uses `501` as its SLIP-44
  /// coin type, but the display layer needs a numeric label for the path
  /// header. The convention in this crate is to pass `ETHEREUM_COIN_TYPE`
  /// (60) for the purpose and `BIP44` (44) for the coin_type field; the
  /// actual path `m/44'/501'/<index>'/0'` is encoded per address row.
  pub purpose: u32,

  /// Purpose field used in the derivation path header display.
  pub coin_type: u32,

  /// Hex-encoded first account signing key (32-byte Ed25519 private scalar).
  pub account_xprv: &'a str,

  /// Hex-encoded first account verifying key (32-byte Ed25519 public key).
  pub account_xpub: &'a str,

  /// When `true`, the private key field is included in the display.
  pub show_privkeys: bool,

  /// Derived Solana addresses with optional on-chain SOL balances.
  ///
  /// Each entry is `(derivation_path, base58_address, optional_balance_sol)`.
  pub addresses: &'a [(String, String, Option<f64>)],
}

/// Renders the full Solana wallet section to stdout.
///
/// Delegates to [`print_standard_wallet`] with the Solana-specific title,
/// keys, and address rows. No fingerprint or output descriptors are included
/// because Solana uses a flat Ed25519 key model rather than Bitcoin-style
/// UTXO descriptors.
///
/// The private key is conditionally included based on [`WalletOutput::show_privkeys`].
pub fn print_wallet_output(output: &WalletOutput<'_>) {
  let account_xprv_opt = if output.show_privkeys {
    Some(output.account_xprv)
  } else {
    None
  };

  print_standard_wallet(
    "Solana Wallet",
    output.purpose,
    output.coin_type,
    None,
    account_xprv_opt,
    output.account_xpub,
    AddressRows::WithBalance(output.addresses),
    vec![],
  );
}
