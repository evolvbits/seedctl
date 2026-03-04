//! Generic wallet trait for chain-agnostic UI operations.
//!
//! [`Wallet`] abstracts over the concrete wallet types produced by each chain
//! crate, allowing the UI and runtime layers to display wallets without
//! depending on chain-specific types.

use crate::traits::address::AddressDisplay;

/// Generic, chain-agnostic wallet interface that can be used by the UI/runtime
/// to operate over any supported chain without knowing its concrete types.
///
/// Each chain crate's wallet struct should implement this trait so that
/// higher-level code (e.g. a unified export or display function) can work
/// with any chain uniformly.
///
/// # Type Parameters
///
/// - `Address` — the concrete address row type for this chain; must implement
///   [`AddressDisplay`] so it can be rendered by the shared UI table functions.
pub trait Wallet {
  /// Concrete address row type used by this wallet.
  ///
  /// Must implement [`AddressDisplay`] so address rows can be passed to the
  /// shared table renderer without any intermediate conversion.
  type Address: AddressDisplay;

  /// Returns the SLIP-44 coin type that identifies this chain.
  ///
  /// For example: `0` for Bitcoin, `60` for Ethereum, `144` for XRP.
  fn coin_type(&self) -> u32;

  /// Returns the human-readable wallet title used in section headers.
  ///
  /// For example: `"Bitcoin Wallet"`, `"Ethereum Wallet"`.
  fn title(&self) -> &str;

  /// Returns the slice of receive addresses to be shown in the address table.
  ///
  /// The number of addresses is determined at wallet construction time
  /// (typically 10–20 for CLI display).
  fn addresses(&self) -> &[Self::Address];
}
