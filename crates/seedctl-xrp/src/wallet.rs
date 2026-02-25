use seedctl_core::export;

use crate::prompts::XrpNetwork;

pub fn build_export(
  info: &[&str],
  network: XrpNetwork,
  base_path: &str,
  account_xpub: &str,
) -> export::WalletExport {
  export::WalletExport {
    software: export::SoftwareInfo {
      name: info.first().copied().unwrap_or("seedctl").to_string(),
      version: info.get(1).copied().unwrap_or("unknown").to_string(),
      repository: info.get(2).copied().unwrap_or("unknown").to_string(),
    },
    network: network.export_network().into(),
    script_type: "xrpl-bip44-secp256k1".into(),
    key_origin: export::KeyOrigin {
      fingerprint: account_xpub.get(..8).unwrap_or("").to_string(),
      derivation_path: base_path.into(),
    },
    watch_only: true,
    keys: export::Keys {
      account_xpub: account_xpub.to_string(),
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      receive: "xrp-classic-address".into(),
      change: "xrp-classic-address".into(),
    },
  }
}
