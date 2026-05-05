//! # Layer 4 — Exact rationals `ℚ` (alloc-only)
//!
//! Thin wrapper around [`num_rational::BigRational`] that pins the
//! shape of the construction to UOR's `cartesian_product_shape!`
//! pattern: a [`Rational`] is the pair `(num: BigInt, denom:
//! NonZeroBigInt)`. The wrapper keeps Rust's standard arithmetic
//! traits but disables every operation that would lose exactness
//! (no `f64`, no truncation, no rounding).
//!
//! ## Place in the eight-layer bridge
//!
//! ```text
//! Layer 0:   neg(bnot(x)) = x + 1                 [ring_substrate]
//! Layer 1:   Succ_L : W_L → W_L                   [ring_substrate]
//! Layer 2:   add, sub, mul, mod, gcd on ℤ/2ⁿ      [arithmetic]
//! Layer 3:   ℤ at arbitrary precision             [num_bigint::BigInt]
//! Layer 4:   ℚ                                    [THIS MODULE]
//! Layer 5:   Computable ℝ (Cauchy<Rational>)      [cauchy]
//! Layer 6:   exp, ln                              [elementary]
//! Layer 7:   eml(x, y) = exp(x) − ln(y)           [elementary]
//! ```
//!
//! Every operation here delegates to `num-rational`, which derives
//! from `num-bigint`'s `BigInt` arithmetic, which derives from `u64`
//! limb arithmetic, which is two's-complement and thus governed by
//! `Succ = Neg ∘ BNot`. Closure on the axiom is preserved.
//!
//! ## UOR shape encoding (Phase 3 O1)
//!
//! The value type [`Rational`] coexists with three zero-sized shape
//! markers — [`RationalNumShape`], [`RationalDenomShape`],
//! [`RationalShape`] — that supply the type-level encoding required
//! by [`uor_foundation::pipeline::ConstrainedTypeShape`]. The shape
//! markers do not carry data; they are pure ontology anchors so that
//! `RationalShape::IRI` and `RationalDenomShape::CONSTRAINTS` are
//! grep-walkable from the project root straight back to UOR's
//! pipeline.

use core::cmp::Ordering;
use core::fmt;
use core::ops::{Add, Mul, Neg, Sub};

extern crate alloc;
use alloc::string::String;

use num_bigint::BigInt;
use num_rational::BigRational;
use num_traits::{One, Pow, Signed, ToPrimitive, Zero};

use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef};
use uor_foundation_sdk::cartesian_product_shape;

use crate::ONTOLOGY_PREFIX;

/// Exact rational number on `ℚ`.
///
/// Internally a fraction `num / denom` with `denom > 0` and `gcd(|num|, denom) = 1`.
/// All arithmetic is exact; no operation in this module returns an approximation.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Rational(BigRational);

impl Rational {
    /// `0 / 1`.
    #[must_use]
    pub fn zero() -> Self {
        Self(BigRational::zero())
    }

    /// `1 / 1`.
    #[must_use]
    pub fn one() -> Self {
        Self(BigRational::one())
    }

    /// Build from a signed integer numerator and denominator.
    ///
    /// Returns `None` if `denom == 0`. Otherwise normalises to lowest
    /// terms with positive denominator.
    #[must_use]
    pub fn new_i64(num: i64, denom: i64) -> Option<Self> {
        if denom == 0 {
            return None;
        }
        Some(Self(BigRational::new(BigInt::from(num), BigInt::from(denom))))
    }

    /// Build from arbitrary `BigInt` numerator and denominator.
    ///
    /// Returns `None` if `denom == 0`.
    #[must_use]
    pub fn new(num: BigInt, denom: BigInt) -> Option<Self> {
        if denom.is_zero() {
            return None;
        }
        Some(Self(BigRational::new(num, denom)))
    }

    /// Build from a single integer (denominator = 1).
    #[must_use]
    pub fn from_int(num: i64) -> Self {
        Self(BigRational::from_integer(BigInt::from(num)))
    }

    /// Build from a `BigInt` (denominator = 1).
    #[must_use]
    pub fn from_bigint(num: BigInt) -> Self {
        Self(BigRational::from_integer(num))
    }

    /// Numerator (signed).
    #[must_use]
    pub fn numer(&self) -> &BigInt {
        self.0.numer()
    }

    /// Denominator (always positive, always coprime with numerator).
    #[must_use]
    pub fn denom(&self) -> &BigInt {
        self.0.denom()
    }

    /// Sign: `-1`, `0`, or `+1`.
    #[must_use]
    pub fn signum(&self) -> i32 {
        if self.0.is_zero() {
            0
        } else if self.0.is_negative() {
            -1
        } else {
            1
        }
    }

    /// Absolute value `|x|`.
    #[must_use]
    pub fn abs(&self) -> Self {
        Self(self.0.abs())
    }

    /// Exact division. Returns `None` iff `other == 0`.
    #[must_use]
    pub fn checked_div(&self, other: &Self) -> Option<Self> {
        if other.0.is_zero() {
            None
        } else {
            Some(Self(&self.0 / &other.0))
        }
    }

    /// Reciprocal `1 / self`. Returns `None` iff `self == 0`.
    #[must_use]
    pub fn recip(&self) -> Option<Self> {
        if self.0.is_zero() {
            None
        } else {
            Some(Self(self.0.recip()))
        }
    }

    /// Integer power `self^n` for `n: u32`. Always exact.
    #[must_use]
    pub fn pow_u32(&self, n: u32) -> Self {
        Self(Pow::pow(&self.0, n))
    }

    /// Floor `⌊self⌋` as a `BigInt`.
    #[must_use]
    pub fn floor(&self) -> BigInt {
        self.0.floor().to_integer()
    }

    /// Approximate as `f64`. **Lossy** — only for display / debugging.
    /// Production code must never depend on this.
    #[must_use]
    pub fn to_f64_lossy(&self) -> Option<f64> {
        self.0.to_f64()
    }

    /// `true` iff `|self| < other` with both arguments rationals.
    #[must_use]
    pub fn abs_less_than(&self, other: &Self) -> bool {
        self.abs().0 < other.0
    }

    /// Render in `num/denom` form.
    #[must_use]
    pub fn to_fraction_string(&self) -> String {
        // No format!() in no_std without alloc; we have alloc.
        alloc::format!("{}/{}", self.0.numer(), self.0.denom())
    }
}

// ---------- Arithmetic traits ----------
// Each delegates to num_rational which preserves exactness. We
// implement &T op &T as the canonical form to avoid clones; T op T,
// T op &T, &T op T are convenience wrappers.

impl Add for &Rational {
    type Output = Rational;
    fn add(self, rhs: Self) -> Rational {
        Rational(&self.0 + &rhs.0)
    }
}
impl Sub for &Rational {
    type Output = Rational;
    fn sub(self, rhs: Self) -> Rational {
        Rational(&self.0 - &rhs.0)
    }
}
impl Mul for &Rational {
    type Output = Rational;
    fn mul(self, rhs: Self) -> Rational {
        Rational(&self.0 * &rhs.0)
    }
}
impl Neg for &Rational {
    type Output = Rational;
    fn neg(self) -> Rational {
        Rational(-&self.0)
    }
}

impl Add for Rational {
    type Output = Rational;
    fn add(self, rhs: Self) -> Rational {
        Rational(self.0 + rhs.0)
    }
}
impl Sub for Rational {
    type Output = Rational;
    fn sub(self, rhs: Self) -> Rational {
        Rational(self.0 - rhs.0)
    }
}
impl Mul for Rational {
    type Output = Rational;
    fn mul(self, rhs: Self) -> Rational {
        Rational(self.0 * rhs.0)
    }
}
impl Neg for Rational {
    type Output = Rational;
    fn neg(self) -> Rational {
        Rational(-self.0)
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Rational {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.denom().is_one() {
            write!(f, "{}", self.0.numer())
        } else {
            write!(f, "{}/{}", self.0.numer(), self.0.denom())
        }
    }
}

impl From<i64> for Rational {
    fn from(n: i64) -> Self {
        Self::from_int(n)
    }
}

impl From<BigInt> for Rational {
    fn from(n: BigInt) -> Self {
        Self::from_bigint(n)
    }
}

// ─── UOR shape encoding (Phase 3 O1) ─────────────────────────────────
//
// `Rational` is the **value** type — a fraction with reduced
// representative. The **shape** of `Rational` in UOR's typed-data
// pipeline is the cartesian product `BigInt × NonZeroBigInt`, encoded
// here by the three zero-sized markers below. This keeps the value
// type free of trait obligations while giving the type-level encoding
// a name that downstream layers can reference.
//
// Each marker carries a `ConstraintRef::Bound` citing a local
// prime-clock ontology IRI. UOR's preflight validator treats unknown
// `Bound` shape IRIs as opaque (admission-passes) but records them in
// the trace, which is the right semantics for our additive ontology
// extensions.

/// Numerator slot of a `Rational` — an arbitrary-precision integer.
///
/// Layer 3 of the BRIDGE.md eight-layer stack. The shape carries no
/// constraint because every `BigInt` is valid by construction.
pub struct RationalNumShape;

impl ConstrainedTypeShape for RationalNumShape {
    const IRI: &'static str = "https://uor.foundation/prime-clock/rational/num";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[];
}

/// Denominator slot of a `Rational` — an arbitrary-precision integer
/// with the **nonzero** semantic obligation.
///
/// Layer 4 boundary of the BRIDGE.md eight-layer stack. Carries a
/// `ConstraintRef::Bound` citing the prime-clock-local term
/// `https://uor.foundation/prime-clock/rational/nonzero`, which the
/// preflight validator (when extended) reads as "this site holds a
/// value `≠ 0`."
pub struct RationalDenomShape;

impl ConstrainedTypeShape for RationalDenomShape {
    const IRI: &'static str = "https://uor.foundation/prime-clock/rational/denom";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Bound {
        observable_iri: "https://uor.foundation/prime-clock/rational/sign",
        bound_shape_iri: "https://uor.foundation/prime-clock/rational/nonzero",
        args_repr: "denom != 0",
    }];
}

cartesian_product_shape!(RationalShape, RationalNumShape, RationalDenomShape);

const _: () = {
    // Cite ONTOLOGY_PREFIX so the canonical-prefix invariant is
    // visible to readers grep-walking the source.
    let _ = ONTOLOGY_PREFIX;
};

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    fn r(n: i64, d: i64) -> Rational {
        Rational::new_i64(n, d).unwrap()
    }

    #[test]
    fn ring_axioms() {
        let a = r(1, 2);
        let b = r(1, 3);
        let c = r(1, 6);
        // associativity
        assert_eq!(&(&a + &b) + &c, &a + &(&b + &c));
        // commutativity
        assert_eq!(&a + &b, &b + &a);
        // multiplicative inverse
        assert_eq!(a.recip().unwrap(), r(2, 1));
    }

    #[test]
    fn zero_division_returns_none() {
        let a = r(3, 4);
        let zero = Rational::zero();
        assert!(a.checked_div(&zero).is_none());
        assert!(zero.recip().is_none());
    }

    #[test]
    fn power_is_exact() {
        let half = r(1, 2);
        assert_eq!(half.pow_u32(10), r(1, 1024));
    }

    #[test]
    fn comparison_total_order() {
        let a = r(2, 3);
        let b = r(3, 4);
        assert!(a < b);
        assert!(b > a);
        assert_eq!(a, r(2, 3));
    }

    #[test]
    fn numer_denom_are_coprime() {
        let q = r(6, 8); // should reduce to 3/4
        assert_eq!(*q.numer(), BigInt::from(3));
        assert_eq!(*q.denom(), BigInt::from(4));
    }

    /// **UOR shape — site arithmetic.** `RationalShape::SITE_COUNT`
    /// equals `RationalNumShape::SITE_COUNT + RationalDenomShape::SITE_COUNT`
    /// because cartesian product is additive on site counts.
    #[test]
    fn rational_shape_site_count_is_two() {
        assert_eq!(<RationalNumShape as ConstrainedTypeShape>::SITE_COUNT, 1);
        assert_eq!(<RationalDenomShape as ConstrainedTypeShape>::SITE_COUNT, 1);
        assert_eq!(<RationalShape as ConstrainedTypeShape>::SITE_COUNT, 2);
    }

    /// **UOR shape — IRI canonicality.** `RationalShape` is emitted
    /// via `cartesian_product_shape!`, so its IRI starts with the
    /// SDK-canonical `urn:uor:cartesian:` prefix.
    #[test]
    fn rational_shape_iri_is_canonical_cartesian() {
        let iri = <RationalShape as ConstrainedTypeShape>::IRI;
        assert!(iri.starts_with("urn:uor:cartesian:"), "got {iri}");
    }

    /// **UOR shape — denom carries the nonzero bound.** The
    /// denominator leaf shape carries a single `ConstraintRef::Bound`
    /// whose `bound_shape_iri` names the nonzero predicate.
    #[test]
    fn rational_denom_has_nonzero_bound() {
        let cs = <RationalDenomShape as ConstrainedTypeShape>::CONSTRAINTS;
        assert_eq!(cs.len(), 1);
        match cs[0] {
            ConstraintRef::Bound { bound_shape_iri, .. } => {
                assert_eq!(
                    bound_shape_iri,
                    "https://uor.foundation/prime-clock/rational/nonzero"
                );
            }
            _ => panic!("expected ConstraintRef::Bound for denom"),
        }
    }

    /// **UOR shape — numerator is unconstrained.** Every `BigInt`
    /// is a valid numerator; no preflight constraint is required.
    #[test]
    fn rational_num_is_unconstrained() {
        let cs = <RationalNumShape as ConstrainedTypeShape>::CONSTRAINTS;
        assert!(cs.is_empty(), "numerator should carry no constraints");
    }
}
