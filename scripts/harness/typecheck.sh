#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root_dir"

if ! command -v cargo >/dev/null 2>&1; then
  echo "missing required command: cargo" >&2
  echo "setup: install Rust with rustup, then run ./script/bootstrap if platform dependencies are missing" >&2
  exit 1
fi

echo "==> Running focused app typecheck"
cargo check -p warp
