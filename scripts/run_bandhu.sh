#!/usr/bin/env bash
set -e

DIR="$(cd "$(dirname "$0")" && pwd)"
BACKEND="$DIR/../backend"
BANDHU="$DIR/../bandhu"

echo "=== Building Rust backend ==="
cd "$BACKEND"
cargo build --release

if [ ! -d "$BACKEND/target/release" ]; then
  echo "Build failed"
  exit 1
fi

echo "=== Backend binary ready ==="
echo "Run with: $BACKEND/target/release/bandhu-backend"

echo "=== Building VS Code extension ==="
cd "$BANDHU"
npm run compile

echo "=== Done ==="
