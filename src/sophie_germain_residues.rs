//! # Sophie Germain residue constraints modulo 30
//!
//! A **Sophie Germain prime** is a prime `p` such that `2p + 1` is
//! also prime. The smaller prime is called the Sophie Germain prime;
//! the larger `2p + 1` is the **safe prime**.
//!
//! For `p > 5`, both `p` and `2p + 1` are coprime to 30, hence both
//! lie in `U(30) = {1, 7, 11, 13, 17, 19, 23, 29}`. Among the eight
//! residues, exactly three satisfy `(2r + 1) mod 30` is also coprime
//! to 30:
//!
//! ```text
//!   r ∈ {11, 23, 29}    (Sophie Germain lower-residue set)
//! ```
//!
//! The remaining five residues force `2p + 1` divisible by 3 or 5,
//! hence composite.
//!
//! ## Necessary, not sufficient
//!
//! This is a **necessary** condition on Sophie Germain primes — it
//! does not certify primality. The wheel-30 sieve in [`crate::sieve`]
//! still does the actual primality test; this module rules out 5/8
//! of all candidates before any test runs.
//!
//! ## Anchor
//!
//! Every constant computed at compile time from
//! [`crate::parameters::MODULUS_U`] = 30 via [`crate::arithmetic::gcd`]
//! and [`crate::u30::RESIDUES`].

use crate::arithmetic::gcd;
use crate::parameters::MODULUS_U;
use crate::u30::{RESIDUES, U30_CARDINALITY};

/// The three residues mod 30 that can be Sophie Germain primes for
/// `p > 5`. Computed at compile time by enumerating `RESIDUES` and
/// keeping those `r` where `gcd((2r + 1) mod 30, 30) = 1`.
pub const SOPHIE_GERMAIN_LOWER_RESIDUES: [u64; 3] = compute_sg_lower();

const fn compute_sg_lower() -> [u64; 3] {
    let mut out = [0u64; 3];
    let mut idx = 0usize;
    let mut i = 0usize;
    while i < U30_CARDINALITY {
        let r = RESIDUES[i];
        let s = (2 * r + 1) % MODULUS_U;
        if gcd(s, MODULUS_U) == 1 {
            out[idx] = r;
            idx += 1;
        }
        i += 1;
    }
    out
}

const _: () = {
    let expected: [u64; 3] = [11, 23, 29];
    let mut i = 0;
    while i < 3 {
        assert!(
            SOPHIE_GERMAIN_LOWER_RESIDUES[i] == expected[i],
            "Sophie Germain lower-residue derivation drift"
        );
        i += 1;
    }
};

/// The five residues mod 30 that **cannot** be Sophie Germain primes
/// (for `p > 5`). For these `r`, `2r + 1` is divisible by 3 or 5.
pub const SG_ISOLATED_RESIDUES: [u64; 5] = compute_sg_isolated();

const fn compute_sg_isolated() -> [u64; 5] {
    let mut out = [0u64; 5];
    let mut idx = 0usize;
    let mut i = 0usize;
    while i < U30_CARDINALITY {
        let r = RESIDUES[i];
        let s = (2 * r + 1) % MODULUS_U;
        if gcd(s, MODULUS_U) > 1 {
            out[idx] = r;
            idx += 1;
        }
        i += 1;
    }
    out
}

const _: () = {
    let expected: [u64; 5] = [1, 7, 13, 17, 19];
    let mut i = 0;
    while i < 5 {
        assert!(
            SG_ISOLATED_RESIDUES[i] == expected[i],
            "SG isolated-residue derivation drift"
        );
        i += 1;
    }
};

/// Sophie Germain residue pairs `(p mod 30, (2p + 1) mod 30)`. There
/// are exactly three: `(11, 23)`, `(23, 17)`, `(29, 29)`.
pub const SG_RESIDUE_PAIRS: [(u64, u64); 3] = compute_sg_pairs();

const fn compute_sg_pairs() -> [(u64, u64); 3] {
    let mut out = [(0u64, 0u64); 3];
    let mut i = 0usize;
    while i < 3 {
        let r = SOPHIE_GERMAIN_LOWER_RESIDUES[i];
        out[i] = (r, (2 * r + 1) % MODULUS_U);
        i += 1;
    }
    out
}

const _: () = {
    let expected: [(u64, u64); 3] = [(11, 23), (23, 17), (29, 29)];
    let mut i = 0;
    while i < 3 {
        let (a, b) = SG_RESIDUE_PAIRS[i];
        let (ea, eb) = expected[i];
        assert!(a == ea && b == eb, "SG pair derivation drift");
        assert!(gcd(a, MODULUS_U) == 1, "SG pair lower not in U(30)");
        assert!(gcd(b, MODULUS_U) == 1, "SG pair upper not in U(30)");
        assert!(b == (2 * a + 1) % MODULUS_U, "SG pair offset not 2p+1");
        i += 1;
    }
};

/// Cardinality identity. The two sets partition `U(30)`:
/// `3 + 5 = 8 = U30_CARDINALITY`.
pub const SG_PARTITION_CHECK: usize =
    SOPHIE_GERMAIN_LOWER_RESIDUES.len() + SG_ISOLATED_RESIDUES.len();

const _: () = assert!(
    SG_PARTITION_CHECK == U30_CARDINALITY,
    "SG twin/isolated partition does not cover U(30)"
);

/// Predicate: `n mod 30 ∈ {11, 23, 29}` — necessary residue
/// condition for `n` to be a Sophie Germain prime.
#[must_use]
pub const fn is_sophie_germain_candidate(n: u64) -> bool {
    let r = n % MODULUS_U;
    let mut i = 0usize;
    while i < 3 {
        if SOPHIE_GERMAIN_LOWER_RESIDUES[i] == r {
            return true;
        }
        i += 1;
    }
    false
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
    use super::*;

    #[test]
    fn lower_residues_canonical() {
        assert_eq!(SOPHIE_GERMAIN_LOWER_RESIDUES, [11, 23, 29]);
    }

    #[test]
    fn isolated_residues_canonical() {
        assert_eq!(SG_ISOLATED_RESIDUES, [1, 7, 13, 17, 19]);
    }

    #[test]
    fn pairs_canonical() {
        assert_eq!(SG_RESIDUE_PAIRS, [(11, 23), (23, 17), (29, 29)]);
    }

    #[test]
    fn partition_covers_u30() {
        assert_eq!(SG_PARTITION_CHECK, U30_CARDINALITY);
    }

    #[test]
    fn predicate_holds_on_known_sg_primes() {
        // Known Sophie Germain primes > 5: 11, 23, 29, 41, 53, 83, 89, 113, ...
        let known = &[11u64, 23, 29, 41, 53, 83, 89, 113, 131, 173, 179];
        for &p in known {
            assert!(
                is_sophie_germain_candidate(p),
                "{p} should pass the SG residue filter"
            );
            assert!(matches!(p % MODULUS_U, 11 | 23 | 29));
        }
    }
}
