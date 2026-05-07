# Prime Clock

[![license](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.83%2B-orange.svg)](https://www.rust-lang.org)
[![Lean 4](https://img.shields.io/badge/Lean%204-zero%20sorry-success.svg)](./lean4)

A strict Rust + Lean 4 encoding of Gary William Croft's published
**Prime Spiral Sieve** (primesdemystified.com), anchored in the UOR
(Universal Object Reference) Framework.

Two parameters in:

```
MODULUS_U     = 30      (Gary's residue modulus, = 2·3·5)
NUMERAL_BASE  = 10      (decimal numeral system)
```

Every constant, every theorem, every numerical claim in this crate
derives from those two values via standard number theory (gcd, lcm,
totient, digital root, Fibonacci recurrence, finite enumeration).
Anything that does not derive has been removed. See
[`RESULTS.md`](./RESULTS.md) for the canonical evidence and
[`sheets_findings.md`](../sheets_findings.md) /
[`primesdemystified_scrape.md`](../primesdemystified_scrape.md) for
the audit.

## Headline result

The wheel-30 sieve produces canonical `π(N)` exactly at every tested
decimal power:

```
k=4   π(N)=1,229          (0.000 s)
k=5   π(N)=9,592          (0.000 s)
k=6   π(N)=78,498         (0.001 s)
k=7   π(N)=664,579        (0.007 s)
k=8   π(N)=5,761,455      (0.089 s)
k=9   π(N)=50,847,534     (1.159 s)
k=10  π(N)=455,052,511    (17.65 s)
```

All match OEIS A006880 byte-for-byte. No χ, no ζ, no δ, no k, no
residual.

## Reproduce

Standalone (no toolchain beyond `rustc`):

```sh
rustc -O validate_sieve.rs -o validate_sieve && ./validate_sieve
rustc -O validate_units_z9.rs -o validate_units_z9 && ./validate_units_z9
```

Full crate:

```sh
cargo test --features alloc
cd lean4 && lake build
```

## What is in scope

Three layers, all anchored in `MODULUS_U = 30` and `NUMERAL_BASE = 10`:

1. **Phase 1 substrate.** `U(30)`, `R(360)`, Pisano period mod 9,
   `F_49`, the 96-row Appendix A, two `432` invariants, the
   Atlas-96 mirror, Dirichlet equidistribution.
2. **Phase 1.5 — `(Z/9Z)*` substrate.** `U_30(90)` (24 elements coprime
   to 30 in `[1, 90]`), the period-24 digital-root cycle, `(Z/9Z)*`
   cyclic of order 6, the 6×6 multiplication table mod 9, the 24×24
   mod-90 multiplication-matrix row sum and diagonal residues.
3. **Phase 2 — wheel-30 sieve.** `π(N) = #{primes in U(30) ≤ N} + 3`
   = `D₃₀(N) − C₃₀(N) + 2`, validated against OEIS A006880 up to
   `10¹⁰`.

## What is out of scope (and why)

The Toroidal Master-Clock identity `π(N) = D(N) − TF(N) + RF(N) + 3`
and its calibration constants χ, ζ, δ, k:

- Do not appear on primesdemystified.com.
- Do not appear in any of Gary's four published spreadsheets (all
  formulas in those sheets are pure integer arithmetic — no logs, no
  exponentials, no transcendental constants).
- Were iterated five times in private correspondence; in each
  iteration the stated formulas did not arithmetically reproduce
  their own tabulated trace values.

When asked for a closed-form `TF(N)` and `TU_Divisor(N)` that would
produce his three claimed residuals `{±112, ±432, ±96}` from any
deterministic computation, no answer reconciled. This crate
therefore treats them as out of scope. The strict Phase 1 substrate
plus the published wheel-30 sieve gives `π(N)` exactly and is
verifiable from first principles.

## Discipline

- `#![no_std]` core, `#![forbid(unsafe_code)]`
- `clippy::float_arithmetic = "deny"` — exact integer or rational only
- All clippy warnings as errors
- Every Rust constant in `const fn` with `const _: () = assert!(…)` cross-check
- Every Lean theorem closed by `decide`, `native_decide`, `rfl`, or constructive proof
- Zero `sorry`

## License

Apache-2.0. See [`LICENSE`](./LICENSE).
