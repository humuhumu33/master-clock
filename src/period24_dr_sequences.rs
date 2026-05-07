//! # Period-24 digital-root sequences across `U_30(90)`
//!
//! For each `r ∈ U_30(90)`, the sequence
//!
//! ```text
//!   s_r(k) = dr( (r · U_30(90)[k]) mod MODULUS_U90 )    for k ∈ [0, 24)
//! ```
//!
//! is a permutation of `(Z/9Z)*` repeated four times — the same
//! digital-root cycle as [`crate::u90::DR_CYCLE`] but reordered.
//! That is because multiplication by `r` is a permutation of
//! `U_30(90)` under mod-90 multiplication (group closure), and
//! the digital-root map factors through the permutation.
//!
//! ## Theorems established at compile time
//!
//! 1. The 24×24 table `DR_SEQUENCES[i][k] = s_{r_i}(k)` has every
//!    entry in `(Z/9Z)* = {1, 2, 4, 5, 7, 8}` (closure).
//! 2. Every row sums to `108 = 4 · 27 = 4 · |Σ (Z/9Z)*|`.
//! 3. Every column sums to `108`.
//! 4. Total table sum is `2592 = 24 · 108 = 6 · 432`.
//! 5. The row at `r = 1` equals [`crate::u90::DR_CYCLE`] verbatim.
//!
//! ## Anchor — derived from Phase 1 + Phase 1.5
//!
//! Computed at compile time from
//! [`crate::u90::RESIDUES_U90`],
//! [`crate::u90::MODULUS_U90`], and
//! [`crate::digital_root::digital_root`]. Nothing else.
//!
//! ## UOR primitive bindings
//!
//! | Object | UOR ontology IRI |
//! |---|---|
//! | `DR_SEQUENCES` | `https://uor.foundation/query/CoordinateQuery` (576-cell lookup) |
//! | row permutation property | `https://uor.foundation/morphism/Isomorphism` |
//! | sum invariants | `https://uor.foundation/predicate/TypePredicate` |

use crate::digital_root::digital_root;
use crate::u90::{DR_CYCLE, MODULUS_U90, RESIDUES_U90, U90_CARDINALITY};

/// 24×24 table. Entry `[i][k]` is `dr( (RESIDUES_U90[i] ·
/// RESIDUES_U90[k]) mod 90 )`.
pub const DR_SEQUENCES: [[u64; 24]; 24] = compute_dr_sequences();

const fn compute_dr_sequences() -> [[u64; 24]; 24] {
    let mut out = [[0u64; 24]; 24];
    let mut i = 0usize;
    while i < U90_CARDINALITY {
        let r = RESIDUES_U90[i];
        let mut k = 0usize;
        while k < U90_CARDINALITY {
            let a = RESIDUES_U90[k];
            let prod = (r * a) % MODULUS_U90;
            out[i][k] = digital_root(prod);
            k += 1;
        }
        i += 1;
    }
    out
}

/// Row sum invariant: `108 = 4 · 27`.
pub const DR_ROW_SUM: u64 = 108;

/// Total table sum: `2592 = 24 · 108 = 6 · SUM_DR = 6 · 432`.
pub const DR_TABLE_TOTAL: u64 = (U90_CARDINALITY as u64) * DR_ROW_SUM;

const _: () = assert!(
    DR_TABLE_TOTAL == 2592,
    "DR-table total must equal 2592 = 24 · 108"
);

/// Every row sums to 108. Verified at compile time.
const _: () = {
    let mut i = 0usize;
    while i < 24 {
        let mut row_sum = 0u64;
        let mut k = 0usize;
        while k < 24 {
            row_sum += DR_SEQUENCES[i][k];
            k += 1;
        }
        assert!(row_sum == DR_ROW_SUM, "row sum drift");
        i += 1;
    }
};

/// Every column sums to 108. Verified at compile time.
const _: () = {
    let mut k = 0usize;
    while k < 24 {
        let mut col_sum = 0u64;
        let mut i = 0usize;
        while i < 24 {
            col_sum += DR_SEQUENCES[i][k];
            i += 1;
        }
        assert!(col_sum == DR_ROW_SUM, "col sum drift");
        k += 1;
    }
};

/// Every entry is in `(Z/9Z)* = {1, 2, 4, 5, 7, 8}`.
const _: () = {
    let mut i = 0usize;
    while i < 24 {
        let mut k = 0usize;
        while k < 24 {
            let v = DR_SEQUENCES[i][k];
            assert!(
                v == 1 || v == 2 || v == 4 || v == 5 || v == 7 || v == 8,
                "DR table entry escapes (Z/9Z)*"
            );
            k += 1;
        }
        i += 1;
    }
};

/// Row at `r = 1` matches [`crate::u90::DR_CYCLE`] exactly.
const _: () = {
    let mut k = 0usize;
    while k < 24 {
        assert!(
            DR_SEQUENCES[0][k] == DR_CYCLE[k],
            "row at r=1 must equal DR_CYCLE"
        );
        k += 1;
    }
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
    use super::*;

    #[test]
    fn row_sums_all_108() {
        for row in &DR_SEQUENCES {
            let s: u64 = row.iter().sum();
            assert_eq!(s, 108);
        }
    }

    #[test]
    fn column_sums_all_108() {
        for k in 0..24 {
            let s: u64 = (0..24).map(|i| DR_SEQUENCES[i][k]).sum();
            assert_eq!(s, 108);
        }
    }

    #[test]
    fn total_2592() {
        let total: u64 = DR_SEQUENCES.iter().flatten().sum();
        assert_eq!(total, 2592);
        assert_eq!(total, 6 * 432);
    }

    #[test]
    fn every_entry_in_units_z9() {
        for row in &DR_SEQUENCES {
            for &v in row {
                assert!(matches!(v, 1 | 2 | 4 | 5 | 7 | 8));
            }
        }
    }

    #[test]
    fn first_row_matches_dr_cycle() {
        assert_eq!(DR_SEQUENCES[0], DR_CYCLE);
    }

    /// Each row is a permutation of the dr cycle (same multiset).
    #[test]
    fn every_row_is_permutation_of_dr_cycle() {
        let mut canonical = DR_CYCLE.to_vec();
        canonical.sort();
        for row in &DR_SEQUENCES {
            let mut sorted = row.to_vec();
            sorted.sort();
            assert_eq!(sorted, canonical);
        }
    }
}
