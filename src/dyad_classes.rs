//! # Dyad classes — `576 = 36 × 16` partition by digital-root pair
//!
//! The 576 ordered factorization dyads in
//! [`crate::factorization_dyads::DYADS_TABLE`] partition into exactly
//! `36 = |(Z/9Z)*|²` classes, where each class collects all dyads
//! `(a, b)` with the same digital-root pair `(dr(a), dr(b))`. Every
//! class has exactly `16 = 4²` members.
//!
//! ## The arithmetic
//!
//! ```text
//!   |U_30(90)|             = 24
//!   |(Z/9Z)*|              = 6
//!   each dr value in DR_CYCLE appears exactly 4 times
//!   ⟹ |class(dr1, dr2)|    = 4 · 4 = 16
//!   ⟹ 36 · 16              = 576 = 24²  ✓
//! ```
//!
//! This is the dyad-level expression of the substrate's triality:
//! the 576-dyad lattice factors as `6² × 4²`, with the `6 = |(Z/9Z)*|`
//! axis carrying the residue-class structure and the `4` axis
//! carrying the in-class redundancy from the period-24 dr cycle.
//!
//! ## Anchor — Phase 1 + Phase 1.5 + Phase 4
//!
//! Computed at compile time from [`crate::u90::RESIDUES_U90`],
//! [`crate::units_z9::UNITS_Z9`], and
//! [`crate::digital_root::digital_root`].
//!
//! ## UOR primitive bindings
//!
//! | Object | UOR ontology IRI |
//! |---|---|
//! | `CLASS_SIZE` | `https://uor.foundation/predicate/TypePredicate` |
//! | `partition_count` | `https://uor.foundation/morphism/Fibration` (36 fibers, 16 each) |
//! | dyad → class | `https://uor.foundation/query/CoordinateQuery` |

use crate::digital_root::digital_root;
use crate::u90::{RESIDUES_U90, U90_CARDINALITY};
use crate::units_z9::{UNITS_Z9, UNITS_Z9_CARDINALITY};

/// Number of classes: `|(Z/9Z)*|² = 6² = 36`.
pub const CLASS_COUNT: usize = UNITS_Z9_CARDINALITY * UNITS_Z9_CARDINALITY;

/// Members per class: `4² = 16`. Each `(Z/9Z)*` value appears 4
/// times in `DR_CYCLE`, so each `(dr1, dr2)` pair is hit `4 × 4 = 16`
/// times across the 576 ordered dyads.
pub const CLASS_SIZE: usize = 16;

const _: () = assert!(
    CLASS_COUNT == 36,
    "class count must equal 36 = |(Z/9Z)*|²"
);

const _: () = assert!(
    CLASS_COUNT * CLASS_SIZE == U90_CARDINALITY * U90_CARDINALITY,
    "36 · 16 must equal 576 = |U_30(90)|²"
);

/// Position of `(dr1, dr2)` within the lexicographic enumeration
/// of `(Z/9Z)* × (Z/9Z)*`. Returns `usize::MAX` if either coordinate
/// is not in `(Z/9Z)*`.
#[must_use]
pub const fn class_index(dr1: u64, dr2: u64) -> usize {
    let mut i = 0usize;
    let mut p1 = usize::MAX;
    let mut p2 = usize::MAX;
    while i < UNITS_Z9_CARDINALITY {
        if UNITS_Z9[i] == dr1 {
            p1 = i;
        }
        if UNITS_Z9[i] == dr2 {
            p2 = i;
        }
        i += 1;
    }
    if p1 == usize::MAX || p2 == usize::MAX {
        return usize::MAX;
    }
    p1 * UNITS_Z9_CARDINALITY + p2
}

/// `CLASS_COUNTS[k]` is the number of dyads `(a, b) ∈ U_30(90)²`
/// with `(dr(a), dr(b))` lexicographically at position `k`.
/// Computed at compile time. Every entry equals `CLASS_SIZE = 16`.
pub const CLASS_COUNTS: [usize; CLASS_COUNT] = compute_class_counts();

const fn compute_class_counts() -> [usize; CLASS_COUNT] {
    let mut out = [0usize; CLASS_COUNT];
    let mut i = 0usize;
    while i < U90_CARDINALITY {
        let dr1 = digital_root(RESIDUES_U90[i]);
        let mut j = 0usize;
        while j < U90_CARDINALITY {
            let dr2 = digital_root(RESIDUES_U90[j]);
            let k = class_index(dr1, dr2);
            // dr is always in (Z/9Z)* for U_30(90) members (coprime to 3).
            out[k] += 1;
            j += 1;
        }
        i += 1;
    }
    out
}

const _: () = {
    // Every class has exactly CLASS_SIZE members.
    let mut k = 0usize;
    while k < CLASS_COUNT {
        assert!(
            CLASS_COUNTS[k] == CLASS_SIZE,
            "class count drift — partition not uniform"
        );
        k += 1;
    }
};

const _: () = {
    // Total over all classes equals 576.
    let mut total = 0usize;
    let mut k = 0usize;
    while k < CLASS_COUNT {
        total += CLASS_COUNTS[k];
        k += 1;
    }
    assert!(total == 576, "sum of class counts must equal 576");
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
    use super::*;

    #[test]
    fn class_count_36() {
        assert_eq!(CLASS_COUNT, 36);
        assert_eq!(CLASS_COUNT, UNITS_Z9_CARDINALITY * UNITS_Z9_CARDINALITY);
    }

    #[test]
    fn class_size_16() {
        assert_eq!(CLASS_SIZE, 16);
        assert_eq!(CLASS_SIZE, 4 * 4);
    }

    #[test]
    fn partition_arithmetic_576() {
        assert_eq!(CLASS_COUNT * CLASS_SIZE, 576);
        assert_eq!(CLASS_COUNT * CLASS_SIZE, U90_CARDINALITY * U90_CARDINALITY);
    }

    #[test]
    fn every_class_has_size_16() {
        for &c in &CLASS_COUNTS {
            assert_eq!(c, CLASS_SIZE);
        }
    }

    #[test]
    fn class_index_matches_lex_order() {
        // (1, 1) → 0; (1, 2) → 1; (1, 4) → 2; (1, 5) → 3; …
        assert_eq!(class_index(1, 1), 0);
        assert_eq!(class_index(1, 2), 1);
        assert_eq!(class_index(2, 1), UNITS_Z9_CARDINALITY); // = 6
        assert_eq!(class_index(8, 8), CLASS_COUNT - 1); // = 35
    }

    #[test]
    fn class_index_rejects_non_units() {
        // 0, 3, 6, 9 are not in (Z/9Z)*.
        assert_eq!(class_index(0, 1), usize::MAX);
        assert_eq!(class_index(1, 3), usize::MAX);
        assert_eq!(class_index(6, 6), usize::MAX);
    }

    /// The total partition count factors three ways through the substrate.
    #[test]
    fn triality_of_576_via_dyad_classes() {
        // Path 1: |U_30(90)|² = 24² = 576
        assert_eq!(U90_CARDINALITY * U90_CARDINALITY, 576);
        // Path 2: |(Z/9Z)*|² · 4² = 36 · 16 = 576
        assert_eq!(CLASS_COUNT * CLASS_SIZE, 576);
        // Path 3: factorization_dyads::DYAD_COUNT
        use crate::factorization_dyads::DYAD_COUNT;
        assert_eq!(DYAD_COUNT, 576);
    }
}
