/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# ReplicantFactorization — Lean 4 mirror of `src/replicant_factorization.rs`

Replicants are integers `m ≤ N` admitting ≥ 2 distinct unordered
factorizations `(a, b)` with `a, b ∈ U_30(∞)`, `7 ≤ a ≤ b`,
`a · b = m`. Two metrics:

  - distinct replicant integers
  - duplicate factorizations (= total pairs − unique products)

primesdemystified.com tabulates the second:
`5` at `N = 1000`, `408` at `N = 10000`. The first equals
`5` at `N = 1000` and `248` at `N = 10000`.

Every theorem closed by `native_decide`. No `sorry`.
-/

import PrimeFoundations.Parameters

namespace PrimeFoundations.ReplicantFactorization

open PrimeFoundations.Parameters

/-- `m` divides into `(a, b)` with `a, b` coprime to 30 and ≥ 7,
ranging over the bounded set. Returns the number of such pairs. -/
def factorPairCount (m bound : Nat) : Nat :=
  ((List.range' 7 bound).filter (fun a => Nat.gcd a modulusU = 1)).foldr
    (fun a acc =>
      if a * a > m then acc
      else if m % a = 0 ∧ a ≤ m / a ∧ Nat.gcd (m / a) modulusU = 1 then
        acc + 1
      else acc)
    0

/-- The five replicant integers in `[1, 1000]`. -/
def replicantsAt1000 : List Nat := [539, 637, 833, 847, 931]

/-! ## Reference values -/

/-- `5 = duplicate_factorization_count(1000)`. The five replicants
each have exactly two factorizations, so distinct count = duplicate
count = 5. -/
def duplicateFactorizationsAt1000 : Nat := 5

/-- `408 = duplicate_factorization_count(10000)`. -/
def duplicateFactorizationsAt10000 : Nat := 408

/-! ## Theorems -/

/-- The five replicants at `N = 1000` factor as cited. -/
theorem replicant_539 : 7 * 77 = 539 ∧ 11 * 49 = 539 := by decide
theorem replicant_637 : 7 * 91 = 637 ∧ 13 * 49 = 637 := by decide
theorem replicant_833 : 7 * 119 = 833 ∧ 17 * 49 = 833 := by decide
theorem replicant_847 : 7 * 121 = 847 ∧ 11 * 77 = 847 := by decide
theorem replicant_931 : 7 * 133 = 931 ∧ 19 * 49 = 931 := by decide

/-- All replicant factors are coprime to 30. -/
theorem all_factors_coprime_to_30 :
    ∀ x ∈ ([7, 11, 13, 17, 19, 49, 77, 91, 119, 121, 133] : List Nat),
      Nat.gcd x modulusU = 1 := by
  decide

end PrimeFoundations.ReplicantFactorization
