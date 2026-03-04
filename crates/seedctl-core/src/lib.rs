//! Core library shared by all `seedctl-*` chain crates.
//!
//! Provides the foundational building blocks for the `seedctl` ecosystem:
//!
//! - **[`args`]** — lightweight CLI argument parsing (version / about / run)
//! - **[`constants`]** — BIP/SLIP-44 coin types and global feature flags
//! - **[`entropy`]** — entropy sources, dice-based generation and resolution
//! - **[`evm`]** — shared EVM derivation logic (ETH, BNB, MATIC, TRX)
//! - **[`export`]** — watch-only wallet JSON serialisation structures
//! - **[`macros`]** — cross-platform `userprofile!` path macro
//! - **[`options`]** — interactive entropy option flow (mnemonic size, dice mode)
//! - **[`ui`]** — themed dialoguer prompts and wallet table rendering
//! - **[`utils`]** — SHA-256 hashing, master key derivation, dice helpers
//!
//! Modular core API:
//!
//! - **[`chain`]** — chain derivation trait and shared context type
//! - **[`derivation`]** — BIP-32 path parsing and wallet generator trait
//! - **[`error`]** — [`error::SeedCtlError`] domain error type
//! - **[`mnemonic`]** — [`mnemonic::MnemonicGenerator`] for BIP-39 mnemonics
//! - **[`output`]** — minimal address output wrapper
//! - **[`security`]** — cold-wallet disclaimer / security warning screen
//! - **[`traits`]** — [`traits::address::AddressDisplay`], [`traits::chain::Chain`], [`traits::wallet::Wallet`]
//! - **[`types`]** — concrete address row and wallet container types

pub mod args;
pub mod constants;
pub mod entropy;
pub mod evm;
pub mod export;
pub mod macros;
pub mod options;
pub mod ui;
pub mod utils;

// New modular core API
pub mod chain;
pub mod derivation;
pub mod error;
pub mod mnemonic;
pub mod output;
pub mod security;
pub mod traits;
pub mod types;
