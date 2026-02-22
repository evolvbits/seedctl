use dialoguer::Input;
use ed25519_hd_key::derive_from_path;
use seedctl_core::ui::dialoguer_theme;
use std::error::Error;

/// Deriva chave privada Ed25519 (32 bytes) para o índice da conta.
/// Usa BIP44 path m/44'/501'/index'/0' (SLIP-0010), compatível com Phantom e Solana CLI.
/// `seed` deve ser a seed BIP39 de 64 bytes (mnemonic.to_seed(passphrase)).
pub fn derive_seed(seed: &[u8], index: u32) -> Result<[u8; 32], Box<dyn Error>> {
  let path = format!("m/44'/501'/{}'/0'", index);
  let (private_key, _chain_code) = derive_from_path(&path, seed);
  Ok(private_key)
}

/// Endereço Solana = base58 da chave pública (32 bytes).
pub fn pubkey_to_address(pubkey: &[u8; 32]) -> String {
  bs58::encode(pubkey).into_string()
}

pub fn prompt_address_count() -> Result<u32, Box<dyn Error>> {
  let n: u32 = Input::with_theme(&dialoguer_theme("►"))
    .with_prompt("How many addresses to generate?")
    .default(10)
    .interact_text()?;
  Ok(n)
}

#[allow(dead_code)]
pub fn prompt_show_privkeys() -> Result<bool, Box<dyn Error>> {
  use dialoguer::Select;
  let choice = Select::with_theme(&dialoguer_theme("►"))
    .with_prompt("Show private keys?")
    .items(["No (recommended)", "Yes (dangerous)"])
    .default(0)
    .interact()?;
  Ok(choice == 1)
}
