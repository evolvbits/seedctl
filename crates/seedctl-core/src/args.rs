//! CLI argument parsing for the `seedctl` binary.
//!
//! Provides a lightweight, dependency-free argument parser that recognises
//! the three top-level actions without pulling in a full CLI framework.

/// The top-level action derived from the command-line arguments.
pub enum CliAction {
  /// Print the version string and exit.
  Version,
  /// Print the about/help screen and exit.
  About,
  /// Run the interactive wallet workflow.
  Run,
}

/// Parses `std::env::args` and returns the corresponding [`CliAction`].
///
/// Recognised flags:
/// - `--version` / `-V` → [`CliAction::Version`]
/// - `--about` / `--help` → [`CliAction::About`]
/// - anything else (including no arguments) → [`CliAction::Run`]
pub fn parse_args() -> CliAction {
  let args: Vec<String> = std::env::args().collect();

  if args.iter().any(|a| a == "--version" || a == "-V") {
    CliAction::Version
  } else if args.iter().any(|a| a == "--about" || a == "--help") {
    CliAction::About
  } else {
    CliAction::Run
  }
}
