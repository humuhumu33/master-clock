/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# U(30) — the reduced residue system modulo MODULUS_U.

Mirrors `src/u30.rs`. Phase 2 — Lean side.

**Builds on:** `PrimeFoundations.Parameters` for `modulusU` and
`u30Cardinality`. Mirrors the Phase 1 Rust constants `RESIDUES` and
`GARY_STATED_RESIDUES`, plus the load-bearing cross-check tests
`derived_residues_match_gary_stated`,
`residues_are_strictly_ascending`,
`residues_are_coprime_to_modulus_u`.

**NO `sorry` POLICY** — every theorem closed by `rfl`, `decide`,
`fin_cases`, or constructive proof.
-/

import PrimeFoundations.Parameters
import Mathlib.Data.Nat.Totient
import Mathlib.Data.List.Sort

namespace PrimeFoundations.U30

open PrimeFoundations.Parameters

/-! ## The 8 residues — derived and stated -/

/-- Gary's verbatim U(30) enumeration from Appendix A reference table.

Used **only** as a verification target against the derived `residues`
below. Production-side code should consult `residues`. -/
def garyStatedResidues : List Nat := [1, 7, 11, 13, 17, 19, 23, 29]

/-- Residues of `U(MODULUS_U)` computed by filtering `[0, MODULUS_U)`
for elements coprime to `MODULUS_U`.

**Derived** at definition time — no hand-typed enumeration.
Mirror of `crate::u30::compute_residues`. -/
def residues : List Nat :=
  (List.range modulusU).filter (fun n => Nat.gcd n modulusU == 1)

/-! ## Cardinality and structural theorems -/

/-- **Theorem.** `|U(30)| = φ(30) = 8`. Closes by `decide` on
`Nat.totient 30`. -/
theorem totient_eq_u30Cardinality :
    Nat.totient modulusU = u30Cardinality := by
  decide

/-- **Theorem.** The computed `residues` exactly equal Gary's stated
list from the Appendix A reference table.

This is the load-bearing cross-check: agreement between `gcd`-based
filtering and Gary's verbatim values. Mirror of the Phase 1
compile-time `const _: () = … RESIDUES[i] == GARY_STATED_RESIDUES[i] …` block. -/
theorem residues_eq_gary_stated : residues = garyStatedResidues := by
  decide

/-- **Theorem.** `residues.length = u30Cardinality = 8`. -/
theorem residues_length : residues.length = u30Cardinality := by
  decide

/-- **Theorem.** `residues` is strictly ascending.

Mirror of `u30::tests::residues_are_strictly_ascending`. The list
is computed by filtering `List.range MODULUS_U`, which is itself
ascending; the filter preserves order. -/
theorem residues_strictly_ascending : residues.Sorted (· < ·) := by
  decide

/-- **Theorem.** Every element of `residues` is coprime to `MODULUS_U`.

Mirror of `u30::tests::residues_are_coprime_to_modulus_u`. The
filter in `residues`'s definition is exactly `gcd · MODULUS_U == 1`,
so this is essentially `List.mem_filter` unfolded. -/
theorem residues_coprime (r : Nat) (h : r ∈ residues) :
    Nat.gcd r modulusU = 1 := by
  rw [residues_eq_gary_stated] at h
  unfold garyStatedResidues at h
  fin_cases h <;> decide

/-- **Theorem.** Every `n ∈ [1, MODULUS_U)` with `gcd(n, MODULUS_U) = 1`
appears in `residues`. Mirror of
`u30::tests::residues_exhaustively_enumerate_u30`.

Together with `residues_coprime` and `residues_strictly_ascending`
this establishes that `residues` enumerates U(MODULUS_U) exactly. -/
theorem residues_exhaustive (n : Nat) (h₁ : 1 ≤ n) (h₂ : n < modulusU)
    (h₃ : Nat.gcd n modulusU = 1) : n ∈ residues := by
  unfold modulusU at h₂
  interval_cases n <;> first | decide | (exfalso; revert h₃; decide)

/-! ## Convenience: cardinality matches Gary's stated value -/

/-- **Theorem.** Gary's stated list also has length `u30Cardinality`.
Trivially true since the lists are equal, but recorded explicitly. -/
theorem garyStatedResidues_length :
    garyStatedResidues.length = u30Cardinality := by
  decide

end PrimeFoundations.U30
