# Empirical results — captured `cargo test` output

This file is the static evidence companion to the README's empirical
claims. Every number below is reproducible by cloning the repo and
running the listed command on stable Rust 1.83+. The same commands
run automatically in [GitHub Actions](https://github.com/humuhumu33/master-clock/actions);
the residual table also appears live on every CI run's *Summary* tab.

---

## Phase 1+2 — Foundations and Atlas correspondence

```
$ cargo test --lib
running 112 tests
…
test result: ok. 112 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

Includes:

- All 8 `U(30)` residues independently re-derived and matched against
  Gary's stated values.
- All 96 `R(360)` elements verified coprime to 30.
- 96-row Appendix A matrix derived from scratch and checked cell-by-cell
  against the FIXED PDF transcription.
- Both `Σ dr(n) = 432` and `Σ dr(F_n) = 432` invariants — proved by direct
  summation **and** by structural decomposition (`12 × 36`).
- Pisano period mod 9 = 24 verified minimal.
- `F_49 = 7,778,742,049` derived from the Fibonacci recurrence and
  matched against Gary's anchor.
- Atlas-96 mirror involution `τ² = id` verified across all 96 vertices.
- Dirichlet equidistribution at `N = 10⁴` verified against canonical bound.
- New: Ω = `LATTICE_LAYERS · χ_fitted = 290208/10000 = 29.0208` derived
  from Phase 1 constants per Gary's 2026-05-05 follow-up.

## Phase 3 — BRIDGE substrate (alloc-only)

```
$ cargo test --lib --features alloc
running 169 tests
…
test result: ok. 167 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 2.92s
```

Adds the eight-layer construction from `Succ = Neg ∘ BNot` up through
`BigInt`, `ℚ`, `Cauchy<ℚ>`, `exp`, `ln`, and `eml(x, y) = exp(x) − ln(y)`.
Two tests are `#[ignore]`'d because the rational `ln`/`exp` series at
`N = 10¹⁰⁻¹²` precision are slow.

## Single-file integration suite

```
$ cargo test --test validation
running 30 tests
test phase1_two_parameters_only ... ok
test phase1_u30_residues_match_pdf ... ok
test phase1_r360_cardinality_is_96 ... ok
…
test phase4_d_at_zero ... ok
test phase4_d_one_period_equals_432 ... ok
test phase4_d_period_invariant ... ok
…
test phase3_harness_returns_specgap ... ok
test validation_summary ... ok
test result: ok. 30 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

All tests in `tests/validation.rs` use only the public API — exactly
what an outside reviewer can verify. Test names are `phaseN_*`-prefixed
so they sort by phase in the cargo output.

---

## Phase 4 — Live Master-Clock evaluation vs canonical π(N)

This is the headline result. The full Master-Clock formula is evaluated
end-to-end at the three N-points Gary tabulates in his Validated Theorem
PDF, and the residual is reported against the canonical π(N) literature
values from OEIS A006880.

```
$ cargo test --features alloc --test validation -- --ignored --nocapture \
      phase4_master_clock_residual_at_gary_n

════════════════════════════════════════════════════════════════════════════════
   PHASE 4 — Master-Clock LIVE EVALUATION (Ω-normalized)
   π_predicted(N) = (D − TF + RF + 3) / Ω
   χ = 24184/10000   F49 = 7,778,742,049   k = 1.272 × 10⁻¹⁰
   Ω = LATTICE_LAYERS · χ = 290208/10000 = 29.0208
════════════════════════════════════════════════════════════════════════════════
        N   π(N) literature              D(N)         TF(N)   RF(N)       raw (D−TF+RF+3)      π_pred / Ω        residual
    ────────────────────────────────────────────────────────────────────────────────
    10^10       455,052,511    12,000,000,000   179,579,262       3        11,820,420,744     407,308,576     -47,743,935
    10^11     4,118,054,813   120,000,000,000  1,632,538,749       0       118,367,461,254   4,078,711,174     -39,343,639
    10^12    37,607,912,018  1,200,000,000,000  14,964,938,535       0     1,185,035,061,468  40,833,990,154   3,226,078,136
    ────────────────────────────────────────────────────────────────────────────────
   Gary's claimed residuals (Validated_Theorem PDF §II):
       N = 10^10 → ±112    (post-F49 stabilisation)
       N = 10^11 → ±432    (= SUM_DR, Phase 1 invariant)
       N = 10^12 → ±96     (= R_CARDINALITY, Phase 1 invariant)

test phase4_master_clock_residual_at_gary_n ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 30 filtered out; finished in 67.44s
```

### Summary table

| `N`    | π(N) literature | π_predicted (this code) | Master-Clock residual | Gary's claimed residual | Li(N) error |
|--------|-----------------|-------------------------|-----------------------|-------------------------|-------------|
| `10¹⁰` | 455,052,511     | 407,308,576             | **−47,743,935**       | ±112                    | 3,104       |
| `10¹¹` | 4,118,054,813   | 4,078,711,174           | **−39,343,639**       | ±432                    | 11,588      |
| `10¹²` | 37,607,912,018  | 40,833,990,154          | **+3,226,078,136**    | ±96                     | 38,263      |

### Reading the result

With Gary's complete specification honored exactly — `χ = 24184/10000`
(Croft Constant), `D` literal `Fibo_DR` lattice tally, `TF = (N/ln N)/χ`
(Tension Force), `RF` two-piece form with `k = 1.272 × 10⁻¹⁰` (decay
constant), divided by `Ω = LATTICE_LAYERS · χ` (Resonant Volume
normalization) — the residuals are 5 to 8 orders of magnitude larger
than the values Gary cites in the Validated Theorem PDF, and `3,400×`
to `84,000×` worse than the canonical `Li(N)` error at the same `N`.

The structural anchors `432 = SUM_DR` and `96 = R_CARDINALITY` that
appear so cleanly in Gary's table do not emerge from the formula
evaluation at all — they appear nowhere in the computed residuals.

The percentage error sequence is `−10.5%, −1.0%, +8.6%` — monotone,
sign-flipping near `N = 10¹¹`. The formula crosses through `π(N)`
exactly once in the tabulated range, suggesting one missing structural
correction rather than a fundamental disagreement.

The full Q&A trail through which the specification was negotiated is
in [`SPEC_REQUEST.md`](./SPEC_REQUEST.md). The structural-correspondence
verdicts (octonion, triality) are in
[`STRUCTURAL_CORRESPONDENCES.md`](./STRUCTURAL_CORRESPONDENCES.md).
The eight-layer derivation from `Succ = Neg ∘ BNot` is in
[`BRIDGE.md`](./BRIDGE.md).

---

## Lean 4 — zero `sorry` static check

```
$ python3 scripts/lean_no_sorry.py   # also runs as part of scripts/ci.sh
OK — zero `sorry` across 11 Lean files.
```

Every theorem in `lean4/PrimeFoundations/` closes by `rfl`, `decide`,
`norm_num`, `interval_cases`, `linarith`, or constructive proof. The
zero-`sorry` guard runs as part of the GitHub Actions `lean-zero-sorry`
job on every push.

## How to reproduce

```bash
git clone https://github.com/humuhumu33/master-clock.git
cd master-clock
cargo test --lib                                                # 112 default tests
cargo test --lib --features alloc                               # 167 + 2 ignored
cargo test --test validation -- --nocapture validation_summary  # human-readable report
cargo test --features alloc --test validation -- --ignored --nocapture \
    phase4_master_clock_residual_at_gary_n                      # ~70s; live evaluation
```

First build takes 30–60 seconds (the git fetch of `uor-foundation` and
`uor-foundation-sdk` from the [UOR-Framework GitHub repo](https://github.com/UOR-Foundation/UOR-Framework)).
Subsequent runs are sub-second except the `--ignored` evaluation.

If you want to verify the Lean side too:

```bash
cd lean4
lake update
lake exe cache get   # pulls mathlib4 v4.23.0 pre-built cache (~5 min first time)
lake build           # ~2 min after cache fetch; zero sorry
```
