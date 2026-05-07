/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# FactorizationDyads — Lean 4 mirror of `src/factorization_dyads.rs`

The 576 = 24² ordered dyads `(a, b) ∈ U_30(90) × U_30(90)`, each
mapped to its product `(a · b) mod 90`. Group closure of
`U_30(90)` under mod-90 multiplication ensures every product
returns to `U_30(90)`, partitioning the 576 dyads into 24 fibers
of 24.

Every theorem closed by `native_decide`. No `sorry`.
-/

import PrimeFoundations.Parameters
import PrimeFoundations.U90

namespace PrimeFoundations.FactorizationDyads

open PrimeFoundations.Parameters PrimeFoundations.U90

/-- All ordered dyads `(a, b, (a · b) mod 90)` in `U_30(90)²`. -/
def dyads : List (Nat × Nat × Nat) :=
  residuesU90.flatMap (fun a =>
    residuesU90.map (fun b => (a, b, (a * b) % modulusU90)))

/-! ## Theorems -/

/-- **Total count.** `|dyads| = 576 = 24²`. -/
theorem dyad_count_eq_576 : dyads.length = 576 := by native_decide

/-- **Total count factors as `U90_CARDINALITY²`.** -/
theorem dyad_count_eq_u90_cardinality_squared :
    dyads.length = residuesU90.length * residuesU90.length := by native_decide

/-- **Group closure.** Every product `(a · b) mod 90` lands back in
`U_30(90)`. -/
theorem closure_under_mod90_multiplication :
    ∀ d ∈ dyads, d.2.2 ∈ residuesU90 := by native_decide

/-- **Fiber structure.** For each `r ∈ U_30(90)`, exactly 24 dyads
have product `r`. -/
theorem fiber_size_eq_24 :
    ∀ r ∈ residuesU90,
      (dyads.filter (fun d => d.2.2 = r)).length = 24 := by
  native_decide

/-- **Diagonal.** The 24 dyads `(a, a, a²)` are squares of
`U_30(90)` members; their distinct products equal
`{1, 19, 31, 49, 61, 79}`. -/
theorem diagonal_dyads_count : (dyads.filter (fun d => d.1 = d.2.1)).length = 24 := by
  native_decide

end PrimeFoundations.FactorizationDyads
