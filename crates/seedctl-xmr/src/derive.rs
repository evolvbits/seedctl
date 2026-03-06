//! Monero (XMR) key derivation and address encoding.
//!
//! Implements the full Monero wallet key flow on top of the Ed25519 / Ristretto
//! curve (`curve25519-dalek`):
//!
//! 1. [`wallet_from_bip39_seed`] — derives the spend and view key scalars from
//!    a BIP-39 64-byte seed via the Monero `Hs()` hash-to-scalar function.
//! 2. [`derive_address`] — produces the standard address (index 0) or a
//!    subaddress (index > 0) using the official Monero subaddress scheme.
//! 3. [`encode_address`] — assembles the Monero address byte layout and
//!    encodes it with the Monero-specific Base58 block encoding.
//!
//! # Key derivation flow
//!
//! ```text
//! BIP-39 seed (64 bytes)
//!     │
//!     ▼  Hs() = Keccak-256 mod l
//! spend_private  ──►  spend_public = spend_private * G
//!     │
//!     ▼  Hs(spend_private)
//! view_private   ──►  view_public  = view_private  * G
//! ```
//!
//! where `G` is the Ed25519 base point and `l` is the Ed25519 group order.
//!
//! # Subaddress scheme (CryptoNote / Monero specification)
//!
//! For `index > 0` (minor index, major always 0):
//!
//! ```text
//! m   = Hs("SubAddr\0" || view_private || major_le32 || minor_le32)
//! D   = spend_public + m * G          (subaddress spend public key)
//! C   = D * view_private              (subaddress view public key)
//! ```

use curve25519_dalek::{constants::ED25519_BASEPOINT_TABLE, edwards::EdwardsPoint, scalar::Scalar};
use ed25519_hd_key::derive_from_path;
use sha3::{Digest, Keccak256};

use crate::prompts::{XmrDerivationMode, XmrNetwork};

// ── Monero Base58 encoding constants ─────────────────────────────────────────

/// The Monero Base58 alphabet.
///
/// Identical to the standard Bitcoin Base58 alphabet. Monero Base58 differs
/// from Bitcoin's **only** in the block-encoding scheme (8-byte blocks encoded
/// to 11 characters), not in the character set itself.
const MONERO_B58_ALPHABET: &[u8; 58] =
  b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Lookup table: input block length (0–8 bytes) → encoded character count.
///
/// Used by [`encode_block`] to determine how many Base58 characters are needed
/// to represent each chunk of the address payload.
const ENCODED_BLOCK_SIZES: [usize; 9] = [0, 2, 3, 5, 6, 7, 9, 10, 11];

// ── Public types ──────────────────────────────────────────────────────────────

/// Monero wallet key material derived from a BIP-39 seed.
///
/// Holds the two private scalars and their corresponding Ed25519 public keys.
/// The public keys are `EdwardsPoint` values on the Ed25519 curve, which are
/// compressed to 32 bytes when encoding addresses.
///
/// # Security
///
/// Both `spend_private` and `view_private` are cryptographic secrets.
/// The `spend_private` key controls spending; the `view_private` key allows
/// scanning the blockchain for incoming transactions without being able to
/// spend funds.
pub struct XmrWallet {
  /// Spend private key scalar (`spend` in Monero terminology).
  ///
  /// Derived as `Hs(seed)` — Keccak-256 of the 64-byte BIP-39 seed, reduced
  /// modulo the Ed25519 group order `l`.
  spend_private: Scalar,

  /// View private key scalar (`view` in Monero terminology).
  ///
  /// Derived as `Hs(spend_private)` — Keccak-256 of the spend private key
  /// bytes, reduced modulo `l`.
  view_private: Scalar,

  /// Spend public key: `spend_private * G`.
  ///
  /// Embedded in standard and subaddress Monero addresses as the first 32
  /// bytes after the network prefix byte.
  spend_public: EdwardsPoint,

  /// View public key: `view_private * G`.
  ///
  /// Embedded in standard and subaddress Monero addresses as the second 32
  /// bytes after the spend public key.
  view_public: EdwardsPoint,
}

/// A single derived Monero address with its associated path label.
///
/// Returned by [`derive_address`] and collected in [`crate::run`] to build
/// the address table shown to the user.
pub struct DerivedAddress {
  /// Human-readable path label used in the address table, e.g.
  /// `"xmr(major=0,minor=3)"`.
  pub path: String,

  /// Base58-encoded Monero address string (95 characters for standard
  /// addresses, 97 for subaddresses).
  pub address: String,
}

// ── XmrWallet accessors ───────────────────────────────────────────────────────

impl XmrWallet {
  /// Returns the spend private key as a lowercase hex string (64 hex chars).
  ///
  /// Used by [`crate::run`] to populate the `account_xprv` display field.
  pub fn spend_private_hex(&self) -> String {
    hex::encode(self.spend_private.to_bytes())
  }

  /// Returns the spend public key as a lowercase hex string (64 hex chars).
  ///
  /// Used by [`crate::run`] to populate the `account_xpub` display field and
  /// the watch-only export fingerprint.
  pub fn spend_public_hex(&self) -> String {
    hex::encode(self.spend_public.compress().to_bytes())
  }
}

// ── Key derivation ────────────────────────────────────────────────────────────

/// Derives a complete Monero [`XmrWallet`] from a BIP-39 seed and mode.
///
/// # Algorithm
///
/// ```text
/// Native mode:
/// spend_private = Hs(seed)              // Keccak-256(seed) mod l
/// view_private  = Hs(spend_private)     // Keccak-256(spend_private) mod l
/// spend_public  = spend_private * G
/// view_public   = view_private  * G
///
/// WalletCore mode:
/// spend_bytes   = SLIP10(seed, "m/44'/128'/0'/0'/0'")
/// spend_private = mod_l(spend_bytes)
/// view_private  = Hs(spend_private)
/// ```
///
/// # Parameters
///
/// - `seed` — 64-byte BIP-39 seed produced by `Mnemonic::to_seed(passphrase)`.
/// - `mode` — derivation mode selected by the user.
///
/// # Returns
///
/// A fully initialised [`XmrWallet`] ready for address derivation.
pub fn wallet_from_bip39_seed(seed: &[u8], mode: XmrDerivationMode) -> XmrWallet {
  let spend_private = match mode {
    XmrDerivationMode::Native => scalar_from_hash(seed),
    XmrDerivationMode::WalletCore => {
      let (spend_bytes, _chain_code) = derive_from_path("m/44'/128'/0'/0'/0'", seed);
      Scalar::from_bytes_mod_order(spend_bytes)
    }
  };

  let view_private = scalar_from_hash(&spend_private.to_bytes());

  // Compute public keys as scalar multiples of the Ed25519 base point.
  let spend_public = &spend_private * ED25519_BASEPOINT_TABLE;
  let view_public = &view_private * ED25519_BASEPOINT_TABLE;

  XmrWallet {
    spend_private,
    view_private,
    spend_public,
    view_public,
  }
}

// ── Address derivation ────────────────────────────────────────────────────────

/// Returns the Monero address path label for the given address `index`.
///
/// Uses the `xmr(major=0,minor=<index>)` notation, where:
/// - `major` is always `0` (account index, not incremented in this crate).
/// - `minor` is the subaddress index within the account.
///
/// # Examples
///
/// ```ignore
/// // Index 0 = standard address.
/// assert_eq!(derivation_path(0), "xmr(major=0,minor=0)");
/// // Index 3 = third subaddress.
/// assert_eq!(derivation_path(3), "xmr(major=0,minor=3)");
/// ```
pub fn derivation_path(index: u32) -> String {
  format!("xmr(major=0,minor={index})")
}

/// Derives the Monero address (standard or subaddress) for the given `index`.
///
/// - **`index == 0`**: returns the standard address, which directly embeds
///   `spend_public` and `view_public`.
/// - **`index > 0`**: returns a subaddress using the official Monero
///   subaddress derivation scheme (see module-level documentation).
///
/// # Parameters
///
/// - `wallet`  — wallet keys produced by [`wallet_from_bip39_seed`].
/// - `network` — target network; determines the prefix byte embedded in the
///   encoded address.
/// - `index`   — subaddress index within account 0 (minor index). `0` yields
///   the standard (primary) address.
///
/// # Returns
///
/// A [`DerivedAddress`] containing the path label and the encoded address.
pub fn derive_address(wallet: &XmrWallet, network: XmrNetwork, index: u32) -> DerivedAddress {
  let address = if index == 0 {
    // Index 0: standard address — use the wallet's public keys directly.
    encode_address(
      network.standard_prefix(),
      &wallet.spend_public.compress().to_bytes(),
      &wallet.view_public.compress().to_bytes(),
    )
  } else {
    // Index > 0: subaddress for account (major) = 0, minor = index.
    let m = subaddress_secret(&wallet.view_private, 0, index);

    // D = spend_public + m * G  (subaddress spend public key)
    let d = wallet.spend_public + (&m * ED25519_BASEPOINT_TABLE);

    // C = D * view_private  (subaddress view public key)
    let c = d * wallet.view_private;

    encode_address(
      network.subaddress_prefix(),
      &d.compress().to_bytes(),
      &c.compress().to_bytes(),
    )
  };

  DerivedAddress {
    path: derivation_path(index),
    address,
  }
}

// ── Internal helpers ──────────────────────────────────────────────────────────

/// Derives the subaddress secret scalar for account `major`, index `minor`.
///
/// Implements the Monero subaddress key derivation:
///
/// ```text
/// m = Hs("SubAddr\0" || view_private || major_le32 || minor_le32)
/// ```
///
/// where `Hs` is Keccak-256 reduced modulo the Ed25519 group order `l`.
///
/// # Parameters
///
/// - `view_private` — wallet view private scalar.
/// - `major`        — account index (always `0` in this crate).
/// - `minor`        — subaddress index within the account.
///
/// # Returns
///
/// A scalar `m` used to offset the spend public key for the subaddress.
fn subaddress_secret(view_private: &Scalar, major: u32, minor: u32) -> Scalar {
  // Build the hash input as specified by the Monero subaddress specification.
  let mut data = Vec::with_capacity(8 + 32 + 4 + 4);
  data.extend_from_slice(b"SubAddr\0"); // Fixed 8-byte domain separator.
  data.extend_from_slice(&view_private.to_bytes()); // 32-byte view private key.
  data.extend_from_slice(&major.to_le_bytes()); // 4-byte major index (LE).
  data.extend_from_slice(&minor.to_le_bytes()); // 4-byte minor index (LE).

  scalar_from_hash(&data)
}

/// Assembles a Monero address from its components and Base58-encodes it.
///
/// Address byte layout:
///
/// ```text
/// [ prefix (1 byte) | spend_public (32 bytes) | view_public (32 bytes) | checksum (4 bytes) ]
/// ```
///
/// The 4-byte checksum is the first 4 bytes of `Keccak-256(payload)` where
/// `payload` is the prefix + public keys without the checksum.
///
/// # Parameters
///
/// - `prefix`        — network/type prefix byte (e.g. `18` for Monero Mainnet
///   standard address, `42` for Mainnet subaddress).
/// - `spend_public`  — 32-byte compressed spend public key.
/// - `view_public`   — 32-byte compressed view public key.
///
/// # Returns
///
/// A Base58-encoded Monero address string (95 chars for standard, 97 for
/// subaddresses, due to differing prefix byte values).
fn encode_address(prefix: u8, spend_public: &[u8; 32], view_public: &[u8; 32]) -> String {
  // Build the payload: prefix + spend key + view key (65 bytes total).
  let mut payload = Vec::with_capacity(1 + 32 + 32);
  payload.push(prefix);
  payload.extend_from_slice(spend_public);
  payload.extend_from_slice(view_public);

  // Compute the 4-byte Keccak-256 checksum and append it.
  let checksum = keccak256(&payload);
  payload.extend_from_slice(&checksum[..4]);

  // Encode with the Monero block-based Base58 scheme.
  monero_base58_encode(&payload)
}

/// Applies Keccak-256 to `data` and reduces the result modulo the Ed25519
/// group order `l` to produce a valid scalar.
///
/// This is the Monero `Hs()` (hash-to-scalar) function, used for both master
/// key derivation and subaddress secret generation.
///
/// # Parameters
///
/// - `data` — arbitrary byte slice to hash and reduce.
///
/// # Returns
///
/// A [`Scalar`] in canonical reduced form (value in `[0, l)`).
fn scalar_from_hash(data: &[u8]) -> Scalar {
  let hash = keccak256(data);
  // `from_bytes_mod_order` reduces the 32-byte hash modulo l.
  Scalar::from_bytes_mod_order(hash)
}

/// Computes Keccak-256 over `data` and returns the 32-byte digest.
///
/// Used both for the `Hs()` hash-to-scalar function and for computing the
/// 4-byte Monero address checksum.
///
/// # Parameters
///
/// - `data` — arbitrary byte slice to hash.
///
/// # Returns
///
/// A 32-byte array containing the Keccak-256 digest of `data`.
fn keccak256(data: &[u8]) -> [u8; 32] {
  let mut hasher = Keccak256::new();
  hasher.update(data);
  let out = hasher.finalize();

  let mut bytes = [0u8; 32];
  bytes.copy_from_slice(&out);
  bytes
}

// ── Monero Base58 block encoding ──────────────────────────────────────────────

/// Encodes `data` using the Monero block-based Base58 scheme.
///
/// Unlike standard Base58Check (which encodes the entire payload as one big
/// integer), Monero Base58 splits `data` into 8-byte blocks, encodes each
/// block independently to a fixed-width Base58 string, and concatenates the
/// results.
///
/// The last block may be shorter than 8 bytes; its encoded width is looked up
/// from [`ENCODED_BLOCK_SIZES`].
///
/// # Parameters
///
/// - `data` — byte slice to encode (typically 69 bytes: 65-byte address
///   payload + 4-byte checksum).
///
/// # Returns
///
/// A Base58-encoded string using the [`MONERO_B58_ALPHABET`].
fn monero_base58_encode(data: &[u8]) -> String {
  let mut out = String::new();

  // Process each 8-byte chunk independently.
  for chunk in data.chunks(8) {
    let encoded_size = ENCODED_BLOCK_SIZES[chunk.len()];
    out.push_str(&encode_block(chunk, encoded_size));
  }

  out
}

/// Encodes a single block of up to 8 bytes as a fixed-width Base58 string.
///
/// Interprets `block` as a big-endian unsigned integer, then repeatedly divides
/// by 58 to extract Base58 digits from least significant to most significant,
/// filling a fixed-width output buffer (right-aligned, left-padded with `'1'`).
///
/// # Parameters
///
/// - `block`        — up to 8 bytes to encode as a big-endian integer.
/// - `encoded_size` — exact number of Base58 characters in the output, looked
///   up from [`ENCODED_BLOCK_SIZES`] based on `block.len()`.
///
/// # Returns
///
/// A string of exactly `encoded_size` characters from [`MONERO_B58_ALPHABET`].
fn encode_block(block: &[u8], encoded_size: usize) -> String {
  // Interpret the block bytes as a big-endian u128 integer.
  let mut num: u128 = 0;
  for &byte in block {
    num = (num << 8) | byte as u128;
  }

  // Pre-fill the output with '1' (Base58 zero digit) to handle leading zeros.
  let mut out = vec!['1'; encoded_size];
  let mut idx = encoded_size;

  // Extract Base58 digits from least significant to most significant.
  while num > 0 {
    let rem = (num % 58) as usize;
    num /= 58;

    if idx == 0 {
      break;
    }

    idx -= 1;
    out[idx] = MONERO_B58_ALPHABET[rem] as char;
  }

  out.into_iter().collect()
}

#[cfg(test)]
mod tests {
  use super::{derive_address, wallet_from_bip39_seed};
  use crate::prompts::{XmrDerivationMode, XmrNetwork};

  #[test]
  fn walletcore_mode_derives_address() {
    let seed = [7u8; 64];
    let wallet = wallet_from_bip39_seed(&seed, XmrDerivationMode::WalletCore);
    let addr = derive_address(&wallet, XmrNetwork::Mainnet, 0);
    assert!(addr.address.starts_with('4'));
  }

  #[test]
  fn native_mode_derives_address() {
    let seed = [11u8; 64];
    let wallet = wallet_from_bip39_seed(&seed, XmrDerivationMode::Native);
    let addr = derive_address(&wallet, XmrNetwork::Mainnet, 0);
    assert!(addr.address.starts_with('4'));
  }
}
