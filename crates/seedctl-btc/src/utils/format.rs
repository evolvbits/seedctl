//! Bitcoin output descriptor and key-origin formatting helpers.
//!
//! Provides [`format_key_origin`] for building the `[fingerprint/purpose'/coin'/0']`
//! prefix used in output descriptors, and [`output_descriptor`] for assembling
//! complete BIP-380-style descriptors for the receive and change chains.

/// Builds the key-origin prefix used inside output descriptors.
///
/// The returned string follows the BIP-380 `[fingerprint/purpose'h/coin_type'h/0h]`
/// notation, using `h` suffix (written as `h` in the BIP but commonly written
/// as `'` in most wallets — this implementation uses the single-quote style).
///
/// # Parameters
///
/// - `fingerprint`  — 4-byte master fingerprint produced by
///   [`seedctl_btc::derive::derive_account`].
/// - `purpose`      — BIP derivation purpose: `84`, `49`, or `44`.
/// - `coin_type`    — SLIP-44 coin type: `0` (Mainnet) or `1` (Testnet).
///
/// # Examples
///
/// ```
/// use seedctl_btc::utils::format::format_key_origin;
///
/// let fp = [0xa1u8, 0xb2, 0xc3, 0xd4];
/// assert_eq!(format_key_origin(fp, 84, 0), "[a1b2c3d4/84h/0h/0h]");
/// ```
pub fn format_key_origin(fingerprint: [u8; 4], purpose: u32, coin_type: u32) -> String {
  format!(
    "[{:02x}{:02x}{:02x}{:02x}/{}h/{}h/0h]",
    fingerprint[0], fingerprint[1], fingerprint[2], fingerprint[3], purpose, coin_type
  )
}

/// Builds a complete BIP-380 output descriptor for the given chain index.
///
/// Selects the appropriate descriptor template based on `purpose`:
///
/// | `purpose` | Template                              | Script type     |
/// |:---------:|:--------------------------------------|:----------------|
/// | `84`      | `wpkh(<key_origin><xpub>/<chain>/*)`  | Native SegWit   |
/// | `49`      | `sh(wpkh(<key_origin><xpub>/<chain>/*))` | Nested SegWit |
/// | `44`      | `pkh(<key_origin><xpub>/<chain>/*)`   | Legacy P2PKH    |
///
/// # Parameters
///
/// - `purpose`     — BIP purpose number determining the descriptor template.
/// - `key_origin`  — key-origin prefix string produced by [`format_key_origin`],
///   e.g. `"[a1b2c3d4/84h/0h/0h]"`.
/// - `xpub`        — account-level extended public key string (xpub / ypub / zpub).
/// - `chain`       — chain index: `0` = external (receive), `1` = internal (change).
///
/// # Panics
///
/// Panics (via `unreachable!`) if `purpose` is anything other than `44`, `49`,
/// or `84`, which should never happen given the controlled call sites.
pub fn output_descriptor(purpose: u32, key_origin: &str, xpub: &str, chain: u32) -> String {
  match purpose {
    // BIP-84: Native SegWit (P2WPKH)
    84 => format!("wpkh({}{}/{chain}/*)", key_origin, xpub),
    // BIP-49: Nested SegWit (P2SH-P2WPKH)
    49 => format!("sh(wpkh({}{}/{chain}/*))", key_origin, xpub),
    // BIP-44: Legacy (P2PKH)
    44 => format!("pkh({}{}/{chain}/*)", key_origin, xpub),
    _ => unreachable!(),
  }
}
