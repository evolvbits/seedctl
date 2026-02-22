use bip39::Mnemonic;
use bitcoin::{Network, bip32::Xpriv};
use seedctl_core::{export, utils::format_fingerprint_hex};
use std::error::Error;

use crate::utils::{
  crypto::{xprv_to_yprv, xprv_to_zprv, xpub_to_ypub, xpub_to_zpub},
  format::{format_key_origin, output_descriptor},
};

/// Gera a chave mestra a partir do mnemonic e passphrase.
pub fn master_from_mnemonic(
  mnemonic: &Mnemonic,
  passphrase: &str,
  network: Network,
) -> Result<Xpriv, Box<dyn Error>> {
  let seed = mnemonic.to_seed(passphrase);
  Ok(Xpriv::new_master(network, &seed)?)
}

pub struct BuildExport<'a> {
  pub info: &'a [&'a str],
  pub network: Network,
  pub script_type: &'a str,
  pub derivation_path: &'a str,
  pub fingerprint: &'a [u8; 4],
  pub account_xpub: &'a str,
  pub account_xprv: Option<&'a str>,
  pub desc_receive: &'a str,
  pub desc_change: &'a str,
}

/// Monta estrutura de export watch-only (JSON).
pub fn build_export(output: &BuildExport) -> export::WalletExport {
  let network_str = match output.network {
    Network::Bitcoin => "bitcoin",
    Network::Testnet => "testnet",
    _ => "unknown",
  };

  export::WalletExport {
    software: export::SoftwareInfo {
      name: output.info[0].to_string(),
      version: output.info[1].to_string(),
      repository: output.info[2].to_string(),
    },
    network: network_str.to_string(),
    script_type: output.script_type.to_string(),
    key_origin: export::KeyOrigin {
      fingerprint: format_fingerprint_hex(output.fingerprint),
      derivation_path: output.derivation_path.to_string(),
    },
    watch_only: output.account_xprv.is_none(),
    keys: export::Keys {
      account_xpub: output.account_xpub.to_string(),
      account_xprv: output.account_xprv.map(String::from),
    },
    descriptors: export::Descriptors {
      receive: output.desc_receive.to_string(),
      change: output.desc_change.to_string(),
    },
  }
}

/// Retorna strings de account xpub/xprv conforme o tipo de endereço (BIP84/49/44).
pub fn account_key_strings(
  acc_xprv: &Xpriv,
  acc_xpub: &bitcoin::bip32::Xpub,
  address_type: usize,
) -> (String, String) {
  let account_xpub = match address_type {
    0 => xpub_to_zpub(acc_xpub),
    1 => xpub_to_ypub(acc_xpub),
    2 => acc_xpub.to_string(),
    _ => unreachable!(),
  };
  let account_xprv = match address_type {
    0 => xprv_to_zprv(acc_xprv),
    1 => xprv_to_yprv(acc_xprv),
    2 => acc_xprv.to_string(),
    _ => unreachable!(),
  };
  (account_xprv, account_xpub)
}

/// Monta key_origin e descriptors usando utils::format.
pub fn key_origin_and_descriptors(
  fingerprint: [u8; 4],
  purpose: u32,
  btc_coin_type: u32,
  account_xpub: &str,
) -> (String, String, String) {
  let key_origin = format_key_origin(fingerprint, purpose, btc_coin_type);
  let desc_receive = output_descriptor(purpose, &key_origin, account_xpub, 0);
  let desc_change = output_descriptor(purpose, &key_origin, account_xpub, 1);
  (key_origin, desc_receive, desc_change)
}
