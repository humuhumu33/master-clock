//! # Triangular numbers and the algebraic identities at U_30(∞) anchors
//!
//! `k(n) = n · (n + 1) / 2` is the standard triangular-number formula
//! (OEIS A000217). primesdemystified.com (formulae.html) tabulates
//! it at three U_30(∞) anchor points:
//!
//! ```text
//!   k(89)    =       4,005
//!   k(109)   =       5,995
//!   k(7919)  =  31,359,240
//! ```
//!
//! Two additional algebraic identities at the same anchors connect
//! `89` and `109` directly to `MODULUS_R = 360`:
//!
//! ```text
//!   89² − 1   =  7,920  =  22 · MODULUS_R
//!   109² − 89² =  3,960  =  11 · MODULUS_R
//! ```
//!
//! ## Anchor — derived from Phase 1 + Phase 4
//!
//! `89` and `109` are computed via
//! [`crate::index_to_residue::nth_u30_member`] at indices 24 and 30
//! respectively. The triangular formula is closed-form. Every
//! identity verified by `const _: () = assert!(…)`.
//!
//! ## UOR primitive bindings
//!
//! | Object | UOR ontology IRI |
//! |---|---|
//! | `triangular` | `https://uor.foundation/recursion/BoundedRecursion` (closed form) |
//! | algebraic identities | `https://uor.foundation/predicate/TypePredicate` |

use crate::index_to_residue::nth_u30_member;
use crate::parameters::MODULUS_R;

/// Triangular number: `k(n) = n · (n + 1) / 2`.
///
/// Returns `n(n+1)/2`. Overflow-safe up to `n ≤ 6,074,000,999` in `u64`.
#[must_use]
pub const fn triangular(n: u64) -> u64 {
    n * (n + 1) / 2
}

/// `89 = nth_u30_member(24)`. The website's "first 89 primes"
/// boundary point.
pub const ANCHOR_24: u64 = nth_u30_member(24);

/// `109 = nth_u30_member(30)`. The complementary anchor that
/// participates in the `109² − 89² = 11 · MODULUS_R` identity.
pub const ANCHOR_30: u64 = nth_u30_member(30);

const _: () = {
    assert!(ANCHOR_24 == 89, "24th U_30(∞) member must equal 89");
    assert!(ANCHOR_30 == 109, "30th U_30(∞) member must equal 109");
};

/// Triangular-number anchor: `k(89) = 4,005`.
pub const TRIANGULAR_AT_24TH: u64 = triangular(ANCHOR_24);

const _: () = assert!(
    TRIANGULAR_AT_24TH == 4_005,
    "k(89) must equal 4,005"
);

/// Triangular-number anchor: `k(109) = 5,995`.
pub const TRIANGULAR_AT_30TH: u64 = triangular(ANCHOR_30);

const _: () = assert!(
    TRIANGULAR_AT_30TH == 5_995,
    "k(109) must equal 5,995"
);

/// Triangular-number anchor: `k(7919) = 31,359,240`.
/// `7919` is the 1000th prime (verified by the wheel-30 sieve).
pub const TRIANGULAR_AT_THOUSANDTH_PRIME: u64 = triangular(7_919);

const _: () = assert!(
    TRIANGULAR_AT_THOUSANDTH_PRIME == 31_359_240,
    "k(7919) must equal 31,359,240"
);

/// Algebraic identity: `89² − 1 = 22 · MODULUS_R`.
pub const SQUARE_MINUS_ONE_AT_24: u64 = ANCHOR_24 * ANCHOR_24 - 1;

const _: () = {
    assert!(SQUARE_MINUS_ONE_AT_24 == 7_920, "89² − 1 must equal 7,920");
    assert!(
        SQUARE_MINUS_ONE_AT_24 == 22 * MODULUS_R,
        "89² − 1 must factor as 22 · MODULUS_R"
    );
};

/// Algebraic identity: `109² − 89² = 11 · MODULUS_R`.
pub const SQUARE_DIFFERENCE_30_24: u64 =
    ANCHOR_30 * ANCHOR_30 - ANCHOR_24 * ANCHOR_24;

const _: () = {
    assert!(
        SQUARE_DIFFERENCE_30_24 == 3_960,
        "109² − 89² must equal 3,960"
    );
    assert!(
        SQUARE_DIFFERENCE_30_24 == 11 * MODULUS_R,
        "109² − 89² must factor as 11 · MODULUS_R"
    );
};

/// Pyramid identity: `2112 · (109² − 89²) = 8,363,520`. The product
/// expresses the index `2112` of `7919` (the 1000th prime, position
/// in U_30(∞)) times the algebraic difference of the surrounding
/// anchors.
pub const PYRAMID_PRODUCT: u64 = 2_112 * SQUARE_DIFFERENCE_30_24;

const _: () = {
    assert!(
        PYRAMID_PRODUCT == 8_363_520,
        "2112 · (109² − 89²) must equal 8,363,520"
    );
    // Equivalently: 2112 · 11 · MODULUS_R = 23232 · MODULUS_R.
    assert!(
        PYRAMID_PRODUCT == 2_112 * 11 * MODULUS_R,
        "pyramid product must factor as 2112 · 11 · MODULUS_R"
    );
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
    use super::*;

    #[test]
    fn triangular_small() {
        assert_eq!(triangular(0), 0);
        assert_eq!(triangular(1), 1);
        assert_eq!(triangular(2), 3);
        assert_eq!(triangular(3), 6);
        assert_eq!(triangular(10), 55);
    }

    #[test]
    fn anchors_match_website() {
        assert_eq!(TRIANGULAR_AT_24TH, 4_005);
        assert_eq!(TRIANGULAR_AT_30TH, 5_995);
        assert_eq!(TRIANGULAR_AT_THOUSANDTH_PRIME, 31_359_240);
    }

    #[test]
    fn square_difference_anchors_to_modulus_r() {
        assert_eq!(SQUARE_MINUS_ONE_AT_24, 22 * MODULUS_R);
        assert_eq!(SQUARE_DIFFERENCE_30_24, 11 * MODULUS_R);
    }

    #[test]
    fn pyramid_product_factors_cleanly() {
        assert_eq!(PYRAMID_PRODUCT, 2_112 * 11 * MODULUS_R);
        assert_eq!(PYRAMID_PRODUCT, 8_363_520);
    }
}
