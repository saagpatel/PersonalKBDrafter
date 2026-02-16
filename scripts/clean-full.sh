#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# Full local reset of reproducible build/dependency caches.
paths=(
  "node_modules"
  "dist"
  "dist-ssr"
  "src-tauri/target"
  "node_modules/.vite"
  ".tmp"
  ".cache"
  ".codex_audit"
)

for path in "${paths[@]}"; do
  rm -rf "$path"
done

find . -name ".DS_Store" -type f -delete
echo "Removed all reproducible local caches."
