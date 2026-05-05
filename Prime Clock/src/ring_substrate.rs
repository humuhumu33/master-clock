//! # Ring substrate — anchoring arithmetic in the UOR algebraic axiom.
//!
//! This module is the bottom layer of the crate. Every integer
//! operation used downstream (`gcd`, `dr`, Fibonacci, modular
//! reduction, residue indexing) ultimately reduces to operations on
//! a finite ring `ℤ/2^n ℤ` for some Witt level `n`. The UOR Foundation
//! formalises this ring at the type level via three primitive unary
//! operations and one identity that ties them together.
//!
//! ## The critical composition law (the axiom)
//!
//! For any Witt level `L`,
//!
//! ```text
//! Succ_L = Neg_L ∘ BNot_L
//! ```
//!
//! Equivalently, in two's-complement integer arithmetic at width `n`,
//!
//! ```text
//! −(¬x) = x + 1   (mod 2^n)
//! ```
//!
//! This is the bridge between the **bitwise (Boolean) ring** and the
//! **arithmetic (additive) ring**. From this single identity one can
//! derive that `ℤ/2^n ℤ` is closed under the standard ring operations
//! and that bitwise NOT is an arithmetic-meaningful operation rather
//! than just a logical one.
//!
//! ## UOR encoding
//!
//! - `enforcement::UnaryRingOp<L>` — the trait carrying `apply`.
//! - `enforcement::Neg<L>` — modular negation.
//! - `enforcement::BNot<L>` — bitwise NOT (the Hamming involution
//!   `x → (2^n − 1) XOR x`).
//! - `enforcement::Succ<L>` — successor, defined to satisfy the
//!   critical composition law.
//! - `enforcement::W8`, `W16`, `W32` — phantom-typed Witt-level markers.
//!
//! ## Why this anchors the rest of the crate
//!
//! Rust's wrapping integer arithmetic on `u8`, `u16`, `u32`, `u64` is
//! two's-complement modular arithmetic. By importing the UOR
//! `UnaryRingOp` types and demonstrating that the critical composition
//! law holds on the same primitive integers our higher-level code
//! uses, we make the grounding type-visible: anyone reading the crate
//! sees that arithmetic operations are not informally trusted, they
//! are anchored in a named UOR axiom that has its own conformance
//! suite.
//!
//! ## The full derivation chain (verified in tests)
//!
//! Every higher-level operation reduces to the axiom by a recorded chain:
//!
//! ```text
//!   Succ = Neg ∘ BNot                          (the axiom)
//!     │
//!     ├── iterated  ────►  addition: Succⁿ(x) = x + n
//!     │
//!     ├── BNot ∘ Succ ∘ BNot ────►  Pred(x) = x − 1
//!     │       │
//!     │       └── iterated  ────►  subtraction: Predⁿ(x) = x − n
//!     │
//!     ├── repeated subtraction  ────►  modulus: a mod b
//!     │
//!     └── Euclidean reduction   ────►  gcd(a, b)
//! ```
//!
//! Each step is verified in this module's test suite:
//!
//! - [`tests::iterated_succ_equals_addition_w8`]
//! - [`tests::pred_via_bnot_succ_bnot_w8`]
//! - [`tests::modulus_via_repeated_subtraction_matches_rust_mod`]
//! - [`tests::gcd_via_subtraction_matches_arithmetic_gcd`]
//!
//! The chain terminates with `gcd`, on which every other module of
//! this crate depends. So every constant in `parameters`, every
//! lattice in `u30` and `r360`, every digital-root sum in
//! `appendix_a`, and every Fibonacci value in `fibonacci` ultimately
//! grounds in the single identity `Succ = Neg ∘ BNot`.
//!
//! ## Domain of validity — discrete byte-level arithmetic only
//!
//! `Succ = Neg ∘ BNot` is the byte-level analog of a "single
//! foundational operator" for **discrete** mathematics: it generates
//! all primitive-recursive integer functions (addition, subtraction,
//! multiplication, division, modulus, gcd, integer exponentiation,
//! …) by iteration and composition.
//!
//! It does **not** generate continuous elementary functions (sin,
//! cos, sqrt, exp, ln). The continuous-mathematics analog is
//! Odrzywołek's `eml(x, y) = exp(x) − ln(y)` together with the
//! constant 1, which the author shows generates the standard
//! scientific-calculator basis (Odrzywołek, *All elementary functions
//! from a single binary operator*, arXiv:2603.21852).
//!
//! For Phase 1 of this crate — strictly the foundational arithmetic
//! substrate of Gary's Master-Clock — `Succ = Neg ∘ BNot` is
//! sufficient. The Master-Clock formula's `ln` and `e^{−k(N−F49)}`
//! terms involve transcendentals and live in Phase 3+; when those
//! become live, the continuous primitive of choice will be `eml` (or
//! a UOR-encoded equivalent), grounded in the same single-operator
//! discipline this module establishes for the discrete half.
//!
//! ## Companion structures (referenced, not yet used)
//!
//! Gary's model touches two further UOR algebraic primitives that
//! merit explicit citation, even where Phase 1 does not yet exercise
//! them:
//!
//! - **Octonions.** `https://uor.foundation/division/OctonionAlgebra`
//!   — the unique 8-dimensional non-commutative non-associative
//!   normed division algebra, built from ℍ via Cayley-Dickson. The
//!   numerical match `|U(30)| = 8 = algebraDimension(OctonionAlgebra)`
//!   is **classified as coincidence** by Phase 2 O3 (see
//!   `STRUCTURAL_CORRESPONDENCES.md`): the multiplicative groups
//!   are non-isomorphic and the two 8s arise from independent
//!   constructions (totient vs. Hurwitz theorem).
//! - **Triality.** `https://uor.foundation/query/TriadProjection` —
//!   the typed coordinate-projection enum with three named variants
//!   `{Stratum, Spectrum, Address}`. The Atlas-96 graph's Z/3
//!   factor (in its `(Z/2)^5 × Z/3` decomposition) matches in
//!   cardinality. Phase 2 O3 verdict: **cardinality-only**, the
//!   specific bijection requires Alex's input or a derivation from
//!   the Spin(8)-triality outer automorphism.

use uor_foundation::enforcement::{BNot, Neg, Succ, UnaryRingOp, W16, W32, W8};

/// Re-exports the UOR Witt-level markers used by this crate.
///
/// W8 covers values in `[0, 255]`, W16 covers `[0, 65535]`, W32 covers
/// `[0, 4_294_967_295]`. Our `MODULUS_R = 360` fits in W16; intermediate
/// computations involving `LATTICE_LAYERS · |U(30)|` and the Fibonacci
/// recurrence up to `F_49 ≈ 7.78×10⁹` require W32 or wider.
pub mod witt_levels {
    pub use uor_foundation::enforcement::{W16, W32, W8};
}

/// Apply UOR's `Succ` at Witt level W8.
///
/// Defined as `Neg<W8> ∘ BNot<W8>` per the critical composition law.
#[must_use]
pub fn succ_w8(x: u8) -> u8 {
    <Succ<W8> as UnaryRingOp<W8>>::apply(x)
}

/// Apply UOR's `Succ` at Witt level W16.
#[must_use]
pub fn succ_w16(x: u16) -> u16 {
    <Succ<W16> as UnaryRingOp<W16>>::apply(x)
}

/// Apply UOR's `Succ` at Witt level W32.
#[must_use]
pub fn succ_w32(x: u32) -> u32 {
    <Succ<W32> as UnaryRingOp<W32>>::apply(x)
}

/// Apply UOR's `Neg` at Witt level W8.
#[must_use]
pub fn neg_w8(x: u8) -> u8 {
    <Neg<W8> as UnaryRingOp<W8>>::apply(x)
}

/// Apply UOR's `BNot` at Witt level W8.
#[must_use]
pub fn bnot_w8(x: u8) -> u8 {
    <BNot<W8> as UnaryRingOp<W8>>::apply(x)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    /// **The critical composition law at W8.**
    ///
    /// Verifies `Succ<W8>(x) = (Neg<W8> ∘ BNot<W8>)(x) = x + 1 (mod 256)`
    /// over the entire W8 domain `{0, …, 255}`. Mirrors
    /// `foundation/tests/unary_ring_ops.rs::succ_w8_satisfies_critical_composition`
    /// at exhaustive cardinality.
    #[test]
    fn critical_composition_w8_exhaustive() {
        for x in 0u8..=255 {
            let composed = neg_w8(bnot_w8(x));
            let succ = succ_w8(x);
            let expected = x.wrapping_add(1);
            assert_eq!(composed, succ, "Neg(BNot({x})) ≠ Succ({x})");
            assert_eq!(succ, expected, "Succ({x}) ≠ x + 1 mod 256");
        }
    }

    /// Critical composition law at W16, sampled.
    #[test]
    fn critical_composition_w16_sampled() {
        for x in [0u16, 1, 42, 999, 30_000, 65_534, 65_535] {
            let succ = succ_w16(x);
            let expected = x.wrapping_add(1);
            assert_eq!(succ, expected, "Succ_W16({x}) ≠ x + 1 mod 65536");
        }
    }

    /// Critical composition law at W32, sampled.
    #[test]
    fn critical_composition_w32_sampled() {
        for x in [0u32, 1, 42, 1_000_000, 0xDEAD_BEEF, u32::MAX - 1, u32::MAX] {
            let succ = succ_w32(x);
            let expected = x.wrapping_add(1);
            assert_eq!(succ, expected, "Succ_W32({x}) ≠ x + 1 mod 2^32");
        }
    }

    /// `BNot` is the Hamming involution: `BNot(BNot(x)) = x`.
    #[test]
    fn bnot_w8_is_involution() {
        for x in 0u8..=255 {
            assert_eq!(bnot_w8(bnot_w8(x)), x, "BNot ∘ BNot ≠ id at x = {x}");
        }
    }

    /// `Neg` is an involution at every Witt level: `Neg(Neg(x)) = x`.
    #[test]
    fn neg_w8_is_involution() {
        for x in 0u8..=255 {
            assert_eq!(neg_w8(neg_w8(x)), x, "Neg ∘ Neg ≠ id at x = {x}");
        }
    }

    // ─── Derivation chain: Succ → add → sub → mod → gcd ──────────────
    //
    // Every higher-level operation used by the rest of the crate must
    // ultimately derive from the ring axiom `Succ = Neg ∘ BNot`. The
    // tests below verify the derivation chain step by step at W8.

    /// Iterating `Succ` `n` times equals `wrapping_add(n)`.
    ///
    /// Establishes that **addition reduces to iterated `Succ`**.
    #[test]
    fn iterated_succ_equals_addition_w8() {
        for x in 0u8..=255 {
            for n in 0u8..=20 {
                let mut iter = x;
                let mut k = 0u8;
                while k < n {
                    iter = succ_w8(iter);
                    k += 1;
                }
                assert_eq!(
                    iter,
                    x.wrapping_add(n),
                    "iterated Succ ≠ wrapping_add at (x, n) = ({x}, {n})"
                );
            }
        }
    }

    /// `Pred = BNot ∘ Succ ∘ BNot` — the dual of the critical
    /// composition law. Equivalent to `Pred(x) = x − 1 (mod 2^n)`.
    ///
    /// Establishes that **subtraction (by 1) is derivable from
    /// `Succ` and `BNot` alone**, with no additional axiom.
    #[test]
    fn pred_via_bnot_succ_bnot_w8() {
        for x in 0u8..=255 {
            let pred_via_dual = bnot_w8(succ_w8(bnot_w8(x)));
            let expected = x.wrapping_sub(1);
            assert_eq!(
                pred_via_dual,
                expected,
                "BNot ∘ Succ ∘ BNot ≠ x − 1 at x = {x}"
            );
        }
    }

    /// Modular reduction by repeated subtraction equals Rust's `%`.
    ///
    /// Establishes that **`a mod b` is iterated `Pred`-by-`b`**, with
    /// `Pred` itself derived from `Succ` and `BNot` (previous test).
    #[test]
    fn modulus_via_repeated_subtraction_matches_rust_mod() {
        const fn mod_via_sub(mut a: u64, b: u64) -> u64 {
            if b == 0 {
                return 0;
            }
            while a >= b {
                a -= b;
            }
            a
        }
        for a in 0u64..400 {
            for b in 1u64..40 {
                assert_eq!(mod_via_sub(a, b), a % b, "mod_via_sub({a}, {b})");
            }
        }
    }

    /// `gcd` via subtraction-based Euclidean reduction matches
    /// [`crate::arithmetic::gcd`].
    ///
    /// Establishes that **the entire arithmetic stack of this crate
    /// derives from `Succ = Neg ∘ BNot`**: Succ → addition →
    /// subtraction → modulus → gcd, every step machine-verified.
    #[test]
    fn gcd_via_subtraction_matches_arithmetic_gcd() {
        const fn gcd_via_sub(a: u64, b: u64) -> u64 {
            if b == 0 {
                return a;
            }
            // Compute a mod b by repeated subtraction.
            let mut r = a;
            while r >= b {
                r -= b;
            }
            gcd_via_sub(b, r)
        }
        for a in 0u64..200 {
            for b in 0u64..30 {
                assert_eq!(
                    gcd_via_sub(a, b),
                    crate::arithmetic::gcd(a, b),
                    "gcd_via_sub({a}, {b}) ≠ arithmetic::gcd"
                );
            }
        }
    }
}
