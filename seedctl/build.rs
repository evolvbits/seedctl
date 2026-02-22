use std::{fs, process::Command};

fn git_commit() -> String {
  Command::new("git")
    .args(["rev-parse", "HEAD"])
    .output()
    .ok()
    .and_then(|output| {
      if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
      } else {
        None
      }
    })
    .unwrap_or_else(|| "unknown".to_string())
}

fn git_date() -> String {
  Command::new("git")
    .args(["show", "-s", "--format=%cs", "HEAD"])
    .output()
    .ok()
    .and_then(|o| {
      o.status
        .success()
        .then(|| String::from_utf8_lossy(&o.stdout).trim().to_string())
    })
    .unwrap_or_else(|| "unknown".into())
}

fn extract_value(content: &str, key: &str) -> Option<String> {
  content
    .lines()
    .find(|line| line.trim_start().starts_with(key))
    .and_then(|line| line.split('=').nth(1))
    .map(|v| v.trim().trim_matches('"').to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let cargo_toml =
    fs::read_to_string("Cargo.toml").expect("It was not possible to read Cargo.toml");

  // Metadata fields
  let copyright_year =
    extract_value(&cargo_toml, "copyright_year").unwrap_or_else(|| "2026".into());
  println!("cargo:rustc-env=COPYRIGHT_YEAR={}", copyright_year);
  let maintainer = extract_value(&cargo_toml, "maintainer").unwrap_or_else(|| "Unknown".into());
  println!("cargo:rustc-env=PROJECT_MAINTAINER={}", maintainer);

  let commit = git_commit();
  println!("cargo:rustc-env=GIT_COMMIT={}", commit);

  let commit_date = git_date();
  println!("cargo:rustc-env=GIT_DATE={}", commit_date);

  let profile = std::env::var("PROFILE").unwrap_or_else(|_| "unknown".into());
  println!("cargo:rustc-env=BUILD_PROFILE={}", profile);

  // Define resource.rc in the Windows executable (.exe)
  #[cfg(target_os = "windows")]
  {
    use std::{
      env::var,
      io::{Write, stderr},
      process::exit,
    };
    use winres;

    if var("PROFILE")? == "release" {
      let mut res = winres::WindowsResource::new();
      res.set_resource_file("resource.rc");
      match res.compile() {
        Err(error) => {
          write!(stderr(), "{}", error).unwrap();
          exit(1);
        }
        Ok(_) => {}
      }
    }
  }

  Ok(())
}
