#[macro_export]
macro_rules! userprofile {
  ($($part:expr),*) => {
    {
      let base_path = if cfg!(windows) {
          std::path::PathBuf::from(std::env::var("USERPROFILE").unwrap())
      } else {
          std::path::PathBuf::from(std::env::var("HOME").unwrap())
      };
      let path_parts = vec![$($part),*];
      let separator = if cfg!(windows) { r"\" } else { "/" };
      let path_str = path_parts.join(separator);
      base_path.join(path_str)
    }
  };
}
