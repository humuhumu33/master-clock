//! # Replicant factorization counts
//!
//! For `m ∈ [1, N]`, consider all unordered factorizations
//! `(a, b)` with `a, b ∈ U_30(∞)`, `7 ≤ a ≤ b`, `a · b = m`. Two
//! quantities matter:
//!
//! - **Distinct replicant integers.** Count of `m ≤ N` admitting
//!   at least two such factorizations.
//! - **Duplicate factorizations.** Total factorizations minus
//!   unique products = `Σ_{m ≤ N} (count(m) − 1)` summed over
//!   `m` with `count(m) ≥ 2`.
//!
//! primesdemystified.com (countingprimes.html) tabulates the
//! second metric:
//!
//! ```text
//!   duplicate_factorization_count(1000)  =   5
//!   duplicate_factorization_count(10000) = 408
//! ```
//!
//! At `N = 1000` the two metrics coincide because every replicant
//! has exactly two factorizations. At `N = 10000` they diverge —
//! some products acquire three or more factorizations.
//!
//! At `N = 1000` the five replicant integers and their double
//! factorizations are:
//!
//! ```text
//!   539 =  7 · 77 = 11 · 49
//!   637 =  7 · 91 = 13 · 49
//!   833 =  7 · 119 = 17 · 49
//!   847 =  7 · 121 = 11 · 77
//!   931 =  7 · 133 = 19 · 49
//! ```
//!
//! ## Anchor — derived from Phase 1
//!
//! Pure enumeration over integers coprime to 30 in `[7, √N]` ×
//! `[7, N/a]`, using only `gcd` and integer multiplication.
//!
//! ## UOR primitive bindings
//!
//! | Object | UOR ontology IRI |
//! |---|---|
//! | factor enumeration | `https://uor.foundation/recursion/BoundedRecursion` |
//! | factorization fiber | `https://uor.foundation/morphism/Fibration` |
//! | count results | `https://uor.foundation/cert/RuntimeCertificate` |

#![cfg(feature = "alloc")]

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::arithmetic::gcd;
use crate::parameters::MODULUS_U;

/// Compute the factorization-multiplicity table for products in
/// `[1, N]`. Key = product `m`, value = number of unordered pairs
/// `(a, b)` with `a, b ∈ U_30(∞)`, `7 ≤ a ≤ b`, `a · b = m`.
fn factorization_table(n: u64) -> BTreeMap<u64, u64> {
    let mut counts: BTreeMap<u64, u64> = BTreeMap::new();
    if n < 49 {
        return counts;
    }
    let mut a = 7u64;
    while a * a <= n {
        if gcd(a, MODULUS_U) == 1 {
            let mut b = a;
            while a * b <= n {
                if gcd(b, MODULUS_U) == 1 {
                    *counts.entry(a * b).or_insert(0) += 1;
                }
                b += 2;
            }
        }
        a += 2;
    }
    counts
}

/// Distinct replicant integers in `[1, N]` — those with at least
/// two unordered factorizations into `U_30(∞)` members ≥ 7.
/// Returned in ascending order.
#[must_use]
pub fn replicant_products(n: u64) -> Vec<u64> {
    let counts = factorization_table(n);
    let mut out: Vec<u64> = counts
        .into_iter()
        .filter(|&(_, c)| c >= 2)
        .map(|(p, _)| p)
        .collect();
    out.sort_unstable();
    out
}

/// Count of distinct replicant integers in `[1, N]`. At `N = 1000`
/// equals 5; at `N = 10000` equals 248.
#[must_use]
pub fn replicant_count_distinct(n: u64) -> u64 {
    replicant_products(n).len() as u64
}

/// **Duplicate-factorization count** — `Σ (count(m) − 1)` over all
/// `m ∈ [1, N]` with `count(m) ≥ 2`. Equivalently:
/// `total_factorizations − unique_products`.
///
/// This matches primesdemystified.com's tabulated values:
/// `5` at `N = 1000`, `408` at `N = 10000`.
#[must_use]
pub fn duplicate_factorization_count(n: u64) -> u64 {
    let counts = factorization_table(n);
    counts.values().filter(|&&c| c >= 2).map(|&c| c - 1).sum()
}

/// Total ordered-with-`a ≤ b` factorization pairs in `[1, N]`.
#[must_use]
pub fn total_factorization_pairs(n: u64) -> u64 {
    factorization_table(n).values().sum()
}

/// Number of distinct products that arise as `a · b` with
/// `a, b ∈ U_30(∞)`, `7 ≤ a ≤ b`, `a · b ≤ N`.
#[must_use]
pub fn unique_factorization_products(n: u64) -> u64 {
    factorization_table(n).len() as u64
}

/// **Reference value** (countingprimes.html):
/// `duplicate_factorization_count(1000) = 5`.
pub const DUPLICATE_FACTORIZATIONS_AT_1000: u64 = 5;

/// **Reference value** (countingprimes.html):
/// `duplicate_factorization_count(10000) = 408`.
pub const DUPLICATE_FACTORIZATIONS_AT_10000: u64 = 408;

/// Enumerated replicant products in `[1, 1000]`.
pub const REPLICANTS_AT_1000: [u64; 5] = [539, 637, 833, 847, 931];

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
    use super::*;

    #[test]
    fn no_replicants_below_100() {
        assert_eq!(replicant_count_distinct(100), 0);
        assert_eq!(duplicate_factorization_count(100), 0);
    }

    #[test]
    fn replicant_set_at_1000_canonical() {
        assert_eq!(replicant_products(1_000), REPLICANTS_AT_1000);
    }

    #[test]
    fn distinct_count_at_1000_eq_5() {
        assert_eq!(replicant_count_distinct(1_000), 5);
    }

    #[test]
    fn duplicate_count_at_1000_eq_5() {
        // At N = 1000 each replicant has exactly 2 factorizations,
        // so distinct count = duplicate count.
        assert_eq!(
            duplicate_factorization_count(1_000),
            DUPLICATE_FACTORIZATIONS_AT_1000
        );
    }

    #[test]
    fn duplicate_count_at_10000_eq_408() {
        assert_eq!(
            duplicate_factorization_count(10_000),
            DUPLICATE_FACTORIZATIONS_AT_10000
        );
    }

    #[test]
    fn distinct_count_diverges_at_10000() {
        // Distinct replicant integers at N = 10000 is 248 (not 408).
        // Some products have three or more factorizations, accounting
        // for the gap.
        let distinct = replicant_count_distinct(10_000);
        let duplicates = duplicate_factorization_count(10_000);
        assert_eq!(distinct, 248);
        assert_eq!(duplicates, 408);
        assert!(duplicates > distinct);
    }

    #[test]
    fn duplicate_equals_total_minus_unique() {
        for &n in &[100u64, 500, 1_000, 5_000, 10_000] {
            let total = total_factorization_pairs(n);
            let unique = unique_factorization_products(n);
            let duplicates = duplicate_factorization_count(n);
            assert_eq!(
                total - unique,
                duplicates,
                "N={n}: total={total}, unique={unique}, dups={duplicates}"
            );
        }
    }

    #[test]
    fn each_replicant_at_1000_has_two_factorizations() {
        let cases: &[(u64, u64, u64, u64, u64)] = &[
            (539, 7, 77, 11, 49),
            (637, 7, 91, 13, 49),
            (833, 7, 119, 17, 49),
            (847, 7, 121, 11, 77),
            (931, 7, 133, 19, 49),
        ];
        for &(p, a1, b1, a2, b2) in cases {
            assert_eq!(a1 * b1, p);
            assert_eq!(a2 * b2, p);
            for &x in &[a1, b1, a2, b2] {
                assert_eq!(gcd(x, MODULUS_U), 1);
                assert!(x >= 7);
            }
        }
    }

    #[test]
    fn duplicate_count_monotone() {
        let mut prev = 0u64;
        for n in (100..=10_000).step_by(500) {
            let c = duplicate_factorization_count(n);
            assert!(c >= prev, "duplicate count not monotone at N={n}");
            prev = c;
        }
    }
}
