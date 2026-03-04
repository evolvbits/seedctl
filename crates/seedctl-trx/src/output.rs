//! Tron (TRX) wallet output rendering.
//!
//! Provides [`print_account_and_addresses`] used by [`crate::run`] to display
//! the complete Tron wallet section — account keys and the address table with
//! optional TRX balances — in a consistent format on the terminal.

use console::style;
use std::error::Error;

/// Prints the Tron account keys and the derived address table to stdout.
///
/// Unlike the EVM crates (ETH, BNB, MATIC), Tron uses a custom renderer
/// instead of [`seedctl_core::ui::print_standard_wallet`] because Tron
/// addresses are Base58Check-encoded rather than EIP-55 hex strings, and
/// balances are displayed in TRX with six decimal places inline.
///
/// # Parameters
///
/// - `account_xprv_hex` — hex-encoded account-level extended private key
///   (32-byte secp256k1 scalar).
/// - `account_xpub_hex` — hex-encoded account-level extended public key
///   (33-byte compressed secp256k1 point).
/// - `show_privkeys`    — when `true`, the private key section is printed;
///   when `false`, only the public key is shown.
/// - `addr_count`       — number of derived addresses; used in the section
///   header label.
/// - `addresses`        — slice of `(derivation_path, tron_address,
///   optional_balance_trx)` triples produced by [`crate::run`].
///
/// # Errors
///
/// This function never fails in practice — the `Result` wrapper exists only
/// for consistency with the rest of the chain-crate API surface. It always
/// returns `Ok(())`.
pub fn print_account_and_addresses(
  account_xprv_hex: &str,
  account_xpub_hex: &str,
  show_privkeys: bool,
  addr_count: u32,
  addresses: &[(String, String, Option<f64>)],
) -> Result<(), Box<dyn Error>> {
  // ── Private key (conditionally) ──────────────────────────────────────────
  if show_privkeys {
    println!(
      "\n{} {}\n{}",
      style("[SECRET]").red().bold(),
      style("→ Account XPRV (hex):").bold().cyan(),
      account_xprv_hex
    );
  }

  // ── Public key ───────────────────────────────────────────────────────────
  println!(
    "\n{} {}\n{}",
    style("[PUBLIC]").yellow().bold(),
    style("→ Account XPUB (hex):").bold().cyan(),
    account_xpub_hex
  );

  // ── Address table header ─────────────────────────────────────────────────
  println!(
    "\n{} {}",
    style("[PUBLIC]").yellow().bold(),
    style(format!("→ First {} addresses (TRX / TRC20):", addr_count))
      .bold()
      .cyan()
  );

  // ── Address rows ─────────────────────────────────────────────────────────
  for (path, addr, balance) in addresses {
    if let Some(bal) = balance {
      println!("{} → {}   [balance: {:.6} TRX]", path, addr, bal);
    } else {
      println!("{} → {}", path, addr);
    }
  }

  Ok(())
}
