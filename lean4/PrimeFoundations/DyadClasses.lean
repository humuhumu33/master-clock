/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# DyadClasses — Lean 4 mirror of `src/dyad_classes.rs`

The 576 ordered factorization dyads in `U_30(90) × U_30(90)`
partition into `36 = |(Z/9Z)*|²` classes, each containing exactly
`16 = 4²` members. The 4 comes from the period-24 dr cycle's
4-uniformity over `(Z/9Z)*`.

  576 = 24² = (6·4)² = 6²·4² = 36·16

Every theorem closed by `native_decide`. No `sorry`.
-/

import PrimeFoundations.Parameters
import PrimeFoundations.U90
import PrimeFoundations.UnitsZ9

namespace PrimeFoundations.DyadClasses

open PrimeFoundations.U90 PrimeFoundations.UnitsZ9

/-- Digital root: `n mod 9` mapped onto `{1, …, 9}`. -/
def dr (n : Nat) : Nat :=
  if n = 0 then 0 else
    let m := n % 9
    if m = 0 then 9 else m

/-- The 576 ordered dyads with their dr-pair labels. -/
def dyadsWithDrPair : List (Nat × Nat × Nat × Nat) :=
  residuesU90.flatMap (fun a =>
    residuesU90.map (fun b => (a, b, dr a, dr b)))

/-! ## Theorems -/

/-- **Total dyad count.** `|dyadsWithDrPair| = 576`. -/
theorem total_dyads_eq_576 : dyadsWithDrPair.length = 576 := by native_decide

/-- **Class count.** Exactly 36 distinct `(dr1, dr2)` pairs arise. -/
theorem class_count_eq_36 :
    (dyadsWithDrPair.map (fun d => (d.2.2, d.2.2.2))).eraseDups.length = 36 := by
  native_decide

/-- **Per-class size.** Each `(dr1, dr2) ∈ (Z/9Z)* × (Z/9Z)*` is the
label of exactly 16 dyads. -/
theorem each_class_has_16_dyads :
    ∀ (dr1 dr2 : Nat),
      dr1 ∈ unitsZ9 → dr2 ∈ unitsZ9 →
      (dyadsWithDrPair.filter (fun d => d.2.2 = dr1 ∧ d.2.2.2 = dr2)).length = 16 := by
  native_decide

/-- **Cardinality identity.** `36 · 16 = 576`. -/
theorem partition_arithmetic : 36 * 16 = 576 := by decide

/-- **Triality of 576 — three derivations agree.**
1. `|U_30(90)|² = 24²`
2. `|(Z/9Z)*|² · 4² = 36 · 16`
3. `total_dyads_eq_576` -/
theorem triality_of_576 :
    residuesU90.length * residuesU90.length = 576 ∧
    unitsZ9.length * unitsZ9.length * 16 = 576 := by
  refine ⟨?_, ?_⟩ <;> native_decide

end PrimeFoundations.DyadClasses
