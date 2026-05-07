//! # Perfect squares of `U(30)` members modulo 30 and modulo 9
//!
//! For every `n ∈ U(30) = {1, 7, 11, 13, 17, 19, 23, 29}`, the
//! square `n²` lands in exactly one of two residues modulo 30:
//!
//! ```text
//!   n² mod 30 ∈ {1, 19}
//! ```
//!
//! and exactly one of three residues modulo 9 (i.e. one of three
//! digital roots):
//!
//! ```text
//!   dr(n²) ∈ {1, 4, 7}
//! ```
//!
//! Both reductions are exhaustive over the eight residues of U(30)
//! and decidable at compile time.
//!
//! ## Why this matters
//!
//! These constraints are dual to the twin and Sophie Germain
//! constraints in [`crate::twin_prime_residues`] and
//! [`crate::sophie_germain_residues`]. Together they pin which
//! residue classes can hold which kind of structured prime
//! configuration. Squares of primes >5 are forced into 2/30 = 6.7%
//! of integers and 3/9 = 33% of digital roots.
//!
//! ## Anchor
//!
//! Computed at compile time from
//! [`crate::parameters::MODULUS_U`] = 30,
//! [`crate::parameters::MODULUS_DR`] = 9, and
//! [`crate::digital_root::digital_root`].

use crate::digital_root::digital_root;
use crate::parameters::{MODULUS_DR, MODULUS_U};
use crate::u30::{RESIDUES, U30_CARDINALITY};

/// Distinct mod-30 residues taken by `n²` for `n ∈ U(30)`. Equals
/// `{1, 19}`. Exactly two values, each hit by 4 of the 8 U(30)
/// members.
pub const SQUARE_MOD30_RESIDUES: [u64; 2] = compute_square_mod30();

const fn compute_square_mod30() -> [u64; 2] {
    let mut out = [0u64; 2];
    let mut idx = 0usize;
    let mut i = 0usize;
    while i < U30_CARDINALITY {
        let sq = (RESIDUES[i] * RESIDUES[i]) % MODULUS_U;
        // Insert if not already present (small dedup).
        let mut seen = false;
        let mut j = 0usize;
        while j < idx {
            if out[j] == sq {
                seen = true;
            }
            j += 1;
        }
        if !seen {
            if idx < 2 {
                out[idx] = sq;
                idx += 1;
            }
        }
        i += 1;
    }
    // Sort the two values ascending.
    if out[0] > out[1] {
        let t = out[0];
        out[0] = out[1];
        out[1] = t;
    }
    out
}

const _: () = {
    let expected: [u64; 2] = [1, 19];
    assert!(
        SQUARE_MOD30_RESIDUES[0] == expected[0]
            && SQUARE_MOD30_RESIDUES[1] == expected[1],
        "n² mod 30 residues for n ∈ U(30) must equal {1, 19}"
    );
};

/// Distinct digital roots of `n²` for `n ∈ U(30)`. Equals
/// `{1, 4, 7}` — exactly the squares of `(Z/9Z)*` ordered cyclically
/// from generator 2: `2² = 4`, `4² = 7`, `8² = 1` (each mod 9).
pub const SQUARE_DR_VALUES: [u64; 3] = compute_square_dr();

const fn compute_square_dr() -> [u64; 3] {
    let mut out = [0u64; 3];
    let mut idx = 0usize;
    let mut i = 0usize;
    while i < U30_CARDINALITY {
        let sq = RESIDUES[i] * RESIDUES[i];
        let dr = digital_root(sq);
        // Dedup.
        let mut seen = false;
        let mut j = 0usize;
        while j < idx {
            if out[j] == dr {
                seen = true;
            }
            j += 1;
        }
        if !seen {
            if idx < 3 {
                out[idx] = dr;
                idx += 1;
            }
        }
        i += 1;
    }
    // Sort ascending.
    let mut a = 0;
    while a < 3 {
        let mut b = a + 1;
        while b < 3 {
            if out[a] > out[b] {
                let t = out[a];
                out[a] = out[b];
                out[b] = t;
            }
            b += 1;
        }
        a += 1;
    }
    out
}

const _: () = {
    let expected: [u64; 3] = [1, 4, 7];
    let mut i = 0;
    while i < 3 {
        assert!(
            SQUARE_DR_VALUES[i] == expected[i],
            "dr(n²) for n ∈ U(30) must give {1, 4, 7}"
        );
        i += 1;
    }
};

/// **Coset identity.** `{1, 4, 7}` is the multiplication-by-3 coset
/// of `1` modulo 9, equivalently the squares of `(Z/9Z)*`. The
/// other two cosets are `{2, 5, 8}` (non-square units) and
/// `{3, 6, 9}` (non-units = multiples of 3 mod 9).
pub const SQUARE_DR_COSET: [u64; 3] = SQUARE_DR_VALUES;
pub const NON_SQUARE_UNIT_DR_COSET: [u64; 3] = [2, 5, 8];
pub const NON_UNIT_DR_COSET: [u64; 3] = [3, 6, 9];

const _: () = {
    // The three cosets together cover all of {1, …, 9}, partitioning Z/9Z.
    let expected_union: [u64; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut found = [false; 9];
    let mut i = 0;
    while i < 3 {
        let v = SQUARE_DR_COSET[i];
        if v >= 1 && v <= 9 {
            found[(v - 1) as usize] = true;
        }
        let u = NON_SQUARE_UNIT_DR_COSET[i];
        if u >= 1 && u <= 9 {
            found[(u - 1) as usize] = true;
        }
        let w = NON_UNIT_DR_COSET[i];
        if w >= 1 && w <= 9 {
            found[(w - 1) as usize] = true;
        }
        i += 1;
    }
    let mut j = 0;
    while j < 9 {
        assert!(found[j], "DR cosets do not partition {1, …, 9}");
        let _ = expected_union[j];
        j += 1;
    }
};

/// **Multiplicities.** Each value of `SQUARE_MOD30_RESIDUES`
/// (`{1, 19}`) is hit by exactly 4 of the 8 U(30) members.
pub const SQUARE_MOD30_MULTIPLICITY: u64 = 4;

const _: () = {
    let mut hits_1 = 0u64;
    let mut hits_19 = 0u64;
    let mut i = 0usize;
    while i < U30_CARDINALITY {
        let sq = (RESIDUES[i] * RESIDUES[i]) % MODULUS_U;
        if sq == 1 {
            hits_1 += 1;
        }
        if sq == 19 {
            hits_19 += 1;
        }
        i += 1;
    }
    assert!(
        hits_1 == SQUARE_MOD30_MULTIPLICITY,
        "1 should be hit by 4 U(30) squares"
    );
    assert!(
        hits_19 == SQUARE_MOD30_MULTIPLICITY,
        "19 should be hit by 4 U(30) squares"
    );
};

/// Predicate: `n mod 30 ∈ {1, 19}`. A necessary condition for `n`
/// to be the square of a U(30) member.
#[must_use]
pub const fn is_square_residue_mod30(n: u64) -> bool {
    let r = n % MODULUS_U;
    r == 1 || r == 19
}

/// Predicate: `dr(n) ∈ {1, 4, 7}`. A necessary condition for `n` to
/// be the square of a U(30) member.
#[must_use]
pub const fn has_square_dr(n: u64) -> bool {
    let d = digital_root(n);
    d == 1 || d == 4 || d == 7
}

const _: () = {
    // For every U(30) member, both predicates accept its square.
    let mut i = 0usize;
    while i < U30_CARDINALITY {
        let sq = RESIDUES[i] * RESIDUES[i];
        assert!(
            is_square_residue_mod30(sq),
            "square of U(30) element fails mod-30 predicate"
        );
        assert!(
            has_square_dr(sq),
            "square of U(30) element fails dr predicate"
        );
        i += 1;
    }
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
    use super::*;

    #[test]
    fn square_mod30_canonical() {
        assert_eq!(SQUARE_MOD30_RESIDUES, [1, 19]);
    }

    #[test]
    fn square_dr_canonical() {
        assert_eq!(SQUARE_DR_VALUES, [1, 4, 7]);
    }

    #[test]
    fn three_dr_cosets_partition_z9() {
        assert_eq!(SQUARE_DR_COSET, [1, 4, 7]);
        assert_eq!(NON_SQUARE_UNIT_DR_COSET, [2, 5, 8]);
        assert_eq!(NON_UNIT_DR_COSET, [3, 6, 9]);
        // Union = {1, …, 9}, disjoint.
        let mut union = [false; 10];
        for &v in SQUARE_DR_COSET
            .iter()
            .chain(NON_SQUARE_UNIT_DR_COSET.iter())
            .chain(NON_UNIT_DR_COSET.iter())
        {
            assert!(!union[v as usize], "duplicate dr value {v} across cosets");
            union[v as usize] = true;
        }
        for v in 1..=9 {
            assert!(union[v]);
        }
    }

    #[test]
    fn each_square_residue_hit_4_times() {
        let mut hits_1 = 0u64;
        let mut hits_19 = 0u64;
        for &n in &RESIDUES {
            let sq = (n * n) % MODULUS_U;
            if sq == 1 {
                hits_1 += 1;
            } else if sq == 19 {
                hits_19 += 1;
            } else {
                panic!("square of {n} mod 30 = {sq} not in {{1, 19}}");
            }
        }
        assert_eq!(hits_1, 4);
        assert_eq!(hits_19, 4);
    }

    #[test]
    fn predicates_agree_on_known_squares() {
        // Squares of primes 7 to 29: 49, 121, 169, 289, 361, 529, 841.
        let squares = [49u64, 121, 169, 289, 361, 529, 841];
        for &sq in &squares {
            assert!(is_square_residue_mod30(sq));
            assert!(has_square_dr(sq));
        }
    }

    #[test]
    fn modulus_dr_is_9() {
        // Sanity: this module's claims rest on dr modulus = 9.
        assert_eq!(MODULUS_DR, 9);
    }
}
