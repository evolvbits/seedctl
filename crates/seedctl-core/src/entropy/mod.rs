//! Entropy generation and resolution for BIP-39 mnemonic seeds.
//!
//! Provides abstractions for entropy sources, combining dice-based and
//! system-random entropy, and printing the selected entropy mode.

use console::style;
use rand::RngExt;

use crate::error::SeedCtlError;
use crate::utils::sha256_hash;

/// Generic entropy provider abstraction.
pub trait EntropySource {
  /// Generates `bytes` random bytes from this source.
  fn generate(&self, bytes: usize) -> Result<Vec<u8>, SeedCtlError>;
}

/// System RNG-based entropy source backed by the OS CSPRNG.
pub struct SystemEntropy;

impl EntropySource for SystemEntropy {
  fn generate(&self, bytes: usize) -> Result<Vec<u8>, SeedCtlError> {
    let mut rng = rand::rng();
    Ok((0..bytes).map(|_| rng.random::<u8>()).collect())
  }
}

/// Combines two entropy byte slices by hashing them together with SHA-256.
///
/// The explicit `as &[&[u8]]` cast triggers the array-to-slice unsizing
/// coercion before the slice is passed to [`sha256_hash`].
fn combine_entropy(a: &[u8], b: &[u8]) -> Vec<u8> {
  sha256_hash(&[a, b] as &[&[u8]])
}

/// Truncates an entropy slice to the first `bits / 8` bytes.
fn truncate_entropy(entropy: &[u8], bits: usize) -> Vec<u8> {
  entropy[..bits / 8].to_vec()
}

/// Resolves the final entropy bytes from a dice sequence.
///
/// - `dice_mode == 0`: **Hybrid** — mixes dice entropy with 32 bytes of
///   system randomness via SHA-256, then truncates to `bits` bits.
/// - `dice_mode == 1`: **Deterministic** — uses only the dice entropy,
///   truncated to `bits` bits.
///
/// # Parameters
/// - `entropy_type`: `(bits, dice_bytes, dice_mode)` tuple returned by
///   [`crate::options::entropy_type`].
/// - `dice_entropy`: pre-hashed dice bytes from [`crate::utils::dice_hash`].
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

/// Prints the active entropy mode label to stdout.
///
/// - `0` → **HYBRID** (dice + system RNG)
/// - `1` → **DETERMINISTIC** (dice only)
pub fn print_entropy_mode(dice_mode: usize) {
  match dice_mode {
    0 => println!(
      "{} {} {}",
      style("✔").green().bold(),
      style("Entropy mode:").bold(),
      style("HYBRID (dice + system)").bold().green(),
    ),
    1 => println!(
      "{} {} {}",
      style("✔").green().bold(),
      style("Entropy mode:").bold(),
      style("DETERMINISTIC (dice only)").bold().green(),
    ),
    _ => unreachable!(),
  }
}
