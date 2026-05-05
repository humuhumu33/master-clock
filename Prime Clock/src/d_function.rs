//! # `D(N)` — the Master-Clock lattice tally
//!
//! Per Gary William Croft's *UOR Developer's Guide v1.1*, posted in
//! the UOR Prime Time chat on **2026-05-04 04:15 UTC**, the lattice
//! tally `D(N)` is the running total of Fibo_DR weights as `n` walks
//! `[1, N]`, addressed by `n mod 360`:
//!
//! ```text
//!   For each n in [1, N]:
//!     index = n mod 360
//!     if index matches one of the 96 R(360) residue positions:
//!       W = Matrix[index]
//!     else:
//!       W = 0
//!     Running_Total += W
//! ```
//!
//! `Matrix[index]` is the `Fibo_DR` (third) column of Appendix A —
//! `dr(F_n)` evaluated at the room offset.
//!
//! ## What the weights actually measure
//!
//! The weights summed by `D(N)` are **not** prime indicators.
//! Each weight is `dr(F_n)` — the digital root of the `n`-th
//! Fibonacci number — evaluated at one of the 96 lattice positions.
//! Those 96 positions are the **irreducibles modulo 30** within
//! `[1, 360]` (units of `(ℤ/30)*` lifted across 12 layers; see
//! [`crate::r360`] and [`crate::u30`]). They include composite
//! integers like `49 = 7²` and `77 = 7·11` whenever the composite
//! is coprime to 30.
//!
//! `D(N)` therefore measures **lattice position × Fibonacci-DR
//! resonance**, not primality. This matches Gary's Technical
//! Reconciliation §1: *"The algorithm does not 'identify' primes;
//! it measures Lattice Vacancy."* Primality is recovered (in the
//! full Master-Clock formula `π(N) = D − TF + RF + 3`) from the
//! interaction of `D` with `TF(N)` and `RF(N)`, not from any
//! per-position prime test inside `D` itself.
//!
//! ## Derivation from first principles
//!
//! Every value in this module is sourced from the Phase 1 derivations:
//!
//! 1. The 96 R(360) residue positions are
//!    [`crate::r360::ELEMENTS`], computed from
//!    [`crate::parameters::MODULUS_U`] via [`crate::arithmetic::gcd`].
//! 2. The `Fibo_DR` weight at each position is the third column of
//!    [`crate::appendix_a::ROWS`], computed from
//!    [`crate::digital_root::dr`] composed with
//!    [`crate::fibonacci::dr_fib`] (which is itself
//!    `dr ∘ fib mod Pisano(9)` per
//!    [`crate::parameters::PISANO_PERIOD_MOD_DR`]).
//! 3. The one-period sum is
//!    [`crate::appendix_a::SUM_FIBO_DR`] = 432, the second 432
//!    invariant, proven structurally as `12 × 36` (twelve layers of
//!    `R(360)`, each summing to 36 over its 8 residues).
//!
//! No magic numbers. The Phase 1 invariant `SUM_FIBO_DR = 432`
//! reappears here as `PERIOD_SUM`, and a compile-time `assert!`
//! confirms the equality.
//!
//! ## Closed form (the periodic landmark structure)
//!
//! `D(N)` decomposes cleanly along the 360-periodicity:
//!
//! ```text
//!   q = N / 360,    r = N mod 360
//!   D(N) = q · PERIOD_SUM  +  partial(r)
//! ```
//!
//! where `partial(r) = Σ_{k=1..r} weight_at(k)`. At every multiple
//! of 360, `D` lands exactly on `q · 432`. At every multiple of
//! 30 (one layer width), `D` advances by exactly 36.
//!
//! ## UOR primitive binding
//!
//! `D(N)` is a `https://uor.foundation/recursion/BoundedRecursion`
//! over the periodic
//! `https://uor.foundation/stream/ProductiveStream` of weight
//! values, with descent measure `n` (the index variable). The
//! `partial(r)` lookup is a
//! `https://uor.foundation/query/CoordinateQuery` over the 96-room
//! matrix.
//!
//! Each IRI above is verified to exist in
//! `UOR-Framework/spec/src/namespaces/{recursion,stream,query}.rs`.
//!
//! These bindings are documented here rather than as
//! `ConstrainedTypeShape` impls because `D(N)` is a function, not a
//! type; the UOR shape lives one level up at the
//! [`crate::master_clock::LatticeTally`] trait.

use crate::appendix_a::{ROWS, SUM_FIBO_DR};
use crate::parameters::{LATTICE_LAYERS, MODULUS_R, R_CARDINALITY};

/// Sum of `Fibo_DR` weights over one full `MODULUS_R`-period — the
/// second 432 invariant from Phase 1, re-anchored here as the period
/// constant of `D(N)`. Equals
/// [`crate::appendix_a::SUM_FIBO_DR`], whose Phase 1 derivation
/// proves `SUM_FIBO_DR = LATTICE_LAYERS · PER_LAYER_FIBO_DR_SUM = 12 · 36`
/// and whose value is compile-time-verified against
/// [`crate::appendix_a::GARY_STATED_TOTAL_FIBO_DR`].
pub const PERIOD_SUM: u64 = SUM_FIBO_DR as u64;

/// Per-`MODULUS_U`-layer Fibo_DR sum — the structural decomposition
/// constant from `appendix_a::sum_fibo_dr_matches_structural_derivation`.
/// **Derived** as `PERIOD_SUM / LATTICE_LAYERS`. Phase 1 proves this
/// equals 36 (the `12 × 36 = 432` structural factorisation).
pub const PER_LAYER_FIBO_DR_SUM: u64 = PERIOD_SUM / LATTICE_LAYERS;

/// `weight_at[k]` is the `Fibo_DR` weight Gary's algorithm assigns
/// to position `n ≡ k (mod 360)`, for `k ∈ [0, 360]`. Position `0`
/// is unused because Gary's tally walks `n ∈ [1, N]`; entries for
/// `k` outside the 96 R(360) residues are zero.
///
/// **Derived** at compile time from [`ROWS`] — no hand-typed
/// weights.
pub const WEIGHTS_PER_OFFSET: [u64; (MODULUS_R as usize) + 1] = compute_weight_table();

const fn compute_weight_table() -> [u64; (MODULUS_R as usize) + 1] {
    let mut out = [0u64; (MODULUS_R as usize) + 1];
    let mut i = 0usize;
    while i < R_CARDINALITY {
        let row = ROWS[i];
        let pos = row.0 as usize;
        let weight = row.2 as u64;
        out[pos] = weight;
        i += 1;
    }
    out
}

// Compile-time anchor: the weight-table sum across one period
// equals PERIOD_SUM. PERIOD_SUM is itself anchored — appendix_a.rs
// proves SUM_FIBO_DR == GARY_STATED_TOTAL_FIBO_DR == 432, and
// PER_LAYER_FIBO_DR_SUM = PERIOD_SUM / LATTICE_LAYERS reproduces the
// structural decomposition `LATTICE_LAYERS · 36 = 432`.
const _: () = {
    let mut sum: u64 = 0;
    let mut k = 0usize;
    while k <= MODULUS_R as usize {
        sum += WEIGHTS_PER_OFFSET[k];
        k += 1;
    }
    assert!(sum == PERIOD_SUM, "WEIGHTS_PER_OFFSET sum does not equal PERIOD_SUM");
    // Restate the structural decomposition: LATTICE_LAYERS · 36 == 432.
    assert!(
        PER_LAYER_FIBO_DR_SUM * LATTICE_LAYERS == PERIOD_SUM,
        "PER_LAYER_FIBO_DR_SUM · LATTICE_LAYERS must reproduce PERIOD_SUM"
    );
};

/// `D(N)` — Gary's lattice tally per Developer's Guide v1.1.
///
/// Computes the running total of `Fibo_DR` weights as `n` walks
/// `[1, N]`, indexed by `n mod 360`. Closed-form:
/// `D(N) = (N / 360) · PERIOD_SUM + partial(N mod 360)`.
///
/// Const-fn so it can be evaluated at compile time for landmark
/// values (e.g., `D(720)`).
#[must_use]
pub const fn d_n(n: u64) -> u64 {
    let q = n / MODULUS_R;
    let r = n % MODULUS_R;
    let mut partial: u64 = 0;
    let mut k: u64 = 1;
    while k <= r {
        partial += WEIGHTS_PER_OFFSET[k as usize];
        k += 1;
    }
    q * PERIOD_SUM + partial
}

/// One-period partial: `partial(r) = Σ_{k=1..r} weight_at(k)`.
/// Useful for tests and for downstream UOR queries that want to
/// inspect within-period structure.
#[must_use]
pub const fn partial(r: u64) -> u64 {
    let r_clamped = if r > MODULUS_R { MODULUS_R } else { r };
    let mut sum: u64 = 0;
    let mut k: u64 = 1;
    while k <= r_clamped {
        sum += WEIGHTS_PER_OFFSET[k as usize];
        k += 1;
    }
    sum
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::appendix_a::{GARY_STATED_TOTAL_FIBO_DR, SUM_FIBO_DR};
    use crate::parameters::{MODULUS_U, U30_CARDINALITY};

    /// **Period anchors.** Every magic-number candidate in this
    /// module reduces to a Phase 1 derived constant.
    /// - `PERIOD_SUM` = `SUM_FIBO_DR` = `GARY_STATED_TOTAL_FIBO_DR` (= 432, proved in `appendix_a.rs`)
    /// - `PER_LAYER_FIBO_DR_SUM` = `PERIOD_SUM / LATTICE_LAYERS` (= 36, structural)
    /// - `MODULUS_R` (= 360, derived from `lcm(MODULUS_U, MODULUS_DR, PISANO_PERIOD_MOD_DR)`)
    /// - `LATTICE_LAYERS` (= 12, derived from `MODULUS_R / MODULUS_U`)
    /// - `MODULUS_U` (= 30, the irreducible Phase 1 parameter)
    #[test]
    fn period_constants_anchor_to_phase_1() {
        assert_eq!(PERIOD_SUM, SUM_FIBO_DR as u64);
        assert_eq!(PERIOD_SUM, GARY_STATED_TOTAL_FIBO_DR as u64);
        assert_eq!(PER_LAYER_FIBO_DR_SUM * LATTICE_LAYERS, PERIOD_SUM);
        assert_eq!(MODULUS_R, MODULUS_U * LATTICE_LAYERS);
        assert_eq!(R_CARDINALITY, U30_CARDINALITY * (LATTICE_LAYERS as usize));
    }

    /// **Anchor.** `D(0) = 0`. The empty walk has zero tally.
    #[test]
    fn d_at_zero_is_zero() {
        assert_eq!(d_n(0), 0);
    }

    /// **One full period.** `D(MODULUS_R) = PERIOD_SUM`. The Phase 1
    /// invariant `SUM_FIBO_DR = 432` reappears as the period constant
    /// of `D`.
    #[test]
    fn d_at_one_period_equals_period_sum() {
        assert_eq!(d_n(MODULUS_R), PERIOD_SUM);
    }

    /// **K full periods.** `D(MODULUS_R · k) = PERIOD_SUM · k` for
    /// `k = 0..=LATTICE_LAYERS`.
    #[test]
    fn d_at_k_full_periods_is_k_times_period_sum() {
        for k in 0u64..=LATTICE_LAYERS {
            let n = MODULUS_R * k;
            assert_eq!(d_n(n), k * PERIOD_SUM, "fail at k = {k}");
        }
    }

    /// **Periodicity.** `D(N + MODULUS_R) − D(N) = PERIOD_SUM` for
    /// every `N`. Walks one full period exhaustively.
    #[test]
    fn d_increments_by_period_sum_every_full_period() {
        for n in 0u64..=MODULUS_R {
            let next = d_n(n + MODULUS_R);
            let here = d_n(n);
            assert_eq!(
                next - here,
                PERIOD_SUM,
                "periodicity broke at N = {n}: D({}) − D({}) = {}",
                n + MODULUS_R,
                n,
                next - here
            );
        }
    }

    /// **Layer landmarks.** `D` advances by exactly
    /// `PER_LAYER_FIBO_DR_SUM` every full `MODULUS_U`-layer. The
    /// `LATTICE_LAYERS` layers in one period contribute exactly
    /// `LATTICE_LAYERS · PER_LAYER_FIBO_DR_SUM = PERIOD_SUM`, the
    /// Phase 1 structural derivation
    /// (`appendix_a::sum_fibo_dr_matches_structural_derivation`).
    #[test]
    fn d_advances_by_per_layer_sum_every_layer() {
        for layer in 0u64..LATTICE_LAYERS {
            let here = d_n(MODULUS_U * layer);
            let next = d_n(MODULUS_U * (layer + 1));
            assert_eq!(
                next - here,
                PER_LAYER_FIBO_DR_SUM,
                "layer {} advances by {}, expected PER_LAYER_FIBO_DR_SUM = {}",
                layer,
                next - here,
                PER_LAYER_FIBO_DR_SUM
            );
        }
        // Total over LATTICE_LAYERS layers = PERIOD_SUM.
        assert_eq!(d_n(MODULUS_R), LATTICE_LAYERS * PER_LAYER_FIBO_DR_SUM);
    }

    /// **Spot values.** First few `D(N)` against the first four
    /// rows of Appendix A. We pull the row data directly from
    /// [`ROWS`] rather than restating it — no magic numbers.
    #[test]
    fn d_spot_values_match_first_appendix_a_rows() {
        let r0 = ROWS[0]; // (1, 1, 1)
        let r1 = ROWS[1]; // (7, 7, 4)
        let r2 = ROWS[2]; // (11, 2, 8)
        let r3 = ROWS[3]; // (13, 4, 8)

        let w0 = r0.2 as u64;
        let w1 = r1.2 as u64;
        let w2 = r2.2 as u64;
        let w3 = r3.2 as u64;

        assert_eq!(d_n(u64::from(r0.0)), w0);
        // r0.0 + 1 .. r1.0 are not in R(360) for these specific rows.
        assert_eq!(d_n(u64::from(r0.0) + 1), w0);
        assert_eq!(d_n(u64::from(r1.0)), w0 + w1);
        assert_eq!(d_n(u64::from(r2.0)), w0 + w1 + w2);
        assert_eq!(d_n(u64::from(r3.0)), w0 + w1 + w2 + w3);
    }

    /// **First-layer sum.** `D(MODULUS_U − 1) = PER_LAYER_FIBO_DR_SUM`.
    /// The first U(30) layer contributes exactly the per-layer sum
    /// from the structural decomposition. `D(MODULUS_U) = D(MODULUS_U − 1)`
    /// because `MODULUS_U` is not in `R(MODULUS_U)`.
    #[test]
    fn d_at_first_layer_boundary_equals_per_layer_sum() {
        assert_eq!(d_n(MODULUS_U - 1), PER_LAYER_FIBO_DR_SUM);
        assert_eq!(d_n(MODULUS_U), PER_LAYER_FIBO_DR_SUM);
    }

    /// **Monotone non-decreasing.** Walks `D(0) … D(MODULUS_R)` and
    /// confirms `D` never decreases. Population: every integer in
    /// one full period.
    #[test]
    fn d_is_monotone_non_decreasing_over_one_period() {
        let mut prev = 0u64;
        for n in 0u64..=MODULUS_R {
            let here = d_n(n);
            assert!(here >= prev, "D not monotone at n = {n}: {here} < {prev}");
            prev = here;
        }
    }

    /// **`partial` at boundary.** `partial(MODULUS_R) = PERIOD_SUM`.
    #[test]
    fn partial_at_full_period_equals_period_sum() {
        assert_eq!(partial(0), 0);
        assert_eq!(partial(MODULUS_R), PERIOD_SUM);
    }

    /// **Closed-form consistency.** `d_n(N) = (N/MODULUS_R)·PERIOD_SUM
    /// + partial(N mod MODULUS_R)` for every `N` in
    /// `[0, 3·MODULUS_R]` — three full periods exhaustive.
    #[test]
    fn d_closed_form_holds_over_three_periods() {
        for n in 0u64..=(3 * MODULUS_R) {
            let q = n / MODULUS_R;
            let r = n % MODULUS_R;
            let expected = q * PERIOD_SUM + partial(r);
            assert_eq!(d_n(n), expected, "closed form fails at N = {n}");
        }
    }

    /// **Weight-table sanity.** `WEIGHTS_PER_OFFSET[0] = 0`
    /// (position 0 is outside the tally), and the table sums to
    /// `PERIOD_SUM` across `[0, MODULUS_R]`.
    #[test]
    fn weight_table_well_formed() {
        assert_eq!(WEIGHTS_PER_OFFSET[0], 0);
        let total: u64 = WEIGHTS_PER_OFFSET.iter().sum();
        assert_eq!(total, PERIOD_SUM);
    }

    /// **Weight table matches Appendix A.** For every R(MODULUS_R)
    /// row, the weight at that position equals the row's `Fibo_DR`.
    /// Exhaustive over all `R_CARDINALITY` rows.
    #[test]
    fn weight_table_matches_every_appendix_a_row() {
        for row in &ROWS {
            let pos = row.0 as usize;
            let weight = row.2 as u64;
            assert_eq!(
                WEIGHTS_PER_OFFSET[pos], weight,
                "weight at offset {pos} disagrees with Appendix A row"
            );
        }
    }
}
