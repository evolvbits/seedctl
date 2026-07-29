---
layout: doc
part: 8
section: Security & Terms
menu: seedctl
version: "0.2.4"
doc_product: seedctl
logo: https://raw.githubusercontent.com/orbitbits/seedctl/refs/heads/main/.github/logo/compact/seedctl-text.svg
title: Reproducibility
description: How to deterministically reproduce wallets with SeedCTL.
date: 2026-04-16 19:49:43 -0300
tags: [CLI, Rust, Cryptocurrency, Seed Generator, Bip39, Multichain]
published: true
permalink: /seedctl/documentation/0.2.4/reproducibility/
---

{% include assign.liquid %}

## Reproducibility & Deterministic Recovery

This section explains how to deterministically reproduce wallets generated with `SeedCTL`. A wallet is reproducible only when all relevant inputs are identical — if any input changes, outputs change.

### Supported Networks

- Bitcoin (BTC)
- Ethereum (ETH)
- BNB Smart Chain (BNB)
- XRP Ledger (XRP)
- Tron (TRX)
- Solana (SOL)
- Litecoin (LTC)
- Polygon (POL/MATIC)
- Cardano (ADA)
- Monero (XMR)

---

### Core Principle

A wallet is reproducible only when all relevant inputs are identical:

- Mnemonic source (generated in `SeedCTL` or imported BIP39 phrase)
- Mnemonic size (12 or 24 words), when generated in `SeedCTL`
- Entropy mode (Hybrid or Deterministic), when generated in `SeedCTL`
- Dice sequence (if used)
- BIP39 passphrase (exactly)
- Selected network / coin
- Selected derivation mode / style / path for that coin

If any item changes, outputs change.

---

### Entropy Model

When creating a new mnemonic in `SeedCTL`, the entropy pipeline is:

```sh
dice_entropy = SHA256(dice_sequence_bytes)
```

#### Deterministic mode (manual dice)

```sh
entropy_final = truncate_bits(dice_entropy, mnemonic_bits)
```

No system randomness is added. Reproducible if the same dice sequence and mnemonic size are used.

#### Hybrid mode (auto dice + system RNG)

```sh
entropy_final = truncate_bits(SHA256(dice_entropy || system_entropy_32B), mnemonic_bits)
```

Adds system RNG. Intended for fresh wallet generation, not deterministic ceremony replay.

> If you need strict reproducibility, use deterministic / manual dice mode or import an existing mnemonic.

---

### What You Must Record for Future Recovery

For a deterministic ceremony, record at minimum:

- Mnemonic size (12 / 24)
- Entropy mode
- Full dice sequence (if used)
- Passphrase (or explicit "empty")
- Selected coin / network
- Selected derivation mode / style / path
- Address index range generated (e.g. 0–9)

For imported wallets, record:

- Full mnemonic words
- Passphrase
- Coin / network
- Derivation mode / style / path

---

### Coin-Specific Reproducibility Parameters

#### Bitcoin (BTC)

- Networks: Mainnet and Testnet
- Coin type: Mainnet 0, Testnet 1
- Derivation purpose selectable: BIP84, BIP49, BIP44
- Account path (BIP84): `m/84'/coin_type'/0'` — native SegWit
- Account path (BIP49): `m/49'/coin_type'/0'` — nested SegWit
- Account path (BIP44): `m/44'/coin_type'/0'` — legacy
- Receive path pattern: `.../0/index`

To reproduce BTC exactly, you must keep both network and purpose identical.

#### Ethereum (ETH), BNB Smart Chain (BNB), Polygon (POL/MATIC)

These three share the same EVM derivation engine.

- Derivation style (Standard): base `m/44'/60'/0'/0`, addresses at `/index`
- Derivation style (Ledger): addresses at `m/44'/60'/index'/0/0`
- Derivation style (Custom): supports `{index}` placeholder; if path ends with `/`, index is appended

For deterministic recovery, use the same style and exact custom template (if any).

#### XRP Ledger (XRP)

- Networks: Mainnet and Testnet
- Base path: `m/44'/144'/0'/0`
- Address paths: `m/44'/144'/0'/0/index`
- Address format: XRPL classic address (`r...`)

#### Tron (TRX)

- Derivation style (Standard): `m/44'/195'/0'/0/index`
- Derivation style (Ledger): `m/44'/195'/0'/index'/0/0`
- Derivation style (Custom): custom path supported
- Address format: Base58Check with Tron prefix (`T...`)

#### Solana (SOL)

- Path: `m/44'/501'/index'/0'`
- Address format: base58 Ed25519 public key

#### Litecoin (LTC)

- Networks: Mainnet and Testnet
- Coin type: Mainnet 2, Testnet 1
- Account path: `m/84'/coin_type'/0'`
- Receive paths: `m/84'/coin_type'/0'/0/index`
- Address format: Mainnet HRP `ltc...`, Testnet HRP `tltc...`

#### Cardano (ADA)

- Networks: Mainnet and Testnet
- Scheme: CIP-1852 / Shelley
- Account path: `m/1852'/1815'/0'`
- Payment paths: `m/1852'/1815'/0'/0/index`
- Address format: Mainnet `addr...`, Testnet `addr_test...`

#### Monero (XMR)

- Networks: Mainnet and Testnet
- Seed input: derived from BIP39 seed bytes + passphrase
- Index 0 = standard address; index ≥1 = subaddress (major=0, minor=index)
- Displayed derivation label: `xmr(major=0,minor=index)`

Monero is deterministic for the same mnemonic, passphrase, network, and index.

---

### Practical Recovery Flow

1. Run `SeedCTL` in a trusted offline environment.
2. Choose **Create new wallet** for ceremony replay using the same entropy inputs, or **Import existing wallet** if you already have the mnemonic.
3. Enter exactly the same passphrase.
4. Select the same coin / network.
5. Select the same derivation mode / style / path.
6. Generate the same address index range.
7. Compare outputs against your recorded reference.

---

### Output Verification Checklist

For a successful reproduction, compare:

- Mnemonic words and order
- BIP39 word indexes
- Displayed derivation path(s)
- Account-level extended / public keys (where applicable)
- Generated addresses for the same indices

If all of the above match, reproduction is confirmed for that coin / path configuration.

---

### Common Causes of Mismatch

- Using Hybrid mode when expecting deterministic replay
- Different dice sequence
- Different mnemonic size
- Different passphrase (including spacing or case differences)
- Wrong network (e.g. mainnet vs testnet)
- Different derivation style (standard vs ledger vs custom)
- Different custom path template
- Comparing different address indices
