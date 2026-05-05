/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# Prime Foundations — Lean 4 mirror of `toroidal-prime-clock`

The Lean 4 formalisation of the foundational arithmetic substrate for
Gary William Croft's *Toroidal Prime Counting Master-Clock*.

Mirrors the Rust crate `toroidal-prime-clock` in `src/` and follows
the discipline of `atlas-embeddings/lean4`:

- **NO `sorry`** — every theorem closes by `decide`, `rfl`,
  `norm_num`, or by structural induction over the small finite
  domains involved.
- Exact arithmetic (`ℤ`, `ℚ`) only — no `ℝ` until Phase 3 brings
  in the Cauchy-rational analytic stack documented in `BRIDGE.md`.
- Every constant is **derived** from `MODULUS_U = 30` and
  `NUMERAL_BASE = 10` via `Nat.gcd`, `Nat.lcm`, and the Pisano
  recurrence.

## Top-level imports

Phase 1+2 modules form the foundational substrate. Phase 3 modules
extend the discrete substrate with the BRIDGE.md Layer 4–7
constructions: exact rationals, Cauchy<ℚ> with explicit modulus,
truncated Taylor/Mercator partial sums, and the `eml` composition.
-/

-- Phase 1+2: discrete foundations
import PrimeFoundations.Parameters
import PrimeFoundations.Fibonacci
import PrimeFoundations.U30
import PrimeFoundations.R360
import PrimeFoundations.AppendixA
import PrimeFoundations.AtlasCorrespondence

-- Phase 3: BRIDGE.md Layers 4–7 (alloc-only Rust paths mirrored in Lean)
import PrimeFoundations.Rational
import PrimeFoundations.Cauchy
import PrimeFoundations.Elementary

-- Phase 3 O7: empirical reference + Gary's claim, with the Phase 4
-- structural-anchor falsifiers (deltas {432, 96} ↔ {sumDR, rCardinality}).
import PrimeFoundations.Li

-- Phase 4 partial: D(N), the Master-Clock lattice tally per Gary's
-- Developer's Guide v1.1 (chat, 2026-05-04). The first computable
-- component of `π(N) = D(N) − TF(N) + RF(N) + 3` to land in code.
import PrimeFoundations.DFunction
