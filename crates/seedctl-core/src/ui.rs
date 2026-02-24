use crate::traits::address::AddressDisplay;
use crate::utils::format_fingerprint_hex;
use console::style;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::error::Error;

pub fn dialoguer_theme(arrow: &str) -> ColorfulTheme {
  let mut theme = ColorfulTheme {
    active_item_prefix: style(arrow.to_string()),
    ..Default::default()
  };

  theme.active_item_prefix = style("►".to_string());
  theme
}

pub fn exit_confirm() {
  #[cfg(target_os = "windows")]
  {
    use std::io;
    println!(
      "{}\n",
      style("The program has ended. Press ENTER to exit.")
        .bold()
        .yellow()
    );
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
  }
}

// ---------- Shared prompts (BTC e ETH) ----------

/// Optional passphrase for BIP39. Returns an empty string if the user does not provide one.
pub fn prompt_passphrase() -> Result<String, Box<dyn Error>> {
  let prompt = style("[Optional] Passphrase (enter = empty)")
    .bold()
    .yellow()
    .to_string();
  let s = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt(prompt)
    .allow_empty(true)
    .interact_text()?;
  Ok(s)
}

// Confirmation "Are the selected options correct?" 0 = Yes (continue), 1 = No (exit).
pub fn prompt_confirm_options() -> Result<usize, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Are the selected options correct?")
    .items(["Yes", "No"])
    .default(0)
    .interact()?;
  Ok(choice)
}

// Export watch-only wallet? 0 = Yeses, 1 = No.
pub fn prompt_export_watch_only() -> Result<usize, Box<dyn Error>> {
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Export watch-only wallet?")
    .items(["Yes", "No"])
    .default(0)
    .interact()?;
  Ok(choice)
}

// ---------- Shared wallet UI (legend, header, mnemonic) ----------

pub fn print_wallet_legend() {
  let rows = vec![
    ("red", "SECRET", "DO NOT SHARE", "red"),
    ("yellow", "PUBLIC", "WATCH-ONLY DATA", "yellow"),
  ];

  let sym_w = rows.iter().map(|r| r.0.len()).max().unwrap_or(1);
  let type_w = rows.iter().map(|r| r.1.len()).max().unwrap_or(6);
  let desc_w = rows.iter().map(|r| r.2.len()).max().unwrap_or(10);

  println!("\n{}", style("Legend:").bold());

  println!(
    "-------------------------------------\n{:<sym_w$}      {:<type_w$}     {:<desc_w$}",
    "Color",
    "Type",
    "Description",
    sym_w = sym_w,
    type_w = type_w,
    desc_w = desc_w
  );

  println!(
    "{}--+--{}--+--{}",
    "-".repeat(sym_w),
    "-".repeat(type_w),
    "-".repeat(desc_w)
  );

  for (icon, kind, desc, color) in rows {
    let icon = match color {
      "red" => style(icon).red().bold(),
      "yellow" => style(icon).yellow().bold(),
      _ => style(icon).bold(),
    };

    println!(
      "{:<sym_w$}  |  {:<type_w$}  |  {:<desc_w$}",
      icon,
      style(kind).bold(),
      desc,
      sym_w = sym_w,
      type_w = type_w,
      desc_w = desc_w
    );
  }
}

pub mod table;

pub fn print_wallet_header(coin_name: &str) {
  print_wallet_legend();

  println!(
    "\n\n{}\n{}",
    style(format!(":: Your wallet {coin_name}:")).bold().blue(),
    style("-".repeat(83)).bold().blue()
  );
}

// A line at the end of the wallet.
pub fn print_closed_wallet() {
  println!(
    "\n{}",
    style(format!(":: {}", "-".repeat(80))).bold().blue()
  );
}

// Table of mnemonic: title + lines (position 1-based, word_index 1-based, word).
// Core does not depend on bip39; each crate assembles the lines and calls this function.
pub fn print_mnemonic_table(title: &str, rows: &[(usize, u16, &str)]) {
  println!(
    "\n{} {}",
    style("[SECRET] →").bold().red(),
    style(title).bold().cyan()
  );
  println!(
    "\n{} {}",
    style("Position |").bold(),
    style("Word Indexes | Word (seed)").bold()
  );
  println!("{}", "-".repeat(37));
  for (pos, idx, word) in rows.iter() {
    println!("{:02}.        {:04}          {}", pos, idx, style(word));
  }
}

pub fn print_address_table<T: AddressDisplay>(rows: &[T]) {
  if rows.is_empty() {
    println!("(no addresses)");
    return;
  }

  let path_width = rows.iter().map(|r| r.path().len()).max().unwrap_or(10);
  let addr_width = rows.iter().map(|r| r.addr().len()).max().unwrap_or(20);

  let extra_width = rows
    .iter()
    .filter_map(|r| r.extra())
    .map(|s| s.len())
    .max()
    .unwrap_or(0);

  if extra_width > 0 {
    println!(
      "{:<path_width$} | {:<addr_width$} | {:<extra_width$}",
      "Derivation Path",
      "Address",
      "Extra",
      path_width = path_width,
      addr_width = addr_width,
      extra_width = extra_width
    );

    println!(
      "{}-+-{}-+-{}",
      "-".repeat(path_width),
      "-".repeat(addr_width),
      "-".repeat(extra_width)
    );

    for row in rows {
      println!(
        "{:<path_width$} | {:<addr_width$} | {:<extra_width$}",
        row.path(),
        row.addr(),
        row.extra().unwrap_or_default(),
        path_width = path_width,
        addr_width = addr_width,
        extra_width = extra_width
      );
    }
  } else {
    println!(
      "{:<path_width$} | {:<addr_width$}",
      "Derivation Path",
      "Address",
      path_width = path_width,
      addr_width = addr_width
    );

    println!("{}-+-{}", "-".repeat(path_width), "-".repeat(addr_width));

    for row in rows {
      println!(
        "{:<path_width$} | {:<addr_width$}",
        row.path(),
        row.addr(),
        path_width = path_width,
        addr_width = addr_width
      );
    }
  }
}

pub struct WalletHeader<'a> {
  pub derivation_path: Option<String>,
  pub fingerprint: Option<String>,
  pub account_xprv: Option<&'a str>,
  pub account_xpub: Option<&'a str>,
  pub addr_count: Option<i64>,
  pub descriptors: Vec<(&'a str, &'a str)>,
  pub title: &'a str,
}

pub enum AddressRows<'a> {
  Basic(&'a [(String, String)]),
  WithBalance(&'a [(String, String, Option<f64>)]),
}

/// Função genérica para imprimir carteiras padrão (m/purpose'/coin_type'/0').
pub fn print_standard_wallet<'a>(
  title: &'a str,
  purpose: u32,
  coin_type: u32,
  fingerprint: Option<&'a [u8; 4]>,
  account_xprv: Option<&'a str>,
  account_xpub: &'a str,
  addr_rows: AddressRows<'a>,
  descriptors: Vec<(&'a str, &'a str)>,
) {
  let addr_len = match addr_rows {
    AddressRows::Basic(addrs) => addrs.len(),
    AddressRows::WithBalance(addrs) => addrs.len(),
  };

  let header = WalletHeader {
    derivation_path: Some(format!("m/{}'/{}'/0'", purpose, coin_type)),
    fingerprint: fingerprint.map(|fp| format_fingerprint_hex(fp)),
    account_xprv,
    account_xpub: Some(account_xpub),
    addr_count: Some(addr_len as i64),
    descriptors,
    title,
  };

  print_wallet(&header, addr_rows);
}

pub fn print_wallet(header: &WalletHeader, rows: AddressRows) {
  // ───── derivation path ─────
  if let Some(path) = &header.derivation_path {
    println!(
      "\n{} {} {}",
      style("[PUBLIC] →").yellow().bold(),
      style("Derivation path:").cyan().bold(),
      path
    );
  }

  // ───── fingerprint ─────
  if let Some(fp) = &header.fingerprint {
    println!(
      "\n{} {} {}",
      style("[PUBLIC] →").yellow().bold(),
      style("Fingerprint:").cyan().bold(),
      fp
    );
  }

  // ───── private key ─────
  if let Some(xprv) = header.account_xprv {
    println!(
      "\n{} {}\n{}",
      style("[SECRET] →").red().bold(),
      style("Account Private Key:").cyan().bold(),
      xprv
    );
  }

  // ───── public key ─────
  if let Some(xpub) = header.account_xpub {
    println!(
      "\n{} {}\n{}",
      style("[PUBLIC] →").yellow().bold(),
      style("Account Public Key:").cyan().bold(),
      xpub
    );
  }

  // ───── descriptors ─────
  for (label, desc) in &header.descriptors {
    println!(
      "\n{} {}",
      style("[PUBLIC] →").yellow().bold(),
      style(*label).cyan().bold()
    );
    println!("{}", desc);
  }

  println!(
    "\n{} {}",
    style("[PUBLIC] →").yellow().bold(),
    style("Addresses:").cyan().bold()
  );

  print_table(rows);
  print_closed_wallet();
}

fn print_table(rows: AddressRows) {
  match rows {
    AddressRows::Basic(list) => {
      let path_w = list.iter().map(|(p, _)| p.len()).max().unwrap_or(10);
      let addr_w = list.iter().map(|(_, a)| a.len()).max().unwrap_or(20);

      println!(
        "{:<path_w$} | {:<addr_w$}",
        "Derivation Path",
        "Address",
        path_w = path_w,
        addr_w = addr_w
      );

      println!("{}-+-{}", "-".repeat(path_w), "-".repeat(addr_w));

      for (p, a) in list {
        println!(
          "{:<path_w$} | {:<addr_w$}",
          p,
          a,
          path_w = path_w,
          addr_w = addr_w
        );
      }
    }

    AddressRows::WithBalance(list) => {
      let path_w = list.iter().map(|(p, _, _)| p.len()).max().unwrap_or(10);
      let addr_w = list.iter().map(|(_, a, _)| a.len()).max().unwrap_or(20);

      println!(
        "{:<path_w$} | {:<addr_w$} | Balance",
        "Derivation Path",
        "Address",
        path_w = path_w,
        addr_w = addr_w
      );

      println!("{}-+-{}-+--------", "-".repeat(path_w), "-".repeat(addr_w));

      for (p, a, b) in list {
        let bal = b.map(|v| v.to_string()).unwrap_or("-".into());

        println!(
          "{:<path_w$} | {:<addr_w$} | {}",
          p,
          a,
          bal,
          path_w = path_w,
          addr_w = addr_w
        );
      }
    }
  }
}
