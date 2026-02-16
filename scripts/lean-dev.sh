#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_BASE="${TMPDIR:-/tmp}"
LEAN_WORK_DIR="$(mktemp -d "${TMP_BASE%/}/pkb-lean-dev.XXXXXX")"
LEAN_CARGO_TARGET_DIR="${LEAN_WORK_DIR}/cargo-target"
LEAN_VITE_CACHE_DIR="${LEAN_WORK_DIR}/vite-cache"
LEAN_TAURI_CONFIG="${LEAN_WORK_DIR}/tauri.lean.conf.json"

mkdir -p "$LEAN_CARGO_TARGET_DIR" "$LEAN_VITE_CACHE_DIR"
export PKB_LEAN_VITE_CACHE_DIR="$LEAN_VITE_CACHE_DIR"
export VITE_CACHE_DIR="$LEAN_VITE_CACHE_DIR"

cleanup() {
  rm -rf "$LEAN_WORK_DIR"
}
trap cleanup EXIT INT TERM

node -e 'const fs=require("fs");
const out=process.argv[1];
const port=process.env.PKB_LEAN_DEV_PORT || "1420";
const config={build:{devUrl:`http://localhost:${port}`,beforeDevCommand:`npm run dev -- --port ${port}`}};
fs.writeFileSync(out, JSON.stringify(config));' "$LEAN_TAURI_CONFIG"

cd "$ROOT_DIR"
CARGO_TARGET_DIR="$LEAN_CARGO_TARGET_DIR" \
npm run tauri -- dev --config "$LEAN_TAURI_CONFIG"
