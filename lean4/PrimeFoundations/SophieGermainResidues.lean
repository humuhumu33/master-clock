/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# SophieGermainResidues — Lean 4 mirror of `src/sophie_germain_residues.rs`

A Sophie Germain prime is a prime `p` such that `2p + 1` is also prime.
For `p > 5`, both `p` and `2p + 1` are coprime to 30, so `p mod 30 ∈
{11, 23, 29}` — the only three residues `r ∈ U(30)` for which
`(2r + 1) mod 30` is also coprime to 30.

Every theorem closed by `decide`. No `sorry`.
-/

import PrimeFoundations.Parameters

namespace PrimeFoundations.SophieGermainResidues

open PrimeFoundations.Parameters

/-- The three residues mod 30 that can be Sophie Germain primes
for `p > 5`. -/
def sgLowerResidues : List Nat := [11, 23, 29]

/-- The five `U(30)` residues that cannot be Sophie Germain
primes. For these `r`, `2r + 1` is divisible by 3 or 5. -/
def sgIsolatedResidues : List Nat := [1, 7, 13, 17, 19]

/-- Sophie Germain residue pairs `(p mod 30, (2p + 1) mod 30)`. -/
def sgResiduePairs : List (Nat × Nat) :=
  [(11, 23), (23, 17), (29, 29)]

/-- Predicate: `n mod 30 ∈ {11, 23, 29}`. -/
def isSophieGermainCandidate (n : Nat) : Bool :=
  let r := n % modulusU
  r = 11 ∨ r = 23 ∨ r = 29

/-! ## Theorems -/

theorem sg_lower_has_coprime_doubled_successor :
    ∀ r ∈ sgLowerResidues,
      Nat.gcd ((2 * r + 1) % modulusU) modulusU = 1 := by
  decide

theorem sg_isolated_has_noncoprime_doubled_successor :
    ∀ r ∈ sgIsolatedResidues,
      Nat.gcd ((2 * r + 1) % modulusU) modulusU > 1 := by
  decide

theorem sg_lower_in_u30 :
    ∀ r ∈ sgLowerResidues, Nat.gcd r modulusU = 1 := by
  decide

theorem sg_isolated_in_u30 :
    ∀ r ∈ sgIsolatedResidues, Nat.gcd r modulusU = 1 := by
  decide

theorem sg_partition_covers_u30 :
    sgLowerResidues.length + sgIsolatedResidues.length = u30Cardinality := by
  decide

theorem sg_pairs_form_2p_plus_1 :
    ∀ p ∈ sgResiduePairs, p.2 = (2 * p.1 + 1) % modulusU := by
  decide

theorem sg_predicate_matches_residue_set :
    ∀ r < modulusU,
      isSophieGermainCandidate r ↔ r ∈ sgLowerResidues := by
  decide

end PrimeFoundations.SophieGermainResidues
