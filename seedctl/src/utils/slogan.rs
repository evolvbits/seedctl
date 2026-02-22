use super::meta;
use crossterm::style::{self, Stylize};

pub fn slogan_view(show_doc: bool, show_version: bool) {
  let mut format_version = format!("{} version: {}", " ".repeat(21), meta::VERSION)
    .bold()
    .white()
    .to_string();

  if !show_version {
    format_version = "".to_string();
  }

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

  // 2. Iterate over the lines and print them side by side in different colors.
  // We use the larger number of lines between the two to avoid index errors.
  let max_height = logo_lines.len().max(name_lines.len());

  for i in 0..max_height {
    let logo_part = logo_lines.get(i).unwrap_or(&"");
    let name_part = name_lines.get(i).unwrap_or(&"");

    // Print the logo in orange and the name in white (or another color).
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

  // Print the description
  println!("\n    {}", meta::PROJECT_DESCRIPTION);

  // If requested, print the documentation link.
  if show_doc {
    println!(
      "    {}{}\n",
      "Documentation: ".bold().yellow(),
      format!("{}/README.md", meta::PROJECT_REPOSITORY).cyan()
    );
  }
}
