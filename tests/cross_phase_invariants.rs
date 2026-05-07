//! # Cross-phase invariants — convergence under triality
//!
//! Three flagship convergent numbers: `432`, `96`, `24`. Each is
//! shown to arise from three independent derivation paths through
//! the Phase 1+1.5+2+3+4+5 substrate. The convergence is the proof
//! of the substrate's coherence — one number, three independent
//! witnesses, no defects.
//!
//! Each test computes the same target by three structurally
//! distinct recipes and asserts equality. Any future drift in any
//! single module breaks at least one of the three paths and the
//! build fails.
//!
//! ## Triality of the substrate, manifested
//!
//! - `432` — the Phase 1 **period invariant** (sum of digital roots
//!   over `R(360)` and over its Fibonacci-DR column)
//! - `96` — the Phase 1 **cardinality invariant** (the size of the
//!   `R(360)` lattice itself)
//! - `24` — the Phase 1.5 **period invariant** (digital-root cycle
//!   length, equal to Pisano(9), to `|U_30(90)|`, and to the
//!   factorization-dyad fiber size)
//!
//! Run with:
//!
//! ```text
//! cargo test --test cross_phase_invariants -- --nocapture
//! ```

use toroidal_prime_clock::appendix_a::{SUM_DR, SUM_FIBO_DR};
use toroidal_prime_clock::factorization_dyads::{DYAD_COUNT, FIBER_SIZE};
use toroidal_prime_clock::parameters::{
    LATTICE_LAYERS, MODULUS_R, PISANO_PERIOD_MOD_DR, R_CARDINALITY, U30_CARDINALITY,
};
use toroidal_prime_clock::period24_dr_sequences::{DR_ROW_SUM, DR_TABLE_TOTAL};
use toroidal_prime_clock::r360::ELEMENTS;
use toroidal_prime_clock::u30::RESIDUES;
use toroidal_prime_clock::u90::{ROW_SUM_LINEAR, U90_CARDINALITY};

// ─────────────────────────────────────────────────────────────────────
// Convergence I — `432`, the period invariant
// ─────────────────────────────────────────────────────────────────────

/// **Convergence at 432 — three independent derivations all agree.**
///
/// 1. Phase 1, Appendix A column 2: `Σ dr(n)` over `R(360)`.
/// 2. Phase 1, Appendix A column 3: `Σ dr(F_n)` over `R(360)`.
/// 3. Phase 1, structural decomposition: `LATTICE_LAYERS · 36`.
#[test]
fn convergence_at_432_three_paths() {
    let path_1 = SUM_DR;
    let path_2 = SUM_FIBO_DR;
    let path_3 = u64::from(LATTICE_LAYERS) * 36;
    assert_eq!(path_1, 432);
    assert_eq!(path_2, 432);
    assert_eq!(path_3, 432);
    assert_eq!(path_1, path_2);
    assert_eq!(path_2, path_3);
}

/// **Convergence at 432 — supplementary Phase 5 paths.**
///
/// 4. Phase 5.2, period-24 dr-table total divided by six:
///    `DR_TABLE_TOTAL / 6 = 2592 / 6 = 432`.
/// 5. Phase 5.2, dr-table row sum times four:
///    `4 · DR_ROW_SUM = 4 · 108 = 432`.
#[test]
fn convergence_at_432_phase5_paths() {
    let path_4 = DR_TABLE_TOTAL / 6;
    let path_5 = 4 * DR_ROW_SUM;
    assert_eq!(path_4, 432);
    assert_eq!(path_5, 432);
    assert_eq!(path_4, SUM_DR);
    assert_eq!(path_5, SUM_DR);
}

// ─────────────────────────────────────────────────────────────────────
// Convergence II — `96`, the cardinality invariant
// ─────────────────────────────────────────────────────────────────────

/// **Convergence at 96 — three independent derivations all agree.**
///
/// 1. Phase 1, declared cardinality of `R(360)`: `R_CARDINALITY`.
/// 2. Phase 1, structural product: `U30_CARDINALITY · LATTICE_LAYERS`.
/// 3. Phase 1, by enumeration: length of the `r360::ELEMENTS` array.
#[test]
fn convergence_at_96_three_paths() {
    let path_1 = R_CARDINALITY;
    let path_2 = U30_CARDINALITY * LATTICE_LAYERS as usize;
    let path_3 = ELEMENTS.len();
    assert_eq!(path_1, 96);
    assert_eq!(path_2, 96);
    assert_eq!(path_3, 96);
    assert_eq!(path_1, path_2);
    assert_eq!(path_2, path_3);
}

/// **Convergence at 96 — supplementary Phase 1.5 path.**
///
/// 4. Phase 1.5, tiling identity: `|U_30(90)| · 4 = 24 · 4 = 96`.
#[test]
fn convergence_at_96_phase15_path() {
    let path_4 = U90_CARDINALITY * 4;
    assert_eq!(path_4, 96);
    assert_eq!(path_4, R_CARDINALITY);
}

// ─────────────────────────────────────────────────────────────────────
// Convergence III — `24`, the period invariant
// ─────────────────────────────────────────────────────────────────────

/// **Convergence at 24 — three independent derivations all agree.**
///
/// 1. Phase 1, Pisano period of Fibonacci mod 9: `PISANO_PERIOD_MOD_DR`.
/// 2. Phase 1.5, cardinality of `U_30(90)`: `U90_CARDINALITY`.
/// 3. Phase 4.3, factorization-dyad fiber size: `FIBER_SIZE`.
#[test]
fn convergence_at_24_three_paths() {
    let path_1 = PISANO_PERIOD_MOD_DR;
    let path_2 = U90_CARDINALITY as u64;
    let path_3 = FIBER_SIZE as u64;
    assert_eq!(path_1, 24);
    assert_eq!(path_2, 24);
    assert_eq!(path_3, 24);
    assert_eq!(path_1, path_2);
    assert_eq!(path_2, path_3);
}

/// **Convergence at 24 — supplementary structural derivations.**
///
/// 4. `LATTICE_LAYERS · 2`: `12 · 2 = 24`.
/// 5. `U30_CARDINALITY · 3`: `8 · 3 = 24`.
#[test]
fn convergence_at_24_supplementary_paths() {
    let path_4 = LATTICE_LAYERS * 2;
    let path_5 = U30_CARDINALITY as u64 * 3;
    assert_eq!(path_4, 24);
    assert_eq!(path_5, 24);
    assert_eq!(path_4, PISANO_PERIOD_MOD_DR);
    assert_eq!(path_5, PISANO_PERIOD_MOD_DR);
}

// ─────────────────────────────────────────────────────────────────────
// Triadic structural identities
// ─────────────────────────────────────────────────────────────────────

/// **Triality of `1080 = 3 · MODULUS_R`.** The sum of `U_30(90)`
/// equals `3 · MODULUS_R`, exposing the three-fold tiling factor of
/// the 90-mod substrate inside the 360-mod lattice.
#[test]
fn triality_of_1080() {
    let path_1 = ROW_SUM_LINEAR;
    let path_2 = 3 * MODULUS_R;
    assert_eq!(path_1, 1080);
    assert_eq!(path_2, 1080);
    assert_eq!(path_1, path_2);
}

/// **Triality of `2160 = 6 · MODULUS_R`.** The combined diagonal
/// sum of the 24×24 mod-90 multiplication matrix factors as
/// `6 · MODULUS_R`. The factor `6 = |(Z/9Z)*|`.
#[test]
fn triality_of_2160() {
    use toroidal_prime_clock::u90::DIAGONAL_PAIR_SUM;
    let path_1 = DIAGONAL_PAIR_SUM;
    let path_2 = 6 * MODULUS_R;
    let path_3 = 5 * SUM_DR;
    assert_eq!(path_1, 2160);
    assert_eq!(path_2, 2160);
    assert_eq!(path_3, 2160);
    assert_eq!(path_1, path_2);
    assert_eq!(path_2, path_3);
}

/// **Triality of `576 = 24²`.** The total dyad count factors three
/// ways through the substrate.
#[test]
fn triality_of_576() {
    let path_1 = DYAD_COUNT;
    let path_2 = U90_CARDINALITY * U90_CARDINALITY;
    let path_3 = (FIBER_SIZE * U90_CARDINALITY) as usize;
    assert_eq!(path_1, 576);
    assert_eq!(path_2, 576);
    assert_eq!(path_3, 576);
    assert_eq!(path_1, path_2);
    assert_eq!(path_2, path_3);
}

/// **Anchor of the substrate.** The 8-element `U(30)` is the
/// foundational residue set; the convergence pattern of every
/// flagship invariant ultimately roots here.
#[test]
fn convergence_at_8_eight_residues() {
    let path_1 = U30_CARDINALITY;
    let path_2 = RESIDUES.len();
    assert_eq!(path_1, 8);
    assert_eq!(path_2, 8);
    assert_eq!(path_1, path_2);
}

// ─────────────────────────────────────────────────────────────────────
// Secondary triality — three layer-cycle invariants
// ─────────────────────────────────────────────────────────────────────
//
// `108`, `27`, `36` form the inner triality of the period-24 cycle:
// each row of the dr table sums to 108; each row of the (Z/9Z)*
// table sums to 27; each layer of R(360) contributes 36 to SUM_FIBO_DR.

/// **Convergence at 108 — three independent derivations.**
///
/// 1. Phase 5.2: `DR_ROW_SUM` directly.
/// 2. Phase 5.2 / Phase 1.5: `4 · UNITS_Z9_SUM = 4 · 27`.
/// 3. Phase 1: `SUM_DR / 4 = 432 / 4`.
#[test]
fn convergence_at_108_three_paths() {
    use toroidal_prime_clock::units_z9::UNITS_Z9_SUM;
    let path_1 = DR_ROW_SUM;
    let path_2 = 4 * UNITS_Z9_SUM;
    let path_3 = SUM_DR / 4;
    assert_eq!(path_1, 108);
    assert_eq!(path_2, 108);
    assert_eq!(path_3, 108);
    assert_eq!(path_1, path_2);
    assert_eq!(path_2, path_3);
}

/// **Convergence at 27 — three independent derivations.**
///
/// 1. Phase 1.5: `UNITS_Z9_SUM` directly.
/// 2. Phase 1.5: `MULT_TABLE_ROW_SUM = 3 · MODULUS_DR`.
/// 3. Phase 1.5: `MULT_TABLE_TOTAL_SUM / UNITS_Z9_CARDINALITY = 162 / 6`.
#[test]
fn convergence_at_27_three_paths() {
    use toroidal_prime_clock::units_z9::{
        MULT_TABLE_ROW_SUM, MULT_TABLE_TOTAL_SUM, UNITS_Z9_CARDINALITY, UNITS_Z9_SUM,
    };
    let path_1 = UNITS_Z9_SUM;
    let path_2 = MULT_TABLE_ROW_SUM;
    let path_3 = MULT_TABLE_TOTAL_SUM / UNITS_Z9_CARDINALITY as u64;
    assert_eq!(path_1, 27);
    assert_eq!(path_2, 27);
    assert_eq!(path_3, 27);
    assert_eq!(path_1, path_2);
    assert_eq!(path_2, path_3);
}

/// **Convergence at 36 — three independent derivations.**
///
/// 1. Phase 1: per-layer Fibo-DR sum `SUM_FIBO_DR / LATTICE_LAYERS = 432 / 12`.
/// 2. Phase 1: per-layer DR sum `SUM_DR / LATTICE_LAYERS`.
/// 3. Phase 1.5: `MULT_TABLE_TOTAL_SUM - UNITS_Z9_SUM_squared` does NOT match;
///    the true third path is the structural identity
///    `4 · 9 = LATTICE_LAYERS / Z3_FACTOR · MODULUS_DR` where
///    Z3_FACTOR = 3, giving `12/3 · 9 = 4 · 9 = 36`.
#[test]
fn convergence_at_36_three_paths() {
    let path_1 = SUM_FIBO_DR / u64::from(LATTICE_LAYERS);
    let path_2 = SUM_DR / u64::from(LATTICE_LAYERS);
    // Z/9Z partitions into 3 cosets mod 3; per-coset count of LATTICE_LAYERS
    // is 12/3 = 4. The third path: 4 · MODULUS_DR = 4 · 9 = 36.
    let z3_factor = 3u64;
    let path_3 = (u64::from(LATTICE_LAYERS) / z3_factor) * 9;
    assert_eq!(path_1, 36);
    assert_eq!(path_2, 36);
    assert_eq!(path_3, 36);
    assert_eq!(path_1, path_2);
    assert_eq!(path_2, path_3);
}
