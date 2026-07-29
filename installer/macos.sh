#!/usr/bin/env bash

# Author: William C. Canin <https://williamcanin.github.io>

set -euo pipefail

NAME="seedctl"
REPO="orbitbits/seedctl"
API_URL="https://api.github.com/repos/${REPO}/releases/latest"
BINARY_NAME="seedctl"
INSTALLATION_DIR="$HOME/.local/bin"
REQUIRED=("curl")

# ----- libs -----
title () {
	printf "\e[0;35m[ %s\e[0m\n" "$1 ]"
}

info () {
	printf "\e[0;36m-> %s\e[0m$2" "$1"
}

finish () {
	printf "\e[0;32m* %s\e[0m\n" "$1"
}

warning () {
	printf "\e[0;33m! %s\e[0m$2" "$1"
}

error () {
	printf "\e[0;31mx %s\e[0m\n" "$1"
}

# ----- macOS check -----
if [ "$(uname -s)" != "Darwin" ]; then
  error "Error: This installer is only supported on macOS."
  exit 1
fi

# ----- Ignore root user -----
if [ "$EUID" -eq 0 ]; then
  error "Error: This script should not be run as root or with sudo."
  exit 1
fi

# ----- Required check -----
for bin in "${REQUIRED[@]}"; do
  if ! command -v "$bin" >/dev/null 2>&1; then
    error "Error: '$bin' not found."
    exit 1
  fi
done

# ----- Architecture check -----
case "$(uname -m)" in
  arm64|aarch64)
    ARCH="macos-aarch64"
    ;;
  x86_64|amd64)
    ARCH="macos-x86_64"
    ;;
  *)
    error "Error: unsupported macOS architecture: $(uname -m)"
    exit 1
    ;;
esac

# ----- Uninstall mode -----
if [ "${1:-}" == "--uninstall" ]; then
    title "$NAME Uninstall"

    if [ -f "$INSTALLATION_DIR/$BINARY_NAME" ]; then
        info "Removing from: " "${INSTALLATION_DIR}\n"
        rm -fv "$INSTALLATION_DIR/$BINARY_NAME"
        finish "Uninstallation completed!"
    else
        warning "No installation found." "\n"
    fi

    exit 0
fi

# ----- Download mode -----
title "$NAME Installation"
VERSION_TAG=$(curl -s "$API_URL" | grep '"tag_name":' | sed -E 's/.*"v?([^"]+)".*/\1/')

if [ -z "$VERSION_TAG" ]; then
		error "Error: Could not retrieve the latest release version from GitHub."
		exit 1
fi

TARGET_FILE="${BINARY_NAME}-${VERSION_TAG}-${ARCH}"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/v${VERSION_TAG}/${TARGET_FILE}"
TMP_FILE="$(mktemp -t "${BINARY_NAME}.XXXXXX")"

info "Latest version: " "${VERSION_TAG}\n"
info "Target file: " "${TARGET_FILE}\n"
info "Download link: " "$DOWNLOAD_URL\n"

if curl -L --fail --progress-bar "$DOWNLOAD_URL" -o "$TMP_FILE"; then
		finish "Download completed successfully."
else
		error "Error: Failed to download the latest release."
		rm -f "$TMP_FILE"
		exit 1
fi

info "Target file rename to: " "${BINARY_NAME}\n"

# ----- Show SHA256SUM Binary -----
if command -v shasum >/dev/null 2>&1; then
	info "SHA256SUM Binary: " "\n"; shasum -a 256 "$TMP_FILE"
fi

# ----- Install mode -----
mkdir -p "$INSTALLATION_DIR"
rm -f "$INSTALLATION_DIR/$BINARY_NAME"
cp -f "$TMP_FILE" "${INSTALLATION_DIR}/$BINARY_NAME"
chmod +x "$INSTALLATION_DIR/$BINARY_NAME"
rm -f "$TMP_FILE"

# ----- Info mode -----
finish "Installation completed successfully!"
warning "$NAME was installed on: " ""; printf "%s\n" "$INSTALLATION_DIR"
warning "NOTE: " "";  printf "Add the path \"%s\" to your shell PATH.\n" "$INSTALLATION_DIR"
