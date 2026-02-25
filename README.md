<p align="center">
  <img src=".github/assets/seedctl.png" alt="SeedCTL" width="350"/>
</p>

[![Build and Release (Linux & Windows)](https://github.com/williamcanin/seedctl/actions/workflows/release.yml/badge.svg)](https://github.com/williamcanin/seedctl/actions/workflows/release.yml)
![Release](https://img.shields.io/github/v/release/williamcanin/seedctl?label=latest&color=blue)
![License](https://img.shields.io/github/license/williamcanin/seedctl)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![Offline](https://img.shields.io/badge/works-offline-important)
![Deterministic](https://img.shields.io/badge/deterministic-yes-success)
![No network](https://img.shields.io/badge/network-none-lightgrey)

🇧🇷 [**Leia em português**](README-prbr.md)

**SeedCTL** is a **multichain** wallet generator (Bitcoin, Ethereum, BNB Smart Chain, XRP Ledger, Tron, Solana, Cardano and Monero), **deterministic, auditable and security‑focused**, written in [**Rust**](https://rust-lang.org/) for **offline** use.

This program lets you generate (or import) a BIP39 seed and derive wallets for multiple networks from **physical data (dice) 🎲** and/or **system entropy**, producing:

- BIP39 mnemonic (12 or 24 words)
- Support for **optional passphrase**
- **BIP84** derivation (Native SegWit – bc1) for Bitcoin (`m/84'/0'/0'` and `m/84'/1'/0'`)
- **BIP44** derivation for Ethereum (`m/44'/60'/0'/0/x`) and **Ledger style** (`m/44'/60'/x'/0/0`)
- **BIP44** derivation for BNB Smart Chain (`m/44'/60'/0'/0/x`) and **Ledger style** (`m/44'/60'/x'/0/0`)
- **BIP44** derivation for XRP Ledger (`m/44'/144'/0'/0/x`)
- **BIP44** derivation for Tron (`m/44'/195'/0'/0/x`)
- **BIP44** derivation for Solana (`m/44'/501'/index'/0'`)
- **CIP-1852** derivation for Cardano (Shelley) (`m/1852'/1815'/account'/0/index`)
- Monero (subaddress) derivation with `xmr(major=0,minor=index)` and watch-only key-origin `m/44'/128'/0'/0/0`
- Support for **Mainnet and Testnet** (Bitcoin, XRP, Cardano and Monero)
- Display of [**BIP39 Word Indexes**](https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt)
- Generation of **deterministic addresses** for:
  - **Bitcoin**: `bc1...` / `tb1...` (Native SegWit)
  - **Ethereum**: `0x...` (EIP‑55 checksum)
  - **BNB Smart Chain**: `0x...` (EIP‑55 checksum)
  - **XRP Ledger**: classic `r...` addresses
  - **Tron**: `T...` (base58check with 0x41 prefix)
  - **Solana**: base58 Ed25519 public key addresses
  - **Cardano**: `addr...` / `addr_test...` (Shelley base addresses)
  - **Monero**: base58 addresses (standard/subaddress)

The main goal is to allow **secure, verifiable, offline generation** of reusable BIP39 seeds across multiple coins, with a high level of paranoia and full control over the process.

---

## Mirrors

This repository is primarily maintained on **GitHub**.

A synchronized mirror is available on **GitLab**:

- **GitHub (canonical)**: https://github.com/williamcanin/seedctl
- **GitLab (mirror)**: https://gitlab.com/williamcanin/seedctl

---

## Project Status

Maintenance and activity indicators for the canonical **GitHub** repository.

![Last commit](https://img.shields.io/github/last-commit/williamcanin/seedctl)
![Issues](https://img.shields.io/github/issues/williamcanin/seedctl)
![Stars](https://img.shields.io/github/stars/williamcanin/seedctl)
![Forks](https://img.shields.io/github/forks/williamcanin/seedctl)

---

## Security Philosophy

- No network dependency
- No data transmission
- No disk persistence
- Ideal for **offline / air‑gapped** use
- Compatible with manual verification (dice, word indexes, derivation path)
- Clear separation between **deterministic mode** and **hybrid mode**

> **WARNING**
> This program **displays sensitive information** (mnemonic, passphrase, keys).
> Use **only in a secure and offline environment**. Using it with [Tails](https://tails.net/) or similar is recommended.

---

## Features

- BIP39 – 12 or 24 words
- Entropy via physical dice (1–6)
- Hybrid entropy (dice + system RNG)
- Automatic generation or manual dice entry
- Visual confirmation of the dice sequence
- Optional passphrase (BIP39)
- Initial menu with **Generate new wallet** and **Import existing wallet** (existing seed)
- Network selection: **Bitcoin, Ethereum, BNB Smart Chain, XRP Ledger, Tron, Solana, Cardano and Monero**
- Support for Mainnet and Testnet (Bitcoin, XRP, Cardano and Monero)
- BIP84 derivation (Bitcoin), BIP44 derivation (Ethereum, BNB, XRP, Tron, Solana), CIP-1852 (Cardano), subaddress (Monero)
- Display of **Word Indexes** (base‑1, format `0001`)
- Address generation:
  - `bc1` / `tb1` (Bitcoin)
  - `0x...` (Ethereum)
  - `0x...` (BNB Smart Chain)
  - `r...` (XRP Ledger)
  - `T...` (Tron)
  - base58 (Solana)
  - `addr...` / `addr_test...` (Cardano)
  - base58 (Monero)

---

## Documentation

- **Deterministic wallet reproduction**
  See [`REPRODUCIBILITY.md`](REPRODUCIBILITY.md)

- **Binary and release verification (SHA256 + GPG)**
  See [`VERIFYING_RELEASES.md`](VERIFYING_RELEASES.md)

---

## Entropy Modes

The program offers **two distinct modes**, with different goals.

### Manual Mode (Deterministic)

Recommended for:

- Recovering an existing wallet
- Auditing
- Reproducible generation ceremonies
- Independent verification

**How it works:**

- The user manually enters the dice sequence (1–6)
- No system entropy is used
- Same sequence + same passphrase ⇒ **always the same wallet**

**Conceptual model:**

```bash
entropy = SHA256(dice_entropy)
```

This mode is **100% deterministic and reproducible**.

---

### Automatic (Hybrid) Mode

Recommended for:

- Creating new wallets
- Increasing entropy against human failures
- Defense in depth

**How it works:**

- The program automatically generates:
  - Random physical dice (1–6)
  - Secure system entropy (CSPRNG)
- Both sources are combined and hashed

**Conceptual model:**

```bash
entropy_final = SHA256(dice_entropy || hex_entropy)
```

✔ Even if one source fails, the other preserves security
✔ Not exclusively dependent on the human
✔ Not exclusively dependent on the system

**Important:**
This mode is **not reproducible** if only the dice are recorded.
For future reproducibility, the manual mode must be used.

---

## Word Indexes (BIP39)

Each mnemonic word is accompanied by its index in the BIP39 wordlist:

```bash
01. 0001 abandon
02. 1845 ability
03. 0097 able
```

## Derivation Paths per Network

- **Bitcoin**
  - Mainnet: `m/84'/0'/0'`
  - Testnet: `m/84'/1'/0'`

- **Ethereum**
  - Standard (MetaMask and others): `m/44'/60'/0'/0/x`
  - Ledger style: `m/44'/60'/x'/0/0`

- **BNB Smart Chain**
  - Standard (EVM): `m/44'/60'/0'/0/x`
  - Ledger style: `m/44'/60'/x'/0/0`

- **XRP Ledger**
  - BIP44 standard: `m/44'/144'/0'/0/x`

- **Tron**
  - BIP44 standard: `m/44'/195'/0'/0/x`

- **Solana**
  - BIP44 standard: `m/44'/501'/index'/0'`

- **Cardano**
  - CIP-1852 standard (Shelley): `m/1852'/1815'/account'/0/index`

- **Monero**
  - Address/subaddress display: `xmr(major=0,minor=index)`
  - Watch-only key origin: `m/44'/128'/0'/0/0`

---

## Addresses

Deterministic address generation from the chosen paths:

- **Bitcoin**

  ```bash
  m/84'/0'/0'/0/0 → bc1...
  ```

- **Ethereum**

  ```bash
  m/44'/60'/0'/0/0 → 0x...
  ```

- **BNB Smart Chain**

  ```bash
  m/44'/60'/0'/0/0 → 0x...
  ```

- **XRP Ledger**

  ```bash
  m/44'/144'/0'/0/0 → r...
  ```

- **Tron**

  ```bash
  m/44'/195'/0'/0/0 → T...
  ```

- **Solana**

  ```bash
  m/44'/501'/0'/0' (index 0) → <base58 address>
  ```

- **Cardano**

  ```bash
  m/1852'/1815'/0'/0/0 → addr... / addr_test...
  ```

- **Monero**

  ```bash
  xmr(major=0,minor=0) → 4... (standard) / xmr(major=0,minor=1) → 8... (subaddress)
  ```

---

## Compatibility

- **Bitcoin**
  - Sparrow Wallet
  - Electrum
  - BlueWallet
  - Bitcoin Core
  - Any BIP39/BIP84‑compatible wallet

- **Ethereum**
  - MetaMask
  - Ledger Live (standard / Ledger paths)
  - Other BIP39/BIP44 wallets with `m/44'/60'/0'/0/x`

- **BNB Smart Chain**
  - MetaMask (BSC network)
  - Trust Wallet
  - Rabby
  - Other EVM wallets with `m/44'/60'/0'/0/x`

- **XRP Ledger**
  - Xaman (XUMM)
  - Ledger Live
  - Other XRPL wallets with `m/44'/144'/0'/0/x`

- **Tron**
  - TronLink and wallets using `m/44'/195'/0'/0/x`

- **Solana**
  - Phantom
  - Solana CLI / `solana-keygen`
  - Other wallets using `m/44'/501'/index'/0'`

- **Cardano**
  - Eternl
  - Yoroi
  - Lace
  - Other Shelley/CIP-1852 wallets with `m/1852'/1815'/account'/0/index`

- **Monero**
  - Monero GUI Wallet
  - Monero CLI (`monero-wallet-cli`)
  - Feather Wallet
  - Wallets compatible with standard/subaddress format

---

## Legal Notice

This software is provided “as is,” without warranties.

You are 100% responsible for the use, storage and security of the generated keys.

> This software can irreversibly expose private keys.
> Use only if you fully understand key management.

---

## Threat Model

**This software DOES NOT PROTECT against:**

- Malware in the operating system
- Keyloggers
- Screen capture
- Compromised firmware
- Supply‑chain attacks

**This software PROTECTS against:**

- System RNG failures (via physical dice)
- Dependence on external services
- Opaque seed generation
- Lack of auditability

For maximum security, use it on a clean, temporary, offline computer.

---

## Development Requirements

- Rust 1.89

Check with:

```bash
rustc --version
```

---

## Credits

This project is built upon well‑established Bitcoin standards and the collective effort of the open‑source community.

### Author & Collaborators

- **William C. Canin** — Creator & Maintainer
- **[Your Name Here]** — Become a contributor! Submit a pull request or report an issue.

### Bitcoin Improvement Proposals (BIPs)

- **BIP32**: Hierarchical Deterministic Wallets.
- **BIP39**: Mnemonic code for deterministic key generation.
- **BIP84**: Derivation scheme for native SegWit wallets.

### Rust Ecosystem

SeedCTL is built using high‑quality open‑source libraries from the Rust community. We stand on the shoulders of giants to ensure security and performance.

You can find the full list of libraries and their versions in our [Cargo.toml](./Cargo.toml).

Special thanks to the maintainers of `bitcoin`, `bip39` and all other crates that make this project possible.

### Community Acknowledgments

Special thanks to the **Bitcoin Core** developers and the global open‑source community for prioritizing transparency and user sovereignty.

---

## Support this project

[![Donate](https://img.shields.io/badge/Donate-Bitcoin%20|%20Pix%20|%20PayPal-F5C400?style=for-the-badge)](
https://github.com/williamcanin/donations
)
[![Sponsor](https://img.shields.io/badge/Sponsor-GitHub-%23ea4aaa?style=for-the-badge)](
https://github.com/sponsors/williamcanin
)

> You will appear in the collaborators section.

---

This project was built with a strong focus on **security, transparency and verifiability**, aiming to give users complete control over their keys and derivations in Bitcoin, Ethereum, BNB Smart Chain, XRP Ledger, Tron, Solana, Cardano and Monero.
