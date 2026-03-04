//! Network connectivity guard for air-gapped operation.
//!
//! [`Connection::check`] aborts the process if any non-loopback network
//! interface is detected, enforcing the cold-wallet air-gap requirement.
//!
//! The check is intentionally **disabled by default** in `main.rs`.
//! Re-enable it when deploying in environments that must be strictly offline.

use crate::utils::copyright_phrase;
use crossterm::style::Stylize;
use if_addrs::get_if_addrs;
use seedctl_core::ui::exit_confirm;

/// Guards against accidental use on an online machine.
///
/// Iterates over all network interfaces and aborts with a security message
/// if any non-loopback interface is found (Wi-Fi, Ethernet, VPN, etc.).
#[allow(dead_code)]
pub struct Connection;

impl Connection {
  /// Checks for active non-loopback network interfaces.
  ///
  /// If one is found, prints an air-gap warning, the copyright footer,
  /// waits for the user to confirm (Windows only), and calls
  /// [`std::process::exit`] with code `1`.
  ///
  /// If the interface list cannot be read, the check is silently skipped
  /// rather than blocking legitimate offline use.
  #[allow(dead_code)]
  pub fn check() {
    let interfaces = match get_if_addrs() {
      Ok(ifaces) => ifaces,
      // Unable to read interfaces — do not block; fail open.
      Err(_) => return,
    };

    for iface in interfaces {
      // Loopback (127.0.0.1 / ::1) is always present; skip it.
      if iface.is_loopback() {
        continue;
      }

      crate::utils::slogan::slogan_view(true, true);

      eprintln!(
        "\n{}\n{}\n",
        "[ SECURITY ABORT ]".to_string().bold().red(),
        "Active network interface detected.\n\
         This program MUST be used offline / air-gapped.\n\n\
         Disable Wi-Fi, Ethernet, VPNs and try again."
          .to_string()
          .yellow()
      );

      copyright_phrase();
      exit_confirm();
      std::process::exit(1);
    }
  }
}
