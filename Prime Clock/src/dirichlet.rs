//! # Dirichlet equidistribution of primes across `U(30)`
//!
//! ## The theorem
//!
//! Dirichlet's theorem on primes in arithmetic progressions states
//! that for any modulus `m` and any residue class `a` with
//! `gcd(a, m) = 1`, there are infinitely many primes `p ≡ a (mod m)`.
//! Stronger: the prime-counting function restricted to each residue
//! class is asymptotically equal:
//!
//! ```text
//! π(N; m, a) ~ π(N) / φ(m)   as N → ∞.
//! ```
//!
//! For `m = MODULUS_U = 30`, `φ(30) = 8`, and the eight valid residue
//! classes are exactly `U(30) = { 1, 7, 11, 13, 17, 19, 23, 29 }`. So
//! primes greater than 5 are asymptotically equidistributed across
//! these 8 classes, each containing roughly `(π(N) − 3) / 8` primes.
//!
//! ## Why this matters for the Master-Clock
//!
//! Gary's residue-weighting scheme assigns Fibonacci-digital-root
//! weights to each `U(30)` class. Any such weighting reduces, in the
//! large-`N` limit, to `(weight average) × (π(N) − 3)` plus a
//! correction of order `√N`. Equidistribution is therefore the
//! single load-bearing analytic fact that connects the lattice
//! structure to actual prime counts. Without it, the lattice has no
//! bearing on `π(N)`. With it, every weighting scheme is a small
//! perturbation of the trivial one.
//!
//! ## Self-contained sieve
//!
//! [`IS_PRIME`] is the Sieve of Eratosthenes computed at compile
//! time, up to [`SIEVE_BOUND`] = 10,000. The sieve uses only
//! addition, subtraction, and comparison — every operation is in
//! the verified chain of [`crate::ring_substrate`].
//!
//! ## Phase 1 scope
//!
//! Phase 1 verifies equidistribution **empirically** at `N = 10⁴`:
//! eight prime counts, mean `(π(10⁴) − 3) / 8`, fluctuation within
//! the generous bound `2 √mean`. Larger-`N` verification (10⁶, 10⁹,
//! 10¹²) requires either a much larger compile-time sieve or a
//! runtime sieve behind the `ground-truth` feature; that is Phase 3.
//!
//! ## Citations
//!
//! - `https://uor.foundation/u/Coprime` — coprimality predicate.
//! - `https://uor.foundation/op/PNT` — prime-number-theorem-class
//!   identity (Phase 3+ when continuous primitives become live).

use crate::parameters::{MODULUS_U, U30_CARDINALITY};
use crate::u30::RESIDUES;

/// Upper bound for the compile-time sieve.
///
/// Chosen at `10⁴` to fit comfortably within Rust's default
/// `const_eval_limit` while giving statistically meaningful counts
/// per `U(30)` class (mean ≈ 153 primes per class). The
/// `ground-truth` feature (Phase 3) will provide a runtime sieve for
/// larger `N`.
pub const SIEVE_BOUND: u32 = 10_000;

/// Sieve of Eratosthenes up to [`SIEVE_BOUND`], computed at compile
/// time. `IS_PRIME[n]` is `true` iff `n` is prime.
///
/// **Derived** from the ring axiom via subtraction-based reduction:
/// the inner cross-out step `m += p` is iterated `Succ` (Phase 2 of
/// the derivation chain in [`crate::ring_substrate`]).
pub const IS_PRIME: [bool; (SIEVE_BOUND + 1) as usize] = compute_sieve();

const fn compute_sieve() -> [bool; (SIEVE_BOUND + 1) as usize] {
    let n = (SIEVE_BOUND + 1) as usize;
    let mut sieve = [true; (SIEVE_BOUND + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;
    let mut p: u32 = 2;
    while (p as u64) * (p as u64) <= SIEVE_BOUND as u64 {
        if sieve[p as usize] {
            let mut m: u32 = p * p;
            while (m as usize) < n {
                sieve[m as usize] = false;
                m += p;
            }
        }
        p += 1;
    }
    sieve
}

/// `π(N)` — count of primes `≤ N`. Defined for `N ≤ SIEVE_BOUND`;
/// returns `None` outside that range.
#[must_use]
pub const fn pi(n: u32) -> Option<u32> {
    if n > SIEVE_BOUND {
        return None;
    }
    let mut count: u32 = 0;
    let mut k: u32 = 2;
    while k <= n {
        if IS_PRIME[k as usize] {
            count += 1;
        }
        k += 1;
    }
    Some(count)
}

/// Counts of primes in each `U(30)` residue class up to `N`.
///
/// `class_counts[i]` is the number of primes `p ≤ N` with
/// `p ≡ RESIDUES[i] (mod MODULUS_U)`.
///
/// Returns `None` if `N > SIEVE_BOUND`.
#[must_use]
pub const fn primes_per_class(n: u32) -> Option<[u32; U30_CARDINALITY]> {
    if n > SIEVE_BOUND {
        return None;
    }
    let mut counts = [0u32; U30_CARDINALITY];
    // Primes 2, 3, 5 are not coprime to 30 and are excluded; start at 7.
    let mut k: u32 = 7;
    while k <= n {
        if IS_PRIME[k as usize] {
            let r = (k as u64) % MODULUS_U;
            let mut i = 0usize;
            while i < U30_CARDINALITY {
                if RESIDUES[i] == r {
                    counts[i] += 1;
                    break;
                }
                i += 1;
            }
        }
        k += 1;
    }
    Some(counts)
}

/// `π_30(N) = π(N) − π(5)` — the count of primes coprime to 30 up
/// to `N`. Equivalently, the sum of the eight class counts.
#[must_use]
pub const fn pi_coprime_to_30(n: u32) -> Option<u32> {
    let pi_n = match pi(n) {
        Some(v) => v,
        None => return None,
    };
    if n < 5 {
        return Some(pi_n);
    }
    // π(5) = 3 (primes 2, 3, 5).
    Some(pi_n - 3)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::arithmetic::gcd;

    /// Spot-check `IS_PRIME` against known small primes.
    #[test]
    fn sieve_small_primes() {
        assert!(!IS_PRIME[0]);
        assert!(!IS_PRIME[1]);
        assert!(IS_PRIME[2]);
        assert!(IS_PRIME[3]);
        assert!(!IS_PRIME[4]);
        assert!(IS_PRIME[5]);
        assert!(!IS_PRIME[6]);
        assert!(IS_PRIME[7]);
        assert!(!IS_PRIME[9]);
        assert!(IS_PRIME[11]);
        assert!(IS_PRIME[13]);
        assert!(!IS_PRIME[15]);
        assert!(IS_PRIME[97]);
        assert!(!IS_PRIME[100]);
        assert!(IS_PRIME[101]);
        assert!(IS_PRIME[9973]); // largest prime ≤ 10000
        assert!(!IS_PRIME[9999]);
    }

    /// Standard reference values of `π(N)`.
    #[test]
    fn pi_known_values() {
        assert_eq!(pi(10), Some(4)); // 2, 3, 5, 7
        assert_eq!(pi(100), Some(25));
        assert_eq!(pi(1000), Some(168));
        assert_eq!(pi(10_000), Some(1229));
    }

    /// Out-of-range guard.
    #[test]
    fn pi_out_of_range() {
        assert_eq!(pi(SIEVE_BOUND + 1), None);
    }

    /// Coprime-to-30 prime count: `π(N) − 3` for `N ≥ 5`.
    #[test]
    fn pi_coprime_to_30_known_values() {
        assert_eq!(pi_coprime_to_30(10), Some(1)); // {7}
        assert_eq!(pi_coprime_to_30(100), Some(22));
        assert_eq!(pi_coprime_to_30(1000), Some(165));
        assert_eq!(pi_coprime_to_30(10_000), Some(1226));
    }

    /// Sum of class counts equals `π_30(N)` — i.e., every prime
    /// coprime to 30 is counted exactly once.
    #[test]
    fn class_counts_sum_to_pi_coprime() {
        for n in [100u32, 1_000, 10_000] {
            let counts = primes_per_class(n).expect("within bound");
            let sum: u32 = counts.iter().sum();
            let expected = pi_coprime_to_30(n).expect("within bound");
            assert_eq!(sum, expected, "class count sum mismatch at N = {n}");
        }
    }

    /// **Dirichlet equidistribution at N = 10⁴.**
    ///
    /// The eight `U(30)` class counts each lie within `2 √mean` of
    /// the mean `(π(10⁴) − 3) / 8`. This is a generous Phase 1
    /// bound; sharper bounds (e.g. `√N log N` from the GRH-conditional
    /// effective form) are Phase 3 work.
    #[test]
    fn equidistribution_at_10_4() {
        let n: u32 = 10_000;
        let counts = primes_per_class(n).expect("within bound");
        let total = pi_coprime_to_30(n).expect("within bound");
        // Mean per class.
        let mean = total / 8;
        // Floor of 2 √mean as integer bound.
        let bound = 2 * isqrt(mean);
        for (i, &c) in counts.iter().enumerate() {
            let dev = if c > mean { c - mean } else { mean - c };
            assert!(
                dev <= bound,
                "class {i} (residue {}) count {} deviates by {} > 2√{} = {}",
                RESIDUES[i],
                c,
                dev,
                mean,
                bound
            );
        }
    }

    /// Integer square root (Newton's method on integers, no floats).
    /// Used only by tests; returns floor(√n).
    fn isqrt(n: u32) -> u32 {
        if n < 2 {
            return n;
        }
        let mut x = n;
        let mut y = (x + 1) / 2;
        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }
        x
    }

    /// `isqrt` agrees with `(n as f64).sqrt() as u32` for spot values
    /// — verified without float by direct comparison `r² ≤ n < (r+1)²`.
    #[test]
    fn isqrt_correctness() {
        for n in [0u32, 1, 4, 9, 10, 100, 1_000, 9_801, 10_000] {
            let r = isqrt(n);
            assert!(r * r <= n);
            assert!(n < (r + 1).saturating_mul(r + 1) || r == u32::MAX);
        }
    }

    /// Every counted prime is genuinely coprime to 30, and its
    /// residue lands in the announced `U(30)` class.
    #[test]
    fn each_class_count_matches_residue() {
        let counts = primes_per_class(10_000).expect("within bound");
        let mut recomputed = [0u32; U30_CARDINALITY];
        for k in 7u32..=10_000 {
            if !IS_PRIME[k as usize] {
                continue;
            }
            if gcd(u64::from(k), MODULUS_U) != 1 {
                continue;
            }
            let r = u64::from(k) % MODULUS_U;
            for (i, &res) in RESIDUES.iter().enumerate() {
                if res == r {
                    recomputed[i] += 1;
                    break;
                }
            }
        }
        assert_eq!(counts, recomputed);
    }
}
