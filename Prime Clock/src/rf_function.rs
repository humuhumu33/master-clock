//! # `RF(N)` — Resonant Redundancy with the F49 ratchet
//!
//! Per Gary William Croft's *Technical Reconciliation* response
//! (`Croft_Master_Clock_Technical_Response_UOR.pdf`, 2026-05-05) §3
//! and the follow-up reply (chat, 2026-05-05) confirming the decay
//! constant:
//!
//! ```text
//!   RF(N) = χ · ln(N / 10⁹)                            for N < F49
//!   RF(N) = [χ · ln(F49 / 10⁹)] · e^{−k · (N − F49)}   for N ≥ F49
//! ```
//!
//! where:
//!
//! - `χ = 24184/10000` (the Croft Constant, Phase 1
//!   [`crate::master_clock::CHI_FITTED`]),
//! - `F49 = 7,778,742,049 = F_49` (Phase 1
//!   [`crate::fibonacci::F49_STATED`], compile-time-verified),
//! - `k = 1272 / 10¹³ = 1.272 × 10⁻¹⁰` ([`K_NUMERATOR`] /
//!   [`K_DENOMINATOR`]). Gary describes `k` as the
//!   **Symmetry-Break Decay Constant**, derived from the
//!   golden-ratio resonance — its 4-digit value matches
//!   `√φ × 10⁻¹⁰ ≈ 1.27201965 × 10⁻¹⁰`. Until the precise
//!   functional form is confirmed, we use the literal stated value.
//!
//! ## First-principles anchoring
//!
//! Every constant traces back to Phase 1:
//!
//! - `χ` ← `master_clock::CHI_FITTED` (= 24184/10000)
//! - `F49` ← `fibonacci::F49_STATED` (= 7,778,742,049)
//! - `ln`, `exp` ← `crate::elementary` (Phase 3 Cauchy<Rational> stack
//!   built on the BRIDGE.md eight-layer derivation from
//!   `Succ = Neg ∘ BNot`)
//!
//! `k` is the only newly-introduced constant — Gary's stated
//! `1.272 × 10⁻¹⁰`. Its golden-ratio derivation
//! (`k = √φ × 10⁻¹⁰`?) is recorded in source comments but not
//! committed in code; flipping to the irrational form is a one-line
//! edit if confirmed.
//!
//! ## UOR primitive bindings
//!
//! - `https://uor.foundation/recursion/BoundedRecursion` (descent
//!   measure on the `exp` and `ln` Taylor/Mercator partial sums).
//! - `https://uor.foundation/stream/ProductiveStream` (the Cauchy
//!   sequences backing both elementary functions).
//! - `https://uor.foundation/predicate/TypePredicate` (the
//!   "approximation lies in `[true_value − ε, true_value + ε]`"
//!   witness).

extern crate alloc;

use num_bigint::BigInt;

use crate::arithmetic::largest_power_below;
use crate::cauchy::Cauchy;
use crate::elementary::{exp, ln, ElementaryError};
use crate::fibonacci::F49_STATED;
use crate::master_clock::CHI_FITTED;
use crate::parameters::NUMERAL_BASE;
use crate::rational::Rational;

// ─── Parameters from Gary's Technical Reconciliation (verification targets) ─
//
// `K_NUMERATOR` and `K_PRECISION_DIGITS` together encode the decay
// constant Gary stated as `k ≈ 1.272 × 10⁻¹⁰`. Following the Phase 1
// discipline (parameters.rs), these are classified as **parameters /
// verification targets** sourced from his
// `Croft_Master_Clock_Technical_Response_UOR.pdf` follow-up, not
// derived from earlier constants. They are documented as such; if
// Gary refines `k` (e.g., to the precise `√φ × 10⁻¹⁰`), only these
// two literals change.

/// Decimal-significant-digits part of Gary's stated `k`.
/// `1.272 × 10⁻¹⁰` → `K_NUMERATOR = 1272` at four-digit precision.
///
/// Verification target. The 4-digit value matches `√φ × 10⁻¹⁰` to
/// four digits (`√φ ≈ 1.27201965`); switching to the irrational
/// form is a one-line edit if Gary confirms.
pub const K_NUMERATOR: u64 = 1_272;

/// Number of significant digits captured by `K_NUMERATOR`.
/// `1272` carries 4 significant digits.
pub const K_PRECISION_DIGITS: u32 = 4;

/// Decimal exponent of Gary's stated `k`. `1.272 × 10⁻¹⁰` →
/// `K_DECIMAL_EXPONENT = 10`.
pub const K_DECIMAL_EXPONENT: u32 = 10;

// Helper: compile-time `base^exp`.
const fn pow_const(base: u64, exp: u32) -> u64 {
    let mut out: u64 = 1;
    let mut i: u32 = 0;
    while i < exp {
        out *= base;
        i += 1;
    }
    out
}

/// **Derived.** Denominator of the exact rational `k`:
/// `NUMERAL_BASE^(K_PRECISION_DIGITS - 1 + K_DECIMAL_EXPONENT)`.
/// For `K_NUMERATOR = 1272`, `K_PRECISION_DIGITS = 4`,
/// `K_DECIMAL_EXPONENT = 10` this is `10^13 = 10_000_000_000_000`.
pub const K_DENOMINATOR: u64 =
    pow_const(NUMERAL_BASE, K_PRECISION_DIGITS - 1 + K_DECIMAL_EXPONENT);

// Compile-time anchor: K_DENOMINATOR matches the value Gary's
// shorthand `× 10⁻¹⁰` plus 4-digit `1.272` implies.
const _: () = assert!(
    K_DENOMINATOR == 10_000_000_000_000,
    "K_DENOMINATOR must reproduce 10^13 from NUMERAL_BASE and Gary's stated precision"
);

/// **Derived.** The threshold scaling factor in `RF(N) = χ ·
/// ln(N / RF_LOG_SCALE)`. Equals the largest power of
/// `NUMERAL_BASE` strictly below `F49_STATED` —
/// `10⁹ = NUMERAL_BASE^9 < F_49 ≈ 7.778 × 10⁹`. Both inputs are
/// Phase 1 parameters, so `RF_LOG_SCALE` is fully derived.
pub const RF_LOG_SCALE: u64 = largest_power_below(NUMERAL_BASE, F49_STATED);

// Compile-time anchor: RF_LOG_SCALE = 10⁹ as Gary's formula uses.
const _: () = assert!(
    RF_LOG_SCALE == 1_000_000_000,
    "RF_LOG_SCALE must equal 10^9 (the largest power of NUMERAL_BASE below F_49)"
);

/// Construction error for `RF(N)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RfError {
    /// `ln(arg)` was undefined or zero (only happens at edges).
    LnUndefined,
    /// The Phase 3 Cauchy `ln` construction failed.
    LnFailed(alloc::string::String),
    /// Division by zero somewhere in the rational pipeline.
    DivisionByZero,
    /// `χ` could not be constructed as a rational. Only fires if
    /// `CHI_FITTED.1 == 0`, which is not possible.
    ChiInvalid,
    /// `k` could not be constructed. Same provenance as `ChiInvalid`.
    KInvalid,
}

impl core::fmt::Display for RfError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::LnUndefined => write!(f, "ln(N/10^9) undefined or zero"),
            Self::LnFailed(s) => write!(f, "ln Cauchy construction failed: {s}"),
            Self::DivisionByZero => write!(f, "division by zero in RF"),
            Self::ChiInvalid => write!(f, "χ rational unconstructable"),
            Self::KInvalid => write!(f, "k rational unconstructable"),
        }
    }
}

/// `RF(N)` as an exact rational, computed within explicit `eps` of
/// the true value via Phase 3 `Cauchy<Rational>`.
///
/// Pre-F49: returns `χ · ln(N/10⁹)`. Post-F49: returns
/// `[χ · ln(F49/10⁹)] · e^{−k·(N − F49)}`.
///
/// # Errors
///
/// Returns [`RfError::LnUndefined`] for `N == 0` (since `ln(0)` is
/// undefined). All other errors are pathological and indicate a
/// catastrophic upstream failure.
pub fn rf_n(n: u64, eps: &Rational) -> Result<Rational, RfError> {
    let chi = build_chi()?;
    let scale = Rational::from_bigint(BigInt::from(RF_LOG_SCALE));

    if n == 0 {
        return Err(RfError::LnUndefined);
    }

    if n < F49_STATED {
        // Pre-F49: RF(N) = χ · ln(N / 10⁹).
        // Note: at N = 10⁹, ln(1) = 0 → RF = 0 exactly.
        let n_rat = Rational::from_bigint(BigInt::from(n));
        let arg = n_rat.checked_div(&scale).ok_or(RfError::DivisionByZero)?;
        let ln_val = approx_ln(arg, eps)?;
        Ok(&chi * &ln_val)
    } else {
        // Post-F49: RF(N) = amplitude · e^{−k·(N − F49)}.
        let f49_rat = Rational::from_bigint(BigInt::from(F49_STATED));
        let amplitude_arg = f49_rat
            .checked_div(&scale)
            .ok_or(RfError::DivisionByZero)?;
        let quarter_eps = eps
            .checked_div(&Rational::from_int(4))
            .ok_or(RfError::DivisionByZero)?;
        let ln_amplitude = approx_ln(amplitude_arg, &quarter_eps)?;
        let amplitude = &chi * &ln_amplitude;

        // Exponent: −k · (N − F49). Both factors are exact rationals.
        let k = Rational::new(
            BigInt::from(K_NUMERATOR),
            BigInt::from(K_DENOMINATOR),
        )
        .ok_or(RfError::KInvalid)?;
        let delta_n = n - F49_STATED;
        let delta_rat = Rational::from_bigint(BigInt::from(delta_n));
        let neg_exponent = -(&k * &delta_rat);

        let exp_seq: Cauchy = exp(neg_exponent);
        let exp_val = exp_seq.approx(&quarter_eps);
        Ok(&amplitude * &exp_val)
    }
}

/// `RF(N)` rounded toward zero as a signed integer, suitable for the
/// Master-Clock harness. Uses `eps = 1 / 100_000`.
///
/// # Errors
///
/// Same as [`rf_n`].
pub fn rf_n_truncated(n: u64) -> Result<i64, RfError> {
    let eps = Rational::new_i64(1, 100_000).ok_or(RfError::DivisionByZero)?;
    let rf = rf_n(n, &eps)?;
    use num_traits::ToPrimitive;
    let bigint = rf.floor();
    bigint.to_i64().ok_or(RfError::DivisionByZero) // overflow shim
}

/// Helper: Cauchy `ln(arg)` with stated `eps`.
fn approx_ln(arg: Rational, eps: &Rational) -> Result<Rational, RfError> {
    if arg.signum() <= 0 {
        return Err(RfError::LnUndefined);
    }
    let ln_seq = ln(arg).map_err(|e| match e {
        ElementaryError::NonPositiveLog(s) => RfError::LnFailed(s),
    })?;
    Ok(ln_seq.approx(eps))
}

/// Helper: build `χ` as exact rational from `CHI_FITTED`.
fn build_chi() -> Result<Rational, RfError> {
    Rational::new(
        BigInt::from(CHI_FITTED.0),
        BigInt::from(CHI_FITTED.1),
    )
    .ok_or(RfError::ChiInvalid)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn small_eps() -> Rational {
        Rational::new_i64(1, 1_000_000).unwrap()
    }

    use crate::parameters::NUMERAL_BASE;

    /// **Constants anchor.** Every `k`-related constant traces to
    /// `NUMERAL_BASE` and Gary's stated precision/exponent.
    /// `K_DENOMINATOR = NUMERAL_BASE^(K_PRECISION_DIGITS − 1 +
    /// K_DECIMAL_EXPONENT) = 10^(4 − 1 + 10) = 10^13`.
    #[test]
    fn k_constants_anchor_to_numeral_base_and_gary_precision() {
        // K_NUMERATOR is Gary's stated 4-digit value (verification target).
        assert_eq!(K_NUMERATOR, 1272);
        assert_eq!(K_PRECISION_DIGITS, 4);
        assert_eq!(K_DECIMAL_EXPONENT, 10);
        // K_DENOMINATOR fully derived from NUMERAL_BASE = 10:
        let mut expected = 1u64;
        for _ in 0..(K_PRECISION_DIGITS - 1 + K_DECIMAL_EXPONENT) {
            expected *= NUMERAL_BASE;
        }
        assert_eq!(K_DENOMINATOR, expected);
    }

    /// **`RF_LOG_SCALE` is fully derived.** From
    /// `NUMERAL_BASE = 10` and `F49_STATED = 7,778,742,049` via
    /// `largest_power_below`. No literal `10⁹` anywhere.
    #[test]
    fn rf_log_scale_is_largest_numeral_power_below_f49() {
        assert_eq!(
            RF_LOG_SCALE,
            largest_power_below(NUMERAL_BASE, F49_STATED)
        );
        // Anchor-cross-check: RF_LOG_SCALE × NUMERAL_BASE > F49_STATED
        // and RF_LOG_SCALE ≤ F49_STATED.
        assert!(RF_LOG_SCALE <= F49_STATED);
        assert!(RF_LOG_SCALE * NUMERAL_BASE > F49_STATED);
    }

    /// **Edge: `N = 0`.** `RF` undefined.
    #[test]
    fn rf_at_zero_undefined() {
        assert_eq!(rf_n(0, &small_eps()), Err(RfError::LnUndefined));
    }

    /// **Pre-F49: `N = RF_LOG_SCALE`.** `RF = χ · ln(N/N) = χ · ln(1)
    /// = 0`. Computed approximately, expected within `1/100_000`.
    /// `RF_LOG_SCALE` is the natural zero-point of pre-F49 RF.
    #[test]
    fn rf_at_log_scale_is_approximately_zero() {
        let r = rf_n(RF_LOG_SCALE, &small_eps()).expect("RF(RF_LOG_SCALE)");
        let bound = Rational::new_i64(1, 100_000).unwrap();
        let abs = r.abs();
        assert!(
            abs < bound,
            "RF(RF_LOG_SCALE) = {} not within 1/100,000 of 0",
            r
        );
    }

    /// **Continuity at F49.** Pre-form and post-form must agree at
    /// `N = F49`. Pre at `F49−1` and post at `F49` differ only by
    /// `χ · ln(F49/(F49−1)) ≈ χ/F49 ≈ 3·10⁻¹⁰` — well within the
    /// 1/100,000 sanity bound.
    #[test]
    fn rf_continuous_at_f49_boundary() {
        let pre = rf_n(F49_STATED - 1, &small_eps()).expect("RF(F49-1)");
        let post = rf_n(F49_STATED, &small_eps()).expect("RF(F49)");
        let diff = (&pre - &post).abs();
        let bound = Rational::new_i64(1, 100_000).unwrap();
        assert!(diff < bound, "discontinuity at F49: |pre − post| = {}", diff);
    }

    /// **Pre-F49 monotone above the log scale.** `RF(N) = χ ·
    /// ln(N/RF_LOG_SCALE)` is monotone increasing for `N >
    /// RF_LOG_SCALE`. Test points are derived multiples of
    /// `RF_LOG_SCALE`, all below `F49_STATED ≈ 7.78 · RF_LOG_SCALE`.
    #[test]
    fn rf_pre_f49_monotone_above_log_scale() {
        // 2·RF_LOG_SCALE = 2·10⁹ < F49.
        let mut prev = rf_n(2 * RF_LOG_SCALE, &small_eps()).expect("RF");
        for k in [3u64, 4, 5, 7] {
            let n = k * RF_LOG_SCALE;
            assert!(n < F49_STATED, "test point must be pre-F49");
            let here = rf_n(n, &small_eps()).expect("RF");
            assert!(here > prev, "RF not monotone at N = {n}");
            prev = here;
        }
    }

    /// **Post-F49 decays.** At `N = F49`, `RF` is at its peak.
    /// At `N = F49 + RF_LOG_SCALE`, `RF` is smaller — decay term is
    /// `e^{−k · RF_LOG_SCALE} < 1`.
    #[test]
    fn rf_post_f49_decays() {
        let at_peak = rf_n(F49_STATED, &small_eps()).expect("RF(F49)");
        let later =
            rf_n(F49_STATED + RF_LOG_SCALE, &small_eps()).expect("RF later");
        assert!(later < at_peak, "RF not decaying after F49");
    }

    /// **Live computation at `N = NUMERAL_BASE^10`** — the smallest
    /// of Gary's three Validated_Theorem N-points and the slowest
    /// post-F49 `exp` argument among them. Marked `#[ignore]`.
    #[test]
    #[ignore = "slow: rational exp at large argument"]
    fn rf_at_gary_smallest_n() {
        let eps = Rational::new_i64(1, 1000).unwrap();
        // 10^10 derived from NUMERAL_BASE.
        let mut n: u64 = 1;
        for _ in 0..10 {
            n *= NUMERAL_BASE;
        }
        let r = rf_n(n, &eps).expect("RF(NUMERAL_BASE^10)");
        // Magnitude: amplitude = χ · ln(F49/RF_LOG_SCALE) ≈ 4.96;
        // decay e^{−k(N−F49)} for delta ≈ 2.22·RF_LOG_SCALE gives
        // e^{−0.282} ≈ 0.754; product ≈ 3.74. Range check in [3, 5].
        let lo = Rational::new_i64(3, 1).unwrap();
        let hi = Rational::new_i64(5, 1).unwrap();
        assert!(r > lo && r < hi, "RF(10^10) = {} out of [3, 5]", r);
    }
}
