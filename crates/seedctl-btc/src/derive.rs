use bitcoin::{
  Address, Network,
  bip32::{ChildNumber, DerivationPath, Xpriv, Xpub},
  key::Secp256k1,
  secp256k1::All,
};
use std::error::Error;

// Deriva a conta (acc_xprv, acc_xpub) a partir do master e do path m/purpose'/coin_type'/0'
pub fn derive_account(
  master: &Xpriv,
  secp: &Secp256k1<All>,
  purpose: u32,
  btc_coin_type: u32,
) -> Result<(Xpriv, Xpub, [u8; 4]), Box<dyn Error>> {
  let path: DerivationPath = format!("m/{}'/{}'/0'", purpose, btc_coin_type)
    .parse()
    .map_err(|e| format!("{:?}", e))?;

  let acc_xprv = master.derive_priv(secp, &path)?;
  let acc_xpub = Xpub::from_priv(secp, &acc_xprv);
  let fingerprint = master.fingerprint(secp);
  let fingerprint_bytes = [
    fingerprint[0],
    fingerprint[1],
    fingerprint[2],
    fingerprint[3],
  ];

  Ok((acc_xprv, acc_xpub, fingerprint_bytes))
}

/// Gera os primeiros `count` endereços de recebimento (receive chain 0/x).
pub fn receive_addresses(
  acc_xpub: &Xpub,
  secp: &Secp256k1<All>,
  btc_network: Network,
  address_type: usize,
  purpose: u32,
  btc_coin_type: u32,
  count: u32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
  let mut out = Vec::with_capacity(count as usize);

  for i in 0..count {
    let child = acc_xpub
      .derive_pub(
        secp,
        &[
          ChildNumber::Normal { index: 0 },
          ChildNumber::Normal { index: i },
        ],
      )
      .map_err(|e| format!("{:?}", e))?;

    let addr = match address_type {
      0 => Address::p2wpkh(&bitcoin::CompressedPublicKey(child.public_key), btc_network),
      1 => Address::p2shwpkh(&bitcoin::CompressedPublicKey(child.public_key), btc_network),
      2 => {
        let pk = bitcoin::PublicKey::new(child.public_key);
        Address::p2pkh(pk, btc_network)
      }
      _ => unreachable!(),
    };

    let path_str = format!("m/{}'/{}'/0'/0/{}", purpose, btc_coin_type, i);
    out.push((path_str, addr.to_string()));
  }

  Ok(out)
}
