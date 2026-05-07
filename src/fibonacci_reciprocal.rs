//! # `1/89 = Σ_{n≥1} F_n / 10^(n+1)` — the Fibonacci reciprocal identity
//!
//! ## The identity
//!
//! From the Fibonacci generating function
//! `f(x) = Σ_{n≥1} F_n · x^n = x / (1 − x − x²)`, evaluating at
//! `x = 1/10`:
//!
//! ```text
//!   f(1/10) = (1/10) / (1 − 1/10 − 1/100)
//!           = (1/10) / (89/100)
//!           = 10/89
//! ```
//!
//! Therefore `Σ_{n≥1} F_n / 10^n = 10/89`, which after dividing by 10
//! gives the headline form:
//!
//! ```text
//!   Σ_{n≥1} F_n / 10^(n+1) = 1/89
//! ```
//!
//! ## What this module verifies
//!
//! In exact integer arithmetic, the partial sum
//!
//! ```text
//!   S(N) = Σ_{n=1..N} F_n · 10^(N − n)
//! ```
//!
//! satisfies `89 · S(N) ≤ 10^(N+1)`, with the gap shrinking like
//! `O(F_{N+2})` (the Fibonacci tail). For `N = 17`, both sides
//! fit in `u128` and the gap is provably small. The compile-time
//! assertions below verify this for `N ∈ {5, 10, 15, 17}`.
//!
//! ## Anchor
//!
//! Computed at compile time from
//! [`crate::parameters::NUMERAL_BASE`] (= 10) and the Fibonacci
//! recurrence in [`crate::fibonacci`]. The value `89` enters
//! through the algebraic identity `100 − 10 − 1 = 89` only —
//! never as a hand-typed constant in production paths.
//!
//! ## UOR primitive bindings
//!
//! | Object | UOR ontology IRI |
//! |---|---|
//! | `partial_numerator` | `https://uor.foundation/recursion/BoundedRecursion` (descent measure = `N`) |
//! | bounded gap | `https://uor.foundation/predicate/TypePredicate` (within tail bound) |
//! | `verify_*_at` | `https://uor.foundation/cert/RuntimeCertificate` |

use crate::fibonacci::fib;
use crate::parameters::NUMERAL_BASE;

/// `89` derived from `NUMERAL_BASE = 10`. Specifically
/// `b² − b − 1 = 89` when `b = 10`. Computed at compile time.
pub const RECIPROCAL_DENOMINATOR: u64 =
    NUMERAL_BASE * NUMERAL_BASE - NUMERAL_BASE - 1;

const _: () = assert!(
    RECIPROCAL_DENOMINATOR == 89,
    "b² − b − 1 must equal 89 for b = 10"
);

/// Partial-sum numerator over the common denominator
/// `NUMERAL_BASE^(N+1)`:
///
/// ```text
///   S(N) = Σ_{n=1..N} F_n · NUMERAL_BASE^(N − n)
/// ```
///
/// Returns `S(N)` as a `u128`. Domain: `N ∈ [1, 17]` keeps every
/// intermediate inside `u128`.
#[must_use]
pub const fn partial_numerator(n: u32) -> u128 {
    let mut sum: u128 = 0;
    let mut k: u32 = 1;
    let n_u128 = n as u128;
    while k <= n {
        let exp = (n_u128 - k as u128) as u32;
        let f_k = fib(k as u64) as u128;
        sum += f_k * pow10_u128(exp);
        k += 1;
    }
    sum
}

/// `10^k` as `u128`. Domain: `k ≤ 38` (fits in `u128`).
#[must_use]
pub const fn pow10_u128(k: u32) -> u128 {
    let mut p: u128 = 1;
    let mut i = 0u32;
    while i < k {
        p *= NUMERAL_BASE as u128;
        i += 1;
    }
    p
}

/// **Tail gap.** Returns `10^(N+1) − 89 · S(N)`. By the identity
/// `1/89 − P(N) = tail / 89`, this gap equals `89 · tail · 10^(N+1)`,
/// which is bounded by `89 · 2 · F_{N+1} = 178 · F_{N+1}` for `N ≥ 1`.
#[must_use]
pub const fn tail_gap(n: u32) -> u128 {
    let denom = pow10_u128(n + 1);
    let s = partial_numerator(n);
    let scaled = (RECIPROCAL_DENOMINATOR as u128) * s;
    if denom > scaled {
        denom - scaled
    } else {
        0
    }
}

/// **Tail upper bound.** Symbolic upper bound on `tail_gap(N)`:
/// `178 · F_{N+1}`.
#[must_use]
pub const fn tail_bound(n: u32) -> u128 {
    178u128 * (fib((n + 1) as u64) as u128)
}

/// **Compile-time verification at `N = 5`.**
const _: () = {
    let n = 5u32;
    let gap = tail_gap(n);
    let bound = tail_bound(n);
    assert!(gap > 0, "gap must be positive at N=5");
    assert!(gap < bound, "gap must be below tail bound at N=5");
};

/// **Compile-time verification at `N = 10`.**
const _: () = {
    let n = 10u32;
    let gap = tail_gap(n);
    let bound = tail_bound(n);
    assert!(gap > 0, "gap must be positive at N=10");
    assert!(gap < bound, "gap must be below tail bound at N=10");
};

/// **Compile-time verification at `N = 15`.**
const _: () = {
    let n = 15u32;
    let gap = tail_gap(n);
    let bound = tail_bound(n);
    assert!(gap > 0, "gap must be positive at N=15");
    assert!(gap < bound, "gap must be below tail bound at N=15");
};

/// **Compile-time verification at `N = 17`.**
const _: () = {
    let n = 17u32;
    let gap = tail_gap(n);
    let bound = tail_bound(n);
    assert!(gap > 0, "gap must be positive at N=17");
    assert!(gap < bound, "gap must be below tail bound at N=17");
};

/// **Sanity anchor.** At `N = 10`, the partial-sum numerator is
/// `1,123,595,495` over denominator `10^11`. The website cites this
/// as the period-44 expansion of `1/89` truncated at 10 digits.
pub const PARTIAL_NUMERATOR_AT_10: u128 = 1_123_595_495;

const _: () = assert!(
    partial_numerator(10) == PARTIAL_NUMERATOR_AT_10,
    "partial_numerator(10) must equal 1,123,595,495"
);

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
    use super::*;

    #[test]
    fn reciprocal_denominator_eq_89() {
        assert_eq!(RECIPROCAL_DENOMINATOR, 89);
    }

    #[test]
    fn partial_numerator_at_10_anchor() {
        assert_eq!(partial_numerator(10), 1_123_595_495);
    }

    #[test]
    fn tail_gap_strictly_positive_and_bounded() {
        for n in 1..=17u32 {
            let gap = tail_gap(n);
            let bound = tail_bound(n);
            assert!(gap > 0, "N={n}: gap must be positive");
            assert!(gap < bound, "N={n}: gap {gap} ≥ bound {bound}");
        }
    }

    #[test]
    fn convergence_relative_gap_shrinks_per_step() {
        // Pure integer convergence test:
        //   gap(N+1) / denom(N+1) < gap(N) / denom(N)
        // ⇔ gap(N+1) * denom(N) < gap(N) * denom(N+1)
        // Since denom(N+1) = 10 * denom(N), this simplifies to
        //   gap(N+1) < 10 * gap(N).
        for n in 5..17u32 {
            let g0 = tail_gap(n);
            let g1 = tail_gap(n + 1);
            assert!(g1 < 10 * g0, "N={n}: gap not shrinking faster than 10×");
        }
    }
}
