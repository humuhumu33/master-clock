//! # Siamese prime generator pattern modulo 30
//!
//! A **Siamese prime pair** is a pair `(n² − 2, n² + 2)` where both
//! are prime. The generator `n` must satisfy `n² − 2 > 0` and the
//! pair must consist of two integers coprime to 30 (so neither is
//! divisible by 2, 3, or 5).
//!
//! Sweeping `n mod 30` across all 30 residues, exactly five
//! generator residues survive both `gcd((n² − 2) mod 30, 30) = 1`
//! and `gcd((n² + 2) mod 30, 30) = 1`:
//!
//! ```text
//!   n mod 30 ∈ {3, 9, 15, 21, 27}    (Siamese generator pattern)
//! ```
//!
//! These are exactly the values `6k + 3` for `k ∈ {0, 1, 2, 3, 4}`
//! ([OEIS A016945](https://oeis.org/A016945)). Note that the
//! generators themselves are NOT in `U(30)` — they are odd
//! multiples of 3.
//!
//! ## Necessary, not sufficient
//!
//! Five residues out of 30 — a 25/30 candidate reduction (83%).
//! Primality of `n² − 2` and `n² + 2` must still be tested
//! independently, e.g. via [`crate::sieve`].
//!
//! ## Anchor
//!
//! Computed at compile time from
//! [`crate::parameters::MODULUS_U`] = 30 and
//! [`crate::arithmetic::gcd`] alone.

use crate::arithmetic::gcd;
use crate::parameters::MODULUS_U;

/// The five generator residues mod 30 that can produce a Siamese
/// prime pair `(n² − 2, n² + 2)`. Computed at compile time by
/// sweeping `n ∈ [0, 30)` and keeping those where both `n² ± 2` mod
/// 30 are coprime to 30.
pub const SIAMESE_GENERATOR_RESIDUES: [u64; 5] = compute_siamese();

const fn compute_siamese() -> [u64; 5] {
    let mut out = [0u64; 5];
    let mut idx = 0usize;
    let mut n = 0u64;
    while n < MODULUS_U {
        let nsq = (n * n) % MODULUS_U;
        // n² − 2 mod 30 (handle wrap)
        let minus_two = if nsq >= 2 {
            nsq - 2
        } else {
            nsq + MODULUS_U - 2
        };
        let plus_two = (nsq + 2) % MODULUS_U;
        let lower_coprime = gcd(minus_two, MODULUS_U) == 1;
        let upper_coprime = gcd(plus_two, MODULUS_U) == 1;
        if lower_coprime && upper_coprime {
            if idx < 5 {
                out[idx] = n;
                idx += 1;
            }
        }
        n += 1;
    }
    out
}

const _: () = {
    let expected: [u64; 5] = [3, 9, 15, 21, 27];
    let mut i = 0;
    while i < 5 {
        assert!(
            SIAMESE_GENERATOR_RESIDUES[i] == expected[i],
            "Siamese generator residue derivation drift"
        );
        i += 1;
    }
};

/// **6k + 3 closed form.** Each Siamese generator residue is
/// `6 * k + 3` for `k ∈ {0, 1, 2, 3, 4}`. Verified at compile time.
pub const SIAMESE_AS_6K_PLUS_3: [u64; 5] = compute_6k_plus_3();

const fn compute_6k_plus_3() -> [u64; 5] {
    let mut out = [0u64; 5];
    let mut k = 0usize;
    while k < 5 {
        out[k] = (6 * k as u64) + 3;
        k += 1;
    }
    out
}

const _: () = {
    let mut i = 0;
    while i < 5 {
        assert!(
            SIAMESE_GENERATOR_RESIDUES[i] == SIAMESE_AS_6K_PLUS_3[i],
            "Siamese residues do not match 6k + 3 closed form"
        );
        i += 1;
    }
};

/// Predicate: `n mod 30 ∈ {3, 9, 15, 21, 27}` — necessary condition
/// for `n` to generate a Siamese prime pair.
///
/// Equivalent to `n mod 6 == 3`. Both forms verified by the
/// compile-time assertions above.
#[must_use]
pub const fn is_siamese_generator_candidate(n: u64) -> bool {
    let r = n % MODULUS_U;
    let mut i = 0usize;
    while i < 5 {
        if SIAMESE_GENERATOR_RESIDUES[i] == r {
            return true;
        }
        i += 1;
    }
    false
}

/// Equivalent characterization: `n mod 6 == 3`. Verified
/// equivalent to [`is_siamese_generator_candidate`] at compile time
/// for the first MODULUS_U residues.
#[must_use]
pub const fn is_siamese_generator_via_mod6(n: u64) -> bool {
    n % 6 == 3
}

const _: () = {
    let mut n = 0u64;
    while n < MODULUS_U {
        let a = is_siamese_generator_candidate(n);
        let b = is_siamese_generator_via_mod6(n);
        assert!(a == b, "Siamese predicates disagree mod 30");
        n += 1;
    }
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
    use super::*;

    #[test]
    fn generator_residues_canonical() {
        assert_eq!(SIAMESE_GENERATOR_RESIDUES, [3, 9, 15, 21, 27]);
    }

    #[test]
    fn matches_6k_plus_3_form() {
        assert_eq!(SIAMESE_AS_6K_PLUS_3, [3, 9, 15, 21, 27]);
        assert_eq!(SIAMESE_GENERATOR_RESIDUES, SIAMESE_AS_6K_PLUS_3);
    }

    #[test]
    fn predicate_matches_mod6_form() {
        for n in 0u64..200 {
            assert_eq!(
                is_siamese_generator_candidate(n),
                is_siamese_generator_via_mod6(n),
                "predicates disagree at n={n}"
            );
        }
    }

    #[test]
    fn predicate_holds_on_small_siamese_pairs() {
        // n=3:  (7, 11)   both prime
        // n=9:  (79, 83)  both prime
        // n=15: (223, 227) both prime
        // n=21: (439, 443) both prime
        // n=27: (727, 731=17·43) — 731 NOT prime
        // We check the residue condition only, not actual primality.
        let known_generators = &[3u64, 9, 15, 21, 27];
        for &n in known_generators {
            assert!(is_siamese_generator_candidate(n));
            assert!(is_siamese_generator_via_mod6(n));
        }
    }

    #[test]
    fn predicate_rejects_non_siamese_generators() {
        // n=4: 16-2=14, gcd(14,30)=2. Rejected.
        assert!(!is_siamese_generator_candidate(4));
        // n=7: 49-2=47, 49+2=51=3·17. gcd(51,30)=3. Rejected.
        assert!(!is_siamese_generator_candidate(7));
        // n=11: 121-2=119, 121+2=123=3·41. gcd(123,30)=3. Rejected.
        assert!(!is_siamese_generator_candidate(11));
    }
}
