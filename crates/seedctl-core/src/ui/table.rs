use crate::traits::address::AddressDisplay;

/// Simple, reusable table printer for address rows.
pub fn print_table<T: AddressDisplay>(rows: &[T]) {
  if rows.is_empty() {
    println!("(no addresses)");
    return;
  }

  let path_w = rows.iter().map(|r| r.path().len()).max().unwrap_or(10);
  let addr_w = rows.iter().map(|r| r.addr().len()).max().unwrap_or(20);

  println!(
    "\n{:<path_w$} | {:<addr_w$}",
    "Derivation Path",
    "Address",
    path_w = path_w,
    addr_w = addr_w,
  );

  println!("{}-+-{}", "-".repeat(path_w), "-".repeat(addr_w));

  for r in rows {
    println!(
      "{:<path_w$} | {:<addr_w$}",
      r.path(),
      r.addr(),
      path_w = path_w,
      addr_w = addr_w,
    );
  }
}
