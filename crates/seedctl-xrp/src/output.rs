//! XRP Ledger wallet output rendering.
//!
//! Provides [`WalletOutput`] and [`print_wallet_output`] used by [`crate::run`]
//! to display the complete XRP Ledger wallet section — derivation path, account
//! keys, and the classic address table with optional XRP balances — in a
//! consistent format on the terminal.

use seedctl_core::ui::{AddressRows, print_standard_wallet};

/// All data required to render an XRP Ledger wallet section to stdout.
///
/// Assembled in [`crate::run`] after BIP-32 key derivation and address
/// generation, then passed to [`print_wallet_output`] for display.
pub struct WalletOutput<'a> {
  /// BIP-44 purpose value (`44`) used in the derivation path header.
  pub purpose: u32,

  /// SLIP-44 coin type for XRP Ledger (`144`) used in the derivation path
  /// header.
  pub coin_type: u32,

  /// Hex-encoded account-level extended private key (32-byte secp256k1
  /// scalar).
  pub account_xprv: &'a str,

  /// Hex-encoded account-level extended public key (33-byte compressed
  /// secp256k1 point).
  pub account_xpub: &'a str,

  /// When `true`, the private key section is included in the terminal output.
  ///
  /// Set to `false` to render a public-only display.
  pub show_privkeys: bool,

  /// Derived XRPL classic addresses with optional on-chain XRP balances.
  ///
  /// Each entry is `(derivation_path, classic_address, optional_balance_xrp)`.
  pub addresses: &'a [(String, String, Option<f64>)],
}

/// Renders the full XRP Ledger wallet section to stdout.
///
/// Delegates to [`print_standard_wallet`] with the XRP-specific title and
/// address rows. No fingerprint or output descriptors are included because
/// XRPL uses classic addresses rather than Bitcoin-style UTXO descriptors.
///
/// The private key is conditionally included based on
/// [`WalletOutput::show_privkeys`].
pub fn print_wallet_output(output: &WalletOutput<'_>) {
  // Conditionally expose the private key based on the caller's preference.
  let account_xprv_opt = if output.show_privkeys {
    Some(output.account_xprv)
  } else {
    None
  };

  print_standard_wallet(
    "XRP Wallet",
    output.purpose,
    output.coin_type,
    None,
    account_xprv_opt,
    output.account_xpub,
    AddressRows::WithBalance(output.addresses),
    vec![],
  );
}
