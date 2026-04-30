#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "Building ghost engine..."
cargo build --release

echo ""
echo "Built: $SCRIPT_DIR/target/release/ghost"
echo ""
echo "For a fully static binary on Linux (optional):"
echo "  rustup target add x86_64-unknown-linux-musl"
echo "  RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-musl"
