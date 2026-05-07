//! # Prelude — three-tier triality of the Prime Clock crate
//!
//! Every public export is grouped into exactly one of three tiers:
//!
//! ```text
//!   atoms       — irreducible parameters and their immediate algebra
//!   substrate   — derived structural objects (sets, tables, lattices)
//!   invariants  — numerical anchors and predicates with structural meaning
//! ```
//!
//! This three-fold partition mirrors the substrate's own triality:
//! the `Z/9Z` partition `{1,4,7} ∪ {2,5,8} ∪ {3,6,9}` (mod 3), the
//! three Phase 3 prime-structure constraints (twin / Sophie Germain
//! / Siamese), and the three diagonals of the 24×24 mod-90
//! multiplication matrix (principal / secondary / combined).
//!
//! ## Usage
//!
//! ```ignore
//! use toroidal_prime_clock::prelude::*;
//!
//! // atoms
//! let r = MODULUS_U;                       // 30
//! let g = gcd(42, 30);                     // 6
//!
//! // substrate
//! let u30 = U30_RESIDUES;                  // [1, 7, 11, …, 29]
//! let dr = DR_CYCLE;                       // 24-element period
//!
//! // invariants
//! assert_eq!(SUM_DR, 432);
//! assert_eq!(R_CARDINALITY, 96);
//! ```

pub use atoms::*;
pub use invariants::*;
pub use substrate::*;

/// **Atoms** — the irreducible parameters and their immediate algebraic
/// helpers. Two parameters in (`MODULUS_U`, `NUMERAL_BASE`); from these
/// `MODULUS_DR` is derived as `NUMERAL_BASE − 1 = 9`.
pub mod atoms {
    pub use crate::arithmetic::{euler_phi, gcd, lcm};
    pub use crate::digital_root::digital_root;
    pub use crate::fibonacci::{dr_fib, fib, fib_mod, fib_mod_dr, F49};
    pub use crate::parameters::{MODULUS_DR, MODULUS_U, NUMERAL_BASE};
}

/// **Substrate** — derived structural objects: residue sets,
/// multiplication tables, lattices, sequences. Every entry computed
/// at compile time from `atoms`.
pub mod substrate {
    pub use crate::appendix_a::ROWS as APPENDIX_A_ROWS;
    pub use crate::factorization_dyads::DYADS_TABLE;
    pub use crate::period24_dr_sequences::DR_SEQUENCES;
    pub use crate::r360::ELEMENTS as R360_ELEMENTS;
    pub use crate::u30::RESIDUES as U30_RESIDUES;
    pub use crate::u90::{DR_CYCLE, RESIDUES_U90 as U90_RESIDUES};
    pub use crate::units_z9::{MULT_TABLE_Z9, POWERS_OF_2_MOD_9, UNITS_Z9};
}

/// **Invariants** — numerical anchors, predicates, and computed
/// functions that capture the structural content of the substrate.
/// These are what the rest of the world cares about: cardinalities,
/// sums, the prime-counting function, and the residue predicates.
pub mod invariants {
    // Numerical anchors —
    pub use crate::appendix_a::{SUM_DR, SUM_FIBO_DR};
    pub use crate::factorization_dyads::{DYAD_COUNT, FIBER_SIZE};
    pub use crate::parameters::{
        LATTICE_LAYERS, MODULUS_R, PISANO_PERIOD_MOD_DR, R_CARDINALITY,
        U30_CARDINALITY,
    };
    pub use crate::period24_dr_sequences::{DR_ROW_SUM, DR_TABLE_TOTAL};
    pub use crate::triangular_numbers::{
        ANCHOR_24, ANCHOR_30, PYRAMID_PRODUCT, SQUARE_DIFFERENCE_30_24,
        SQUARE_MINUS_ONE_AT_24, TRIANGULAR_AT_24TH, TRIANGULAR_AT_30TH,
        TRIANGULAR_AT_THOUSANDTH_PRIME,
    };
    pub use crate::u90::{
        DIAGONAL_PAIR_SUM, DIAGONAL_SUM, MODULUS_U90, ROW_SUM_LINEAR,
        SECONDARY_DIAGONAL_SUM, SECONDARY_DIAGONAL_VALUES,
        SQUARE_RESIDUE_VALUES, U90_CARDINALITY,
    };
    pub use crate::units_z9::{
        MULT_TABLE_ROW_SUM, MULT_TABLE_TOTAL_SUM, UNITS_Z9_CARDINALITY,
        UNITS_Z9_SUM,
    };

    // Functions —
    pub use crate::index_to_residue::{index_of_u30_member, nth_u30_member};
    pub use crate::triangular_numbers::triangular;

    // Predicates —
    pub use crate::perfect_squares_mod30::{has_square_dr, is_square_residue_mod30};
    pub use crate::siamese_prime_pattern::{
        is_siamese_generator_candidate, is_siamese_generator_via_mod6,
    };
    pub use crate::sophie_germain_residues::is_sophie_germain_candidate;
    pub use crate::twin_prime_residues::is_twin_prime_candidate_lower;
}

/// Sieve and `π(N)` — gated behind the `alloc` feature.
#[cfg(feature = "alloc")]
pub mod sieve_api {
    pub use crate::pi_function::{identity_holds, pi_additive, pi_structural};
    pub use crate::replicant_factorization::{
        duplicate_factorization_count, replicant_count_distinct,
        replicant_products,
    };
    pub use crate::sieve::{pi_via_u30_sieve, primes_in_u30};
}
