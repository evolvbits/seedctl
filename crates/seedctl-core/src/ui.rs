//! Shared UI utilities used across all `seedctl-*` crates.
//!
//! Provides themed dialoguer prompts, wallet header/footer printing,
//! mnemonic and address table rendering, and watch-only export helpers.

use crate::traits::address::AddressDisplay;
use crate::utils::format_fingerprint_hex;
use console::style;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::error::Error;

/// Builds the default [`ColorfulTheme`] with the custom active-item arrow.
///
/// Always bind the result to a `let` before passing as `&dyn Theme` so that
/// the temporary lives long enough for the dialoguer builder chain.
pub fn dialoguer_theme(_arrow: &str) -> ColorfulTheme {
  ColorfulTheme {
    active_item_prefix: style("►".to_string()),
    ..Default::default()
  }
}

/// Waits for the user to press Enter on Windows before the process exits.
///
/// No-op on non-Windows platforms.
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

// ---------- Shared prompts (BTC, ETH, …) ----------

/// Prompts the user for an optional BIP-39 passphrase.
///
/// Returns an empty string if the user presses Enter without typing anything.
pub fn prompt_passphrase() -> Result<String, Box<dyn Error>> {
  let prompt = style("[Optional] Passphrase (enter = empty)")
    .bold()
    .yellow()
    .to_string();
  let theme = dialoguer_theme("►");
  let s = Input::with_theme(&theme)
    .with_prompt(prompt)
    .allow_empty(true)
    .interact_text()?;
  Ok(s)
}

/// Asks the user to confirm that the selected options are correct.
///
/// Returns `0` = Yes (continue), `1` = No (exit).
pub fn prompt_confirm_options() -> Result<usize, Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let choice = Select::with_theme(&theme)
    .with_prompt("Are the selected options correct?")
    .items(["Yes", "No"])
    .default(0)
    .interact()?;
  Ok(choice)
}

/// Asks the user whether to export a watch-only wallet JSON file.
///
/// Returns `0` = Yes, `1` = No.
pub fn prompt_export_watch_only() -> Result<usize, Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let choice = Select::with_theme(&theme)
    .with_prompt("Export watch-only wallet?")
    .items(["Yes", "No"])
    .default(0)
    .interact()?;
  Ok(choice)
}

// ---------- Shared wallet UI (legend, header, mnemonic, addresses) ----------

/// Prints a two-row colour legend explaining SECRET vs PUBLIC data.
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

/// Sub-module with a generic table printer based on the [`AddressDisplay`] trait.
pub mod table;

/// Prints the wallet legend followed by the coin-specific section header.
pub fn print_wallet_header(coin_name: &str) {
  print_wallet_legend();

  println!(
    "\n\n{}\n{}",
    style(format!(":: Your wallet {coin_name}:")).bold().blue(),
    style("-".repeat(83)).bold().blue()
  );
}

/// Prints the closing separator line that marks the end of a wallet section.
pub fn print_closed_wallet() {
  println!(
    "\n{}",
    style(format!(":: {}", "-".repeat(80))).bold().blue()
  );
}

/// Prints a BIP-39 mnemonic as a numbered two-column table (position, index, word).
///
/// `rows` is a slice of `(1-based position, 1-based word index, word)` tuples.
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

/// Prints an address table for any type that implements [`AddressDisplay`].
///
/// Renders an optional third "Extra" column (e.g. balance) when at least
/// one row provides a non-`None` value.
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

/// Header data passed to [`print_wallet`] for structured wallet display.
pub struct WalletHeader<'a> {
  /// Optional BIP-32 derivation path, e.g. `"m/84'/0'/0'"`.
  pub derivation_path: Option<String>,
  /// Optional master fingerprint in lowercase hex.
  pub fingerprint: Option<String>,
  /// Optional account-level extended private key string.
  pub account_xprv: Option<&'a str>,
  /// Optional account-level extended public key string.
  pub account_xpub: Option<&'a str>,
  /// Optional number of addresses to display.
  pub addr_count: Option<i64>,
  /// Output descriptors as `(label, descriptor)` pairs.
  pub descriptors: Vec<(&'a str, &'a str)>,
  /// Human-readable wallet title shown in the header.
  pub title: &'a str,
}

/// Address rows passed to [`print_wallet`].
///
/// - [`AddressRows::Basic`]: plain `(path, address)` pairs.
/// - [`AddressRows::WithBalance`]: `(path, address, optional_balance)` triples.
pub enum AddressRows<'a> {
  /// Simple path + address pairs with no balance information.
  Basic(&'a [(String, String)]),
  /// Path + address pairs that may include an on-chain balance.
  WithBalance(&'a [(String, String, Option<f64>)]),
}

/// Prints a complete standard wallet section (path → fingerprint → keys →
/// descriptors → addresses).
///
/// This is a convenience wrapper around [`print_wallet`] for the common
/// `m/purpose'/coin_type'/0'` layout.
#[allow(clippy::too_many_arguments)]
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
    fingerprint: fingerprint.map(format_fingerprint_hex),
    account_xprv,
    account_xpub: Some(account_xpub),
    addr_count: Some(addr_len as i64),
    descriptors,
    title,
  };

  print_wallet(&header, addr_rows);
}

/// Renders a full wallet section to stdout using the provided header metadata
/// and address rows.
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

/// Renders the address table for [`AddressRows::Basic`] or
/// [`AddressRows::WithBalance`] variants.
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
        let bal = b.map(|v| v.to_string()).unwrap_or_else(|| "-".into());

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
