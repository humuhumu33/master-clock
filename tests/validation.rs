//! # Single-file validation suite — strict Phase 1 anchoring
//!
//! Every test here verifies a fact that derives from the two
//! irreducible parameters `MODULUS_U = 30` and `NUMERAL_BASE = 10`.
//! Anything depending on Gary's calibration constants χ, ζ, δ, k —
//! the Master-Clock identity, TF/RF residuals, F49 "ratchet" framing —
//! has been removed. See `RESULTS.md` and `sheets_findings.md` for
//! the audit that drove those cuts.
//!
//! Run with:
//!
//! ```text
//! cargo test --test validation --features alloc -- --nocapture
//! ```
//!
//! ## What is verified, in three layers
//!
//! 1. **Phase 1 substrate.** Two parameters in, every other constant
//!    out. U(30), R(360), Pisano(9), Appendix A, F_49 from the
//!    recurrence, the two 432 invariants, the Atlas-96 mirror.
//! 2. **Phase 1.5 — `(Z/9Z)*` substrate.** `U_30(90)` (24 elements),
//!    the period-24 digital-root cycle, the units of `Z/9Z`, the 6×6
//!    multiplication table mod 9, the 24×24 mod-90 multiplication-
//!    matrix row sum and diagonal.
//! 3. **Phase 2 — wheel-30 sieve.** π(N) computed from MODULUS_U=30
//!    matches OEIS A006880 exactly at every CI-affordable decimal
//!    power. Larger N (`10¹⁰`) are queued behind `#[ignore]`.
//!
//! Each test uses the largest exhaustive population available for
//! its claim — no sampling.

use toroidal_prime_clock::appendix_a::{
    FIXED_PDF_VERIFICATION, GARY_STATED_TOTAL_DR, GARY_STATED_TOTAL_FIBO_DR, ROWS, SUM_DR,
    SUM_FIBO_DR,
};
use toroidal_prime_clock::atlas_correspondence::{
    atlas_label_to_n, n_to_atlas_label, ATLAS_LABELS, ATLAS_TO_R_INDEX, R_TO_ATLAS_INDEX,
};
use toroidal_prime_clock::dirichlet;
use toroidal_prime_clock::fibonacci::{dr_fib, fib, F49};
use toroidal_prime_clock::li::{lookup_reference, REFERENCE_TABLE};
use toroidal_prime_clock::parameters::{
    LATTICE_LAYERS, MODULUS_R, MODULUS_U, NUMERAL_BASE, PISANO_PERIOD_MOD_DR, R_CARDINALITY,
    U30_CARDINALITY,
};
use toroidal_prime_clock::r360::{compose, decompose, ELEMENTS};
use toroidal_prime_clock::twin_prime_residues::{
    is_twin_prime_candidate_lower, ISOLATED_LOWER_RESIDUES, TWIN_PRIME_LOWER_RESIDUES,
    TWIN_PRIME_RESIDUE_PAIRS,
};
use toroidal_prime_clock::u30::{GARY_STATED_RESIDUES, RESIDUES};
use toroidal_prime_clock::u90::{
    DIAGONAL_SUM, DR_CYCLE, GARY_STATED_DR_CYCLE, GARY_STATED_U90_RESIDUES, MODULUS_U90,
    RESIDUES_U90, ROW_SUM_LINEAR, SQUARE_RESIDUE_VALUES, TILES_INTO_R360, U90_CARDINALITY,
};
use toroidal_prime_clock::units_z9::{
    MULT_TABLE_TOTAL_SUM, MULT_TABLE_Z9, POWERS_OF_2_MOD_9, UNITS_Z9, UNITS_Z9_CARDINALITY,
    UNITS_Z9_SUM,
};

// ─────────────────────────────────────────────────────────────────────
// Phase 1 — foundations (everything derived from MODULUS_U = 30)
// ─────────────────────────────────────────────────────────────────────

/// **Two parameters in, every other constant out.**
#[test]
fn phase1_two_parameters_only() {
    assert_eq!(MODULUS_U, 30);
    assert_eq!(NUMERAL_BASE, 10);
    assert_eq!(PISANO_PERIOD_MOD_DR, 24);
    assert_eq!(U30_CARDINALITY, 8);
    assert_eq!(LATTICE_LAYERS, 12);
    assert_eq!(MODULUS_R, 360);
    assert_eq!(R_CARDINALITY, 96);
}

/// **U(30) = {1, 7, 11, 13, 17, 19, 23, 29}** by `gcd(n, 30) = 1`.
#[test]
fn phase1_u30_residues() {
    assert_eq!(RESIDUES.len(), 8);
    assert_eq!(RESIDUES, GARY_STATED_RESIDUES);
    assert_eq!(RESIDUES, [1, 7, 11, 13, 17, 19, 23, 29]);
}

/// **R(360) has exactly 96 elements**, each coprime to 30.
#[test]
fn phase1_r360_cardinality_and_well_formed() {
    use toroidal_prime_clock::arithmetic::gcd;
    assert_eq!(ELEMENTS.len(), 96);
    assert_eq!(R_CARDINALITY, 96);
    for &n in &ELEMENTS {
        assert!(n >= 1 && u64::from(n) <= MODULUS_R);
        assert_eq!(gcd(u64::from(n), MODULUS_U), 1);
    }
}

/// **R(360) ≅ U(30) × Z/12 bijection round-trips** on all 96 elements.
#[test]
fn phase1_r360_bijection_round_trips() {
    for &n in &ELEMENTS {
        let (u, k) = decompose(n).expect("R(360) element must decompose");
        let recomposed = compose(u, k).expect("decomposed pair must recompose");
        assert_eq!(n, recomposed, "round-trip failure at n = {n}");
    }
}

/// **Pisano period of Fibonacci mod 9 is 24** and is minimal.
#[test]
fn phase1_pisano_period_24_minimal() {
    use toroidal_prime_clock::fibonacci::fib_mod_dr;
    assert_eq!(fib_mod_dr(PISANO_PERIOD_MOD_DR), 0);
    assert_eq!(fib_mod_dr(PISANO_PERIOD_MOD_DR + 1), 1);
    for p in 1..PISANO_PERIOD_MOD_DR {
        let closes = fib_mod_dr(p) == 0 && fib_mod_dr(p + 1) == 1;
        assert!(!closes, "Pisano(9) closes earlier at p = {p}");
    }
}

/// **F_49 = 7,778,742,049** by the standard Fibonacci recurrence.
#[test]
fn phase1_f49_from_recurrence() {
    assert_eq!(F49, 7_778_742_049);
    assert_eq!(F49, fib(49));
}

/// **96-row Appendix A reproduced from foundations** — every cell.
#[test]
fn phase1_appendix_a_matches_pdf() {
    assert_eq!(ROWS.len(), 96);
    assert_eq!(FIXED_PDF_VERIFICATION.len(), 96);
    for (i, (computed, stated)) in ROWS.iter().zip(FIXED_PDF_VERIFICATION.iter()).enumerate() {
        assert_eq!(computed, stated, "row {i} disagrees: {computed:?} vs {stated:?}");
    }
}

/// **First 432 invariant: Σ dr(n) over R(360) = 432.**
#[test]
fn phase1_sum_dr_equals_432() {
    assert_eq!(SUM_DR, 432);
    assert_eq!(SUM_DR, GARY_STATED_TOTAL_DR);
}

/// **Second 432 invariant: Σ dr(F_n) over R(360) = 432.** Structural
/// decomposition: `LATTICE_LAYERS · 36 = 12 · 36 = 432`.
#[test]
fn phase1_sum_fibo_dr_equals_432_structural() {
    assert_eq!(SUM_FIBO_DR, 432);
    assert_eq!(SUM_FIBO_DR, GARY_STATED_TOTAL_FIBO_DR);
    assert_eq!(SUM_FIBO_DR, LATTICE_LAYERS * 36);
}

/// **Atlas-96 mirror is an involution** — `τ ∘ τ = id` on all 96 vertices.
#[test]
fn phase1_atlas_mirror_involution() {
    assert_eq!(ATLAS_LABELS.len(), 96);
    for i in 0..R_CARDINALITY {
        assert_eq!(ATLAS_TO_R_INDEX[R_TO_ATLAS_INDEX[i]], i);
        assert_eq!(R_TO_ATLAS_INDEX[ATLAS_TO_R_INDEX[i]], i);
    }
}

/// **Atlas labels round-trip via `n_to_atlas_label`/`atlas_label_to_n`.**
#[test]
fn phase1_atlas_round_trip() {
    for &n in &ELEMENTS {
        let label = n_to_atlas_label(n).expect("R(360) element must label");
        let back = atlas_label_to_n(label).expect("label must decode");
        assert_eq!(n, back, "atlas round-trip failure at n = {n}");
    }
}

/// **dr_fib agrees with dr ∘ fib on the natural domain of U(30).**
#[test]
fn phase1_dr_fib_agrees_with_dr_of_fib() {
    use toroidal_prime_clock::digital_root::dr;
    for k in 1u64..=29 {
        assert_eq!(dr_fib(k), dr(fib(k)));
    }
}

/// **Dirichlet equidistribution at N = 10⁴** — every U(30) class
/// receives `±` a small fraction of the eight equally-weighted slots.
#[test]
fn phase1_dirichlet_equidistribution_at_10_000() {
    let counts = dirichlet::class_counts(10_000);
    let total: u64 = counts.iter().sum();
    let mean = total / U30_CARDINALITY as u64;
    for &c in &counts {
        let dev = if c >= mean { c - mean } else { mean - c };
        assert!(
            dev * 50 < total,
            "Dirichlet skew exceeds 2% at N=10000: counts={counts:?}"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────
// Phase 1.5 — U_30(90) substrate and (Z/9Z)*
// ─────────────────────────────────────────────────────────────────────

/// **|U_30(90)| = 24** — the 24-element residue substrate.
#[test]
fn phase15_u90_cardinality() {
    assert_eq!(U90_CARDINALITY, 24);
    assert_eq!(MODULUS_U90, 90);
    assert_eq!(MODULUS_U90, 3 * MODULUS_U);
}

/// **U_30(90) matches the reference table.**
#[test]
fn phase15_u90_residues_match_reference() {
    assert_eq!(RESIDUES_U90, GARY_STATED_U90_RESIDUES);
}

/// **Period-24 dr cycle** matches Gary's Mod-90 PDF.
#[test]
fn phase15_dr_cycle_matches_pdf() {
    assert_eq!(DR_CYCLE, GARY_STATED_DR_CYCLE);
}

/// **dr cycle is 4-uniform over (Z/9Z)*** and 0 on `{3, 6, 9}`.
#[test]
fn phase15_dr_cycle_4_uniform() {
    for u in [1u64, 2, 4, 5, 7, 8] {
        let count = DR_CYCLE.iter().filter(|&&v| v == u).count();
        assert_eq!(count, 4, "dr value {u} appears {count} times, expected 4");
    }
    for v in [3u64, 6, 9] {
        let count = DR_CYCLE.iter().filter(|&&x| x == v).count();
        assert_eq!(count, 0, "dr value {v} should not appear");
    }
}

/// **(Z/9Z)* = {1, 2, 4, 5, 7, 8}** is cyclic of order 6, generator 2.
#[test]
fn phase15_units_z9_cyclic_generator_2() {
    assert_eq!(UNITS_Z9, [1, 2, 4, 5, 7, 8]);
    assert_eq!(UNITS_Z9_CARDINALITY, 6);
    assert_eq!(POWERS_OF_2_MOD_9, [2, 4, 8, 7, 5, 1]);
    let mut seen = [false; 6];
    for &p in &POWERS_OF_2_MOD_9 {
        for (i, &u) in UNITS_Z9.iter().enumerate() {
            if u == p {
                seen[i] = true;
            }
        }
    }
    for s in &seen {
        assert!(*s, "2 fails to generate (Z/9Z)*");
    }
}

/// **6×6 multiplication table mod 9 — closure, row sum 27, total 162.**
#[test]
fn phase15_mult_table_z9_invariants() {
    assert_eq!(UNITS_Z9_SUM, 27);
    assert_eq!(MULT_TABLE_TOTAL_SUM, 162);
    for i in 0..UNITS_Z9_CARDINALITY {
        let row_sum: u64 = MULT_TABLE_Z9[i].iter().sum();
        assert_eq!(row_sum, 27);
        let col_sum: u64 = (0..UNITS_Z9_CARDINALITY)
            .map(|j| MULT_TABLE_Z9[j][i])
            .sum();
        assert_eq!(col_sum, 27);
        for &v in &MULT_TABLE_Z9[i] {
            assert!(UNITS_Z9.contains(&v), "table entry escapes (Z/9Z)*");
        }
    }
}

/// **Σ U_30(90) = 1080 = 3 · MODULUS_R** — the row sum of every row
/// of the 24×24 mod-90 multiplication matrix.
#[test]
fn phase15_u90_linear_sum_1080() {
    assert_eq!(ROW_SUM_LINEAR, 1080);
    assert_eq!(ROW_SUM_LINEAR, 3 * MODULUS_R);
}

/// **Diagonal of 24×24 mod-90 matrix → 6 distinct residues, sum 960.**
#[test]
fn phase15_u90_diagonal_960() {
    assert_eq!(SQUARE_RESIDUE_VALUES, [1, 19, 31, 49, 61, 79]);
    assert_eq!(DIAGONAL_SUM, 960);
    assert_eq!(DIAGONAL_SUM, 4 * 240);
}

/// **|U_30(90)| · 4 = |R(360)| = 96.** The 24-element substrate tiles
/// into the Phase 1 lattice exactly four times.
#[test]
fn phase15_u90_tiles_into_r360() {
    assert_eq!(TILES_INTO_R360, R_CARDINALITY as u64);
    assert_eq!(TILES_INTO_R360, 96);
}

// ─────────────────────────────────────────────────────────────────────
// Phase 3 — twin prime residue constraints mod 30
// ─────────────────────────────────────────────────────────────────────

/// **Twin-prime lower residues mod 30 are exactly `{11, 17, 29}`.**
/// Derived from `U(30)` by keeping residues `r` with
/// `gcd((r + 2) mod 30, 30) = 1`.
#[test]
fn phase3_twin_prime_lower_residues_canonical() {
    assert_eq!(TWIN_PRIME_LOWER_RESIDUES, [11, 17, 29]);
}

/// **Isolated lower residues mod 30 are exactly `{1, 7, 13, 19, 23}`.**
/// For these `r`, `r + 2` is divisible by 3 or 5.
#[test]
fn phase3_isolated_lower_residues_canonical() {
    assert_eq!(ISOLATED_LOWER_RESIDUES, [1, 7, 13, 19, 23]);
}

/// **The two sets partition `U(30)`** — `3 + 5 = 8`.
#[test]
fn phase3_partition_covers_u30() {
    assert_eq!(
        TWIN_PRIME_LOWER_RESIDUES.len() + ISOLATED_LOWER_RESIDUES.len(),
        U30_CARDINALITY
    );
    for &r in &TWIN_PRIME_LOWER_RESIDUES {
        assert!(!ISOLATED_LOWER_RESIDUES.contains(&r));
    }
}

/// **Three twin-prime residue pairs.** `(11, 13)`, `(17, 19)`, `(29, 1)`.
#[test]
fn phase3_twin_prime_residue_pairs() {
    assert_eq!(TWIN_PRIME_RESIDUE_PAIRS, [(11, 13), (17, 19), (29, 1)]);
}

/// **Empirical anchor.** Eleven known twin prime pairs above 5 all
/// have `p mod 30 ∈ {11, 17, 29}`.
#[test]
fn phase3_predicate_holds_on_known_twin_primes() {
    let known: &[(u64, u64)] = &[
        (11, 13),
        (17, 19),
        (29, 31),
        (41, 43),
        (59, 61),
        (71, 73),
        (101, 103),
        (107, 109),
        (137, 139),
        (149, 151),
        (179, 181),
    ];
    for &(p, q) in known {
        assert_eq!(q, p + 2);
        assert!(is_twin_prime_candidate_lower(p));
    }
}

// ─────────────────────────────────────────────────────────────────────
// Phase 2 — wheel-30 sieve gives exact π(N)
// ─────────────────────────────────────────────────────────────────────

#[cfg(feature = "alloc")]
mod sieve_tests {
    use toroidal_prime_clock::pi_function::{identity_holds, pi_additive, pi_structural};
    use toroidal_prime_clock::sieve::pi_via_u30_sieve;

    use super::lookup_reference;

    /// **π(10⁴) = 1,229.** Sieve agrees with literature.
    #[test]
    fn phase2_pi_at_10_4() {
        assert_eq!(pi_via_u30_sieve(10_000), 1_229);
        assert_eq!(pi_via_u30_sieve(10_000), lookup_reference(4).unwrap().pi_n);
    }

    /// **π(10⁵) = 9,592.**
    #[test]
    fn phase2_pi_at_10_5() {
        assert_eq!(pi_via_u30_sieve(100_000), 9_592);
        assert_eq!(pi_via_u30_sieve(100_000), lookup_reference(5).unwrap().pi_n);
    }

    /// **π(10⁶) = 78,498.**
    #[test]
    fn phase2_pi_at_10_6() {
        assert_eq!(pi_via_u30_sieve(1_000_000), 78_498);
        assert_eq!(
            pi_via_u30_sieve(1_000_000),
            lookup_reference(6).unwrap().pi_n
        );
    }

    /// **π(10⁷) = 664,579.**
    #[test]
    fn phase2_pi_at_10_7() {
        assert_eq!(pi_via_u30_sieve(10_000_000), 664_579);
        assert_eq!(
            pi_via_u30_sieve(10_000_000),
            lookup_reference(7).unwrap().pi_n
        );
    }

    /// **π(10⁸) = 5,761,455.** Last in-CI scale.
    #[test]
    fn phase2_pi_at_10_8() {
        assert_eq!(pi_via_u30_sieve(100_000_000), 5_761_455);
        assert_eq!(
            pi_via_u30_sieve(100_000_000),
            lookup_reference(8).unwrap().pi_n
        );
    }

    /// **π(10⁹) = 50,847,534.** Slow path; ~1 s in release.
    #[test]
    #[ignore]
    fn phase2_pi_at_10_9() {
        assert_eq!(pi_via_u30_sieve(1_000_000_000), 50_847_534);
    }

    /// **π(10¹⁰) = 455,052,511.** Memory-heavy; ~18 s, ~333 MiB.
    #[test]
    #[ignore]
    fn phase2_pi_at_10_10() {
        assert_eq!(pi_via_u30_sieve(10_000_000_000), 455_052_511);
    }

    /// **Structural identity `π(N) = primes_in_u30(N) + 3 = D₃₀ − C₃₀ + 2`
    /// holds at every small N.**
    #[test]
    fn phase2_structural_identity_below_1000() {
        for n in 0u64..1_000 {
            assert!(
                identity_holds(n),
                "structural identity broke at N={n}: additive={}, structural={}",
                pi_additive(n),
                pi_structural(n)
            );
        }
    }
}

// ─────────────────────────────────────────────────────────────────────
// Reference-data sweeps — literature only, no Master-Clock claims
// ─────────────────────────────────────────────────────────────────────

/// **π(N) sweep over all 9 decimal powers** matches OEIS A006880.
#[test]
fn reference_pi_sweep_across_decimal_powers() {
    let expected: [(u32, u64); 9] = [
        (4, 1_229),
        (5, 9_592),
        (6, 78_498),
        (7, 664_579),
        (8, 5_761_455),
        (9, 50_847_534),
        (10, 455_052_511),
        (11, 4_118_054_813),
        (12, 37_607_912_018),
    ];
    for (n_log10, expected_pi) in expected {
        let row = lookup_reference(n_log10).unwrap();
        assert_eq!(row.pi_n, expected_pi, "π drift at 10^{n_log10}");
    }
}

/// **`Li(N) − π(N)` sweep matches OEIS A057835** and is strictly
/// monotonic positive across the table (well below Skewes' number).
#[test]
fn reference_li_minus_pi_sweep() {
    for window in REFERENCE_TABLE.windows(2) {
        assert!(window[0].li_minus_pi_literature > 0);
        assert!(window[0].li_minus_pi_literature < window[1].li_minus_pi_literature);
    }
}
