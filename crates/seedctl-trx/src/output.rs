use console::style;
use std::error::Error;

pub fn print_account_and_addresses(
  account_xprv_hex: &str,
  account_xpub_hex: &str,
  show_privkeys: bool,
  addr_count: u32,
  addresses: &[(String, String)],
) -> Result<(), Box<dyn Error>> {
  if show_privkeys {
    println!(
      "\n{} {}\n{}",
      style("[SECRET]").red().bold(),
      style("→ Account XPRV (hex):").bold().cyan(),
      account_xprv_hex
    );
  }

  println!(
    "\n{} {}\n{}",
    style("[PUBLIC]").yellow().bold(),
    style("→ Account XPUB (hex):").bold().cyan(),
    account_xpub_hex
  );

  println!(
    "\n{} {}",
    style("[PUBLIC]").yellow().bold(),
    style(format!("→ First {} addresses (TRX / TRC20):", addr_count))
      .bold()
      .cyan()
  );

  for (path, addr) in addresses {
    println!("{} → {}", path, addr);
  }

  Ok(())
}
