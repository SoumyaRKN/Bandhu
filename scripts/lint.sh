#!/usr/bin/env bash
set -euo pipefail

DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$DIR/.."
BACKEND="$ROOT/backend"
EXT="$ROOT/bandhu"

echo "=== Rust: cargo fmt --check ==="
(cd "$BACKEND" && cargo fmt -- --check)

echo "=== Rust: clippy ==="
(cd "$BACKEND" && cargo clippy)

echo "=== TypeScript: type check ==="
(cd "$EXT" && npx tsc --noEmit)

echo "=== TypeScript: eslint ==="
(cd "$EXT" && npm run lint)

echo "=== Markdown/JSON: prettier check ==="
npx prettier --check "**/*.md" "**/*.json"

echo "=== All checks passed ==="
