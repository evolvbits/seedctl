use crate::utils::copyright_phrase;
use crossterm::style::Stylize;
use if_addrs::get_if_addrs;
use seedctl_core::ui::exit_confirm;

pub struct Connection;

impl Connection {
  pub fn check() {
    let interfaces = match get_if_addrs() {
      Ok(ifaces) => ifaces,
      Err(_) => return, // If you can't read it, DO NOT block it.
    };

    for iface in interfaces {
      // ignore loopback
      if iface.is_loopback() {
        continue;
      }

      crate::utils::slogan::slogan_view(true, true);

      // If any non-loopback interface is encountered → ABORT
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
