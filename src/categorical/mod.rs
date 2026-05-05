//! # Categorical layer — the EML universality formalism
//!
//! This module mirrors the architectural pattern of
//! `research-main/atlas-embeddings/src/categorical/`:
//!
//! - That crate proves *the Atlas is initial in `ResGraph`* and
//!   exhibits each exceptional Lie group as a categorical operation
//!   on it (Product, Quotient, Filtration, Augmentation, Embedding).
//! - This module proves *the singleton `{1}` together with `eml` is
//!   the free magma in the category of elementary expressions over
//!   `ℚ`* and exhibits each elementary function (exp, ln, add, mul,
//!   …) as an EML-tree morphism, following Odrzywołek (arXiv:2603.21852).
//!
//! Both are universality-of-an-initial-object claims, formalised via
//! executable constructors plus universal-property tests, with an
//! eventual Lean 4 proof track.
//!
//! ## Place in the eight-layer bridge
//!
//! ```text
//! Layer 7: eml(x, y) = exp(x) − ln(y)              [elementary.rs]
//!     │
//!     │  free magma over {1, x, y, …} under eml
//!     ▼
//! Layer 7+: EML tree + universal property         [THIS MODULE]
//! ```
//!
//! ## Status
//!
//! Phase 3+ scaffolding. Two of Odrzywołek's identities are wired
//! up with verification tests:
//!
//! - `exp(x) = eml(x, 1)` (immediate from the definition `eml(a,b) =
//!   exp(a) − ln(b)`, since `ln(1) = 0`).
//! - `ln(x) = eml(1, eml(eml(1, x), 1))` (the paper's stated form).
//!
//! Additional identities (`add`, `sub`, `mul`, `div`, trigonometric
//! and inverse-trig functions, `sqrt`, `pow`, etc.) are stubbed for
//! follow-up.
//!
//! ## What is NOT here
//!
//! - **Compilation to `.holo`.** This module's `EmlTree` is a
//!   recursive Rust ADT and `evaluate` walks the tree at runtime.
//!   `.holo` archives are forward DAGs of byte-domain primitives and
//!   cannot represent recursive expression trees. The link to
//!   hologram is a *build-time exporter* (planned, not in this
//!   module) that takes an `EmlTree`, samples it on a discrete
//!   domain, and emits an LUT-fused `.holo` archive validated
//!   against the Cauchy reference here.

pub mod eml;
