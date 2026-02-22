pub enum CliAction {
  Version,
  About,
  Run,
}

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
