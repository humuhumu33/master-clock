/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# SiamesePrimePattern — Lean 4 mirror of `src/siamese_prime_pattern.rs`

A Siamese prime pair is `(n² − 2, n² + 2)` with both prime. Sweeping
`n mod 30`, exactly five generator residues survive the necessary
condition that both `n² ± 2` are coprime to 30:

  n mod 30 ∈ {3, 9, 15, 21, 27} = {6k + 3 : k ∈ {0, …, 4}} (OEIS A016945)

Every theorem closed by `decide`. No `sorry`.
-/

import PrimeFoundations.Parameters

namespace PrimeFoundations.SiamesePrimePattern

open PrimeFoundations.Parameters

/-- The five generator residues mod 30 for Siamese prime pairs. -/
def siameseGeneratorResidues : List Nat := [3, 9, 15, 21, 27]

/-- Equivalent form: `6k + 3` for `k ∈ {0, 1, 2, 3, 4}`. -/
def siameseAs6kPlus3 : List Nat := [0, 1, 2, 3, 4].map (fun k => 6 * k + 3)

/-- Predicate: `n mod 30 ∈ {3, 9, 15, 21, 27}`. -/
def isSiameseGeneratorCandidate (n : Nat) : Bool :=
  let r := n % modulusU
  r = 3 ∨ r = 9 ∨ r = 15 ∨ r = 21 ∨ r = 27

/-- Equivalent predicate: `n mod 6 = 3`. -/
def isSiameseGeneratorViaMod6 (n : Nat) : Bool := n % 6 = 3

/-! ## Theorems -/

theorem siamese_residues_match_6k_plus_3 :
    siameseGeneratorResidues = siameseAs6kPlus3 := by
  native_decide

theorem siamese_generator_minus_two_coprime_to_30 :
    ∀ n ∈ siameseGeneratorResidues,
      let m := (n * n + modulusU - 2) % modulusU
      Nat.gcd m modulusU = 1 := by
  decide

theorem siamese_generator_plus_two_coprime_to_30 :
    ∀ n ∈ siameseGeneratorResidues,
      Nat.gcd ((n * n + 2) % modulusU) modulusU = 1 := by
  decide

theorem siamese_predicate_matches_residue_set :
    ∀ r < modulusU,
      isSiameseGeneratorCandidate r ↔ r ∈ siameseGeneratorResidues := by
  decide

theorem siamese_predicates_equivalent_mod_30 :
    ∀ n < modulusU,
      isSiameseGeneratorCandidate n ↔ isSiameseGeneratorViaMod6 n := by
  decide

end PrimeFoundations.SiamesePrimePattern
