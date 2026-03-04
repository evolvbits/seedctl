//! Cardano key derivation using the Icarus (CIP-0003) master-key scheme.
//!
//! Implements the full CIP-1852 derivation flow:
//!
//! 1. [`master_from_mnemonic_icarus`] — produces the Icarus master `XPrv` from
//!    BIP-39 entropy + passphrase via PBKDF2-HMAC-SHA512.
//! 2. [`derive_account`] — derives the account key pair at
//!    `m/1852'/1815'/account'` together with the stake key.
//! 3. [`keypair_and_address`] — produces a payment `XPrv` and a bech32
//!    Shelley base address for a given address index.

use bech32::{Bech32, Hrp};
use bip39::Mnemonic;
use cryptoxide::{blake2b::Blake2b, digest::Digest};
use ed25519_bip32::{DerivationIndex, DerivationScheme, XPrv, XPub};
use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha512;
use std::error::Error;

use crate::prompts::AdaNetwork;

/// Hardened-index offset (bit 31 set) used in CIP-1852 derivation paths.
const HARDENED: u32 = 0x8000_0000;

/// CIP-1852 derivation purpose for Shelley-era Cardano wallets.
const PURPOSE_CIP1852: u32 = 1852;

/// SLIP-44 coin type for Cardano (ADA).
const COIN_TYPE_ADA: u32 = 1815;

/// Account-level key pair plus the stake key derived alongside it.
///
/// Produced by [`derive_account`] and consumed by [`keypair_and_address`]
/// to generate individual payment addresses.
pub struct AdaAccount {
  /// Account-level extended private key at `m/1852'/1815'/account'`.
  pub account_xprv: XPrv,

  /// Account-level extended public key derived from [`account_xprv`](Self::account_xprv).
  pub account_xpub: XPub,

  /// Stake (reward) extended public key at `m/1852'/1815'/account'/2/0`.
  ///
  /// Embedded in every Shelley base address to enable staking rewards.
  pub stake_xpub: XPub,
}

/// Returns the CIP-1852 account derivation path string for the given account index.
///
/// # Example
///
/// ```
/// assert_eq!(account_path(0), "m/1852'/1815'/0'");
/// ```
pub fn account_path(account: u32) -> String {
  format!("m/1852'/1815'/{}'", account)
}

/// Returns the CIP-1852 payment key derivation path string for the given
/// account and address indices.
///
/// # Example
///
/// ```
/// assert_eq!(payment_path(0, 3), "m/1852'/1815'/0'/0/3");
/// ```
pub fn payment_path(account: u32, index: u32) -> String {
  format!("m/1852'/1815'/{}'/0/{}", account, index)
}

/// Derives the Icarus (CIP-0003) master extended private key from a BIP-39
/// mnemonic and an optional passphrase.
///
/// # Algorithm
///
/// 1. Extract the raw entropy bytes from the mnemonic (NOT the 64-byte BIP-39
///    seed — Icarus uses the entropy directly).
/// 2. Run PBKDF2-HMAC-SHA512 with the passphrase as the password and the
///    entropy as the salt for 4096 iterations, producing 96 bytes.
/// 3. Prune the scalar bits to ensure it is a valid Ed25519 scalar:
///    - Clear the three lowest bits of byte 0 (`&= 0b1111_1000`).
///    - Clear the top 3 bits of byte 31 (`&= 0b0001_1111`).
///    - Set the second-highest bit of byte 31 (`|= 0b0100_0000`).
/// 4. Treat bytes `0..64` as the extended scalar and bytes `64..96` as the
///    chain code to form the root `XPrv`.
///
/// # Parameters
///
/// - `mnemonic`   — validated BIP-39 mnemonic object.
/// - `passphrase` — BIP-39 passphrase (empty string `""` for no passphrase).
pub fn master_from_mnemonic_icarus(mnemonic: &Mnemonic, passphrase: &str) -> XPrv {
  // CIP-0003 (Icarus): derive master key from entropy + passphrase via PBKDF2-HMAC-SHA512.
  let entropy = mnemonic.to_entropy();
  let mut data = pbkdf2_hmac_array::<Sha512, 96>(passphrase.as_bytes(), &entropy, 4096);

  // Prune scalar bits to produce a valid Ed25519 private key.
  data[0] &= 0b1111_1000;
  data[31] &= 0b0001_1111;
  data[31] |= 0b0100_0000;

  let mut extended = [0u8; 64];
  extended.copy_from_slice(&data[..64]);

  let mut chain_code = [0u8; 32];
  chain_code.copy_from_slice(&data[64..]);

  XPrv::from_extended_and_chaincode(&extended, &chain_code)
}

/// Derives the account-level key pair at `m/1852'/1815'/account'` and the
/// stake key at `m/1852'/1815'/account'/2/0`.
///
/// # Parameters
///
/// - `master`  — Icarus master `XPrv` produced by [`master_from_mnemonic_icarus`].
/// - `account` — account index (typically `0`); hardened automatically.
///
/// # Returns
///
/// An [`AdaAccount`] containing the account `XPrv`, the account `XPub`, and
/// the stake `XPub` needed to construct Shelley base addresses.
pub fn derive_account(master: &XPrv, account: u32) -> AdaAccount {
  let account_xprv = derive_path(
    master.clone(),
    &[
      hardened(PURPOSE_CIP1852),
      hardened(COIN_TYPE_ADA),
      hardened(account),
    ],
  );

  let account_xpub = account_xprv.public();

  // Stake key lives at role 2, index 0 within the account.
  let stake_xprv = derive_path(account_xprv.clone(), &[2, 0]);
  let stake_xpub = stake_xprv.public();

  AdaAccount {
    account_xprv,
    account_xpub,
    stake_xpub,
  }
}

/// Derives the payment extended private key at role 0, index `index` within
/// the given account key.
///
/// Equivalent to `account_xprv / 0 / index` using the Shelley derivation
/// scheme (V2).
pub fn derive_payment_xprv(account_xprv: &XPrv, index: u32) -> XPrv {
  derive_path(account_xprv.clone(), &[0, index])
}

/// Encodes a Shelley base address from a payment public key and a stake
/// public key using Blake2b-224 hashes.
///
/// # Address structure
///
/// ```text
/// [ header (1 byte) | payment_key_hash (28 bytes) | stake_key_hash (28 bytes) ]
/// ```
///
/// The header byte encodes the address type (enterprise = `0b0000`) and the
/// network ID in its low nibble.
///
/// # Errors
///
/// Returns a boxed error if the HRP string is invalid or if bech32 encoding
/// fails.
pub fn address_from_payment_key(
  payment_xpub: &XPub,
  stake_xpub: &XPub,
  network: AdaNetwork,
) -> Result<String, Box<dyn Error>> {
  let payment_hash = blake2b224(&payment_xpub.public_key());
  let stake_hash = blake2b224(&stake_xpub.public_key());

  // Shelley base address: header(4-bit type + 4-bit network) + payment_hash + stake_hash.
  let header = network.base_header();
  let mut bytes = Vec::with_capacity(1 + 28 + 28);
  bytes.push(header);
  bytes.extend_from_slice(&payment_hash);
  bytes.extend_from_slice(&stake_hash);

  Ok(bech32::encode::<Bech32>(
    Hrp::parse(network.hrp())?,
    &bytes,
  )?)
}

/// Derives the payment key pair and bech32 base address for the given address
/// index within an account.
///
/// # Parameters
///
/// - `account` — account struct produced by [`derive_account`].
/// - `index`   — address index within the external (payment) chain.
/// - `network` — target network determining the address HRP and header byte.
///
/// # Returns
///
/// A tuple of `(payment_XPrv, bech32_address)`.
///
/// # Errors
///
/// Returns a boxed error if bech32 encoding fails.
pub fn keypair_and_address(
  account: &AdaAccount,
  index: u32,
  network: AdaNetwork,
) -> Result<(XPrv, String), Box<dyn Error>> {
  let payment_xprv = derive_payment_xprv(&account.account_xprv, index);
  let payment_xpub = payment_xprv.public();
  let address = address_from_payment_key(&payment_xpub, &account.stake_xpub, network)?;

  Ok((payment_xprv, address))
}

/// Applies a sequence of derivation indices to a root `XPrv` using the
/// Shelley derivation scheme (V2), returning the fully derived descendant key.
fn derive_path(mut key: XPrv, segments: &[DerivationIndex]) -> XPrv {
  for segment in segments {
    key = key.derive(DerivationScheme::V2, *segment);
  }
  key
}

/// Returns the hardened index for the given soft index value.
///
/// Sets bit 31 (`index | 0x8000_0000`) as required by the Cardano
/// derivation scheme.
#[inline]
fn hardened(index: u32) -> DerivationIndex {
  index | HARDENED
}

/// Computes the Blake2b-224 hash (28 bytes) of `data`.
///
/// Used to produce payment and stake key hashes embedded in Shelley
/// base addresses.
fn blake2b224(data: &[u8]) -> [u8; 28] {
  let mut hasher = Blake2b::new(28);
  hasher.input(data);

  let mut out = [0u8; 28];
  hasher.result(&mut out);
  out
}
