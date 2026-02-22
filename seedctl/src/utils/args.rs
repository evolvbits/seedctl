use super::{meta, slogan::slogan_view};
use console::style;

pub fn print_version() {
  println!(
    "{} {} ({} {})",
    meta::PROJECT_NAME,
    meta::VERSION,
    meta::GIT_COMMIT,
    meta::GIT_DATE
  );
}

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
    style("- Build date:: ").bold().yellow(),
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
