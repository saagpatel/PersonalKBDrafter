#!/usr/bin/env bash
set -euo pipefail

fail() {
  echo "FAIL: $1" >&2
  exit 1
}

require_cmd() {
  local cmd="$1"
  local label="$2"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    fail "$label is not installed or not on PATH"
  fi
}

require_cmd node "Node.js"
require_cmd npm "npm"
require_cmd rustc "rustc"
require_cmd cargo "cargo"
require_cmd xcode-select "Xcode Command Line Tools"
require_cmd ollama "Ollama"
require_cmd rg "ripgrep"

if ! xcode-select -p >/dev/null 2>&1; then
  fail "Xcode Command Line Tools are not configured"
fi

NODE_VERSION="$(node -v)"
NPM_VERSION="$(npm -v)"
RUSTC_VERSION="$(rustc --version)"
CARGO_VERSION="$(cargo --version)"
OLLAMA_VERSION="$(ollama --version | head -n 1)"

echo "Node: $NODE_VERSION"
echo "npm: $NPM_VERSION"
echo "rustc: $RUSTC_VERSION"
echo "cargo: $CARGO_VERSION"
echo "xcode-select: $(xcode-select -p)"
echo "Ollama: $OLLAMA_VERSION"

if ! ollama list >/dev/null 2>&1; then
  fail "Ollama is installed but not responding"
fi

if ! ollama list | rg -q '^llama3\.2'; then
  echo "WARN: Recommended model 'llama3.2' is not installed"
  echo "      Run: ollama pull llama3.2"
else
  echo "Ollama model: llama3.2 detected"
fi

echo "Prerequisite check passed"
