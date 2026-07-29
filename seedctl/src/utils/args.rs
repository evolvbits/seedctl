//! CLI argument printing helpers for the `seedctl` binary.
//!
//! Provides [`print_version`] and [`print_about`] used when the user passes
//! `--version` / `-V` or `--about` / `--help` respectively.

use super::{meta, slogan::slogan_view};
use console::style;

fn authors_lines(authors: &str) -> String {
  authors
    .replace("://", "\0PROTO\0")
    .split(':')
    .map(|author| author.trim().replace("\0PROTO\0", "://"))
    .filter(|author| !author.is_empty())
    .map(|author| format!("  - {}", author))
    .collect::<Vec<_>>()
    .join("\n")
}

fn print_credits() {
  println!();
  println!(
    "{}",
    style(format!("Credits: {}", "-".repeat(65))).cyan().bold()
  );
  println!("- {}:", style("Authors").bold());
  println!("{}", authors_lines(meta::PROJECT_AUTHORS));
  println!(
    "- {}: {}",
    style("Maintainer").bold(),
    meta::PROJECT_MAINTAINER
  );
  println!(
    "- {}: {}",
    style("Copyright").bold(),
    meta::PROJECT_COPYRIGHT
  );
}

/// Prints the short version string to stdout.
///
/// Format: `<name> <version> (<commit> <date>)`
///
/// Example output:
/// ```text
/// seedctl 0.2.1 (abc1234 2025-01-01)
/// ```
pub fn print_version() {
  println!(
    "{} {} ({} {})",
    meta::PROJECT_NAME,
    meta::VERSION,
    meta::GIT_COMMIT,
    meta::GIT_DATE
  );
}

/// Prints the full about screen to stdout.
///
/// Renders the ASCII art slogan, followed by a structured list of build
/// metadata (version, commit, build profile, date, homepage, repository,
/// documentation link), and final credits.
pub fn print_about() {
  slogan_view(false, false);
  println!();
  println!(
    "{}",
    style(format!("About: {}", "-".repeat(67))).cyan().bold()
  );
  println!("{}{}", style("- Version: ").bold().yellow(), meta::VERSION);
  println!(
    "{}{}",
    style("- Commit: ").bold().yellow(),
    meta::GIT_COMMIT
  );
  println!(
    "{}{}",
    style("- Build: ").bold().yellow(),
    meta::BUILD_PROFILE
  );
  println!(
    "{}{}",
    style("- Build date: ").bold().yellow(),
    meta::GIT_DATE
  );
  println!(
    "{}{}",
    style("- Homepage: ").bold().yellow(),
    meta::PROJECT_HOMEPAGE
  );
  println!(
    "{}{}",
    style("- Repository: ").bold().yellow(),
    meta::PROJECT_REPOSITORY
  );
  println!(
    "{}{}",
    style("- Documentation: ").bold().yellow(),
    meta::PROJECT_DOCUMENTATION
  );
  print_credits();
}
