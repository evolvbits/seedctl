pub fn format_key_origin(fingerprint: [u8; 4], purpose: u32, coin_type: u32) -> String {
  format!(
    "[{:02x}{:02x}{:02x}{:02x}/{}h/{}h/0h]",
    fingerprint[0], fingerprint[1], fingerprint[2], fingerprint[3], purpose, coin_type
  )
}

pub fn output_descriptor(purpose: u32, key_origin: &str, xpub: &str, chain: u32) -> String {
  match purpose {
    // BIP84
    84 => format!("wpkh({}{}/{chain}/*)", key_origin, xpub),
    // BIP49
    49 => format!("sh(wpkh({}{}/{chain}/*))", key_origin, xpub),
    // BIP44
    44 => format!("pkh({}{}/{chain}/*)", key_origin, xpub),
    _ => unreachable!(),
  }
}
