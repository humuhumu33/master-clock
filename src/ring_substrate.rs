//! # Ring substrate — anchoring arithmetic in the UOR algebraic axiom.
//!
//! This module is the bottom layer of the crate. Every integer
//! operation used downstream (`gcd`, `dr`, Fibonacci, modular
//! reduction, residue indexing) ultimately reduces to operations on
//! a finite ring `ℤ/2^n ℤ` for some Witt level `n`. The UOR Foundation
//! formalises this ring at the type level via three primitive unary
//! operations and one identity that ties them together.
//!
//! ## The critical composition law (the axiom)
//!
//! For any Witt level `L`,
//!
//! ```text
//! Succ_L = Neg_L ∘ BNot_L
//! ```
//!
//! Equivalently, in two's-complement integer arithmetic at width `n`,
//!
//! ```text
//! −(¬x) = x + 1   (mod 2^n)
//! ```
//!
//! This is the bridge between the **bitwise (Boolean) ring** and the
//! **arithmetic (additive) ring**. From this single identity one can
//! derive that `ℤ/2^n ℤ` is closed under the standard ring operations
//! and that bitwise NOT is an arithmetic-meaningful operation rather
//! than just a logical one.
//!
//! ## UOR encoding
//!
//! - `enforcement::UnaryRingOp<L>` — the trait carrying `apply`.
//! - `enforcement::Neg<L>` — modular negation.
//! - `enforcement::BNot<L>` — bitwise NOT (the Hamming involution
//!   `x → (2^n − 1) XOR x`).
//! - `enforcement::Succ<L>` — successor, defined to satisfy the
//!   critical composition law.
//! - `enforcement::W8`, `W16`, `W32` — phantom-typed Witt-level markers.
//!
//! ## Why this anchors the rest of the crate
//!
//! Rust's wrapping integer arithmetic on `u8`, `u16`, `u32`, `u64` is
//! two's-complement modular arithmetic. By importing the UOR
//! `UnaryRingOp` types and demonstrating that the critical composition
//! law holds on the same primitive integers our higher-level code
//! uses, we make the grounding type-visible: anyone reading the crate
//! sees that arithmetic operations are not informally trusted, they
//! are anchored in a named UOR axiom that has its own conformance
//! suite.
//!
//! ## The full derivation chain (verified in tests)
//!
//! Every higher-level operation reduces to the axiom by a recorded chain:
//!
//! ```text
//!   Succ = Neg ∘ BNot                          (the axiom)
//!     │
//!     ├── iterated  ────►  addition: Succⁿ(x) = x + n
//!     │
//!     ├── BNot ∘ Succ ∘ BNot ────►  Pred(x) = x − 1
//!     │       │
//!     │       └── iterated  ────►  subtraction: Predⁿ(x) = x − n
//!     │
//!     ├── repeated subtraction  ────►  modulus: a mod b
//!     │
//!     └── Euclidean reduction   ────►  gcd(a, b)
//! ```
//!
//! Each step is verified in this module's test suite:
//!
//! - [`tests::iterated_succ_equals_addition_w8`]
//! - [`tests::pred_via_bnot_succ_bnot_w8`]
//! - [`tests::modulus_via_repeated_subtraction_matches_rust_mod`]
//! - [`tests::gcd_via_subtraction_matches_arithmetic_gcd`]
//!
//! The chain terminates with `gcd`, on which every other module of
//! this crate depends. So every constant in `parameters`, every
//! lattice in `u30` and `r360`, every digital-root sum in
//! `appendix_a`, and every Fibonacci value in `fibonacci` ultimately
//! grounds in the single identity `Succ = Neg ∘ BNot`.
//!
//! ## Domain of validity — discrete byte-level arithmetic only
//!
//! `Succ = Neg ∘ BNot` is the byte-level analog of a "single
//! foundational operator" for **discrete** mathematics: it generates
//! all primitive-recursive integer functions (addition, subtraction,
//! multiplication, division, modulus, gcd, integer exponentiation,
//! …) by iteration and composition.
//!
//! It does **not** generate continuous elementary functions (sin,
//! cos, sqrt, exp, ln). For Phase 1 — strictly the foundational
//! arithmetic substrate — `Succ = Neg ∘ BNot` is sufficient. Every
//! claim in the crate is integer-arithmetic; no transcendentals
//! enter, and the wheel-30 sieve uses only addition, subtraction,
//! division, and `gcd`.
//!
//! ## Companion structures (referenced, not yet used)
//!
//! Two further UOR algebraic primitives are cited in source comments
//! for forward compatibility but are not exercised by Phase 1:
//!
//! - **Octonions.** `https://uor.foundation/division/OctonionAlgebra`
//!   — the unique 8-dimensional non-commutative non-associative
//!   normed division algebra, built from ℍ via Cayley-Dickson. The
//!   numerical match `|U(30)| = 8 = algebraDimension(OctonionAlgebra)`
//!   is **classified as coincidence** by Phase 2 O3 (see
//!   `RESULTS.md`): the multiplicative groups
//!   are non-isomorphic and the two 8s arise from independent
//!   constructions (totient vs. Hurwitz theorem).
//! - **Triality.** `https://uor.foundation/query/TriadProjection` —
//!   the typed coordinate-projection enum with three named variants
//!   `{Stratum, Spectrum, Address}`. The Atlas-96 graph's Z/3
//!   factor (in its `(Z/2)^5 × Z/3` decomposition) matches in
//!   cardinality. Phase 2 O3 verdict: **cardinality-only**, the
//!   specific bijection requires Alex's input or a derivation from
//!   the Spin(8)-triality outer automorphism.

use uor_foundation::enforcement::{BNot, Neg, Succ, UnaryRingOp, W16, W32, W8};

/// Re-exports the UOR Witt-level markers used by this crate.
///
/// W8 covers values in `[0, 255]`, W16 covers `[0, 65535]`, W32 covers
/// `[0, 4_294_967_295]`. Our `MODULUS_R = 360` fits in W16; intermediate
/// computations involving `LATTICE_LAYERS · |U(30)|` and the Fibonacci
/// recurrence up to `F_49 ≈ 7.78×10⁹` require W32 or wider.
pub mod witt_levels {
    pub use uor_foundation::enforcement::{W16, W32, W8};
}

/// Apply UOR's `Succ` at Witt level W8.
///
/// Defined as `Neg<W8> ∘ BNot<W8>` per the critical composition law.
#[must_use]
pub fn succ_w8(x: u8) -> u8 {
    <Succ<W8> as UnaryRingOp<W8>>::apply(x)
}

/// Apply UOR's `Succ` at Witt level W16.
#[must_use]
pub fn succ_w16(x: u16) -> u16 {
    <Succ<W16> as UnaryRingOp<W16>>::apply(x)
}

/// Apply UOR's `Succ` at Witt level W32.
#[must_use]
pub fn succ_w32(x: u32) -> u32 {
    <Succ<W32> as UnaryRi