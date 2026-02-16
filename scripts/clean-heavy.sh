#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# Heavy build artifacts that can be recreated from source/dependencies.
paths=(
  "dist"
  "dist-ssr"
  "src-tauri/target"
  "node_modules/.vite"
  ".tmp"
  ".cache"
)

for path in "${paths[@]}"; do
  rm -rf "$path"
done

echo "Removed heavy build artifacts."
