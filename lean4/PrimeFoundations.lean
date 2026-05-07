/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# Prime Foundations — Lean 4 mirror of `toroidal-prime-clock`

Lean 4 formalisation of the Rust crate `toroidal-prime-clock`.
Every theorem derives from the two irreducible parameters
`MODULUS_U = 30` and `NUMERAL_BASE = 10` via standard number theory.

- **NO `sorry`** — every theorem closes by `decide`, `native_decide`,
  `rfl`, or constructive proof.
- Integer arithmetic only; no real-valued analysis.
- Every constant is **derived** from the two parameters via
  `Nat.gcd`, `Nat.lcm`, the Fibonacci recurrence, or finite
  enumeration.

## Top-level imports

The 10 modules below form a strict Phase 1 substrate:

- Foundations: `Parameters`, `Fibonacci`
- Substrate: `U30`, `R360`, `U90`, `UnitsZ9`, `AppendixA`, `AtlasCorrespondence`
- Sieve: `PrimeCount`
- Reference: `Li`
-/

import PrimeFoundations.Parameters
import PrimeFoundations.Fibonacci
import PrimeFoundations.U30
import PrimeFoundations.R360
import PrimeFoundations.U90
import PrimeFoundations.UnitsZ9
import PrimeFoundations.Twi