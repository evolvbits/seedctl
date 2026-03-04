//! Polygon (MATIC/POL) RPC client re-export.
//!
//! Re-exports [`seedctl_core::evm::RpcClient`] under the local `rpc` module
//! path so that [`crate::run`] can reference `rpc::RpcClient` without
//! importing from `seedctl_core` directly.
//!
//! The [`RpcClient`] uses the standard Ethereum JSON-RPC `eth_getBalance`
//! method, which is fully compatible with any EVM-compatible node including
//! Polygon PoS (`https://polygon-rpc.com`) and its edge/zkEVM variants.

/// EVM-compatible JSON-RPC client for Polygon balance queries.
///
/// Re-exported from [`seedctl_core::evm::RpcClient`]. See that type for full
/// documentation on construction and usage.
pub use seedctl_core::evm::RpcClient;
