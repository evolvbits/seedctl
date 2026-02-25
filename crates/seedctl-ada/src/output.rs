use seedctl_core::ui::{AddressRows, print_standard_wallet};

pub struct WalletOutput<'a> {
  pub purpose: u32,
  pub coin_type: u32,
  pub account_xprv: &'a str,
  pub account_xpub: &'a str,
  pub show_privkeys: bool,
  pub addresses: &'a [(String, String)],
}

pub fn print_wallet_output(output: &WalletOutput<'_>) {
  let account_xprv_opt = if output.show_privkeys {
    Some(output.account_xprv)
  } else {
    None
  };

  print_standard_wallet(
    "Cardano Wallet",
    output.purpose,
    output.coin_type,
    None,
    account_xprv_opt,
    output.account_xpub,
    AddressRows::Basic(output.addresses),
    vec![],
  );
}
