#!/usr/bin/env bash
# CI gauntlet for the Prime Clock crate.
#
# Six steps:
#   1. cargo check                       — clean build under default features
#   2. cargo check --no-default-features — no_std integrity
#   3. cargo clippy -- -D warnings       — strictest lint posture
#   4. cargo test                        — unit tests at every feature posture
#   5. standalone validators             — six single-file binaries
#   6. Lean zero-sorry guard             — comment-stripped grep across lean4/
#
# Usage:  bash scripts/ci.sh
# Exit:   0 = pass, non-zero = fail

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$CRATE_ROOT"

red()    { printf "\033[31m%s\033[0m\n" "$*"; }
green()  { printf "\033[32m%s\033[0m\n" "$*"; }
yellow() { printf "\033[33m%s\033[0m\n" "$*"; }
header() { printf "\n\033[1m▶ %s\033[0m\n" "$*"; }

failures=0

# ── 1. Build clean ────────────────────────────────────────────────────
header "1/6  cargo check --lib"
if cargo check --lib 2>&1 | tee /tmp/ci-check.log; then
    green "    PASS — clean build."
else
    red   "    FAIL — cargo check returned non-zero."
    failures=$((failures + 1))
fi

# ── 2. no_std integrity ───────────────────────────────────────────────
header "2/6  cargo check --lib --no-default-features  (no_std build)"
if cargo check --lib --no-default-features 2>&1 | tail -10; then
    green "    PASS — crate builds cleanly under #![no_std]."
else
    red   "    FAIL — no_std build broken."
    failures=$((failures + 1))
fi

# ── 3. Strict lint posture ────────────────────────────────────────────
header "3/6  cargo clippy --lib --features alloc -- -D warnings"
if cargo clippy --lib --features alloc -- -D warnings 2>&1 | tee /tmp/ci-clippy.log; then
    green "    PASS — zero warnings."
else
    red   "    FAIL — clippy produced warnings or errors."
    failures=$((failures + 1))
fi

# ── 4. Test sweep at every feature posture ────────────────────────────
header "4/6  cargo test  (default / alloc / ground-truth, plus integration)"
run_test_posture() {
    local label="$1"
    local feature_args="$2"
    local extra_args="${3:-}"

    header "    posture: ${label}  (cargo test ${feature_args} ${extra_args})"
    # shellcheck disable=SC2086
    if cargo test ${extra_args} ${feature_args} 2>&1 | tail -5; then
        green "        PASS — ${label}"
    else
        red   "        FAIL — ${label}"
        failures=$((failures + 1))
    fi
}

run_test_posture "default --lib"       ""                       "--lib"
run_test_posture "alloc --lib"         "--features alloc"       "--lib"
run_test_posture "validation"          "--features alloc"       "--test validation"
run_test_posture "cross_phase"         "--features alloc"       "--test cross_phase_invariants"
run_test_posture "triality_witnesses"  ""                       "--test triality_witnesses"

# ── 5. Standalone validators ──────────────────────────────────────────
header "5/6  standalone validators (six single-file Rust binaries)"
TMP_VAL="$(mktemp -d)"
trap 'rm -rf "$TMP_VAL"' EXIT
for v in validate_sieve validate_units_z9 verify_twin_rigor \
         verify_phase3_batch verify_phase4_batch verify_phase5_batch; do
    if [ ! -f "$v.rs" ]; then
        yellow "    SKIP — $v.rs not present."
        continue
    fi
    cp "$v.rs" "$TMP_VAL/"
    if (cd "$TMP_VAL" && rustc -O "$v.rs" -o "$v" 2>&1 | tail -3 && ./"$v" >/dev/null); then
        green "    PASS — $v"
    else
        red   "    FAIL — $v"
        failures=$((failures + 1))
    fi
done

# ── 6. Lean zero-sorry guard ──────────────────────────────────────────
header "6/6  Lean zero-sorry guard"
if [ -d "lean4" ]; then
    python3 - <<'PYEOF' || sorry_check_failed=$?
import re, sys, pathlib

def strip_lean_comments(text: str) -> str:
    out = list(text)
    n = len(out)
    i = 0
    depth = 0
    while i < n:
        c = out[i]
        nxt = out[i + 1] if i + 1 < n else ''
        if depth == 0:
            if c == '/' and nxt == '-':
                depth = 1
                out[i] = ' '; out[i + 1] = ' '
                i += 2
            elif c == '-' and nxt == '-':
                while i < n and out[i] != '\n':
                    out[i] = ' '
                    i += 1
            else:
                i += 1
        else:
            if c == '/' and nxt == '-':
                depth += 1
                out[i] = ' '; out[i + 1] = ' '
                i += 2
            elif c == '-' and nxt == '/':
                depth -= 1
                out[i] = ' '; out[i + 1] = ' '
                i += 2
            elif c == '\n':
                i += 1
            else:
                out[i] = ' '
                i += 1
    return ''.join(out)

hits = []
for path in pathlib.Path("lean4").rglob("*.lean"):
    text = path.read_text(encoding="utf-8")
    stripped = strip_lean_comments(text)
    for lineno, line in enumerate(stripped.splitlines(), start=1):
        if re.search(r"\bsorry\b", line):
            orig = text.splitlines()[lineno - 1] if lineno - 1 < len(text.splitlines()) else line
            hits.append((path, lineno, orig.rstrip()))
if hits:
    print(f"    {len(hits)} `sorry` token(s) found in proof bodies:")
    for path, lineno, line in hits:
        print(f"      {path}:{lineno}: {line}")
    sys.exit(1)
print("    No `sorry` tokens in proof bodies.")
PYEOF
    if [ "${sorry_check_failed:-0}" -ne 0 ]; then
        red   "    FAIL — sorry found in Lean proof bodies."
        failures=$((failures + 1))
    else
        green "    PASS — zero sorry outside comments."
    fi
else
    yellow "    SKIP — no lean4/ directory."
fi

# ── Final verdict ─────────────────────────────────────────────────────
echo
if [ "$failures" -eq 0 ]; then
    green "════════════════════════════════════════════"
    green "  CI GAUNTLET PASS"
    green "════════════════════════════════════════════"
    exit 0
else
    red "════════════════════════════════════════════"
    red "  CI GAUNTLET FAIL — $failures step(s) failed."
    red "════════════════════════════════════════════"
    exit 1
fi
