/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# TwinPrimeResidues — Lean 4 mirror of `src/twin_prime_residues.rs`

Twin prime pairs `(p, p+2)` with `p > 5` must satisfy `p mod 30 ∈
{11, 17, 29}`. The other five `U(30)` residues `{1, 7, 13, 19, 23}`
force `p + 2` divisible by 3 or 5.

This is a **necessary** condition on twin primes — it does not test
primality. Combined with the wheel-30 sieve in `PrimeFoundations.PrimeCount`
it gives a 5/8 candidate reduction before any primality test runs.

## Anchor

Derived from `MODULUS_U = 30` via `Nat.gcd`. Every theorem closed by
`decide`. No `sorry`.
-/

import PrimeFoundations.Parameters
import PrimeFoundations.U30

namespace PrimeFoundations.TwinPrimeResidues

open PrimeFoundations.Parameters

/-! ## Definitions -/

/-- The three residues mod 30 that can be the smaller of a twin
prime pair `(p, p+2)` with `p > 5`. -/
def twinPrimeLowerResidues : List Nat := [11, 17, 29]

/-- The five `U(30)` residues that cannot be the smaller of a twin
prime pair `(p > 5)` because `r + 2` is divisible by 3 or 5. -/
def isolatedLowerResidues : List Nat := [1, 7, 13, 19, 23]

/-- Twin prime residue pairs `(p mod 30, (p + 2) mod 30)` for the
three residues that survive both gcd-with-30 = 1 conditions. -/
def twinPrimeResiduePairs : List (Nat × Nat) :=
  [(11, 13), (17, 19), (29, 1)]

/-- Predicate: `n mod 30 ∈ {11, 17, 29}`. The necessary residue
condition for `n` to be the smaller member of a twin prime pair. -/
def isTwinPrimeCandidateLower (n : Nat) : Bool :=
  let r := n % modulusU
  r = 11 ∨ r = 17 ∨ r = 29

/-! ## Theorems (zero sorry) -/

/-- **Theorem.** For every `r` in `twinPrimeLowerResidues`, the
successor `(r + 2) mod 30` is coprime to 30. -/
theorem twin_lower_has_coprime_successor :
    ∀ r ∈ twinPrimeLowerResidues,
      Nat.gcd ((r + 2) % modulusU) modulusU = 1 := by
  decide

/-- **Theorem.** For every `r` in `isolatedLowerResidues`, the
successor `(r + 2) mod 30` is **not** coprime to 30. -/
theorem isolated_lower_has_noncoprime_successor :
    ∀ r ∈ isolatedLowerResidues,
      Nat.gcd ((r + 2) % modulusU) modulusU > 1 := by
  decide

/-- **Theorem.** Every residue in `twinPrimeLowerResidues` is itself
coprime to 30 (i.e. lies in `U(30)`). -/
theorem twin_lower_in_u30 :
    ∀ r ∈ twinPrimeLowerResidues, Nat.gcd r modulusU = 1 := by
  decide

/-- **Theorem.** Every residue in `isolatedLowerResidues` is itself
coprime to 30. The "isolated" predicate is about the successor, not
the residue itself — both sets live inside `U(30)`. -/
theorem isolated_lower_in_u30 :
    ∀ r ∈ isolatedLowerResidues, Nat.gcd r modulusU = 1 := by
  decide

/-- **Theorem.** The two sets partition `U(30)`. -/
theorem partition_covers_u30 :
    twinPrimeLowerResidues.length + isolatedLowerResidues.length
    = u30Cardinality := by
  decide

/-- **Theorem.** The two sets are disjoint. -/
theorem twin_and_isolated_disjoint :
    ∀ r ∈ twinPrimeLowerResidues, r ∉ isolatedLowerResidues := by
  decide

/-- **Theorem.** Each twin prime pair `(a, b)` satisfies `b = (a+2) mod 30`. -/
theorem pairs_offset_by_two :
    ∀ (p : Nat × Nat), p ∈ twinPrimeResiduePairs → p.2 = (p.1 + 2) % modulusU := by
  decide

/-- **Theorem.** Both members of every pair are coprime to 30. -/
theorem pairs_both_in_u30 :
    ∀ (p : Nat × Nat), p ∈ twinPrimeResiduePairs →
      Nat.gcd p.1 modulusU = 1 ∧ Nat.gcd p.2 modulusU = 1 := by
  decide

/-- **Theorem.** The predicate `isTwinPrimeCandidateLower` accepts
exactly the residues in `twinPrimeLowerResidues`. -/
theorem predicate_matches_residue_set :
    ∀ r < modulusU,
      isTwinPrimeCandidateLower r ↔ r ∈ twinPrimeLowerResidues := by
  decide

end PrimeFoundations.TwinPrimeResidues
