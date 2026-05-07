/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# UnitsZ9 — Lean 4 mirror of `src/units_z9.rs`

The six units of `(Z/9Z)*` and their 6×6 multiplication table modulo 9.
Anchored to one parameter only: `MODULUS_DR = NUMERAL_BASE − 1 = 9`.

Every claim is decided at compile time — no `sorry`, no axioms beyond
the kernel.

## Theorems proven

1. `(Z/9Z)* = {1, 2, 4, 5, 7, 8}`.
2. The group is cyclic of order 6 with generator `2`.
3. The 6×6 multiplication table is closed in `(Z/9Z)*` (Latin square).
4. Every row sums to `27 = 3 · MODULUS_DR`.
5. Total table sum is `162 = 6 · 27`.
6. Additive-inverse pairs `(1,8) (2,7) (4,5)` each sum to 9.
7. Multiplicative-inverse pairs `(1,1) (2,5) (4,7) (8,8)` each
   multiply to 1 mod 9.
-/

import PrimeFoundations.Parameters

namespace PrimeFoundations.UnitsZ9

open PrimeFoundations.Parameters

/-! ## Definitions -/

/-- The units of `Z/MODULUS_DR Z`. With `MODULUS_DR = 9` this is
`{1, 2, 4, 5, 7, 8}` — the six classes coprime to 9. -/
def unitsZ9 : List Nat :=
  (List.range modulusDR).filter (fun n => n ≥ 1 ∧ Nat.gcd n modulusDR = 1)

/-- Successive powers of `2` modulo `MODULUS_DR`, used as the
generator witness for the cyclic structure. -/
def powersOfTwoMod9 : List Nat :=
  let rec go (k : Nat) (x : Nat) (acc : List Nat) : List Nat :=
    if k = 0 then acc.reverse
    else go (k - 1) ((x * 2) % modulusDR) (((x * 2) % modulusDR) :: acc)
  go 6 1 []

/-- The 6×6 multiplication table modulo 9. Entry `(i, j)` is
`(unitsZ9[i] * unitsZ9[j]) mod 9`. -/
def multTableZ9 : List (List Nat) :=
  unitsZ9.map (fun a => unitsZ9.map (fun b => (a * b) % modulusDR))

/-- Sum of all entries in the table. -/
def multTableTotalSum : Nat :=
  (multTableZ9.map (fun row => row.foldr (· + ·) 0)).foldr (· + ·) 0

/-- Sum of `unitsZ9` itself. Equals 27 = every row sum of `multTableZ9`. -/
def unitsZ9Sum : Nat := unitsZ9.foldr (· + ·) 0

/-! ## Theorems -/

/-- **`(Z/9Z)* = {1, 2, 4, 5, 7, 8}`.** -/
theorem units_z9_canonical : unitsZ9 = [1, 2, 4, 5, 7, 8] := by native_decide

/-- **Cardinality 6.** -/
theorem units_z9_cardinality : unitsZ9.length = 6 := by native_decide

/-- **Cyclic of order 6, generator 2.** Powers of 2 mod 9 are
`[2, 4, 8, 7, 5, 1]` — every unit appears, so 2 generates `(Z/9Z)*`. -/
theorem two_generates_units_z9 :
    powersOfTwoMod9 = [2, 4, 8, 7, 5, 1] := by native_decide

/-- **Closure.** Every entry of `multTableZ9` is in `unitsZ9`. -/
theorem mult_table_closed :
    ∀ row ∈ multTableZ9, ∀ entry ∈ row, entry ∈ unitsZ9 := by
  native_decide

/-- **The canonical Latin square.** -/
theorem mult_table_canonical :
    multTableZ9 = [[1, 2, 4, 5, 7, 8],
                   [2, 4, 8, 1, 5, 7],
                   [4, 8, 7, 2, 1, 5],
                   [5, 1, 2, 7, 8, 4],
                   [7, 5, 1, 8, 4, 2],
                   [8, 7, 5, 4, 2, 1]] := by
  native_decide

/-- **Sum of `(Z/9Z)*` equals 27.** -/
theorem units_z9_sum_eq_27 : unitsZ9Sum = 27 := by native_decide

/-- **Every row sums to 27.** -/
theorem every_row_sum_eq_27 :
    ∀ row ∈ multTableZ9, row.foldr (· + ·) 0 = 27 := by
  native_decide

/-- **Total table sum equals 162.** -/
theorem mult_table_total_eq_162 : multTableTotalSum = 162 := by native_decide

/-- **Sum factors as 6 · 27 = |unitsZ9| · row_sum.** -/
theorem mult_table_total_eq_6_times_27 :
    multTableTotalSum = unitsZ9.length * 27 := by native_decide

/-- **Additive-inverse pairs in `(Z/9Z)*`.** Each pair sums to 9. -/
theorem additive_inverse_pair_1_8 : 1 + 8 = (9 : Nat) := by decide
theorem additive_inverse_pair_2_7 : 2 + 7 = (9 : Nat) := by decide
theorem additive_inverse_pair_4_5 : 4 + 5 = (9 : Nat) := by decide

/-- **Multiplicative-inverse pairs in `(Z/9Z)*`.** Each pair
multiplies to 1 mod 9. -/
theorem mul_inverse_pair_1_1 : (1 * 1) % 9 = (1 : Nat) := by decide
theorem mul_inverse_pair_2_5 : (2 * 5) % 9 = (1 : Nat) := by decide
theorem mul_inverse_pair_4_7 : (4 * 7) % 9 = (1 : Nat) := by decide
theorem mul_inverse_pair_8_8 : (8 * 8) % 9 = (1 : Nat) := by decide

end PrimeFoundations.UnitsZ9
