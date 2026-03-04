//! Address display trait for table rendering.
//!
//! [`AddressDisplay`] is the minimal interface that any address row type must
//! implement to be rendered by the shared UI table functions in
//! [`crate::ui`] and [`crate::ui::table`].

/// Minimal interface for anything that can be rendered as an address row
/// in a two-column table (`path` | `addr`), independent of the concrete
/// storage type.
///
/// Implement this trait on your chain-specific address struct to make it
/// compatible with [`crate::ui::print_address_table`] and
/// [`crate::ui::table::print_table`] without any extra conversion step.
///
/// # Optional extra column
///
/// The [`extra`] method provides an optional third column (e.g. an on-chain
/// balance). When at least one row returns `Some`, the UI renders a third
/// `"Extra"` column; when all rows return `None`, the column is omitted.
pub trait AddressDisplay {
  /// Returns the BIP-32 derivation path for this address row,
  /// e.g. `"m/84'/0'/0'/0/0"`.
  fn path(&self) -> &str;

  /// Returns the encoded blockchain address string,
  /// e.g. `"bc1q…"` for Bitcoin or `"0x…"` for Ethereum.
  fn addr(&self) -> &str;

  /// Returns an optional extra value for a third table column.
  ///
  /// Typical uses include on-chain balances (`"0.00123456"`) or
  /// script-type labels. Defaults to `None` so implementors only need to
  /// override this when they have meaningful extra data to show.
  fn extra(&self) -> Option<String> {
    None
  }
}
