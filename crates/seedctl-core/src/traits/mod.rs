//! Shared traits for chain-agnostic abstractions.
//!
//! This module re-exports the three core traits used across all
//! `seedctl-*` chain crates:
//!
//! - [`address::AddressDisplay`] — minimal interface for rendering an address
//!   row in a table (path + address + optional extra column).
//! - [`chain::Chain`] — BIP-32/BIP-39 derivation interface operating on
//!   mnemonic strings and derivation paths.
//! - [`wallet::Wallet`] — chain-agnostic wallet interface exposing coin type,
//!   title, and a slice of typed address rows.

/// Address display trait for table rendering.
pub mod address;

/// Chain derivation trait and shared derivation context.
pub mod chain;

/// Generic wallet trait for chain-agnostic UI operations.
pub mod wallet;
