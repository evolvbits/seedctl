//! Utility sub-modules for the `seedctl-btc` crate.
//!
//! Re-exports two focused helper modules:
//!
//! - [`crypto`] — SLIP-132 extended key prefix conversion (`xpub` → `zpub`,
//!   `ypub`; `xprv` → `zprv`, `yprv`).
//! - [`format`] — BIP-380 output descriptor and key-origin string assembly.

/// SLIP-132 extended key prefix conversion utilities.
///
/// Provides functions to re-encode `xprv` / `xpub` keys with the alternative
/// version bytes required for BIP-84 (`z`-prefix) and BIP-49 (`y`-prefix)
/// wallet types.
pub mod crypto;

/// Bitcoin output descriptor and key-origin formatting helpers.
///
/// Provides [`format::format_key_origin`] for building the
/// `[fingerprint/purpose'h/coin'h/0h]` prefix and [`format::output_descriptor`]
/// for assembling complete BIP-380 descriptors for receive and change chains.
pub mod format;
