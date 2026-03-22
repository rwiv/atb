#!/bin/bash

# atb installation script

set -e

# Find project root by looking for Cargo.toml
while [ ! -f "Cargo.toml" ] && [ "$PWD" != "/" ]; do
    cd ..
done

if [ ! -f "Cargo.toml" ]; then
    echo "Error: Project root not found (Cargo.toml not found)." >&2
    exit 1
fi

BINARY_NAME="atb"
INSTALL_DIR="$HOME/.local/bin"
TARGET_PATH="$INSTALL_DIR/$BINARY_NAME"

echo "Building $BINARY_NAME in release mode..."
cargo build --release

# Ensure install directory exists
mkdir -p "$INSTALL_DIR"

# Remove existing binary if it exists
if [ -f "$TARGET_PATH" ]; then
  echo "Removing existing binary at $TARGET_PATH"
  rm "$TARGET_PATH"
fi

echo "Installing $BINARY_NAME to $INSTALL_DIR"
cp "target/release/$BINARY_NAME" "$TARGET_PATH"

echo "Successfully installed $BINARY_NAME to $TARGET_PATH"
echo "Make sure $INSTALL_DIR is in your PATH."
