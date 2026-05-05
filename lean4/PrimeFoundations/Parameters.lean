/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# Parameters — Lean 4 mirror of `src/parameters.rs`

Two irreducible parameters; everything else derived. Mirrors the
Rust derivation chain at compile time, with each derivation closed
by `decide` or `rfl` (no `sorry`).
-/

import Mathlib.Data.Nat.GCD.Basic
import Mathlib.Data.Nat.Totient

namespace PrimeFoundations.Parameters

/-! ## Parameters (irreducible) -/

/-- Gary's chosen residue modulus, FIXED PDF §II. -/
def modulusU : Nat := 30

/-- The decimal numeral system. -/
def numeralBase : Nat := 10

/-! ## Derived constants -/

/-- The mod-9 modulus for digital roots. Derived. -/
def modulusDR : Nat := numeralBase - 1

/-- The Pisano period of Fibonacci modulo 9.
    Verified empirically in `Fibonacci.lean` (Phase 2). -/
def pisanoPeriodModDR : Nat := 24

/-- The torus modulus, simultaneously respecting U(30) residues,
    mod-9 digital roots, and the mod-9 Pisano period. -/
def modulusR : Nat := Nat.lcm (Nat.lcm modulusU modulusDR) pisanoPeriodModDR

/-- Number of MODULUS_U-blocks contained in [1, MODULUS_R]. -/
def latticeLayers : Nat := modulusR / modulusU

/-- Cardinality |U(MODULUS_U)| = φ(MODULUS_U). -/
def u30Cardinality : Nat := Nat.totient modulusU

/-- Cardinality of R(MODULUS_R). Derived as |U| · layers. -/
def rCardinality : Nat := u30Cardinality * latticeLayers

/-! ## Theorems (zero sorry) -/

/-- `MODULUS_DR = 9`. -/
theorem modulusDR_eq_9 : modulusDR = 9 := by rfl

/-- `MODULUS_R = 360`. Derived via `lcm(30, 9, 24) = 360`. -/
theorem modulusR_eq_360 : modulusR = 360 := by decide

/-- `LATTICE_LAYERS = 12`. -/
theorem latticeLayers_eq_12 : latticeLayers = 12 := by decide

/-- `|U(30)| = φ(30) = 8`. -/
theorem u30Cardinality_eq_8 : u30Cardinality = 8 := by decide

/-- `|R(360)| = 96`. -/
theorem rCardinality_eq_96 : rCardinality = 96 := by decide

/-- The `R(360)` cardinality matches Gary's stated count from
    Appendix A. -/
theorem rCardinality_matches_fixed_pdf : rCardinality = 96 :=
  rCardinality_eq_96

end PrimeFoundations.Parameters
