//! Entropy option flow: mnemonic size, dice mode (auto/manual) selection.
//!
//! Called by the `seedctl` binary before dispatching to a chain crate.

use console::style;
use dialoguer::Select;
use std::error::Error;

use crate::{
  ui::dialoguer_theme,
  utils::{generate_random_dice, read_manual_dice_with_feedback, required_dice},
};

/// Prompts the user to choose between 12-word (128-bit) and 24-word (256-bit)
/// mnemonic sizes.
///
/// Returns the number of entropy bits required (`128` or `256`).
fn mnemonic_bits() -> Result<i32, Box<dyn Error>> {
  let theme = dialoguer_theme("►");
  let mnemonic_choice = Select::with_theme(&theme)
    .with_prompt("Mnemonic size (seed):")
    .items(["12 words (128 bits)", "24 words (256 bits)"])
    .default(0)
    .interact()?;

  let bits = match mnemonic_choice {
    0 => 128,
    1 => 256,
    _ => unreachable!(),
  };
  Ok(bits)
}

/// Guides the user through entropy configuration and returns
/// `(bits, dice_sequence, dice_mode)`.
///
/// - `bits`: `128` or `256` — entropy size matching the chosen mnemonic length.
/// - `dice_sequence`: raw dice values `[1, 6]` collected from user or RNG.
/// - `dice_mode`: `0` = Auto/Hybrid (dice + system RNG), `1` = Manual/Deterministic.
pub fn entropy_type() -> Result<(i32, Vec<u8>, usize), Box<dyn Error>> {
  let bits = mnemonic_bits()?;

  let theme = dialoguer_theme("►");
  let choice = Select::with_theme(&theme)
    .with_prompt("Entropy type:")
    .items(["Dice [1-6]"])
    .default(0)
    .interact()?;

  let (bits, dice, dice_mode) = match choice {
    0 => {
      let min_dice = required_dice(bits as usize);

      let theme = dialoguer_theme("►");
      let dice_mode = Select::with_theme(&theme)
        .with_prompt("DICE mode:")
        .items([
          "Auto (HYBRID (dice + system RNG))",
          "Manual (inform sequence)",
        ])
        .default(0)
        .interact()?;

      let dice: Vec<u8> = match dice_mode {
        0 => generate_random_dice(min_dice),
        1 => {
          let dice = read_manual_dice_with_feedback(bits as usize);

          if dice.len() < min_dice {
            return Err(
              format!(
                "Insufficient data: {} provided, minimum {}",
                dice.len(),
                min_dice
              )
              .into(),
            );
          }

          dice
        }
        _ => unreachable!(),
      };

      let dice_str: String = dice.iter().map(|d| char::from(b'0' + *d)).collect();

      println!(
        "{} {} {} {}",
        style("✔").green().bold(),
        style("[SECRET] →").red(),
        style("Dice used:").bold(),
        style(dice_str).green(),
      );

      (bits, dice, dice_mode)
    }
    _ => unreachable!(),
  };

  Ok((bits, dice, dice_mode))
}
