/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# PerfectSquaresMod30 — Lean 4 mirror of `src/perfect_squares_mod30.rs`

For every `n ∈ U(30)`:
  - `n² mod 30 ∈ {1, 19}` (each value hit by 4 of the 8 members)
  - `dr(n²) ∈ {1, 4, 7}` (the squares of `(Z/9Z)*`)

Every theorem closed by `decide`. No `sorry`.
-/

import PrimeFoundations.Parameters

namespace PrimeFoundations.PerfectSquaresMod30

open PrimeFoundations.Parameters

/-- The 8 residues of `U(30)`. -/
def u30 : List Nat := [1, 7, 11, 13, 17, 19, 23, 29]

/-- Digital root: `n mod 9` mapped onto `{1, …, 9}`, with `dr(0) = 0`. -/
def dr (n : Nat) : Nat :=
  if n = 0 then 0 else
    let m := n % 9
    if m = 0 then 9 else m

/-- Distinct mod-30 residues of `n²` for `n ∈ u30`. -/
def squareMod30Residues : List Nat := [1, 19]

/-- Distinct digital roots of `n²` for `n ∈ u30`. -/
def squareDrValues : List Nat := [1, 4, 7]

/-- Predicate: `n mod 30 ∈ {1, 19}`. -/
def isSquareResidueMod30 (n : Nat) : Bool :=
  let r := n % modulusU
  r = 1 ∨ r = 19

/-- Predicate: `dr(n) ∈ {1, 4, 7}`. -/
def hasSquareDr (n : Nat) : Bool :=
  let d := dr n
  d = 1 ∨ d = 4 ∨ d = 7

/-! ## Theorems -/

theorem squares_mod_30_in_one_or_nineteen :
    ∀ n ∈ u30, (n * n) % modulusU = 1 ∨ (n * n) % modulusU = 19 := by
  decide

theorem squares_dr_in_one_four_seven :
    ∀ n ∈ u30, dr (n * n) ∈ squareDrValues := by
  decide

theorem each_square_residue_hit_4_times :
    (u30.filter (fun n => (n * n) % modulusU = 1)).length = 4 ∧
    (u30.filter (fun n => (n * n) % modulusU = 19)).length = 4 := by
  decide

theorem mod30_predicate_accepts_all_u30_squares :
    ∀ n ∈ u30, isSquareResidueMod30 (n * n) = true := by
  decide

theorem dr_predicate_accepts_all_u30_squares :
    ∀ n ∈ u30, hasSquareDr (n * n) = true := by
  decide

end PrimeFoundations.PerfectSquaresMod30
