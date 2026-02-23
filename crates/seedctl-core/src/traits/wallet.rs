use crate::traits::address::AddressDisplay;

/// Generic, chain‑agnostic wallet interface that can be used by the UI/runtime
/// to operate over any supported chain.
pub trait Wallet {
  /// Concrete address row type used by this wallet.
  type Address: AddressDisplay;

  /// Returns the SLIP‑44 coin type.
  fn coin_type(&self) -> u32;

  /// Human‑readable title used in headers.
  fn title(&self) -> &str;

  /// List of receive addresses to be shown in tables.
  fn addresses(&self) -> &[Self::Address];
}
