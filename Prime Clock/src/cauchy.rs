//! # Layer 5 — Computable real numbers as Cauchy sequences of rationals
//!
//! A real number is encoded as a pair: a sequence `nth: ℕ → ℚ` whose
//! limit is the value, and a *modulus of convergence* `modulus: ℚ⁺ →
//! ℕ` such that for every `eps > 0`,
//!
//! ```text
//! ∀ n, m ≥ modulus(eps),  |nth(n) − nth(m)| < eps.
//! ```
//!
//! With both pieces in hand, you can request any precision: call
//! [`approx`] with the desired `eps` and you get a rational
//! approximation guaranteed to be within `eps` of the true limit.
//!
//! ## Place in the eight-layer bridge
//!
//! Layer 5 is the constructive realisation of UOR's
//! `convergence::L0_State` — the algebraic identity "ℝ has dimension 1
//! and characteristic identity 'identity'" made into a concrete,
//! computable type.
//!
//! ## Closure under arithmetic
//!
//! Cauchy sequences over `ℚ` form a ring. We expose the three
//! constructive ring operations: [`Cauchy::add`], [`Cauchy::sub`],
//! and [`Cauchy::mul`]. Each produces a new Cauchy sequence whose
//! modulus is computed from the operands' moduli.
//!
//! ### Modulus combination rules
//!
//! - **Sum / difference.** `|(a_n+b_n) − (a_m+b_m)| ≤ |a_n−a_m| +
//!   |b_n−b_m|`. Choose each operand to converge within `eps/2`. So
//!   `modulus_{a±b}(eps) = max(a.modulus(eps/2), b.modulus(eps/2))`.
//! - **Product.** `|a_n b_n − a_m b_m| ≤ |a_n||b_n−b_m| + |b_m||a_n−a_m|`.
//!   We need a priori bounds on `|a|` and `|b|`. The product
//!   constructor takes those bounds explicitly to keep the modulus
//!   computation total. (`a_bound` and `b_bound` are rationals such
//!   that `|a| ≤ a_bound` and `|b| ≤ b_bound`.) Then choose each
//!   operand to converge within `eps / (2 · max(a_bound, b_bound))`.
//!
//! ## Implementation
//!
//! Because Taylor-series term construction needs to capture the
//! input rational, the trait is realised as a struct that owns two
//! boxed closures. This is one heap allocation per sequence — cheap
//! relative to the cost of the rational arithmetic each call performs.
//!
//! ## Example: a constant sequence
//!
//! ```ignore
//! use toroidal_prime_clock::cauchy::Cauchy;
//! use toroidal_prime_clock::rational::Rational;
//!
//! let half = Cauchy::constant(Rational::new_i64(1, 2).unwrap());
//! assert_eq!(half.approx(&Rational::new_i64(1, 1_000).unwrap()),
//!            Rational::new_i64(1, 2).unwrap());
//! ```

extern crate alloc;
use alloc::rc::Rc;
use core::cmp::max;

use crate::rational::Rational;

// Closure types. `Rc` lets sequences be cloned/composed without
// re-running the term computation.
type Nth = Rc<dyn Fn(u64) -> Rational>;
type Modulus = Rc<dyn Fn(&Rational) -> u64>;

/// A computable real number, encoded as a Cauchy sequence of
/// rationals together with its modulus of convergence.
///
/// Constructed via [`Cauchy::constant`], [`Cauchy::from_fn`], or by
/// the elementary-function helpers in [`crate::elementary`].
#[derive(Clone)]
pub struct Cauchy {
    nth: Nth,
    modulus: Modulus,
}

impl Cauchy {
    /// Build a sequence from raw `nth` and `modulus` closures.
    ///
    /// The caller is responsible for ensuring that the closures
    /// satisfy the Cauchy condition `∀ n, m ≥ modulus(eps), |nth(n) −
    /// nth(m)| < eps`.
    #[must_use]
    pub fn from_fn<F, M>(nth: F, modulus: M) -> Self
    where
        F: Fn(u64) -> Rational + 'static,
        M: Fn(&Rational) -> u64 + 'static,
    {
        Self {
            nth: Rc::new(nth),
            modulus: Rc::new(modulus),
        }
    }

    /// The constant sequence `n ↦ value`. Modulus is `0` for any `eps`.
    #[must_use]
    pub fn constant(value: Rational) -> Self {
        let v = value.clone();
        Self {
            nth: Rc::new(move |_| v.clone()),
            modulus: Rc::new(|_| 0),
        }
    }

    /// The `n`-th term of the sequence.
    #[must_use]
    pub fn nth(&self, n: u64) -> Rational {
        (self.nth)(n)
    }

    /// Modulus of convergence: the smallest `N` such that all terms
    /// from index `N` onward stay within `eps` of each other.
    #[must_use]
    pub fn modulus(&self, eps: &Rational) -> u64 {
        (self.modulus)(eps)
    }

    /// Compute a rational approximation guaranteed within `eps` of
    /// the true limit.
    #[must_use]
    pub fn approx(&self, eps: &Rational) -> Rational {
        let n = self.modulus(eps);
        self.nth(n)
    }

    /// Sum of two Cauchy sequences.
    #[must_use]
    pub fn add(&self, other: &Self) -> Self {
        let a_nth = Rc::clone(&self.nth);
        let b_nth = Rc::clone(&other.nth);
        let a_mod = Rc::clone(&self.modulus);
        let b_mod = Rc::clone(&other.modulus);
        Self {
            nth: Rc::new(move |n| &a_nth(n) + &b_nth(n)),
            modulus: Rc::new(move |eps| {
                let half = halve(eps);
                max(a_mod(&half), b_mod(&half))
            }),
        }
    }

    /// Difference of two Cauchy sequences.
    #[must_use]
    pub fn sub(&self, other: &Self) -> Self {
        let a_nth = Rc::clone(&self.nth);
        let b_nth = Rc::clone(&other.nth);
        let a_mod = Rc::clone(&self.modulus);
        let b_mod = Rc::clone(&other.modulus);
        Self {
            nth: Rc::new(move |n| &a_nth(n) - &b_nth(n)),
            modulus: Rc::new(move |eps| {
                let half = halve(eps);
                max(a_mod(&half), b_mod(&half))
            }),
        }
    }

    /// Negation `−a`. Modulus is unchanged.
    #[must_use]
    pub fn neg(&self) -> Self {
        let a_nth = Rc::clone(&self.nth);
        let a_mod = Rc::clone(&self.modulus);
        Self {
            nth: Rc::new(move |n| -&a_nth(n)),
            modulus: Rc::new(move |eps| a_mod(eps)),
        }
    }

    /// Product of two Cauchy sequences with explicit a-priori bounds.
    ///
    /// `a_bound ≥ |limit(self)|` and `b_bound ≥ |limit(other)|`.
    /// Both bounds must be strictly positive rationals — they appear
    /// in the modulus denominator.
    ///
    /// # Errors
    ///
    /// Returns `None` if either bound is non-positive.
    #[must_use]
    pub fn mul(&self, other: &Self, a_bound: &Rational, b_bound: &Rational) -> Option<Self> {
        if a_bound.signum() <= 0 || b_bound.signum() <= 0 {
            return None;
        }
        let a_nth = Rc::clone(&self.nth);
        let b_nth = Rc::clone(&other.nth);
        let a_mod = Rc::clone(&self.modulus);
        let b_mod = Rc::clone(&other.modulus);
        let a_b = a_bound.clone();
        let b_b = b_bound.clone();
        Some(Self {
            nth: Rc::new(move |n| &a_nth(n) * &b_nth(n)),
            modulus: Rc::new(move |eps| {
                // Need: |a_n||b_n−b_m| + |b_m||a_n−a_m| < eps
                // Split each term ≤ eps/2; use bounds for |a_n|, |b_m|:
                //   |b_n−b_m| < eps / (2 a_bound)
                //   |a_n−a_m| < eps / (2 b_bound)
                let two = Rational::from_int(2);
                let denom_b = &two * &a_b;
                let denom_a = &two * &b_b;
                let eps_b = eps.checked_div(&denom_b).unwrap_or_else(Rational::zero);
                let eps_a = eps.checked_div(&denom_a).unwrap_or_else(Rational::zero);
                max(a_mod(&eps_a), b_mod(&eps_b))
            }),
        })
    }
}

/// Divide a rational by 2.
fn halve(eps: &Rational) -> Rational {
    let two = Rational::from_int(2);
    // `two` is non-zero so checked_div always returns Some.
    eps.checked_div(&two).unwrap_or_else(Rational::zero)
}

/// Convenience: Box-free top-level approximator. Equivalent to
/// `seq.approx(eps)` but avoids exposing the method on doctests.
#[must_use]
pub fn approx(seq: &Cauchy, eps: &Rational) -> Rational {
    seq.approx(eps)
}

// Cauchy intentionally does NOT implement core::ops::{Add,Sub,Mul}.
// Multiplication requires explicit bounds; addition/subtraction are
// fine but kept as named methods for symmetry with `mul`. The named
// methods make modulus-of-convergence reasoning visible at call sites.

// ─── UOR shape encoding (Phase 3 O2) ─────────────────────────────────
//
// Layer 5 of the BRIDGE.md eight-layer stack. The shape encodes one
// `convergence::L0_State` realisation: a productive stream paired
// with an epoch-boundary descent measure (the modulus of convergence).
//
// UOR primitive mapping:
//   Cauchy::nth      ↔  stream::ProductiveStream  (each term is one Epoch)
//   Cauchy::modulus  ↔  stream::EpochBoundary     (descent measure → ε)
//
// The shape carries two `ConstraintRef::Bound`s naming the two UOR
// terms above. They are opaque to the current preflight validator
// (extension IRIs) but make the binding searchable: `grep
// "uor.foundation/stream/ProductiveStream"` from the project root
// hits both this shape and the UOR namespace.

use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef};

/// UOR shape for [`Cauchy`].
///
/// The two leaf-level constraints witness the binding to
/// `convergence::L0_State` (constructive ℝ) and
/// `stream::ProductiveStream` (the epoch-indexed term sequence).
pub struct CauchyShape;

impl ConstrainedTypeShape for CauchyShape {
    const IRI: &'static str = "https://uor.foundation/prime-clock/cauchy/L0_State";
    const SITE_COUNT: usize = 2;
    const CONSTRAINTS: &'static [ConstraintRef] = &[
        // Site 0 — the term sequence is a productive stream.
        ConstraintRef::Bound {
            observable_iri: "https://uor.foundation/stream/ProductiveStream",
            bound_shape_iri: "https://uor.foundation/prime-clock/cauchy/term_stream",
            args_repr: "nth: ℕ → ℚ, total at every index",
        },
        // Site 1 — the modulus is an epoch boundary on ℚ⁺.
        ConstraintRef::Bound {
            observable_iri: "https://uor.foundation/stream/EpochBoundary",
            bound_shape_iri: "https://uor.foundation/prime-clock/cauchy/modulus",
            args_repr: "modulus: ℚ⁺ → ℕ, monotone descent",
        },
    ];
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn r(n: i64, d: i64) -> Rational {
        // Tests can `unwrap` since they construct known-good inputs.
        Rational::new_i64(n, d).unwrap()
    }

    #[test]
    fn constant_sequence_returns_value_at_any_precision() {
        let half = Cauchy::constant(r(1, 2));
        for prec in [1, 1_000, 1_000_000] {
            let approx = half.approx(&r(1, prec));
            assert_eq!(approx, r(1, 2));
        }
    }

    #[test]
    fn sum_of_constants_has_zero_modulus() {
        let a = Cauchy::constant(r(1, 3));
        let b = Cauchy::constant(r(1, 6));
        let s = a.add(&b);
        assert_eq!(s.modulus(&r(1, 1_000_000)), 0);
        assert_eq!(s.approx(&r(1, 1_000_000)), r(1, 2));
    }

    #[test]
    fn alternating_harmonic_partial_sums_converge() {
        // x_n = sum_{k=1}^{n} (-1)^{k+1} / k. Limit is ln(2).
        // Each next term has magnitude 1/(n+1), so |x_{n+1} - x_n| < 1/(n+1).
        // For an alternating series with monotone-decreasing |a_k|,
        // |x_n - x_m| ≤ |a_{min(n,m)+1}|. So choosing N ≥ ceil(1/eps)
        // suffices: 1/(N+1) ≤ 1/N ≤ eps → N ≥ 1/eps.
        let nth = |n: u64| -> Rational {
            let mut acc = Rational::zero();
            for k in 1..=n {
                #[allow(clippy::cast_possible_wrap)]
                let term_num: i64 = if k % 2 == 1 { 1 } else { -1 };
                #[allow(clippy::cast_possible_wrap)]
                let denom: i64 = k as i64;
                acc = &acc + &r(term_num, denom);
            }
            acc
        };
        let modulus = |eps: &Rational| -> u64 {
            // N ≥ 1/eps ⇒ N = ceil(numer(1/eps) / denom(1/eps))
            // = ceil(eps.denom() / eps.numer()).
            let numer = eps.numer();
            let denom = eps.denom();
            // Handle zero or negative eps defensively.
            if numer.sign() == num_bigint::Sign::Minus || numer.sign() == num_bigint::Sign::NoSign
            {
                return u64::MAX;
            }
            // ceil_div(denom, numer)
            let q = (denom + numer - 1u32) / numer;
            // Best-effort conversion; for huge eps we cap.
            num_traits::ToPrimitive::to_u64(&q).unwrap_or(u64::MAX)
        };

        let seq = Cauchy::from_fn(nth, modulus);

        // Coarse approx: eps = 1/100. Limit ln(2) ≈ 0.6931.
        let coarse = seq.approx(&r(1, 100));
        // The Cauchy condition says |coarse - true| < 1/100 (in fact for
        // alternating monotone series the bound is also a one-sided
        // truncation error). Numerically:
        let ln2_approx = 0.693_147_180_559_945_3_f64;
        let coarse_f = coarse.to_f64_lossy().unwrap();
        assert!((coarse_f - ln2_approx).abs() < 0.05);
    }

    #[test]
    fn negation_preserves_modulus() {
        let v = Cauchy::constant(r(7, 11));
        let neg_v = v.neg();
        assert_eq!(neg_v.approx(&r(1, 1000)), r(-7, 11));
        assert_eq!(neg_v.modulus(&r(1, 1000)), v.modulus(&r(1, 1000)));
    }

    #[test]
    fn multiplication_requires_positive_bounds() {
        let a = Cauchy::constant(r(1, 2));
        let b = Cauchy::constant(r(2, 3));
        let zero = Rational::zero();
        let one = Rational::one();
        assert!(a.mul(&b, &zero, &one).is_none());
        assert!(a.mul(&b, &one, &zero).is_none());
    }

    /// **UOR shape — site count.** `CauchyShape::SITE_COUNT = 2`
    /// reflects the (term-stream, modulus) pair.
    #[test]
    fn cauchy_shape_site_count_is_two() {
        assert_eq!(<CauchyShape as ConstrainedTypeShape>::SITE_COUNT, 2);
    }

    /// **UOR shape — IRI carries the L0_State binding.**
    #[test]
    fn cauchy_shape_iri_anchors_l0_state() {
        let iri = <CauchyShape as ConstrainedTypeShape>::IRI;
        assert!(iri.ends_with("/L0_State"), "got {iri}");
    }

    /// **UOR shape — both Bound constraints reference the
    /// stream namespace.** This is the searchable anchor that lets a
    /// reviewer trace from the Phase 3 source back to the UOR
    /// `stream::ProductiveStream` and `stream::EpochBoundary` traits.
    #[test]
    fn cauchy_shape_bounds_reference_stream_namespace() {
        let cs = <CauchyShape as ConstrainedTypeShape>::CONSTRAINTS;
        assert_eq!(cs.len(), 2);
        let mut hit_productive = false;
        let mut hit_boundary = false;
        for c in cs {
            if let ConstraintRef::Bound { observable_iri, .. } = c {
                if *observable_iri == "https://uor.foundation/stream/ProductiveStream" {
                    hit_productive = true;
                }
                if *observable_iri == "https://uor.foundation/stream/EpochBoundary" {
                    hit_boundary = true;
                }
            }
        }
        assert!(hit_productive, "ProductiveStream binding missing");
        assert!(hit_boundary, "EpochBoundary binding missing");
    }

    #[test]
    fn product_of_constants_with_bounds() {
        let a = Cauchy::constant(r(2, 3));
        let b = Cauchy::constant(r(3, 4));
        let one = Rational::one();
        let prod = a.mul(&b, &one, &one).unwrap();
        assert_eq!(prod.approx(&r(1, 1000)), r(1, 2));
    }
}
