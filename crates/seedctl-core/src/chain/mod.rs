use crate::error::SeedCtlError;

/// Chain‑specific address derivation interface operating on mnemonic/paths.
pub trait Chain {
  fn name(&self) -> &'static str;
  fn coin_type(&self) -> u32;

  fn derive_address(
    &self,
    mnemonic: &str,
    passphrase: Option<&str>,
    account: u32,
    index: u32,
  ) -> Result<String, SeedCtlError>;
}

/// Shared context for chain derivation calls.
pub struct ChainContext<'a> {
  pub mnemonic: &'a str,
  pub passphrase: Option<&'a str>,
  pub account: u32,
  pub index: u32,
}
