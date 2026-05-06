#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root_dir"

require_command() {
  local command_name="$1"
  local setup_hint="$2"
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "missing required command: $command_name" >&2
    echo "setup: $setup_hint" >&2
    exit 1
  fi
}

require_command "cargo" "install Rust with rustup, then run ./script/bootstrap if platform dependencies are missing"
require_command "cargo-nextest" "run ./script/install_cargo_test_deps or install cargo-nextest with cargo binstall"

echo "==> Running workspace tests via nextest"
cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2

echo "==> Running warp_completer v2 tests"
cargo nextest run -p warp_completer --features v2

echo "==> Running doc tests"
cargo test --doc
