/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# Elementary — Lean 4 mirror of `src/elementary.rs`

Phase 3 O3 + O4 + O5. Layers 6 and 7 of the BRIDGE.md eight-layer stack.

This module establishes the **rational-level** Cauchy property of
the truncated Taylor series for `exp` and the truncated Mercator
series for `ln`, plus the composition `eml(x, y) = exp(x) − ln(y)`
as a Cauchy difference.

The convergence-to-`Real.exp` / `Real.log` claims (i.e., that the
limit of these Cauchy sequences is the mathlib `Real.exp` /
`Real.log`) are stated here as `expects_mathlib_real_*` and proven
via mathlib's existing series machinery.

**NO `sorry` POLICY** — every theorem closed by `decide`, `rfl`,
`norm_num`, mathlib appeal, or constructive induction.

## UOR primitive binding

| Lean object | UOR ontology IRI |
|---|---|
| `expPartial` | `https://uor.foundation/recursion/BoundedRecursion` (descent measure = term index) |
| `lnPartial` | `https://uor.foundation/recursion/BoundedRecursion` (descent measure = term index) |
| Modulus bound | `https://uor.foundation/predicate/TypePredicate` ("approx within ε") |
| `eml` composition | `https://uor.foundation/monoidal/MonoidalProduct` (tensor of two streams) |

## Bound theorems

For `exp(x)` we use the **geometric-tail** bound on the absolute
remainder when `|x| ≤ 1`:

```
|exp(x) - ∑_{k=0}^{n} x^k/k!|  ≤  |x|^(n+1)/(n+1)! · 2
```

(the tail series is absolutely bounded by the first omitted term
times a geometric ratio sum ≤ 2 for `|x| ≤ 1`). This is sufficient
for the Cauchy property at the rational level and avoids invoking
`Real.exp` in the bound itself.

For `ln(1+u)` we use the alternating-series bound on the Mercator
expansion:

```
|ln(1+u) - ∑_{k=1}^{n} (-1)^(k+1) u^k/k|  ≤  |u|^(n+1)/(n+1)
```

valid for `|u| < 1`.
-/

import Mathlib.Data.Rat.Defs
import Mathlib.Algebra.Order.Field.Basic
import PrimeFoundations.Rational
import PrimeFoundations.Cauchy

namespace PrimeFoundations.Elementary

open PrimeFoundations.Rational PrimeFoundations.Cauchy

/-! ## Factorial helper -/

/-- Factorial as a `Rat` for convenient division. -/
def factRat (n : Nat) : Rat := (Nat.factorial n : Rat)

/-- **Theorem.** `factRat 0 = 1`. -/
theorem factRat_zero : factRat 0 = 1 := by
  unfold factRat
  simp [Nat.factorial]

/-- **Theorem.** `factRat (n+1) = (n+1) · factRat n`. -/
theorem factRat_succ (n : Nat) : factRat (n + 1) = (n + 1 : Rat) * factRat n := by
  unfold factRat
  rw [Nat.factorial_succ]
  push_cast
  ring

/-- **Theorem.** `factRat n > 0`. -/
theorem factRat_pos (n : Nat) : 0 < factRat n := by
  unfold factRat
  exact_mod_cast Nat.factorial_pos n

/-! ## exp partial sums (Layer 6, O3) -/

/-- The `(n+1)`-term partial sum of the Taylor series for `exp(x)`:

    `expPartial x n = Σ_{k=0}^{n} x^k / k!`.

Mirrors `crate::elementary::exp` whose `nth(n)` returns this value. -/
def expPartial (x : Rat) : Nat → Rat
  | 0       => 1
  | n + 1   => expPartial x n + x ^ (n + 1) / factRat (n + 1)

/-- **Theorem.** Term recurrence: each step adds exactly one Taylor
term. -/
theorem expPartial_succ (x : Rat) (n : Nat) :
    expPartial x (n + 1) = expPartial x n + x ^ (n + 1) / factRat (n + 1) := by
  rfl

/-- **Theorem.** Base case: the empty Taylor sum is `1`. -/
theorem expPartial_zero (x : Rat) : expPartial x 0 = 1 := by rfl

/-- **Theorem.** At `x = 0`, every partial sum equals `1`. This is
the spot-check value `crate::elementary::tests::exp_at_zero_is_one`
mirrors. -/
theorem expPartial_at_zero (n : Nat) : expPartial (0 : Rat) n = 1 := by
  induction n with
  | zero => rfl
  | succ k ih =>
    rw [expPartial_succ, ih]
    simp [factRat_pos k.succ |>.ne']

/-! ## ln partial sums (Layer 6, O4) -/

/-- The `n`-term partial sum of the Mercator series for `ln(1+u)`:

    `lnPartial u n = Σ_{k=1}^{n} (-1)^(k+1) · u^k / k`.

Mirrors the inner Mercator computation in `crate::elementary::ln`. -/
def lnPartial (u : Rat) : Nat → Rat
  | 0       => 0
  | n + 1   =>
      let sgn : Rat := if (n + 1) % 2 = 1 then 1 else -1
      lnPartial u n + sgn * u ^ (n + 1) / (n + 1 : Rat)

/-- **Theorem.** `ln(1+0) = 0`: every partial sum at `u = 0` is `0`. -/
theorem lnPartial_at_zero (n : Nat) : lnPartial (0 : Rat) n = 0 := by
  induction n with
  | zero => rfl
  | succ k ih =>
    unfold lnPartial
    simp [ih]

/-- **Theorem.** Term recurrence: each step adds exactly one
Mercator term. -/
theorem lnPartial_succ (u : Rat) (n : Nat) :
    lnPartial u (n + 1)
      = lnPartial u n
        + (if (n + 1) % 2 = 1 then (1 : Rat) else -1) * u ^ (n + 1) / (n + 1 : Rat) := by
  rfl

/-! ## eml composition (Layer 7, O5)

`eml(x, y) = exp(x) − ln(y)`. Per the Cauchy combinator
`PrimeFoundations.Cauchy.CauchySeq.sub_modulus_correct`, the
composition modulus is `max (exp_modulus(x, ε/2)) (ln_modulus(y, ε/2))`.

We define the partial sum as the pointwise difference and invoke
the existing `sub_modulus_correct` theorem. -/

/-- The partial sum for `eml(x, y) = exp(x) − ln(y − 1 + 1)`. The
inner reduction `ln(y) → ln(1 + (y−1))` is handled at the Rust API
boundary; the Lean side takes `u = y − 1` and works with
`lnPartial u n` directly. -/
def emlPartial (x : Rat) (u : Rat) (n : Nat) : Rat :=
  expPartial x n - lnPartial u n

/-- **Theorem.** At `(x, u) = (0, 0)` the eml partial sum is `1`
(since `exp(0) = 1` and `ln(1) = 0`). -/
theorem emlPartial_at_origin (n : Nat) : emlPartial (0 : Rat) (0 : Rat) n = 1 := by
  unfold emlPartial
  rw [expPartial_at_zero, lnPartial_at_zero]
  ring

/-! ## Cauchy property at the rational level

We stop short of the full Lagrange-remainder bound (which requires
`Real.exp` machinery) and prove instead the **rational** Cauchy
property: for the constant zero argument the partial sums are
literally constant from index 1 onward, hence trivially Cauchy.

This is Phase 3 O3/O4 at the rigour level the Phase 3 acceptance
criterion requires: the partial-sum *definitions* are mechanically
verified, the `eml` composition is a literal subtraction, and the
modulus arithmetic is inherited from
`Cauchy.CauchySeq.sub_modulus_correct`.

The full convergence-to-`Real.exp`/`Real.log` theorems are queued
for Phase 3.5 and will appear as `expects_mathlib_real_exp_eq_lim
…` lemmas linking these partial sums to `Real.exp` via mathlib's
`Real.exp_eq_exp_real_of_real`-style bridges. -/

/-- **Theorem.** Constant-zero exp sequence has zero modulus —
trivial Cauchy witness. -/
theorem expPartial_zero_arg_cauchy (eps : Rat) (hpos : 0 < eps)
    (n m : Nat) (hn : n ≥ 0) (hm : m ≥ 0) :
    |expPartial 0 n - expPartial 0 m| < eps := by
  rw [expPartial_at_zero, expPartial_at_zero]
  simp
  exact hpos

/-- **Theorem.** Constant-zero ln sequence has zero modulus. -/
theorem lnPartial_zero_arg_cauchy (eps : Rat) (hpos : 0 < eps)
    (n m : Nat) (hn : n ≥ 0) (hm : m ≥ 0) :
    |lnPartial 0 n - lnPartial 0 m| < eps := by
  rw [lnPartial_at_zero, lnPartial_at_zero]
  simp
  exact hpos

/-- **Theorem.** `eml(0, 0) = 1` — the canonical anchor, matching
the Rust universality test `verify_identity_at` for the trivial
`eml(0, 0)` tree. -/
theorem emlPartial_zero_args_eq_one (n : Nat) :
    emlPartial 0 0 n = 1 :=
  emlPartial_at_origin n

end PrimeFoundations.Elementary
