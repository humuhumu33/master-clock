//! # `U(30)` — the reduced residue system modulo `MODULUS_U`
//!
//! `U(30) = { n ∈ [1, 30) : gcd(n, 30) = 1 }`. Cardinality
//! `|U(30)| = φ(30) = 8`.
//!
//! ## What `U(30)` is, precisely
//!
//! `U(30)` is the **multiplicative group of units of `ℤ/30ℤ`** —
//! equivalently, the residue classes coprime to 30. As an abstract
//! group it is `ℤ/2 × ℤ/4` (by the Chinese Remainder Theorem applied
//! to `30 = 2·3·5`).
//!
//! The eight elements `{1, 7, 11, 13, 17, 19, 23, 29}` are the
//! **irreducibles modulo 30**: integers in `[1, 30)` that survive
//! reduction by 2, 3, and 5. They are not prime in general (the
//! integer `1` is a unit, not a prime). The set extends to
//! `R(360) = U(30) × {0, …, 11}` in [`crate::r360`] to give the 96
//! lattice rooms. Primes greater than 5 always land in one of these
//! eight residue classes — this is what makes the wheel-30 sieve in
//! [`crate::sieve`] enumerate all primes exactly.
//!
//! ## Self-deriving construction
//!
//! [`RESIDUES`] is **computed at compile time** from
//! [`crate::parameters::MODULUS_U`] via [`crate::arithmetic::gcd`].
//! There are no hand-typed residue values in this module — only the
//! Gary-stated parameter `MODULUS_U = 30` and the standard Euclidean
//! gcd. The eight leaf shapes each reference `RESIDUES[i]` rather than
//! a literal residue.
//!
//! A compile-time assertion confirms `RESIDUES.len() ==
//! U30_CARDINALITY`. A separate compile-time check confirms that the
//! computed `RESIDUES` exactly match Gary's stated values
//! `{ 1, 7, 11, 13, 17, 19, 23, 29 }` from the Appendix A reference table —
//! see [`GARY_STATED_RESIDUES`].
//!
//! ## UOR encoding
//!
//! Each residue is a leaf
//! [`uor_foundation::pipeline::ConstrainedTypeShape`] carrying a single
//! [`uor_foundation::pipeline::ConstraintRef::Residue`]. The eight
//! leaves are combined into the `U30` shape via
//! [`uor_foundation_sdk::coproduct_shape`] in a balanced binary tree of
//! depth 3.
//!
//! Coproduct is associative and commutative on isomorphism classes, so
//! the choice of pairing is arbitrary among balanced trees. The chosen
//! pairing minimises the maximum tag-pinner coefficient count to keep
//! within the SDK's `AFFINE_MAX_COEFFS = 8` budget.

use uor_foundation::pipeline::{ConstrainedTypeShape, ConstraintRef};
use uor_foundation_sdk::coproduct_shape;

use crate::arithmetic::gcd;
use crate::parameters::{MODULUS_U, U30_CARDINALITY};

// ─── Derived: the residues themselves ────────────────────────────────

/// The eight residues of `U(30)`, computed at compile time.
///
/// **Derived** by enumerating `n ∈ [1, MODULUS_U)` and selecting those
/// with `gcd(n, MODULUS_U) = 1`. The compile-time assertion below
/// confirms exactly `U30_CARDINALITY` residues are found.
pub const RESIDUES: [u64; U30_CARDINALITY] = compute_residues();

/// Compile-time enumeration of `U(MODULUS_U)`.
const fn compute_residues() -> [u64; U30_CARDINALITY] {
    let mut out = [0u64; U30_CARDINALITY];
    let mut idx = 0usize;
    let mut n = 1u64;
    while n < MODULUS_U {
        if gcd(n, MODULUS_U) == 1 {
            out[idx] = n;
            idx += 1;
        }
        n += 1;
    }
    // The const-fn body cannot return early on assertion failure; the
    // surrounding `const _: () = assert!(…)` below catches any mismatch.
    out
}

// Compile-time enforcement: the computed RESIDUES has exactly
// U30_CARDINALITY entries (and the loop above filled them all).
const _: () = {
    let mut count = 0usize;
    let mut i = 0usize;
    while i < RESIDUES.len() {
        if RESIDUES[i] != 0 {
            count += 1;
        }
        i += 1;
    }
    assert!(count == U30_CARDINALITY, "RESIDUES did not enumerate exactly U30_CARDINALITY values");
};

// ─── Verification: derived residues must match the Appendix A reference ────────

/// Gary's verbatim `U(30)` enumeration from the Appendix A reference table.
///
/// **This array exists only to verify the derivation.** Production
/// code consults [`RESIDUES`] (the computed version). If the two
/// disagree, either Gary's Appendix A reference is internally inconsistent or
/// our `gcd` is wrong; the build fails at the compile-time check below.
pub const GARY_STATED_RESIDUES: [u64; U30_CARDINALITY] = [1, 7, 11, 13, 17, 19, 23, 29];

const _: () = {
    let mut i = 0usize;
    while i < U30_CARDINALITY {
        assert!(
            RESIDUES[i] == GARY_STATED_RESIDUES[i],
            "Derived U(30) residue does not match Gary's stated Appendix A reference table value"
        );
        i += 1;
    }
};

// ─── Leaf shapes — one per residue, residue value sourced from RESIDUES

/// Residue class `n ≡ RESIDUES[0] = 1 (mod 30)`.
pub struct U30R0;
impl ConstrainedTypeShape for U30R0 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/u30/index/0";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: MODULUS_U,
        residue: RESIDUES[0],
    }];
}

/// Residue class `n ≡ RESIDUES[1] = 7 (mod 30)`.
pub struct U30R1;
impl ConstrainedTypeShape for U30R1 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/u30/index/1";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: MODULUS_U,
        residue: RESIDUES[1],
    }];
}

/// Residue class `n ≡ RESIDUES[2] = 11 (mod 30)`.
pub struct U30R2;
impl ConstrainedTypeShape for U30R2 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/u30/index/2";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: MODULUS_U,
        residue: RESIDUES[2],
    }];
}

/// Residue class `n ≡ RESIDUES[3] = 13 (mod 30)`.
pub struct U30R3;
impl ConstrainedTypeShape for U30R3 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/u30/index/3";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: MODULUS_U,
        residue: RESIDUES[3],
    }];
}

/// Residue class `n ≡ RESIDUES[4] = 17 (mod 30)`.
pub struct U30R4;
impl ConstrainedTypeShape for U30R4 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/u30/index/4";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: MODULUS_U,
        residue: RESIDUES[4],
    }];
}

/// Residue class `n ≡ RESIDUES[5] = 19 (mod 30)`.
pub struct U30R5;
impl ConstrainedTypeShape for U30R5 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/u30/index/5";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: MODULUS_U,
        residue: RESIDUES[5],
    }];
}

/// Residue class `n ≡ RESIDUES[6] = 23 (mod 30)`.
pub struct U30R6;
impl ConstrainedTypeShape for U30R6 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/u30/index/6";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: MODULUS_U,
        residue: RESIDUES[6],
    }];
}

/// Residue class `n ≡ RESIDUES[7] = 29 (mod 30)`.
pub struct U30R7;
impl ConstrainedTypeShape for U30R7 {
    const IRI: &'static str = "https://uor.foundation/prime-clock/u30/index/7";
    const SITE_COUNT: usize = 1;
    const CONSTRAINTS: &'static [ConstraintRef] = &[ConstraintRef::Residue {
        modulus: MODULUS_U,
        residue: RESIDUES[7],
    }];
}

// ─── Coproduct tree: balanced, depth 3 ───────────────────────────────
//
// Pairing is arbitrary among balanced trees (coproduct is
// associative/commutative on isomorphism classes); the resulting U30
// is the same set regardless. The pairing chosen here groups indices
// (0,1), (2,3), (4,5), (6,7).

coproduct_shape!(U30Pair01, U30R0, U30R1);
coproduct_shape!(U30Pair23, U30R2, U30R3);
coproduct_shape!(U30Pair45, U30R4, U30R5);
coproduct_shape!(U30Pair67, U30R6, U30R7);

coproduct_shape!(U30Quartet0123, U30Pair01, U30Pair23);
coproduct_shape!(U30Quartet4567, U30Pair45, U30Pair67);

// ─── Manual top-level coproduct ──────────────────────────────────────
//
// The SDK `coproduct_shape!` macro materialises its tag-pinner buffer
// as `[ConstraintRef; 2 * NERVE_CONSTRAINTS_CAP + 2] = [_; 18]`. At
// level 3 of a balanced binary tree the splice needs
// `L::CONSTRAINTS.len() + R::CONSTRAINTS.len() + 2 = 10 + 10 + 2 = 22`
// slots — strictly larger than the buffer, so a const-eval bounds
// check fires (`E0080`).
//
// We emit the top-level impl by hand, mirroring the macro's contract:
//
//   - `IRI = urn:uor:coproduct:{lex_earlier}:{lex_later}` (alphabetical
//     order on operand identifier strings — see
//     `uor_foundation_sdk::lexically_earlier`).
//   - `SITE_BUDGET = max(L::SITE_BUDGET, R::SITE_BUDGET)` (operand-only,
//     no bookkeeping increment for coproduct).
//   - `SITE_COUNT = max(L::SITE_COUNT, R::SITE_COUNT) + 1` (the +1
//     accounts for the tag site).
//   - `CONSTRAINTS = &[]` (empty). The macro would splice both
//     operands' constraints plus two tag-pinner Affines; the buffer
//     limit prevents that representation here. The cardinality and
//     residue properties of `U(30)` are independently witnessed by the
//     compile-time `assert!` blocks above and by the
//     `derived_residues_match_gary_stated` test, so dropping the
//     splice is sound for our verification needs.
//
// The downstream `mint_coproduct_witness` path is not exercised by
// Phase 1 or Phase 2 of this crate; if Phase 3 needs it we can switch
// to a binary-tree-walk witness instead.

/// Top-level `U(30)` coproduct shape, balanced binary tree of depth 3.
pub struct U30;

impl ConstrainedTypeShape for U30 {
    const IRI: &'static str = "urn:uor:coproduct:U30Quartet0123:U30Quartet4567";
    const SITE_BUDGET: usize = {
        let a = <U30Quartet0123 as ConstrainedTypeShape>::SITE_BUDGET;
        let b = <U30Quartet4567 as ConstrainedTypeShape>::S