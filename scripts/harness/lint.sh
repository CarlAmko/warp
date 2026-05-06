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

echo "==> Running cargo fmt"
cargo fmt -- --check

echo "==> Running clippy"
cargo clippy --workspace --exclude warp_completer --all-targets --tests -- -D warnings
cargo clippy -p warp_completer --all-targets --tests -- -D warnings

echo "==> Running clang-format check"
clang_format_executable="$(command -v clang-format || true)"
if [[ -z "$clang_format_executable" ]] && command -v xcrun >/dev/null 2>&1; then
  clang_format_executable="$(xcrun -find clang-format 2>/dev/null || true)"
fi
if [[ -z "$clang_format_executable" ]]; then
  echo "missing required command: clang-format" >&2
  echo "setup: install clang-format with ./script/bootstrap or brew install clang-format" >&2
  exit 1
fi
./script/run-clang-format.py \
  --clang-format-executable "$clang_format_executable" \
  -r --extensions 'c,h,cpp,m' ./crates/warpui/src/ ./app/src/

echo "==> Running WGSL formatting check"
find . -name "*.wgsl" -exec wgslfmt --check {} +

if command -v pwsh >/dev/null 2>&1; then
  echo "==> Running PSScriptAnalyzer"
  ./script/lint_powershell -ci
elif [[ "${GITHUB_ACTIONS:-}" == "true" ]]; then
  echo "missing required command in CI: pwsh" >&2
  echo "setup: install PowerShell before running make check in CI" >&2
  exit 1
else
  echo "==> Skipping PSScriptAnalyzer because pwsh is not installed"
fi
