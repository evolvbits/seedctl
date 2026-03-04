//! Generic wallet container built from a [`Chain`] implementation.
//!
//! [`Wallet`] is a concrete, chain-agnostic struct that derives and stores
//! the account-level keys and a batch of receive addresses for any type that
//! implements the [`Chain`] trait from [`crate::traits::chain`].

use crate::traits::chain::Chain;

/// Generic wallet container for any [`Chain`] implementation.
///
/// Holds the account-level private key, the corresponding public key, and a
/// pre-derived batch of receive addresses, all produced from a single BIP-39
/// seed in one shot.
///
/// # Type Parameters
///
/// - `C` — a type that implements [`Chain`], defining the concrete key and
///   address types as well as the derivation logic for a specific blockchain.
///
/// # Examples
///
/// ```rust,ignore
/// // Assuming `MyChain` implements `Chain`:
/// let seed: [u8; 64] = mnemonic.to_seed("passphrase");
/// let wallet = Wallet::<MyChain>::new(&seed, 0, 10);
/// ```
pub struct Wallet<C: Chain> {
  /// The account index used to derive this wallet (typically `0`).
  pub account: u32,

  /// Account-level private (extended) key derived at
  /// `m/purpose'/coin_type'/account'`.
  pub private: C::PrivateKey,

  /// Account-level public key derived from [`private`](Self::private).
  pub public: C::PublicKey,

  /// Batch of receive addresses derived from [`public`](Self::public).
  ///
  /// The number of addresses is determined by the `count` argument passed to
  /// [`Wallet::new`].
  pub addresses: Vec<C::Address>,
}

impl<C: Chain> Wallet<C> {
  /// Builds a new wallet for the given chain by deriving:
  ///
  /// 1. The account-level private key at `m/purpose'/coin_type'/account'`
  ///    (the exact path is defined by `C::derive_account`).
  /// 2. The account-level public key via `C::public_from_private`.
  /// 3. `count` receive addresses via `C::derive_addresses`.
  ///
  /// # Parameters
  ///
  /// - `seed`    — 64-byte BIP-39 seed produced by `Mnemonic::to_seed(passphrase)`.
  /// - `account` — account index (hardened), typically `0`.
  /// - `count`   — number of receive addresses to pre-derive (typically 10–20).
  pub fn new(seed: &[u8], account: u32, count: u32) -> Self {
    let privkey = C::derive_account(seed, account);
    let pubkey = C::public_from_private(&privkey);
    let addresses = C::derive_addresses(&pubkey, count);

    Self {
      account,
      private: privkey,
      public: pubkey,
      addresses,
    }
  }
}
