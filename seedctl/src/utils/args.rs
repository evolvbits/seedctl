//! CLI argument printing helpers for the `seedctl` binary.
//!
//! Provides [`print_version`] and [`print_about`] used when the user passes
//! `--version` / `-V` or `--about` / `--help` respectively.

use super::{meta, slogan::slogan_view};
use console::style;

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
/// metadata (version, commit, build profile, date, maintainer, repository,
/// documentation link), and closes with the copyright footer.
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
    style("- Maintainer: ").bold().yellow(),
    meta::PROJECT_MAINTAINER
  );
  println!(
    "{}{}",
    style("- Repository: ").bold().yellow(),
    meta::PROJECT_REPOSITORY
  );
  println!(
    "{}{}/README.md",
    style("- Documentation: ").bold().yellow(),
    meta::PROJECT_REPOSITORY
  );

  super::copyright_phrase();
}
