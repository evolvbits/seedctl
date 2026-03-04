# [0.1.0] – 2026-02-06

- Added hybrid entropy (dice || hex)
- Added reproducibility documentation
- Improved security warnings

# [0.1.1] – 2026-02-07

- Adding a terminal lock in Windows
- Removing icons/emojis from documentation
- Logo creation for documentation and SeedCTL
- Enhancing visuals with the Dialoguer library

# [0.2.0] – 2026-02-17

- Changes to the logo and layout
- Improving visuals and information
- Run only in offline mode ([#1](https://github.com/evolvbits/seedctl/issues/1))
- Security card added ([#2](https://github.com/evolvbits/seedctl/issues/2))
- Feedback for a Dice Manual option ([#3](https://github.com/evolvbits/seedctl/issues/3))
- Adding subcommands: --version and --about
- Code refactoring, improving on-screen information.
- Adding an AppImage executable for Linux
- Print Fingerprint
- Option of: Native SegWit (BIP84), Nested SegWit (BIP49) and Legacy (BIP44) for Bitcoin
- Adding an option to export watch-only wallet
- Adding addresses coin: Ethereum, Solana, Tron, Litecoin, Polygon

# [0.2.1] – 2026-03-04

- Adding addresses coin: Cardano, Monero, BNB, XRP
- Improving the code structure
- Improving documentation: DISCLAIMER.md
- Adding documentation to the code
- Migrating `SeedCTL` to `@evolvbits`

# [0.3.0] – 2026-??-??

Adding Cold Signing functionality:
Use the "`alloy" crate for EVM.

The flow would be:

- The user pastes the Unsigned Transaction Hex (which they got from Rabby) into their program.

- The seedctl derives the Private Key from the Seed (which you already create).
- The alloy signs this Hex using the Private Key and the Chain ID (e.g., 1 for Ethereum, 56 for BSC).
- The seedctl outputs the Signed Hex.

Example:

```
		// Conceptual example with Alloy
		use alloy_signer_local::LocalSigner;
		use alloy_primitives::Bytes;

		// 1. Loads the private key from your Seed logic.
		let signer: LocalSigner<SigningKey> = private_key.parse()?;

		// 2. Receives the hex data from the unsigned transaction (coming from Rabby).
		let unsigned_tx_bytes = Bytes::from_str("0x...")?;

		// 3. Sign (Offline)
		let signature = signer.sign_transaction(&unsigned_tx).await?;
```

For Bitcoin, the best practice is the PSBT (BIP-174) standard. This is exactly what Electrum and Hardware Wallets use.

**Suggested library:** `rust-bitcoin`.

**Flow:** The online wallet generates a .psbt file. Your program reads this file, applies the signature with the Private Key, and returns the signed PSBT.

Suggested Architectural Design for the New Menu:

```shell
-> Manage Seeds (Generate or Import)

-> Derive Addresses (BTC, ETH/EVM, SOL)

-> Sign Offline Transaction (Cold Sign)

	-> Bitcoin (via PSBT)

	-> EVM (via Raw Hex / EIP-155)
```

