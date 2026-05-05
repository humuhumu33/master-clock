/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# Rational — Lean 4 mirror of `src/rational.rs`

Phase 3 O1. Layer 4 of the BRIDGE.md eight-layer stack.

The Rust crate's `Rational` is a thin wrapper around `BigRational =
(BigInt, NonZeroBigInt)`. We mirror this in Lean by reusing
`mathlib4`'s `Rat`, which is the same construction up to representation
detail (`Rat` enforces `den > 0` and `Nat.gcd num.natAbs den = 1` by
invariant; the Rust side enforces the same via `num-rational`).

**NO `sorry` POLICY** — every theorem closed by `rfl`, `decide`, or
mathlib appeal.

## UOR primitive binding

| Lean object | UOR ontology IRI |
|---|---|
| `Rat` (mathlib) | `https://uor.foundation/prime-clock/rational/L4` |
| `Rat.num` slot | `https://uor.foundation/prime-clock/rational/num` |
| `Rat.den` slot | `https://uor.foundation/prime-clock/rational/denom` (constrained ≠ 0) |

The denominator's nonzero obligation is the field invariant
`Rat.den_pos`: `∀ q : Rat, 0 < q.den`. This is the load-bearing
inhabitant condition that makes `Rat` a well-formed Layer 4 carrier.
-/

import Mathlib.Data.Rat.Defs
import Mathlib.Algebra.Order.Field.Basic

namespace PrimeFoundations.Rational

/-! ## Type-level mirror

We reuse `Rat` directly. The shape's load-bearing property is that
the denominator is positive (and hence nonzero) for every value. -/

/-- The Layer 4 carrier. -/
abbrev Rational : Type := Rat

/-! ## UOR shape obligations -/

/-- **Theorem.** Every `Rational` has a strictly positive denominator.
This is the inhabitant condition of `RationalDenomShape` from
`src/rational.rs::RationalDenomShape::CONSTRAINTS`
(`bound_shape_iri = ".../rational/nonzero"`). -/
theorem denom_pos (q : Rational) : 0 < q.den := q.den_pos

/-- **Theorem.** The denominator is nonzero for every `Rational`.
Direct corollary of `denom_pos`. -/
theorem denom_nonzero (q : Rational) : q.den ≠ 0 :=
  Nat.pos_iff_ne_zero.mp (denom_pos q)

/-- **Theorem.** The numerator and denominator are coprime — the
canonical-form invariant from `BigRational`. -/
theorem num_den_coprime (q : Rational) : q.num.natAbs.Coprime q.den :=
  q.reduced

/-! ## Field axioms (sanity mirrors of Rust ring tests) -/

/-- **Theorem.** Addition is associative on `Rational`. -/
theorem add_assoc' (a b c : Rational) : (a + b) + c = a + (b + c) :=
  add_assoc a b c

/-- **Theorem.** Addition is commutative on `Rational`. -/
theorem add_comm' (a b : Rational) : a + b = b + a :=
  add_comm a b

/-- **Theorem.** Multiplicative inverse: `a · a⁻¹ = 1` for `a ≠ 0`. -/
theorem mul_inv_cancel' (a : Rational) (ha : a ≠ 0) : a * a⁻¹ = 1 :=
  mul_inv_cancel₀ ha

/-! ## Spot-check fractions matching the Rust test suite -/

/-- **Theorem.** `1/2 + 1/3 = 5/6`, mirrors the Rust `ring_axioms`
spot value. -/
theorem half_plus_third : (1 / 2 : Rat) + (1 / 3 : Rat) = 5 / 6 := by
  norm_num

/-- **Theorem.** `(1/2)^10 = 1/1024`, mirrors Rust `power_is_exact`. -/
theorem half_pow_ten : ((1 / 2 : Rat) ^ (10 : Nat)) = 1 / 1024 := by
  norm_num

/-- **Theorem.** `2/3 < 3/4`, mirrors Rust `comparison_total_order`. -/
theorem two_thirds_lt_three_quarters : (2 / 3 : Rat) < 3 / 4 := by
  norm_num

end PrimeFoundations.Rational
