#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/audit_harness.sh [repo_path]

Audit this repository for Phase 0 Harness Engineering Baseline artifacts.
EOF
}

target_path="${1:-.}"
if [[ "$target_path" == "-h" || "$target_path" == "--help" ]]; then
  usage
  exit 0
fi

if [[ ! -d "$target_path" ]]; then
  echo "error: target path does not exist: $target_path" >&2
  exit 1
fi

target_path="$(cd "$target_path" && pwd)"
failures=0

ok() {
  echo "[ok]      $1"
}

fail() {
  echo "[fail]    $1"
  failures=$((failures + 1))
}

check_file() {
  local relative="$1"
  if [[ -f "$target_path/$relative" ]]; then
    ok "$relative"
  else
    fail "missing file: $relative"
  fi
}

check_executable() {
  local relative="$1"
  if [[ -x "$target_path/$relative" ]]; then
    ok "$relative is executable"
  else
    fail "$relative is missing or not executable"
  fi
}

check_contains() {
  local relative="$1"
  local pattern="$2"
  local label="$3"
  local full="$target_path/$relative"

  if [[ ! -f "$full" ]]; then
    fail "$label (file missing: $relative)"
    return
  fi

  if grep -Eq -- "$pattern" "$full"; then
    ok "$label"
  else
    fail "$label"
  fi
}

echo "Auditing harness artifacts in: $target_path"
echo

check_file "AGENTS.md"
check_file "PLANS.md"
check_file "docs/ARCHITECTURE.md"
check_file "docs/OBSERVABILITY.md"
check_file "Makefile"
check_file "Makefile.harness"
check_file "scripts/audit_harness.sh"
check_file "scripts/harness/smoke.sh"
check_file "scripts/harness/test.sh"
check_file "scripts/harness/lint.sh"
check_file "scripts/harness/typecheck.sh"
check_file ".github/workflows/harness.yml"

echo
check_executable "scripts/audit_harness.sh"
check_executable "scripts/harness/smoke.sh"
check_executable "scripts/harness/test.sh"
check_executable "scripts/harness/lint.sh"
check_executable "scripts/harness/typecheck.sh"

echo
check_contains "Makefile" "^-include Makefile\\.harness" "Makefile includes Makefile.harness"
check_contains "Makefile.harness" "^smoke:" "Makefile.harness has smoke target"
check_contains "Makefile.harness" "^test:" "Makefile.harness has test target"
check_contains "Makefile.harness" "^lint:" "Makefile.harness has lint target"
check_contains "Makefile.harness" "^typecheck:" "Makefile.harness has typecheck target"
check_contains "Makefile.harness" "^check:" "Makefile.harness has check target"
check_contains "Makefile.harness" "^ci:" "Makefile.harness has ci target"

echo
check_contains "PLANS.md" "## Active Phase" "PLANS.md records active phase"
check_contains "PLANS.md" "Current Command Surface" "PLANS.md records command surface"
check_contains "PLANS.md" "Current Blockers" "PLANS.md records blockers"
check_contains "PLANS.md" "Command Results" "PLANS.md records command results"
check_contains "docs/ARCHITECTURE.md" "## Boundaries" "ARCHITECTURE.md defines boundaries"
check_contains "docs/ARCHITECTURE.md" "Symphony local orchestration" "ARCHITECTURE.md defines Symphony boundary"
check_contains "docs/OBSERVABILITY.md" "## Required Event Fields" "OBSERVABILITY.md defines required fields"
check_contains "docs/OBSERVABILITY.md" "## Redaction Rules" "OBSERVABILITY.md defines redaction rules"

echo
check_contains ".github/workflows/harness.yml" "make smoke" "harness CI runs make smoke"
check_contains ".github/workflows/harness.yml" "make check" "harness CI runs make check"
check_contains ".github/workflows/harness.yml" "scripts/audit_harness.sh" "harness CI runs audit"

echo
if [[ "$failures" -gt 0 ]]; then
  echo "Harness audit failed: $failures issue(s) detected."
  exit 1
fi

echo "Harness audit passed."
