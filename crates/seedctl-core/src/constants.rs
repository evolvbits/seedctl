//! Shared constants used across all `seedctl-*` chain crates.
//!
//! Includes BIP/SLIP-44 coin types, derivation purposes, and
//! global feature flags such as RPC URL prompting.

/// BIP-44 purpose value used in derivation paths (`m/44'/…`).
pub const BIP44: u32 = 44u32;

/// SLIP-44 coin type for Ethereum and all EVM-compatible chains.
pub const ETHEREUM_COIN_TYPE: u32 = 60u32;

/// Derivation purpose for Cardano wallets following CIP-1852.
pub const CARDANO_PURPOSE: u32 = 1852u32;

/// SLIP-44 coin type for Cardano (ADA).
pub const CARDANO_COIN_TYPE: u32 = 1815u32;

/// SLIP-44 coin type for Monero (XMR).
pub const MONERO_COIN_TYPE: u32 = 128u32;

/// SLIP-44 coin type for Solana (SOL).
pub const SOLANA_COIN_TYPE: u32 = 501u32;

/// SLIP-44 coin type for XRP Ledger (XRP).
pub const XRP_COIN_TYPE: u32 = 144u32;

/// Controls whether RPC URL prompts are shown to the user.
///
/// When `false` (default), balance checks are skipped and no RPC URL is
/// requested, keeping the tool fully offline-capable out of the box.
///
/// Set to `true` to enable per-network RPC URL prompts for balance queries.
///
/// Default RPC endpoints for reference:
/// 1. Bitcoin (BTC):           `http://user:pass@127.0.0.1:8332`
/// 2. Ethereum (ETH):          `https://cloudflare-eth.com/v1/mainnet`
/// 3. BNB Smart Chain (BNB):   `https://bsc-dataseed.bnbchain.org`
/// 4. XRP Ledger (XRP):        `https://s1.ripple.com:51234/`
/// 5. Tron (TRX):              `https://api.trongrid.io`
/// 6. Solana (SOL):            `https://api.mainnet-beta.solana.com`
/// 7. Litecoin (LTC):          `http://user:pass@127.0.0.1:9332`
/// 8. Polygon (MATIC/POL):     `https://polygon-rpc.com`
/// 9. Cardano (ADA):           `https://api.koios.rest/api/v1/address_info`
/// 10. Monero (XMR):           `http://127.0.0.1:18088/json_rpc`
pub const RPC_URL_ENABLE: bool = false;

/// Entropy bits provided by a single standard six-sided die roll.
///
/// Computed as `log2(6) ≈ 2.585`, used to determine how many dice rolls
/// are required to reach a target entropy level.
pub const BITS_PER_DIE: f64 = 2.584962500721156;
