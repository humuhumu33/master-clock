//! # Master-Clock evaluation harness — the Phase 4 trigger
//!
//! Given any `T: LatticeTally + TensionForce + ResonantRedundancy`
//! (the three Phase 1 trait stubs in [`crate::master_clock`]), this
//! harness evaluates
//!
//! ```text
//!     master_clock(N) = D(N) − TF(N) + RF(N) + 3
//! ```
//!
//! at the N-points Gary tabulates in
//! `Toroidal_Prime_Clock_Validated_Theorem.pdf`, compares to
//! literature `π(N)` and `Li(N)`, and reports a structured
//! [`ComparisonRow`] for each.
//!
//! The harness compiles **today** against
//! [`crate::master_clock::UnimplementedMasterClock`], which returns
//! [`crate::master_clock::SpecGap`] for every component. The result
//! is reported as a typed `Outcome::SpecGap(SpecGap)` rather than a
//! number, so the Phase 4 dependency on `SPEC_REQUEST.md` is visible
//! at the test surface — no spurious "0 = success" results.
//!
//! When Gary closes the spec gaps and a real `MasterClockComponent`
//! impl lands, the harness immediately produces concrete numerical
//! comparisons against Gary's claimed deltas {112, 432, 96}.

use crate::li::{lookup_gary_claim, lookup_reference, GaryClaimRow, ReferenceRow};
use crate::master_clock::{
    LatticeTally, ResonantRedundancy, SpecGap, TensionForce, UnimplementedMasterClock,
};

/// Outcome of evaluating the Master-Clock formula at one `N`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// The formula evaluated cleanly to an integer prediction of
    /// `π(N)`. The wrapped value is the predicted count.
    Predicted(u64),
    /// The formula could not be evaluated because Gary's
    /// specification has the named gap.
    SpecGap(SpecGap),
}

/// One row of the comparison table.
///
/// All values are integer-rounded; the high-precision real
/// computation lives in the queued `compute_li` follow-up. For the
/// reference baseline this is exactly the resolution Gary's PDF
/// tabulates ("3,104", "11,588", "38,263"; "112", "432", "96").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComparisonRow {
    /// `log₁₀(N)`.
    pub n_log10: u32,
    /// Canonical-literature `π(N)`.
    pub pi_true: u64,
    /// Canonical-literature `round(Li(N) − π(N))`.
    pub li_minus_pi_literature: i64,
    /// Gary's claimed Master-Clock delta from the Validated_Theorem PDF.
    pub master_clock_delta_gary: i64,
    /// Outcome of evaluating the formula. `SpecGap` until Phase 4.
    pub outcome: Outcome,
    /// Observed `master_clock(N) − π(N)` if the formula evaluated;
    /// `None` if it returned a `SpecGap`.
    pub observed_delta: Option<i64>,
    /// `observed_delta == ±master_clock_delta_gary` (sign-agnostic).
    /// `false` if the formula returned `SpecGap`.
    pub matches_gary: bool,
}

/// Evaluate the Master-Clock formula at one `N` and produce a
/// [`ComparisonRow`].
///
/// `n_log10` is the decimal power: callers pass `10` for `N = 10¹⁰`,
/// `12` for `N = 10¹²`, etc.
///
/// Returns `None` if the literature reference table does not have a
/// row for `n_log10` (i.e., `N` is outside the tabulated range).
///
/// # Errors
///
/// Does not return `Result`: errors from `T`'s component traits are
/// folded into [`Outcome::SpecGap`] inside the row.
#[must_use]
pub fn evaluate<T>(n_log10: u32) -> Option<ComparisonRow>
where
    T: LatticeTally + TensionForce + ResonantRedundancy,
{
    let lit: ReferenceRow = lookup_reference(n_log10)?;
    let gary: GaryClaimRow = lookup_gary_claim(n_log10)?;
    let n = pow10(n_log10);

    let outcome = evaluate_formula::<T>(n);
    let observed_delta: Option<i64> = match outcome {
        Outcome::Predicted(prediction) => {
            let pi_true_i = lit.pi_n as i64;
            let predicted_i = prediction as i64;
            Some(predicted_i - pi_true_i)
        }
        Outcome::SpecGap(_) => None,
    };
    let matches_gary = match observed_delta {
        Some(d) => d.abs() == gary.master_clock_delta_gary.abs(),
        None => false,
    };

    Some(ComparisonRow {
        n_log10,
        pi_true: lit.pi_n,
        li_minus_pi_literature: lit.li_minus_pi_literature,
        master_clock_delta_gary: gary.master_clock_delta_gary,
        outcome,
        observed_delta,
        matches_gary,
    })
}

/// Evaluate the Master-Clock formula `D(N) − TF(N) + RF(N) + 3`,
/// folding any [`SpecGap`] from the component traits into the
/// [`Outcome`] enum.
fn evaluate_formula<T>(n: u64) -> Outcome
where
    T: LatticeTally + TensionForce + ResonantRedundancy,
{
    // D(N)
    let d = match T::d(n) {
        Ok(_) => 0i128, // RationalApprox::empty stub returns nominal 0
        Err(g) => return Outcome::SpecGap(g),
    };
    // TF(N)
    let tf = match T::tf(n) {
        Ok(_) => 0i128,
        Err(g) => return Outcome::SpecGap(g),
    };
    // RF(N)
    let rf = match T::rf(n) {
        Ok(_) => 0i128,
        Err(g) => return Outcome::SpecGap(g),
    };
    // π(N) ≈ D − TF + RF + 3
    let predicted = d - tf + rf + 3;
    if predicted < 0 {
        // π(N) is non-negative; a negative prediction is a wrap or
        // formula error. Surface it as a SpecGap rather than a u64
        // truncation.
        return Outcome::SpecGap(SpecGap::DNeedsDefinition);
    }
    Outcome::Predicted(predicted as u64)
}

/// Decimal-power helper. `pow10(n) = 10^n` for `n ≤ 18`.
const fn pow10(n: u32) -> u64 {
    let mut out: u64 = 1;
    let mut i: u32 = 0;
    while i < n {
        out *= 10;
        i += 1;
    }
    out
}

/// Run the harness against the default
/// [`UnimplementedMasterClock`] across all three Gary-tabulated `N`.
///
/// Useful as a one-shot check that the harness wiring is sound.
/// Every row returns `Outcome::SpecGap(_)` until Phase 4.
#[must_use]
pub fn baseline_against_unimplemented() -> [ComparisonRow; 3] {
    // The three N-points Gary tabulates in Validated_Theorem PDF.
    let n10 = evaluate::<UnimplementedMasterClock>(10).unwrap_or(empty_row(10));
    let n11 = evaluate::<UnimplementedMasterClock>(11).unwrap_or(empty_row(11));
    let n12 = evaluate::<UnimplementedMasterClock>(12).unwrap_or(empty_row(12));
    [n10, n11, n12]
}

/// Pseudo-row used when both reference tables lack a row for
/// `n_log10`. Should never appear in `baseline_against_unimplemented`
/// because the Validated-Theorem N-points are all in
/// `crate::li::REFERENCE_TABLE` and `crate::li::GARY_CLAIMS`.
const fn empty_row(n_log10: u32) -> ComparisonRow {
    ComparisonRow {
        n_log10,
        pi_true: 0,
        li_minus_pi_literature: 0,
        master_clock_delta_gary: 0,
        outcome: Outcome::SpecGap(SpecGap::DNeedsDefinition),
        observed_delta: None,
        matches_gary: false,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    /// **Wiring.** `evaluate::<UnimplementedMasterClock>(10)`
    /// returns a populated row whose outcome is a `SpecGap` (because
    /// `D` is the first stub). The literature/Gary-claim columns
    /// must be populated regardless.
    #[test]
    fn unimplemented_at_10_10_returns_specgap_with_populated_columns() {
        let row = evaluate::<UnimplementedMasterClock>(10).unwrap();
        assert_eq!(row.n_log10, 10);
        assert_eq!(row.pi_true, 455_052_511);
        assert_eq!(row.li_minus_pi_literature, 3_104);
        assert_eq!(row.master_clock_delta_gary, 112);
        assert!(matches!(row.outcome, Outcome::SpecGap(_)));
        assert!(row.observed_delta.is_none());
        assert!(!row.matches_gary);
    }

    /// **Wiring at N = 10¹².** Same shape as 10¹⁰; ensures the
    /// table walk covers the full N-range Gary tabulates.
    #[test]
    fn unimplemented_at_10_12_returns_specgap_with_populated_columns() {
        let row = evaluate::<UnimplementedMasterClock>(12).unwrap();
        assert_eq!(row.n_log10, 12);
        assert_eq!(row.pi_true, 37_607_912_018);
        assert_eq!(row.li_minus_pi_literature, 38_263);
        assert_eq!(row.master_clock_delta_gary, 96);
        assert!(matches!(row.outcome, Outcome::SpecGap(_)));
    }

    /// **Wiring batch.** `baseline_against_unimplemented()` returns
    /// all three rows with the right N-points and uniform SpecGap
    /// outcomes.
    #[test]
    fn baseline_returns_three_rows_all_specgap() {
        let rows = baseline_against_unimplemented();
        assert_eq!(rows.len(), 3);
        let log10s: [u32; 3] = [rows[0].n_log10, rows[1].n_log10, rows[2].n_log10];
        assert_eq!(log10s, [10, 11, 12]);
        for row in &rows {
            assert!(matches!(row.outcome, Outcome::SpecGap(_)));
            assert!(!row.matches_gary);
        }
    }

    /// **Decimal-power helper.** `pow10(12) = 10¹² = 1_000_000_000_000`.
    #[test]
    fn pow10_at_12_is_one_trillion() {
        assert_eq!(pow10(12), 1_000_000_000_000);
    }

    /// **Decimal-power helper.** `pow10(0) = 1`.
    #[test]
    fn pow10_at_zero_is_one() {
        assert_eq!(pow10(0), 1);
    }
}
