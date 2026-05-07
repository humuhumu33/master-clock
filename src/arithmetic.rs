//! # Number-theoretic helpers
//!
//! `const fn` implementations of `gcd`, `lcm`, and Euler's totient `φ`.

/// Greatest common divisor via Euclid's algorithm.
#[must_use]
pub const fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Least common multiple via the identity `lcm(a, b) · gcd(a, b) = a · b`.
#[must_use]
pub const fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { 0 } else { (a / gcd(a, b)) * b }
}

/// Euler's totient `φ(n)` — the count of integers in `[1, n)` coprime to `n`.
#[must_use]
pub const fn euler_phi(n: u64) -> u64 {
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    let mut count = 0u64;
    let mut k = 1u64;
    while k < n {
        if gcd(k, n) == 1 { count += 1; }
        k += 1;
    }
    count
}

/// Largest power of `base` strictly less than `n`.
#[must_use]
pub const fn largest_power_below(base: u64, n: u64) -> u64 {
    if base <= 1 || n <= 1 { return 1; }
    let mut p: u64 = 1;
    while let Some(next) = p.checked_mul(base) {
        if next >= n { return p; }
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
        assert_eq!(gcd(12, 18), 6);
        assert_eq!(gcd(30, 360), 30);
    }

    #[test]
    fn lcm_basic_identities() {
        assert_eq!(lcm(0, 5), 0);
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(lcm(30, 9), 24), 360);
    }

    #[test]
    fn euler_phi_small_cases() {
        assert_eq!(euler_phi(1), 1);
        assert_eq!(euler_phi(30), 8);
        assert_eq!(euler_phi(360), 96);
    }

    #[test]
    fn largest_power_below_spot_checks() {
        assert_eq!(largest_power_below(10, 7_778_742_049), 1_000_000_000);
        assert_eq!(largest_power_below(10, 11), 10);
        assert_eq!(largest_power_below(2, 1000), 512);
    }
}
