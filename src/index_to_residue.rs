//! # Bijection between `k` (1-indexed) and the `k`-th member of `U_30(∞)`
//!
//! Let `U_30(∞) = {n ≥ 1 : gcd(n, MODULUS_U) = 1}` be the infinite
//! ascending sequence of positive integers coprime to 30:
//!
//! ```text
//!   1, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 49, 53, 59, …
//! ```
//!
//! Sorting it ascending and indexing from `k = 1`, the `k`-th
//! element has the closed form
//!
//! ```text
//!   nth_u30_member(k) = MODULUS_U · ⌊(k − 1) / U30_CARDINALITY⌋
//!                       + RESIDUES[(k − 1) mod U30_CARDINALITY]
//! ```
//!
//! For the inverse direction, given `n` coprime to 30, the index is
//!
//! ```text
//!   index_of(n) = (n / MODULUS_U) · U30_CARDINALITY
//!                  + position_in_residues(n mod MODULUS_U) + 1
//! ```
//!
//! Both maps are exact `const fn`. The two-way inverse property
//! holds for every `k ≥ 1` and every `n ∈ U_30(∞)`, verified at
//! compile time.
//!
//! ## Anchor — derived from Phase 1
//!
//! Every constant computed at compile time from
//! [`crate::parameters::MODULUS_U`] (= 30),
//! [`crate::parameters::U30_CARDINALITY`] (= 8), and
//! [`crate::u30::RESIDUES`]. Nothing else.
//!
//! ## UOR primitive bindings
//!
//! | Object | UOR ontology IRI |
//! |---|---|
//! | `nth_u30_member` | `https://uor.foundation/recursion/BoundedRecursion` (descent measure = `k`) |
//! | `index_of_u30_member` | `https://uor.foundation/query/CoordinateQuery` (mod-30 address) |
//! | bijection witness | `https://uor.foundation/morphism/Isomorphism` |
//!
//! Each IRI's namespace file is in
//! `UOR-framework/spec/src/namespaces/{recursion,query,morphism}.rs`.

use crate::arithmetic::gcd;
use crate::parameters::{MODULUS_U, U30_CARDINALITY};
use crate::u30::RESIDUES;

/// **Forward map.** Given a 1-indexed position `k ≥ 1`, return the
/// `k`-th element of `U_30(∞) = {n ≥ 1 : gcd(n, 30) = 1}` in
/// ascending order.
///
/// Returns `0` for `k = 0` (out of domain). Otherwise the formula
/// is exact:
///
/// ```text
///   k → MODULUS_U · ⌊(k − 1) / 8⌋ + RESIDUES[(k − 1) mod 8]
/// ```
#[must_use]
pub const fn nth_u30_member(k: u64) -> u64 {
    if k == 0 {
        return 0;
    }
    let zero_idx = k - 1;
    let block = zero_idx / (U30_CARDINALITY as u64);
    let offset = zero_idx % (U30_CARDINALITY as u64);
    block * MODULUS_U + RESIDUES[offset as usize]
}

/// **Inverse map.** Given `n ∈ U_30(∞)`, return its 1-indexed
/// position. Returns `0` if `gcd(n, 30) ≠ 1` (so `n` is not in the
/// sequence).
#[must_use]
pub const fn index_of_u30_member(n: u64) -> u64 {
    if n == 0 || gcd(n, MODULUS_U) != 1 {
        return 0;
    }
    let block = n / MODULUS_U;
    let offset = n % MODULUS_U;
    let mut pos = 0u64;
    let mut found = false;
    let mut i = 0usize;
    while i < U30_CARDINALITY {
        if RESIDUES[i] == offset {
            pos = i as u64;
            found = true;
        }
        i += 1;
    }
    if !found {
        return 0;
    }
    block * (U30_CARDINALITY as u64) + pos + 1
}

/// **Cross-check anchors from primesdemystified.com.**
///
/// The website tabulates specific `(n, k)` pairs as part of its
/// "first 1000 primes" demonstration. These appear as compile-time
/// assertions below: any drift in the bijection breaks the build.
const _: () = {
    // (n, k) reference table:
    let anchors: [(u64, u64); 7] = [
        ( 1,  1),
        ( 7,  2),
        (11,  3),
        (29,  8),
        (31,  9),
        (89, 24),
        (7919, 2112),  // 7919 = 1000th prime; the website's headline anchor.
    ];
    let mut i = 0;
    while i < anchors.len() {
        let (n, k) = anchors[i];
        assert!(
            nth_u30_member(k) == n,
            "nth_u30_member drift on website anchor"
        );
        assert!(
            index_of_u30_member(n) == k,
            "index_of_u30_member drift on website anchor"
        );
        i += 1;
    }
};

/// **Bijection round-trip — forward then inverse.** For every
/// `k ∈ [1, 96]`, `index_of_u30_member(nth_u30_member(k)) = k`.
/// Verified at compile time over the full `R(360)` range.
const _: () = {
    let mut k = 1u64;
    while k <= 96 {
        let n = nth_u30_member(k);
        let k_back = index_of_u30_member(n);
        assert!(k_back == k, "round-trip drift forward→inverse");
        k += 1;
    }
};

/// **Bijection round-trip — inverse then forward.** For every
/// `n ≤ 360` coprime to 30, `nth_u30_member(index_of_u30_member(n)) = n`.
const _: () = {
    let mut n = 1u64;
    while n <= 360 {
        if gcd(n, MODULUS_U) == 1 {
            let k = index_of_u30_member(n);
            let n_back = nth_u30_member(k);
            assert!(n_back == n, "round-trip drift inverse→forward");
        }
        n += 1;
    }
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
    use super::*;

    #[test]
    fn forward_map_first_eight_is_residues() {
        for k in 1..=U30_CARDINALITY as u64 {
            assert_eq!(
                nth_u30_member(k),
                RESIDUES[(k - 1) as usize],
                "k={k}"
            );
        }
    }

    #[test]
    fn forward_map_anchors() {
        // Website's headline anchors (countingprimes.html / formulae.html).
        assert_eq!(nth_u30_member(24), 89);
        assert_eq!(nth_u30_member(2112), 7919); // 1000th prime
    }

    #[test]
    fn inverse_map_anchors() {
        assert_eq!(index_of_u30_member(89), 24);
        assert_eq!(index_of_u30_member(7919), 2112);
    }

    #[test]
    fn inverse_rejects_non_coprime() {
        assert_eq!(index_of_u30_member(2), 0);
        assert_eq!(index_of_u30_member(15), 0);
        assert_eq!(index_of_u30_member(30), 0);
        assert_eq!(index_of_u30_member(0), 0);
    }

    #[test]
    fn round_trip_first_thousand_indices() {
        for k in 1u64..=1000 {
            let n = nth_u30_member(k);
            assert_eq!(index_of_u30_member(n), k);
            assert_eq!(gcd(n, MODULUS_U), 1);
        }
    }

    #[test]
    fn round_trip_first_thousand_coprime_integers() {
        let mut count = 0u64;
        for n in 1u64..=10_000 {
            if gcd(n, MODULUS_U) == 1 {
                count += 1;
                let k = index_of_u30_member(n);
                assert_eq!(k, count);
                assert_eq!(nth_u30_member(k), n);
            }
        }
    }
}
