use crate::traits::address::AddressDisplay;

/// Minimal representation of an address row ready for output.
pub struct AddressOutput<'a, T: AddressDisplay> {
  pub title: &'a str,
  pub rows: &'a [T],
}

impl<'a, T: AddressDisplay> AddressOutput<'a, T> {
  pub fn new(title: &'a str, rows: &'a [T]) -> Self {
    Self { title, rows }
  }
}
