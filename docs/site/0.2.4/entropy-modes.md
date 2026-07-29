---
layout: doc
part: 3
section: Entropy Modes
menu: seedctl
version: "0.2.4"
doc_product: seedctl
logo: https://raw.githubusercontent.com/orbitbits/seedctl/refs/heads/main/.github/logo/compact/seedctl-text.svg
title: Entropy Modes
description: Deterministic and hybrid entropy generation modes.
date: 2026-04-16 19:49:43 -0300
tags: [CLI, Rust, Cryptocurrency, Seed Generator, Bip39, Multichain]
published: true
permalink: /seedctl/documentation/0.2.4/entropy-modes/
---

{% include assign.liquid %}

## Entropy Modes

### Deterministic Mode

- Manual dice sequence input
- No hidden runtime randomness
- Best for recovery and audit workflows

### Hybrid Mode

- Combines dice entropy and system RNG.
- Good for creating new wallets with defense in depth.
- Not intended for exact deterministic replay.

```sh
dice_entropy = SHA256(dice_sequence_bytes)

deterministic: entropy_final = truncate_bits(dice_entropy, mnemonic_bits)
hybrid:        entropy_final = truncate_bits(SHA256(dice_entropy || system_entropy_32B), mnemonic_bits)
```
