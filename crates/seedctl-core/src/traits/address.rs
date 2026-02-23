/// Minimal interface for anything that can be rendered as an address row
/// in a table (`path` + `addr`), independent of concrete storage.
pub trait AddressDisplay {
  fn path(&self) -> &str;
  fn addr(&self) -> &str;

  /// Optional extra column (e.g. balance). Defaults to none.
  fn extra(&self) -> Option<String> {
    None
  }
}
