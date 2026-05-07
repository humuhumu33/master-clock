//! # Triality witnesses — three structural three-fold symmetries
//!
//! Where `cross_phase_invariants.rs` proves that *numbers* converge
//! through three independent paths, this file proves that
//! *structures* exhibit three-fold symmetry. Each test demonstrates
//! a triadic decomposition of one component of the substrate.
//!
//! Three witnesses, three structures each:
//!
//! 1. **Trinity of cosets.** `Z/9Z` partitions into three cosets
//!    `{1,4,7} ∪ {2,5,8} ∪ {3,6,9}` under the equivalence
//!    `n mod 3`. The first two are subsets of `(Z/9Z)*`; the third
//!    is the singular set (multiples of 3 mod 9).
//!
//! 2. **Trinity of constraint types.** The three Phase 3 prime-
//!    structure modules (twin / Sophie Germain / Siamese) partition
//!    the question of "which mod-30 residues admit which kind of
//!    structured prime configuration."
//!
//! 3. **Trinity of structural layers.** Eight-element `U(30)` →
//!    24-element `U_30(90)` → 96-element `R(360)`. Each step
//!    multiplies cardinality by a small integer factor (3 then 4),
//!    revealing the substrate's nested-tile structure.
//!
//! Run with:
//!
//! ```text
//! cargo test --test triality_witnesses -- --nocapture
//! ```

use toroidal_prime_clock::digital_root::digital_root;
use toroidal_prime_clock::parameters::{LATTICE_LAYERS, MODULUS_R, U30_CARDINALITY};
use toroidal_prime_clock::r360::ELEMENTS;
use toroidal_prime_clock::siamese_prime_pattern::SIAMESE_GENERATOR_RESIDUES;
use toroidal_prime_clock::sophie_germain_residues::SOPHIE_GERMAIN_LOWER_RESIDUES;
use toroidal_prime_clock::twin_prime_residues::TWIN_PRIME_LOWER_RESIDUES;
use toroidal_prime_clock::u30::RESIDUES as U30_RESIDUES;
use toroidal_prime_clock::u90::{RESIDUES_U90, U90_CARDINALITY};
use toroidal_prime_clock::units_z9::UNITS_Z9;

// ─────────────────────────────────────────────────────────────────────
// Triality I — three cosets of Z/9Z mod 3
// ─────────────────────────────────────────────────────────────────────

/// **Coset I — the squares of `(Z/9Z)*`.** `{1, 4, 7}` ⊂ `Z/9Z`.
const COSET_SQUARES: [u64; 3] = [1, 4, 7];

/// **Coset II — the non-square units of `(Z/9Z)*`.** `{2, 5, 8}` ⊂ `Z/9Z`.
const COSET_NON_SQUARE_UNITS: [u64; 3] = [2, 5, 8];

/// **Coset III — the non-units (multiples of 3).** `{3, 6, 9}` ⊂ `Z/9Z`.
const COSET_NON_UNITS: [u64; 3] = [3, 6, 9];

/// **Witness — three cosets partition `Z/9Z`.** Their union is
/// `{1, …, 9}`; pairwise disjoint.
#[test]
fn triality_of_z9_cosets() {
    let mut union = [false; 10];
    for cosets in [&COSET_SQUARES, &COSET_NON_SQUARE_UNITS, &COSET_NON_UNITS] {
        for &v in cosets {
            assert!(!union[v as usize], "duplicate value {v} across cosets");
            union[v as usize] = true;
        }
    }
    for v in 1..=9 {
        assert!(union[v], "value {v} missing from union of cosets");
    }
}

/// **Witness — each coset is a `mod 3` class.** Within a single
/// coset, every element has the same value `mod 3`.
#[test]
fn cosets_are_mod3_classes() {
    for cosets in [&COSET_SQUARES, &COSET_NON_SQUARE_UNITS, &COSET_NON_UNITS] {
        let r0 = cosets[0] % 3;
        for &v in cosets {
            assert_eq!(v % 3, r0, "coset member {v} mod 3 ≠ {r0}");
        }
    }
}

/// **Witness — the union of cosets I and II equals `(Z/9Z)*`.**
#[test]
fn squares_plus_non_square_units_equal_units_z9() {
    let mut combined: Vec<u64> = COSET_SQUARES
        .iter()
        .chain(COSET_NON_SQUARE_UNITS.iter())
        .copied()
        .collect();
    combined.sort();
    assert_eq!(combined, UNITS_Z9.to_vec());
}

// ─────────────────────────────────────────────────────────────────────
// Triality II — three Phase 3 prime-constraint types
// ─────────────────────────────────────────────────────────────────────

/// **Witness — each constraint type partitions `U(30)` into a
/// "qualifying" subset and an "isolated" complement.** The three
/// qualifying sets are:
///
/// - twin primes: `{11, 17, 29}` (3 elements, isolated complement of 5)
/// - Sophie Germain: `{11, 23, 29}` (3 elements)
/// - Siamese generators: `{3, 9, 15, 21, 27}` mod 30 (5 elements, NOT in U(30))
#[test]
fn triality_of_constraint_types() {
    // Twin prime lower residues are a 3-element subset of U(30).
    assert_eq!(TWIN_PRIME_LOWER_RESIDUES.len(), 3);
    for &r in &TWIN_PRIME_LOWER_RESIDUES {
        assert!(U30_RESIDUES.contains(&r));
    }
    // Sophie Germain lower residues are also a 3-element subset of U(30).
    assert_eq!(SOPHIE_GERMAIN_LOWER_RESIDUES.len(), 3);
    for &r in &SOPHIE_GERMAIN_LOWER_RESIDUES {
        assert!(U30_RESIDUES.contains(&r));
    }
    // Siamese generators are NOT in U(30) (they are ≡ 3 mod 6).
    assert_eq!(SIAMESE_GENERATOR_RESIDUES.len(), 5);
    for &n in &SIAMESE_GENERATOR_RESIDUES {
        assert!(!U30_RESIDUES.contains(&n));
        assert_eq!(n % 6, 3, "Siamese generator {n} ≢ 3 mod 6");
    }
}

/// **Witness — twin and Sophie Germain qualifying sets share `{11, 29}`.**
/// The intersection of two of the three constraint types is itself
/// a structurally significant pair: residues that are simultaneously
/// twin-prime-eligible AND Sophie-Germain-eligible.
#[test]
fn twin_sophie_intersection_is_eleven_and_twentynine() {
    let intersection: Vec<u64> = TWIN_PRIME_LOWER_RESIDUES
        .iter()
        .filter(|r| SOPHIE_GERMAIN_LOWER_RESIDUES.contains(r))
        .copied()
        .collect();
    assert_eq!(intersection, vec![11, 29]);
}

/// **Witness — the symmetric difference of twin and Sophie Germain
/// is `{17, 23}`.** Each constraint type contributes one residue
/// the other excludes: `17` is twin-only, `23` is Sophie-only.
#[test]
fn twin_sophie_symmetric_difference_is_seventeen_and_twentythree() {
    let only_twin: Vec<u64> = TWIN_PRIME_LOWER_RESIDUES
        .iter()
        .filter(|r| !SOPHIE_GERMAIN_LOWER_RESIDUES.contains(r))
        .copied()
        .collect();
    let only_sg: Vec<u64> = SOPHIE_GERMAIN_LOWER_RESIDUES
        .iter()
        .filter(|r| !TWIN_PRIME_LOWER_RESIDUES.contains(r))
        .copied()
        .collect();
    assert_eq!(only_twin, vec![17]);
    assert_eq!(only_sg, vec![23]);
}

// ─────────────────────────────────────────────────────────────────────
// Triality III — three structural layers
// ─────────────────────────────────────────────────────────────────────

/// **Witness — three layers, two scaling factors, one anchor.**
///
/// ```text
///   |U(30)|     = 8
///        × 3    (mod-30 → mod-90 tiling)
///   |U_30(90)|  = 24
///        × 4    (mod-90 → mod-360 tiling)
///   |R(360)|    = 96
/// ```
///
/// The two scaling factors `3` and `4` are coprime, so the
/// composition `3 · 4 = 12 = LATTICE_LAYERS` is the total tiling
/// from the U(30) scale to the R(360) scale.
#[test]
fn triality_of_structural_layers() {
    // Layer 1 → Layer 2: factor 3.
    assert_eq!(U90_CARDINALITY, U30_CARDINALITY * 3);
    // Layer 2 → Layer 3: factor 4.
    assert_eq!(ELEMENTS.len(), U90_CARDINALITY * 4);
    // Composition: 3 · 4 = 12 = LATTICE_LAYERS.
    assert_eq!(3u64 * 4, u64::from(LATTICE_LAYERS));
    // Direct: 8 · 12 = 96.
    assert_eq!(U30_CARDINALITY * LATTICE_LAYERS as usize, ELEMENTS.len());
}

/// **Witness — each U(30) residue lifts to exactly 12 R(360)
/// elements.** The 8 → 96 inflation is uniform: every residue
/// class participates equally, with multiplicity `LATTICE_LAYERS`.
#[test]
fn each_u30_residue_lifts_uniformly() {
    for &r in &U30_RESIDUES {
        let lifts: u64 = ELEMENTS
            .iter()
            .filter(|&&n| u64::from(n) % 30 == r)
            .count() as u64;
        assert_eq!(
            lifts,
            u64::from(LATTICE_LAYERS),
            "residue {r} lifts to {lifts} elements, expected {LATTICE_LAYERS}"
        );
    }
}

/// **Witness — three diagonals of the 24×24 mod-90 multiplication
/// matrix.** The principal diagonal hosts squares; the secondary
/// diagonal hosts `90 − n²` (six primes); the combined sum factors
/// as `6 · MODULUS_R`.
#[test]
fn triality_of_diagonals() {
    use toroidal_prime_clock::u90::{
        DIAGONAL_PAIR_SUM, DIAGONAL_SUM, MODULUS_U90, SECONDARY_DIAGONAL_SUM,
    };
    // Three diagonal-sum invariants.
    assert_eq!(DIAGONAL_SUM, 960);
    assert_eq!(SECONDARY_DIAGONAL_SUM, 1200);
    assert_eq!(DIAGONAL_PAIR_SUM, 2160);
    // The combined sum factors through MODULUS_R.
    assert_eq!(DIAGONAL_PAIR_SUM, 6 * MODULUS_R);
    // The three are arithmetically distinct.
    assert!(DIAGONAL_SUM < SECONDARY_DIAGONAL_SUM);
    assert!(SECONDARY_DIAGONAL_SUM < DIAGONAL_PAIR_SUM);
    // Each value of the secondary diagonal equals 90 minus a primary value.
    for &a in &RESIDUES_U90 {
        let primary = (a * a) % MODULUS_U90;
        let secondary = (MODULUS_U90 - primary) % MODULUS_U90;
        // Every secondary value is one of {11, 29, 41, 59, 71, 89}.
        assert!(matches!(secondary, 11 | 29 | 41 | 59 | 71 | 89));
    }
}

// ─────────────────────────────────────────────────────────────────────
// Capstone — the three trialities together
// ─────────────────────────────────────────────────────────────────────

/// **Capstone witness — every digital root in `U(30) ∪ U_30(90)`
/// belongs to exactly one of the three Z/9Z cosets.** This ties
/// triality I (cosets) to triality III (structural layers).
#[test]
fn cosets_classify_every_substrate_dr() {
    for &n in U30_RESIDUES.iter().chain(RESIDUES_U90.iter()) {
        let d = digital_root(n);
        let in_squares = COSET_SQUARES.contains(&d);
        let in_non_square_units = COSET_NON_SQUARE_UNITS.contains(&d);
        let in_non_units = COSET_NON_UNITS.contains(&d);
        // Exactly one of the three is true.
        let count = in_squares as u8 + in_non_square_units as u8 + in_non_units as u8;
        assert_eq!(
            count, 1,
            "n={n}, dr={d} classified into {count} cosets (must be exactly 1)"
        );
        // For elements of U(30) ⊂ U_30(90), dr cannot be in coset III
        // (since they are coprime to 3 and hence dr coprime to 3).
        assert!(!in_non_units, "dr({n}) = {d} is non-unit but n is in U(30)");
    }
}
