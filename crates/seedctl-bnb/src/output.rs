pub type WalletOutput<'a> = seedctl_core::evm::WalletOutput<'a>;

pub fn print_wallet_output(output: &WalletOutput<'_>) {
  seedctl_core::evm::print_wallet_output(&seedctl_core::evm::BNB_PROFILE, output);
}
