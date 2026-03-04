//! Generic address-table printer based on the [`AddressDisplay`] trait.
//!
//! Provides a single, reusable [`print_table`] function that renders any
//! slice of address rows — as long as the element type implements
//! [`AddressDisplay`] — into a neatly aligned two-column table.

use crate::traits::address::AddressDisplay;

/// Prints a two-column address table (derivation path | address) to stdout.
///
/// Column widths are computed dynamically from the widest value in each
/// column so that all rows line up regardless of address format length.
///
/// # Behaviour
///
/// - If `rows` is empty, prints `"(no addresses)"` and returns immediately.
/// - The optional third column returned by [`AddressDisplay::extra`] is
///   **not** rendered here; use [`crate::ui::print_address_table`] for that.
pub fn print_table<T: AddressDisplay>(rows: &[T]) {
  if rows.is_empty() {
    println!("(no addresses)");
    return;
  }

  // Compute column widths from the widest value in each column.
  let path_w = rows.iter().map(|r| r.path().len()).max().unwrap_or(10);
  let addr_w = rows.iter().map(|r| r.addr().len()).max().unwrap_or(20);

  // Header row.
  println!(
    "\n{:<path_w$} | {:<addr_w$}",
    "Derivation Path",
    "Address",
    path_w = path_w,
    addr_w = addr_w,
  );

  // Separator line.
  println!("{}-+-{}", "-".repeat(path_w), "-".repeat(addr_w));

  // Data rows.
  for r in rows {
    println!(
      "{:<path_w$} | {:<addr_w$}",
      r.path(),
      r.addr(),
      path_w = path_w,
      addr_w = addr_w,
    );
  }
}
