use bech32::{Bech32, Hrp};
use bip32::{ChildNumber, DerivationPath, XPrv, XPub};
use ripemd::Ripemd160;
use sha2::{Digest as ShaDigest, Sha256};
use std::error::Error;

use crate::prompts::LtcNetwork;

pub fn derive_account(
  master: &XPrv,
  coin_type: u32,
) -> Result<(XPrv, XPub, [u8; 4]), Box<dyn Error>> {
  let account_path: DerivationPath = format!("m/84'/{}'/0'", coin_type).parse()?;
  let account_xprv = derive_path(master.clone(), &account_path)?;
  let account_xpub = account_xprv.public_key();
  let fingerprint = XPub::from(master).fingerprint();

  Ok((account_xprv, account_xpub, fingerprint))
}

pub fn receive_addresses(
  account_xprv: &XPrv,
  network: LtcNetwork,
  coin_type: u32,
  count: u32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
  let mut out = Vec::with_capacity(count as usize);
  let receive = ChildNumber::new(0, false)?;

  for i in 0..count {
    let index = ChildNumber::new(i, false)?;
    let xprv = account_xprv
      .clone()
      .derive_child(receive)?
      .derive_child(index)?;

    let signing = k256::ecdsa::SigningKey::from_bytes(&xprv.private_key().to_bytes())?;
    let pubkey = signing.verifying_key().to_encoded_point(true);
    let addr = ltc_bech32_p2wpkh(pubkey.as_bytes(), network)?;

    out.push((format!("m/84'/{}'/0'/0/{}", coin_type, i), addr));
  }

  Ok(out)
}

fn derive_path(mut key: XPrv, path: &DerivationPath) -> Result<XPrv, Box<dyn Error>> {
  for child in path.iter() {
    key = key.derive_child(child)?;
  }
  Ok(key)
}

fn ltc_bech32_p2wpkh(
  pubkey_compressed: &[u8],
  network: LtcNetwork,
) -> Result<String, Box<dyn Error>> {
  let sha = Sha256::digest(pubkey_compressed);
  let rip = Ripemd160::digest(&sha);

  let mut program = Vec::with_capacity(1 + rip.len());
  program.push(0u8);
  program.extend_from_slice(&rip);

  let hrp = match network {
    LtcNetwork::Mainnet => "ltc",
    LtcNetwork::Testnet => "tltc",
  };

  Ok(bech32::encode::<Bech32>(Hrp::parse(hrp)?, &program)?)
}
