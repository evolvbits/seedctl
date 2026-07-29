---
layout: doc
part: 2
section: Execution Flow
menu: seedctl
version: "0.2.4"
doc_product: seedctl
logo: https://raw.githubusercontent.com/orbitbits/seedctl/refs/heads/main/.github/logo/compact/seedctl-text.svg
title: Execution Flow
description: Understanding the SeedCTL execution pipeline.
date: 2026-04-16 19:49:43 -0300
tags: [CLI, Rust, Cryptocurrency, Seed Generator, Bip39, Multichain]
published: true
permalink: /seedctl/documentation/0.2.4/execution-flow/
---

{% include assign.liquid %}

## Execution Flow

### Entropy Input

Manual dice for reproducibility or hybrid with system RNG for fresh generation.

### Mnemonic Stage

BIP39 12/24 words, optional passphrase, deterministic seed derivation.

### Path Selection

Choose chain, network, derivation style/path and address index range.

### Cold Signing

SeedCTL can use the same BIP39 seed material for offline signing:

- Bitcoin PSBT: reads a PSBT payload or `.psbt` file, signs matching inputs via `rust-bitcoin`, and outputs the signed PSBT.
- EVM raw hex: reads an unsigned EIP-2718 transaction hex, applies the selected chain ID, signs with Alloy, and outputs signed raw transaction hex.

Bitcoin PSBT signing requires key origin metadata in the PSBT so SeedCTL can match inputs to derived keys.

### Verification

Validate paths, keys and addresses against expected values before use.
