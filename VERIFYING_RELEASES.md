# Verifying Releases

This document explains how to verify the **integrity** and **authenticity** of the binaries published in GitHub Releases.

These steps are designed to match exactly how releases are produced by the current `release.yml` workflow.

---

## What is provided in each release

For every tagged release (`vX.Y.Z`), the GitHub Release contains:

* `seedctl` — Linux binary
* `seedctl.exe` — Windows binary
* `SHA256SUMS` — SHA-256 checksums for all binaries
* `SHA256SUMS.asc` — GPG signature of `SHA256SUMS`

The tag name is derived from `Cargo.toml`:

```
v<version-from-Cargo.toml>
```

---

## Threat model

This verification protects against:

* Accidental corruption during download
* Malicious modification of binaries
* Compromised GitHub account or CI artifacts

It does **not** protect against:

* A compromised signing key
* A malicious maintainer

For stronger guarantees, combine this with **reproducible builds** (see `REPRODUCIBILITY.md`).

---

## Step 1 — Download release files

From the GitHub Releases page, download **all** the following into the same directory:

* `seedctl-linux`
* `seedctl-windows.exe`
* `SHA256SUMS`
* `SHA256SUMS.asc`

Example:

```bash
ls
seedctl-linux
seedctl-windows.exe
SHA256SUMS
SHA256SUMS.asc
```

---

## Step 2 — Import the release signing key

Import the maintainer's public GPG key:

```bash
gpg --import seedctl-release.pub
```

Verify the key fingerprint matches the one published in the repository README:

```bash
gpg --fingerprint
```

⚠️ **Always verify the fingerprint via an independent channel** (website, multiple repositories, or trusted contacts).

---

## Step 3 — Verify authenticity (GPG)

Verify that `SHA256SUMS` was signed by the release key:

```bash
gpg --verify SHA256SUMS.asc SHA256SUMS
```

Expected output:

```text
gpg: Good signature from "<Release Signing Key>"
```

If this step fails, **do not trust the binaries**.

---

## Step 4 — Verify integrity (SHA-256)

Verify that the binaries match the published checksums:

```bash
sha256sum -c SHA256SUMS
```

Expected output:

```text
seedctl-linux: OK
seedctl-windows.exe: OK
```

If any file fails verification, discard it immediately.

---

## Notes on how releases are generated

The release artifacts are built by GitHub Actions using the following guarantees:

* Builds run on:

  * `ubuntu-latest`
  * `windows-latest`
* Rust toolchain: `stable`
* Build command:

```bash
cargo build --release
```

* Version is extracted directly from `Cargo.toml`
* The Git tag is created as:

```text
v<version>
```

* `SHA256SUMS` is generated from the final binaries
* `SHA256SUMS.asc` is created by signing `SHA256SUMS` with GPG inside CI

---

## Reproducible builds

To independently verify that the binaries correspond to the source code, see:

* `REPRODUCIBILITY.md`

Reproducibility + cryptographic signatures together provide the strongest guarantees.

---

## Summary

✔ GPG verifies **who** produced the release
✔ SHA-256 verifies **what** you downloaded
✔ Reproducible builds verify **how** it was built

Always verify **before** using this software, and prefer an offline, air‑gapped environment when possible.
