//! # `U_30(90)` — the 24-element residue substrate
//!
//! `U_30(90) = { n ∈ [1, 90] : gcd(n, 30) = 1 }`. Cardinality 24.
//!
//! ## Why this exists as a Phase 1 module
//!
//! `U_30(90)` is the bridge between the eight-fold residue group
//! `U(30)` and the 96-room `R(360)` lattice. The decomposition
//!
//! ```text
//!   R(360) = U_30(90) × {0, 1, 2, 3}
//!         |U_30(90)| = 24
//!         |R(360)|   = 24 · 4 = 96
//! ```
//!
//! exposes the period-24 digital-root cycle. Every structural claim
//! about it (row sums of the 24×24 mod-90 multiplication matrix,
//! squares-on-the-diagonal, 4-uniformity over `(Z/9Z)*`) reduces
//! to facts about `U_30(90)` and the units of `Z/9Z` — see
//! [`crate::units_z9`].
//!
//! ## Self-deriving discipline
//!
//! Every constant in this module is computed at compile time from
//! [`crate::parameters::MODULUS_U`] = 30 and the standard
//! `gcd` / `euler_phi` helpers in [`crate::arithmetic`]. The constant
//! `MODULUS_U90 = 90` is itself derived: `90 = 3 · MODULUS_U`,
//! anchored to the smallest extension of `MODULUS_U` whose
//! associated `R` lattice has `|R| ≡ 0 (mod 4)` (i.e. tiles four
//! times into `R(360)`).
//!
//! ## UOR primitive bindings
//!
//! | Object | UOR ontology IRI |
//! |---|---|
//! | `RESIDUES_U90` | `https://uor.foundation/stream/ProductiveStream` (24 ordered residues) |
//! | `dr_cycle` | `https://uor.foundation/recursion/BoundedRecursion` (descent measure = position in cycle) |
//! | `square_residues` | `https://uor.foundation/query/CoordinateQuery` |

use crate::arithmetic::gcd;
use crate::digital_root::digital_root;
use crate::parameters::MODULUS_U;

/// `MODULUS_U90 = 3 · MODULUS_U = 90`. Smallest multiple of
/// `MODULUS_U` that tiles `MODULUS_R = 360` exactly 4 times.
pub const MODULUS_U90: u64 = 3 * MODULUS_U;

/// Cardinality of `U_30(90)` — derived, not stated. Equals 24.
pub const U90_CARDINALITY: usize = compute_u90_cardinality();

const fn compute_u90_cardinality() -> usize {
    let mut count = 0usize;
    let mut n = 1u64;
    while n <= MODULUS_U90 {
        if gcd(n, MODULUS_U) == 1 {
            count += 1;
        }
        n += 1;
    }
    count
}

const _: () = assert!(
    U90_CARDINALITY == 24,
    "U_30(90) cardinality must be 24"
);

/// The 24 residues of `U_30(90)`, computed at compile time.
pub const RESIDUES_U90: [u64; U90_CARDINALITY] = compute_residues_u90();

const fn compute_residues_u90() -> [u64; U90_CARDINALITY] {
    let mut out = [0u64; U90_CARDINALITY];
    let mut idx = 0usize;
    let mut n = 1u64;
    while n <= MODULUS_U90 {
        if gcd(n, MODULUS_U) == 1 {
            out[idx] = n;
            idx += 1;
        }
        n += 1;
    }
    out
}

/// **Verified anchor.** The 24 residues match Gary's reference
/// Matrix row labels at `magicmatrix.html`. Hand-typed; used only
/// to cross-check the derivation above.
pub const GARY_STATED_U90_RESIDUES: [u64; 24] = [
    1, 7, 11, 13, 17, 19, 23, 29,
    31, 37, 41, 43, 47, 49, 53, 59,
    61, 67, 71, 73, 77, 79, 83, 89,
];

const _: () = {
    let mut i = 0;
    while i < U90_CARDINALITY {
        assert!(RESIDUES_U90[i] == GARY_STATED_U90_RESIDUES[i],
            "derived U_30(90) residue disagrees with reference table");
        i += 1;
    }
};

/// Digital-root cycle over `U_30(90)`: `dr(n)` for each
/// `n ∈ U_30(90)` in order. Verifiably has period 24.
pub const DR_CYCLE: [u64; U90_CARDINALITY] = compute_dr_cycle();

const fn compute_dr_cycle() -> [u64; U90_CARDINALITY] {
    let mut out = [0u64; U90_CARDINALITY];
    let mut i = 0usize;
    while i < U90_CARDINALITY {
        out[i] = digital_root(RESIDUES_U90[i]);
        i += 1;
    }
    out
}

/// **Verified anchor.** Gary's stated dr cycle from
/// `Mod_90_Factorization_Digital_Root_Principles.pdf`.
pub const GARY_STATED_DR_CYCLE: [u64; 24] = [
    1, 7, 2, 4, 8, 1, 5, 2,
    4, 1, 5, 7, 2, 4, 8, 5,
    7, 4, 8, 1, 5, 7, 2, 8,
];

const _: () = {
    let mut i = 0;
    while i < U90_CARDINALITY {
        assert!(DR_CYCLE[i] == GARY_STATED_DR_CYCLE[i],
            "derived dr cycle disagrees with Mod-90 PDF");
        i += 1;
    }
};

/// **Theorem at compile time.** Each unit of `(Z/9Z)*` =
/// `{1, 2, 4, 5, 7, 8}` appears exactly 4 times in `DR_CYCLE`,
/// and `{3, 6, 9}` never appear (since all members of `U_30(90)`
/// are coprime to 3).
pub const DR_CYCLE_UNIFORMITY: [(u64, u64); 9] = compute_uniformity();

const fn compute_uniformity() -> [(u64, u64); 9] {
    let mut out = [(0u64, 0u64); 9];
    let mut v = 1u64;
    while v <= 9 {
        let mut count = 0u64;
        let mut i = 0usize;
        while i < U90_CARDINALITY {
            if DR_CYCLE[i] == v {
                count += 1;
            }
            i += 1;
        }
        out[(v - 1) as usize] = (v, count);
        v += 1;
    }
    out
}

const _: () = {
    // Each of {1,2,4,5,7,8} appears 4 times. {3,6,9} appear 0 times.
    let expected: [(u64, u64); 9] = [
        (1, 4), (2, 4), (3, 0), (4, 4), (5, 4),
        (6, 0), (7, 4), (8, 4), (9, 0),
    ];
    let mut i = 0;
    while i < 9 {
        assert!(DR_CYCLE_UNIFORMITY[i].0 == expected[i].0);
        assert!(DR_CYCLE_UNIFORMITY[i].1 == expected[i].1,
            "dr-cycle uniformity broken");
        i += 1;
    }
};

/// **Square residues mod 90 over U_30(90).** Distinct values
/// of `(a · a) mod 90` for `a ∈ U_30(90)`. Equals
/// `{1, 19, 31, 49, 61, 79}` — six values, each appearing 4 times
/// among the 24 squares.
pub const SQUARE_RESIDUE_VALUES: [u64; 6] = [1, 19, 31, 49, 61, 79];

/// Sum of the principal diagonal of the 24×24 mod-90
/// multiplication matrix `(a, b) ↦ (a · b) mod 90`. Equals
/// `4 · (1+19+31+49+61+79) = 4 · 240 = 960`.
pub const DIAGONAL_SUM: u64 = compute_diagonal_sum();

const fn compute_diagonal_sum() -> u64 {
    let mut s = 0u64;
    let mut i = 0usize;
    while i < U90_CARDINALITY {
        let a = RESIDUES_U90[i];
        s += (a * a) % MODULUS_U90;
        i += 1;
    }
    s
}

const _: () = assert!(DIAGONAL_SUM == 960, "diagonal sum must equal 960");

/// **Sum of `U_30(90)` itself.** Equals `1080 = 3 · MODULUS_R`.
/// This is the row sum of every row of the 24×24 mod-90
/// multiplication matrix, since `a · U_30(90) ≅ U_30(90)` as a
/// permutation for any `a ∈ U_30(90)`.
pub const ROW_SUM_LINEAR: u64 = compute_linear_sum();

const fn compute_linear_sum() -> u64 {
    let mut s = 0u64;
    let mut i = 0usize;
    while i < U90_CARDINALITY {
        s += RESIDUES_U90[i];
        i += 1;
    }
    s
}

const _: () = assert!(ROW_SUM_LINEAR == 1080, "Σ U_30(90) must equal 1080 = 3·MODULUS_R");

/// **Anchor to Phase 1.** `|U_30(90)| · 4 = |R(360)| = 96`. The
/// 24-element substrate tiles into `R(360)` exactly 4 times.
pub const TILES_INTO_R360: u64 = (U90_CARDINALITY as u64)
    * (crate::parameters::MODULUS_R / MODULUS_U90);

const _: () = assert!(
    TILES_INTO_R360 == crate::parameters::R_CARDINALITY as u64,
    "U_30(90) × 4 must equal R(360) cardinality"
);

// ─── Secondary diagonal of the 24×24 mod-90 multiplication matrix ────

/// Distinct values of `(MODULUS_U90 − (n · n) mod MODULUS_U90) mod
/// MODULUS_U90` over `n ∈ U_30(90)`. Equals `{11, 29, 41, 59, 71, 89}` —
/// six values, each appearing 4 times among the 24 entries.
///
/// Every element is itself prime: `{11, 29, 41, 59, 71, 89}` are
/// primes congruent to ±1 mod 6.
pub const SECONDARY_DIAGONAL_VALUES: [u64; 6] = [11, 29, 41, 59, 71, 89];

/// Sum of the secondary diagonal: `4 · (11+29+41+59+71+89) =
/// 4 · 300 = 1200`. Combined with [`DIAGONAL_SUM`] = 960, the two
/// diagonals sum to `2160 = 6 · MODULUS_R`.
pub const SECONDARY_DIAGONAL_SUM: u64 = compute_secondary_diagonal_sum();

const fn compute_secondary_diagonal_sum() -> u64 {
    let mut s = 0u64;
    let mut i = 0usize;
    while i < U90_CARDINALITY {
        let a = RESIDUES_U90[i];
        let sq = (a * a) % MODULUS_U90;
        // 90 − sq, with the wrap case sq == 0 mapped to 0 (cannot
        // happen here since a is coprime to 90, so a² is too).
        let entry = if sq == 0 { 0 } else { MODULUS_U90 - sq };
        s += entry;
        i += 1;
    }
    s
}

const _: () = assert!(
    SECONDARY_DIAGONAL_SUM == 1200,
    "secon