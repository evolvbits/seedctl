/// Core abstraction for a BIP‑39/BIP‑32 compatible chain.
///
/// The goal is to centralize the common derivation logic so that each
/// `seedctl-*` crate only needs to implement this trait instead of
/// duplicating derivation and address generation functions.
pub trait Chain {
  /// Address type used by this chain (e.g. bech32 string wrapper, base58, etc).
  type Address;

  /// Public key type (usually an extended or raw public key).
  type PublicKey;

  /// Private key / account key type.
  type PrivateKey;

  /// Human‑readable chain name, e.g. `"Bitcoin (BTC)"`.
  fn name() -> &'static str;

  /// Ticker symbol, e.g. `"BTC"`, `"LTC"`, `"ETH"`.
  fn symbol() -> &'static str;

  /// SLIP‑44 coin type for this chain.
  fn coin_type() -> u32;

  /// Derive the account‑level private key at `m/purpose'/coin_type'/account'`.
  ///
  /// `seed` is the BIP‑39 seed (64 bytes) produced from mnemonic+passphrase.
  fn derive_account(seed: &[u8], account: u32) -> Self::PrivateKey;

  /// Compute a public key from a private/account key.
  fn public_from_private(privkey: &Self::PrivateKey) -> Self::PublicKey;

  /// Derive a batch of receive addresses from an account‑level public key.
  ///
  /// `count` is typically 10–20 for CLI display.
  fn derive_addresses(pubkey: &Self::PublicKey, count: u32) -> Vec<Self::Address>;
}
