/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# Cauchy — Lean 4 mirror of `src/cauchy.rs`

Phase 3 O2. Layer 5 of the BRIDGE.md eight-layer stack.

A computable real number is encoded as a pair `(nth, modulus)` where
`nth : ℕ → ℚ` is the term sequence and `modulus : ℚ⁺ → ℕ` returns
an index past which the sequence stays within ε of itself. This is
the constructive realisation of `convergence::L0_State` — UOR's
Level-0 (real) algebra position.

**NO `sorry` POLICY** — every theorem closed by `rfl`, `decide`,
`simp`, `linarith`, or constructive proof.

## UOR primitive binding

| Lean object | UOR ontology IRI |
|---|---|
| `CauchySeq` (this file) | `https://uor.foundation/prime-clock/cauchy/L0_State` |
| `CauchySeq.nth` | `https://uor.foundation/stream/ProductiveStream` |
| `CauchySeq.modulus` | `https://uor.foundation/stream/EpochBoundary` |

The pair (term-stream, modulus) realises the `(productive, descent)`
contract: every epoch of the stream terminates, and the descent
measure (the modulus index) decreases monotonically with ε → 0.

## Relationship to mathlib's `CauSeq`

Mathlib has `CauSeq` (Cauchy sequences in a normed space). The
encoding here pins the **modulus** explicitly, which mathlib's
`CauSeq` derives from a propositional witness. The two are
inter-convertible: any `CauchySeq` here yields a `CauSeq` by
existential-introduction on the modulus, and any `CauSeq` whose
witness is constructively recoverable yields a `CauchySeq` here.
-/

import Mathlib.Data.Rat.Defs
import Mathlib.Data.Rat.Order
import PrimeFoundations.Rational

namespace PrimeFoundations.Cauchy

open PrimeFoundations.Rational

/-! ## The constructive Cauchy carrier -/

/-- A computable real: term sequence + explicit modulus of
convergence. Mirrors `crate::cauchy::Cauchy`. -/
structure CauchySeq where
  nth     : Nat → Rat
  modulus : Rat → Nat
  /-- Productivity: for every positive ε, the sequence past index
      `modulus eps` lies inside an ε-window. This is the algebraic
      content of `convergence::L0_State`'s "convergence" property. -/
  cauchy  : ∀ eps : Rat, 0 < eps →
              ∀ n m : Nat, n ≥ modulus eps → m ≥ modulus eps →
                |nth n - nth m| < eps

namespace CauchySeq

/-! ## Constructors -/

/-- Constant sequence `n ↦ v`. Modulus is `0` for any `eps > 0`
because every two terms are equal. Mirrors
`crate::cauchy::Cauchy::constant`. -/
def constant (v : Rat) : CauchySeq where
  nth     := fun _ => v
  modulus := fun _ => 0
  cauchy  := by
    intro eps hpos _ _ _ _
    simp
    exact hpos

/-! ## Modulus combinators (Phase 3 O2) -/

/-- **Theorem.** Sum of two Cauchy sequences is Cauchy with modulus
`max (a.modulus (eps/2)) (b.modulus (eps/2))`.

This is the load-bearing combinator that justifies
`crate::cauchy::Cauchy::add`'s modulus formula. -/
theorem add_modulus_correct (a b : CauchySeq) (eps : Rat) (hpos : 0 < eps) :
    let m := Nat.max (a.modulus (eps / 2)) (b.modulus (eps / 2))
    ∀ n k : Nat, n ≥ m → k ≥ m →
      |(a.nth n + b.nth n) - (a.nth k + b.nth k)| < eps := by
  intro m n k hn hk
  have h_half : (0 : Rat) < eps / 2 := by linarith
  have ha : |a.nth n - a.nth k| < eps / 2 :=
    a.cauchy (eps / 2) h_half n k
      (le_trans (Nat.le_max_left _ _) hn)
      (le_trans (Nat.le_max_left _ _) hk)
  have hb : |b.nth n - b.nth k| < eps / 2 :=
    b.cauchy (eps / 2) h_half n k
      (le_trans (Nat.le_max_right _ _) hn)
      (le_trans (Nat.le_max_right _ _) hk)
  have : |(a.nth n + b.nth n) - (a.nth k + b.nth k)|
        = |(a.nth n - a.nth k) + (b.nth n - b.nth k)| := by ring_nf
  rw [this]
  calc |(a.nth n - a.nth k) + (b.nth n - b.nth k)|
      ≤ |a.nth n - a.nth k| + |b.nth n - b.nth k| := abs_add _ _
    _ < eps / 2 + eps / 2                          := by linarith
    _ = eps                                        := by ring

/-- **Theorem.** Difference of two Cauchy sequences is Cauchy with
the same combined modulus as `add`. -/
theorem sub_modulus_correct (a b : CauchySeq) (eps : Rat) (hpos : 0 < eps) :
    let m := Nat.max (a.modulus (eps / 2)) (b.modulus (eps / 2))
    ∀ n k : Nat, n ≥ m → k ≥ m →
      |(a.nth n - b.nth n) - (a.nth k - b.nth k)| < eps := by
  intro m n k hn hk
  have h_half : (0 : Rat) < eps / 2 := by linarith
  have ha : |a.nth n - a.nth k| < eps / 2 :=
    a.cauchy (eps / 2) h_half n k
      (le_trans (Nat.le_max_left _ _) hn)
      (le_trans (Nat.le_max_left _ _) hk)
  have hb : |b.nth n - b.nth k| < eps / 2 :=
    b.cauchy (eps / 2) h_half n k
      (le_trans (Nat.le_max_right _ _) hn)
      (le_trans (Nat.le_max_right _ _) hk)
  have eq1 : (a.nth n - b.nth n) - (a.nth k - b.nth k)
           = (a.nth n - a.nth k) - (b.nth n - b.nth k) := by ring
  rw [eq1]
  calc |(a.nth n - a.nth k) - (b.nth n - b.nth k)|
      ≤ |a.nth n - a.nth k| + |b.nth n - b.nth k| := abs_sub _ _
    _ < eps / 2 + eps / 2                          := by linarith
    _ = eps                                        := by ring

/-- **Theorem.** Negation preserves the modulus: `(-a).cauchy =
a.cauchy`. -/
theorem neg_modulus_correct (a : CauchySeq) (eps : Rat) (hpos : 0 < eps) :
    ∀ n k : Nat, n ≥ a.modulus eps → k ≥ a.modulus eps →
      |(-a.nth n) - (-a.nth k)| < eps := by
  intro n k hn hk
  have h := a.cauchy eps hpos n k hn hk
  have eq1 : (-a.nth n) - (-a.nth k) = -(a.nth n - a.nth k) := by ring
  rw [eq1, abs_neg]
  exact h

end CauchySeq

end PrimeFoundations.Cauchy
