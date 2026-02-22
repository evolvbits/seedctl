use serde::Serialize;

#[derive(Serialize)]
pub struct WalletExport {
  pub software: SoftwareInfo,
  pub network: String,
  pub script_type: String,
  pub key_origin: KeyOrigin,
  pub watch_only: bool,
  pub keys: Keys,
  pub descriptors: Descriptors,
}

#[derive(Serialize)]
pub struct SoftwareInfo {
  pub name: String,
  pub version: String,
  pub repository: String,
}

#[derive(Serialize)]
pub struct KeyOrigin {
  pub fingerprint: String,
  pub derivation_path: String,
}

#[derive(Serialize)]
pub struct Keys {
  pub account_xpub: String,
  pub account_xprv: Option<String>,
}

#[derive(Serialize)]
pub struct Descriptors {
  pub receive: String,
  pub change: String,
}
