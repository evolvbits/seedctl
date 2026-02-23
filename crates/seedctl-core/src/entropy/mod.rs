use console::style;
use rand::RngExt;

use crate::error::SeedCtlError;
use crate::utils::sha256_hash;

/// Generic entropy provider abstraction.
pub trait EntropySource {
  fn generate(&self, bytes: usize) -> Result<Vec<u8>, SeedCtlError>;
}

/// System RNG‑based entropy source.
pub struct SystemEntropy;

impl EntropySource for SystemEntropy {
  fn generate(&self, bytes: usize) -> Result<Vec<u8>, SeedCtlError> {
    let mut rng = rand::rng();
    Ok((0..bytes).map(|_| rng.random::<u8>()).collect())
  }
}

fn combine_entropy(a: &[u8], b: &[u8]) -> Vec<u8> {
  sha256_hash(&[a, b])
}

fn truncate_entropy(entropy: &[u8], bits: usize) -> Vec<u8> {
  entropy[..bits / 8].to_vec()
}

// Resolve entropia final: 0 = híbrido (dice + system), 1 = determinístico (só dice).
// Retorna os bytes de entropia já truncados conforme `entropy_type.0` (bits).
pub fn resolve_final_entropy(
  entropy_type: (i32, Vec<u8>, usize),
  dice_entropy: Vec<u8>,
) -> Vec<u8> {
  let dice_mode = entropy_type.2;
  let bits = entropy_type.0 as usize;

  match dice_mode {
    0 => {
      let system_entropy = SystemEntropy.generate(32).expect("system entropy failed");
      let combined = combine_entropy(&dice_entropy, &system_entropy);
      truncate_entropy(&combined, bits)
    }
    1 => truncate_entropy(&dice_entropy, bits),
    _ => unreachable!(),
  }
}

/// Imprime a mensagem de modo de entropia (híbrido ou determinístico).
pub fn print_entropy_mode(dice_mode: usize) {
  match dice_mode {
    0 => {
      println!(
        "{} {} {}",
        style("✔").green().bold(),
        style("Entropy mode:").bold(),
        style("HYBRID (dice + system)").bold().green(),
      );
    }
    1 => {
      println!(
        "{} {} {}",
        style("✔").green().bold(),
        style("Entropy mode: ").bold(),
        style("DETERMINISTIC (dice only)").bold().green(),
      );
    }
    _ => unreachable!(),
  }
}
