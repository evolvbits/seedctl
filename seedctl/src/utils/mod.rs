//! Utility modules for the `seedctl` binary.
//!
//! Provides helpers for CLI argument printing, network connectivity checks,
//! build metadata constants, ASCII art slogan rendering, and shared output
//! functions used throughout the binary.

pub mod args;
pub mod connection;
pub mod meta;
pub mod slogan;

/// Prints the copyright footer to stdout.
///
/// Renders a horizontal rule, the project name, copyright year, maintainer
/// name, and a closing rule — used at the end of every interaction and in
/// the `--about` screen.
pub fn copyright_phrase() {
  let line_size = 50;
  println!("\n{}", "-".repeat(line_size));
  println!(
    "{}",
    console::style(format!(
      "{} © {} {} and collaborators.",
      meta::PROJECT_NAME,
      meta::COPYRIGHT_YEAR,
      meta::PROJECT_MAINTAINER
    ))
    .bold(),
  );
  println!("{}", "-".repeat(line_size));
}
