//! Monero (XMR) wallet output rendering.
//!
//! Provides [`WalletOutput`] and [`print_wallet_output`] used by [`crate::run`]
//! to display the complete Monero wallet section — spend keys and the address /
//! subaddress table with optional XMR balances — in a consistent format on the
//! terminal.

use seedctl_core::ui::{AddressRows, print_standard_wallet};

/// All data required to render a Monero wallet section to stdout.
///
/// Assembled in [`crate::run`] after Monero key derivation and address
/// generation, then passed to [`print_wallet_output`] for display.
pub struct WalletOutput<'a> {
  /// BIP-44 purpose value (`44`) used in the derivation path header.
  ///
  /// Monero does not follow BIP-32 / BIP-44, but this field is kept for
  /// consistency with the shared [`print_standard_wallet`] interface.
  pub purpose: u32,

  /// SLIP-44 coin type for Monero (`128`) used in the derivation path header.
  pub coin_type: u32,

  /// Hex-encoded spend private key (32-byte Ed25519 scalar).
  ///
  /// This is the `spend_private` scalar derived via `Hs(seed)`.
  pub account_xprv: &'a str,

  /// Hex-encoded spend public key (32-byte compressed Ed25519 point).
  ///
  /// This is `spend_private * G`, embedded in every Monero address as the
  /// first 32 bytes after the prefix byte.
  pub account_xpub: &'a str,

  /// When `true`, the private key section is included in the terminal output.
  ///
  /// Set to `false` to render a public-only display (view-key wallet).
  pub show_privkeys: bool,

  /// Derived Monero addresses (standard + subaddresses) with optional
  /// on-chain XMR balances.
  ///
  /// Each entry is `(path_label, monero_address, optional_balance_xmr)`.
  /// The path label uses the `"xmr(major=0,minor=<index>)"` notation.
  pub addresses: &'a [(String, String, Option<f64>)],
}

/// Renders the full Monero wallet section to stdout.
///
/// Delegates to [`print_standard_wallet`] with the Monero-specific title,
/// spend keys, and address rows. No fingerprint or output descriptors are
/// included because Monero uses a custom address scheme (not Bitcoin-style
/// UTXO descriptors).
///
/// The private key (spend scalar) is conditionally included based on
/// [`WalletOutput::show_privkeys`].
pub fn print_wallet_output(output: &WalletOutput<'_>) {
  // Conditionally expose the spend private key based on the caller's preference.
  let account_xprv_opt = if output.show_privkeys {
    Some(output.account_xprv)
  } else {
    None
  };

  print_standard_wallet(
    "Monero Wallet",
    output.purpose,
    output.coin_type,
    // Monero does not expose a BIP-32 master fingerprint.
    None,
    account_xprv_opt,
    output.account_xpub,
    AddressRows::WithBalance(output.addresses),
    // Monero uses custom address encoding, not Bitcoin-style descriptors.
    vec![],
  );
}
