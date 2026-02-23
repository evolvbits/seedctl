use crate::traits::address::AddressDisplay;

/// Simple address row used across chains for tabular display.
#[derive(Clone, Debug)]
pub struct AddressRow {
  pub path: String,
  pub address: String,
  /// Optional extra column (e.g. balance).
  pub extra: Option<String>,
}

impl AddressRow {
  pub fn new<P: Into<String>, A: Into<String>>(path: P, address: A) -> Self {
    Self {
      path: path.into(),
      address: address.into(),
      extra: None,
    }
  }

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

/// Convenience trait similar to your `IntoRows`, allowing any container
/// to expose a homogeneous list of address rows.
pub trait IntoRows {
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

// Backwards‑compatibility: existing `(String, String)` and
// `(String, String, Option<f64>)` rows can still be displayed by the
// new `AddressDisplay`‑based UI.

impl AddressDisplay for (String, String) {
  fn path(&self) -> &str {
    &self.0
  }

  fn addr(&self) -> &str {
    &self.1
  }
}

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

/// Ethereum-style address with optional balance.
pub struct EthAddress {
  pub path: String,
  pub address: String,
  pub balance: Option<f64>,
}

impl AddressDisplay for EthAddress {
  fn path(&self) -> &str {
    &self.path
  }

  fn addr(&self) -> &str {
    &self.address
  }

  fn extra(&self) -> Option<String> {
    self.balance.map(|v| format!("{:.8}", v))
  }
}

/// Simple Bitcoin-style address (no extra metadata for now).
pub struct BtcAddress {
  pub path: String,
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
