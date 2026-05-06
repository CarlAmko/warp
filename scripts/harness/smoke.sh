#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root_dir"

require_file() {
  local path="$1"
  if [[ ! -e "$path" ]]; then
    echo "missing required harness file: $path" >&2
    exit 1
  fi
}

require_command() {
  local command_name="$1"
  local setup_hint="$2"
  if ! command -v "$command_name" >/dev/null 2>&1; then
    echo "missing required command: $command_name" >&2
    echo "setup: $setup_hint" >&2
    exit 1
  fi
}

echo "==> Checking harness files"
require_file "AGENTS.md"
require_file "WARP.md"
require_file "Cargo.toml"
require_file "Makefile.harness"
require_file "PLANS.md"
require_file "docs/ARCHITECTURE.md"
require_file "docs/OBSERVABILITY.md"
require_file "scripts/audit_harness.sh"

echo "==> Checking required commands"
require_command "cargo" "install Rust with rustup, then run ./script/bootstrap if platform dependencies are missing"

echo "==> Running focused app typecheck"
cargo check -p warp
