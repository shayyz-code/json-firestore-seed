#!/bin/sh
set -e

REPO="shayyz-code/json-firestore-seed"
BINARY="json-firestore-seed"

# Detect OS
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
case "${OS}" in
  linux*)  OS='linux' ;;
  darwin*) OS='apple-darwin' ;;
  *)       echo "Unsupported OS: ${OS}"; exit 1 ;;
esac

# Detect Architecture
ARCH="$(uname -m)"
case "${ARCH}" in
  x86_64) ARCH='x86_64' ;;
  arm64|aarch64) ARCH='aarch64' ;;
  *)      echo "Unsupported architecture: ${ARCH}"; exit 1 ;;
esac

# Construct Target
if [ "${OS}" = "apple-darwin" ]; then
    TARGET="${ARCH}-${OS}"
else
    TARGET="x86_64-unknown-linux-gnu" # Default to x86_64 for Linux for now
fi

# Get latest version
VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' | cut -c 2-)

if [ -z "${VERSION}" ]; then
  echo "Could not find latest version"; exit 1
fi

URL="https://github.com/${REPO}/releases/download/v${VERSION}/${BINARY}_${VERSION}_${TARGET}.tar.gz"

echo "Downloading ${BINARY} v${VERSION} for ${TARGET}..."
curl -L "${URL}" | tar -xz

chmod +x "${BINARY}"

echo "Installing to /usr/local/bin..."
if [ -w /usr/local/bin ]; then
    mv "${BINARY}" /usr/local/bin/
else
    sudo mv "${BINARY}" /usr/local/bin/
fi

echo "Successfully installed ${BINARY} to /usr/local/bin/${BINARY}"
