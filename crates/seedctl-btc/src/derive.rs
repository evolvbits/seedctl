//! Bitcoin address derivation from BIP-32 extended keys.
//!
//! Provides [`derive_account`] for deriving the account-level key pair from a
//! BIP-32 master key, and [`receive_addresses`] for generating a batch of
//! receive addresses using BIP-84 (Native SegWit), BIP-49 (Nested SegWit),
//! or BIP-44 (Legacy P2PKH) depending on the chosen derivation purpose.

use bitcoin::{
  Address, Network,
  bip32::{ChildNumber, DerivationPath, Xpriv, Xpub},
  key::Secp256k1,
  secp256k1::All,
};
use std::error::Error;

/// Derives the account-level key pair at `m/purpose'/coin_type'/0'`.
///
/// Returns a tuple of:
/// - `Xpriv`    — account-level extended private key.
/// - `Xpub`     — corresponding account-level extended public key.
/// - `[u8; 4]`  — master fingerprint (first 4 bytes of the hash of the master
///   public key), used in output descriptors and watch-only exports.
///
/// # Parameters
///
/// - `master`        — BIP-32 master extended private key derived from the
///   BIP-39 seed.
/// - `secp`          — shared secp256k1 context (can be reused across calls).
/// - `purpose`       — BIP purpose number: `84` (SegWit), `49` (P2SH-SegWit),
///   or `44` (Legacy).
/// - `btc_coin_type` — SLIP-44 coin type: `0` for Mainnet, `1` for Testnet.
///
/// # Errors
///
/// Returns a boxed error if the derivation path cannot be parsed or if the
/// `bip32` derivation step fails (e.g. an unhardened key at a hardened step).
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

/// Generates the first `count` receive addresses from an account-level public key.
///
/// Derives child keys along the external (receive) chain `0/x` and encodes
/// each public key as a Bitcoin address using the script type indicated by
/// `address_type`:
///
/// | `address_type` | Script type  | Address format  | BIP   |
/// |:-:|:--|:--|:-:|
/// | `0` | P2WPKH       | Native SegWit (`bc1q…`) | 84 |
/// | `1` | P2SH-P2WPKH  | Nested SegWit (`3…`)    | 49 |
/// | `2` | P2PKH        | Legacy (`1…`)           | 44 |
///
/// # Parameters
///
/// - `acc_xpub`     — account-level extended public key.
/// - `secp`         — shared secp256k1 context.
/// - `btc_network`  — `Network::Bitcoin` (Mainnet) or `Network::Testnet`.
/// - `address_type` — script type index (0 = SegWit, 1 = Nested, 2 = Legacy).
/// - `purpose`      — BIP purpose used to build the derivation path label.
/// - `btc_coin_type`— SLIP-44 coin type used in the derivation path label.
/// - `count`        — number of addresses to generate.
///
/// # Returns
///
/// A `Vec` of `(derivation_path_string, encoded_address)` pairs, one entry per
/// address index `0..count`.
///
/// # Errors
///
/// Returns a boxed error if child key derivation fails for any index.
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
    // Derive the child public key at external chain 0/i.
    let child = acc_xpub
      .derive_pub(
        secp,
        &[
          ChildNumber::Normal { index: 0 },
          ChildNumber::Normal { index: i },
        ],
      )
      .map_err(|e| format!("{:?}", e))?;

    // Encode the public key as the appropriate address type.
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
