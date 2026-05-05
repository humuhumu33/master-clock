//! # Parameters of Gary's Master-Clock model
//!
//! Gary William Croft's *Toroidal Prime Counting Master-Clock* (FIXED
//! revision) rests on a small number of **irreducible parameters**;
//! every other constant in the system follows from these by standard
//! number theory. This module makes that derivation chain explicit.
//!
//! ## Derivation graph
//!
//! ```text
//!     MODULUS_U = 30   (Gary's residue modulus, = 2·3·5)
//!     NUMERAL_BASE = 10  (decimal digital root)
//!         │
//!         ├──► MODULUS_DR = NUMERAL_BASE − 1 = 9
//!         │       │
//!         │       └──► PISANO_PERIOD_MOD_DR = 24
//!         │            (theorem; verified in `fibonacci`)
//!         │
//!         ├──► U30_CARDINALITY = φ(MODULUS_U) = 8
//!         │
//!         └──► MODULUS_R = lcm(MODULUS_U, MODULUS_DR, PISANO_PERIOD_MOD_DR)
//!                        = lcm(30, 9, 24) = 360
//!                  │
//!                  ├──► LATTICE_LAYERS = MODULUS_R / MODULUS_U = 12
//!                  │
//!                  └──► R_CARDINALITY = U30_CARDINALITY · LATTICE_LAYERS = 96
//! ```
//!
//! ## Why these parameters
//!
//! `MODULUS_U = 30` is Gary's stated choice (FIXED PDF §II): the
//! "96-element U(30) matrix." Smaller choices (6 = 2·3) yield only
//! `φ(6) = 2` residues; larger (210 = 2·3·5·7) yields `φ(210) = 48`,
//! exceeding the SDK macro depth budget. 30 is the smallest non-trivial
//! product of primes giving a sieve with eight equally-weighted classes.
//!
//! `NUMERAL_BASE = 10` is the decimal numeral system. Gary's "digital
//! root" is the conventional base-10 iterated digit sum, equivalent to
//! `n mod 9` mapped onto `{1, …, 9}`.
//!
//! Every other parameter in this crate is **derived** from these two
//! plus standard arithmetic.
//!
//! ## Compile-time guarantees
//!
//! The compile-time assertions at the bottom of this module enforce
//! that the derived constants equal the values Gary states in the
//! FIXED PDF. If anyone changes a parameter and the math no longer
//! matches Gary's claims, the build fails.

use crate::arithmetic::{euler_phi, lcm};

// ─── Irreducible parameters ───────────────────────────────────────────

/// **Parameter (Gary's choice, FIXED PDF §II).**
///
/// The residue modulus. `MODULUS_U = 30 = 2 · 3 · 5`.
pub const MODULUS_U: u64 = 30;

/// **Parameter (decimal numeral system).**
///
/// The base of the numeral system used for digital roots.
pub const NUMERAL_BASE: u64 = 10;

// ─── Derived: digital-root arithmetic ────────────────────────────────

/// **Derived.**
///
/// The modulus of digital-root arithmetic.
/// `MODULUS_DR = NUMERAL_BASE − 1 = 9`.
pub const MODULUS_DR: u64 = NUMERAL_BASE - 1;

/// **Theorem (verified in [`crate::fibonacci`]).**
///
/// The Pisano period of Fibonacci modulo `MODULUS_DR`.
/// For `MODULUS_DR = 9`, the period is 24.
pub const PISANO_PERIOD_MOD_DR: u64 = 24;

// ─── Derived: U(M_U) cardinality ─────────────────────────────────────

/// **Derived.**
///
/// `|U(MODULUS_U)| = φ(MODULUS_U)`. For `MODULUS_U = 30`,
/// `|U(30)| = φ(30) = φ(2) · φ(3) · φ(5) = 1 · 2 · 4 = 8`.
pub const U30_CARDINALITY: usize = euler_phi(MODULUS_U) as usize;

// ─── Derived: torus modulus and lattice ──────────────────────────────

/// **Derived.**
///
/// The torus modulus `M_R` simultaneously respecting U(MODULUS_U)
/// residue classes, mod-`MODULUS_DR` digital roots, and the
/// `MODULUS_DR`-Pisano period.
///
/// `MODULUS_R = lcm(MODULUS_U, MODULUS_DR, PISANO_PERIOD_MOD_DR)
///            = lcm(30, 9, 24) = 360`.
pub const MODULUS_R: u64 = lcm(lcm(MODULUS_U, MODULUS_DR), PISANO_PERIOD_MOD_DR);

/// **Derived.**
///
/// Number of `MODULUS_U`-blocks contained in `[1, MODULUS_R]`.
/// `LATTICE_LAYERS = MODULUS_R / MODULUS_U = 360 / 30 = 12`.
pub const LATTICE_LAYERS: u64 = MODULUS_R / MODULUS_U;

/// **Derived.**
///
/// Cardinality of the residue lattice `R(MODULUS_R) = { n ∈ [1, MODULUS_R]
/// : gcd(n, MODULUS_U) = 1 }`. By stratification,
/// `|R(MODULUS_R)| = |U(MODULUS_U)| · LATTICE_LAYERS = 8 · 12 = 96`.
pub const R_CARDINALITY: usize = U30_CARDINALITY * (LATTICE_LAYERS as usize);

// ─── Compile-time enforcement of Gary's stated values ────────────────

// If any of these break, either Gary's claim from the FIXED PDF is
// inconsistent with our derivation, or a parameter has been edited.
// Either way the build fails and the discrepancy must be addressed.

const _: () = assert!(MODULUS_U == 30, "Gary's MODULUS_U = 30 (FIXED PDF §II)");
const _: () = assert!(MODULUS_DR == 9, "Decimal digital roots use modulus 9");
const _: () = assert!(PISANO_PERIOD_MOD_DR == 24, "Pisano(9) = 24");
const _: () = assert!(U30_CARDINALITY == 8, "Gary's |U(30)| = 8 (FIXED PDF Appendix A)");
const _: () = assert!(MODULUS_R == 360, "Gary's MODULUS_R = 360 (FIXED PDF Appendix A)");
const _: () = assert!(LATTICE_LAYERS == 12, "Gary's 12 layers (FIXED PDF Appendix A)");
const _: () = assert!(R_CARDINALITY == 96, "Gary's 96-room lattice (FIXED PDF §II)");

#[cfg(test)]
mod tests {
    use super::*;

    /// Spot-checks that each derivation step matches Gary's stated values.
    ///
    /// (The compile-time `const _: () = assert!(…)` block above is the
    /// load-bearing check; this test is documentation.)
    #[test]
    fn derivations_match_gary_stated_values() {
        assert_eq!(MODULUS_U, 30);
        assert_eq!(MODULUS_DR, 9);
        assert_eq!(PISANO_PERIOD_MOD_DR, 24);
        assert_eq!(U30_CARDINALITY, 8);
        assert_eq!(MODULUS_R, 360);
        assert_eq!(LATTICE_LAYERS, 12);
        assert_eq!(R_CARDINALITY, 96);
    }
}
