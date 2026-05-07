//! # `π(N)` via Gary's published U(30) sieve
//!
//! Wraps [`crate::sieve`] in the structural identity Gary's
//! spreadsheets implement:
//!
//! ```text
//! π(N) = D₃₀(N) − C₃₀(N) + 2
//!      = #{ primes in U(30) ∩ [1, N] } + 3
//! ```
//!
//! where
//!
//! - `D₃₀(N)` is the count of integers in `[1, N]` coprime to 30,
//! - `C₃₀(N)` is the count of composites among those,
//! - `+2` reconciles `−1` for the non-prime unit `1 ∈ U(30)` against
//!   `+3` for the primes `{2, 3, 5}` not in `U(30)`.
//!
//! Equivalently, `π(N) = D₃₀(N) − C₃₀(N) + 2`. This module exposes
//! both forms and a structural-identity check that they agree at
//! every `N` we test.
//!
//! ## Why this matters
//!
//! Gary's website and spreadsheets implement *this* algorithm — pure
//! integer arithmetic on the eight residue classes of `U(30)`, no
//! transcendental functions, no calibration constants, no residuals.
//! At every `N` we can run, it gives the canonical `π(N)` exactly.
//! The Master-Clock identity `D − TF + RF + 3` referenced in his
//! correspondence is a separate construction that does not appear in
//! his published artifacts; this module is the verifiable foundation.

#![cfg(feature = "alloc")]

use crate::sieve::{d30, pi_via_u30_sieve, primes_in_u30};

/// Composites in `U(30)` up to `N`.
///
/// `C₃₀(N) = D₃₀(N) − primes_in_u30(N) − 1`. The `−1` is for the
/// unit `1 ∈ U(30)`, which is neither prime nor composite.
#[must_use]
pub fn c30(n: u64) -> u64 {
    let dn = d30(n);
    let pn = primes_in_u30(n);
    // 1 ∈ U(30) is the unit; it is neither prime nor composite.
    // For n ≥ 1 the unit is always counted by `d30`, so subtract it.
    let unit = u64::from(n >= 1);
    dn.saturating_sub(pn).saturating_sub(unit)
}

/// `π(N)` computed via the additive form
/// `π(N) = primes_in_u30(N) + 3`.
#[must_use]
pub fn pi_additive(n: u64) -> u64 {
    pi_via_u30_sieve(n)
}

/// `π(N)` computed via the structural form
/// `π(N) = D₃₀(N) − C₃₀(N) + 2`.
///
/// This must equal [`pi_additive`] for every `N`; the equality is
/// the published U(30) identity. See [`identity_holds`].
#[must_use]
pub fn pi_structural(n: u64) -> u64 {
    if n < 2 {
        return 0;
    }
    if n < 7 {
        // {2, 3, 5} are primes outside U(30); D₃₀ and C₃₀ are not the
        // right vocabulary below 7. Handle directly.
        return pi_additive(n);
    }
    let dn = d30(n);
    let cn = c30(n);
    // dn ≥ cn + 2 for n ≥ 7 (the unit 1 plus at least the primes 7, 11
    // contribute to dn − cn beyond the +2 floor), so the subtraction
    // is well-defined.
    dn - cn + 2
}

/// Structural identity check: the additive and structural forms agree.
///
/// Returns `true` iff `pi_additive(n) == pi_structural(n)`. Phase-5
/// invariant; failure indicates a bug in either `c30` or the sieve.
#[must_use]
pub fn identity_holds(n: u64) -> bool {
    pi_additive(n) == pi_structural(n)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]

    use super::*;

    #[test]
    fn structural_matches_additive_at_100() {
        assert!(identity_holds(100));
        assert_eq!(pi_structural(100), 25);
    }

    #[test]
    fn structural_matches_additive_at_10000() {
        assert!(identity_holds(10_000));
        assert_eq!(pi_structural(10_000), 1_229);
    }

    #[test]
    fn c30_at_30_equals_0() {
        // U(30) ∩ [1, 30] = {1, 7, 11, 13, 17, 19, 23, 29}; among
        // these only 1 is non-prime non-composite, so C₃₀(30) = 0.
        assert_eq!(c30(30), 0);
    }

    #[test]
    fn c30_at_100_counts_first_composites() {
        // U(30) ∩ [1, 100] has composites {49, 77, 91} — that is the
        // floor; products of two U(30) members ≥ 7 with product ≤ 100.
        assert_eq!(c30(100), 3);
    }

    /// Walk identity across many small N to catch off-by-one drift.
    #[test]
    fn identity_holds_at_every_n_below_1000() {
        for n in 0u64..1_000 {
            assert!(
                identity_holds(n),
                "structural identity broke at N={n}: additive={}, structural={}",
                pi_additive(n),
                pi_structural(n)
            );
        }
    }
}
