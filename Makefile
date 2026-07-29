# ----- Makefile -----
BRANCH := $(shell git branch --show-current)
REMOTES := $(shell git remote)
.DEFAULT_GOAL := help

.PHONY: help all build release-linux release-macos release-windows check fmt fmt-check check-workspace clippy test clean audit deny install-hooks push push-lease

help:
	@echo "SeedCTL Makefile"
	@echo
	@echo "Build:"
	@echo "  make build             -> Build the workspace with Cargo.lock"
	@echo "  make release-linux     -> Build Linux x86_64 release binary"
	@echo "  make release-macos     -> Build macOS x86_64 and aarch64 release binaries"
	@echo "  make release-windows   -> Build Windows x86_64 release binary"
	@echo
	@echo "Checks:"
	@echo "  make check             -> Run fmt-check, check-workspace, clippy, and tests"
	@echo "  make fmt               -> Format all Rust code"
	@echo "  make fmt-check         -> Check Rust formatting"
	@echo "  make check-workspace   -> Run cargo check-workspace alias"
	@echo "  make clippy            -> Run Clippy with warnings denied"
	@echo "  make test              -> Run workspace tests"
	@echo "  make audit             -> Run cargo audit"
	@echo "  make deny              -> Run cargo deny check"
	@echo
	@echo "Maintenance:"
	@echo "  make clean             -> Remove Cargo build artifacts"
	@echo "  make install-hooks     -> Configure Git to use hooks/"
	@echo
	@echo "Git:"
	@echo "  make push              -> Push the current branch to all remotes"
	@echo "  make push-lease        -> Force-push with lease to all remotes"

all: build

build:
	cargo build --locked

release-linux:
	cargo release-linux-x86_64

release-macos:
	cargo release-macos-x86_64
	cargo release-macos-aarch64

release-windows:
	cargo release-windows-x86_64

check: fmt-check check-workspace clippy test

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

check-workspace:
	cargo check-workspace

clippy:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-targets --locked

clean:
	cargo clean

audit:
	cargo audit

deny:
	cargo deny check

install-hooks:
	git config core.hooksPath hooks

# ----- GIT PUSH -----
push:
	@echo "Push normal → branch: $(BRANCH)"
	@for remote in $(REMOTES); do \
		echo "  pushing to $$remote..."; \
		git push $$remote $(BRANCH); \
	done

push-lease:
	@echo "Push --force-with-lease → branch: $(BRANCH)"
	@for remote in $(REMOTES); do \
		echo "  pushing to $$remote..."; \
		git push --force-with-lease $$remote $(BRANCH); \
	done
