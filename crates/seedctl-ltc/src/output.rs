use seedctl_core::ui::{AddressRows, print_standard_wallet};

pub struct WalletOutput<'a> {
  pub coin_type: u32,
  pub fingerprint: &'a [u8; 4],
  pub account_xprv: &'a str,
  pub account_xpub: &'a str,
  pub addresses: &'a [(String, String, Option<f64>)],
}

pub fn print_wallet_output(output: &WalletOutput<'_>) {
  print_standard_wallet(
    "Litecoin Wallet",
    84,
    output.coin_type,
    Some(output.fingerprint),
    Some(output.account_xprv),
    output.account_xpub,
    AddressRows::WithBalance(output.addresses),
    vec![
      ("Output Descriptor (receive):", "ltc-bip84"),
      ("Output Descriptor (change):", "ltc-bip84"),
    ],
  );
}
