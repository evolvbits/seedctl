//! Minimal address output wrapper used by chain crates for structured display.
//!
//! [`AddressOutput`] bundles a human-readable title with a slice of address
//! rows that implement [`AddressDisplay`], providing a clean boundary between
//! data assembly (chain crates) and rendering (UI layer).

use crate::traits::address::AddressDisplay;

/// A labelled collection of address rows ready to be passed to a UI renderer.
///
/// Chain crates construct an `AddressOutput` after deriving addresses and
/// hand it off to the UI layer, keeping derivation logic decoupled from
/// presentation.
///
/// # Type Parameters
///
/// - `'a` — lifetime of the borrowed title string and address slice.
/// - `T`  — concrete address row type; must implement [`AddressDisplay`].
pub struct AddressOutput<'a, T: AddressDisplay> {
  /// Human-readable label shown above the address table (e.g. `"Bitcoin Wallet"`).
  pub title: &'a str,

  /// Slice of address rows to be rendered in the table.
  pub rows: &'a [T],
}

impl<'a, T: AddressDisplay> AddressOutput<'a, T> {
  /// Creates a new [`AddressOutput`] with the given title and address rows.
  ///
  /// # Parameters
  ///
  /// - `title` — section label printed above the address table.
  /// - `rows`  — slice of typed address rows implementing [`AddressDisplay`].
  pub fn new(title: &'a str, rows: &'a [T]) -> Self {
    Self { title, rows }
  }
}
