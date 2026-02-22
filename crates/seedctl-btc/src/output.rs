use seedctl_core::ui::{AddressRows, print_standard_wallet};

// Wallet data for printing (path, fingerprint, keys, descriptors, addresses).
pub struct WalletOutput<'a> {
  pub purpose: u32,
  pub coin_type: u32,
  pub fingerprint: &'a [u8; 4],
  pub account_xprv: &'a str,
  pub account_xpub: &'a str,
  pub desc_receive: &'a str,
  pub desc_change: &'a str,
  pub addresses: &'a [(String, String)],
}

pub fn print_wallet_output(output: &WalletOutput<'_>) {
  print_standard_wallet(
    "Bitcoin Wallet",
    output.purpose,
    output.coin_type,
    Some(output.fingerprint),
    Some(output.account_xprv),
    output.account_xpub,
    AddressRows::Basic(output.addresses),
    vec![
      ("Output Descriptor (receive):", output.desc_receive),
      ("Output Descriptor (change):", output.desc_change),
    ],
  );
}
