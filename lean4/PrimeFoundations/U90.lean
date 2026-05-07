/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# U90 — Lean 4 mirror of `src/u90.rs`

The 24-element residue substrate `U_30(90) = { n ∈ [1, 90] : gcd(n, 30) = 1 }`.

This is the bridge between Phase 1's `U(30)` (8 elements) and the
`R(360)` lattice (96 elements). Every structural fact on
primesdemystified.com about the reference table and the
period-24 digital-root cycle reduces to facts about this set and the
units of `Z/9Z`.

Each theorem is closed by `decide` or `native_decide`. There are no
`sorry` statements anywhere in this module.

## Theorems proven

1. `|U_30(90)| = 24`.
2. `U_30(90)` matches Gary's reference table row labels exactly.
3. The digital-root cycle has period 24 with values in `{1,2,4,5,7,8}`.
4. Each unit of `(Z/9Z)*` appears exactly 4 times in the cycle.
5. Sum of `U_30(90)` equals `1080 = 3 · MODULUS_R = 3 · 360`.
6. Squares of `U_30(90)` mod 90 take exactly six distinct values.
7. The 24 squares sum to `960 = 4 · 240`.
8. `|U_30(90)| · 4 = |R(360)| = 96` (the four-fold tiling).
-/

import PrimeFoundations.Parameters

namespace PrimeFoundations.U90

open PrimeFoundations.Parameters

/-! ## Definitions -/

/-- `MODULUS_U90 = 3 · MODULUS_U = 90`. Smallest multiple of
`MODULUS_U` that tiles `MODULUS_R = 360` exactly four times. -/
def modulusU90 : Nat := 3 * modulusU

/-- The 24 residues of `U_30(90)`. Computed via filter; mirrors
`compute_residues_u90` in `src/u90.rs`. -/
def residuesU90 : List Nat :=
  (List.range (modulusU90 + 1)).filter (fun n => n ≥ 1 ∧ Nat.gcd n modulusU = 1)

/-- Digital root: the iterated digit sum, equivalently `n mod 9`
mapped onto `{1, …, 9}` (with `dr(0) = 0`). -/
def dr (n : Nat) : Nat :=
  if n = 0 then 0 else
    let m := n % 9
    if m = 0 then 9 else m

/-- The digital-root cycle over `U_30(90)`. -/
def drCycle : List Nat := residuesU90.map dr

/-- The six units of `(Z/9Z)*`. -/
def unitsZ9 : List Nat := [1, 2, 4, 5, 7, 8]

/-! ## Theorems -/

/-- **|U_30(90)| = 24.** -/
theorem cardinality_u90 : residuesU90.length = 24 := by native_decide

/-- **The 24 residues match the reference table.** -/
theorem residues_match_gary :
    residuesU90 = [1, 7, 11, 13, 17, 19, 23, 29,
                   31, 37, 41, 43, 47, 49, 53, 59,
                   61, 67, 71, 73, 77, 79, 83, 89] := by
  native_decide

/-- **The dr cycle matches the Mod-90 PDF.** Period 24 with the
exact value sequence Gary tabulates. -/
theorem dr_cycle_matches_pdf :
    drCycle = [1, 7, 2, 4, 8, 1, 5, 2,
               4, 1, 5, 7, 2, 4, 8, 5,
               7, 4, 8, 1, 5, 7, 2, 8] := by
  native_decide

/-- **4-uniformity of the dr cycle.** Each unit of `(Z/9Z)*`
appears exactly 4 times. -/
theorem dr_cycle_unit_1_count : (drCycle.filter (· = 1)).length = 4 := by native_decide
theorem dr_cycle_unit_2_count : (drCycle.filter (· = 2)).length = 4 := by native_decide
theorem dr_cycle_unit_4_count : (drCycle.filter (· = 4)).length = 4 := by native_decide
theorem dr_cycle_unit_5_count : (drCycle.filter (· = 5)).length = 4 := by native_decide
theorem dr_cycle_unit_7_count : (drCycle.filter (· = 7)).length = 4 := by native_decide
theorem dr_cycle_unit_8_count : (drCycle.filter (· = 8)).length = 4 := by native_decide

/-- **Zero-count on `{3, 6, 9}`.** All members of `U_30(90)` are
coprime to 3, so their digital roots cannot be multiples of 3. -/
theorem dr_cycle_no_3 : (drCycle.filter (· = 3)).length = 0 := by native_decide
theorem dr_cycle_no_6 : (drCycle.filter (· = 6)).length = 0 := by native_decide
theorem dr_cycle_no_9 : (drCycle.filter (· = 9)).length = 0 := by native_decide

/-- **Linear sum of `U_30(90)`** — equals `1080 = 3 · MODULUS_R`. -/
theorem linear_sum_eq_1080 :
    residuesU90.foldr (· + ·) 0 = 1080 := by native_decide

/-- **Sum equals `3 · MODULUS_R`.** Anchors the row sum of the 24×24
mod-90 multiplication matrix to the `MODULUS_R` constant. -/
theorem linear_sum_eq_three_modulus_r :
    residuesU90.foldr (· + ·) 0 = 3 * modulusR := by native_decide

/-- **Diagonal squares mod 90 take exactly 6 distinct values.** The
list `(a · a) mod 90` for `a ∈ U_30(90)` reduces to the canonical
6-element set `{1, 19, 31, 49, 61, 79}`. -/
theorem diagonal_residues_eq_six :
    (residuesU90.map (fun a => (a * a) % modulusU90)).eraseDups
    = [1, 49, 31, 79, 19, 61] := by
  native_decide

/-- **Diagonal sum equals 960.** -/
theorem diagonal_sum_eq_960 :
    (residuesU90.map (fun a => (a * a) % modulusU90)).foldr (· + ·) 0
    = 960 := by native_decide

/-- **Diagonal sum factors as `4 · 240`.** -/
theorem diagonal_sum_eq_four_times_240 :
    (residuesU90.map (fun a => (a * a) % modulusU90)).foldr (· + ·) 0
    = 4 * 240 := by native_decide

/-- **Anchor to Phase 1.** `|U_30(90)| · (MODULUS_R / MODULUS_U90) =
|R(360)| = 96`. The 24-element substrate tiles into the 96-element
lattice exactly four times. -/
theorem tiles_into_r360 :
    residuesU90.length * (modulusR / modulusU90) = rCardinality := by
  native_decide

/-- **Secondary diagonal residues.** Distinct value