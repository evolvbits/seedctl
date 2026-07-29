---
layout: doc
part: 9
section: Security & Terms
menu: seedctl
version: "0.2.4"
doc_product: seedctl
logo: https://raw.githubusercontent.com/orbitbits/seedctl/refs/heads/main/.github/logo/compact/seedctl-text.svg
title: Terms of Use
description: Terms of use and operational responsibilities.
date: 2026-04-16 19:49:43 -0300
tags: [CLI, Rust, Cryptocurrency, Seed Generator, Bip39, Multichain]
published: true
permalink: /seedctl/documentation/0.2.4/terms-of-use/
---

{% include assign.liquid %}

## Terms of Use

### Last updated: February 26, 2026

These terms describe the operational responsibilities and risk assumptions accepted when you use `SeedCTL`.

### Scope

- `SeedCTL` is provided as a technical tool for deterministic wallet generation and verification workflows.
- The software does not custody assets, execute transactions, or provide investment advice.
- `SeedCTL` is distributed under the GNU General Public License v3.0 only (`GPL-3.0-only`).

### User Responsibilities

- Use the software only in trusted environments under your control.
- Protect mnemonic phrases, passphrases, private keys, and exported files at all times.
- Validate derivation paths, network selection, and addresses before funding any wallet.

### Security Care

- Keep host OS, firmware, and security controls hardened and updated.
- Prefer offline or air-gapped procedures for high-value key generation.
- Maintain tested backup and disaster-recovery procedures.

### Risk Disclosure

- Any compromise of your host, inputs, storage, or operational process can cause irreversible asset loss.
- Blockchain transfers are irreversible; wrong addresses or wrong networks may not be recoverable.
- Deterministic reproduction depends on exact matching inputs, passphrase, path, and network.

### Acceptance

- By using `SeedCTL`, you agree to these terms and confirm you understand the operational risks.
