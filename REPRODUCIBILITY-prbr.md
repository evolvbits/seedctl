# Reprodutibilidade e Recuperação Determinística de Carteiras

Este documento explica como reproduzir deterministicamente carteiras geradas pelo SeedCTL.

Redes/moedas atuais no projeto:

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

O objetivo é permitir reconstruir os mesmos dados de carteira a partir dos mesmos inputs, sem depender de confiança em uma máquina ou sessão específica.

---

## Princípio Central

Uma carteira só é reproduzível quando todos os parâmetros relevantes são idênticos:

1. Origem do mnemonic (gerado no SeedCTL ou frase BIP39 importada)
2. Tamanho do mnemonic (12 ou 24 palavras), quando gerado no SeedCTL
3. Modo de entropia (Híbrido ou Determinístico), quando gerado no SeedCTL
4. Sequência de dados (dice), se utilizada
5. Passphrase BIP39 (exatamente igual)
6. Rede/moeda selecionada
7. Modo/estilo/path de derivação selecionado para a moeda

Se qualquer item mudar, os resultados mudam.

---

## Modelo de Entropia (Como Implementado)

Ao criar um novo mnemonic no SeedCTL, o pipeline de entropia é:

- `dice_entropy = SHA256(bytes_da_sequencia_de_dados)`

### Modo determinístico (dice manual)

```bash
entropy_final = truncate_bits(dice_entropy, mnemonic_bits)
```

- Nenhuma aleatoriedade do sistema é adicionada.
- Reproduzível com a mesma sequência de dados e o mesmo tamanho de mnemonic.

### Modo híbrido (dice automático + RNG do sistema)

```bash
entropy_final = truncate_bits(SHA256(dice_entropy || system_entropy_32B), mnemonic_bits)
```

- Adiciona RNG do sistema.
- Indicado para geração de carteiras novas, não para replay de cerimônia determinística.

Importante:
- Para reprodutibilidade estrita, use modo determinístico/manual ou importe um mnemonic existente.

---

## O Que Registrar para Recuperação Futura

Para uma cerimônia determinística, registre no mínimo:

- tamanho do mnemonic (12/24)
- modo de entropia
- sequência completa de dados (se usada)
- passphrase (ou explicitamente "vazia")
- moeda/rede selecionada
- modo/estilo/path de derivação selecionado
- faixa de índices de endereço gerada (ex.: 0..9)

Para carteiras importadas, registre:

- frase mnemonic completa
- passphrase
- moeda/rede
- modo/estilo/path de derivação

---

## Parâmetros de Reprodutibilidade por Moeda

### Bitcoin (BTC)

- Redes: Mainnet e Testnet
- Coin type: Mainnet `0`, Testnet `1`
- Finalidade de derivação selecionável: BIP84, BIP49, BIP44
- Account path (BIP84): `m/84'/coin_type'/0'` (native SegWit)
- Account path (BIP49): `m/49'/coin_type'/0'` (nested SegWit)
- Account path (BIP44): `m/44'/coin_type'/0'` (legacy)
- Padrão de receive path: `.../0/index`

Para reproduzir BTC com exatidão, mantenha iguais tanto a rede quanto o purpose.

### Ethereum (ETH), BNB Smart Chain (BNB), Polygon (POL/MATIC)

As três usam o mesmo mecanismo de derivação EVM.

- Modo de derivação: gerar endereços, ou escanear paths comuns
- Estilo (Standard): base `m/44'/60'/0'/0`, endereços em `/index`
- Estilo (Ledger): endereços em `m/44'/60'/index'/0/0`
- Estilo (Custom): suporta placeholder `{index}`; se o path termina com `/`, o índice é anexado

Para recuperação determinística, use o mesmo estilo e o mesmo template customizado (se houver).

### XRP Ledger (XRP)

- Redes: Mainnet e Testnet
- Base path: `m/44'/144'/0'/0`
- Address paths: `m/44'/144'/0'/0/index`
- Formato de endereço: XRPL clássico (`r...`)

### Tron (TRX)

- Fluxo atual: focado em mainnet (sem seletor explícito de rede no UX atual)
- Estilo (Standard): `m/44'/195'/0'/0/index`
- Estilo (Ledger): `m/44'/195'/0'/index'/0/0`
- Estilo (Custom): path customizado suportado
- Formato de endereço: Base58Check com prefixo Tron (`T...`)

### Solana (SOL)

- Fluxo atual: focado em mainnet (sem seletor explícito de rede no UX atual)
- Path: `m/44'/501'/index'/0'`
- Formato de endereço: chave pública Ed25519 em base58

### Litecoin (LTC)

- Redes: Mainnet e Testnet
- Coin type: Mainnet `2`, Testnet `1`
- Account path: `m/84'/coin_type'/0'`
- Receive paths: `m/84'/coin_type'/0'/0/index`
- Formato de endereço: HRP mainnet `ltc...`, HRP testnet `tltc...`

### Cardano (ADA)

- Redes: Mainnet e Testnet
- Esquema: CIP-1852 / Shelley
- Conta atualmente fixa em `0`
- Account path: `m/1852'/1815'/0'`
- Payment paths: `m/1852'/1815'/0'/0/index`
- Formato de endereço: Mainnet `addr...`, Testnet `addr_test...`

### Monero (XMR)

- Redes: Mainnet e Testnet
- Seed de entrada: derivada dos bytes da seed BIP39 + passphrase
- Modelo de indexação de endereços usado no projeto: índice `0` = padrão; índice `>=1` = subaddress (`major=0, minor=index`)
- Rótulo de derivação exibido: `xmr(major=0,minor=index)`

No projeto atual, Monero é determinístico para o mesmo mnemonic, passphrase, rede e índice.

---

## Fluxo Prático de Recuperação

1. Execute o SeedCTL em ambiente offline confiável.
2. Escolha `Create new wallet` para replay de cerimônia com os mesmos inputs de entropia, ou `Import existing wallet` se você já possui o mnemonic.
3. Informe/selecione a mesma passphrase.
4. Selecione a mesma moeda/rede.
5. Selecione o mesmo modo/estilo/path de derivação.
6. Gere a mesma faixa de índices de endereço.
7. Compare os resultados.

---

## Checklist de Verificação de Saída

Para confirmar reprodução bem-sucedida, compare:

- palavras do mnemonic e ordem
- índices BIP39 das palavras
- path(s) de derivação exibidos
- chaves públicas/estendidas de nível de conta (quando aplicável)
- endereços gerados para os mesmos índices

Se tudo acima coincidir, a reprodução está correta para aquela configuração de moeda/path.

---

## Causas Comuns de Divergência

- Usar modo Híbrido esperando replay determinístico
- Sequência de dados diferente
- Tamanho de mnemonic diferente
- Passphrase diferente (incluindo espaços/maiúsculas)
- Rede errada (ex.: mainnet vs testnet)
- Estilo de derivação diferente (standard vs ledger vs custom)
- Template de path customizado diferente
- Comparar índices de endereço diferentes

---

## Nota Final

Reprodutibilidade é um controle de segurança: se você consegue regenerar os resultados da carteira a partir dos inputs documentados, não precisa confiar em uma única sessão de execução.
