//! Concrete address row types used across all `seedctl-*` chain crates.
//!
//! Provides [`AddressRow`], a general-purpose address row, along with
//! chain-specific specialisations ([`BtcAddress`], [`EthAddress`]) and
//! backwards-compatible [`AddressDisplay`] implementations for the raw
//! `(String, String)` and `(String, String, Option<f64>)` tuple types.

use crate::traits::address::AddressDisplay;

/// Simple, reusable address row used across all chains for tabular display.
///
/// Stores a derivation path, an encoded address string, and an optional extra
/// value (e.g. an on-chain balance or a script-type label).
///
/// # Examples
///
/// ```rust
/// use seedctl_core::types::address::AddressRow;
///
/// // Basic row — no extra column.
/// let row = AddressRow::new("m/84'/0'/0'/0/0", "bc1q…");
///
/// // Row with a balance column.
/// let row_with_balance = AddressRow::with_extra(
///     "m/84'/0'/0'/0/0",
///     "bc1q…",
///     "0.00123456 BTC",
/// );
/// ```
#[derive(Clone, Debug)]
pub struct AddressRow {
  /// BIP-32 derivation path, e.g. `"m/84'/0'/0'/0/0"`.
  pub path: String,

  /// Encoded blockchain address string.
  pub address: String,

  /// Optional extra value rendered in a third table column (e.g. balance).
  pub extra: Option<String>,
}

impl AddressRow {
  /// Creates a new [`AddressRow`] with only path and address (no extra column).
  ///
  /// # Parameters
  ///
  /// - `path`    — BIP-32 derivation path string.
  /// - `address` — encoded blockchain address.
  pub fn new<P: Into<String>, A: Into<String>>(path: P, address: A) -> Self {
    Self {
      path: path.into(),
      address: address.into(),
      extra: None,
    }
  }

  /// Creates a new [`AddressRow`] with path, address, and an extra column value.
  ///
  /// # Parameters
  ///
  /// - `path`    — BIP-32 derivation path string.
  /// - `address` — encoded blockchain address.
  /// - `extra`   — additional value for the third table column (e.g. balance).
  pub fn with_extra<P: Into<String>, A: Into<String>, E: Into<String>>(
    path: P,
    address: A,
    extra: E,
  ) -> Self {
    Self {
      path: path.into(),
      address: address.into(),
      extra: Some(extra.into()),
    }
  }
}

/// Allows any container to expose a homogeneous list of [`AddressRow`] values
/// for rendering by the shared table printer.
pub trait IntoRows {
  /// Returns this value as a `Vec<AddressRow>` suitable for table rendering.
  fn rows(&self) -> Vec<AddressRow>;
}

impl AddressDisplay for AddressRow {
  fn path(&self) -> &str {
    &self.path
  }

  fn addr(&self) -> &str {
    &self.address
  }

  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
}

// ── Backwards-compatible tuple impls ─────────────────────────────────────────
//
// Existing chain crates build `Vec<(String, String)>` or
// `Vec<(String, String, Option<f64>)>` address lists. These impls let those
// tuples be passed directly to any function that accepts `&[impl AddressDisplay]`
// without requiring a migration to `AddressRow` first.

/// [`AddressDisplay`] for a plain `(path, address)` pair.
impl AddressDisplay for (String, String) {
  fn path(&self) -> &str {
    &self.0
  }

  fn addr(&self) -> &str {
    &self.1
  }
}

/// [`AddressDisplay`] for a `(path, address, optional_balance)` triple.
///
/// The balance is formatted with 8 decimal places when present
/// (e.g. `"0.00123456"`).
impl AddressDisplay for (String, String, Option<f64>) {
  fn path(&self) -> &str {
    &self.0
  }

  fn addr(&self) -> &str {
    &self.1
  }

  fn extra(&self) -> Option<String> {
    self.2.map(|v| format!("{:.8}", v))
  }
}

// ── Chain-specific address types ─────────────────────────────────────────────

/// Ethereum-style address row with an optional on-chain balance.
///
/// Used by EVM-compatible chains (ETH, BNB, MATIC, TRX) to carry a balance
/// value retrieved from an RPC node alongside the derived address.
pub struct EthAddress {
  /// BIP-32 derivation path, e.g. `"m/44'/60'/0'/0/0"`.
  pub path: String,

  /// EIP-55 checksum-encoded Ethereum address, e.g. `"0xABcd…"`.
  pub address: String,

  /// On-chain balance in the chain's native unit (e.g. ETH, BNB).
  /// `None` when no RPC URL was provided or the query failed.
  pub balance: Option<f64>,
}

impl AddressDisplay for EthAddress {
  fn path(&self) -> &str {
    &self.path
  }

  fn addr(&self) -> &str {
    &self.address
  }

  /// Returns the balance formatted to 8 decimal places, or `None` if unknown.
  fn extra(&self) -> Option<String> {
    self.balance.map(|v| format!("{:.8}", v))
  }
}

/// Bitcoin-style address row without extra metadata.
///
/// Used by UTXO chains (BTC, LTC) where balance information is not fetched
/// inline during address derivation.
pub struct BtcAddress {
  /// BIP-32 derivation path, e.g. `"m/84'/0'/0'/0/0"`.
  pub path: String,

  /// Bech32 / Base58Check encoded Bitcoin address.
  pub address: String,
}

impl AddressDisplay for BtcAddress {
  fn path(&self) -> &str {
    &self.path
  }

  fn addr(&self) -> &str {
    &self.address
  }
}
