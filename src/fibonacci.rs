//! # Fibonacci numbers, Pisano periods, and Fibonacci digital roots
//!
//! `F_0 = 0`, `F_1 = 1`, `F_n = F_{n-1} + F_{n-2}` for `n ≥ 2`.
//!
//! ## Pisano period mod 9 = 24
//!
//! The sequence `(F_n mod 9)` is periodic. The minimal period is 24,
//! verified in [`tests::pisano_period_mod_dr_is_minimal`] by exhaustive
//! check that no `p < 24` satisfies `F_p ≡ 0 (mod 9)` and
//! `F_{p+1} ≡ 1 (mod 9)` simultaneously.
//!
//! ## `F_49` — the largest 32-bit-decimal Fibonacci index
//!
//! `F_49 = 7,778,742,049`. The largest Fibonacci number that fits in
//! a 32-bit decimal magnitude (`< 10¹⁰`); a convenient upper bound
//! for any test sweep that lives in `u64`. Computed at compile time
//! from the recurrence below.

use crate::parameters::MODULUS_DR;
#[cfg(test)]
use crate::parameters::PISANO_PERIOD_MOD_DR;

/// `F_49 = 7,778,742,049`, computed at compile time from the
/// Fibonacci recurrence.
pub const F49: u64 = fib(49);

const _: () = assert!(
    F49 == 7_778_742_049,
    "F_49 must equal 7,778,742,049 by the standard Fibonacci recurrence"
);

/// Compute `F_k` exactly. Overflows `u64` for `k ≥ 94`; only used here
/// for `k ≤ 49`.
#[must_use]
pub const fn fib(k: u64) -> u64 {
    if k == 0 {
        return 0;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut i: u64 = 1;
    while i < k {
        let next = a + b;
        a = b;
        b = next;
        i += 1;
    }
    b
}

/// Compute `F_k mod m` using the standard recurrence with reduction at
/// each step. Avoids `u64` overflow by keeping the running pair below
/// `2m`.
#[must_use]
pub const fn fib_mod(k: u64, m: u64) -> u64 {
    if m == 0 {
        return 0;
    }
    if k == 0 {
        return 0;
    }
    let mut a: u64 = 0;
    let mut b: u64 = 1 % m;
    let mut i: u64 = 1;
    while i < k {
        let next = (a + b) % m;
        a = b;
        b = next;
        i += 1;
    }
    b
}

/// `F_k mod MODULUS_DR` (i.e., mod 9).
#[must_use]
pub const fn fib_mod_dr(k: u64) -> u64 {
    fib_mod(k, MODULUS_DR)
}

/// Digital root of `F_k` for `k ≥ 1`.
///
/// `dr(F_k) = if F_k mod 9 = 0 then 9 else (F_k mod 9)`.
///
/// For `k = 0`, `F_0 = 0` and `dr(0) = 0` per the [`crate::digital_root`]
/// convention.
#[must_use]
pub const fn dr_fib(k: u64) -> u8 {
    if k == 0 {
        return 0;
    }
    let v = fib_mod_dr(k);
    #[allow(clippy::cast_possible_truncation)]
    {
        if v == 0 {
            9
        } else {
            v as u8
        }
    }
}

/// First `PISANO_PERIOD_MOD_DR + 2` values of `F_k mod 9`, used by the
/// minimality check.
#[cfg(test)]
const FIB_MOD_DR_TABLE: [u64; (PISANO_PERIOD_MOD_DR + 2) as usize] = {
    let mut out = [0u64; (PISANO_PERIOD_MOD_DR + 2) as usize];
    out[1] = 1 % MODULUS_DR;
    let mut i: usize = 2;
    while i < out.len() {
        out[i] = (out[i - 1] + out[i - 2]) % MODULUS_DR;
        i += 1;
    }
    out
};

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;
    use crate::digital_root::dr;

    /// Spot-check the Fibonacci recurrence against the standard sequence.
    #[test]
    fn fib_matches_known_values() {
        let known = [
            0u64, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
        ];
        for (k, &expected) in known.iter().enumerate() {
            assert_eq!(fib(k as u64), expected, "F_{k}");
        }
    }

    /// `fib_mod` agrees with `fib(k) mod m` for small inputs.
    #[test]
    fn fib_mod_agrees_with_full_fib() {
        for k in 0..30u64 {
            for m in 1..30u64 {
                assert_eq!(
                    fib_mod(k, m),
                    fib(k) % m,
                    "fib_mod({k}, {m}) ≠ fib({k}) mod {m}"
                );
            }
        }
    }

    /// **Theorem.** `F_24 ≡ 0 (mod 9)` and `F_25 ≡ 1 (mod 9)`. This is
    /// the closing-of-period condition.
    #[test]
    fn pisano_period_mod_dr_closes_at_24() {
        assert_eq!(fib_mod_dr(PISANO_PERIOD_MOD_DR), 0);
        assert_eq!(fib_mod_dr(PISANO_PERIOD_MOD_DR + 1), 1);
    }

    /// **Theorem.** No `p < 24` satisfies the closing-of-period
    /// condition simultaneously. Confirms that 24 is the **minimal**
    /// Pisano period mod 9.
    #[test]
    fn pisano_period_mod_dr_is_minimal() {
        for p in 1..PISANO_PERIOD_MOD_DR {
            let f_p = fib_mod_dr(p);
            let f_p1 = fib_mod_dr(p + 1);
            assert!(
                !(f_p == 0 && f_p1 == 1),
                "F_{p} ≡ 0 and F_{p_plus_1} ≡ 1 (mod 9) — period would be {p} < 24",
                p = p,
                p_plus_1 = p + 1
            );
        }
    }

    /// `F_49 = 7,778,742,049`, by the recurrence. (The compile-time
    /// `const _: () = assert!` already forces this; this test is
    /// documentation.)
    #[test]
    fn f49_equals_seven_billion() {
        assert_eq!(F49, 7_778_742_049);
    }

    /// `dr(F_k)` over one Pisano period must contain only values in
    /// `{1, …, 9}` (since `F_k ≠ 0` for `k ≥ 1`).
    #[test]
    fn dr_fib_range_over_one_period() {
        for k in 1..=PISANO_PERIOD_MOD_DR {
            let r = dr_fib(k);
            assert!((1..=9).contains(&r), "dr(F_{k}) = {r} out of [1, 9]");
        }
    }

    /// `dr(F_k)` matches `dr` applied to `F_k` for `k ∈ [1, 29]`,
    /// covering the full natural domain of `U(30)`.
    ///
    /// This is the **load-bearing independent check**: `dr_fib`
    /// (which uses `fib_mod 9`) and `dr ∘ fib` (which computes the full
    /// Fibonacci value first) are independent code paths; their
    /// agreement on every `k` in this range is what makes Appendix A
    /// column 3 verifiable against the recurrence rather than against
    /// Gary's own OEIS A227896 entry. `F_29 = 514,229` fits comfortably
    /// in `u64`; extending to `k ≤ 93` would still be safe.
    #[test]
    fn dr_fib_agrees_with_dr_of_fib() {
        for k in 1u64..=29 {
            assert_eq!(
                dr_fib(k),
                dr(fib(k)),
                "dr_fib({k}) ≠ dr(F_{k}) = dr({})",
                fib(k)
            );
        }
    }

    /// **Palindrome identity.** `dr(F_u) = dr(F_{24 − u})` for `u ∈ U(24)`.
    ///
    /// `U(24) = { 1, 5, 7, 11, 13, 17, 19, 23 }`. The palindrome is the
    /// genuine non-trivial mathematical content underpinning the
    /// `Σ dr(F_n) = 432` invariant on Appendix A's column 3.
    ///
    /// **Theoretical proof** (one line, from two standard identities):
    ///
    /// ```text
    /// F_{24−u}  ≡  F_{−u}            (Pisano period mod 9 = 24)
    /// F_{−n}    =  (−1)^{n+1} F_n    (negative-index Fibonacci)
    /// u o