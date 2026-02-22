use seedctl_core::export;
use std::error::Error;

pub fn build_export(
  info: &[&str],
  first_pubkey_hex: &str,
) -> Result<export::WalletExport, Box<dyn Error>> {
  Ok(export::WalletExport {
    software: export::SoftwareInfo {
      name: info[0].to_string(),
      version: info[1].to_string(),
      repository: info[2].to_string(),
    },
    network: "solana".into(),
    script_type: "solana-ed25519".into(),
    key_origin: export::KeyOrigin {
      fingerprint: first_pubkey_hex.get(..8).unwrap_or("").to_string(),
      derivation_path: "m/44'/501'/0'/0'".into(),
    },
    watch_only: true,
    keys: export::Keys {
      account_xpub: first_pubkey_hex.to_string(),
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      receive: "solana-address".into(),
      change: "solana-address".into(),
    },
  })
}
