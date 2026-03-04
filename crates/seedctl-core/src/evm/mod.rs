//! Shared EVM (Ethereum Virtual Machine) derivation and display logic.
//!
//! This module centralises all cryptographic and UI operations that are common
//! to every EVM-compatible chain crate (`seedctl-eth`, `seedctl-bnb`,
//! `seedctl-matic`). Chain-specific behaviour is injected through the
//! [`EvmProfile`] configuration struct, so each chain crate only needs to
//! declare its own profile constant and delegate to the functions here.
//!
//! # Main responsibilities
//!
//! - **Key derivation**: BIP-32 path parsing, child-key derivation, and
//!   EIP-55 address encoding ([`address_from_xprv`], [`derive_address_key`]).
//! - **Interactive prompts**: derivation mode, address count, derivation style,
//!   and optional RPC URL ([`select_derivation_mode`], [`prompt_address_count`],
//!   [`select_derivation_style`], [`prompt_rpc_url`]).
//! - **Balance queries**: JSON-RPC `eth_getBalance` client ([`RpcClient`],
//!   [`get_balance`]).
//! - **Wallet display**: formatted terminal output via [`print_wallet_output`]
//!   and the common-path scanner via [`scan_common_paths`].
//! - **Watch-only export**: assembles a [`crate::export::WalletExport`]
//!   document ready to be serialised to JSON ([`build_watch_only_export`]).
//!
//! # Chain profiles
//!
//! Three pre-built profiles are provided:
//! - [`ETHEREUM_PROFILE`] — Ethereum Mainnet (`m/44'/60'/0'/0/x`)
//! - [`POLYGON_PROFILE`]  — Polygon (same path as Ethereum)
//! - [`BNB_PROFILE`]      — BNB Smart Chain (same path as Ethereum)

use crate::{
  export,
  ui::{AddressRows, dialoguer_theme, print_standard_wallet},
};
use bip32::{ChildNumber, DerivationPath, XPrv};
use console::style;
use dialoguer::{Input, Select};
use k256::ecdsa::{SigningKey, VerifyingKey};
use sha3::{Digest, Keccak256};
use std::error::Error;

/// Type alias for an EVM account-level extended public key.
///
/// Wraps a [`bip32::ExtendedPublicKey`] whose inner key type is the
/// `k256` verifying key, matching the secp256k1 curve used by all EVM chains.
pub type EvmAccountXpub = bip32::ExtendedPublicKey<VerifyingKey>;

/// Standard EVM derivation base path: `m/44'/60'/0'/0`.
///
/// Compatible with MetaMask, Trust Wallet, and most EVM wallets.
const EVM_STANDARD_PATH: &str = "m/44'/60'/0'/0";

/// Ledger-style EVM derivation base path: `m/44'/60'/0'`.
///
/// Used by the Ledger Ethereum app, which treats account indices differently
/// from the standard BIP-44 layout.
const EVM_LEDGER_PATH: &str = "m/44'/60'/0'";

/// Common Ethereum derivation paths scanned by [`scan_common_paths`].
///
/// Covers the most popular wallet layouts so the user can identify which path
/// their existing wallet uses.
const ETHEREUM_SCAN_PATHS: &[&str] = &[
  "m/44'/60'/0'/0/0",
  "m/44'/60'/0'/0/1",
  "m/44'/60'/1'/0/0",
  "m/44'/60'/0'/1/0",
  "m/44'/60'/0'/0/5",
  "m/44'/60'/0'",
  "m/44'/60'/1'",
];

/// Common Polygon derivation paths scanned by [`scan_common_paths`].
const POLYGON_SCAN_PATHS: &[&str] = &[
  "m/44'/60'/0'/0/0",
  "m/44'/60'/0'/0/1",
  "m/44'/60'/0'/1/0",
  "m/44'/60'/1'/0/0",
];

/// Common BNB Smart Chain derivation paths scanned by [`scan_common_paths`].
const BNB_SCAN_PATHS: &[&str] = &[
  "m/44'/60'/0'/0/0",
  "m/44'/60'/0'/0/1",
  "m/44'/60'/1'/0/0",
  "m/44'/60'/0'/1/0",
];

// ── Chain profile ─────────────────────────────────────────────────────────────

/// Chain-specific configuration injected into shared EVM helpers.
///
/// Each EVM chain crate defines one `EvmProfile` constant and passes it to
/// the shared functions in this module. This removes the need for duplicating
/// prompt strings, export identifiers, or scan-path tables across crates.
///
/// All fields are `'static` references so that profile values can be declared
/// as top-level constants with zero heap allocation.
#[derive(Clone, Copy)]
pub struct EvmProfile {
  /// Short chain name, e.g. `"Ethereum"`.
  pub name: &'static str,

  /// Human-readable wallet section title, e.g. `"Ethereum Wallet"`.
  pub wallet_title: &'static str,

  /// Prompt text for the derivation-mode selection step.
  pub derivation_mode_prompt: &'static str,

  /// Prompt text for the derivation-style selection step.
  pub derivation_style_prompt: &'static str,

  /// Prompt text for the address-count input step.
  pub address_count_prompt: &'static str,

  /// Prompt text for the RPC URL input step.
  pub rpc_prompt: &'static str,

  /// Title printed above the common-path scan table.
  pub scan_title: &'static str,

  /// Optional hint shown below the scan table (e.g. how to compare addresses).
  pub scan_tip: Option<&'static str>,

  /// Network identifier written into watch-only export JSON, e.g. `"ethereum"`.
  pub export_network: &'static str,

  /// Script-type label written into watch-only export JSON,
  /// e.g. `"ethereum-bip44"`.
  pub export_script_type: &'static str,

  /// Descriptor string written into watch-only export JSON,
  /// e.g. `"ethereum-address"`.
  pub export_descriptor: &'static str,

  /// File-name prefix for the exported watch-only JSON file, e.g. `"eth"`.
  pub export_file_prefix: &'static str,

  /// Derivation paths iterated by [`scan_common_paths`].
  pub scan_paths: &'static [&'static str],
}

/// Pre-built [`EvmProfile`] for Ethereum (ETH).
pub const ETHEREUM_PROFILE: EvmProfile = EvmProfile {
  name: "Ethereum",
  wallet_title: "Ethereum Wallet",
  derivation_mode_prompt: "Select derivation mode:",
  derivation_style_prompt: "Select derivation style:",
  address_count_prompt: "How many addresses generate?",
  rpc_prompt: "RPC URL (enter to skip balance check)",
  scan_title: "🔎 Automatic derivation path scanner",
  scan_tip: Some("Tip: compare with your wallet known address to find correct path."),
  export_network: "ethereum",
  export_script_type: "ethereum-bip44",
  export_descriptor: "ethereum-address",
  export_file_prefix: "eth",
  scan_paths: ETHEREUM_SCAN_PATHS,
};

/// Pre-built [`EvmProfile`] for Polygon (MATIC/POL).
pub const POLYGON_PROFILE: EvmProfile = EvmProfile {
  name: "Polygon",
  wallet_title: "Polygon Wallet",
  derivation_mode_prompt: "Select derivation mode (Polygon):",
  derivation_style_prompt: "Select derivation style (Polygon):",
  address_count_prompt: "How many Polygon addresses generate?",
  rpc_prompt: "Polygon RPC URL (enter to skip balance check)",
  scan_title: "Scanning common Polygon derivation paths:",
  scan_tip: None,
  export_network: "polygon",
  export_script_type: "polygon-evm-bip44",
  export_descriptor: "polygon-address",
  export_file_prefix: "matic",
  scan_paths: POLYGON_SCAN_PATHS,
};

/// Pre-built [`EvmProfile`] for BNB Smart Chain (BNB).
pub const BNB_PROFILE: EvmProfile = EvmProfile {
  name: "BNB Smart Chain",
  wallet_title: "BNB Wallet",
  derivation_mode_prompt: "Select derivation mode (BNB):",
  derivation_style_prompt: "Select derivation style (BNB):",
  address_count_prompt: "How many BNB addresses generate?",
  rpc_prompt: "BSC RPC URL (enter to skip balance check)",
  scan_title: "Scanning common BNB derivation paths:",
  scan_tip: None,
  export_network: "bsc",
  export_script_type: "bsc-evm-bip44",
  export_descriptor: "bsc-address",
  export_file_prefix: "bnb",
  scan_paths: BNB_SCAN_PATHS,
};

// ── Derivation style ──────────────────────────────────────────────────────────

/// Supported BIP-32 derivation path layouts for EVM chains.
///
/// Determines how the full per-address path is constructed from a base path
/// and an integer address `index`.
#[derive(Clone)]
pub enum DerivationStyle {
  /// Standard BIP-44 layout: `m/44'/60'/0'/0/<index>`.
  ///
  /// Compatible with MetaMask, Trust Wallet, and most software wallets.
  Standard,

  /// Ledger Ethereum app layout: `m/44'/60'/<index>'/0/0`.
  ///
  /// Each Ledger account index lives at a *hardened* path component.
  Ledger,

  /// Fully custom base path supplied by the user.
  ///
  /// The `index` is appended verbatim unless the template contains `{index}`
  /// (which is replaced) or already ends with `/` (no extra separator added).
  Custom(String),
}

// ── Wallet output ─────────────────────────────────────────────────────────────

/// All data needed to render an EVM wallet section to stdout.
///
/// Constructed in each chain crate's `run` function after key derivation and
/// passed to [`print_wallet_output`] together with the chain's [`EvmProfile`].
pub struct WalletOutput<'a> {
  /// BIP-44 purpose number used in the derivation path header (typically `44`).
  pub purpose: u32,

  /// SLIP-44 coin type used in the derivation path header (typically `60` for
  /// all EVM chains).
  pub coin_type: u32,

  /// Hex-encoded account-level extended private key (32-byte secp256k1
  /// scalar).
  pub account_xprv: &'a str,

  /// Hex-encoded account-level extended public key (33-byte compressed
  /// secp256k1 point).
  pub account_xpub: &'a str,

  /// When `true`, the private key section is rendered in the terminal output.
  ///
  /// Set to `false` to produce a public-only display.
  pub show_privkeys: bool,

  /// Derived EVM addresses with optional on-chain native-token balances.
  ///
  /// Each entry is `(derivation_path, eip55_address, optional_balance)`.
  pub addresses: &'a [(String, String, Option<f64>)],
}

// ── JSON-RPC client ───────────────────────────────────────────────────────────

/// Blocking JSON-RPC client for EVM `eth_getBalance` queries.
///
/// Wraps a [`reqwest::blocking::Client`] with a fixed endpoint URL so that
/// `get_balance` can be called multiple times without re-constructing the
/// client on every request.
pub struct RpcClient {
  /// Full URL to the EVM JSON-RPC endpoint.
  url: String,

  /// Underlying blocking HTTP client.
  client: reqwest::blocking::Client,
}

impl RpcClient {
  /// Creates a new [`RpcClient`] targeting `url`.
  ///
  /// The underlying [`reqwest::blocking::Client`] is created once and reused
  /// for all subsequent [`get_balance`](Self::get_balance) calls.
  ///
  /// # Parameters
  ///
  /// - `url` — full URL to an EIP-1474-compatible JSON-RPC endpoint, e.g.
  ///   `"https://cloudflare-eth.com/v1/mainnet"`.
  pub fn new(url: impl Into<String>) -> Self {
    Self {
      url: url.into(),
      client: reqwest::blocking::Client::new(),
    }
  }

  /// Queries the RPC endpoint for the native-token balance of `address`.
  ///
  /// Sends `eth_getBalance(address, "latest")` and converts the returned
  /// wei value (hex string) to a floating-point ETH/BNB/MATIC amount.
  ///
  /// # Returns
  ///
  /// - `Some(balance)` — balance in the chain's native token (wei ÷ 1e18).
  /// - `None` — if the request fails or the response cannot be parsed.
  pub fn get_balance(&self, address: &str) -> Option<f64> {
    let body = serde_json::json!({
      "jsonrpc": "2.0",
      "method": "eth_getBalance",
      "params": [address, "latest"],
      "id": 1,
    });

    let res = self.client.post(&self.url).json(&body).send().ok()?;
    let payload: serde_json::Value = res.json().ok()?;
    let hex = payload.get("result")?.as_str()?;

    // Parse the hex wei value and convert to the chain's native unit.
    let wei = u128::from_str_radix(hex.trim_start_matches("0x"), 16).ok()?;
    Some(wei as f64 / 1e18)
  }
}

/// Convenience wrapper that creates a one-shot [`RpcClient`] and queries the
/// balance of `address`.
///
/// Prefer using [`RpcClient`] directly when querying multiple addresses to
/// avoid re-creating the underlying HTTP client on every call.
///
/// # Returns
///
/// - `Some(balance)` — native-token balance (wei ÷ 1e18).
/// - `None` — on any network or parse error.
pub fn get_balance(url: &str, address: &str) -> Option<f64> {
  RpcClient::new(url).get_balance(address)
}

// ── Interactive prompts ───────────────────────────────────────────────────────

/// Prompts the user to choose between generating addresses and scanning common
/// derivation paths for the given chain profile.
///
/// Returns:
/// - `0` — generate addresses (continue to address-count / style prompts).
/// - `1` — scan common paths (prints a table then returns early).
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn select_derivation_mode(profile: &EvmProfile) -> Result<usize, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt(profile.derivation_mode_prompt)
    .items(["Generate addresses", "Scan common derivation paths"])
    .default(0)
    .interact()?;

  Ok(choice)
}

/// Prompts the user for the number of EVM addresses to generate.
///
/// Uses the `address_count_prompt` field from `profile` and defaults to `10`
/// if the user presses Enter without typing a value.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn prompt_address_count(profile: &EvmProfile) -> Result<u32, Box<dyn Error>> {
  let count: u32 = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt(profile.address_count_prompt)
    .default(10)
    .interact_text()?;

  Ok(count)
}

/// Prompts the user for an EVM JSON-RPC URL used for balance queries.
///
/// Returns an empty string immediately when
/// [`crate::constants::RPC_URL_ENABLE`] is `false` (the default), keeping
/// the tool fully offline-capable out of the box.
///
/// When enabled, the user may press Enter without typing anything to skip
/// the balance check for this session.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn prompt_rpc_url(profile: &EvmProfile) -> Result<String, Box<dyn Error>> {
  if !crate::constants::RPC_URL_ENABLE {
    return Ok(String::new());
  }

  let url: String = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt(profile.rpc_prompt)
    .allow_empty(true)
    .interact_text()?;

  Ok(url)
}

/// Prompts the user to choose a BIP-32 derivation style for an EVM chain.
///
/// Presents three options:
/// - `Standard (m/44'/60'/0'/0/x)` — MetaMask-compatible (default).
/// - `Ledger style`                — Ledger Ethereum app layout.
/// - `Custom path`                 — user-supplied base path string.
///
/// # Errors
///
/// Returns a boxed error if the terminal interaction fails.
pub fn select_derivation_style(profile: &EvmProfile) -> Result<DerivationStyle, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt(profile.derivation_style_prompt)
    .items(["Standard (m/44'/60'/0'/0/x)", "Ledger style", "Custom path"])
    .default(0)
    .interact()?;

  Ok(match choice {
    0 => DerivationStyle::Standard,
    1 => DerivationStyle::Ledger,
    2 => {
      let input: String = Input::with_theme(&dialoguer_theme("►"))
        .with_prompt("Enter custom derivation base path")
        .default(EVM_STANDARD_PATH.into())
        .interact_text()?;
      DerivationStyle::Custom(input)
    }
    _ => unreachable!(),
  })
}

// ── Derivation helpers ────────────────────────────────────────────────────────

/// Converts a [`DerivationStyle`] to its canonical base path string.
///
/// Returns:
/// - `Standard`  → `"m/44'/60'/0'/0"`
/// - `Ledger`    → `"m/44'/60'/0'"`
/// - `Custom(s)` → `s` verbatim
pub fn style_to_string(style: &DerivationStyle) -> String {
  match style {
    DerivationStyle::Standard => EVM_STANDARD_PATH.into(),
    DerivationStyle::Ledger => EVM_LEDGER_PATH.into(),
    DerivationStyle::Custom(custom) => custom.clone(),
  }
}

/// Constructs the full [`DerivationPath`] for the given address `index` and
/// derivation `style`.
///
/// # Path construction rules
///
/// | Style      | Pattern                                              |
/// |:-----------|:-----------------------------------------------------|
/// | Standard   | `m/44'/60'/0'/0/<index>`                             |
/// | Ledger     | `m/44'/60'/<index>'/0/0`                             |
/// | Custom     | template with `{index}` replaced, or `/<index>` appended |
///
/// # Errors
///
/// Returns a boxed error if the constructed path string cannot be parsed as a
/// valid [`DerivationPath`].
pub fn build_path(style: &DerivationStyle, index: u32) -> Result<DerivationPath, Box<dyn Error>> {
  let path_str = match style {
    DerivationStyle::Standard => format!("{}/{}", EVM_STANDARD_PATH, index),
    DerivationStyle::Ledger => format!("m/44'/60'/{}'/0/0", index),
    DerivationStyle::Custom(template) => {
      if template.contains("{index}") {
        template.replace("{index}", &index.to_string())
      } else if template.ends_with('/') {
        format!("{}{}", template, index)
      } else {
        template.clone()
      }
    }
  };

  Ok(path_str.parse()?)
}

/// Iteratively applies each component of `path` to `key`, returning the fully
/// derived descendant [`XPrv`].
///
/// Consumes `key` by value; clone before calling if the original is still
/// needed after this call.
///
/// # Errors
///
/// Returns a boxed error if any child derivation step fails.
pub fn derive_path(mut key: XPrv, path: &DerivationPath) -> Result<XPrv, Box<dyn Error>> {
  for child in path.iter() {
    key = key.derive_child(child)?;
  }

  Ok(key)
}

/// Parses `path` as a [`DerivationPath`] and derives the descendant [`XPrv`]
/// from `master`.
///
/// # Parameters
///
/// - `master` — BIP-32 master extended private key.
/// - `path`   — BIP-32 derivation path string, e.g. `"m/44'/60'/0'/0"`.
///
/// # Errors
///
/// Returns a boxed error if `path` cannot be parsed or if any child
/// derivation step fails.
pub fn derive_from_path(master: XPrv, path: &str) -> Result<XPrv, Box<dyn Error>> {
  let derivation_path: DerivationPath = path.parse()?;
  derive_path(master, &derivation_path)
}

/// Derives the address-level [`XPrv`] and its full path string for the given
/// address `index` using the specified derivation `style`.
///
/// # Strategy per style
///
/// - `Standard`: appends a single non-hardened child `index` to
///   `account_xprv` (which is already at `m/44'/60'/0'/0`), avoiding a full
///   re-derivation from the master key.
/// - `Ledger` / `Custom`: derives the full path from `master` to correctly
///   handle hardened components that cannot be derived from a public key.
///
/// # Parameters
///
/// - `master`       — BIP-32 master extended private key.
/// - `account_xprv` — Account-level key used for the fast Standard single-step
///   child derivation.
/// - `style`        — Derivation style determining the path layout.
/// - `index`        — Address index within the style's layout.
///
/// # Returns
///
/// A tuple of `(leaf_XPrv, derivation_path_string)`.
///
/// # Errors
///
/// Returns a boxed error if path construction or any derivation step fails.
pub fn derive_address_key(
  master: &XPrv,
  account_xprv: &XPrv,
  style: &DerivationStyle,
  index: u32,
) -> Result<(XPrv, String), Box<dyn Error>> {
  let path = build_path(style, index)?;
  let path_str = path.to_string();

  let key = match style {
    DerivationStyle::Standard => {
      // Fast path: account key is already at m/44'/60'/0'/0; append index only.
      let child = ChildNumber::new(index, false)?;
      account_xprv.clone().derive_child(child)?
    }
    // Full re-derivation required for Ledger and Custom styles.
    DerivationStyle::Ledger | DerivationStyle::Custom(_) => derive_path(master.clone(), &path)?,
  };

  Ok((key, path_str))
}

// ── Address encoding ──────────────────────────────────────────────────────────

/// Encodes a 20-byte raw Ethereum address as an EIP-55 checksum string.
///
/// The EIP-55 algorithm applies Keccak-256 to the lowercase hex address and
/// uses the resulting nibbles to determine whether each letter in the output
/// should be upper- or lower-case, providing a built-in integrity check.
///
/// # Parameters
///
/// - `addr` — 20-byte raw address (last 20 bytes of `keccak256(pubkey[1..])`).
///
/// # Returns
///
/// A 42-character `0x`-prefixed EIP-55 checksum address string.
pub fn to_checksum_address(addr: &[u8]) -> String {
  let hex_addr = hex::encode(addr);
  let hash = Keccak256::digest(hex_addr.as_bytes());

  let mut out = String::from("0x");

  for (idx, ch) in hex_addr.chars().enumerate() {
    let hash_byte = hash[idx / 2];
    // Use the high nibble for even positions, low nibble for odd positions.
    let nibble = if idx % 2 == 0 {
      hash_byte >> 4
    } else {
      hash_byte & 0x0f
    };

    if ch.is_ascii_digit() {
      out.push(ch);
    } else if nibble >= 8 {
      // Nibble ≥ 8 → uppercase letter.
      out.push(ch.to_ascii_uppercase());
    } else {
      out.push(ch);
    }
  }

  out
}

/// Derives an EIP-55 checksum-encoded Ethereum address from a leaf [`XPrv`].
///
/// # Algorithm
///
/// 1. Extract the 32-byte secp256k1 private scalar from `xprv`.
/// 2. Derive the uncompressed public key (65 bytes, `0x04` prefix).
/// 3. Apply Keccak-256 to the last 64 bytes (public key without the prefix).
/// 4. Take the last 20 bytes of the hash as the raw account ID.
/// 5. Apply EIP-55 checksum encoding via [`to_checksum_address`].
///
/// # Parameters
///
/// - `xprv` — leaf extended private key at the full derivation path
///   (e.g. `m/44'/60'/0'/0/0`).
///
/// # Errors
///
/// Returns a boxed error if the private-key bytes cannot be loaded into a
/// [`k256::ecdsa::SigningKey`], which should never happen for keys produced
/// by the `bip32` crate.
pub fn address_from_xprv(xprv: XPrv) -> Result<String, Box<dyn Error>> {
  let pk = xprv.private_key().to_bytes();
  let signing = SigningKey::from_bytes(&pk)?;
  let pubkey = signing.verifying_key().to_encoded_point(false);

  // Hash the 64-byte uncompressed public key (strip the `0x04` prefix byte).
  let hash = Keccak256::digest(&pubkey.as_bytes()[1..]);

  // The Ethereum account ID is the last 20 bytes of the hash.
  let addr_bytes = &hash[12..];

  Ok(to_checksum_address(addr_bytes))
}

// ── Common-path scanner ───────────────────────────────────────────────────────

/// Derives and prints EVM addresses for the most common derivation paths
/// defined in `profile.scan_paths`.
///
/// Useful when the user already has an existing wallet and wants to discover
/// which derivation path it used. Each path is derived from `master` and the
/// resulting EIP-55 address is printed on the same line.
///
/// # Parameters
///
/// - `master`  — BIP-32 master extended private key derived from the BIP-39
///   seed + passphrase.
/// - `profile` — chain profile supplying the scan paths and the section title.
///
/// # Errors
///
/// Returns a boxed error if any derivation path cannot be parsed or if key
/// derivation fails for any path in the scan list.
pub fn scan_common_paths(master: XPrv, profile: &EvmProfile) -> Result<(), Box<dyn Error>> {
  println!(
    "\n{} {}\n",
    style("🔎").cyan().bold(),
    style(profile.scan_title).cyan().bold(),
  );

  for path_str in profile.scan_paths.iter() {
    let path: DerivationPath = path_str.parse()?;
    let child = derive_path(master.clone(), &path)?;
    let address = address_from_xprv(child)?;
    println!("{:<22} → {}", path_str, address);
  }

  if let Some(tip) = profile.scan_tip {
    println!("\n{}", style(tip).yellow().bold());
  }

  Ok(())
}

// ── Wallet display ────────────────────────────────────────────────────────────

/// Renders the full EVM wallet section to stdout using the given profile.
///
/// Delegates to [`print_standard_wallet`] with the profile's wallet title,
/// and conditionally includes the private key based on
/// [`WalletOutput::show_privkeys`].
///
/// # Parameters
///
/// - `profile` — chain profile supplying the wallet title.
/// - `output`  — wallet data assembled by the chain crate's `run` function.
pub fn print_wallet_output(profile: &EvmProfile, output: &WalletOutput<'_>) {
  let account_xprv_opt = if output.show_privkeys {
    Some(output.account_xprv)
  } else {
    None
  };

  print_standard_wallet(
    profile.wallet_title,
    output.purpose,
    output.coin_type,
    None,
    account_xprv_opt,
    output.account_xpub,
    AddressRows::WithBalance(output.addresses),
    vec![],
  );
}

// ── Watch-only export ─────────────────────────────────────────────────────────

/// Assembles a [`export::WalletExport`] document for an EVM watch-only wallet.
///
/// Fills in all chain-specific fields from `profile` and uses the raw bytes of
/// `xpub` as the account public key identifier. No private key is included —
/// this is always a watch-only export.
///
/// # Parameters
///
/// - `profile`   — chain profile supplying the network and script-type labels.
/// - `info`      — software metadata slice `[name, version, repository]`.
/// - `base_path` — account-level BIP-32 derivation path string, e.g.
///   `"m/44'/60'/0'/0"`.
/// - `xpub`      — account-level EVM extended public key. The raw byte
///   representation is hex-encoded to form the `account_xpub` field and the
///   fingerprint prefix.
///
/// # Returns
///
/// A fully populated [`export::WalletExport`] with `watch_only = true` and
/// no `account_xprv`.
pub fn build_watch_only_export(
  profile: &EvmProfile,
  info: &[&str],
  base_path: &str,
  xpub: EvmAccountXpub,
) -> export::WalletExport {
  let xpub_bytes = xpub.to_bytes();

  export::WalletExport {
    software: export::SoftwareInfo {
      name: info.first().copied().unwrap_or("seedctl").to_string(),
      version: info.get(1).copied().unwrap_or("unknown").to_string(),
      repository: info.get(2).copied().unwrap_or("unknown").to_string(),
    },
    network: profile.export_network.into(),
    script_type: profile.export_script_type.into(),
    key_origin: export::KeyOrigin {
      // Use the first 4 raw bytes of the xpub as a fingerprint substitute.
      fingerprint: hex::encode(&xpub_bytes[..4]),
      derivation_path: base_path.into(),
    },
    watch_only: true,
    keys: export::Keys {
      account_xpub: hex::encode(xpub_bytes),
      // Watch-only export: private key is intentionally omitted.
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      // EVM chains use a single descriptor style for both receive and change.
      receive: profile.export_descriptor.into(),
      change: profile.export_descriptor.into(),
    },
  }
}
