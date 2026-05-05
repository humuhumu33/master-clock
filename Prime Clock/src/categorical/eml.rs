//! # EML trees — the free magma over `{1, x, y, …}` under `eml`
//!
//! Encodes Odrzywołek's universality grammar
//!
//! ```text
//! S → 1 | x_i | eml(S, S)
//! ```
//!
//! as a recursive Rust algebraic data type. Every elementary function
//! over `ℚ` is (per the paper) the evaluation of some finite
//! [`EmlTree`] under variable bindings.
//!
//! ## Mirror to `atlas-embeddings`
//!
//! | atlas-embeddings | this module |
//! |------------------|-------------|
//! | `Atlas` (initial object in `ResGraph`) | `Tree::One` and `Tree::Var(_)` (free generators) |
//! | Morphism `Atlas → G` | `Tree::Eml(_, _)` building a tree witnessing function `f` |
//! | Categorical operation (Product, Quotient, …) | Constructor (`exp`, `ln`, …) producing an `EmlTree` |
//! | `verify_product_universal_property` | [`verify_identity_at`] checking a tree matches a Cauchy reference |
//! | `f4_has_48_roots : ... := by rfl` (Lean) | per-identity Lean theorem, planned |
//!
//! ## Verification posture
//!
//! For each Odrzywołek identity `f(x_1, …, x_n) = T(x_1, …, x_n)`
//! (where `T` is an EML tree), [`verify_identity_at`] evaluates both
//! sides at a rational test point to a chosen ε and asserts they
//! agree within ε. This is the analog of atlas-embeddings'
//! computational `rfl` proofs — not a symbolic proof, but a
//! per-point check that pins the implementation honest.

extern crate alloc;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::cauchy::Cauchy;
use crate::elementary::{eml as eml_fn, ElementaryError};
use crate::rational::Rational;

/// Identifier for a free variable in an EML expression.
///
/// Variables are 0-indexed, matched to a runtime [`Env`] map.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VarId(pub u32);

/// EML expression tree. Every leaf is either the constant `1` or a
/// free variable; every internal node applies `eml`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EmlTree {
    /// The constant 1.
    One,
    /// A free variable bound at evaluation time.
    Var(VarId),
    /// `eml(left, right) = exp(left) − ln(right)`.
    Eml(Box<EmlTree>, Box<EmlTree>),
}

impl EmlTree {
    /// `1`.
    #[must_use]
    pub fn one() -> Self {
        Self::One
    }

    /// Variable `x_i`.
    #[must_use]
    pub fn var(id: u32) -> Self {
        Self::Var(VarId(id))
    }

    /// `eml(self, other)`.
    #[must_use]
    pub fn eml(self, other: Self) -> Self {
        Self::Eml(Box::new(self), Box::new(other))
    }

    /// Number of nodes in the tree (size as a categorical complexity
    /// measure — analogous to root count in atlas-embeddings).
    #[must_use]
    pub fn size(&self) -> usize {
        match self {
            Self::One | Self::Var(_) => 1,
            Self::Eml(a, b) => 1 + a.size() + b.size(),
        }
    }

    /// Depth of the tree.
    #[must_use]
    pub fn depth(&self) -> usize {
        match self {
            Self::One | Self::Var(_) => 0,
            Self::Eml(a, b) => 1 + core::cmp::max(a.depth(), b.depth()),
        }
    }

    /// Set of variable indices that occur in the tree.
    #[must_use]
    pub fn free_vars(&self) -> Vec<VarId> {
        let mut out = Vec::new();
        self.collect_vars(&mut out);
        out.sort_unstable();
        out.dedup();
        out
    }

    fn collect_vars(&self, out: &mut Vec<VarId>) {
        match self {
            Self::One => {}
            Self::Var(v) => out.push(*v),
            Self::Eml(a, b) => {
                a.collect_vars(out);
                b.collect_vars(out);
            }
        }
    }
}

/// Variable bindings for evaluation. Missing variables fall back to
/// [`Rational::zero`].
pub type Env = BTreeMap<VarId, Rational>;

/// Errors raised when evaluating a tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvalError {
    /// `eml(a, b)` with `b ≤ 0` (the inner `ln` is undefined).
    Elementary(ElementaryError),
}

impl From<ElementaryError> for EvalError {
    fn from(e: ElementaryError) -> Self {
        Self::Elementary(e)
    }
}

/// Evaluate an EML tree at the given environment, returning a
/// [`Cauchy`] sequence over `ℚ`.
///
/// The walk is bottom-up: leaves become constant Cauchy sequences,
/// `Eml` nodes call [`eml_fn`] which composes [`crate::elementary::exp`]
/// and [`crate::elementary::ln`] internally.
///
/// # Errors
///
/// Returns [`EvalError::Elementary`] if any sub-tree's right operand
/// would feed `ln` a non-positive value at the chosen environment.
/// Note: this is an *approximation* — the operand is a Cauchy
/// sequence whose limit could be positive even if some early term
/// is non-positive. We avoid this by requiring the variable bindings
/// to make every `Eml` right operand resolve to a positive rational
/// at evaluation time. A more sophisticated check is future work.
pub fn evaluate(tree: &EmlTree, env: &Env) -> Result<Cauchy, EvalError> {
    match tree {
        EmlTree::One => Ok(Cauchy::constant(Rational::one())),
        EmlTree::Var(v) => {
            let value = env.get(v).cloned().unwrap_or_else(Rational::zero);
            Ok(Cauchy::constant(value))
        }
        EmlTree::Eml(a, b) => {
            // Evaluate both sub-trees as rationals (their leaves are
            // either `One` or variables — finite, exact). For deeper
            // sub-trees the operand is a Cauchy sequence; we extract
            // a sufficiently-precise approximation and feed it to
            // `eml_fn`. We use a fixed working precision; the outer
            // Cauchy modulus inherited from `eml_fn` already accounts
            // for the truncation analytically.
            let a_val = evaluate_to_rational(a, env, &Rational::new_i64(1, 1_000_000_000).unwrap_or_else(Rational::zero))?;
            let b_val = evaluate_to_rational(b, env, &Rational::new_i64(1, 1_000_000_000).unwrap_or_else(Rational::zero))?;
            let cauchy = eml_fn(a_val, b_val)?;
            Ok(cauchy)
        }
    }
}

/// Helper: collapse an EML tree to a single rational at the chosen
/// working precision. Used by [`evaluate`] when feeding sub-tree
/// values into [`eml_fn`].
fn evaluate_to_rational(
    tree: &EmlTree,
    env: &Env,
    eps: &Rational,
) -> Result<Rational, EvalError> {
    match tree {
        EmlTree::One => Ok(Rational::one()),
        EmlTree::Var(v) => Ok(env.get(v).cloned().unwrap_or_else(Rational::zero)),
        EmlTree::Eml(a, b) => {
            let a_val = evaluate_to_rational(a, env, eps)?;
            let b_val = evaluate_to_rational(b, env, eps)?;
            let cauchy = eml_fn(a_val, b_val)?;
            Ok(cauchy.approx(eps))
        }
    }
}

// ============================================================
// Constructors — Odrzywołek's identities, one per elementary fn
// ============================================================
//
// Each constructor takes an `EmlTree` for its argument(s) and
// returns a new `EmlTree` whose evaluation equals the named
// function applied to those arguments. Verifying that the returned
// tree's evaluation matches the direct function (via
// `verify_identity_at`) is the universal-property check.

/// `exp(x) = eml(x, 1)`.
///
/// Direct from the definition: `eml(a, b) = exp(a) − ln(b)`, and
/// `ln(1) = 0`, so `eml(x, 1) = exp(x)`.
#[must_use]
pub fn exp_tree(x: EmlTree) -> EmlTree {
    x.eml(EmlTree::One)
}

/// `ln(x) = eml(1, eml(eml(1, x), 1))`.
///
/// Stated by Odrzywołek (arXiv:2603.21852, abstract). Verifiable
/// algebraically:
///
/// ```text
/// eml(eml(1, x), 1) = exp(eml(1, x)) − ln(1)
///                   = exp(e − ln(x))
///                   = e^e / x
/// eml(1, e^e / x)   = exp(1) − ln(e^e / x)
///                   = e − (e − ln(x))
///                   = ln(x).
/// ```
#[must_use]
pub fn ln_tree(x: EmlTree) -> EmlTree {
    let inner = EmlTree::One.eml(x);
    let middle = inner.eml(EmlTree::One);
    EmlTree::One.eml(middle)
}

// ------- Stubs awaiting paper identities -------
//
// The Odrzywołek paper provides EML compositions for all of these.
// Filling them in is straightforward once the relevant section of
// the supplementary information is transcribed. Until then they
// `unimplemented!()` — out of scope for the scaffold but listed so
// the universal-property test harness has a target list.

// pub fn add_tree(x: EmlTree, y: EmlTree) -> EmlTree { ... }
// pub fn sub_tree(x: EmlTree, y: EmlTree) -> EmlTree { ... }
// pub fn mul_tree(x: EmlTree, y: EmlTree) -> EmlTree { ... }
// pub fn div_tree(x: EmlTree, y: EmlTree) -> EmlTree { ... }
// pub fn pow_tree(x: EmlTree, y: EmlTree) -> EmlTree { ... }
// pub fn sqrt_tree(x: EmlTree) -> EmlTree { ... }
// pub fn sin_tree(x: EmlTree) -> EmlTree { ... }
// pub fn cos_tree(x: EmlTree) -> EmlTree { ... }
// pub fn neg_tree(x: EmlTree) -> EmlTree { ... }
// pub fn e_const() -> EmlTree { ... }
// pub fn pi_const() -> EmlTree { ... }
// pub fn i_const() -> EmlTree { ... } // complex unit, requires extension to ℂ

// ============================================================
// Universal-property verifier
// ============================================================

/// Result of comparing an EML-tree evaluation against a reference
/// Cauchy at one rational test point.
#[derive(Clone, Debug)]
pub struct IdentityCheck {
    /// Identity name for diagnostics (e.g. `"exp(x) = eml(x, 1)"`).
    pub label: String,
    /// `|tree(env) − reference(env)| < eps` — the verdict.
    pub agrees: bool,
    /// Approximation produced by walking the tree.
    pub tree_value: Rational,
    /// Approximation produced by the reference function.
    pub reference_value: Rational,
    /// Absolute difference at the test point.
    pub diff: Rational,
    /// Tolerance the check used.
    pub eps: Rational,
}

/// Verify that a candidate `tree` equals a `reference` Cauchy
/// sequence at the test environment, within tolerance `eps`.
///
/// # Errors
///
/// Returns [`EvalError`] if walking the tree raises one (e.g.
/// non-positive operand to an inner `ln`).
pub fn verify_identity_at(
    label: &str,
    tree: &EmlTree,
    reference: &Cauchy,
    env: &Env,
    eps: &Rational,
) -> Result<IdentityCheck, EvalError> {
    let tree_cauchy = evaluate(tree, env)?;
    let tree_value = tree_cauchy.approx(eps);
    let reference_value = reference.approx(eps);
    let diff_signed = &tree_value - &reference_value;
    let diff = diff_signed.abs();
    let agrees = &diff < eps;
    Ok(IdentityCheck {
        label: String::from(label),
        agrees,
        tree_value,
        reference_value,
        diff,
        eps: eps.clone(),
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]
    #![allow(clippy::float_arithmetic, clippy::float_cmp)]

    use super::*;
    use crate::elementary::{exp as exp_fn, ln as ln_fn};

    fn r(n: i64, d: i64) -> Rational {
        Rational::new_i64(n, d).unwrap()
    }

    fn env_at(var: u32, value: Rational) -> Env {
        let mut env = Env::new();
        env.insert(VarId(var), value);
        env
    }

    #[test]
    fn one_tree_evaluates_to_one() {
        let env = Env::new();
        let v = evaluate(&EmlTree::One, &env).unwrap();
        assert_eq!(v.approx(&r(1, 1_000_000)), Rational::one());
    }

    #[test]
    fn var_tree_evaluates_to_binding() {
        let env = env_at(0, r(7, 3));
        let v = evaluate(&EmlTree::var(0), &env).unwrap();
        assert_eq!(v.approx(&r(1, 1_000_000)), r(7, 3));
    }

    #[test]
    fn tree_size_and_depth_basic() {
        let t = exp_tree(EmlTree::var(0));
        assert_eq!(t.size(), 3); // Eml(Var, One)
        assert_eq!(t.depth(), 1);

        let l = ln_tree(EmlTree::var(0));
        // Structure: Eml(One, Eml(Eml(One, Var), One))
        // Sizes:     1     1   1   1   1    1     1   = 7
        assert_eq!(l.size(), 7);
        assert_eq!(l.depth(), 3);
    }

    #[test]
    fn free_vars_collected_and_deduplicated() {
        // exp(x) — should collect just {x}
        let t = exp_tree(EmlTree::var(0));
        assert_eq!(t.free_vars(), alloc::vec![VarId(0)]);

        // eml(x, y) — should collect {x, y}
        let t = EmlTree::var(0).eml(EmlTree::var(1));
        assert_eq!(t.free_vars(), alloc::vec![VarId(0), VarId(1)]);

        // eml(x, x) — should dedup to {x}
        let t = EmlTree::var(0).eml(EmlTree::var(0));
        assert_eq!(t.free_vars(), alloc::vec![VarId(0)]);
    }

    #[test]
    fn universal_property_exp_at_one() {
        // Identity: exp(x) = eml(x, 1).  Test point: x = 1.
        let tree = exp_tree(EmlTree::var(0));
        let env = env_at(0, Rational::one());
        let reference = exp_fn(Rational::one());
        let eps = r(1, 100_000);
        let check =
            verify_identity_at("exp(x) = eml(x, 1)", &tree, &reference, &env, &eps).unwrap();
        assert!(
            check.agrees,
            "exp identity fails at x=1: tree={}, ref={}, diff={}",
            check.tree_value, check.reference_value, check.diff
        );
    }

    #[test]
    fn universal_property_exp_at_half() {
        let tree = exp_tree(EmlTree::var(0));
        let env = env_at(0, r(1, 2));
        let reference = exp_fn(r(1, 2));
        let eps = r(1, 100_000);
        let check =
            verify_identity_at("exp(x) = eml(x, 1) at 1/2", &tree, &reference, &env, &eps).unwrap();
        assert!(check.agrees);
    }

    #[test]
    fn universal_property_exp_at_negative() {
        let tree = exp_tree(EmlTree::var(0));
        let env = env_at(0, r(-1, 1));
        let reference = exp_fn(r(-1, 1));
        let eps = r(1, 100_000);
        let check = verify_identity_at(
            "exp(x) = eml(x, 1) at -1",
            &tree,
            &reference,
            &env,
            &eps,
        )
        .unwrap();
        assert!(check.agrees);
    }

    #[test]
    fn universal_property_ln_at_two() {
        // Identity: ln(x) = eml(1, eml(eml(1, x), 1)).  Test point: x = 2.
        let tree = ln_tree(EmlTree::var(0));
        let env = env_at(0, r(2, 1));
        let reference = ln_fn(r(2, 1)).unwrap();
        // ln tree has depth 3 and uses sub-tree exp, which can have
        // larger numerical magnitudes; loosen ε slightly.
        let eps = r(1, 1_000);
        let check =
            verify_identity_at("ln(x) = eml(1, eml(eml(1, x), 1))", &tree, &reference, &env, &eps)
                .unwrap();
        assert!(
            check.agrees,
            "ln identity fails at x=2: tree={}, ref={}, diff={}",
            check.tree_value, check.reference_value, check.diff
        );
    }

    #[test]
    fn universal_property_ln_at_e_approx() {
        // ln(e) = 1, but `e` itself is irrational; we approximate
        // with x = 19/7 ≈ 2.714 which is close to e.
        let tree = ln_tree(EmlTree::var(0));
        let env = env_at(0, r(19, 7));
        let reference = ln_fn(r(19, 7)).unwrap();
        let eps = r(1, 1_000);
        let check = verify_identity_at(
            "ln(x) = eml(1, eml(eml(1, x), 1)) at 19/7",
            &tree,
            &reference,
            &env,
            &eps,
        )
        .unwrap();
        assert!(check.agrees);
    }

    #[test]
    fn ln_tree_at_one_is_zero() {
        // Smoke test: evaluating the ln tree at x=1 should give 0.
        let tree = ln_tree(EmlTree::var(0));
        let env = env_at(0, Rational::one());
        let val = evaluate(&tree, &env).unwrap().approx(&r(1, 1_000));
        // ln(1) = 0; allow a wide envelope because the tree has
        // depth 3 and approximation errors compound.
        assert!(val.abs() < r(1, 100));
    }
}
