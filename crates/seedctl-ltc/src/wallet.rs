use seedctl_core::{export, utils::format_fingerprint_hex};

use crate::prompts::LtcNetwork;

pub struct BuildExport<'a> {
  pub info: &'a [&'a str],
  pub network: LtcNetwork,
  pub script_type: &'a str,
  pub derivation_path: &'a str,
  pub fingerprint: &'a [u8; 4],
  pub account_xpub: &'a str,
}

pub fn build_export(output: &BuildExport<'_>) -> export::WalletExport {
  let network_str = match output.network {
    LtcNetwork::Mainnet => "litecoin",
    LtcNetwork::Testnet => "litecoin-testnet",
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
    watch_only: true,
    keys: export::Keys {
      account_xpub: output.account_xpub.to_string(),
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      receive: "ltc-bip84".into(),
      change: "ltc-bip84".into(),
    },
  }
}
