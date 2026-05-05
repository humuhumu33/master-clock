# Toroidal Prime-Counting Master-Clock

**A reproducible Rust + Lean 4 implementation of Gary William Croft's
*Toroidal Prime Counting Master-Clock* model, anchored in the UOR
(Universal Object Reference) Framework.**

The headline claim under test: `π(N) = D(N) − TF(N) + RF(N) + 3` and
"more accurate than `Li(N)`" at `N = 10¹², 10¹¹, 10¹⁰`. Every constant
Gary specifies is honored exactly. Every Phase 1 derivation is
machine-checked. The Master-Clock identity is evaluated end-to-end
against canonical literature `π(N)` and `Li(N)` values.

---

## Show — the empirical result

```
$ cargo test --lib
running 112 tests
…
test result: ok. 112 passed; 0 failed; 0 ignored

$ cargo test --features alloc --test validation -- --ignored \
      --nocapture phase4_master_clock_residual_at_gary_n
```

```text
════════════════════════════════════════════════════════════════
   PHASE 4 — Master-Clock LIVE EVALUATION (Ω-normalized)
   π_predicted(N) = (D − TF + RF + 3) / Ω
   χ = 24184/10000   F49 = 7,778,742,049   k = 1.272 × 10⁻¹⁰
   Ω = LATTICE_LAYERS · χ = 290208/10000 = 29.0208
════════════════════════════════════════════════════════════════
   N      π(N)            π_pred / Ω         residual         Gary's claim
   10^10  455,052,511     407,308,576        −47,743,935      ±112
   10^11  4,118,054,813   4,078,711,174      −39,343,639      ±432
   10^12  37,607,912,018  40,833,990,154     +3,226,078,136   ±96
```

| `N`    | π(N) literature | Master-Clock residual (this code) | Gary's claimed residual | Li(N) error |
|--------|-----------------|-----------------------------------|-------------------------|-------------|
| `10¹⁰` | 455,052,511     | **−47,743,935**                   | ±112                    | 3,104       |
| `10¹¹` | 4,118,054,813   | **−39,343,639**                   | ±432                    | 11,588      |
| `10¹²` | 37,607,912,018  | **+3,226,078,136**                | ±96                     | 38,263      |

These numbers are produced from Gary's complete specification — every
constant `χ = 24184/10000`, `F₄₉ = 7,778,742,049`, `k = 1.272·10⁻¹⁰`,
`Ω = 12 · χ` — with no curve-fitting and no interpretation. They are
reproducible on any machine running stable Rust 1.83+.

**What the table shows.** With the formula as Gary specified, the
prediction is 5 to 8 orders of magnitude off his claimed precision,
and `3,400×` to `84,000×` worse than `Li(N)` at the same `N`. The
"more accurate than `Li(N)`" headline is, with the literal formula,
empirically inverted. Discussion of possible missing structural
corrections is in [`SPEC_REQUEST.md`](./SPEC_REQUEST.md).

---

## Tell

### Why

Gary William Croft's model proposes a deterministic prime-counting
identity built on a 96-room toroidal residue lattice and the
432-invariant. The model has been described in three documents (FIXED
PDF, Validated Theorem PDF, Twin-Prime Conjecture PDF) and refined
through chat correspondence. **The question this repo answers is: does
the formula, faithfully implemented, produce the claimed accuracy?**

The answer matters because the model — if it works — would replace
probabilistic prime-counting heuristics with a closed-form arithmetic
identity. If it doesn't, the gap between prediction and reality is
itself informative. Either outcome belongs in the literature, but
only if the implementation is rigorous, reproducible, and free of
interpretation. That is what this repo provides.

### How

Three computational phases, each independently auditable:

**Phase 1 — Foundations.** Two parameters (`MODULUS_U = 30`,
`NUMERAL_BASE = 10`) generate everything else: the 8 elements of
`U(30)`, the 96 elements of `R(360) = U(30) × Z/12`, the digital-root
function, the Pisano period mod 9, the Fibonacci digital roots, the
two `432` invariants, and the F₄₉ ratchet anchor. Every value Gary
states in the FIXED PDF Appendix A is independently re-derived and
cross-checked at compile time.

**Phase 2 — Mathematical Deepening.** The Atlas-96 mirror
correspondence, Dirichlet equidistribution at `N = 10⁴`, structural
verdicts on the octonion and triality correspondences. Every Phase 2
fact has a Lean 4 theorem that closes by `decide`, `rfl`, or
`norm_num` — zero `sorry`.

**Phase 3 — BRIDGE substrate.** The eight-layer construction from
Phase 1's discrete `Succ = Neg ∘ BNot` (UOR's `UnaryRingOp`) up
through `BigInt`, `ℚ`, `Cauchy<ℚ>` (computable `ℝ`), `exp`, `ln`, and
finally Odrzywołek's `eml(x, y) = exp(x) − ln(y)`. This is the
analytic stack `RF(N)` and the Master-Clock comparison need.

**Phase 4 — Master-Clock evaluation.** `D(N)`, `TF(N)`, `RF(N)`
implemented per Gary's stated formulas; harness reports the residual
against canonical `π(N)` literature.

### What framework — UOR primitive bindings

Every layer of the construction is bound to a specific term in the
UOR ontology, verifiable in
[`UOR-Framework/spec/src/namespaces/`](https://github.com/UOR-Foundation/UOR-Framework):

| Construct | UOR ontology term |
|-----------|-------------------|
| `Succ = Neg ∘ BNot` (the axiom) | `enforcement::UnaryRingOp` |
| `U(30)` shape | `pipeline::ConstrainedTypeShape` (8 leaves, `coproduct_shape!`) |
| `R(360)` shape | `pipeline::CartesianProductShape` (`U30 × Z12`) |
| `Cauchy<ℚ>` real-number realisation | `convergence::L0_State` |
| Cauchy term sequence | `stream::ProductiveStream` |
| Modulus of convergence | `stream::EpochBoundary` |
| `D(N)` lattice tally | `recursion::BoundedRecursion` |
| Per-room weight lookup | `query::CoordinateQuery` |

Each IRI in the table is grep-checkable against the framework spec.

---

## Quick start

You need: stable Rust 1.83+ and a network connection (the first
`cargo test` fetches `uor-foundation` and `uor-foundation-sdk` from
GitHub — about 30–60 seconds; subsequent builds are sub-second).

```bash
git clone https://github.com/<your-org>/<repo-name>.git
cd <repo-name>
cargo test --lib                          # 112 default tests
cargo test --lib --features alloc         # 168 with BRIDGE substrate
cargo test --test validation -- --nocapture validation_summary
```

The third command prints a human-readable validation report:
the 96-room lattice rendered as ASCII art, every Phase 1 invariant
with its computed value, the empirical reference table, and the
Phase 4 residual.

For the slow live Master-Clock evaluation (rational `exp` at large
argument; ~70 seconds):

```bash
cargo test --features alloc --test validation -- --ignored --nocapture \
    phase4_master_clock_residual_at_gary_n
```

For the Lean 4 proofs (optional; needs Lean toolchain):

```bash
cd lean4
lake build
```

Zero `sorry` — every theorem closes mechanically.

---

## Audit posture

**Every numeric literal is one of three kinds, classified explicitly:**

1. **Parameter** — irreducible input (`MODULUS_U = 30`,
   `NUMERAL_BASE = 10`, plus Gary-stated constants `CHI_FITTED`,
   `F49_STATED`, `K_NUMERATOR`).
2. **Derived** — computed at compile time from parameters via
   `arithmetic::gcd`, `lcm`, `totient`, `largest_power_below`. Examples:
   `LATTICE_LAYERS`, `MODULUS_R`, `R_CARDINALITY`, `RESIDUES`,
   `RF_LOG_SCALE`, `K_DENOMINATOR`, `OMEGA`.
3. **Verification target** — Gary-stated values from the FIXED PDF,
   used only as compile-time cross-checks against derived values
   (e.g. `GARY_STATED_RESIDUES`, `GARY_STATED_TOTAL_FIBO_DR`,
   `FIXED_PDF_VERIFICATION`).

If any derivation ever drifts from Gary's stated value, the build
fails. There are no magic numbers in production paths.

**Every UOR primitive IRI in the source is verified against the
framework spec.** Run `grep "https://uor.foundation/" src/` and check
each IRI exists in `UOR-Framework/spec/src/namespaces/`.

**Every Lean theorem closes mechanically — zero `sorry`.** The
`scripts/ci.sh` gauntlet enforces this with a comment-stripped
recursive grep across `lean4/`.

---

## Project map

```
.
├── Cargo.toml             — git deps to UOR-Framework, no local paths
├── README.md              — this file
├── LICENSE                — Apache-2.0
│
├── BRIDGE.md              — eight-layer construction, Succ → eml
├── SPEC_REQUEST.md        — historical Q&A with Gary; full record of
│                            specifications resolved + open questions
├── STRUCTURAL_CORRESPONDENCES.md — Phase 2 O3 verdicts on octonion
│                                   and triality correspondences
│
├── src/                   — 17 Rust modules
│   ├── parameters.rs      — irreducible parameters + derivation chain
│   ├── arithmetic.rs      — gcd, lcm, totient, largest_power_below
│   ├── ring_substrate.rs  — Succ = Neg ∘ BNot anchor (UOR axiom)
│   ├── digital_root.rs    — dr(n) = 1 + (n−1) mod 9
│   ├── fibonacci.rs       — fib, dr_fib, F₄₉ verification
│   ├── u30.rs             — 8-element reduced residue system + UOR shape
│   ├── r360.rs            — 96-element lattice + UOR shape
│   ├── appendix_a.rs      — 96-row matrix + Σ dr = Σ dr(F) = 432
│   ├── atlas_correspondence.rs — Atlas-96 mirror, R(360) ↔ Resonance-96
│   ├── dirichlet.rs       — sieve, equidistribution at N = 10⁴
│   ├── li.rs              — π(N) and Li(N) reference table at 9 powers
│   ├── master_clock.rs    — formula traits + χ, F₄₉, Ω constants
│   ├── d_function.rs      — D(N) lattice tally per Gary v1.1
│   ├── tf_function.rs     — TF(N) = (N/ln N)/χ per Tech Reconciliation
│   ├── rf_function.rs     — RF(N) two-piece with k = 1.272·10⁻¹⁰
│   ├── harness.rs         — Master-Clock evaluation harness
│   ├── ground_truth.rs    — primal-backed π(N) at N ≤ 10⁹
│   ├── rational.rs        — Layer 4 ℚ + UOR shape (alloc-only)
│   ├── cauchy.rs          — Layer 5 Cauchy<ℚ> + UOR shape (alloc-only)
│   ├── elementary.rs      — Layer 6+7 exp, ln, eml (alloc-only)
│   └── categorical/       — Phase 3 universality grammar (alloc-only)
│
├── tests/
│   └── validation.rs      — single-file integration suite, 30 tests
│                            + the live Master-Clock evaluation
│
├── lean4/PrimeFoundations/ — 11 Lean 4 modules, zero `sorry`
│   ├── Parameters.lean, Fibonacci.lean, U30.lean, R360.lean
│   ├── AppendixA.lean, AtlasCorrespondence.lean
│   ├── Rational.lean, Cauchy.lean, Elementary.lean
│   ├── Li.lean, DFunction.lean
│
└── scripts/ci.sh          — full CI gauntlet (counts tests, checks
                              `sorry`, verifies UOR/source consistency)
```

---

## Discipline

- `#![no_std]` core; allocator-using paths gated behind the `alloc`
  feature.
- `#![forbid(unsafe_code)]`.
- `clippy::float_arithmetic = "deny"` — every constant exact rational
  or integer; `Cauchy<Rational>` for analytic primitives.
- All clippy warnings are errors. CI runs `cargo clippy --all-targets
  -- -D warnings`.
- Every Lean theorem closes by `rfl`, `decide`, `norm_num`,
  `interval_cases`, `linarith`, or constructive proof. **Zero
  `sorry`.**

---

## License

Apache-2.0. See [`LICENSE`](./LICENSE).

## Credits

The Toroidal Prime-Counting Master-Clock model is the work of
**Gary William Croft** (PrimesDemystified.com). This repository is a
mechanical implementation of his specification, built and audited by
the UOR Foundation engineering team. All claims about the model's
accuracy in this README are derived empirically from the cargo test
output, not asserted by us. See
[`SPEC_REQUEST.md`](./SPEC_REQUEST.md) for the complete Q&A record.
