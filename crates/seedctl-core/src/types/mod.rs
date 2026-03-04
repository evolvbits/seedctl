//! Concrete types that implement the shared traits from [`crate::traits`].
//!
//! Provides ready-to-use structs and backwards-compatible tuple impls so
//! that chain crates can build address lists and wallet containers without
//! duplicating boilerplate:
//!
//! - [`address`] — [`address::AddressRow`], [`address::EthAddress`],
//!   [`address::BtcAddress`], and [`AddressDisplay`] impls for raw tuples.
//! - [`wallet`]  — [`wallet::Wallet`], a generic wallet container built
//!   from any [`crate::traits::chain::Chain`] implementation.

/// Concrete address row types and backwards-compatible tuple impls.
pub mod address;

/// Generic wallet container built from a [`crate::traits::chain::Chain`] impl.
pub mod wallet;
