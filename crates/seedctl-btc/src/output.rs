//! Bitcoin wallet output rendering.
//!
//! Provides [`WalletOutput`] and [`print_wallet_output`] used by [`crate::run`]
//! to display the complete Bitcoin wallet section — derivation path, fingerprint,
//! account keys, output descriptors, and the address table — in a consistent,
//! human-readable format on the terminal.

use seedctl_core::ui::{AddressRows, print_standard_wallet};

/// All data required to render a Bitcoin wallet section to stdout.
///
/// Assembled in [`crate::run`] after key derivation and address generation,
/// then passed to [`print_wallet_output`] for display.
pub struct WalletOutput<'a> {
  /// BIP derivation purpose: `84` (SegWit), `49` (Nested SegWit), or `44` (Legacy).
  pub purpose: u32,

  /// SLIP-44 coin type: `0` for Mainnet, `1` for Testnet.
  pub coin_type: u32,

  /// 4-byte master fingerprint, used in the derivation path header and
  /// output descriptor key-origin prefix.
  pub fingerprint: &'a [u8; 4],

  /// Account-level extended private key string (xprv / yprv / zprv).
  pub account_xprv: &'a str,

  /// Account-level extended public key string (xpub / ypub / zpub).
  pub account_xpub: &'a str,

  /// BIP-380 output descriptor for the external (receive) chain.
  pub desc_receive: &'a str,

  /// BIP-380 output descriptor for the internal (change) chain.
  pub desc_change: &'a str,

  /// Derived receive addresses with optional on-chain balances.
  ///
  /// Each entry is `(derivation_path, address, optional_balance_btc)`.
  pub addresses: &'a [(String, String, Option<f64>)],
}

/// Renders the full Bitcoin wallet section to stdout.
///
/// Delegates to [`print_standard_wallet`] with the Bitcoin-specific title,
/// fingerprint, keys, descriptors, and address rows.
///
/// The private key is always shown (the `account_xprv` field is unconditionally
/// passed as `Some`). To suppress it, set `account_xprv` to an empty string
/// and handle visibility at the call site.
pub fn print_wallet_output(output: &WalletOutput<'_>) {
  print_standard_wallet(
    "Bitcoin Wallet",
    output.purpose,
    output.coin_type,
    Some(output.fingerprint),
    Some(output.account_xprv),
    output.account_xpub,
    AddressRows::WithBalance(output.addresses),
    vec![
      ("Output Descriptor (receive):", output.desc_receive),
      ("Output Descriptor (change):", output.desc_change),
    ],
  );
}
