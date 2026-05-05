//! # Digital root function
//!
//! The digital root `dr : ℤ⁺ → {1, …, 9}` is defined here as the unique
//! integer in `{1, …, MODULUS_DR}` congruent to its argument modulo
//! `MODULUS_DR = NUMERAL_BASE − 1 = 9`.
//!
//! ## Closed-form definition
//!
//! `dr(n) = 1 + ((n − 1) mod MODULUS_DR)` for `n ≥ 1`.
//!
//! ## Properties (proved in tests)
//!
//! - **Range.** `dr(n) ∈ {1, …, 9}` for all `n ≥ 1`.
//! - **Congruence.** `dr(n) ≡ n (mod 9)`.
//! - **Periodicity.** `dr(n + 9) = dr(n)` for all `n ≥ 1`.
//! - **Equivalence to iterated digit sum.** Repeatedly summing the
//!   decimal digits of `n` reaches a fixed point equal to `dr(n)`.
//!
//! ## Edge case
//!
//! For `n = 0`, `dr(0)` is mathematically undefined / conventionally
//! `0`. This implementation returns `0` for `n = 0`. All callers in
//! this crate apply `dr` to `n ≥ 1`.

use crate::parameters::{MODULUS_DR, NUMERAL_BASE};

/// Digital root of `n`.
///
/// Domain: `n ∈ ℕ`. Range: `{0}` for `n = 0`, `{1, …, 9}` for `n ≥ 1`.
///
/// Closed-form definition: for `n ≥ 1`, `dr(n) = 1 + ((n − 1) mod 9)`.
#[must_use]
pub const fn dr(n: u64) -> u8 {
    if n == 0 {
        return 0;
    }
    // (n - 1) mod (MODULUS_DR) yields 0..=MODULUS_DR-1; +1 gives 1..=MODULUS_DR.
    #[allow(clippy::cast_possible_truncation)]
    {
        (1 + (n - 1) % MODULUS_DR) as u8
    }
}

/// Iterated digit sum of `n` in base [`NUMERAL_BASE`], for cross-checking.
///
/// Repeatedly sums the digits of `n` in base 10 until the result is a
/// single digit. This is the *original* schoolbook definition of the
/// digital root; comparing it to [`dr`] verifies the closed-form.
#[must_use]
pub const fn iterated_digit_sum(n: u64) -> u8 {
    let mut x = n;
    while x >= NUMERAL_BASE {
        let mut acc = 0u64;
        let mut y = x;
        while y > 0 {
            acc += y % NUMERAL_BASE;
            y /= NUMERAL_BASE;
        }
        x = acc;
    }
    #[allow(clippy::cast_possible_truncation)]
    {
        x as u8
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    /// `dr(n) ∈ {1, …, 9}` for `n ∈ [1, 10000]`.
    #[test]
    fn dr_range_is_one_through_nine() {
        for n in 1..=10_000u64 {
            let r = dr(n);
            assert!(
                (1..=9).contains(&r),
                "dr({n}) = {r} is outside the valid range [1, 9]"
            );
        }
    }

    /// `dr(n) ≡ n (mod 9)` for `n ∈ [1, 10000]`.
    #[test]
    fn dr_congruent_to_n_mod_nine() {
        for n in 1..=10_000u64 {
            let r = dr(n);
            let expected = if n % 9 == 0 { 9 } else { (n % 9) as u8 };
            assert_eq!(r, expected, "dr({n}) ≢ n (mod 9)");
        }
    }

    /// `dr(n + 9) = dr(n)` for `n ∈ [1, 10000]`.
    #[test]
    fn dr_periodic_with_period_nine() {
        for n in 1..=10_000u64 {
            assert_eq!(dr(n), dr(n + 9), "dr is not 9-periodic at n = {n}");
        }
    }

    /// The closed-form `dr` agrees with the iterated digit sum.
    #[test]
    fn dr_equals_iterated_digit_sum() {
        for n in 1..=10_000u64 {
            assert_eq!(
                dr(n),
                iterated_digit_sum(n),
                "dr({n}) disagrees with iterated digit sum"
            );
        }
    }

    /// `dr(0) = 0` (edge case convention).
    #[test]
    fn dr_zero_is_zero() {
        assert_eq!(dr(0), 0);
    }
}
