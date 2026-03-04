//! Ethereum RPC client re-export.
//!
//! Re-exports [`seedctl_core::evm::RpcClient`] under the local `rpc` module
//! path so that [`crate::run`] can reference `rpc::RpcClient` without
//! importing from `seedctl_core` directly.
//!
//! The [`RpcClient`] sends standard Ethereum JSON-RPC `eth_getBalance`
//! requests, which are compatible with any EIP-1474 compliant endpoint
//! (Cloudflare, Infura, Alchemy, local Geth / Reth nodes, etc.).

/// Ethereum JSON-RPC client for balance queries.
///
/// Re-exported from [`seedctl_core::evm::RpcClient`]. See that type for full
/// documentation on construction and usage.
pub use seedctl_core::evm::RpcClient;
