use crate::traits::chain::Chain;

/// Generic wallet container for any `Chain` implementation.
pub struct Wallet<C: Chain> {
  pub account: u32,
  pub private: C::PrivateKey,
  pub public: C::PublicKey,
  pub addresses: Vec<C::Address>,
}

impl<C: Chain> Wallet<C> {
  /// Builds a new wallet for the given chain, deriving:
  /// - account key at `m/purpose'/coin_type'/account'` (defined by `C`)
  /// - public key from the private/account key
  /// - `count` receive addresses from the public key
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
