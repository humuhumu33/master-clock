//! # 576 = 24² ordered factorization dyads in `U_30(90) × U_30(90)`
//!
//! For each ordered pair `(a, b) ∈ U_30(90) × U_30(90)`, the product
//! `(a · b) mod MODULUS_U90` lands in another `U_30(90)` residue
//! (since `U_30(90)` is closed under multiplication mod 90 — the
//! multiplicative group of units of `Z/90Z` restricted to those
//! coprime to 30).
//!
//! There are exactly `24² = 576` ordered dyads. Grouped by product
//! residue, they form 24 fibers of 24 dyads each — one fiber per
//! residue class in `U_30(90)`. Each fiber is the set of all
//! `(a, b)` whose product reduces to a fixed `r ∈ U_30(90)`.
//!
//! ## What this module establishes
//!
//! - The total dyad count is `576 = U90_CARDINALITY²`.
//! - Each fiber has exactly `U90_CARDINALITY = 24` dyads.
//! - The product of any two `U_30(90)` members reduces back into
//!   `U_30(90)` (group-closure property).
//!
//! All claims compile-time-checked; the full 576-element table is
//! emitted as a `pub const` so callers can inspect, iterate, or
//! pattern-match without runtime computation.
//!
//! ## Anchor — derived from Phase 1 + Phase 1.5
//!
//! Computed at compile time from
//! [`crate::u90::RESIDUES_U90`] and
//! [`crate::u90::MODULUS_U90`]. Nothing else.
//!
//! ## UOR primitive bindings
//!
//! | Object | UOR ontology IRI |
//! |---|---|
//! | `DYADS_TABLE` | `https://uor.foundation/query/CoordinateQuery` (576-cell lookup) |
//! | dyad enumeration | `https://uor.foundation/stream/ProductiveStream` |
//! | fiber structure | `https://uor.foundation/morphism/Fibration` |

use crate::u90::{MODULUS_U90, RESIDUES_U90, U90_CARDINALITY};

/// Total number of ordered dyads. Equals `24² = 576`.
pub const DYAD_COUNT: usize = U90_CARDINALITY * U90_CARDINALITY;

const _: () = assert!(DYAD_COUNT == 576, "ordered dyad count must equal 576");

/// `(a, b, product mod 90)` for every ordered dyad. Computed at
/// compile time. Length `DYAD_COUNT = 576`.
pub const DYADS_TABLE: [(u64, u64, u64); 576] = compute_dyads_table();

const fn compute_dyads_table() -> [(u64, u64, u64); 576] {
    let mut out = [(0u64, 0u64, 0u64); 576];
    let mut idx = 0usize;
    let mut i = 0usize;
    while i < U90_CARDINALITY {
        let a = RESIDUES_U90[i];
        let mut j = 0usize;
        while j < U90_CARDINALITY {
            let b = RESIDUES_U90[j];
            let prod = (a * b) % MODULUS_U90;
            out[idx] = (a, b, prod);
            idx += 1;
            j += 1;
        }
        i += 1;
    }
    out
}

const _: () = {
    // Every product is in U_30(90).
    let mut i = 0usize;
    while i < 576 {
        let (_, _, p) = DYADS_TABLE[i];
        let mut hit = false;
        let mut k = 0usize;
        while k < U90_CARDINALITY {
            if RESIDUES_U90[k] == p {
                hit = true;
            }
            k += 1;
        }
        assert!(
            hit,
            "dyad product escapes U_30(90) — closure property broken"
        );
        i += 1;
    }
};

/// Index of `r` within [`crate::u90::RESIDUES_U90`], or `usize::MAX`
/// if absent. `const fn` lookup; used as a coordinate query.
#[must_use]
pub const fn u90_position(r: u64) -> usize {
    let mut i = 0usize;
    while i < U90_CARDINALITY {
        if RESIDUES_U90[i] == r {
            return i;
        }
        i += 1;
    }
    usize::MAX
}

/// **Fiber size** of each residue class. Every `r ∈ U_30(90)` is the
/// product of exactly `U90_CARDINALITY = 24` ordered dyads. (Group
/// closure + cancellation: `a · ? = r` has a unique solution `?` in
/// `U_30(90)` for each `a`.)
pub const FIBER_SIZE: usize = U90_CARDINALITY;

const _: () = {
    // Verify by exhaustive count: each residue appears 24 times.
    let mut counts = [0u64; 24];
    let mut i = 0usize;
    while i < 576 {
        let (_, _, p) = DYADS_TABLE[i];
        let pos = u90_position(p);
        counts[pos] += 1;
        i += 1;
    }
    let mut k = 0usize;
    while k < U90_CARDINALITY {
        assert!(
            counts[k] == FIBER_SIZE as u64,
            "fiber size drift: not all residues hit 24 times"
        );
        k += 1;
    }
};

/// **Cardinality identity.** `DYAD_COUNT = U90_CARDINALITY² =
/// FIBER_SIZE · U90_CARDINALITY`.
pub const CARDINALITY_IDENTITY: usize = FIBER_SIZE * U90_CARDINALITY;

const _: () = assert!(
    CARDINALITY_IDENTITY == DYAD_COUNT,
    "fibers · base must equal total dyad count"
);

/// **Diagonal.** The 24 dyads `(a, a)` for `a ∈ U_30(90)` —
/// products are squares mod 90, equal to
/// [`crate::u90::SQUARE_RESIDUE_VALUES`] each appearing 4 times.
pub const DIAGONAL_DYADS: [(u64, u64, u64); 24] = compute_diagonal_dyads();

const fn compute_diagonal_dyads() -> [(u64, u64, u64); 24] {
    let mut out = [(0u64, 0u64, 0u64); 24];
    let mut i = 0usize;
    while i < U90_CARDINALITY {
        let a = RESIDUES_U90[i];
        out[i] = (a, a, (a * a) % MODULUS_U90);
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic, clippy::expect_used)]
    use super::*;

    #[test]
    fn dyad_count_576() {
        assert_eq!(DYAD_COUNT, 576);
        assert_eq!(DYADS_TABLE.len(), 576);
    }

    #[test]
    fn every_product_in_u90() {
        for &(_, _, p) in &DYADS_TABLE {
            assert!(RESIDUES_U90.contains(&p), "product {p} escapes U_30(90)");
        }
    }

    #[test]
    fn fiber_size_24() {
        assert_eq!(FIBER_SIZE, 24);
        let mut counts = std::collections::BTreeMap::<u64, usize>::new();
        for &(_, _, p) in &DYADS_TABLE {
            *counts.entry(p).or_insert(0) += 1;
        }
        assert_eq!(counts.len(), U90_CARDINALITY);
        for &c in counts.values() {
            assert_eq!(c, FIBER_SIZE);
        }
    }

    #[test]
    fn cardinality_identity_holds() {
        assert_eq!(CARDINALITY_IDENTITY, DYAD_COUNT);
        assert_eq!(U90_CARDINALITY * U90_CARDINALITY, 576);
    }

    #[test]
    fn diagonal_has_24_entries_squared() {
        assert_eq!(DIAGONAL_DYADS.len(), 24);
        for &(a, b, p) in &DIAGONAL_DYADS {
            assert_eq!(a, b);
            assert_eq!(p, (a * a) % MODULUS_U90);
        }
    }

    #[test]
    fn diagonal_residues_match_square_set() {
        use crate::u90::SQUARE_RESIDUE_VALUES;
        let mut seen = std::collections::BTreeSet::new();
        for &(_, _, p) in &DIAGONAL_DYADS {
            seen.insert(p);
        }
        let expected: std::collections::BTreeSet<u64> =
            SQUARE_RESIDUE_VALUES.iter().copied().collect();
        assert_eq!(seen, expected);
    }
}
