use crate::constants::BITS_PER_DIE;
use bip32::XPrv;
use bip39::Mnemonic;
use console::style;
use crossterm::{
  cursor::{Hide, Show},
  event::{Event, KeyCode, read},
  execute,
  terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use rand::RngExt;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::io::{self, Write};

// SHA-256 genérico sobre uma ou mais fatias de bytes.
// Usado por dice_hash e por entropy::combine_entropy.
pub fn sha256_hash(slices: &[&[u8]]) -> Vec<u8> {
  let mut hasher = Sha256::new();
  for slice in slices {
    hasher.update(slice);
  }
  hasher.finalize().to_vec()
}

pub fn master_from_mnemonic(mnemonic: &Mnemonic, passphrase: &str) -> Result<XPrv, Box<dyn Error>> {
  let seed = mnemonic.to_seed(passphrase);
  Ok(XPrv::new(seed)?)
}

pub fn generate_random_dice(count: usize) -> Vec<u8> {
  let mut rng = rand::rng();
  (0..count).map(|_| rng.random_range(1..=6)).collect()
}

pub fn read_manual_dice_with_feedback(bits_target: usize) -> Vec<u8> {
  enable_raw_mode().unwrap();
  execute!(io::stdout(), Hide).unwrap();

  let mut dice: Vec<u8> = Vec::new();

  println!("{}", style("[ Enter dice sequence (1–6) ]").yellow().bold());

  loop {
    if let Event::Key(event) = read().unwrap() {
      match event.code {
        KeyCode::Char(c) if ('1'..='6').contains(&c) => {
          dice.push(c.to_digit(10).unwrap() as u8);
        }
        KeyCode::Backspace => {
          dice.pop();
        }
        KeyCode::Enter => {
          break;
        }
        _ => {}
      }

      let dice_count = dice.len();
      let bits = (dice_count as f64) * BITS_PER_DIE;
      let ready = bits >= bits_target as f64;

      let status = if ready {
        style("✔ enough").bold().green().to_string()
      } else {
        style("… not enough").bold().to_string()
      };

      let dice_str: String = dice.iter().map(|d| char::from(b'0' + *d)).collect();

      // Rewrite ONLY the current line
      print!("\r");
      execute!(io::stdout(), Clear(ClearType::CurrentLine)).unwrap();

      print!(
        "> Dice: {:3} | Bits: {:7.2} / {:3} | {} | [{}]",
        dice_count, bits, bits_target, status, dice_str
      );

      io::stdout().flush().unwrap();
    }
  }

  execute!(io::stdout(), Show).unwrap();
  disable_raw_mode().unwrap();
  println!();

  dice
}

/// Helper genérico para imprimir o mnemonic com índices.
/// Usado por todas as crates para evitar duplicação.
pub fn print_mnemonic(mnemonic: &Mnemonic, title: &str) {
  let rows: Vec<(usize, u16, &str)> = mnemonic
    .words()
    .zip(mnemonic.word_indices())
    .enumerate()
    .map(|(i, (word, idx))| (i + 1, (idx + 1) as u16, word))
    .collect();
  crate::ui::print_mnemonic_table(title, &rows);
}

/// Helper genérico para criar XPrv (bip32) a partir de mnemonic e passphrase.
/// Usado por ETH, TRX, MATIC para evitar duplicação.
pub fn master_from_mnemonic_bip32(
  mnemonic: &Mnemonic,
  passphrase: &str,
) -> Result<XPrv, Box<dyn Error>> {
  let seed = mnemonic.to_seed(passphrase);
  Ok(XPrv::new(seed)?)
}

pub fn dice_hash(dice: &[u8]) -> Vec<u8> {
  sha256_hash(&[dice])
}

pub fn required_dice(bits: usize) -> usize {
  ((bits as f64) / BITS_PER_DIE).ceil() as usize
}

/// Formata fingerprint (4 bytes) em hex minúsculo, ex.: "a1b2c3d4".
/// Usado em nomes de arquivo de export e em key_origin.
pub fn format_fingerprint_hex(fingerprint: &[u8; 4]) -> String {
  format!(
    "{:02x}{:02x}{:02x}{:02x}",
    fingerprint[0], fingerprint[1], fingerprint[2], fingerprint[3],
  )
}
