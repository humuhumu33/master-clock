//! # Master-Clock components — the speculative superstructure
//!
//! Gary's *Toroidal Prime Counting Master-Clock* (FIXED revision)
//! defines an identity over the prime-counting function:
//!
//! ```text
//! π(N) = D(N) − TF(N) + RF(N) + 3.
//! ```
//!
//! This module provides the **trait scaffolding** for the three
//! components `D`, `TF`, `RF` and the F49 ratchet logic. Every
//! computational body is **stubbed** — the FIXED PDF does not specify
//! enough to implement them. Each stub returns
//! [`Err(SpecGap::*)`](SpecGap) carrying a typed description of what
//! input is missing and which PDF passage it would come from.
//!
//! Phase 1's role is to fix the trait surface so downstream Phase 3
//! impls can plug in once the missing data arrives. The scaffolding
//! is forward-compatible with the Phase 3 [`crate::BRIDGE`] stack —
//! components return [`RationalApprox`] today, which will widen to
//! `Cauchy<Rational>` in Phase 3 without breaking callers.
//!
//! ## What is fully specified by the FIXED PDF
//!
//! - The 96-row residue / Fibonacci-digital-root matrix
//!   ([`crate::appendix_a::ROWS`])
//! - The two `432` invariants ([`crate::appendix_a::SUM_DR`],
//!   [`crate::appendix_a::SUM_FIBO_DR`])
//! - The F49 ratchet anchor `F_49 = 7,778,742,049`
//!   ([`crate::fibonacci::F49_STATED`])
//! - The piecewise structure of `RF(N)`: one form for `N < F49`,
//!   another for `N ≥ F49`
//! - The Croft Constant `χ` in two readings: derived (5/2) and
//!   fitted (24184/10000), see [`CHI_DERIVED`] and [`CHI_FITTED`]
//!
//! ## What is NOT specified
//!
//! Three concrete gaps prevent implementation:
//!
//! - **`D(N)`.** The "raw lattice tally." The FIXED PDF describes
//!   it narratively; the chat-only Developer's Guide v1.1 sketches
//!   `Σ Matrix[N mod 360]` over `N`, but the FIXED PDF alone does
//!   not pin the definition.
//! - **`TF(N)`.** The "tension force / logarithmic thinning offset."
//!   No closed-form expression in either document.
//! - **`RF(N)` parameters.** The post-F49 decay constant `k` in
//!   `e^{−k(N − F49)}` is not given. The Developer's Guide v1.1
//!   states `λ = 0.08333 = 1/12` but does not relate it explicitly
//!   to `k`. Also, the floating-point exponential conflicts with
//!   our exact-arithmetic policy and would need a discrete
//!   re-interpretation (Phase 3, after [`crate::BRIDGE`]'s `eml`
//!   primitive lands).
//!
//! ## How to enable Phase 3 impls
//!
//! 1. Add a [`crate::BRIDGE`]-derived `Cauchy<Rational>` type.
//! 2. Replace [`RationalApprox`] with that type as the components'
//!    return type.
//! 3. Provide a concrete `impl LatticeTally`, `impl TensionForce`,
//!    `impl ResonantRedundancy` for a `MasterClock` struct.
//! 4. Validate the resulting `π(N) = D − TF + RF + 3` against
//!    `crate::dirichlet::pi(N)` ground truth at the canonical
//!    checkpoints.

use crate::fibonacci::F49_STATED;

// ─── Croft Constant — both readings, with explicit residual ──────────

/// **Derived** reading of the Croft Constant.
///
/// `χ_derived = LATTICE_LAYERS / (LATTICE_LAYERS · 4 / R_CARDINALITY)`
/// — equivalently `240 / 96 = 5/2`. Gary's "geometric ratio" of the
/// Pisano-style 240 to the 96-room lattice. Stored as a `(num, den)`
/// rational.
pub const CHI_DERIVED: (u64, u64) = (5, 2);

/// **Fitted** reading of the Croft Constant.
///
/// `χ_fitted = 24184 / 10000`. The value Gary states in the FIXED
/// PDF Developer's Guide v1.1.
pub const CHI_FITTED: (u64, u64) = (24_184, 10_000);

/// Residual between derived and fitted readings.
///
/// `χ_derived − χ_fitted = 5/2 − 24184/10000 = (5·10000 − 24184·2) / (2·10000)
/// = (50000 − 48368) / 20000 = 1632 / 20000 = 51 / 625`.
///
/// This residual is recorded explicitly. Phase 1 makes no claim
/// about which reading is correct; the empirical comparison against
/// `π(N)` ground truth (Phase 3+) decides.
pub const CHI_RESIDUAL: (i64, u64) = (51, 625);

// ─── F49 ratchet anchor (re-exported for clarity) ────────────────────

/// `F_49 = 7,778,742,049` — the ratchet threshold from FIXED PDF §III.
///
/// Below this value, `RF(N)` follows one form; at or above it,
/// the exponential-decay form takes over.
pub const F49: u64 = F49_STATED;

// ─── Ω — the Resonant Volume normalization (Gary 2026-05-05 follow-up) ─

/// **Ω, the Resonant Volume normalization**, as exact rational
/// `(numerator, denominator)`.
///
/// Per Gary William Croft's response to the UOR engineering team
/// (2026-05-05): the literal Master-Clock identity
/// `D − TF + RF + 3` measures **Total Potential Resonance** of the
/// torus, and the **External Prime Count** `π(N)` is recovered by
/// dividing by the Resonant Volume `Ω = LATTICE_LAYERS · χ`.
///
/// Both factors are Phase 1 derived constants:
/// [`crate::parameters::LATTICE_LAYERS`] = 12 from
/// `MODULUS_R / MODULUS_U`, and [`CHI_FITTED`] = `24184/10000`.
/// `Ω = (12 · 24184) / 10000 = 290208 / 10000 = 29.0208`.
///
/// Stored as exact rational so the harness can divide without
/// precision loss.
pub const OMEGA: (u64, u64) = (
    crate::parameters::LATTICE_LAYERS * CHI_FITTED.0,
    CHI_FITTED.1,
);

// Compile-time anchor: Ω = 290208 / 10000.
const _: () = assert!(OMEGA.0 == 290_208, "OMEGA numerator must be 12 · 24184");
const _: () = assert!(OMEGA.1 == 10_000, "OMEGA denominator must be CHI_FITTED.1");

// ─── Phase 3 placeholder for rational approximation ──────────────────

/// **Phase 1 placeholder** for an exact rational approximation to a
/// real-valued quantity.
///
/// In Phase 3 this will be replaced by `Cauchy<Rational>` from
/// [`crate::BRIDGE`]'s analytic stack. The trait surface below uses
/// this opaque type so the API does not change when the upgrade
/// happens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RationalApprox {
    /// Phase 3 will populate this with a `Cauchy<Rational>` handle.
    /// Phase 1 leaves it empty.
    _placeholder: (),
}

impl RationalApprox {
    /// Construct an empty placeholder.
    #[must_use]
    pub const fn empty() -> Self {
        Self { _placeholder: () }
    }
}

// ─── Specification-gap error type ────────────────────────────────────

/// Typed description of an unmet spec gap. Returned by every Phase 1
/// stub instead of `unimplemented!()` (which would panic and trip
/// our `clippy::panic` deny).
///
/// Each variant cites the corresponding question in
/// [`SPEC_REQUEST.md`](../../SPEC_REQUEST.md), the formal blocker
/// for Phase 3 awaiting Gary's reply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SpecGap {
    /// `D(N)` — "raw lattice tally" — has no closed-form definition
    /// in the FIXED PDF. See `SPEC_REQUEST.md` Q1: summand, index
    /// domain, and boundary all unspecified.
    DNeedsDefinition,
    /// `TF(N)` — "tension force / logarithmic thinning offset" — has
    /// no closed-form definition in any source document. See
    /// `SPEC_REQUEST.md` Q2.
    TfNeedsDefinition,
    /// The decay constant `k` in `e^{−k(N − F49)}` is not given
    /// in the FIXED PDF. See `SPEC_REQUEST.md` Q3.
    RfDecayConstantMissing,
    /// The floating-point exponential `e^{x}` violates this crate's
    /// `clippy::float_arithmetic = "deny"` policy. Phase 3 introduces
    /// `eml(x, y) = exp(x) − ln(y)` over `Cauchy<Rational>` to give
    /// an exact-arithmetic encoding. Sufficiency is conditional on
    /// `k ∈ ℚ`; see `SPEC_REQUEST.md` Q4.
    ExponentialEncodingPending,
}

// ─── Component traits ────────────────────────────────────────────────

/// `D(N)` — the raw lattice tally component.
pub trait LatticeTally {
    /// Compute `D(N)`.
    ///
    /// # Errors
    ///
    /// Phase 1 returns [`SpecGap::DNeedsDefinition`] until the FIXED
    /// PDF specifies the exact summation formula.
    fn d(n: u64) -> Result<RationalApprox, SpecGap>;
}

/// `TF(N)` — the tension-force / logarithmic-thinning offset.
pub trait TensionForce {
    /// Compute `TF(N)`.
    ///
    /// # Errors
    ///
    /// Phase 1 returns [`SpecGap::TfNeedsDefinition`] until the FIXED
    /// PDF specifies the formula.
    fn tf(n: u64) -> Result<RationalApprox, SpecGap>;
}

/// `RF(N)` — resonant redundancy with the F49 ratchet.
pub trait ResonantRedundancy {
    /// Compute `RF(N)`.
    ///
    /// # Errors
    ///
    /// Phase 1 returns [`SpecGap::RfDecayConstantMissing`] (when
    /// `N ≥ F49` and the post-ratchet decay form is invoked) or
    /// [`SpecGap::ExponentialEncodingPending`] (when the
    /// exact-arithmetic encoding of `e^{−k(N − F49)}` is required).
    fn rf(n: u64) -> Result<RationalApprox, SpecGap>;
}

// ─── Phase 1 null impl: every component is `Err` ─────────────────────

/// Phase 1 sentinel: implements all three component traits as
/// returning the relevant [`SpecGap`]. Forward-compatible signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UnimplementedMasterClock;

impl LatticeTally for UnimplementedMasterClock {
    fn d(_n: u64) -> Result<RationalApprox, SpecGap> {
        Err(SpecGap::DNeedsDefinition)
    }
}

impl TensionForce for UnimplementedMasterClock {
    fn tf(_n: u64) -> Result<RationalApprox, SpecGap> {
        Err(SpecGap::TfNeedsDefinition)
    }
}

impl ResonantRedundancy for UnimplementedMasterClock {
    fn rf(n: u64) -> Result<RationalApprox, SpecGap> {
        if n >= F49 {
            Err(SpecGap::ExponentialEncodingPending)
        } else {
            Err(SpecGap::RfDecayConstantMissing)
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    /// `χ_derived = 5/2 = 2.5` exactly.
    #[test]
    fn chi_derived_is_five_halves() {
        assert_eq!(CHI_DERIVED, (5, 2));
    }

    /// `χ_fitted = 2.4184` exactly as `(24184, 10000)`.
    #[test]
    fn chi_fitted_is_gary_stated() {
        assert_eq!(CHI_FITTED, (24_184, 10_000));
    }

    /// **Ω = LATTICE_LAYERS · χ_fitted** as exact rational, the
    /// Resonant Volume normalization Gary's follow-up specifies.
    /// `Ω = 12 · 24184/10000 = 290208/10000 = 29.0208`.
    /// Both factors are Phase 1 derived; OMEGA is fully traceable.
    #[test]
    fn omega_is_lattice_layers_times_chi_fitted() {
        use crate::parameters::LATTICE_LAYERS;
        assert_eq!(OMEGA.0, LATTICE_LAYERS * CHI_FITTED.0);
        assert_eq!(OMEGA.1, CHI_FITTED.1);
        assert_eq!(OMEGA, (290_208, 10_000));
    }

    /// `χ_residual = 51/625` — the rational difference between the
    /// derived and fitted readings.
    #[test]
    fn chi_residual_is_consistent() {
        let (n_d, d_d) = CHI_DERIVED;
        let (n_f, d_f) = CHI_FITTED;
        // χ_d − χ_f = (n_d · d_f − n_f · d_d) / (d_d · d_f).
        let num = (n_d as i64) * (d_f as i64) - (n_f as i64) * (d_d as i64);
        let den = d_d * d_f;
        // Reduce to lowest terms manually for the assertion.
        // num = 5·10000 − 24184·2 = 50000 − 48368 = 1632
        // den = 2·10000 = 20000
        // gcd(1632, 20000) = 32, so 1632/20000 = 51/625
        let g = crate::arithmetic::gcd(num.unsigned_abs(), den);
        let reduced_num = num / (g as i64);
        let reduced_den = den / g;
        assert_eq!((reduced_num, reduced_den), CHI_RESIDUAL);
    }

    /// `F49 = 7,778,742,049` — re-exported anchor.
    #[test]
    fn f49_is_consistent_with_fibonacci_module() {
        assert_eq!(F49, 7_778_742_049);
        assert_eq!(F49, F49_STATED);
    }

    /// Phase 1 stubs return the right `SpecGap` for each component.
    #[test]
    fn unimplemented_clock_returns_typed_gaps() {
        assert_eq!(
            UnimplementedMasterClock::d(100),
            Err(SpecGap::DNeedsDefinition)
        );
        assert_eq!(
            UnimplementedMasterClock::tf(100),
            Err(SpecGap::TfNeedsDefinition)
        );
        // Pre-F49 → decay constant missing.
        assert_eq!(
            UnimplementedMasterClock::rf(1_000_000),
            Err(SpecGap::RfDecayConstantMissing)
        );
        // At-or-post-F49 → exponential encoding pending.
        assert_eq!(
            UnimplementedMasterClock::rf(F49),
            Err(SpecGap::ExponentialEncodingPending)
        );
        assert_eq!(
            UnimplementedMasterClock::rf(F49 + 1),
            Err(SpecGap::ExponentialEncodingPending)
        );
    }
}
