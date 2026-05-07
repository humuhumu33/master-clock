/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# IndexToResidue — Lean 4 mirror of `src/index_to_residue.rs`

Closed-form bijection between `k ≥ 1` and the `k`-th member of
`U_30(∞) = {n ≥ 1 : gcd(n, 30) = 1}` in ascending order:

  nth_u30_member(k) = MODULUS_U · ⌊(k − 1) / 8⌋ + RESIDUES[(k − 1) mod 8]

The reverse map computes the index of any `n ∈ U_30(∞)`.

Every theorem closed by `decide` or `native_decide`. No `sorry`.
-/

import PrimeFoundations.Parameters

namespace PrimeFoundations.IndexToResidue

open PrimeFoundations.Parameters

/-- The 8 residues of `U(30)`. -/
def residues : List Nat := [1, 7, 11, 13, 17, 19, 23, 29]

/-- The `k`-th element (1-indexed) of `U_30(∞)` in ascending order. -/
def nthU30Member (k : Nat) : Nat :=
  if k = 0 then 0
  else
    let zero_idx := k - 1
    let block := zero_idx / 8
    let offset_idx := zero_idx % 8
    block * modulusU + (residues.get? offset_idx).getD 0

/-- Index of `n` in `U_30(∞)`, or `0` if `n` is not coprime to 30. -/
def indexOfU30Member (n : Nat) : Nat :=
  if n = 0 ∨ Nat.gcd n modulusU ≠ 1 then 0
  else
    let block := n / modulusU
    let offset := n % modulusU
    let pos := (residues.findIdx (fun r => r = offset))
    block * 8 + pos + 1

/-! ## Anchors from primesdemystified.com -/

theorem nth_at_one : nthU30Member 1 = 1 := by native_decide
theorem nth_at_eight : nthU30Member 8 = 29 := by native_decide
theorem nth_at_nine : nthU30Member 9 = 31 := by native_decide
theorem nth_at_twentyfour : nthU30Member 24 = 89 := by native_decide
theorem nth_at_2112_eq_7919 : nthU30Member 2112 = 7919 := by native_decide

theorem index_at_89_eq_24 : indexOfU30Member 89 = 24 := by native_decide
theorem index_at_7919_eq_2112 : indexOfU30Member 7919 = 2112 := by native_decide

/-! ## Round-trip theorems -/

/-- Forward then inverse round-trips on `k ∈ [1, 96]`. -/
theorem forward_inverse_round_trip :
    ∀ k ∈ List.range' 1 96, indexOfU30Member (nthU30Member k) = k := by
  native_decide

/-- Inverse then forward round-trips on every `n ∈ [1, 360]`
coprime to 30. -/
theorem inverse_forward_round_trip :
    ∀ n ∈ (List.range' 1 360).filter (fun n => Nat.gcd n modulusU = 1),
      nthU30Member (indexOfU30Member n) = n := by
  native_decide

/-- Inverse rejects non-coprime arguments. -/
theorem inverse_rejects_non_coprime :
    indexOfU30Member 2 = 0 ∧
    indexOfU30Member 15 = 0 ∧
    indexOfU30Member 30 = 0 := by
  refine ⟨?_, ?_, ?_⟩ <;> native_decide

end PrimeFoundations.IndexToResidue
