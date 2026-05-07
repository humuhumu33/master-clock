//! # Wheel-30 sieve — Gary's published prime-counting algorithm
//!
//! This module implements the U(30) wheel sieve as documented in Gary
//! William Croft's *Prime Spiral Sieve* (primesdemystified.com,
//! `Factorization_Computation_Method_Zero_to_10000.xlsx`,
//! `89_Squared_First_1000_Primes.xlsx`). It is the algorithm actually
//! present in his published spreadsheets — pure integer arithmetic on
//! the eight residue classes coprime to 30, no transcendental functions
//! and no floating-point arithmetic.
//!
//! ## The published algorithm
//!
//! The reduced residue system [`crate::u30::RESIDUES`] is closed under
//! multiplication mod 30. Every prime `p > 5` lies in one of the eight
//! classes. Composites in those classes are exactly products of two or
//! more elements of `U(30)` (each ≥ 7, since 1 is the multiplicative
//! identity).
//!
//! Therefore `π(N) = 3 + #{ primes in U(30) ∩ [1, N] }`, the `+3`
//! covering the three primes `{2, 3, 5}` that are not in `U(30)`. The
//! sieve marks composites by iterating `p · q` for `p, q ∈ U(30)` with
//! `p ≤ q` and `p·q ≤ N`.
//!
//! ## Memory layout
//!
//! One bit per `U(30)` member up to `N`: `8 · ⌈N/30⌉` bits, i.e.
//! `≈ N/30` bytes. At `N = 10⁹` that is 33 MiB; at `N = 10¹⁰`,
//! 333 MiB; at `N = 10¹²`, 33 GiB (out of reach for in-memory; would
//! require segmentation).
//!
//! ## UOR primitive bindings
//!
//! | Object | UOR ontology IRI |
//! |---|---|
//! | `is_prime_table` | `https://uor.foundation/recursion/BoundedRecursion` (descent measure = sieve index) |
//! | wheel-step iterator | `https://uor.foundation/stream/ProductiveStream` |
//! | residue lookup | `https://uor.foundation/query/CoordinateQuery` (8-room mod-30 lookup) |
//!
//! Each IRI exists in the cloned UOR-Framework spec under
//! `spec/src/namespaces/{recursion,stream,query}.rs`.
//!
//! ## Discipline
//!
//! - `alloc` feature only — uses `Vec<bool>` for the sieve table.
//! - No floating-point arithmetic.
//! - No `unsafe`. No `unwrap`/`expect`/`panic` in production paths.
//! - All `clippy::pedantic` warnings respected.

#![cfg(feature = "alloc")]

use alloc::vec;
use alloc::vec::Vec;

use crate::parameters::{MODULUS_U, U30_CARDINALITY};
use crate::u30::RESIDUES;

/// Index of a `U(30)` residue within [`crate::u30::RESIDUES`].
///
/// Returns `usize::MAX` if `r` is not in `U(30)`. Callers that hand
/// only valid residues never observe the sentinel.
#[must_use]
const fn residue_index(r: u64) -> usize {
    let mut i = 0;
    while i < U30_CARDINALITY {
        if RESIDUES[i] == r {
            return i;
        }
        i += 1;
    }
    usize::MAX
}

/// Lookup table: `MOD30_TO_INDEX[r] = i` such that `RESIDUES[i] = r`,
/// or `usize::MAX` if `gcd(r, 30) ≠ 1`. Computed once at compile time.
const MOD30_TO_INDEX: [usize; 30] = {
    let mut tbl = [usize::MAX; 30];
    let mut r = 0u64;
    while r < MODULUS_U {
        tbl[r as usize] = residue_index(r);
        r += 1;
    }
    tbl
};

/// Integer square root, rounded down. Used to bound the sieve loop at
/// `√N` without resorting to floating-point arithmetic.
#[must_use]
const fn isqrt(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    // Newton's method seeded with a power-of-two upper bound.
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

/// Number of `U(30)` rooms required to cover `[1, N]`.
#[must_use]
const fn room_count(n: u64) -> u64 {
    n / MODULUS_U + 1
}

/// One bit per `U(30)` member up to `N`. Bit `(b · 8 + i)` corresponds
/// to the integer `b · 30 + RESIDUES[i]`.
#[must_use]
fn build_table(n: u64) -> Vec<bool> {
    let blocks = room_count(n) as usize;
    vec![true; blocks * U30_CARDINALITY]
}

/// Mark composites in `is_prime` by iterating `p · q` over `U(30) × U(30)`
/// with `p ≤ q` and `p · q ≤ n`.
fn mark_composites(is_prime: &mut [bool], n: u64) {
    let sqrt_n = isqrt(n);
    let blocks = room_count(n) as usize;

    // 1 is not prime — it is `RESIDUES[0]` in block 0.
    if !is_prime.is_empty() {
        is_prime[0] = false;
    }

    // Outer: walk every U(30) member up to √N in increasing order.
    for p_block in 0..blocks {
        for p_off in 0..U30_CARDINALITY {
            let p = (p_block as u64) * MODULUS_U + RESIDUES[p_off];
            if p > sqrt_n {
                return;
            }
            if p < 7 {
                continue;
            }
            let p_idx = p_block * U30_CARDINALITY + p_off;
            if !is_prime[p_idx] {
                continue;
            }

            // Inner: mark p·q for all U(30) members q ≥ p with p·q ≤ n.
            let mut q_block = p_block;
            let mut q_off_start = p_off;
            'outer: loop {
                for q_off in q_off_start..U30_CARDINALITY {
                    let q = (q_block as u64) * MODULUS_U + RESIDUES[q_off];
                    let prod = match p.checked_mul(q) {
                        Some(v) if v <= n => v,
                        _ => break 'outer,
                    };
                    let prod_block = (prod / MODULUS_U) as usize;
                    let prod_off = MOD30_TO_INDEX[(prod % MODULUS_U) as usize];
                    let prod_idx = prod_block * U30_CARDINALITY + prod_off;
                    if prod_idx < is_prime.len() {
                        is_prime[prod_idx] = false;
                    }
                }
                q_block += 1;
                q_off_start = 0;
                let next_min = match (q_block as u64).checked_mul(MODULUS_U) {
                    Some(v) => v,
                    None => break,
                };
                if next_min.saturating_mul(p) > n {
                    break;
                }
            }
        }
    }
}

/// Count primes in the sieve table that fall within `[1, n]`.
fn count_primes_in_table(is_prime: &[bool], n: u64) -> u64 {
    let mut count: u64 = 0;
    let blocks = room_count(n) as usize;
    for b in 0..blocks {
        for off in 0..U30_CARDINALITY {
            let val = (b as u64) * MODULUS_U + RESIDUES[off];
            if val > n {
                return count;
            }
            if val < 7 {
                continue;
            }
            let idx = b * U30_CARDINALITY + off;
            if idx < is_prime.len() && is_prime[idx] {
                count += 1;
            }
        }
    }
    count
}

/// Count of primes in `U(30)` (i.e. ≥ 7) up to `N`.
///
/// This is the inner kernel of [`pi_via_u30_sieve`]. Public so callers
/// can verify the structural identity
/// `π(N) = primes_in_u30(N) + 3` directly.
#[must_use]
pub fn primes_in_u30(n: u64) -> u64 {
    if n < 7 {
        return 0;
    }
    let mut tbl = build_table(n);
    mark_composites(&mut tbl, n);
    count_primes_in_table(&tbl, n)
}

/// `π(N)` — exact prime count via Gary's published U(30) sieve.
///
/// `π(N) = #{ primes in U(30) ∩ [1, N] } + 3`, where the `+3`
/// accounts for the three primes `{2, 3, 5}` not in `U(30)`.
///
/// This is *exactly* the algorithm Gary's spreadsheets implement,
/// scaled to large `N`. Returns the literature value of `π(N)` with
/// zero error — no χ, no ζ, no δ, no residual.
#[must_use]
pub fn pi_via_u30_sieve(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }
    if n < 3 {
        return 1;
    }
    if n < 5 {
        return 2;
    }
    if n < 7 {
        return 3;
    }
    primes_in_u30(n) + 3
}

/// Number of integers in `[1, N]` coprime to 30.
///
/// `D₃₀(N) = 8 · ⌊N/30⌋ + |{ r ∈ U(30) : r ≤ N mod 30 }|`.
///
/// Together with `C₃₀(N) = D₃₀(N) − π(N) + 2` (composites in U(30)
/// up to N), this gives the structural decomposition
/// `π(N) = D₃₀(N) − C₃₀(N) + 2`. The `+2` is `+3` for `{2, 3, 5}`
/// minus `−1` for the non-prime `1 ∈ U(30)`.
#[must_use]
pub const fn d30(n: u64) -> u64 {
    let q = n / MODULUS_U;
    let r = n % MODULUS_U;
    let mut count = 0u64;
    let mut i = 0;
    while i < U30_CARDINALITY {
        if RESIDUES[i] <= r {
            count += 1;
        }
        i += 1;
    }
    8 * q + count
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]

    use super::*;

    #[test]
    fn isqrt_spot_checks() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(1), 1);
        assert_eq!(isqrt(2), 1);
        assert_eq!(isqrt(99), 9);
        assert_eq!(isqrt(100), 10);
        assert_eq!(isqrt(10_000), 100);
        assert_eq!(isqrt(1_000_000_000), 31_622);
    }

    #[test]
    fn d30_at_30_equals_8() {
        assert_eq!(d30(30), 8);
    }

    #[test]
    fn d30_at_300_equals_80() {
        assert_eq!(d30(300), 80);
    }

    #[test]
    fn d30_at_10000_matches_published() {
        // Gary's spreadsheet states ⌊10000 · 8/30⌋ = 2666; D₃₀(10000)
        // counts the same set ≤ 10000 exactly.
        assert_eq!(d30(10_000), 2_667);
    }

    #[test]
    fn pi_at_100_equals_25() {
        assert_eq!(pi_via_u30_sieve(100), 25);
    }

    #[test]
    fn pi_at_1000_equals_168() {
        assert_eq!(pi_via_u30_sieve(1_000), 168);
    }

    #[test]
    fn pi_at_7919_equals_1000() {
        // Gary's headline anchor: 7919 is the 1000th prime, == 89² − 2.
        assert_eq!(pi_via_u30_sieve(7_919), 1_000);
    }

    #[test]
    fn pi_at_10000_equals_1229() {
        assert_eq!(pi_via_u30_sieve(10_000), 1_229);
    }
}
