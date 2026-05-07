//! # `(Z/9Z)*` — the six units modulo 9, and their multiplication table
//!
//! `(Z/9Z)* = { n ∈ [1, 9) : gcd(n, 9) = 1 } = {1, 2, 4, 5, 7, 8}`.
//! The multiplicative group of integers mod 9. Order 6, cyclic, with
//! generator 2 (since `2^k mod 9` cycles through every unit before
//! returning to 1).
//!
//! This module is the digital-root counterpart of [`crate::u30`]:
//! where `U(30)` is the residue substrate of the wheel-30 sieve,
//! `(Z/9Z)*` is the residue substrate of the digital-root cycle on
//! [`crate::u90`]. Anchored to one parameter only:
//! `MODULUS_DR = NUMERAL_BASE − 1 = 9`.
//!
//! ## What this module establishes
//!
//! Every claim is decided at compile time:
//!
//! 1. The six units of `Z/9Z` are exactly `{1, 2, 4, 5, 7, 8}`.
//! 2. The group is cyclic of order 6, with `2` as a generator.
//! 3. The 6×6 multiplication table modulo 9 is closed in those six
//!    values — a Latin square (every row and column is a permutation
//!    of the units).
//! 4. Every row and every column sums to `27 = 3 · MODULUS_DR`.
//! 5. The total sum of the table is `162 = 6 · 27`.
//! 6. Additive-inverse pairs are `(1,8)`, `(2,7)`, `(4,5)`, each
//!    summing to `9`. (These are NOT multiplicative inverses.)
//! 7. Multiplicative-inverse pairs are `(1,1)`, `(2,5)`, `(4,7)`,
//!    `(8,8)`, each multiplying to `1` mod 9.
//!
//! ## Anchor to Phase 1
//!
//! The 24-element digital-root cycle [`crate::u90::DR_CYCLE`] takes
//! values exclusively in `(Z/9Z)*` — that is the structural reason
//! exactly four copies of each element of `{1, 2, 4, 5, 7, 8}`
//! appear in the cycle.
//!
//! ## UOR primitive bindings
//!
//! | Object | UOR ontology IRI |
//! |---|---|
//! | `UNITS_Z9` | `https://uor.foundation/stream/ProductiveStream` (six ordered units) |
//! | `MULT_TABLE_Z9` | `https://uor.foundation/query/CoordinateQuery` (6×6 lookup) |
//! | generator-2 cycle | `https://uor.foundation/recursion/BoundedRecursion` |

use crate::arithmetic::gcd;
use crate::parameters::MODULUS_DR;

/// Cardinality of `(Z/9Z)*` — derived. Equals
/// `MODULUS_DR · (1 − 1/3) = 6` (Euler's totient at 9).
pub const UNITS_Z9_CARDINALITY: usize = 6;

/// The six units of `Z/9Z`, computed at compile time from
/// `MODULUS_DR = 9` via gcd.
pub const UNITS_Z9: [u64; UNITS_Z9_CARDINALITY] = compute_units_z9();

const fn compute_units_z9() -> [u64; UNITS_Z9_CARDINALITY] {
    let mut out = [0u64; UNITS_Z9_CARDINALITY];
    let mut idx = 0usize;
    let mut n = 1u64;
    while n < MODULUS_DR {
        if gcd(n, MODULUS_DR) == 1 {
            out[idx] = n;
            idx += 1;
        }
        n += 1;
    }
    out
}

const _: () = {
    let expected: [u64; 6] = [1, 2, 4, 5, 7, 8];
    let mut i = 0;
    while i < UNITS_Z9_CARDINALITY {
        assert!(UNITS_Z9[i] == expected[i], "(Z/9Z)* derivation drift");
        i += 1;
    }
};

/// **Cyclic-of-order-6 witness.** Successive powers of `2` mod 9.
/// Equals `[2, 4, 8, 7, 5, 1]` — every unit of `(Z/9Z)*` is a power
/// of 2, hence the group is cyclic with generator 2.
pub const POWERS_OF_2_MOD_9: [u64; 6] = compute_powers_of_2_mod_9();

const fn compute_powers_of_2_mod_9() -> [u64; 6] {
    let mut out = [0u64; 6];
    let mut x = 1u64;
    let mut i = 0usize;
    while i < 6 {
        x = (x * 2) % MODULUS_DR;
        out[i] = x;
        i += 1;
    }
    out
}

const _: () = {
    let expected: [u64; 6] = [2, 4, 8, 7, 5, 1];
    let mut i = 0;
    while i < 6 {
        assert!(POWERS_OF_2_MOD_9[i] == expected[i], "cyclic witness drift");
        i += 1;
    }
};

/// 6×6 multiplication table modulo 9: `MULT_TABLE_Z9[i][j] =
/// (UNITS_Z9[i] · UNITS_Z9[j]) mod 9`. Every entry is in `UNITS_Z9`
/// (closure / Latin-square property).
pub const MULT_TABLE_Z9: [[u64; UNITS_Z9_CARDINALITY]; UNITS_Z9_CARDINALITY] =
    compute_mult_table_z9();

const fn compute_mult_table_z9()
    -> [[u64; UNITS_Z9_CARDINALITY]; UNITS_Z9_CARDINALITY]
{
    let mut out = [[0u64; UNITS_Z9_CARDINALITY]; UNITS_Z9_CARDINALITY];
    let mut i = 0usize;
    while i < UNITS_Z9_CARDINALITY {
        let a = UNITS_Z9[i];
        let mut j = 0usize;
        while j < UNITS_Z9_CARDINALITY {
            let b = UNITS_Z9[j];
            let prod = (a * b) % MODULUS_DR;
            // Map 0 → 9 in digital-root convention. (a·b) mod 9 cannot
            // be 0 here since a, b are units. Defensive only.
            out[i][j] = if prod == 0 { MODULUS_DR } else { prod };
            j += 1;
        }
        i += 1;
    }
    out
}

/// Row sum of [`MULT_TABLE_Z9`]: `27 = 3 · MODULUS_DR`. Every row
/// hits this sum because each row is a permutation of `UNITS_Z9` and
/// `Σ UNITS_Z9 = 1+2+4+5+7+8 = 27`.
pub const MULT_TABLE_ROW_SUM: u64 = 3 * MODULUS_DR;

/// Total sum of [`MULT_TABLE_Z9`]: `162 = 6 · 27 =
/// |UNITS_Z9| · MULT_TABLE_ROW_SUM`.
pub const MULT_TABLE_TOTAL_SUM: u64 =
    (UNITS_Z9_CARDINALITY as u64) * MULT_TABLE_ROW_SUM;

const _: () = {
    let mut i = 0;
    while i < UNITS_Z9_CARDINALITY {
        let mut row_sum = 0u64;
        let mut col_sum = 0u64;
        let mut j = 0usize;
        while j < UNITS_Z9_CARDINALITY {
            row_sum += MULT_TABLE_Z9[i][j];
            col_sum += MULT_TABLE_Z9[j][i];
            // Closure: every entry is a unit.
            let v = MULT_TABLE_Z9[i][j];
            let mut found = false;
            let mut k = 0usize;
            while k < UNITS_Z9_CARDINALITY {
                if UNITS_Z9[k] == v {
                    found = true;
                }
                k += 1;
            }
            assert!(found, "table entry escapes (Z/9Z)*");
            j += 1;
        }
        assert!(row_sum == MULT_TABLE_ROW_SUM, "row sum drift");
        assert!(col_sum == MULT_TABLE_ROW_SUM, "col sum drift");
        i += 1;
    }
};

/// Linear sum of `UNITS_Z9` = 27. Equal to every row/column sum
/// of [`MULT_TABLE_Z9`] by the Latin-square property.
pub const UNITS_Z9_SUM: u64 = compute_units_z9_sum();

const fn compute_units_z9_sum() -> u64 {
    let mut s = 0u64;
    let mut i = 0usize;
    while i < UNITS_Z9_CARDINALITY {
        s += UNITS_Z9[i];
        i += 1;
    }
    s
}

const _: () = assert!(UNITS_Z9_SUM == 27, "Σ (Z/9Z)* must equal 27");

/// Additive-inverse pairs in `(Z/9Z)*`: `(a, MODULUS_DR − a)`. Each
/// pair sums to `MODULUS_DR = 9`. There are exactly 3 such pairs:
/// `(1,8)`, `(2,7)`, `(4,5)`.
pub const ADDITIVE_INVERSE_PAIRS: [(u64, u64); 3] = [(1, 8), (2, 7), (4, 5)];

/// Multiplicative-inverse pairs in `(Z/9Z)*`. There are 4 pairs
/// (two with `a = b`): `(1,1)`, `(2,5)`, `(4,7)`, `(8,8)`.
pub const MULTIPLICATIVE_INVERSE_PAIRS: [(u64, u64); 4] =
    [(1, 1), (2, 5), (4, 7), (8, 8)];

const _: () = {
    let mut i = 0;
    while i < 3 {
        let (a, b) = ADDITIVE_INVERSE_PAIRS[i];
        assert!(a + b == MODULUS_DR, "additive inverse pair drift");
        i += 1;
    }
    let mut j = 0;
    while j < 4 {
        let (a, b) = MULTIPLICATIVE_INVERSE_PAIRS[j];
        assert!((a * b) % MODULUS_DR == 1, "multiplicative inverse pair drift");
        j += 1;
    }
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
    use super::*;

    #[test]
    fn units_z9_is_canonical() {
        assert_eq!(UNITS_Z9, [1, 2, 4, 5, 7, 8]);
    }

    #[test]
    fn two_generates_units_z9() {
        let mut seen = [false; UNITS_Z9_CARDINALITY];
        for &p in &POWERS_OF_2_MOD_9 {
            for (i, &u) in UNITS_Z9.iter().enumerate() {
                if u == p {
                    seen[i] = true;
                }
            }
        }
        for s in &seen {
            assert!(*s, "2 fails to generate (Z/9Z)*");
        }
    }

    #[test]
    fn mult_table_row_col_sums_27() {
        for i in 0..UNITS_Z9_CARDINALITY {
            let row_sum: u64 = MULT_TABLE_Z9[i].iter().sum();
            assert_eq!(row_sum, 27);
            let col_sum: u64 = (0..UNITS_Z9_CARDINALITY)
                .map(|j| MULT_TABLE_Z9[j][i])
                .sum();
            assert_eq!(col_sum, 27);
        }
    }

    #[test]
    fn mult_table_total_162() {
        let total: u64 = (0..UNITS_Z9_CARDINALITY)
            .flat_map(|i| {
                (0..UNITS_Z9_CARDINALITY).map(move |j| MULT_TABLE_Z9[i][j])
            })
            .sum();
        assert_eq!(total, 162);
    }

    #[test]
    fn additive_inverse_pairs_sum_to_9() {
        for &(a, b) in &ADDITIVE_INVERSE_PAIRS {
            assert_eq!(a + b, 9);
        }
    }

    #[test]
    fn multiplicative_inverse_pairs_product_1_mod_9() {
        for &(a, b) in &MULTIPLICATIVE_INVERSE_PAIRS {
            assert_eq!((a * b) % 9, 1);
        }
    }
}
