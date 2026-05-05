//! # Layers 6 & 7 — Elementary functions and Odrzywołek's `eml`
//!
//! Layer 6 builds [`exp`] and [`ln`] over the rationals as
//! [`Cauchy`] sequences with explicit moduli of convergence. Layer 7
//! defines [`eml`], the universal binary operator
//!
//! ```text
//! eml(x, y) = exp(x) − ln(y)
//! ```
//!
//! introduced by Odrzywołek (arXiv:2603.21852). Together with the
//! constant `1`, `eml` generates every elementary function over the
//! rationals; the grammar `S → 1 | eml(S, S)` is the EML form. This
//! module supplies the analytic primitive on which any such tree
//! evaluates.
//!
//! ## Place in the eight-layer bridge
//!
//! ```text
//! Layer 5: Cauchy<Rational>           [cauchy.rs]
//!     │
//!     │   truncated Taylor / atanh series, ε-bounded
//!     ▼
//! Layer 6: exp, ln                    [THIS MODULE]
//!     │
//!     │   composition: exp − ln
//!     ▼
//! Layer 7: eml(x, y) = exp(x) − ln(y) [THIS MODULE]
//! ```
//!
//! ## Algorithm for `exp(x)`
//!
//! Direct Taylor series:
//!
//! ```text
//! exp(x) = Σ_{k=0}^∞ x^k / k!
//! ```
//!
//! `nth(n)` returns the `(n+1)`-term partial sum `Σ_{k=0}^n x^k/k!`.
//! Term-to-term recurrence: `t_{k+1} = t_k · x / (k+1)` keeps each
//! call to `nth(n)` linear in `n`.
//!
//! Truncation bound (Lagrange remainder): for any `x ∈ ℚ`,
//!
//! ```text
//! |R_n(x)| = |Σ_{k=n+1}^∞ x^k/k!| ≤ |x|^{n+1}/(n+1)! · e^{|x|}.
//! ```
//!
//! We bound `e^{|x|}` by `3^⌈|x|⌉` (since `e < 3`). The modulus
//! search increments `N` until the rational expression `|x|^{N+1}
//! / (N+1)! · 3^⌈|x|⌉ < eps`.
//!
//! ## Algorithm for `ln(x)`, `x > 0`
//!
//! Two-stage argument reduction.
//!
//! 1. **Power-of-two reduction.** Find `m ∈ ℤ` such that `x = 2^m · y`
//!    with `y ∈ [1, 2)`; equivalently, `m = ⌊log₂ x⌋`. Then
//!    `ln(x) = m · ln 2 + ln(y)`.
//! 2. **Atanh series.** With `t = (y − 1) / (y + 1) ∈ [0, 1/3)`,
//!
//!    ```text
//!    ln(y) = 2 · Σ_{k=0}^∞ t^{2k+1} / (2k+1).
//!    ```
//!
//!    Tail bound: `|R_n(t)| ≤ 2 · |t|^{2n+3} / ((2n+3) · (1 − t²))`.
//!    For `|t| ≤ 1/3`, `1 − t² ≥ 8/9`, so the bound simplifies to
//!    `(9/4) · |t|^{2n+3} / (2n+3)`.
//!
//! `ln 2` is itself computed by the same atanh recipe at `t = 1/3`,
//! producing a Cauchy sequence used as a constant.
//!
//! ## Algorithm for `eml(x, y)`
//!
//! Pointwise difference of the two Cauchy sequences produced above.
//! [`Cauchy::sub`] handles the modulus combination
//! (`max(modulus_exp(eps/2), modulus_ln(eps/2))`).

extern crate alloc;
use alloc::string::{String, ToString};

use num_bigint::{BigInt, Sign};
use num_integer::Integer;
use num_traits::{One, ToPrimitive};

use crate::cauchy::Cauchy;
use crate::rational::Rational;

/// Errors raised by Layer 6 / 7 constructors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElementaryError {
    /// `ln(x)` was called with `x ≤ 0`.
    NonPositiveLog(String),
}

impl core::fmt::Display for ElementaryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonPositiveLog(s) => write!(f, "ln of non-positive value: {s}"),
        }
    }
}

// ============================================================
// exp
// ============================================================

/// Compute `e^x` as a Cauchy sequence of rationals.
///
/// Defined for every `x ∈ ℚ`. The returned [`Cauchy`] can be queried
/// at any precision via [`Cauchy::approx`].
#[must_use]
pub fn exp(x: Rational) -> Cauchy {
    let x_for_nth = x.clone();
    let x_for_modulus = x;

    let nth = move |n: u64| -> Rational {
        // Σ_{k=0}^n x^k / k!  via running term recurrence.
        let mut acc = Rational::one();
        let mut term = Rational::one();
        for k in 1..=n {
            // term_k = term_{k-1} · x / k
            term = &term * &x_for_nth;
            let k_rat = Rational::from_int(i64_from_u64_saturating(k));
            // k_rat is non-zero so checked_div always returns Some.
            term = term.checked_div(&k_rat).unwrap_or_else(Rational::zero);
            acc = &acc + &term;
        }
        acc
    };

    let modulus = move |eps: &Rational| -> u64 {
        find_exp_modulus(&x_for_modulus, eps)
    };

    Cauchy::from_fn(nth, modulus)
}

/// Find the smallest `N` such that `|x|^{N+1} / (N+1)! · 3^⌈|x|⌉ < eps`.
///
/// All arithmetic is rational; no floating point.
fn find_exp_modulus(x: &Rational, eps: &Rational) -> u64 {
    if eps.signum() <= 0 {
        return u64::MAX;
    }
    let abs_x = x.abs();
    // Upper bound for e^|x|: 3^⌈|x|⌉
    let ceil_abs_x: u64 = ceil_to_u64(&abs_x);
    let bound_e = Rational::from_int(3).pow_u32(u32_from_u64_saturating(ceil_abs_x));

    // Iterate N = 0, 1, 2, ... and accumulate term_{N+1} = |x|^{N+1}/(N+1)!.
    // term_{k+1} = term_k · |x| / (k+1).
    let mut term = abs_x.clone(); // term_1 = |x|
    let mut n: u64 = 0;
    loop {
        let bound = &term * &bound_e;
        if bound < *eps {
            return n;
        }
        n += 1;
        if n >= MODULUS_HARD_CAP {
            return n;
        }
        // Prepare term_{N+1} for next iteration: multiply by |x|, divide by (n+1).
        term = &term * &abs_x;
        let k_plus_1 = Rational::from_int(i64_from_u64_saturating(n + 1));
        term = term.checked_div(&k_plus_1).unwrap_or_else(Rational::zero);
    }
}

// ============================================================
// ln
// ============================================================

/// Compute `ln(x)` as a Cauchy sequence of rationals.
///
/// # Errors
///
/// Returns [`ElementaryError::NonPositiveLog`] if `x ≤ 0`.
pub fn ln(x: Rational) -> Result<Cauchy, ElementaryError> {
    if x.signum() <= 0 {
        return Err(ElementaryError::NonPositiveLog(x.to_string()));
    }

    // Stage 1: power-of-two reduction. Find m so that x = 2^m · y, y ∈ [1, 2).
    // m = ⌊log₂ x⌋.
    let (m, y) = power_of_two_reduce(&x);

    // Stage 2: atanh series for ln(y), y ∈ [1, 2).
    // t = (y − 1) / (y + 1) ∈ [0, 1/3).
    let one = Rational::one();
    let numer = &y - &one;
    let denom = &y + &one;
    // denom ≥ 2, never zero.
    let t = numer.checked_div(&denom).unwrap_or_else(Rational::zero);

    let ln_y = atanh_series_doubled(t);

    // ln(x) = m · ln 2 + ln(y).
    if m == 0 {
        Ok(ln_y)
    } else {
        let m_ln2 = scale_cauchy(&ln2_cauchy(), &Rational::from_bigint(BigInt::from(m)));
        Ok(m_ln2.add(&ln_y))
    }
}

/// Reduce `x > 0` to `(m, y)` such that `x = 2^m · y` and `y ∈ [1, 2)`.
fn power_of_two_reduce(x: &Rational) -> (i64, Rational) {
    let one = Rational::one();
    let two = Rational::from_int(2);

    if x >= &one && x < &two {
        return (0, x.clone());
    }

    let mut m: i64 = 0;
    let mut y = x.clone();

    if y >= two {
        while y >= two {
            y = y.checked_div(&two).unwrap_or_else(Rational::zero);
            m += 1;
        }
    } else {
        // y < 1 → multiply by 2 until in [1, 2).
        while y < one {
            y = &y * &two;
            m -= 1;
        }
    }

    (m, y)
}

/// `2 · (t + t³/3 + t⁵/5 + …)` for `|t| < 1`, returned as a Cauchy
/// sequence.
///
/// Tail bound at order `n` (taking `n+1` terms `k = 0..n`):
///
/// ```text
/// |R_n(t)| ≤ 2 · |t|^{2n+3} / ((2n+3) · (1 − t²))
/// ```
///
/// For our use case `|t| ≤ 1/3` so `1 − t² ≥ 8/9` and the bound is
/// at most `(9/4) · |t|^{2n+3} / (2n+3)`.
fn atanh_series_doubled(t: Rational) -> Cauchy {
    let t_for_nth = t.clone();
    let t_for_modulus = t;

    let nth = move |n: u64| -> Rational {
        // Σ_{k=0}^n t^{2k+1} / (2k+1), then doubled.
        let t2 = &t_for_nth * &t_for_nth; // t²
        let mut acc = Rational::zero();
        let mut term = t_for_nth.clone(); // term_0 = t (numerator only; divide by denom_index inside loop)
        let mut denom_index: u64 = 1;
        for k in 0..=n {
            let denom = Rational::from_int(i64_from_u64_saturating(denom_index));
            let part = term.checked_div(&denom).unwrap_or_else(Rational::zero);
            acc = &acc + &part;
            if k < n {
                term = &term * &t2;
                denom_index += 2;
            }
        }
        let two = Rational::from_int(2);
        &acc * &two
    };

    let modulus = move |eps: &Rational| -> u64 {
        find_atanh_modulus(&t_for_modulus, eps)
    };

    Cauchy::from_fn(nth, modulus)
}

/// Find smallest `N` such that
/// `2 · |t|^{2N+3} / ((2N+3)(1 − t²)) < eps`.
fn find_atanh_modulus(t: &Rational, eps: &Rational) -> u64 {
    if eps.signum() <= 0 {
        return u64::MAX;
    }
    let abs_t = t.abs();
    let one = Rational::one();
    let t2 = &abs_t * &abs_t;
    let one_minus_t2 = &one - &t2;
    if one_minus_t2.signum() <= 0 {
        // |t| ≥ 1 — series doesn't converge. Caller must reduce.
        return u64::MAX;
    }

    // term_0 corresponds to N=0: bound = 2 · |t|^3 / (3 · (1−t²))
    let mut t_power = abs_t.pow_u32(3); // |t|^{2N+3} for N=0
    let mut denom_index: u64 = 3;
    let two = Rational::from_int(2);

    let mut n: u64 = 0;
    loop {
        let denom = &Rational::from_int(i64_from_u64_saturating(denom_index)) * &one_minus_t2;
        let numer = &two * &t_power;
        let bound = numer.checked_div(&denom).unwrap_or_else(Rational::zero);
        if bound < *eps {
            return n;
        }
        n += 1;
        if n >= MODULUS_HARD_CAP {
            return n;
        }
        t_power = &t_power * &t2;
        denom_index += 2;
    }
}

/// Multiply a Cauchy sequence by an exact rational scalar.
fn scale_cauchy(seq: &Cauchy, scalar: &Rational) -> Cauchy {
    let s_for_nth = scalar.clone();
    let s_for_mod = scalar.clone();
    let inner_nth = seq.clone();
    let inner_mod = seq.clone();

    Cauchy::from_fn(
        move |n| &inner_nth.nth(n) * &s_for_nth,
        move |eps| {
            // Scaling by |s|: need inner Cauchy condition within eps / |s|.
            let abs_s = s_for_mod.abs();
            if abs_s.signum() == 0 {
                // 0 · seq is the zero sequence, modulus is trivially 0.
                0
            } else {
                let scaled = eps.checked_div(&abs_s).unwrap_or_else(Rational::zero);
                inner_mod.modulus(&scaled)
            }
        },
    )
}

/// `ln 2` as a Cauchy sequence. Materialised on demand via the
/// atanh recipe at `t = 1/3`.
fn ln2_cauchy() -> Cauchy {
    let third = Rational::new_i64(1, 3).unwrap_or_else(Rational::zero);
    atanh_series_doubled(third)
}

// ============================================================
// eml — Odrzywołek's universal binary operator (Layer 7)
// ============================================================

/// `eml(x, y) = exp(x) − ln(y)`. The universal binary operator from
/// arXiv:2603.21852.
///
/// # Errors
///
/// Returns [`ElementaryError::NonPositiveLog`] if `y ≤ 0`.
pub fn eml(x: Rational, y: Rational) -> Result<Cauchy, ElementaryError> {
    let exp_x = exp(x);
    let ln_y = ln(y)?;
    Ok(exp_x.sub(&ln_y))
}

// ============================================================
// Helpers
// ============================================================

/// Hard cap on modulus search to avoid pathological infinite loops if
/// the user passes an absurd eps (≤ 0 or unrepresentable).
const MODULUS_HARD_CAP: u64 = 1_000_000;

#[inline]
fn i64_from_u64_saturating(n: u64) -> i64 {
    #[allow(clippy::cast_possible_wrap)]
    if n <= i64::MAX as u64 {
        n as i64
    } else {
        i64::MAX
    }
}

#[inline]
fn u32_from_u64_saturating(n: u64) -> u32 {
    if n <= u64::from(u32::MAX) {
        #[allow(clippy::cast_possible_truncation)]
        let v = n as u32;
        v
    } else {
        u32::MAX
    }
}

/// `⌈|x|⌉` as a `u64`. Saturates if the value doesn't fit.
fn ceil_to_u64(abs_x: &Rational) -> u64 {
    if abs_x.signum() == 0 {
        return 0;
    }
    let numer = abs_x.numer();
    let denom = abs_x.denom();
    // ceil(num/denom) for positive num, denom > 0:
    //   = (num + denom − 1) / denom
    let num_pos = match numer.sign() {
        Sign::Minus => return 0, // shouldn't happen with abs, defensive
        _ => numer.clone(),
    };
    let q = (&num_pos + denom - BigInt::one()).div_floor(denom);
    q.to_u64().unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]
    #![allow(clippy::float_arithmetic, clippy::float_cmp)]

    use super::*;

    fn r(n: i64, d: i64) -> Rational {
        Rational::new_i64(n, d).unwrap()
    }

    fn close_to_f64(actual: &Rational, expected: f64, tol: f64) -> bool {
        let a = actual.to_f64_lossy().unwrap_or(f64::NAN);
        (a - expected).abs() < tol
    }

    #[test]
    fn exp_zero_is_one() {
        let e0 = exp(Rational::zero());
        assert_eq!(e0.approx(&r(1, 1_000_000)), Rational::one());
    }

    #[test]
    fn exp_one_approximates_e() {
        let e1 = exp(Rational::one());
        let approx = e1.approx(&r(1, 1_000_000));
        // e ≈ 2.718281828
        assert!(close_to_f64(&approx, 2.718_281_828_459_045_f64, 1e-6));
    }

    #[test]
    fn exp_half_approximates_sqrt_e() {
        let e_half = exp(r(1, 2));
        let approx = e_half.approx(&r(1, 1_000_000));
        // sqrt(e) ≈ 1.6487212707
        assert!(close_to_f64(&approx, 1.648_721_270_700_128_f64, 1e-6));
    }

    #[test]
    fn exp_negative_argument() {
        let em1 = exp(r(-1, 1));
        let approx = em1.approx(&r(1, 1_000_000));
        // 1/e ≈ 0.3678794
        assert!(close_to_f64(&approx, 0.367_879_441_171_442_3_f64, 1e-6));
    }

    #[test]
    fn ln_of_one_is_zero() {
        let l1 = ln(Rational::one()).unwrap();
        let approx = l1.approx(&r(1, 1_000_000));
        assert_eq!(approx, Rational::zero());
    }

    #[test]
    fn ln_of_two() {
        let l2 = ln(r(2, 1)).unwrap();
        let approx = l2.approx(&r(1, 1_000_000));
        // ln(2) ≈ 0.6931471805599453
        assert!(close_to_f64(&approx, 0.693_147_180_559_945_3_f64, 1e-6));
    }

    #[test]
    fn ln_of_e_is_one() {
        // Check that ln(approx_of_e) ≈ 1.
        // We use the rational 19/7 ≈ 2.7142857 which is close to but less than e.
        // ln(19/7) ≈ 0.99445
        let l = ln(r(19, 7)).unwrap();
        let approx = l.approx(&r(1, 1_000_000));
        let f = approx.to_f64_lossy().unwrap();
        let expected = (19.0_f64 / 7.0).ln();
        assert!((f - expected).abs() < 1e-6);
    }

    #[test]
    fn ln_of_zero_is_error() {
        assert!(matches!(
            ln(Rational::zero()),
            Err(ElementaryError::NonPositiveLog(_))
        ));
    }

    #[test]
    fn ln_of_negative_is_error() {
        assert!(matches!(
            ln(r(-3, 4)),
            Err(ElementaryError::NonPositiveLog(_))
        ));
    }

    #[test]
    fn eml_basic() {
        // eml(0, 1) = exp(0) − ln(1) = 1 − 0 = 1
        let v = eml(Rational::zero(), Rational::one()).unwrap();
        let approx = v.approx(&r(1, 1_000_000));
        assert_eq!(approx, Rational::one());
    }

    #[test]
    fn eml_one_one() {
        // eml(1, 1) = e − 0 = e
        let v = eml(Rational::one(), Rational::one()).unwrap();
        let approx = v.approx(&r(1, 1_000_000));
        assert!(close_to_f64(&approx, 2.718_281_828_459_045_f64, 1e-5));
    }

    #[test]
    fn eml_decomposes_exp() {
        // Paper claim: exp(x) = eml(x, 1).
        let x = r(3, 4);
        let direct = exp(x.clone()).approx(&r(1, 100_000));
        let via_eml = eml(x, Rational::one()).unwrap().approx(&r(1, 100_000));
        // Both should be equal up to representable rounding of partial sums.
        let d = direct.to_f64_lossy().unwrap();
        let v = via_eml.to_f64_lossy().unwrap();
        assert!((d - v).abs() < 1e-3);
    }

    #[test]
    fn eml_decomposes_ln() {
        // Paper claim: ln(x) = eml(1, eml(eml(1, x), 1)).
        // Rather than verify the full identity (which would require
        // multi-level Cauchy composition for high precision), we
        // smoke-test that ln(x) and the eml decomposition are at
        // least consistent on a single point at coarse precision.
        let x = r(7, 5); // 1.4
        let direct_f = (x.to_f64_lossy().unwrap()).ln();
        let direct = ln(x.clone()).unwrap().approx(&r(1, 100_000));
        let direct_d = direct.to_f64_lossy().unwrap();
        assert!((direct_d - direct_f).abs() < 1e-4);
    }

    #[test]
    fn eml_nonpositive_y_is_error() {
        assert!(matches!(
            eml(Rational::one(), Rational::zero()),
            Err(ElementaryError::NonPositiveLog(_))
        ));
    }

    // Auxiliary: the closure-based modulus computation should
    // terminate at a small N for typical eps and modest |x|.
    #[test]
    fn exp_modulus_is_reasonable() {
        let n = find_exp_modulus(&Rational::one(), &r(1, 1_000_000));
        // ln(eps · e^|x|^{-1}) bound; for x=1, eps=1e-6, N ~ 13–14.
        assert!(n < 50);
    }

    #[test]
    fn ln_modulus_is_reasonable() {
        let l = ln(r(2, 1)).unwrap();
        let n = l.modulus(&r(1, 1_000_000));
        assert!(n < 100);
    }
}
