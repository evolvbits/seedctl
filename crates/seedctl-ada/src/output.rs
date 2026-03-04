//! Cardano wallet output rendering.
//!
//! Provides [`WalletOutput`] and [`print_wallet_output`] used by [`crate::run`]
//! to display the complete Cardano wallet section — derivation path, account
//! keys, and the Shelley base address table — in a consistent format on the
//! terminal.

use seedctl_core::ui::{AddressRows, print_standard_wallet};

/// All data required to render a Cardano wallet section to stdout.
///
/// Assembled in [`crate::run`] after CIP-1852 key derivation and address
/// generation, then passed to [`print_wallet_output`] for display.
pub struct WalletOutput<'a> {
  /// CIP-1852 purpose value (`1852`).
  pub purpose: u32,

  /// SLIP-44 coin type for Cardano (`1815`).
  pub coin_type: u32,

  /// Hex-encoded first payment extended private key (index 0).
  ///
  /// This is the payment key at `m/1852'/1815'/0'/0/0`, not the account root,
  /// because Cardano does not expose the account `XPrv` directly to the user.
  pub account_xprv: &'a str,

  /// Hex-encoded account-level extended public key at `m/1852'/1815'/0'`.
  pub account_xpub: &'a str,

  /// When `true`, the private key field is included in the display.
  pub show_privkeys: bool,

  /// Derived Shelley base addresses with optional on-chain ADA balances.
  ///
  /// Each entry is `(derivation_path, bech32_address, optional_balance_ada)`.
  pub addresses: &'a [(String, String, Option<f64>)],
}

/// Renders the full Cardano wallet section to stdout.
///
/// Delegates to [`print_standard_wallet`] with the Cardano-specific title,
/// keys, and address rows. No fingerprint or output descriptors are included
/// because the Cardano key scheme does not use Bitcoin-style UTXO descriptors.
///
/// The private key is conditionally included based on [`WalletOutput::show_privkeys`].
pub fn print_wallet_output(output: &WalletOutput<'_>) {
  let account_xprv_opt = if output.show_privkeys {
    Some(output.account_xprv)
  } else {
    None
  };

  print_standard_wallet(
    "Cardano Wallet",
    output.purpose,
    output.coin_type,
    None,
    account_xprv_opt,
    output.account_xpub,
    AddressRows::WithBalance(output.addresses),
    vec![],
  );
}
