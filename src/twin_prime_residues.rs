//! # Twin prime residue constraints modulo 30
//!
//! For any twin prime pair `(p, p+2)` with `p > 5`, both `p` and
//! `p + 2` are coprime to 30 (since neither can be divisible by 2,
//! 3, or 5). Therefore both lie in `U(30) = {1, 7, 11, 13, 17, 19,
//! 23, 29}`. Among the eight residues, exactly three satisfy
//! `(r + 2) mod 30` is also coprime to 30:
//!
//! ```text
//!   r ∈ {11, 17, 29}    (twin-prime lower-residue set)
//! ```
//!
//! The remaining five residues are "isolated lower" — `p mod 30`
//! in this set forces `p + 2` divisible by 3 or 5, hence composite:
//!
//! ```text
//!   r ∈ {1, 7, 13, 19, 23}
//! ```
//!
//! This is a **necessary** condition on twin primes — it does not
//! certify primality. The wheel-30 sieve in [`crate::sieve`] still
//! does the actual primality test; this module provides the
//! upstream filter that rules out 5/8 of all candidates before the
//! sieve runs.
//!
//! ## Anchor
//!
//! Every constant is computed at compile time from
//! [`crate::parameters::MODULUS_U`] = 30 via
//! [`crate::arithmetic::gcd`] and [`crate::u30::RESIDUES`]. There
//! are no hand-typed residue values in production paths; the
//! `[11, 17, 29]` literals appear only in compile-time cross-checks.

use crate::arithmetic::gcd;
use crate::parameters::MODULUS_U;
use crate::u30::{RESIDUES, U30_CARDINALITY};

/// The three residues mod 30 that can be the smaller member of a
/// twin prime pair `(p, p+2)` with `p > 5`. Computed at compile time
/// by enumerating `RESIDUES` and keeping those `r` where
/// `gcd((r + 2) mod 30, 30) = 1`.
pub const TWIN_PRIME_LOWER_RESIDUES: [u64; 3] = compute_twin_prime_lower();

const fn compute_twin_prime_lower() -> [u64; 3] {
    let mut out = [0u64; 3];
    let mut idx = 0usize;
    let mut i = 0usize;
    while i < U30_CARDINALITY {
        let r = RESIDUES[i];
        let s = (r + 2) % MODULUS_U;
        if gcd(s, MODULUS_U) == 1 {
            out[idx] = r;
            idx += 1;
        }
        i += 1;
    }
    out
}

const _: () = {
    // Cross-check derivation against the canonical {11, 17, 29}.
    let expected: [u64; 3] = [11, 17, 29];
    let mut i = 0;
    while i < 3 {
        assert!(
            TWIN_PRIME_LOWER_RESIDUES[i] == expected[i],
            "twin-prime lower-residue derivation drift"
        );
        i += 1;
    }
};

/// The five residues mod 30 that **cannot** be the lower member of
/// a twin prime pair with `p > 5`. For these `r`, the value
/// `r + 2` is divisible by 3 or 5 (or both), so `p + 2` is forced
/// composite.
pub const ISOLATED_LOWER_RESIDUES: [u64; 5] = compute_isolated_lower();

const fn compute_isolated_lower() -> [u64; 5] {
    let mut out = [0u64; 5];
    let mut idx = 0usize;
    let mut i = 0usize;
    while i < U30_CARDINALITY {
        let r = RESIDUES[i];
        let s = (r + 2) % MODULUS_U;
        if gcd(s, MODULUS_U) > 1 {
            out[idx] = r;
            idx += 1;
        }
        i += 1;
    }
    out
}

const _: () = {
    let expected: [u64; 5] = [1, 7, 13, 19, 23];
    let mut i = 0;
    while i < 5 {
        assert!(
            ISOLATED_LOWER_RESIDUES[i] == expected[i],
            "isolated-lower-residue derivation drift"
        );
        i += 1;
    }
};

/// Twin prime residue pairs `(p mod 30, (p + 2) mod 30)` for every
/// `p` such that both `p` and `p + 2` are coprime to 30. There are
/// exactly three such pairs.
pub const TWIN_PRIME_RESIDUE_PAIRS: [(u64, u64); 3] = compute_twin_pairs();

const fn compute_twin_pairs() -> [(u64, u64); 3] {
    let mut out = [(0u64, 0u64); 3];
    let mut i = 0usize;
    while i < 3 {
        let r = TWIN_PRIME_LOWER_RESIDUES[i];
        out[i] = (r, (r + 2) % MODULUS_U);
        i += 1;
    }
    out
}

const _: () = {
    let expected: [(u64, u64); 3] = [(11, 13), (17, 19), (29, 1)];
    let mut i = 0;
    while i < 3 {
        let (a, b) = TWIN_PRIME_RESIDUE_PAIRS[i];
        let (ea, eb) = expected[i];
        assert!(a == ea && b == eb, "twin-prime pair derivation drift");
        // Both pair members must be coprime to 30.
        assert!(gcd(a, MODULUS_U) == 1, "twin pair lower not in U(30)");
        assert!(gcd(b, MODULUS_U) == 1, "twin pair upper not in U(30)");
        // Second member must equal first + 2 mod 30.
        assert!(b == (a + 2) % MODULUS_U, "twin pair offset not 2");
        i += 1;
    }
};

/// **Cardinality identity.** The two sets partition `U(30)`:
/// `|TWIN_PRIME_LOWER_RESIDUES| + |ISOLATED_LOWER_RESIDUES|
/// = 3 + 5 = 8 = U30_CARDINALITY`.
pub const PARTITION_CHECK: usize =
    TWIN_PRIME_LOWER_RESIDUES.len() + ISOLATED_LOWER_RESIDUES.len();

const _: () = assert!(
    PARTITION_CHECK == U30_CARDINALITY,
    "twin/isolated partition does not cover U(30)"
);

/// Predicate: `n mod 30 ∈ {11, 17, 29}` — the necessary residue
/// condition for `n` to be the smaller member of a twin prime pair.
///
/// Returns `true` iff `n mod 30` is in `TWIN_PRIME_LOWER_RESIDUES`.
/// **Does not test primality.** A `true` result is necessary but
/// not sufficient; combine with [`crate::sieve::pi_via_u30_sieve`]
/// or any primality test to certify the pair.
#[must_use]
pub const fn is_twin_prime_candidate_lower(n: u64) -> bool {
    let r = n % MODULUS_U;
    let mut i = 0usize;
    while i < 3 {
        if TWIN_PRIME_LOWER_RESIDUES[i] == r {
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
    fn lower_residues_are_canonical() {
        assert_eq!(TWIN_PRIME_LOWER_RESIDUES, [11, 17, 29]);
    }

    #[test]
    fn isolated_residues_are_canonical() {
        assert_eq!(ISOLATED_LOWER_RESIDUES, [1, 7, 13, 19, 23]);
    }

    #[test]
    fn pairs_are_canonical() {
        assert_eq!(TWIN_PRIME_RESIDUE_PAIRS, [(11, 13), (17, 19), (29, 1)]);
    }

    #[test]
    fn lower_plus_isolated_partitions_u30() {
        assert_eq!(
            TWIN_PRIME_LOWER_RESIDUES.len() + ISOLATED_LOWER_RESIDUES.len(),
            U30_CARDINALITY
        );
        // No overlap.
        for &r in &TWIN_PRIME_LOWER_RESIDUES {
            assert!(!ISOLATED_LOWER_RESIDUES.contains(&r));
        }
    }

    #[test]
    fn lower_residues_have_coprime_successor() {
        for &r in &TWIN_PRIME_LOWER_RESIDUES {
            let s = (r + 2) % MODULUS_U;
            assert_eq!(gcd(s, MODULUS_U), 1);
        }
    }

    #[test]
    fn isolated_residues_have_noncoprime_successor() {
        for &r in &ISOLATED_LOWER_RESIDUES {
            let s = (r + 2) % MODULUS_U;
            assert!(gcd(s, MODULUS_U) > 1);
        }
    }

    /// **Empirical anchor.** The first eleven twin prime pairs above
    /// 5 all have `p mod 30 ∈ {11, 17, 29}`, confirming the
    /// necessary-condition predicate.
    #[test]
    fn predicate_holds_on_known_twin_primes() {
        let known_twin_primes: &[(u64, u64)] = &[
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
        for &(p, q) in known_twin_primes {
            assert_eq!(q, p + 2, "{p} and {q} not consecutive odd");
            assert!(
                is_twin_prime_candidate_lower(p),
                "{p} should pass the residue filter"
            );
            let r = p % MODULUS_U;
            assert!(
                matches!(r, 11 | 17 | 29),
                "{p} mod 30 = {r} not in {{11, 17, 29}}"
            );
        }
    }

    /// **Negative anchor.** Primes congruent to `{1, 7, 13, 19, 23}`
    /// mod 30 are isolated (cannot be lower twin). Spot-check.
    #[test]
    fn predicate_rejects_isolated_primes() {
        // 7 (= 7 mod 30): 7+2=9 = 3·3, composite. So 7 isolated.
        assert!(!is_twin_prime_candidate_lower(7));
        // 23: 23+2=25 = 5², composite. So 23 isolated.
        assert!(!is_twin_prime_candidate_lower(23));
        // 31 ≡ 1 mod 30: 31+2=33 = 3·11, composite.
        assert!(!is_twin_prime_candidate_lower(31));
        // 43 ≡ 13 mod 30: 43+2=45 = 9·5, composite.
        assert!(!is_twin_prime_candidate_lower(43));
        // 79 ≡ 19 mod 30: 79+2=81 = 3⁴, composite.
        assert!(!is_twin_prime_candidate_lower(79));
    }

    /// **Cross-check with the sieve.** Among primes from 7 to 200,
    /// every twin-prime lower passes the predicate, and every prime
    /// failing the predicate is genuinely isolated (no prime two
    /// above it in the same range).
    #[cfg(feature = "alloc")]
    #[test]
    fn sieve_cross_check_below_200() {
        use crate::sieve::pi_via_u30_sieve;
        // Build a tiny "is_prime" table by repeated π differences.
        let is_prime = |n: u64| -> bool {
            n >= 2 && pi_via_u30_sieve(n) > pi_via_u30_sieve(n - 1)
        };
        for p in 7u64..=200 {
            if !is_prime(p) {
                continue;
            }
            let twin = is_prime(p + 2);
            let candidate = is_twin_prime_candidate_lower(p);
            if twin {
                assert!(candidate, "{p}: twin pair but predicate rejected");
            }
            // The converse — candidate ⟹ twin — is not necessary;
            // candidate is a residue filter, not a primality test.
        }
    }
}
