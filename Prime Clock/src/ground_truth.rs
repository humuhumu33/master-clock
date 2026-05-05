//! # Ground-truth π(N) and per-class prime counts at large N
//!
//! Behind the `ground-truth` feature flag because it depends on the
//! `primal` crate (allocator-using) and runs slow tests at N = 10⁹.
//! The Phase 1 compile-time sieve in [`crate::dirichlet`] caps at
//! `SIEVE_BOUND = 10_000`; this module extends the same evidence to
//! N = 10⁶ and N = 10⁹ for sharper Dirichlet-equidistribution
//! verification.
//!
//! ## Phase 2 O4 scope
//!
//! - **N = 10⁶** — fast (~tens of milliseconds), runs by default.
//!   Mean per U(30) class: `(π(10⁶) − 3) / 8 ≈ 9,812`.
//! - **N = 10⁹** — slow (~30–60 seconds with primal). Marked
//!   `#[ignore]`. Mean per class: `(π(10⁹) − 3) / 8 ≈ 6,355,941`.
//! - **N = 10¹²** — out of scope for this module. Primal would
//!   need ~125 GB to allocate the sieve. The C-library
//!   `primesieve` can do this in seconds; future work.
//!
//! ## Builds on Phase 1
//!
//! - [`crate::parameters::MODULUS_U`] — the residue modulus 30.
//! - [`crate::parameters::U30_CARDINALITY`] — 8 residue classes.
//! - [`crate::u30::RESIDUES`] — the 8 elements of `U(30)`.
//! - [`crate::dirichlet`] — the compile-time sieve at `N ≤ 10⁴`,
//!   mirrored at the larger scales here.
//!
//! ## Discipline
//!
//! - No floating point — bound `2 √mean` computed via integer
//!   square root.
//! - The integer-square-root helper [`isqrt_u64`] is the same form
//!   as `crate::dirichlet::tests::isqrt`, just over `u64`.

#![cfg(feature = "ground-truth")]

extern crate alloc;

use crate::parameters::{MODULUS_U, U30_CARDINALITY};
use crate::u30::RESIDUES;

/// `π(N)` — count of primes ≤ N. Uses primal's `Sieve::prime_pi`.
///
/// Allocates an `O(N / 8)`-byte sieve. For `N = 10⁹` this is
/// ~125 MB; doable on a typical workstation. For `N = 10¹²` this
/// would be ~125 GB and is out of scope for this module.
#[must_use]
pub fn pi_n(n: u64) -> u64 {
    #[allow(clippy::cast_possible_truncation)]
    let limit = n as usize;
    let sieve = primal::Sieve::new(limit);
    sieve.prime_pi(limit) as u64
}

/// Counts of primes `≤ N` in each `U(30)` residue class.
///
/// Returns `[c_0, c_1, …, c_7]` where `c_i` is the count of primes
/// `p ≤ N` with `p ≡ RESIDUES[i] (mod MODULUS_U)`.
///
/// Excludes the primes 2, 3, 5 (they are not coprime to 30 and so
/// fall outside `U(30)`).
#[must_use]
pub fn primes_per_class(n: u64) -> [u64; U30_CARDINALITY] {
    let mut counts = [0u64; U30_CARDINALITY];
    #[allow(clippy::cast_possible_truncation)]
    let limit = n as usize;
    let sieve = primal::Sieve::new(limit);
    for p_usize in sieve.primes_from(7) {
        let p = p_usize as u64;
        if p > n {
            break;
        }
        let r = p % MODULUS_U;
        for (i, &res) in RESIDUES.iter().enumerate() {
            if res == r {
                counts[i] += 1;
                break;
            }
        }
    }
    counts
}

/// `π_30(N) = π(N) − π(5)` — the count of primes coprime to 30 up
/// to `N`. Equivalently, the sum of the eight class counts.
#[must_use]
pub fn pi_coprime_to_30(n: u64) -> u64 {
    let pi_n = pi_n(n);
    if n < 5 {
        pi_n
    } else {
        // π(5) = 3 (primes 2, 3, 5).
        pi_n - 3
    }
}

/// Integer square root of `n`, computed via Newton's method.
/// Returns `floor(√n)`. No floating point.
#[must_use]
pub fn isqrt_u64(n: u64) -> u64 {
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

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    /// Reference value: `π(10⁶) = 78,498`.
    #[test]
    fn pi_at_10_6() {
        assert_eq!(pi_n(1_000_000), 78_498);
    }

    /// Class counts at `N = 10⁶` sum to `π(10⁶) − 3 = 78,495`.
    #[test]
    fn class_counts_sum_at_10_6() {
        let counts = primes_per_class(1_000_000);
        let total: u64 = counts.iter().sum();
        assert_eq!(total, 78_495);
        assert_eq!(total, pi_coprime_to_30(1_000_000));
    }

    /// **Dirichlet equidistribution at N = 10⁶.** Each U(30) class
    /// count lies within `2 √mean` of the mean
    /// `(π(10⁶) − 3) / 8 = 9811.875`.
    ///
    /// The 10⁴ compile-time sieve in [`crate::dirichlet`] verifies
    /// the same claim with `mean ≈ 153`. At 10⁶ the mean grows to
    /// ~9,812 and the bound `2 √mean ≈ 198` becomes tighter
    /// relative to the mean (~2% vs ~16% at 10⁴).
    #[test]
    fn equidistribution_at_10_6() {
        let n = 1_000_000;
        let counts = primes_per_class(n);
        let total = pi_coprime_to_30(n);
        let mean = total / 8;
        let bound = 2 * isqrt_u64(mean);
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

    /// Reference value: `π(10⁹) = 50,847,534`.
    /// Slow: ~30 seconds with primal. Marked `#[ignore]`.
    #[test]
    #[ignore = "slow: ~30s with primal sieve at N = 10⁹"]
    fn pi_at_10_9() {
        assert_eq!(pi_n(1_000_000_000), 50_847_534);
    }

    /// **Dirichlet equidistribution at N = 10⁹.** Same form as the
    /// 10⁶ test, but with `mean ≈ 6,355,941` and `bound ≈ 5,042`
    /// (~0.08% of mean). The relative bound continues to tighten.
    ///
    /// Slow: ~30–60 seconds with primal. Marked `#[ignore]`; run
    /// manually via `cargo test --features ground-truth -- --ignored
    /// equidistribution_at_10_9`.
    #[test]
    #[ignore = "slow: ~30s with primal sieve at N = 10⁹"]
    fn equidistribution_at_10_9() {
        let n = 1_000_000_000;
        let counts = primes_per_class(n);
        let total = pi_coprime_to_30(n);
        assert_eq!(total, 50_847_531); // π(10⁹) - 3
        let mean = total / 8;
        let bound = 2 * isqrt_u64(mean);
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

    /// Cross-check: the Phase 1 compile-time sieve at `N = 10⁴`
    /// agrees with this module's primal-based sieve at the same N.
    #[test]
    fn agrees_with_phase_1_sieve_at_10_4() {
        let phase_1_pi = crate::dirichlet::pi(10_000).expect("within Phase 1 bound");
        let phase_2_pi = pi_n(10_000);
        assert_eq!(u64::from(phase_1_pi), phase_2_pi);

        let phase_1_classes =
            crate::dirichlet::primes_per_class(10_000).expect("within Phase 1 bound");
        let phase_2_classes = primes_per_class(10_000);
        for (i, (&p1, p2)) in phase_1_classes.iter().zip(phase_2_classes.iter()).enumerate() {
            assert_eq!(u64::from(p1), *p2, "class {i} disagreement");
        }
    }

    /// Sanity check on `isqrt_u64` for moderately large inputs.
    #[test]
    fn isqrt_correctness() {
        for n in [0u64, 1, 4, 9, 100, 1_000_000, 9_999_999, 10_000_000_000] {
            let r = isqrt_u64(n);
            assert!(r.saturating_mul(r) <= n);
            assert!(n < (r + 1).saturating_mul(r + 1) || r == u64::MAX);
        }
    }
}
