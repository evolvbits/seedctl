//! Shared utility functions used across all `seedctl-*` crates.
//!
//! Provides hashing helpers, BIP-32 master key derivation, dice input,
//! mnemonic display and fingerprint formatting.

use crate::constants::BITS_PER_DIE;
use bip32::XPrv;
use bip39::Mnemonic;
use console::style;
use crossterm::{
  cursor::{Hide, Show},
  event::{self, Event, KeyCode, read},
  execute,
  terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use rand::RngExt;
use sha2::{Digest, Sha256};
use std::{
  error::Error,
  io::{self, Write},
  time::Duration,
};

/// Computes SHA-256 over one or more byte slices concatenated in order.
///
/// Used by [`dice_hash`] and [`crate::entropy`] to combine entropy sources.
pub fn sha256_hash(slices: &[&[u8]]) -> Vec<u8> {
  let mut hasher = Sha256::new();
  for slice in slices {
    hasher.update(slice);
  }
  hasher.finalize().to_vec()
}

/// Derives a BIP-32 master extended private key from a BIP-39 mnemonic and passphrase.
pub fn master_from_mnemonic(mnemonic: &Mnemonic, passphrase: &str) -> Result<XPrv, Box<dyn Error>> {
  let seed = mnemonic.to_seed(passphrase);
  Ok(XPrv::new(seed)?)
}

/// Generates `count` random dice rolls in the range `[1, 6]` using the system RNG.
pub fn generate_random_dice(count: usize) -> Vec<u8> {
  let mut rng = rand::rng();
  (0..count).map(|_| rng.random_range(1..=6)).collect()
}

/// Reads a manual dice sequence from the terminal with real-time feedback.
///
/// Blocks until the user has entered at least enough dice rolls to reach
/// `bits_target` bits of entropy, then presses Enter.
pub fn read_manual_dice_with_feedback(bits_target: usize) -> Vec<u8> {
  enable_raw_mode().unwrap();
  execute!(io::stdout(), Hide).unwrap();

  // Drain any pending key events left over from previous menu interactions.
  while event::poll(Duration::from_millis(0)).unwrap() {
    let _ = event::read();
  }

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
          // Require at least one die before accepting.
          if !dice.is_empty() {
            break;
          }
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

/// Prints a BIP-39 mnemonic as a numbered table with word indices.
///
/// Used by all chain crates to avoid UI duplication.
pub fn print_mnemonic(mnemonic: &Mnemonic, title: &str) {
  let rows: Vec<(usize, u16, &str)> = mnemonic
    .words()
    .zip(mnemonic.word_indices())
    .enumerate()
    .map(|(i, (word, idx))| (i + 1, (idx + 1) as u16, word))
    .collect();
  crate::ui::print_mnemonic_table(title, &rows);
}

/// Derives a BIP-32 master `XPrv` from a mnemonic and optional passphrase.
///
/// Alias used by EVM-compatible chains (ETH, TRX, MATIC).
pub fn master_from_mnemonic_bip32(
  mnemonic: &Mnemonic,
  passphrase: &str,
) -> Result<XPrv, Box<dyn Error>> {
  let seed = mnemonic.to_seed(passphrase);
  Ok(XPrv::new(seed)?)
}

/// Returns the SHA-256 hash of a dice byte sequence.
///
/// The explicit `as &[&[u8]]` cast is required to trigger array-to-slice
/// unsizing coercion before passing to [`sha256_hash`].
pub fn dice_hash(dice: &[u8]) -> Vec<u8> {
  sha256_hash(&[dice] as &[&[u8]])
}

/// Returns the minimum number of dice rolls needed to reach `bits` of entropy.
pub fn required_dice(bits: usize) -> usize {
  ((bits as f64) / BITS_PER_DIE).ceil() as usize
}

/// Formats a 4-byte fingerprint as a lowercase hex string (e.g. `"a1b2c3d4"`).
///
/// Used in export filenames and `key_origin` descriptors.
pub fn format_fingerprint_hex(fingerprint: &[u8; 4]) -> String {
  format!(
    "{:02x}{:02x}{:02x}{:02x}",
    fingerprint[0], fingerprint[1], fingerprint[2], fingerprint[3],
  )
}
