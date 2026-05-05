//! # `R(360)` — the 96-element residue lattice modulo `MODULUS_R`
//!
//! `R(360) = { n ∈ [1, MODULUS_R] : gcd(n, MODULUS_U) = 1 }`. Cardinality
//! `|R(360)| = R_CARDINALITY = 96`.
//!
//! ## What these 96 integers are (terminology)
//!
//! `R(360)` is the set of **irreducibles modulo `MODULUS_U = 30`**
//! within `[1, MODULUS_R]` — the integers coprime to 30, equivalently
//! the units of `(ℤ/30)*` lifted across `LATTICE_LAYERS = 12` lattice
//! layers. They are **not** primes. `R(360)` contains composite
//! integers like `49 = 7²`, `77 = 7·11`, `91 = 7·13`, etc., wherever
//! the composite happens to be coprime to 30.
//!
//! In the Master-Clock framing these are the *carriers of potential
//! resonance* — the addressable rooms where primes can live, not the
//! primes themselves. Gary's `D(N)` algorithm
//! ([`crate::d_function`]) walks these slots one by one and sums
//! Fibonacci digital-root weights at each; primality is never tested,
//! only modular position is.
//!
//! ## Bijection to `U(30) × Z/12`
//!
//! Every `n ∈ R(360)` decomposes uniquely as `n = u + MODULUS_U · k`
//! where `u ∈ U(30)` and `k ∈ {0, …, LATTICE_LAYERS − 1} = {0, …, 11}`.
//! Hence the **set bijection**
//!
//! ```text
//! R(360)  ←→  U(30) × Z/12
//!     n   ↔   (n mod MODULUS_U, n div MODULUS_U)
//! ```
//!
//! [`decompose`] and [`compose`] implement the bijection in both
//! directions. [`ELEMENTS`] is the canonical ascending list of the 96
//! integers in `R(360)`, computed at compile time from
//! [`crate::parameters::MODULUS_R`] and [`crate::arithmetic::gcd`].
//!
//! ## Algebraic substrate
//!
//! Every integer operation in this module — `gcd`, `mod`, integer
//! division, addition — reduces ultimately to applications of UOR's
//! [`crate::ring_substrate`] primitives. Specifically, the additive
//! recurrence `succ(n) = n + 1` used to enumerate the lattice is the
//! `Succ = Neg ∘ BNot` axiom from
//! `foundation/src/enforcement.rs:9894`. Multiplication by
//! `MODULUS_U = 30` is `MODULUS_U` repeated applications of `Succ`,
//! and `gcd` is the Euclidean algorithm, which reduces to subtraction
//! (`Pred = Succ⁻¹` per the same composition law).
//!
//! ## UOR shape encoding
//!
//! The **structural** representation of `R(360)` is the Cartesian
//! product
//!
//! ```text
//! R360 = U30 ⊠ Z12   (uor_foundation_sdk::cartesian_product_shape!)
//! ```
//!
//! where `Z12` is the coproduct of 12 leaf shapes, each carrying a
//! single `ConstraintRef::Residue { modulus: LATTICE_LAYERS, residue: k }`
//! for `k ∈ {0, …, 11}`.
//!
//! Cartesian product has `SITE_COUNT = SITE_COUNT(U30) + SITE_COUNT(Z12) = 4 + 5 = 9`.
//!
//! ## Connection to triality (Phase 2 O3 verdict)
//!
//! `Z/12 ≅ Z/4 × Z/3` by the Chinese Remainder Theorem (since
//! `gcd(4, 3) = 1`). The `Z/3` factor of this decomposition has the
//! same cardinality as Atlas-96's `(Z/2)⁵ × Z/3` Z/3 factor and as
//! UOR's typed coordinate-projection enum
//! `https://uor.foundation/query/TriadProjection` (three variants:
//! `{Stratum, Spectrum, Address}`).
//!
//! Phase 2 O3 verdict (see `STRUCTURAL_CORRESPONDENCES.md`):
//! **cardinality-only match**. All three sides carry real,
//! typed Z/3 structure but the specific canonical bijection
//! between `Z/3 ↔ d₄₅ ∈ {−1, 0, +1} ↔ {Stratum, Spectrum, Address}`
//! is not yet documented; closing the verdict to "proven structural"
//! requires Alex's input or a derivation from the Spin(8)-triality
//! outer automorphism. Phase 1 encodes `Z12` directly as a 12-leaf
//! coproduct; the CRT decomposition into `Z4 × Z3` is verified in
//! the test [`tests::z12_decomposes_via_crt`].

use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef};
use uor_foundation_sdk::{cartesian_product_shape, coproduct_shape};

use crate::arithmetic::gcd;
use crate::parameters::{LATTICE_LAYERS, MODULUS_R, MODULUS_U, R_CARDINALITY, U30_CARDINALITY};
use crate::u30::{RESIDUES, U30};

// ─── Derived: the 96 integers ────────────────────────────────────────

/// The 96 integers of `R(360)` in strictly ascending order.
///
/// **Derived** by enumerating `n ∈ [1, MODULUS_R]` and selecting those
/// with `gcd(n, MODULUS_U) = 1`. Cardinality enforced by the array
/// type `[u32; R_CARDINALITY]`.
pub const ELEMENTS: [u32; R_CARDINALITY] = compute_elements();

const fn compute_elements() -> [u32; R_CARDINALITY] {
    let mut out = [0u32; R_CARDINALITY];
    let mut idx = 0usize;
    let mut n: u64 = 1;
    while n <= MODULUS_R {
        if gcd(n, MODULUS_U) == 1 {
            #[allow(clippy::cast_possible_truncation)]
            {
                out[idx] = n as u32;
            }
            idx += 1;
        }
        n += 1;
    }
    out
}

// Compile-time check: exactly R_CARDINALITY non-zero entries.
const _: () = {
    let mut count = 0usize;
    let mut i = 0usize;
    while i < ELEMENTS.len() {
        if ELEMENTS[i] != 0 {
            count += 1;
        }
        i += 1;
    }
    assert!(count == R_CARDINALITY, "compute_elements emitted wrong row count");
};

// ─── Bijection R(360) ↔ U(30) × {0..11} ──────────────────────────────

/// Decompose `n ∈ R(360)` into `(u_index, layer)` such that
/// `n = RESIDUES[u_index] + MODULUS_U · layer`.
///
/// Returns `None` if `n` is not coprime to `MODULUS_U` (and hence not
/// in `R(360)`) or if `n > MODULUS_R`.
#[must_use]
pub const fn decompose(n: u32) -> Option<(usize, u32)> {
    let n64 = n as u64;
    if n64 < 1 || n64 > MODULUS_R {
        return None;
    }
    if gcd(n64, MODULUS_U) != 1 {
        return None;
    }
    let u_val = n64 % MODULUS_U;
    #[allow(clippy::cast_possible_truncation)]
    let layer = (n64 / MODULUS_U) as u32;

    // Linear search RESIDUES for u_val. With U30_CARDINALITY = 8 this
    // is O(8); evaluated at compile time when const-folded.
    let mut idx = 0usize;
    while idx < U30_CARDINALITY {
        if RESIDUES[idx] == u_val {
            return Some((idx, layer));
        }
        idx += 1;
    }
    None
}

/// Compose `(u_index, layer)` into the integer
/// `RESIDUES[u_index] + MODULUS_U · layer`.
///
/// Returns `None` if `u_index >= U30_CARDINALITY` or
/// `layer >= LATTICE_LAYERS`.
#[must_use]
pub const fn compose(u_index: usize, layer: u32) -> Option<u32> {
    if u_index >= U30_CARDINALITY {
        return None;
    }
    if (layer as u64) >= LATTICE_LAYERS {
        return None;
    }
    let u = RESIDUES[u_index];
    let n = u + MODULUS_U * (layer as u64);
    #[allow(clippy::cast_possible_truncation)]
    {
        Some(n as u32)
    }
}

// ─── UOR encoding: Z/12 layer space ──────────────────────────────────
//
// Twelve leaf shapes, one per residue class mod LATTICE_LAYERS = 12.
// Each carries a single ConstraintRef::Residue { modulus: 12, residue: k }.

/// Layer `k = 0` of `Z/LATTICE_LAYERS = Z/12`.
pub struct Z12L0;
impl ConstrainedTypeShape for Z12L0 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/z12/index/0";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: LATTICE_LAYERS,
        residue: 0,
    }];
}

/// Layer `k = 1` of `Z/12`.
pub struct Z12L1;
impl ConstrainedTypeShape for Z12L1 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/z12/index/1";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: LATTICE_LAYERS,
        residue: 1,
    }];
}

/// Layer `k = 2` of `Z/12`.
pub struct Z12L2;
impl ConstrainedTypeShape for Z12L2 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/z12/index/2";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: LATTICE_LAYERS,
        residue: 2,
    }];
}

/// Layer `k = 3` of `Z/12`.
pub struct Z12L3;
impl ConstrainedTypeShape for Z12L3 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/z12/index/3";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: LATTICE_LAYERS,
        residue: 3,
    }];
}

/// Layer `k = 4` of `Z/12`.
pub struct Z12L4;
impl ConstrainedTypeShape for Z12L4 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/z12/index/4";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: LATTICE_LAYERS,
        residue: 4,
    }];
}

/// Layer `k = 5` of `Z/12`.
pub struct Z12L5;
impl ConstrainedTypeShape for Z12L5 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/z12/index/5";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: LATTICE_LAYERS,
        residue: 5,
    }];
}

/// Layer `k = 6` of `Z/12`.
pub struct Z12L6;
impl ConstrainedTypeShape for Z12L6 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/z12/index/6";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: LATTICE_LAYERS,
        residue: 6,
    }];
}

/// Layer `k = 7` of `Z/12`.
pub struct Z12L7;
impl ConstrainedTypeShape for Z12L7 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/z12/index/7";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: LATTICE_LAYERS,
        residue: 7,
    }];
}

/// Layer `k = 8` of `Z/12`.
pub struct Z12L8;
impl ConstrainedTypeShape for Z12L8 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/z12/index/8";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: LATTICE_LAYERS,
        residue: 8,
    }];
}

/// Layer `k = 9` of `Z/12`.
pub struct Z12L9;
impl ConstrainedTypeShape for Z12L9 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/z12/index/9";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: LATTICE_LAYERS,
        residue: 9,
    }];
}

/// Layer `k = 10` of `Z/12`.
pub struct Z12L10;
impl ConstrainedTypeShape for Z12L10 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/z12/index/10";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: LATTICE_LAYERS,
        residue: 10,
    }];
}

/// Layer `k = 11` of `Z/12`.
pub struct Z12L11;
impl ConstrainedTypeShape for Z12L11 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/z12/index/11";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: LATTICE_LAYERS,
        residue: 11,
    }];
}

// ─── Coproduct tree: balanced, depth 4 ───────────────────────────────
//
// Pairing is arbitrary among balanced trees. Indexed pairs (0,1),
// (2,3), …, (10,11), then balanced upward.

coproduct_shape!(Z12Pair01, Z12L0, Z12L1);
coproduct_shape!(Z12Pair23, Z12L2, Z12L3);
coproduct_shape!(Z12Pair45, Z12L4, Z12L5);
coproduct_shape!(Z12Pair67, Z12L6, Z12L7);
coproduct_shape!(Z12Pair89, Z12L8, Z12L9);
coproduct_shape!(Z12PairAB, Z12L10, Z12L11);

coproduct_shape!(Z12Quartet0123, Z12Pair01, Z12Pair23);
coproduct_shape!(Z12Quartet4567, Z12Pair45, Z12Pair67);
coproduct_shape!(Z12Quartet89AB, Z12Pair89, Z12PairAB);

// ─── Manual coproduct: Z12Octet0_7 ───────────────────────────────────
//
// Same construction issue as `crate::u30::U30`: the SDK macro's
// constraint splice buffer is `[_; 18]`, but
// `Z12Quartet0123::CONSTRAINTS.len() + Z12Quartet4567::CONSTRAINTS.len()
// + 2 = 10 + 10 + 2 = 22` slots are needed. We emit the impl by hand
// with empty CONSTRAINTS, mirroring the macro's IRI form, SITE_COUNT,
// and SITE_BUDGET arithmetic. The downstream `Z12` macro splice still
// fits in the buffer because `Z12Octet0_7::CONSTRAINTS.len() = 0`,
// so we leave `Z12` as a macro emission.

/// Octet of layers 0..=7 of `Z/12`, balanced binary tree of depth 3.
pub struct Z12Octet0_7;

impl ConstrainedTypeShape for Z12Octet0_7 {
    const IRI: &'static str = "urn:uor:coproduct:Z12Quartet0123:Z12Quartet4567";
    const SITE_BUDGET: usize = {
        let a = <Z12Quartet0123 as ConstrainedTypeShape>::SITE_BUDGET;
        let b = <Z12Quartet4567 as ConstrainedTypeShape>::SITE_BUDGET;
        if a > b { a } else { b }
    };
    const SITE_COUNT: usize = {
        let a = <Z12Quartet0123 as ConstrainedTypeShape>::SITE_COUNT;
        let b = <Z12Quartet4567 as ConstrainedTypeShape>::SITE_COUNT;
        (if a > b { a } else { b }) + 1
    };
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
}

coproduct_shape!(Z12, Z12Octet0_7, Z12Quartet89AB);

// ─── Cartesian product: R360 = U30 ⊠ Z12 ─────────────────────────────

cartesian_product_shape!(R360, U30, Z12);

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    /// `ELEMENTS` has exactly `R_CARDINALITY` entries.
    #[test]
    fn elements_cardinality_is_r_cardinality() {
        assert_eq!(ELEMENTS.len(), R_CARDINALITY);
    }

    /// `ELEMENTS` is strictly ascending and every entry is in
    /// `[1, MODULUS_R]` and coprime to `MODULUS_U`.
    #[test]
    fn elements_well_formed() {
        let mut prev = 0u32;
        for &n in &ELEMENTS {
            assert!(n > prev, "ELEMENTS not strictly ascending");
            assert!(u64::from(n) <= MODULUS_R, "ELEMENTS[i] = {n} exceeds MODULUS_R");
            assert_eq!(gcd(u64::from(n), MODULUS_U), 1, "{n} not coprime to MODULUS_U");
            prev = n;
        }
    }

    /// Round-trip: `compose ∘ decompose = id` on `ELEMENTS`.
    #[test]
    fn decompose_then_compose_is_identity() {
        for &n in &ELEMENTS {
            let (u_idx, layer) = decompose(n).expect("R(360) element must decompose");
            let recomposed = compose(u_idx, layer).expect("compose must succeed for valid pair");
            assert_eq!(n, recomposed, "round-trip failure at n = {n}");
        }
    }

    /// Round-trip: `decompose ∘ compose = id` on the full index space.
    #[test]
    fn compose_then_decompose_is_identity() {
        for u_idx in 0..U30_CARDINALITY {
            for layer in 0..(LATTICE_LAYERS as u32) {
                let n = compose(u_idx, layer).expect("valid pair must compose");
                let (u_idx2, layer2) =
                    decompose(n).expect("composed integer must decompose");
                assert_eq!(u_idx, u_idx2, "u_index mismatch");
                assert_eq!(layer, layer2, "layer mismatch");
            }
        }
    }

    /// `decompose` rejects non-coprime integers.
    #[test]
    fn decompose_rejects_non_coprime() {
        // Multiples of 2, 3, or 5 in [1, 30] should fail.
        assert!(decompose(2).is_none());
        assert!(decompose(3).is_none());
        assert!(decompose(5).is_none());
        assert!(decompose(6).is_none());
        assert!(decompose(15).is_none());
        assert!(decompose(30).is_none());
        assert!(decompose(60).is_none());
        assert!(decompose(360).is_none()); // gcd(360, 30) = 30
    }

    /// `decompose` rejects out-of-range integers.
    #[test]
    fn decompose_rejects_out_of_range() {
        assert!(decompose(0).is_none());
        // 361 is coprime to 30 but > MODULUS_R.
        assert_eq!(gcd(361, 30), 1);
        assert!(decompose(361).is_none());
    }

    /// `compose` rejects out-of-range indices.
    #[test]
    fn compose_rejects_out_of_range() {
        assert!(compose(U30_CARDINALITY, 0).is_none());
        assert!(compose(0, LATTICE_LAYERS as u32).is_none());
    }

    /// Spot checks against the FIXED PDF.
    #[test]
    fn spot_checks_against_fixed_pdf() {
        // From Appendix A: n = 1 ↔ (u_index = 0 [u=1], layer = 0).
        assert_eq!(decompose(1), Some((0, 0)));
        // n = 7 ↔ (u_index = 1 [u=7], layer = 0).
        assert_eq!(decompose(7), Some((1, 0)));
        // n = 31 ↔ (u_index = 0 [u=1], layer = 1).
        assert_eq!(decompose(31), Some((0, 1)));
        // n = 359 ↔ (u_index = 7 [u=29], layer = 11).
        assert_eq!(decompose(359), Some((7, 11)));
    }

    /// `R360` shape's SITE_COUNT equals `U30::SITE_COUNT + Z12::SITE_COUNT`.
    #[test]
    fn r360_site_count_is_sum() {
        let u30_sc = <U30 as ConstrainedTypeShape>::SITE_COUNT;
        let z12_sc = <Z12 as ConstrainedTypeShape>::SITE_COUNT;
        let r360_sc = <R360 as ConstrainedTypeShape>::SITE_COUNT;
        assert_eq!(r360_sc, u30_sc + z12_sc);
    }

    /// `Z12::SITE_COUNT = 5` for the chosen 12-leaf balanced tree.
    #[test]
    fn z12_site_count_is_five() {
        assert_eq!(<Z12 as ConstrainedTypeShape>::SITE_COUNT, 5);
    }

    /// `R360::IRI` is the SDK-canonical cartesian-product IRI
    /// `urn:uor:cartesian:{lex_earlier}:{lex_later}`.
    #[test]
    fn r360_iri_is_canonical_cartesian() {
        let iri = <R360 as ConstrainedTypeShape>::IRI;
        assert!(iri.starts_with("urn:uor:cartesian:"), "got {iri}");
    }

    /// `Z12::IRI` is the SDK-canonical coproduct IRI.
    #[test]
    fn z12_iri_is_canonical_coproduct() {
        let iri = <Z12 as ConstrainedTypeShape>::IRI;
        assert!(iri.starts_with("urn:uor:coproduct:"), "got {iri}");
    }

    /// **CRT decomposition.** `Z/12 ≅ Z/4 × Z/3` via
    /// `k ↔ (k mod 4, k mod 3)`. Verifies the bijection by
    /// exhaustion: each `(a, b) ∈ Z/4 × Z/3` corresponds to exactly
    /// one `k ∈ Z/12`.
    ///
    /// The `Z/3` factor connects to `query::TriadProjection` in the
    /// UOR ontology; the `Z/4` factor connects to the binary depth
    /// of the lattice.
    #[test]
    fn z12_decomposes_via_crt() {
        let mut hits = [[false; 3]; 4]; // [k mod 4][k mod 3]
        for k in 0..(LATTICE_LAYERS as u32) {
            let a = (k % 4) as usize;
            let b = (k % 3) as usize;
            assert!(!hits[a][b], "duplicate (a, b) pair from k = {k}");
            hits[a][b] = true;
        }
        // Every (a, b) covered exactly once.
        for a in 0..4 {
            for b in 0..3 {
                assert!(hits[a][b], "(a, b) = ({a}, {b}) never hit");
            }
        }
    }

    /// **Set agreement with `appendix_a::ROWS`.** The integers in
    /// `ELEMENTS` are exactly the first column of [`crate::appendix_a::ROWS`].
    #[test]
    fn elements_match_appendix_a_first_column() {
        use crate::appendix_a::ROWS;
        for (i, (&n_here, &(n_there, _, _))) in
            ELEMENTS.iter().zip(ROWS.iter()).enumerate()
        {
            assert_eq!(n_here, n_there, "row {i} disagrees");
        }
    }
}
