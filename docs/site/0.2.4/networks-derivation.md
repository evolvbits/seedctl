---
layout: doc
part: 4
section: Networks & Derivation
menu: seedctl
version: "0.2.4"
doc_product: seedctl
logo: https://raw.githubusercontent.com/orbitbits/seedctl/refs/heads/main/.github/logo/compact/seedctl-text.svg
title: Networks & Derivation Paths
description: Supported network derivation paths and address formats.
date: 2026-04-16 19:49:43 -0300
tags: [CLI, Rust, Cryptocurrency, Seed Generator, Bip39, Multichain]
published: true
permalink: /seedctl/documentation/0.2.4/networks-derivation/
---

{% include assign.liquid %}

## Networks & Derivation Paths

| Network    | Primary Path / Style                  | Address Format             |
| ---------- | ------------------------------------- | -------------------------- |
| Bitcoin    | `m/84'/coin_type'/0'` (+ BIP49/BIP44) | `bc1...` / `tb1...`        |
| Ethereum   | `m/44'/60'/0'/0/x` (+ ledger/custom)  | `0x...`                    |
| BNB Chain  | `m/44'/60'/0'/0/x` (+ ledger/custom)  | `0x...`                    |
| XRP Ledger | `m/44'/144'/0'/0/x`                   | `r...`                     |
| Tron       | `m/44'/195'/0'/0/x` (+ ledger/custom) | `T...`                     |
| Solana     | `m/44'/501'/index'/0'`                | base58                     |
| Litecoin   | `m/84'/coin_type'/0'/0/x`             | `ltc...` / `tltc...`       |
| Polygon    | `m/44'/60'/0'/0/x` (+ ledger/custom)  | `0x...`                    |
| Cardano    | `m/1852'/1815'/0'/0/index`            | `addr...` / `addr_test...` |
| Monero     | `xmr(major=0,minor=index)`            | base58                     |
