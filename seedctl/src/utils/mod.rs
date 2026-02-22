pub mod args;
pub mod meta;
pub mod security;
pub mod slogan;

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
