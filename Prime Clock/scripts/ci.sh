#!/usr/bin/env bash
# CI gauntlet for the toroidal-prime-clock crate.
#
# Seven steps:
#   1. cargo check          — clean build
#   2. cargo clippy         — strictest lint posture
#   3. cargo test           — exact-count guard at three feature postures
#                             (default / alloc / ground-truth)
#   4. cargo check --no-default-features  — no_std integrity
#   5. Lean zero-sorry guard  — comment-stripped grep across lean4/
#   6. Verdict consistency   — STRUCTURAL_CORRESPONDENCES.md ↔ source
#                              disclaimers in lockstep
#   7. lake build            — Lean proof rebuild (skipped if lake absent)
#
# Usage:  bash scripts/ci.sh
# Exit:   0 = pass, non-zero = fail

set -euo pipefail

# Test-count guards. One per feature posture. Source-of-truth: a
# `grep -c '^\s*#\[test\]' src/**/*.rs` run partitioned by `#[cfg]`
# attributes plus the lib.rs module-gate flags.
#
#   - Default (no features): the foundational Phase 1/2 modules only.
#   - --features alloc:      adds the BRIDGE Layers 4-7 modules
#                            (cauchy, rational, elementary, categorical).
#   - --features ground-truth: adds the primal-backed sieve module
#                            (ground_truth.rs, 5 default-on tests +
#                            2 #[ignore]'d).
#
# When test counts drift, update this script and the corresponding
# Phase doc in lockstep.
EXPECTED_TEST_COUNT_DEFAULT=112
# `passed` count, excludes `#[ignore]`'d tests (which `running N`
# does include). Cargo prints `X passed; Y failed; Z ignored`; the
# CI guard greps the first `N passed`, so these match `passed`,
# not the `running` total.
EXPECTED_TEST_COUNT_ALLOC=168
EXPECTED_TEST_COUNT_GROUND_TRUTH=173
# The integration test crate at `tests/validation.rs` is the
# single skeptic-facing surface — public API only, walks the full
# Phase 1+2+3 validation surface using the largest available
# population per claim. Counted separately because cargo runs it
# as its own test crate.
EXPECTED_TEST_COUNT_VALIDATION=30
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$CRATE_ROOT"

red()    { printf "\033[31m%s\033[0m\n" "$*"; }
green()  { printf "\033[32m%s\033[0m\n" "$*"; }
yellow() { printf "\033[33m%s\033[0m\n" "$*"; }
header() { printf "\n\033[1m▶ %s\033[0m\n" "$*"; }

failures=0

# ── 1. Build clean ────────────────────────────────────────────────────
header "1/7  cargo check --lib"
if cargo check --lib 2>&1 | tee /tmp/ci-check.log; then
    green "    PASS — clean build."
else
    red   "    FAIL — cargo check returned non-zero."
    failures=$((failures + 1))
fi

# ── 2. Strict lint posture ────────────────────────────────────────────
header "2/7  cargo clippy --lib -- -D warnings"
if cargo clippy --lib -- -D warnings 2>&1 | tee /tmp/ci-clippy.log; then
    green "    PASS — zero warnings."
else
    red   "    FAIL — clippy produced warnings or errors."
    failures=$((failures + 1))
fi

# ── 3. Test sweep with exact-count guards (Probe 4 closure) ────────────
#
# Three feature postures, each with its own exact-count guard. Probe 4
# (drop a test → cargo test still passes with one fewer) is closed at
# every posture: a deletion in any of the four gated modules
# (cauchy/rational/elementary/categorical/ground_truth) flips the
# matching guard, and a deletion in the foundational modules flips the
# default guard.
run_test_posture() {
    local label="$1"
    local expected="$2"
    local feature_args="$3"
    local log="$4"
    # Optional 5th arg: extra cargo args (e.g., `--test validation`).
    local extra_args="${5:-}"
    local cargo_target="--lib"
    if [ -n "$extra_args" ]; then
        cargo_target="$extra_args"
    fi

    header "    posture: ${label}  (cargo test ${cargo_target} ${feature_args})"
    # shellcheck disable=SC2086
    local out
    out="$(cargo test ${cargo_target} ${feature_args} 2>&1)"
    echo "$out" | tee "$log" >/dev/null
    echo "$out" | tail -5

    local actual
    actual="$(echo "$out" | grep -oE '[0-9]+ passed' | head -1 | grep -oE '[0-9]+' || true)"
    if [ -z "${actual:-}" ]; then
        red "        FAIL — could not parse 'N passed' from cargo test output."
        failures=$((failures + 1))
    elif [ "$actual" -ne "$expected" ]; then
        red "        FAIL — expected $expected tests, found $actual."
        red "        A test was added or deleted. Update the matching"
        red "        EXPECTED_TEST_COUNT_* in this script intentionally."
        failures=$((failures + 1))
    else
        green "        PASS — exactly $expected tests passed."
    fi

    if echo "$out" | grep -qE '[1-9][0-9]* failed'; then
        red "        FAIL — at least one test failed."
        failures=$((failures + 1))
    fi
}

header "3/7  cargo test  (unit + integration, exact-count guards)"
run_test_posture "default"      "$EXPECTED_TEST_COUNT_DEFAULT"      ""                          /tmp/ci-test-default.log
run_test_posture "alloc"        "$EXPECTED_TEST_COUNT_ALLOC"        "--features alloc"          /tmp/ci-test-alloc.log
run_test_posture "ground-truth" "$EXPECTED_TEST_COUNT_GROUND_TRUTH" "--features ground-truth"   /tmp/ci-test-ground-truth.log
run_test_posture "validation"   "$EXPECTED_TEST_COUNT_VALIDATION"   ""                          /tmp/ci-test-validation.log "--test validation"

# ── 4. no_std integrity ───────────────────────────────────────────────
header "4/7  cargo check --lib --no-default-features  (no_std build)"
if cargo check --lib --no-default-features 2>&1 | tail -10; then
    green "    PASS — crate builds cleanly under #![no_std]."
else
    red   "    FAIL — no_std build broken."
    failures=$((failures + 1))
fi

# ── 5. Lean zero-sorry guard (Probe 6 closure) ────────────────────────
header "5/7  Lean zero-sorry guard"
if [ -d "lean4" ]; then
    # Find every `sorry` token, exclude lines inside `--` line comments
    # and inside /-…-/ block comments. Lean 4 permits NESTED block
    # comments (`/- /- … -/ … -/`), so a non-greedy regex is unsafe.
    # We use a stack-based depth counter that replaces commented bytes
    # with spaces (preserving newlines so reported line numbers stay
    # aligned with the original file).
    python3 - <<'PYEOF' || sorry_check_failed=$?
import re, sys, pathlib

def strip_lean_comments(text: str) -> str:
    """
    Replace Lean 4 /-…-/ block comments (depth-counted) and -- line
    comments with spaces; preserve newlines so line numbers match the
    original. Handles nesting: `/- /- … -/ … -/` strips correctly.
    Also handles `/-! … -/` doc blocks (same syntax, deeper nesting safe).
    """
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
                # Line comment: blank to end of line, keep the newline.
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
            # Read original line for the report.
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
        red   "    FAIL — `sorry` found in Lean proof bodies."
        failures=$((failures + 1))
    else
        green "    PASS — zero `sorry` outside comments."
    fi
else
    yellow "    SKIP — no lean4/ directory."
fi

# ── 6. Verdict-consistency guard (Sabotage 5 closure) ─────────────────
#
# The Phase 2 O3 verdicts in `STRUCTURAL_CORRESPONDENCES.md` must
# match the disclaimer language in source comments. If the verdict
# document is updated (e.g., octonion → "proven structural") without a
# matching source-comment update, downstream readers see contradictory
# claims. This step greps for verdict markers in both places and fails
# if either the document or the corresponding source disclaimer is
# missing the expected phrase.
#
# When a verdict legitimately flips, update both the doc and the
# source comments in the same commit, then update the regexes below
# in lockstep.
header "6/7  STRUCTURAL_CORRESPONDENCES.md ↔ source-disclaimer consistency"
verdict_failed=0

check_grep() {
    local file="$1"
    local pattern="$2"
    local label="$3"
    # Collapse newlines and `//!` rustdoc continuation prefixes into a
    # single space before grepping. Source disclaimers wrap across
    # multiple `//!` lines, so a per-line grep would miss line-broken
    # phrases like `**classified as\n//!   coincidence**`.
    if sed 's|^[[:space:]]*//![[:space:]]*||' "$file" \
        | tr '\n' ' ' \
        | grep -qiE "$pattern"; then
        green "    PASS — $label found in $file"
    else
        red   "    FAIL — $label MISSING in $file (expected pattern: $pattern)"
        verdict_failed=1
    fi
}

# STRUCTURAL_CORRESPONDENCES.md must list both verdicts.
check_grep STRUCTURAL_CORRESPONDENCES.md "classified[- ]as[- ]coincidence" "octonion verdict"
check_grep STRUCTURAL_CORRESPONDENCES.md "cardinality[- ]only"             "triad verdict"

# Source disclaimers must echo the same verdicts.
check_grep src/lib.rs            "classified as coincidence"   "lib.rs octonion disclaimer"
check_grep src/lib.rs            "cardinality[- ]only"         "lib.rs triad disclaimer"
check_grep src/ring_substrate.rs "classified as coincidence"   "ring_substrate.rs octonion disclaimer"
check_grep src/ring_substrate.rs "cardinality[- ]only"         "ring_substrate.rs triad disclaimer"
check_grep src/r360.rs           "cardinality[- ]only"         "r360.rs triad disclaimer"

if [ "$verdict_failed" -ne 0 ]; then
    red "    FAIL — verdict drift detected. Re-sync STRUCTURAL_CORRESPONDENCES.md"
    red "           and the source disclaimer comments in lockstep."
    failures=$((failures + 1))
else
    green "    PASS — verdict markers present in both doc and source."
fi

# ── 7. Lean lake build (optional — runs if lake is available) ─────────
header "7/7  lake build (lean4/)"
if command -v lake >/dev/null 2>&1 && [ -d "lean4" ]; then
    pushd lean4 >/dev/null
    if lake build 2>&1 | tee /tmp/ci-lake.log; then
        green "    PASS — lake build clean."
    else
        red   "    FAIL — lake build errored."
        failures=$((failures + 1))
    fi
    popd >/dev/null
else
    yellow "    SKIP — lake not on PATH (or no lean4/ dir). Install Lean 4 toolchain to enable."
fi

# ── Final verdict ─────────────────────────────────────────────────────
echo
if [ "$failures" -eq 0 ]; then
    green "════════════════════════════════════════════"
    green "  CI GAUNTLET PASS — Phase 1 audit closed."
    green "════════════════════════════════════════════"
    echo
    echo "Transcripts saved to /tmp/ci-*.log. Attach these to the"
    echo "stress-test report to satisfy Section J deliverables 1-3."
    exit 0
else
    red "════════════════════════════════════════════"
    red "  CI GAUNTLET FAIL — $failures step(s) failed."
    red "════════════════════════════════════════════"
    exit 1
fi
