use std::{fs, path::Path, process::Command};

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

fn extract_section<'a>(content: &'a str, section: &str) -> Vec<&'a str> {
  let header = format!("[{}]", section);
  let mut in_section = false;
  let mut lines = Vec::new();

  for line in content.lines() {
    let trimmed = line.trim();

    if trimmed.starts_with('[') && trimmed.ends_with(']') {
      in_section = trimmed == header;
      continue;
    }

    if in_section {
      lines.push(line);
    }
  }

  lines
}

fn extract_value(lines: &[&str], key: &str) -> Option<String> {
  lines
    .iter()
    .find(|line| line.trim_start().starts_with(key))
    .and_then(|line| line.split('=').nth(1))
    .map(|v| v.trim().trim_matches('"').to_string())
}

fn extract_array_values(lines: &[&str], key: &str) -> Vec<String> {
  let mut raw = String::new();
  let mut collecting = false;

  for line in lines {
    let trimmed = line.trim();

    if !collecting {
      if !trimmed.starts_with(key) {
        continue;
      }

      if let Some(value) = trimmed.split_once('=') {
        raw.push_str(value.1.trim());
      }
      collecting = !trimmed.contains(']');
      if !collecting {
        break;
      }
      continue;
    }

    raw.push_str(trimmed);
    if trimmed.contains(']') {
      break;
    }
  }

  raw
    .trim()
    .trim_start_matches('[')
    .trim_end_matches(']')
    .split(',')
    .map(|value| value.trim().trim_matches('"'))
    .filter(|value| !value.is_empty())
    .map(ToOwned::to_owned)
    .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let workspace_manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../Cargo.toml");
  println!(
    "cargo:rerun-if-changed={}",
    workspace_manifest.to_string_lossy()
  );
  let cargo_toml = fs::read_to_string(workspace_manifest)?;
  let workspace_package = extract_section(&cargo_toml, "workspace.package");
  let seedctl_metadata = extract_section(&cargo_toml, "workspace.metadata.seedctl");

  // Metadata fields
  let documentation = extract_value(&seedctl_metadata, "documentation")
    .unwrap_or_else(|| "https://orbitbits.com/seedctl/documentation/".into());
  println!("cargo:rustc-env=PROJECT_DOCUMENTATION={}", documentation);

  let homepage = extract_value(&workspace_package, "homepage")
    .unwrap_or_else(|| "https://orbitbits.com/seedctl/".into());
  println!("cargo:rustc-env=PROJECT_HOMEPAGE={}", homepage);

  let maintainer =
    extract_value(&seedctl_metadata, "maintainer").unwrap_or_else(|| "Unknown".into());
  println!("cargo:rustc-env=PROJECT_MAINTAINER={}", maintainer);

  let copyright =
    extract_value(&seedctl_metadata, "copyright").unwrap_or_else(|| "© 2026 OrbitBits.".into());
  println!("cargo:rustc-env=PROJECT_COPYRIGHT={}", copyright);

  let authors = extract_array_values(&workspace_package, "authors").join(":");
  println!("cargo:rustc-env=PROJECT_AUTHORS={}", authors);

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
