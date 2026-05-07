/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# Period24DrSequences — Lean 4 mirror of `src/period24_dr_sequences.rs`

For each `r ∈ U_30(90)`, the sequence `s_r(k) = dr(r · U_30(90)[k]
mod 90)` is a permutation of the dr cycle (since multiplication by
`r` is a bijection of `U_30(90)` under mod-90 multiplication).
Therefore every row of the 24×24 table sums to 108 and the total
is 2592 = 6·432.

Every theorem closed by `native_decide`. No `sorry`.
-/

import PrimeFoundations.Parameters
import PrimeFoundations.U90

namespace PrimeFoundations.Period24DrSequences

open PrimeFoundations.Parameters PrimeFoundations.U90

/-- Digital root: `n mod 9` mapped onto `{1, …, 9}`. -/
def dr (n : Nat) : Nat :=
  if n = 0 then 0 else
    let m := n % 9
    if m = 0 then 9 else m

/-- The 24×24 dr table: `seq[i][k] = dr(residuesU90[i] · residuesU90[k] mod 90)`. -/
def seq : List (List Nat) :=
  residuesU90.map (fun r =>
    residuesU90.map (fun a => dr ((r * a) % modulusU90)))

/-! ## Theorems -/

/-- Every row sums to 108. -/
theorem every_row_sums_to_108 :
    ∀ row ∈ seq, row.foldr (· + ·) 0 = 108 := by
  native_decide

/-- Total table sum equals 2592 = 6 · 432. -/
theorem total_eq_2592 :
    (seq.map (fun row => row.foldr (· + ·) 0)).foldr (· + ·) 0 = 2592 := by
  native_decide

/-- Total factors as `6 · 432`. -/
theorem total_factors_through_432 :
    (seq.map (fun row => row.foldr (· + ·) 0)).foldr (· + ·) 0 = 6 * 432 := by
  native_decide

/-- Every entry is in `(Z/9Z)*`. -/
theorem every_entry_in_units_z9 :
    ∀ row ∈ seq, ∀ entry ∈ row, entry ∈ ([1, 2, 4, 5, 7, 8] : List Nat) := by
  native_decide

end PrimeFoundations.Period24DrSequences
