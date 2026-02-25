use seedctl_core::export;

use crate::derive;
use crate::prompts::AdaNetwork;

pub fn build_export(
  info: &[&str],
  network: AdaNetwork,
  account: u32,
  account_xpub_hex: &str,
) -> export::WalletExport {
  export::WalletExport {
    software: export::SoftwareInfo {
      name: info.first().copied().unwrap_or("seedctl").to_string(),
      version: info.get(1).copied().unwrap_or("unknown").to_string(),
      repository: info.get(2).copied().unwrap_or("unknown").to_string(),
    },
    network: network.export_network().into(),
    script_type: "cardano-cip1852-shelley-base".into(),
    key_origin: export::KeyOrigin {
      fingerprint: account_xpub_hex.get(..8).unwrap_or("").to_string(),
      derivation_path: derive::account_path(account),
    },
    watch_only: true,
    keys: export::Keys {
      account_xpub: account_xpub_hex.to_string(),
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      receive: "cardano-base-address".into(),
      change: "cardano-base-address".into(),
    },
  }
}
