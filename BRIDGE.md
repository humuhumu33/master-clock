# BRIDGE.md — discrete to continuous, all rooted in `Succ = Neg ∘ BNot`

This document fixes the architectural bridge between the byte-level
ring axiom of `ring_substrate.rs` and the continuous primitives Gary's
full Master-Clock model invokes (`ln`, `exp`). The bridge is a stack
of eight layers, each derived from the previous, all closing back on
the single identity `Succ = Neg ∘ BNot` from
`uor_foundation::enforcement::UnaryRingOp`.

It is documented now so that when Phase 3 makes `D`, `TF`, `RF`
computable, the path from axiom to elementary function is already
laid out.

## The eight-layer bridge

| Layer | Object | UOR primitive | Status |
|-------|--------|--------------|--------|
| 0 | `Succ = Neg ∘ BNot` (the axiom) | `enforcement::UnaryRingOp<L>` | done |
| 1 | `Succ_L` at Witt level `L` | `enforcement::Succ<W8/W16/W32>` | done |
| 2 | Integer arithmetic on `ℤ/2^n` (add, sub, mul, mod, gcd) | derived in `arithmetic.rs` from Layer 1 | done |
| 3 | `ℤ` at arbitrary precision | `enforcement::Embed<L1, L2>` widening chain (or `num-bigint`) | pending |
| 4 | `ℚ` (rationals) | `cartesian_product_shape!(Rational, BigInt, NonZeroBigInt)` | pending |
| 5 | Computable `ℝ` (Cauchy sequences of `ℚ`) | analytic — corresponds to `convergence::L0_State` | pending |
| 6 | `exp`, `ln` on `ℝ` via Taylor series of `ℚ`-terms | rational `Stream` with explicit modulus of convergence | pending |
| 7 | `eml(x, y) = exp(x) − ln(y)` (the paper's primitive) | composition of Layer 6 ops | pending |

## Diagram

```
Layer 0:   neg(bnot(x)) = x + 1                 ← UOR axiom
                  │
                  │  iterate (Succ ∘ Succ ∘ … = +)
                  ▼
Layer 1:   Succ_L : W_L → W_L                   ← UOR UnaryRingOp
                  │
                  │  iterate, dualise (BNot ∘ Succ ∘ BNot = Pred)
                  ▼
Layer 2:   add, sub, mul, mod, gcd on ℤ/2ⁿ      ← arithmetic.rs (verified)
                  │
                  │  embed (W8 → W16 → W32 → … → W∞)
                  ▼
Layer 3:   ℤ at arbitrary precision             ← UOR enforcement::Embed
                  │
                  │  cartesian product with constraint denom ≠ 0
                  ▼
Layer 4:   ℚ                                    ← cartesian_product_shape!
                  │
                  │  Cauchy sequence with modulus of convergence
                  ▼
Layer 5:   Computable ℝ                         ← convergence::L0_State
                  │
                  │  truncated Taylor series, ε-bounded
                  ▼
Layer 6:   exp, ln                              ← rational Stream + bound
                  │
                  │  composition: exp − ln
                  ▼
Layer 7:   eml(x, y) = exp(x) − ln(y)           ← Odrzywołek arXiv:2603.21852
```

## Convergence ladder (algebraic side, already in UOR)

UOR's `convergence` namespace encodes the Cayley-Dickson ladder
extending Layer 5 algebraically. Each step doubles the algebra
dimension and preserves the division-algebra property until
associativity is lost at `𝕆`:

| Level | Algebra | Dim | Identity | Fiber |
|-------|---------|-----|----------|-------|
| `L0_State` | `ℝ` | 1 | identity | `hopf_S0` |
| `L1_Memory` | `ℂ` | 2 | feedback | `hopf_S1` |
| `L2_Agency` | `ℍ` | 4 | choice | `hopf_S3` |
| `L3_Self` | `𝕆` | 8 | self-reference | `hopf_S7` |

The Cayley-Dickson doubling is encoded as
`https://uor.foundation/division/CayleyDicksonConstruction` with
`cayleyDicksonTarget` properties on the levels.

The numerical match `dim(𝕆) = 8 = |U(30)|` (which appears in our
`u30.rs`) lives at the top of this ladder; whether the match is a
structural correspondence is a Phase 2+ research question.

## What's missing — the analytic construction (Phase 3)

UOR provides Layers 0–3 (ring axiom + Witt levels + Embed) and the
algebraic identities for Layers 5+ (convergence ladder + division
algebras). What is **not** provided is the analytic construction
that turns rational arithmetic into computable real arithmetic. This
is the load-bearing missing piece. Three modules cover it:

### `src/rational.rs` (Phase 3)

`Rational` as `cartesian_product_shape!(Rational, BigInt, NonZeroBigInt)`.
Public API:

```rust
pub struct Rational { num: BigInt, denom: NonZeroBigInt }
impl Rational {
    pub fn add(self, other: Self) -> Self { /* num·other.denom + other.num·denom over denom·other.denom */ }
    pub fn sub(self, other: Self) -> Self { /* … */ }
    pub fn mul(self, other: Self) -> Self { /* … */ }
    pub fn div(self, other: NonZero<Self>) -> Self { /* … */ }
    pub fn cmp(self, other: Self) -> Ordering { /* cross-multiply */ }
}
```

Every operation derives from integer arithmetic in `arithmetic.rs`,
which derives from `Succ` in `ring_substrate.rs`.

### `src/cauchy.rs` (Phase 3)

`Cauchy<Rational>` is a pair `(stream, modulus)` where `stream :
ℕ → ℚ` and `modulus : ε ↦ N(ε)` such that
`∀ n, m ≥ N(ε), |x_n − x_m| < ε`.

Public API:

```rust
pub trait CauchySequence {
    fn nth(&self, n: u64) -> Rational;
    fn modulus(&self, eps: Rational) -> u64;
}
pub fn approx<C: CauchySequence>(c: &C, eps: Rational) -> Rational;
```

This module **is** UOR's `convergence::L0_State` made constructive:
the algebraic identity "ℝ has dimension 1 and characteristic identity
'identity'" is realised as a concrete Cauchy-sequence-of-rationals
type with computable convergence.

### `src/elementary.rs` (Phase 3)

`exp`, `ln`, `eml` as functions producing `Cauchy<Rational>`.

```rust
pub fn exp(x: Rational) -> impl CauchySequence {
    // Truncated Taylor: e^x ≈ Σ_{k=0}^N x^k / k!
    // Error bound: |e^x − partial| ≤ |x|^{N+1}/(N+1)! · e^|x|
    // → modulus(ε) := smallest N such that bound < ε
}

pub fn ln(x: Rational) -> impl CauchySequence {
    // Truncated Mercator: ln(1+u) = Σ (-1)^{k+1} u^k / k for |u| < 1
    // Reduce general x to (1+u) via integer log₂
}

pub fn eml(x: Rational, y: Rational) -> impl CauchySequence {
    // Pointwise difference of exp(x) and ln(y) sequences.
    // Modulus(ε) := max(modulus_exp(ε/2), modulus_ln(ε/2)).
}
```

Every term in every series is a `Rational`; every approximation has
an explicit, provable error bound; the entire computation closes back
on `Succ = Neg ∘ BNot` through the eight-layer stack.

## Closure

Bottom-up, every higher operation is a finite construction over the
previous layer. No magic numbers, no floating point, no axiomatic
gaps. The bridge is unidirectional: discrete → continuous, with rigor
preserved at every step. Higher-level code (Master-Clock formula,
Riemann zeros, twin-prime channels, anything Gary or anyone else
asks of the model) plugs in at Layer 7 and inherits the entire
derivation chain.

## Phase ordering

- **Phase 1 (current):** Layers 0–2 implemented and verified. Layers
  3–7 are documented here only.
- **Phase 2:** Atlas correspondence, Dirichlet equidistribution,
  Lean 4 proofs of foundational identities. Stays within Layer 2.
- **Phase 3:** Implement Layers 3–7 in the order listed. Required
  before `D(N)`, `TF(N)`, `RF(N)` can be computed in exact arithmetic.
- **Phase 4+:** Apply the now-rigorous `eml` to Gary's RF formula.
  Test the Master-Clock formula against `π(N)` ground truth at
  `N ∈ {10¹⁰, 10¹¹, 10¹²}`. The headline claim becomes testable.
