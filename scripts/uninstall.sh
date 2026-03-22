#!/bin/bash

# atb uninstallation script

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

if [ -f "$TARGET_PATH" ]; then
    echo "Uninstalling $BINARY_NAME from $TARGET_PATH..."
    rm "$TARGET_PATH"
    echo "Successfully uninstalled $BINARY_NAME."
else
    echo "$BINARY_NAME is not installed at $TARGET_PATH."
fi
