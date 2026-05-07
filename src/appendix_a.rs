//! # Appendix A — the 96-row residue / digital-root / Fibonacci-DR matrix
//!
//! ## Self-deriving construction
//!
//! [`ROWS`] is **computed at compile time** from
//! [`crate::parameters::MODULUS_R`], [`crate::arithmetic::gcd`],
//! [`crate::digital_root::dr`], and [`crate::fibonacci::dr_fib`].
//! There are no hand-typed entries in [`ROWS`].
//!
//! A reference transcription of the same table from
//! primesdemystified.com is preserved as [`FIXED_PDF_VERIFICATION`]
//! and exists **only to cross-check the derivation**. A compile-time
//! assertion confirms that [`ROWS`] equals [`FIXED_PDF_VERIFICATION`]
//! entry by entry; the build fails on any drift.
//!
//! ## The two `432` invariants
//!
//! Over the 96 rows:
//!
//! ```text
//! Σ dr(n)   = 432
//! Σ dr(F_n) = 432
//! ```
//!
//! Both sums are **derived** as [`SUM_DR`] and [`SUM_FIBO_DR`] by
//! summing the corresponding columns of the derived [`ROWS`].
//! Compile-time assertions confirm both equal `432`. The reference
//! values [`GARY_STATED_TOTAL_DR`] and [`GARY_STATED_TOTAL_FIBO_DR`]
//! exist solely as cross-check anchors.
//!
//! The structural derivation `432 = LATTICE_LAYERS · 36 = 12 · 36`
//! falls out of stratifying `R(360)` by `U(30)` residue mod 3; see
//! the test `sum_dr_matches_structural_derivation` for the
//! one-paragraph proof.

use crate::arithmetic::gcd;
use crate::digital_root::dr;
use crate::fibonacci::dr_fib;
use crate::parameters::{MODULUS_R, MODULUS_U, R_CARDINALITY};

/// One row of Appendix A: `(n, dr(n), dr(F_n))`.
pub type Row = (u32, u8, u8);

// ─── Derived: the matrix itself ──────────────────────────────────────

/// The 96 rows of Appendix A.
///
/// **Derived** by enumerating `R(MODULUS_R) = { n ∈ [1, MODULUS_R] :
/// gcd(n, MODULUS_U) = 1 }` and pairing each `n` with `(dr(n), dr(F_n))`.
pub const ROWS: [Row; R_CARDINALITY] = compute_rows();

const fn compute_rows() -> [Row; R_CARDINALITY] {
    let mut out = [(0u32, 0u8, 0u8); R_CARDINALITY];
    let mut idx = 0usize;
    let mut n: u64 = 1;
    while n <= MODULUS_R {
        if gcd(n, MODULUS_U) == 1 {
            #[allow(clippy::cast_possible_truncation)]
            let n_u32 = n as u32;
            out[idx] = (n_u32, dr(n), dr_fib(n));
            idx += 1;
        }
        n += 1;
    }
    out
}

// Compile-time check: exactly R_CARDINALITY rows were emitted.
const _: () = {
    let mut count = 0usize;
    let mut i = 0usize;
    while i < ROWS.len() {
        if ROWS[i].0 != 0 {
            count += 1;
        }
        i += 1;
    }
    assert!(count == R_CARDINALITY, "compute_rows emitted wrong row count");
};

// ─── Verification: derived rows must match the Appendix A reference ────────────

/// **Verification data only.** Verbatim transcription of the 96 rows
/// of Appendix A from the Appendix A reference table.
///
/// Production code consults [`ROWS`] (the derived version). This
/// constant exists solely for compile-time cross-checking against
/// Gary's published values.
pub const FIXED_PDF_VERIFICATION: [Row; R_CARDINALITY] = [
    (1, 1, 1),
    (7, 7, 4),
    (11, 2, 8),
    (13, 4, 8),
    (17, 8, 4),
    (19, 1, 5),
    (23, 5, 1),
    (29, 2, 5),
    (31, 4, 4),
    (37, 1, 8),
    (41, 5, 4),
    (43, 7, 5),
    (47, 2, 1),
    (49, 4, 1),
    (53, 8, 5),
    (59, 5, 8),
    (61, 7, 8),
    (67, 4, 5),
    (71, 8, 1),
    (73, 1, 1),
    (77, 5, 5),
    (79, 7, 4),
    (83, 2, 8),
    (89, 8, 4),
    (91, 1, 5),
    (97, 7, 1),
    (101, 2, 5),
    (103, 4, 4),
    (107, 8, 8),
    (109, 1, 8),
    (113, 5, 4),
    (119, 2, 1),
    (121, 4, 1),
    (127, 1, 4),
    (131, 5, 8),
    (133, 7, 8),
    (137, 2, 4),
    (139, 4, 5),
    (143, 8, 1),
    (149, 5, 5),
    (151, 7, 4),
    (157, 4, 8),
    (161, 8, 4),
    (163, 1, 5),
    (167, 5, 1),
    (169, 7, 1),
    (173, 2, 5),
    (179, 8, 8),
    (181, 1, 8),
    (187, 7, 5),
    (191, 2, 1),
    (193, 4, 1),
    (197, 8, 5),
    (199, 1, 4),
    (203, 5, 8),
    (209, 2, 4),
    (211, 4, 5),
    (217, 1, 1),
    (221, 5, 5),
    (223, 7, 4),
    (227, 2, 8),
    (229, 4, 8),
    (233, 8, 4),
    (239, 5, 1),
    (241, 7, 1),
    (247, 4, 4),
    (251, 8, 8),
    (253, 1, 8),
    (257, 5, 4),
    (259, 7, 5),
    (263, 2, 1),
    (269, 8, 5),
    (271, 1, 4),
    (277, 7, 8),
    (281, 2, 4),
    (283, 4, 5),
    (287, 8, 1),
    (289, 1, 1),
    (293, 5, 5),
    (299, 2, 8),
    (301, 4, 8),
    (307, 1, 5),
    (311, 5, 1),
    (313, 7, 1),
    (317, 2, 5),
    (319, 4, 4),
    (323, 8, 8),
    (329, 5, 4),
    (331, 7, 5),
    (337, 4, 1),
    (341, 8, 5),
    (343, 1, 4),
    (347, 5, 8),
    (349, 7, 8),
    (353, 2, 4),
    (359, 8, 1),
];

// Compile-time enforcement: the derived ROWS exactly match the
// transcribed Appendix A reference data.
const _: () = {
    let mut i = 0usize;
    while i < R_CARDINALITY {
        let (a_n, a_dr, a_fib) = ROWS[i];
        let (b_n, b_dr, b_fib) = FIXED_PDF_VERIFICATION[i];
        assert!(a_n == b_n, "ROWS[i].n disagrees with reference table");
        assert!(a_dr == b_dr, "ROWS[i].dr disagrees with reference table");
        assert!(a_fib == b_fib, "ROWS[i].dr_fib disagrees with reference table");
        i += 1;
    }
};

// ─── Derived: the column sums ────────────────────────────────────────

/// **Derived.** `Σ dr(n)` over the 96 rows, computed by summing column 2
/// of [`ROWS`].
pub const SUM_DR: u32 = sum_dr_column();

/// **Derived.** `Σ dr(F_n)` over the 96 rows, computed by summing
/// column 3 of [`ROWS`].
pub const SUM_FIBO_DR: u32 = sum_fibo_dr_column();

const fn sum_dr_column() -> u32 {
    let mut s = 0u32;
    let mut i = 0usize;
    while i < ROWS.len() {
        s += ROWS[i].1 as u32;
        i += 1;
    }
    s
}

const fn sum_fibo_dr_column() -> u32 {
    let mut s = 0u32;
    let mut i = 0usize;
    while i < ROWS.len() {
        s += ROWS[i].2 as u32;
        i += 1;
    }
    s
}

// ─── Verification: derived sums must equal Gary's stated 432 ─────────

/// **Verification data only.** Gary's stated `Σ dr(n) = 432` from the
/// Appendix A reference table "TOTAL" row.
pub const GARY_STATED_TOTAL_DR: u32 = 432;

/// **Verification data only.** Gary's stated `Σ dr(F_n) = 432` from the
/// Appendix A reference table "TOTAL" row.
pub const GARY_STATED_TOTAL_FIBO_DR: u32 = 432;

const _: () = assert!(
    SUM_DR == GARY_STATED_TOTAL_DR,
    "Σ dr(n) over R(MODULUS_R) does not equal Gary's stated 432"
);
const _: () = assert!(
    SUM_FIBO_DR == GARY_STATED_TOTAL_FIBO_DR,
    "Σ dr(F_n) over R(MODULUS_R) does not equal Gary's stated 432"
);

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    /// `ROWS` has exactly `R_CARDINALITY` entries.
    #[test]
    fn row_count_is_r_cardinality() {
        assert_eq!(ROWS.len(), R_CARDINALITY);
    }

    /// Every `n` lies in `[1, MODULUS_R]` and is coprime to `MODULUS_U`.
    #[test]
    fn every_n_is_in_r_lattice() {
        for &(n, _, _) in &ROWS {
            assert!(
                u64::from(n) >= 1 && u64::from(n) <= MODULUS_R,
                "n = {n} out of [1, MODULUS_R]"
            );
            assert_eq!(gcd(u64::from(n), MODULUS_U), 1, "{n} not coprime to MODULUS_U");
        }
    }

    /// Rows are in strictly ascending `n` order.
    #[test]
    fn rows_strictly_ascending() {
        for window in ROWS.windows(2) {
            assert!(
                window[0].0 < window[1].0,
                "rows not strictly ascending at ({}, {})",
                window[0].0,
                window[1].0
            );
        }
    }

    /// Derived `ROWS` matches Gary's verbatim transcription.
    /// (The compile-time assert is the load-bearing check.)
    #[test]
    fn derived_rows_match_fixed_pdf() {
        for (i, (a, b)) in ROWS.iter().zip(FIXED_PDF_VERIFICATION.iter()).enumerate() {
            assert_eq!(a, b, "row {i} mismatch: derived {a:?}, Appendix A reference {b:?}");
        }
    }

    /// `Σ dr(n) = 432`.
    #[test]
    fn sum_dr_equals_432() {
        assert_eq!(SUM_DR, 432);
        assert_eq!(SUM_DR, GARY_STATED_TOTAL_DR);
    }

    /// `Σ dr(F_n) = 432`.
    #[test]
    fn sum_fibo_dr_equals_432() {
        assert_eq!(SUM_FIBO_DR, 432);
        assert_eq!(SUM_FIBO_DR, GARY_STATED_TOTAL_FIBO_DR);
    }

    /// **Structural derivation of `Σ dr(n) = 432`.**
    ///
    /// Stratify `R(MODULUS_R)` by `U(MODULUS_U)` residue. For each
    /// `u ∈ U(MODULUS_U)`, the dr values of `u + MODULUS_U · k` for
    /// `k ∈ [0, LATTICE_LAYERS)` cycle through three residues mod
    /// `MODULUS_DR` (because `MODULUS_U mod MODULUS_DR = 30 mod 9 = 3`),
    /// each appearing `LATTICE_LAYERS / 3 = 4` times. The cycle sum is
    /// `12` for `u ≡ 1 (mod 3)` and `15` for `u ≡ 2 (mod 3)`. `U(30)`
    /// splits 4–4 between these classes, so the total is
    /// `4 · 4 · 12 + 4 · 4 · 15 = 192 + 240 = 432`.
    #[test]
    fn sum_dr_matches_structural_derivation() {
        use crate::parameters::LATTICE_LAYERS;
        use crate::u30::RESIDUES;

        let mut count_u_mod3_eq_1 = 0u32;
        let mut count_u_mod3_eq_2 = 0u32;
        for &u in &RESIDUES {
            match u % 3 {
                1 => count_u_mod3_eq_1 += 1,
                2 => count_u_mod3_eq_2 += 1,
                _ => panic!("U(30) residue {u} divisible by 3"),
            }
        }
        assert_eq!(count_u_mod3_eq_1, 4);
        assert_eq!(count_u_mod3_eq_2, 4);

        // 12 = 1 + 4 + 7 (drs cycled by +3 mod 9 starting from 1)
        // 15 = 2 + 5 + 8 (drs cycled by +3 mod 9 starting from 2)
        let cycle_sum_class_1: u32 = 1 + 4 + 7;
        let cycle_sum_class_2: u32 = 2 + 5 + 8;
        let cycles_per_layer = (LATTICE_LAYERS as u32) / 3;

        let derived_sum = count_u_mod3_eq_1 * cycles_per_layer * cycle_sum_class_1
            + count_u_mod3_eq_2 * cycles_per_layer * cycle_sum_class_2;
        assert_eq!(derived_sum, SUM_DR);
        assert_eq!(derived_sum, 432);
    }

    /// **Structural derivation of `Σ dr(F_n) = 432`.**
    ///
    /// `R(MODULUS_R) mod PISANO_PERIOD_MOD_DR` hits each residue
    /// class in `U(PISANO_PERIOD_MOD_DR) = U(24)` with multiplicity
    /// 12. The sum `Σ_{u ∈ U(24)} dr(F_u) = 36` by the palindrome
    /// identity. Hence `Σ dr(F_n) = 12 · 36 = 432`.
    #[test]
    fn sum_fibo_dr_matches_structural_derivation() {
        use crate::fibonacci::dr_fib as dr_fib_fn;

        // U(24) = {1, 5, 7, 11, 13, 17, 19, 23}.
        let u24: [u64; 8] = [1, 5, 7, 11, 13, 17, 19, 23];

        // Multiplicity 12 for each U(24) class within R(360).
        // (Compile-time R_CARDINALITY / 8 = 96/8 = 12.)
        let multiplicity: u32 = (R_CARDINALITY / u24.len()) as u32;
        assert_eq!(multiplicity, 12);

        let sum_dr_fib_u24: u32 = u24.iter().map(|&u| u32::from(dr_fib_fn(u))).sum();
        assert_eq!(sum_dr_fib_u24, 36);

        let derived_sum = multiplicity * sum_dr_fib_u24;
        assert_eq!(derived_sum,