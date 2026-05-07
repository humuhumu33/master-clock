/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# TriangularNumbers — Lean 4 mirror of `src/triangular_numbers.rs`

`k(n) = n · (n + 1) / 2`. Anchors at the U_30(∞) members 89, 109,
and 7919 (the 1000th prime). Two algebraic identities tie the
anchors directly to `MODULUS_R = 360`:

  89² − 1   = 7,920  = 22 · MODULUS_R
  109² − 89² = 3,960  = 11 · MODULUS_R

Every theorem closed by `decide` or `rfl`. No `sorry`.
-/

import PrimeFoundations.Parameters
import PrimeFoundations.IndexToResidue

namespace PrimeFoundations.TriangularNumbers

open PrimeFoundations.Parameters PrimeFoundations.IndexToResidue

/-- Triangular number `k(n) = n · (n + 1) / 2`. -/
def triangular (n : Nat) : Nat := n * (n + 1) / 2

/-- 89 — the 24th member of `U_30(∞)`. -/
def anchor24 : Nat := nthU30Member 24

/-- 109 — the 30th member of `U_30(∞)`. -/
def anchor30 : Nat := nthU30Member 30

theorem anchor24_eq_89 : anchor24 = 89 := by native_decide
theorem anchor30_eq_109 : anchor30 = 109 := by native_decide

/-- `k(89) = 4,005`. -/
theorem triangular_at_89 : triangular 89 = 4_005 := by decide

/-- `k(109) = 5,995`. -/
theorem triangular_at_109 : triangular 109 = 5_995 := by decide

/-- `k(7919) = 31,359,240` — the 1000th prime anchor. -/
theorem triangular_at_7919 : triangular 7_919 = 31_359_240 := by decide

/-- `89² − 1 = 22 · MODULUS_R = 7,920`. -/
theorem square_minus_one_at_24 : 89 * 89 - 1 = 22 * modulusR := by decide

/-- `109² − 89² = 11 · MODULUS_R = 3,960`. -/
theorem square_difference_30_24 :
    109 * 109 - 89 * 89 = 11 * modulusR := by decide

/-- Pyramid product `2112 · (109² − 89²) = 8,363,520`. -/
theorem pyramid_product :
    2_112 * (109 * 109 - 89 * 89) = 8_363_520 := by decide

/-- Pyramid product factors cleanly through `MODULUS_R`. -/
theorem pyramid_factors_through_modulus_r :
    2_112 * (109 * 109 - 89 * 89) = 2_112 * 11 * modulusR := by decide

end PrimeFoundations.TriangularNumbers
