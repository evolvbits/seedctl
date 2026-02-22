use seedctl_core::export;
use std::error::Error;

pub fn build_export(
  info: &[&str],
  base_path: &str,
  xpub: bip32::ExtendedPublicKey<k256::ecdsa::VerifyingKey>,
) -> Result<export::WalletExport, Box<dyn Error>> {
  Ok(export::WalletExport {
    software: export::SoftwareInfo {
      name: info[0].to_string(),
      version: info[1].to_string(),
      repository: info[2].to_string(),
    },
    network: "polygon".into(),
    script_type: "polygon-evm-bip44".into(),
    key_origin: export::KeyOrigin {
      fingerprint: hex::encode(&xpub.to_bytes()[0..4]),
      derivation_path: base_path.into(),
    },
    watch_only: true,
    keys: export::Keys {
      account_xpub: hex::encode(xpub.to_bytes()),
      account_xprv: None,
    },
    descriptors: export::Descriptors {
      receive: "polygon-address".into(),
      change: "polygon-address".into(),
    },
  })
}
