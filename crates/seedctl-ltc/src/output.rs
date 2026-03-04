//! Litecoin wallet output rendering.
//!
//! Provides [`WalletOutput`] and [`print_wallet_output`] used by [`crate::run`]
//! to display the complete Litecoin wallet section — derivation path, fingerprint,
//! account keys, output descriptors, and the bech32 address table — in a
//! consistent format on the terminal.

use seedctl_core::ui::{AddressRows, print_standard_wallet};

/// All data required to render a Litecoin wallet section to stdout.
///
/// Assembled in [`crate::run`] after BIP-84 key derivation and address
/// generation, then passed to [`print_wallet_output`] for display.
pub struct WalletOutput<'a> {
  /// SLIP-44 coin type used in the derivation path header.
  ///
  /// - `2` for Litecoin Mainnet.
  /// - `1` for Litecoin Testnet.
  pub coin_type: u32,

  /// 4-byte master fingerprint used in the derivation path header and
  /// output descriptor key-origin prefix.
  pub fingerprint: &'a [u8; 4],

  /// Hex-encoded account-level extended private key (32-byte secp256k1
  /// scalar).
  pub account_xprv: &'a str,

  /// Hex-encoded account-level extended public key (33-byte compressed
  /// secp256k1 point).
  pub account_xpub: &'a str,

  /// Derived bech32 P2WPKH receive addresses with optional on-chain LTC
  /// balances.
  ///
  /// Each entry is `(derivation_path, ltc1_address, optional_balance_ltc)`.
  pub addresses: &'a [(String, String, Option<f64>)],
}

/// Renders the full Litecoin wallet section to stdout.
///
/// Delegates to [`print_standard_wallet`] with:
/// - BIP purpose `84` (Native SegWit P2WPKH).
/// - The Litecoin-specific coin type and master fingerprint.
/// - Placeholder output descriptor labels (`"ltc-bip84"`) for both the
///   receive and change chains.
///
/// The private key is always included in the display. To suppress it, set
/// `account_xprv` to an empty string and handle visibility at the call site.
pub fn print_wallet_output(output: &WalletOutput<'_>) {
  print_standard_wallet(
    "Litecoin Wallet",
    // BIP-84: Native SegWit (P2WPKH) purpose.
    84,
    output.coin_type,
    Some(output.fingerprint),
    Some(output.account_xprv),
    output.account_xpub,
    AddressRows::WithBalance(output.addresses),
    vec![
      ("Output Descriptor (receive):", "ltc-bip84"),
      ("Output Descriptor (change):", "ltc-bip84"),
    ],
  );
}
