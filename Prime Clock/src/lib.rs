//! # Toroidal Prime Counting Master-Clock — UOR-grounded encoding
//!
//! Rigorous Rust implementation of the foundational arithmetic
//! substrate of Gary William Croft's *Toroidal Prime Counting
//! Master-Clock* (FIXED revision, 2026), built strictly on top of the
//! UOR Foundation ([`uor_foundation`], [`uor_foundation_sdk`]) and
//! following the discipline established by `atlas-embeddings`.
//!
//! # Source documents
//!
//! - Gary William Croft, *Toroidal Prime Counting Master-Clock* (FIXED
//!   revision, 2026). Located at
//!   `UOR-Prime/attachments/Toroidal_Prime_Counting_Master_Clock_FIXED.pdf`.
//! - OEIS [A227896](https://oeis.org/A227896) — Fibonacci digital
//!   roots indexed to U(30), the prior-art anchor cited by Gary (2013).
//!
//! # Algebraic substrate
//!
//! The bottom layer of this crate is [`ring_substrate`], which anchors
//! every integer operation in UOR's `UnaryRingOp` axiom. The critical
//! composition law `Succ = Neg ∘ BNot` (equivalently `−(¬x) = x + 1`
//! in two's-complement) is the bridge between bitwise and arithmetic
//! rings; UOR formalises it as a typeclass at
//! `foundation/src/enforcement.rs:9894`. Our integer operations sit
//! on top of this axiom by virtue of using Rust's wrapping integer
//! arithmetic (which is two's complement); [`ring_substrate`] makes
//! that grounding explicit and re-verifies the law inside our own
//! test suite.
//!
//! Two further UOR algebraic primitives are referenced in source as
//! cited connections to Gary's framework. Their structural status
//! is fixed by `STRUCTURAL_CORRESPONDENCES.md` (Phase 2 O3 verdict):
//!
//! - `https://uor.foundation/division/OctonionAlgebra` — the
//!   8-dimensional non-associative normed division algebra. The
//!   numerical match `|U(30)| = 8 = dim(𝕆)` is **classified as
//!   coincidence**: `U(30) ≅ ℤ/2 × ℤ/4` (multiplicative group),
//!   while the octonion basis under multiplication is the
//!   non-associative Cayley loop of order 16. The two 8s arise
//!   from independent constructions (Euler totient vs. Hurwitz
//!   theorem). No canonical structure-preserving bijection is
//!   known; if one is exhibited later the verdict can be revisited.
//! - `https://uor.foundation/query/TriadProjection` — the typed
//!   coordinate-projection enum with three named variants
//!   `{Stratum, Spectrum, Address}`. The Atlas-96 graph carries a
//!   Z/3 factor `d₄₅ ∈ {−1, 0, +1}`. **Cardinality-only match**:
//!   both sides have a real, typed Z/3 structure and a canonical
//!   bijection plausibly exists, but the specific assignment of
//!   d₄₅ values to TriadProjection variants is not yet documented
//!   in `atlas-embeddings` or in this crate. Verdict open pending
//!   Alex's input.
//!
//! # Self-deriving discipline
//!
//! Every constant in this crate is one of three kinds, classified
//! explicitly:
//!
//! - **Parameter.** An irreducible input chosen by Gary or by the
//!   numeral system. Currently: [`parameters::MODULUS_U`] = 30 and
//!   [`parameters::NUMERAL_BASE`] = 10. Two values, period.
//! - **Derived.** Computed at compile time from parameters via
//!   [`arithmetic`] (gcd, lcm, totient), [`digital_root`], and
//!   [`fibonacci`]. Examples: [`parameters::U30_CARDINALITY`] = 8,
//!   [`parameters::MODULUS_R`] = 360, [`u30::RESIDUES`],
//!   [`appendix_a::ROWS`], [`appendix_a::SUM_DR`] = 432.
//! - **Verification target.** Hand-typed values from Gary's FIXED PDF,
//!   used **only** as cross-checks against derivations. Examples:
//!   [`u30::GARY_STATED_RESIDUES`], [`appendix_a::FIXED_PDF_VERIFICATION`],
//!   [`appendix_a::GARY_STATED_TOTAL_DR`].
//!
//! Compile-time `const _: () = assert!(…)` blocks enforce that every
//! derivation matches Gary's stated values. If a mismatch ever
//! appears — because Gary's PDF was wrong, or our derivation is wrong,
//! or a parameter was changed — the build fails. There are no magic
//! numbers in production paths.
//!
//! # Strict UOR encoding
//!
//! Every set is expressed as a
//! [`uor_foundation::pipeline::ConstrainedTypeShape`] composed via
//! [`uor_foundation_sdk`] procedural macros. The 8 residues of
//! [`u30::RESIDUES`] are 8 leaf shapes carrying
//! [`uor_foundation::pipeline::ConstraintRef::Residue`]; [`u30::U30`]
//! itself is a balanced coproduct tree built with
//! [`uor_foundation_sdk::coproduct_shape`]. Each leaf's residue value
//! is sourced from the derived [`u30::RESIDUES`] array, not from
//! literal integers.
//!
//! # Discipline
//!
//! - `#![forbid(unsafe_code)]`
//! - `clippy::float_arithmetic` denied — all arithmetic is exact rational or integer
//! - All clippy warnings as errors (parity with `atlas-embeddings`)
//! - `#![no_std]` core; `alloc` and `ground-truth` features for opt-in
//!   host-only paths

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![deny(clippy::float_arithmetic, clippy::float_cmp, clippy::float_cmp_const)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod appendix_a;
pub mod arithmetic;
pub mod atlas_correspondence;
pub mod d_function;
pub mod digital_root;
pub mod dirichlet;
pub mod fibonacci;
#[cfg(feature = "ground-truth")]
pub mod ground_truth;
pub mod harness;
pub mod li;
pub mod master_clock;
pub mod parameters;
pub mod r360;
pub mod ring_substrate;
pub mod u30;

// --- Phase 3: Layers 4–7 of the BRIDGE.md eight-layer stack ---
// Allocator-backed paths that lift the discrete byte-ring substrate
// into exact rationals, computable reals, and the elementary
// functions / `eml` of Odrzywołek (arXiv:2603.21852).
#[cfg(feature = "alloc")]
pub mod cauchy;
#[cfg(feature = "alloc")]
pub mod categorical;
#[cfg(feature = "alloc")]
pub mod elementary;
#[cfg(feature = "alloc")]
pub mod rational;
#[cfg(feature = "alloc")]
pub mod rf_function;
#[cfg(feature = "alloc")]
pub mod tf_function;

/// IRI prefix for ontology terms minted by this crate.
pub const ONTOLOGY_PREFIX: &str = "https://uor.foundation/prime-clock";
