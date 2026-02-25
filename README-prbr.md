<p align="center">
  <img src=".github/assets/seedctl.png" alt="SeedCTL" width="480"/>
</p>

[![Build and Release (Linux & Windows)](https://github.com/williamcanin/seedctl/actions/workflows/release.yml/badge.svg)](https://github.com/williamcanin/seedctl/actions/workflows/release.yml)
![Release](https://img.shields.io/github/v/release/williamcanin/seedctl?label=latest&color=blue)
![License](https://img.shields.io/github/license/williamcanin/seedctl)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange)
![Offline](https://img.shields.io/badge/works-offline-important)
![Deterministic](https://img.shields.io/badge/deterministic-yes-success)
![No network](https://img.shields.io/badge/network-none-lightgrey)

🇺🇸 [**Read in English**](README.md)

**SeedCTL** é um gerador de carteiras **multichain** (‘Bitcoin’, Ethereum, BNB Smart Chain, XRP Ledger, Tron, Solana, Cardano e Monero), **determinístico, auditável e focado em segurança**, escrito em [**Rust**](https://rust-lang.org/) para uso **offline**.

Este programa permite gerar (ou importar) uma seed BIP39 e derivar carteiras para múltiplas redes a partir de **dados físicos (dado/dice) 🎲** e/ou **entropia do sistema**, produzindo:

- Mnemonic BIP39 (12 ou 24 palavras)
- Suporte a **passphrase opcional**
- Derivação **BIP84 (Native SegWit – bc1)** para ‘Bitcoin’ (m/84'/0'/0' e m/84'/1'/0')
- Derivação **BIP44 padrão Ethereum** (m/44'/60'/0'/0/x) e estilo **Ledger** (m/44'/60'/x'/0/0)
- Derivação **BIP44 para BNB Smart Chain** (m/44'/60'/0'/0/x) e estilo **Ledger** (m/44'/60'/x'/0/0)
- Derivação **BIP44 XRP Ledger** (m/44'/144'/0'/0/x)
- Derivação **BIP44 Tron** (m/44'/195'/0'/0/x)
- Derivação **BIP44 Solana** (m/44'/501'/index'/0')
- Derivação **CIP-1852 Cardano (Shelley)** (m/1852'/1815'/account'/0/index)
- Derivação **Monero (subaddress)** com `xmr(major=0,minor=index)` e key-origin `m/44'/128'/0'/0/0`
- Suporte a **Mainnet e Testnet** (Bitcoin, XRP, Cardano e Monero)
- Exibição de [**Word Indexes BIP39**](https://github.com/bitcoin/bips/blob/master/bip-0039/english.txt)
- Geração de **endereços determinísticos** para:
  - **Bitcoin**: endereços `bc1...` / `tb1...` (Native SegWit)
  - **Ethereum**: endereços `0x...` (checksum EIP‑55)
  - **BNB Smart Chain**: endereços `0x...` (checksum EIP‑55)
  - **XRP Ledger**: endereços clássicos `r...`
  - **Tron**: endereços `T...` (base58check com prefixo 0x41)
  - **Solana**: endereços base58 de chave pública Ed25519
  - **Cardano**: endereços `addr...` / `addr_test...` (Shelley base address)
  - **Monero**: endereços base58 (standard/subaddress)

O objetivo principal é permitir **geração segura, verificável e ‘offline’** de seeds BIP39 reutilizáveis em múltiplas moedas, com alto nível de paranoia e controle total do processo.

---

## Mirrors

Este repositório é mantido principalmente no **GitHub**.

Um mirror sincronizado está disponível no **GitLab**:

- **GitHub (canonical)**: https://github.com/williamcanin/seedctl
- **GitLab (mirror)**: https://gitlab.com/williamcanin/seedctl

---

## Status do projeto

Indicadores de manutenção e atividade para o repositório canonical do **GitHub**.

![Last commit](https://img.shields.io/github/last-commit/williamcanin/seedctl)
![Issues](https://img.shields.io/github/issues/williamcanin/seedctl)
![Stars](https://img.shields.io/github/stars/williamcanin/seedctl)
![Forks](https://img.shields.io/github/forks/williamcanin/seedctl)

---

## Filosofia de Segurança

- Nenhuma dependência de rede
- Nenhum envio de dados
- Nenhuma persistência em disco
- Ideal para uso **offline / air-gapped**
- Compatível com verificação manual (dice, word indexes, derivation path)
- Separação clara entre **modo determinístico** e **modo híbrido**

> **ATENÇÃO**
> Este programa **exibe informações sensíveis** (mnemonic, passphrase, chaves).
> Utilize **somente em ambiente seguro e ‘offline’**. Recomendável usar com [Tails](https://tails.net/)

---

## Funcionalidades

- BIP39 – 12 ou 24 palavras
- Entropia via dados físicos (1–6)
- Entropia híbrida (dados físicos + RNG do sistema)
- Geração automática ou entrada manual de dados
- Confirmação visual da sequência de dados
- Passphrase opcional (BIP39)
- Menu inicial com **Generate new wallet** e **Import existing wallet** (seed já existente)
- Seleção de rede: **Bitcoin, Ethereum, BNB Smart Chain, XRP Ledger, Tron, Solana, Cardano e Monero**
- Suporte a Mainnet e Testnet (Bitcoin, XRP, Cardano e Monero)
- Derivação BIP84 (Bitcoin), BIP44 (Ethereum, BNB, XRP, Tron, Solana), CIP-1852 (Cardano), subaddress (Monero)
- Exibição dos **Word Indexes** (base 1, formato `0001`)
- Geração de endereços:
  - `bc1` / `tb1` (Bitcoin)
  - `0x...` (Ethereum)
  - `0x...` (BNB Smart Chain)
  - `r...` (XRP Ledger)
  - `T...` (Tron)
  - base58 (Solana)
  - `addr...` / `addr_test...` (Cardano)
  - base58 (Monero)

---

## Documentação

- **Reprodução determinística de carteiras**
  Veja [`REPRODUCIBILITY.md`](REPRODUCIBILITY.md)

- **Verificação de binários e releases (SHA256 + GPG)**
  Veja [`VERIFYING_RELEASES.md`](VERIFYING_RELEASES.md)

---

## Modos de Entropia

O programa oferece **dois modos distintos**, com objetivos diferentes.

### Modo Manual (Determinístico)

Indicado para:

- Recuperar uma carteira existente
- Auditoria
- Cerimônias de geração reproduzíveis
- Verificação independente

**Como funciona:**

- O utilizador informa manualmente a sequência de dados (1–6)
- Nenhuma entropia do sistema é utilizada
- A mesma sequência + mesma passphrase ⇒ **sempre a mesma carteira**

**Modelo conceitual:**

```bash
entropy = SHA256(dice_entropy)
```

Este modo é **100% determinístico e reproduzível**.

---

### Modo Automático (Híbrido)

Indicado para:

- Criar carteiras novas
- Aumentar entropia contra falhas humanas
- Defesa em profundidade

**Como funciona:**

- O programa gera automaticamente:
  - Dados físicos aleatórios (1–6)
  - Entropia segura do sistema (CSPRNG)
- As duas fontes são combinadas e hash

**Modelo conceitual:**

```bash
entropy_final = SHA256(dice_entropy || hex_entropy)
```

✔ Mesmo que uma fonte falhe, a outra preserva a segurança
✔ Não depende exclusivamente do humano
✔ Não depende exclusivamente do sistema

**Importante:**
Este modo **não é reproduzível** se apenas o dice for anotado.
Para reprodução futura, o modo manual deve ser utilizado.

---

## Word Indexes (BIP39)

Cada palavra do mnemonic é acompanhada do seu índice na wordlist BIP39:

```bash
01. 0001 abandon
02. 1845 ability
03. 0097 able
```

## Derivation Paths por rede

- **Bitcoin**
  - Mainnet: `m/84'/0'/0'`
  - Testnet: `m/84'/1'/0'`

- **Ethereum**
  - Padrão (MetaMask e afins): `m/44'/60'/0'/0/x`
  - Ledger style: `m/44'/60'/x'/0/0`

- **BNB Smart Chain**
  - Padrão (EVM): `m/44'/60'/0'/0/x`
  - Ledger style: `m/44'/60'/x'/0/0`

- **XRP Ledger**
  - Padrão BIP44: `m/44'/144'/0'/0/x`

- **Tron**
  - Padrão BIP44: `m/44'/195'/0'/0/x`

- **Solana**
  - Padrão BIP44: `m/44'/501'/index'/0'`

- **Cardano**
  - Padrão CIP-1852 (Shelley): `m/1852'/1815'/account'/0/index`

- **Monero**
  - Exibição de endereços/subendereços: `xmr(major=0,minor=index)`
  - Key-origin de export/watch-only: `m/44'/128'/0'/0/0`

---

## Endereços

Geração de endereços determinísticos a partir dos paths escolhidos:

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
  m/44'/501'/0'/0' (index 0) → <endereço base58>
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

## Compatibilidade

- **Bitcoin**
  - Sparrow Wallet
  - Electrum
  - BlueWallet
  - Bitcoin Core
  - Qualquer wallet BIP39/BIP84 compatível

- **Ethereum**
  - MetaMask
  - Ledger Live (via paths padrão / Ledger)
  - Outras wallets BIP39/BIP44 com `m/44'/60'/0'/0/x`

- **BNB Smart Chain**
  - MetaMask (rede BSC)
  - Trust Wallet
  - Rabby
  - Outras wallets EVM com `m/44'/60'/0'/0/x`

- **XRP Ledger**
  - Xaman (XUMM)
  - Ledger Live
  - Outras wallets XRPL com `m/44'/144'/0'/0/x`

- **Tron**
  - TronLink e carteiras que usem `m/44'/195'/0'/0/x`

- **Solana**
  - Phantom
  - Solana CLI / solana-keygen
  - Outras wallets que usem `m/44'/501'/index'/0'`

- **Cardano**
  - Eternl
  - Yoroi
  - Lace
  - Outras wallets Shelley/CIP-1852 com `m/1852'/1815'/account'/0/index`

- **Monero**
  - Monero GUI Wallet
  - Monero CLI (`monero-wallet-cli`)
  - Feather Wallet
  - Carteiras compatíveis com endereços standard/subaddress

---

## Aviso Legal

Este ‘software’ é fornecido “como está”, sem garantias.

Você é 100% responsável pelo uso, armazenamento e segurança das chaves geradas.

> Este software pode expor chaves privadas de forma irreversível.
> Use somente se você compreender completamente o gerenciamento de chaves.

---

## Threat Model

**Este ‘software’ NÃO PROTEGE contra:**

- Malware no sistema operacional
- Keyloggers
- Screen capture
- Firmware comprometido
- Supply-chain attacks

**Este software PROTEGE contra:**

- Falhas de RNG do sistema (via dados físicos)
- Dependência de serviços externos
- Seed generation opaca
- Falta de auditabilidade

Para máxima segurança, use num computador offline, limpo e temporário.

---

## Requisitos para desenvolvimento

- Rust 1.89

Verifique com:

```bash
rustc --version
```

---

## Créditos

Este projeto foi construído com base em padrões bem estabelecidos do Bitcoin e no esforço coletivo da comunidade de código aberto.

### Autor e Colaboradores

- **William C. Canin** — Criador e Mantenedor
- **[O seu Nome Aqui]** — Torne-se um colaborador! Envie uma solicitação de pull request ou relate um problema.

### Propostas de Melhoria do ‘Bitcoin’ (‘BIPs’)

- **BIP32**: Carteiras Hierárquicas Determinísticas.

- **BIP39**: código mnemonic para geração de chaves determinísticas.

- **BIP84**: esquema de derivação para carteiras SegWit nativas.

### Ecossistema Rust

O **SeedCTL** foi construído usando bibliotecas de código aberto de alta qualidade da comunidade Rust. Apoiam o-nos nos ombros de gigantes para garantir segurança e desempenho.

Você pode encontrar a lista completa de bibliotecas e as suas versões no nosso [Cargo.toml](./Cargo.toml).

Agradecimentos especiais aos mantenedores do `bitcoin`, `bip39` e de todos os outros crates que tornam este projeto possível.

### Agradecimentos à Comunidade

Agradecimentos especiais aos desenvolvedores do **Bitcoin Core** e à comunidade global de código aberto por priorizarem a transparência e a soberania do usuário.

---

## Suporte para este projeto

[![Donate](https://img.shields.io/badge/Donate-Bitcoin%20|%20Pix%20|%20PayPal-F5C400?style=for-the-badge)](
https://github.com/williamcanin/donations
)
[![Sponsor](https://img.shields.io/badge/Sponsor-GitHub-%23ea4aaa?style=for-the-badge)](
https://github.com/sponsors/williamcanin
)

> Você aparecerá nos colaboradores.

---

Este projeto foi construído com um forte foco em **segurança, transparência e verificabilidade**, visando dar aos utilizadores controle total sobre as suas chaves e derivations em Bitcoin, Ethereum, BNB Smart Chain, XRP Ledger, Tron, Solana, Cardano e Monero.
