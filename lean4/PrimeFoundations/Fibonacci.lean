/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# Fibonacci numbers, Pisano period mod 9, F_49, and the
  Fibonacci-digital-root palindrome.

Mirrors `src/fibonacci.rs`. Phase 2 — Lean side.

**Builds on:** `PrimeFoundations.Parameters` for `pisanoPeriodModDR`,
`modulusDR`, and the irreducible parameters. Mirrors the Phase 1
Rust tests `pisano_period_mod_dr_closes_at_24`,
`pisano_period_mod_dr_is_minimal`, `f49_equals_gary_stated`,
`fibonacci_dr_palindrome_over_u24`, and the structural sum
`sum_dr_fib_over_u24`.

**NO `sorry` POLICY** — every theorem closed by `rfl`, `decide`,
`norm_num`, `interval_cases`, `fin_cases`, or constructive proof.
-/

import PrimeFoundations.Parameters
import Mathlib.Data.Nat.Fib.Basic

namespace PrimeFoundations.Fibonacci

open PrimeFoundations.Parameters

/-! ## Fibonacci modulo `MODULUS_DR` and the digital root of `F_n` -/

/-- `F_n mod MODULUS_DR` (i.e., mod 9). Uses mathlib's `Nat.fib`
under the hood; mirror of `crate::fibonacci::fib_mod_dr`. -/
def fibModDR (n : Nat) : Nat := Nat.fib n % modulusDR

/-- Digital root of `F_n`.

`dr(0) = 0` by convention (no positive `dr` exists for `0`);
otherwise the unique value in `{1, …, 9}` congruent to `F_n`
modulo 9. Mirror of `crate::fibonacci::dr_fib`. -/
def drFib (n : Nat) : Nat :=
  if n = 0 then 0
  else
    let v := fibModDR n
    if v = 0 then 9 else v

/-! ## `F_49` — the F49 ratchet anchor (FIXED PDF §III) -/

/-- Gary's stated value for `F_49` (FIXED PDF §III): `7,778,742,049`.
Mirror of `crate::fibonacci::F49_STATED`. -/
def f49Stated : Nat := 7_778_742_049

/-- `F_49` computed from the standard Fibonacci recurrence via
mathlib's `Nat.fib`. Mirror of `crate::fibonacci::F49_COMPUTED`. -/
def f49Computed : Nat := Nat.fib 49

/-- **Theorem.** `F_49 = 7,778,742,049`. The FIXED PDF's stated
anchor matches the recurrence value to all decimal digits.

This is the Lean mirror of the compile-time
`const _: () = assert!(F49_COMPUTED == F49_STATED)` in
`src/fibonacci.rs`. -/
theorem f49_equals_gary_stated : f49Computed = f49Stated := by
  unfold f49Computed f49Stated
  rfl

/-! ## Pisano period mod 9 -/

/-- **Theorem.** `F_24 ≡ 0 (mod 9)` and `F_25 ≡ 1 (mod 9)` — the
Pisano period closes at 24. Mirror of
`crate::fibonacci::pisano_period_mod_dr_closes_at_24`. -/
theorem pisano_period_mod_dr_closes_at_24 :
    fibModDR pisanoPeriodModDR = 0 ∧
    fibModDR (pisanoPeriodModDR + 1) = 1 := by
  unfold pisanoPeriodModDR
  decide

/-- **Theorem.** No `p` strictly between 0 and `pisanoPeriodModDR`
satisfies the period-closing condition
`(F_p ≡ 0 ∧ F_{p+1} ≡ 1) (mod 9)`. Therefore 24 is the **minimal**
Pisano period mod 9.

Mirror of `crate::fibonacci::pisano_period_mod_dr_is_minimal`. The
Rust test loops `p in 1..PISANO_PERIOD_MOD_DR` and asserts the
negation; `interval_cases` does the same exhaustive case-split here. -/
theorem pisano_period_mod_dr_is_minimal (p : Nat)
    (h₁ : 1 ≤ p) (h₂ : p < pisanoPeriodModDR) :
    ¬ (fibModDR p = 0 ∧ fibModDR (p + 1) = 1) := by
  unfold pisanoPeriodModDR at h₂
  interval_cases p <;> decide

/-! ## Fibonacci-digital-root palindrome over `U(24)` -/

/-- `U(24) = {1, 5, 7, 11, 13, 17, 19, 23}` enumerated as a Lean list.

The eight integers in `[1, 24)` coprime to 24, equivalently the
units of `ℤ/24ℤ`. All are odd because `2 ∣ 24`. -/
def U24 : List Nat := [1, 5, 7, 11, 13, 17, 19, 23]

/-- **Palindrome theorem.** `dr(F_u) = dr(F_{24 − u})` for every
`u ∈ U(24)`.

This is the genuine non-trivial mathematical content underpinning
the `Σ dr(F_n) = 432` invariant on Appendix A column 3.

**Theoretical proof** (one line, from two textbook identities):

```
F_{24-u}  ≡  F_{-u}            (Pisano period mod 9 = 24)
F_{-n}    =  (-1)^{n+1} F_n    (Vajda identity 49 / Koshy §5)
u odd for u ∈ U(24)            (since 2 ∣ 24, gcd(u, 2) = 1)
⟹  F_{24-u}  ≡  +F_u  (mod 9)
⟹  dr(F_{24-u}) = dr(F_u)     ∎
```

The Lean proof below is the **case-enumeration** version: eight
subgoals from `fin_cases hu`, each closed by `decide` on the
specific `drFib u = drFib (24 − u)` arithmetic equality.

Phase 2-bis target: replace this with the constructive proof above
once mathlib's integer-index Fibonacci extension is available, or
contribute the negative-index identity upstream. The case-enumeration
version is zero-sorry today and unblocks `AppendixA.lean`'s 432
derivations. -/
theorem fibonacci_dr_palindrome (u : Nat) (hu : u ∈ U24) :
    drFib u = drFib (pisanoPeriodModDR - u) := by
  unfold pisanoPeriodModDR
  fin_cases hu <;> decide

/-! ## `Σ dr(F_u) over U(24) = 36` -/

/-- **Theorem.** The eight digital roots of Fibonacci values at
`U(24)` indices sum to 36.

```
dr(F_1) + dr(F_5) + dr(F_7) + dr(F_11)
  + dr(F_13) + dr(F_17) + dr(F_19) + dr(F_23)
= 1 + 5 + 4 + 8 + 8 + 4 + 5 + 1
= 36
```

Combined with the multiplicity-12 hit pattern of `R(360) mod 24`
(every U(24) class contains exactly `LATTICE_LAYERS = 12` elements
of R(360)), this gives the Phase 1 structural derivation
`Σ dr(F_n) over R(360) = 12 × 36 = 432`. -/
theorem sum_dr_fib_over_u24 : (U24.map drFib).sum = 36 := by
  decide

end PrimeFoundations.Fibonacci
