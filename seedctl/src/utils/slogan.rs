//! ASCII art banner and version slogan for the `seedctl` binary.
//!
//! [`slogan_view`] renders the project logo (orange ASCII art) side-by-side
//! with the stylised project name and an optional version line, followed by
//! the project description and an optional documentation link.

use super::meta;
use crossterm::style::{self, Stylize};

/// Renders the full `seedctl` welcome banner to stdout.
///
/// The banner consists of two parts printed side-by-side on each row:
///
/// 1. **Logo** — an ASCII art icon rendered in orange (`RGB(255, 165, 0)`).
/// 2. **Name** — the stylised `seedctl` wordmark in cyan, with an optional
///    version line appended below it.
///
/// After the two-column section, the crate description is printed, and
/// optionally a documentation link.
///
/// # Parameters
///
/// - `show_doc`     — when `true`, prints a `Documentation: <url>` line after
///   the description.
/// - `show_version` — when `true`, appends the current version string below
///   the wordmark.
///
/// # Examples
///
/// ```rust,ignore
/// // Welcome screen: show version, hide documentation link.
/// slogan_view(false, true);
///
/// // About screen: hide version, hide documentation link
/// // (the about function adds the link separately).
/// slogan_view(false, false);
/// ```
pub fn slogan_view(show_doc: bool, show_version: bool) {
  let mut format_version = format!("{} version: {}", " ".repeat(21), meta::VERSION)
    .bold()
    .white()
    .to_string();

  if !show_version {
    format_version = String::new();
  }

  // Left column: ASCII art logo lines.
  let logo_lines = vec![
    r#""#,
    r#"   ****       ****"#,
    r#"     ****  *****"#,
    r#"         *** "#,
    r#"          * "#,
    r#"        ** ** "#,
    r#"      ***   *** "#,
    r#"      *       *"#,
    r#"      *   *   *"#,
    r#"        ***** "#,
    r#"          * "#,
  ];

  // Right column: stylised project name and optional version string.
  let name_lines = vec![
    r#"                     "#,
    r#"                     "#,
    r#"                     "#,
    r#"                     "#,
    r#"                         _      _   _ "#,
    r#"     ___  ___  ___  __| | ___| |_| |"#,
    r#"  / __|/ _ \/ _ \/ _` |/ __| __| |"#,
    r#"   \__ \  __/  __/ (_| | (__| |_| |"#,
    r#"   |___/\___|\___|\__,_|\___|\__|_|"#,
    &format_version,
  ];

  // Iterate over both columns in parallel, padding whichever is shorter.
  let max_height = logo_lines.len().max(name_lines.len());

  for i in 0..max_height {
    let logo_part = logo_lines.get(i).unwrap_or(&"");
    let name_part = name_lines.get(i).unwrap_or(&"");

    // Logo column in orange; name column in cyan.
    print!(
      "{}",
      logo_part
        .with(style::Color::Rgb {
          r: 255,
          g: 165,
          b: 0
        })
        .bold()
    );
    println!("{}", name_part.cyan().bold());
  }

  // Project description line.
  println!("\n    {}", meta::PROJECT_DESCRIPTION);

  // Optional documentation link.
  if show_doc {
    println!(
      "    {}{}\n",
      "Documentation: ".bold().yellow(),
      format!("{}/README.md", meta::PROJECT_REPOSITORY).cyan()
    );
  } else {
    println!("\n");
  }
}
