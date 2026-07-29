---
layout: doc
part: 5
section: Wallet Compatibility
menu: seedctl
version: "0.2.4"
doc_product: seedctl
logo: https://raw.githubusercontent.com/orbitbits/seedctl/refs/heads/main/.github/logo/compact/seedctl-text.svg
title: Wallet Compatibility
description: Wallet compatibility across supported blockchains.
date: 2026-04-16 19:49:43 -0300
tags: [CLI, Rust, Cryptocurrency, Seed Generator, Bip39, Multichain]
published: true
permalink: /seedctl/documentation/0.2.4/wallet-compatibility/
---

{% include assign.liquid %}

## Wallet Compatibility

### Bitcoin

- Sparrow Wallet
- Electrum
- BlueWallet
- Bitcoin Core

Cold signing uses PSBT (BIP174). The unsigned PSBT should include input UTXO data and key origin metadata (`bip32_derivation` or Taproot origins).

### Ethereum

- MetaMask
- Rabby
- Ledger Live
- Other `BIP39/BIP44 wallets`

EVM cold signing accepts unsigned raw transaction hex and signs it offline with the configured chain ID, such as `1` for Ethereum or `56` for BNB Smart Chain.

### Other Chains

Compatibility for BNB, XRP, Tron, Solana, Cardano, and others follows their respective derivation standards listed above.
