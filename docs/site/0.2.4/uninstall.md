---
layout: doc
part: 11
section: Uninstall
menu: seedctl
version: "0.2.4"
doc_product: seedctl
logo: https://raw.githubusercontent.com/orbitbits/seedctl/refs/heads/main/.github/logo/compact/seedctl-text.svg
title: Uninstall
description: How to uninstall SeedCTL.
date: 2026-04-16 19:49:43 -0300
tags: [CLI, Rust, Cryptocurrency, Seed Generator, Bip39, Multichain]
published: true
permalink: /seedctl/documentation/0.2.4/uninstall/
---

{% include assign.liquid %}

## Uninstall

To uninstall, simply **delete the binary**, or if you want a **complete cleanup**, run the same installation command with the uninstallation parameter:

For [Linux](https://www.kernel.org/){:target="_blank"}:

```sh
bash <(curl -fsSL {{ url_full }}/seedctl/linux.sh) --uninstall
```

For [macOS](https://www.apple.com/macos/){:target="_blank"}:

```sh
bash <(curl -fsSL {{ url_full }}/seedctl/macos.sh) --uninstall
```

Para [Windows](https://www.microsoft.com/windows/){:target="_blank"}:

```batch
& ([scriptblock]::Create((irm '{{ url_full }}/seedctl/windows.ps1'))) -Uninstall
```
