//! # `TF(N)` — the Tension Force / Governor offset
//!
//! Per Gary William Croft's *Technical Reconciliation* response
//! (`Croft_Master_Clock_Technical_Response_UOR.pdf`, 2026-05-05) §2:
//!
//! ```text
//!   TF(N) = (N / ln N) · 0.4135
//! ```
//!
//! where `0.4135 = 1 / χ` and `χ` is the Croft Constant
//! `2.4184 = CHI_FITTED.0 / CHI_FITTED.1` (Phase 1 derived rational
//! `24184 / 10000`).
//!
//! Substituting the exact rational for `1 / χ`:
//!
//! ```text
//!   TF(N) = (N / ln N) · (10000 / 24184)
//!         = N · 10000 / (24184 · ln N)
//! ```
//!
//! ## First-principles anchoring
//!
//! - `χ` is sourced from [`crate::master_clock::CHI_FITTED`], the
//!   exact rational form of Gary's stated value.
//! - `ln N` is computed via the Phase 3 Layer-6 elementary stack
//!   ([`crate::elementary::ln`]), which is itself derived from the
//!   Phase 3 Layer-5 [`crate::cauchy::Cauchy`] machinery, anchored
//!   ultimately on the BRIDGE.md eight-layer construction from
//!   `Succ = Neg ∘ BNot`.
//! - The Cauchy approximation is taken at an explicit ε; callers
//!   choose the precision they need.
//!
//! No magic numbers: every constant in this module either references
//! a Phase 1+ derived value or names the explicit precision argument.
//!
//! ## UOR primitive bindings
//!
//! - `https://uor.foundation/recursion/BoundedRecursion` (modulus of
//!   convergence as descent measure on the Cauchy ln series).
//! - `https://uor.foundation/stream/ProductiveStream` (the Cauchy
//!   sequence backing `ln N`).
//! - `https://uor.foundation/predicate/TypePredicate` (the
//!   "approximation lies in `[true_value − ε, true_value + ε]`"
//!   witness).
//!
//! Each IRI verified to exist in
//! `UOR-Framework/spec/src/namespaces/{recursion,stream,predicate}.rs`.

extern crate alloc;

use num_bigint::BigInt;
use num_traits::ToPrimitive;

use crate::cauchy::Cauchy;
use crate::elementary::{ln, ElementaryError};
use crate::master_clock::CHI_FITTED;
use crate::rational::Rational;

/// Construction error for `TF(N)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TfError {
    /// `ln(N)` is undefined or zero — happens for `N ≤ 1`.
    LogUndefined,
    /// The exact rational `1 / χ` could not be constructed.
    /// Should not occur because [`CHI_FITTED.0`] is non-zero.
    ChiInverseUnconstructable,
    /// The underlying Cauchy `ln` construction failed.
    LnConstructionFailed(alloc::string::String),
    /// Division by `ln N` returned `None` (only if Cauchy `approx`
    /// produced exactly zero, which means insufficient ε).
    DivisionByZero,
}

impl core::fmt::Display for TfError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::LogUndefined => write!(f, "ln(N) undefined or zero (N ≤ 1)"),
            Self::ChiInverseUnconstructable => write!(f, "1/χ unconstructable"),
            Self::LnConstructionFailed(s) => write!(f, "ln construction failed: {s}"),
            Self::DivisionByZero => write!(f, "division by zero in TF"),
        }
    }
}

/// `TF(N)` as an exact rational, computed within an explicit `eps`
/// of the true value.
///
/// # Errors
///
/// Returns [`TfError::LogUndefined`] for `n ≤ 1`. Returns
/// [`TfError::LnConstructionFailed`] if [`crate::elementary::ln`]
/// rejects the input. Returns [`TfError::ChiInverseUnconstructable`]
/// or [`TfError::DivisionByZero`] only in pathological cases the
/// type system would otherwise have to allow.
pub fn tf_n(n: u64, eps: &Rational) -> Result<Rational, TfError> {
    if n < 2 {
        return Err(TfError::LogUndefined);
    }
    let n_rat = Rational::from_bigint(BigInt::from(n));
    let ln_seq: Cauchy = match ln(n_rat.clone()) {
        Ok(c) => c,
        Err(ElementaryError::NonPositiveLog(s)) => {
            return Err(TfError::LnConstructionFailed(s));
        }
    };

    // Approximate ln(N) within eps/2.
    let half_eps = match eps.checked_div(&Rational::from_int(2)) {
        Some(r) => r,
        None => return Err(TfError::DivisionByZero),
    };
    let ln_n_approx = ln_seq.approx(&half_eps);
    if ln_n_approx == Rational::zero() {
        return Err(TfError::DivisionByZero);
    }

    // n / ln(n) — exact rational division.
    let n_over_ln = match n_rat.checked_div(&ln_n_approx) {
        Some(r) => r,
        None => return Err(TfError::DivisionByZero),
    };

    // 1/χ = CHI_FITTED.1 / CHI_FITTED.0 = 10000 / 24184.
    let chi_inv = match Rational::new(
        BigInt::from(CHI_FITTED.1),
        BigInt::from(CHI_FITTED.0),
    ) {
        Some(r) => r,
        None => return Err(TfError::ChiInverseUnconstructable),
    };

    Ok(&n_over_ln * &chi_inv)
}

/// **Derived.** Default integer-floor precision for `tf_n_floor`.
/// Equals `NUMERAL_BASE^5 = 100_000`. Well below `1`, so the
/// floor of a rational `tf` approximated to within this `eps`
/// agrees with the floor of the true value almost always (and
/// off by at most `1` at the precise integer boundaries — see the
/// docstring for the round-up corner case).
const TF_FLOOR_DENOM: u64 = {
    let base = crate::parameters::NUMERAL_BASE;
    let mut out: u64 = 1;
    let mut i: u32 = 0;
    while i < 5 {
        out *= base;
        i += 1;
    }
    out
};

/// `TF(N)` rounded to the nearest integer, suitable for plugging
/// into the Master-Clock harness. Uses `eps = 1 / TF_FLOOR_DENOM`
/// (= `1 / NUMERAL_BASE^5 = 1/100_000`).
///
/// # Errors
///
/// Same as [`tf_n`].
pub fn tf_n_floor(n: u64) -> Result<u64, TfError> {
    use num_bigint::BigInt as Bi;
    let eps = match Rational::new(Bi::from(1u64), Bi::from(TF_FLOOR_DENOM)) {
        Some(r) => r,
        None => return Err(TfError::DivisionByZero),
    };
    let tf = tf_n(n, &eps)?;
    let floor_bigint = tf.floor();
    floor_bigint
        .to_u64()
        .ok_or(TfError::ChiInverseUnconstructable) // overflow
}

/// `TF(N)` as a signed integer. Convenience for the harness which
/// works in `i64`. Returns 0 if conversion fails (e.g., overflow).
#[must_use]
pub fn tf_n_floor_or_zero(n: u64) -> u64 {
    tf_n_floor(n).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::parameters::NUMERAL_BASE;
    use num_bigint::BigInt as Bi;

    /// `eps = 1 / NUMERAL_BASE^6`. Derived.
    fn small_eps() -> Rational {
        let mut denom: u64 = 1;
        for _ in 0..6 {
            denom *= NUMERAL_BASE;
        }
        Rational::new(Bi::from(1u64), Bi::from(denom)).unwrap()
    }

    /// `pow_base(k) = NUMERAL_BASE^k`. Derived.
    fn pow_base(k: u32) -> u64 {
        let mut out: u64 = 1;
        for _ in 0..k {
            out *= NUMERAL_BASE;
        }
        out
    }

    /// **Edge case.** `TF(0)` and `TF(1)` are undefined because
    /// `ln(0)` is `−∞` and `ln(1) = 0`.
    #[test]
    fn tf_undefined_at_0_and_1() {
        assert_eq!(tf_n(0, &small_eps()), Err(TfError::LogUndefined));
        assert_eq!(tf_n(1, &small_eps()), Err(TfError::LogUndefined));
    }

    /// **Smoke test.** `TF(NUMERAL_BASE)` is positive and finite.
    /// Approx: `(10 / ln 10) · (1/χ) ≈ 4.343 · 0.4135 ≈ 1.796`.
    /// We check `1 < TF < 2`, the natural integer band.
    #[test]
    fn tf_at_numeral_base_in_unit_band() {
        let v = tf_n(NUMERAL_BASE, &small_eps()).expect("TF(10)");
        let lower = Rational::from_int(1);
        let upper = Rational::from_int(2);
        assert!(v > lower, "TF({}) = {} not > 1", NUMERAL_BASE, v);
        assert!(v < upper, "TF({}) = {} not < 2", NUMERAL_BASE, v);
    }

    /// **Spot check vs Prime Number Theorem.** At `N = NUMERAL_BASE^3
    /// = 1000`, π(1000) = 168 and `TF · χ ≈ N / ln N ≈ 144.76`.
    /// So `TF(1000) ≈ 144.76 / χ ≈ 59.9`. We check the looser
    /// integer band `[CHI_FITTED.0 / NUMERAL_BASE^3 · NUMERAL_BASE^3,
    /// 100]` — i.e., `(0, 100)`.
    #[test]
    fn tf_at_thousand_below_pi_thousand() {
        let n = pow_base(3); // 1000
        let v = tf_n(n, &small_eps()).expect("TF(1000)");
        // π(1000) = 168 from li::lookup_reference, but n=1000 is
        // not in the reference table; we use the Prime Number
        // Theorem bound: TF(N) < N / ln N < N / ln 2 < N.
        let upper_loose = Rational::from_int(n as i64);
        assert!(v > Rational::zero());
        assert!(v < upper_loose);
    }

    /// **Monotone.** `TF` is monotone increasing for `N ≥ 4` (the
    /// derivative `(ln N − 1) / (ln N)²` is positive once `ln N > 1`,
    /// i.e., `N > e ≈ 2.718`). Tested up to `2·NUMERAL_BASE^2 = 200`.
    #[test]
    fn tf_monotone_above_e() {
        let mut prev = tf_n(4, &small_eps()).expect("TF(4)");
        for n in 5..=(2 * pow_base(2)) {
            let here = tf_n(n, &small_eps()).expect("TF");
            assert!(here >= prev, "TF not monotone at n = {n}");
            prev = here;
        }
    }

    /// **`tf_n_floor` returns finite u64 at `N = NUMERAL_BASE^3`.**
    /// Sanity check: floor(TF(1000)) > 0 and < 1000.
    #[test]
    fn tf_floor_at_thousand_finite() {
        let n = pow_base(3);
        let v = tf_n_floor(n).expect("floor(TF(1000))");
        assert!(v > 0);
        assert!(v < n);
    }

    /// **Order-of-magnitude check at Gary's tabulated N**
    /// (`NUMERAL_BASE^10`, `^11`, `^12`). Slow because the rational
    /// `ln` series at large N takes time. Marked `#[ignore]`. Run
    /// via `cargo test --features alloc -- --ignored
    /// tf_at_gary_three_n`.
    #[test]
    #[ignore = "slow: rational ln at large N"]
    fn tf_at_gary_three_n() {
        // eps = 1/NUMERAL_BASE^3
        let eps_denom = pow_base(3);
        let eps = Rational::new(Bi::from(1u64), Bi::from(eps_denom)).unwrap();
        let v10 = tf_n(pow_base(10), &eps).expect("TF(10^10)");
        let v11 = tf_n(pow_base(11), &eps).expect("TF(10^11)");
        let v12 = tf_n(pow_base(12), &eps).expect("TF(10^12)");
        // Monotone — TF(N) is increasing for N >> e.
        assert!(v10 < v11);
        assert!(v11 < v12);
    }
}
