#!/usr/bin/env bash
# Basic deployment helper for KAIRO
set -e

ROOT_DIR=$(cd "$(dirname "$0")" && pwd)

echo "Building rust-core..."
cargo build --manifest-path "$ROOT_DIR/rust-core/Cargo.toml" --release

# Package Python client
ARCHIVE="$ROOT_DIR/kairo.tar.gz"

tar czf "$ARCHIVE" -C "$ROOT_DIR" src scripts

echo "Deployment package created: $ARCHIVE"
