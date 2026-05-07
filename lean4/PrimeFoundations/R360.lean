/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# R(360) — the 96-element residue lattice modulo MODULUS_R.

Mirrors `src/r360.rs`. Phase 2 — Lean side.

**Builds on:** `PrimeFoundations.Parameters` for `modulusU`,
`modulusR`, `latticeLayers`, `u30Cardinality`, `rCardinality`.
`PrimeFoundations.U30` for the eight residues. Mirrors the Phase 1
Rust constants `ELEMENTS`, the bijection `decompose`/`compose`, the
spot-check tests, the rejection tests, and the CRT decomposition
`Z/12 ≅ Z/4 × Z/3`.

**NO `sorry` POLICY** — every theorem closed by `rfl`, `decide`, or
constructive proof.
-/

import PrimeFoundations.Parameters
import PrimeFoundations.U30
import Mathlib.Data.List.Basic

namespace PrimeFoundations.R360

open PrimeFoundations.Parameters

/-! ## Two equivalent enumerations -/

/-- The 96 integers of `R(360)` enumerated by **filter**: every
`n ∈ [0, modulusR + 1)` with `gcd(n, modulusU) = 1`. Mirror of the
Rust `compute_elements` const fn. -/
def elements : List Nat :=
  (List.range (modulusR + 1)).filter (fun n => Nat.gcd n modulusU == 1)

/-- The same 96 integers enumerated as a **Cartesian product**
`U(MODULUS_U) × {0, …, LATTICE_LAYERS − 1}` under the map
`(u, k) ↦ u + MODULUS_U · k`. This is the structurally-meaningful
form used by `decompose`/`compose`. -/
def elementsByCartesian : List Nat :=
  (List.range latticeLayers).flatMap fun k =>
    U30.residues.map fun u => u + modulusU * k

/-! ## Cardinality and agreement -/

/-- **Theorem.** `|elements| = R_CARDINALITY = 96`. -/
theorem elements_length : elements.length = rCardinality := by
  decide

/-- **Theorem.** Cartesian and filter enumerations agree. The
Phase 1 invariant that the bijection `(u, k) ↔ u + 30k` is well-
defined on R(360) becomes this list equality. -/
theorem elements_eq_byCartesian : elements = elementsByCartesian := by
  decide

/-- **Theorem.** `|elementsByCartesian| = R_CARDINALITY`. Useful when
reasoning structurally. -/
theorem elementsByCartesian_length :
    elementsByCartesian.length = rCardinality := by
  decide

/-! ## Bijection R(360) ↔ U(30) × Z/LATTICE_LAYERS -/

/-- Decompose `n ∈ R(360)` into `(u_index, layer)` such that
`n = U30.residues[u_index] + MODULUS_U · layer`. Returns `none` if
`n` is not in `R(360)`. -/
def decompose (n : Nat) : Option (Nat × Nat) :=
  if n = 0 ∨ modulusR < n then none
  else if Nat.gcd n modulusU ≠ 1 then none
  else
    let u := n % modulusU
    let layer := n / modulusU
    let idx := U30.residues.findIdx (· = u)
    some (idx, layer)

/-- Compose `(u_index, layer)` back into `n = residues[u_index] + MODULUS_U · layer`. -/
def compose (uIdx : Nat) (layer : Nat) : Option Nat :=
  if uIdx < u30Cardinality ∧ layer < latticeLayers then
    let u := U30.residues.getD uIdx 0
    some (u + modulusU * layer)
  else
    none

/-! ## Spot checks against the Appendix A reference (Phase 1 mirror) -/

/-- **Theorem.** `decompose` matches Appendix A reference table on the
spot-check rows. Mirror of `r360::tests::spot_checks_against_fixed_pdf`. -/
theorem decompose_spot_checks :
    decompose 1 = some (0, 0) ∧
    decompose 7 = some (1, 0) ∧
    decompose 31 = some (0, 1) ∧
    decompose 359 = some (7, 11) := by
  decide

/-- **Theorem.** `compose` is the inverse of `decompose` on those
same spot rows. -/
theorem compose_spot_checks :
    compose 0 0 = some 1 ∧
    compose 1 0 = some 7 ∧
    compose 0 1 = some 31 ∧
    compose 7 11 = some 359 := by
  decide

/-- **Theorem.** `decompose` rejects integers outside `R(360)`.
Mirror of `r360::tests::decompose_rejects_non_coprime` and
`r360::tests::decompose_rejects_out_of_range`. -/
theorem decompose_rejects :
    decompose 0 = none ∧
    decompose 2 = none ∧
    decompose 5 = none ∧
    decompose 30 = none ∧
    decompose 360 = none ∧
    decompose 361 = none := by
  decide

/-- **Theorem.** `compose` rejects out-of-range indices. Mirror of
`r360::tests::compose_rejects_out_of_range`. -/
theorem compose_rejects :
    compose u30Cardinality 0 = none ∧
    compose 0 latticeLayers = none := by
  decide

/-! ## Exhaustive round-trip -/

/-- **Theorem.** `compose ∘ decompose = id` on every element of
`R(360)`. The load-bearing bijection mirror of
`r360::tests::decompose_then_compose_is_identity`.

Stated in Bool form via `List.all` so `decide` can evaluate it
exhaustively over all 96 elements. -/
theorem all_elements_round_trip :
    elements.all (fun n =>
      ((decompose n).bind fun p => compose p.1 p.2) == some n) = true := by
  decide

/-! ## CRT decomposition Z/12 ≅ Z/4 × Z/3 -/

/-- **Theorem.** `Z/12 ≅ Z/4 × Z/3` via `k ↦ (k mod 4, k mod 3)`.

Each `(a, b) ∈ Z/4 × Z/3` corresponds to exactly one
`k ∈ Z/LATTICE_LAYERS = Z/12`. The `Z/3` factor is the candidate
for matching `query::TriadProjection` in the UOR ontology
(Phase 2 O3); the `Z/4` factor encodes the binary depth.

Mirror of `r360::tests::z12_decomposes_via_crt`. -/
theorem z12_decomposes_via_crt :
    ∀ a, a < 4 → ∀ b, b < 3 →
      (List.range latticeLayers).countP (fun k => k % 4 = a ∧ k % 3 = b) = 1 := by
  decide

/-! ## Set agreement with `appendix_a::ROWS` (forward reference) -/

/-- **Theorem.** Every `n ∈ elements` is coprime to `MODULUS_U`. By
construction (the filter), but recorded explicitly so downstream
modules (`AppendixA.lean`) can cite it. -/
theorem elements_coprime (n : Nat) (h : n ∈ elements) :
    Nat.gcd n modulusU = 1 := by
  unfold elements at h
  rw [List.mem_filter] at h
  obtain ⟨_, hgcd⟩ := h
  exact beq_iff_eq.mp hgcd

end PrimeFoundations.R360