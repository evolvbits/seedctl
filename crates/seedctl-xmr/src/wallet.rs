use seedctl_core::export;

use crate::prompts::XmrNetwork;

pub fn build_export(
  info: &[&str],
  network: XmrNetwork,
  first_public_hex: &str,
) -> export::WalletExport {
  export::WalletExport {
    software: export::SoftwareInfo {
      name: info.first().copied().unwrap_or("seedctl").to_string(),
      version: info.get(1).copied().unwrap_or("unknown").to_string(),
      repository: info.get(2).copied().unwrap_or("unknown").to_string(),
    },
    network: network.export_network().into(),
    script_type: "monero-standard-subaddress".into(),
    key_origin: export::KeyOrigin {
      fingerprint: first_public_hex.get(..8).unwrap_or("").to_string(),
      derivation_path: "m/44'/128'/0'/0/0".into(),
    },
    watch_only: true,
    keys: export::Keys {
      account_xpub: first_public_hex.to_string(),
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      receive: "monero-subaddress".into(),
      change: "monero-subaddress".into(),
    },
  }
}
