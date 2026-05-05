//! # Number-theoretic helpers
//!
//! `const fn` implementations of `gcd`, `lcm`, and Euler's totient `φ`.
//! Used everywhere downstream for compile-time derivation of all
//! constants in the model.
//!
//! All functions are `const fn` so derived parameters
//! (`MODULUS_R`, `LATTICE_LAYERS`, `U30_CARDINALITY`, `R_CARDINALITY`,
//! `RESIDUES`, etc.) compute at compile time. There are no runtime
//! parameters in this crate; every fact is either decided at build or
//! falsified at build.

/// Greatest common divisor via Euclid's algorithm.
///
/// `gcd(a, 0) = a` and `gcd(a, b) = gcd(b, a mod b)` for `b > 0`.
#[must_use]
pub const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Least common multiple via the identity `lcm(a, b) · gcd(a, b) = a · b`.
///
/// Returns `0` if either argument is `0`.
#[must_use]
pub const fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a / gcd(a, b)) * b
    }
}

/// Euler's totient `φ(n)` — the count of integers in `[1, n)` coprime
/// to `n`. Brute-force enumeration; intended only for the small moduli
/// (≤ 360) used in this crate.
#[must_use]
pub const fn euler_phi(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let mut count = 0u64;
    let mut k = 1u64;
    while k < n {
        if gcd(k, n) == 1 {
            count += 1;
        }
        k += 1;
    }
    count
}

/// Largest power of `base` strictly less than `n`.
///
/// `largest_power_below(base, n) = base^k` such that
/// `base^k < n ≤ base^(k+1)`.
///
/// Used to derive constants like `RF_LOG_SCALE = 10⁹` from
/// `F49_STATED ≈ 7.778 × 10⁹` and `NUMERAL_BASE = 10` — no
/// hardcoded `10⁹` literal anywhere in the production paths.
///
/// Returns `1` if `n ≤ 1` or `base ≤ 1` (the trivial cases).
#[must_use]
pub const fn largest_power_below(base: u64, n: u64) -> u64 {
    if base <= 1 || n <= 1 {
        return 1;
    }
    let mut p: u64 = 1;
    while let Some(next) = p.checked_mul(base) {
        if next >= n {
            return p;
        }
        p = next;
    }
    p
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    #[test]
    fn gcd_basic_identities() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(7, 0), 7);
        assert_eq!(gcd(0, 7), 7);
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(30, 360), 30);
        assert_eq!(gcd(17, 13), 1);
    }

    #[test]
    fn lcm_basic_identities() {
        assert_eq!(lcm(0, 5), 0);
        assert_eq!(lcm(5, 0), 0);
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(30, 24), 120);
        assert_eq!(lcm(lcm(30, 9), 24), 360);
    }

    #[test]
    fn euler_phi_small_cases() {
        assert_eq!(euler_phi(1), 1);
        assert_eq!(euler_phi(2), 1);
        assert_eq!(euler_phi(3), 2);
        assert_eq!(euler_phi(5), 4);
        assert_eq!(euler_phi(6), 2);
        assert_eq!(euler_phi(9), 6);
        assert_eq!(euler_phi(24), 8);
        assert_eq!(euler_phi(30), 8);
        assert_eq!(euler_phi(360), 96);
    }

    /// `largest_power_below` derives `10⁹` from
    /// `(NUMERAL_BASE, F49_STATED)`, removing the `10⁹` literal
    /// from production paths. Spot-checks across small bases.
    #[test]
    fn largest_power_below_spot_checks() {
        // 10^9 < 7,778,742,049 < 10^10  →  largest_power = 10^9.
        assert_eq!(largest_power_below(10, 7_778_742_049), 1_000_000_000);
        // 10^0 < 10 ≤ 10^1  →  largest_power(10, 10) = 1.
        assert_eq!(largest_power_below(10, 10), 1);
        // 10^1 < 11 ≤ 10^2  →  largest_power(10, 11) = 10.
        assert_eq!(largest_power_below(10, 11), 10);
        // base ≤ 1 → 1.
        assert_eq!(largest_power_below(1, 100), 1);
        assert_eq!(largest_power_below(0, 100), 1);
        // n ≤ 1 → 1.
        assert_eq!(largest_power_below(10, 0), 1);
        assert_eq!(largest_power_below(10, 1), 1);
        // base 2 sanity: 2^9 = 512 < 1000 ≤ 2^10 = 1024.
        assert_eq!(largest_power_below(2, 1000), 512);
    }

    /// `φ` is multiplicative on coprime factors: `φ(m·n) = φ(m)·φ(n)` when
    /// `gcd(m, n) = 1`. Confirms our brute-force agrees with the
    /// product formula on `30 = 2·3·5` and `360 = 8·9·5`.
    #[test]
    fn euler_phi_multiplicative_on_coprimes() {
        // 30 = 2 · 15, gcd = 1
        assert_eq!(euler_phi(30), euler_phi(2) * euler_phi(15));
        // 360 = 8 · 45, gcd = 1
        assert_eq!(euler_phi(360), euler_phi(8) * euler_phi(45));
    }
}
