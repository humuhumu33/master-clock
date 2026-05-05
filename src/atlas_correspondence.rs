//! # Atlas correspondence — `R(360) ↔ Resonance-96`
//!
//! This module encodes the bijection between [`crate::r360::ELEMENTS`]
//! (the 96 integers of `R(360)`) and the 96-vertex Atlas of Resonance
//! Classes (`atlas-embeddings::Atlas`, also the vertex set of the
//! `resonance-spectral-gap` graph).
//!
//! ## What Alex confirmed
//!
//! From the WhatsApp transcript: the 96-vertex Resonance Classes graph
//! in `resonance-spectral-gap` corresponds to `R(360)`. The
//! correspondence is graph-level (vertex bijection plus edge structure
//! preserved), not abelian-group-level — `U(360) ≅ (Z/2)³ × Z/3 × Z/4`
//! has an element of order 4, while Atlas-96 ≅ `(Z/2)⁵ × Z/3` does
//! not. So whatever map carries vertices, it does not respect the
//! multiplicative group structure of `U(360)`.
//!
//! ## Atlas vertex labels
//!
//! Per `atlas-embeddings::atlas::Label`, an Atlas vertex is a 6-tuple
//! `(e1, e2, e3, d45, e6, e7)` with:
//!
//! - `e1, e2, e3, e6, e7 ∈ {0, 1}` (binary coordinates)
//! - `d45 ∈ {−1, 0, +1}` (ternary, the `e4 − e5` difference)
//!
//! Total: `2⁵ · 3 = 96` vertices. The canonical iteration order
//! (matching `atlas-embeddings/src/atlas/mod.rs::generate_labels`):
//!
//! ```text
//! for e1 in 0..=1 {
//!   for e2 in 0..=1 {
//!     for e3 in 0..=1 {
//!       for e6 in 0..=1 {
//!         for e7 in 0..=1 {
//!           for d45 in -1..=1 {
//!             labels.push((e1, e2, e3, d45, e6, e7));
//! } } } } } }
//! ```
//!
//! `d45` cycles fastest (period 3), then `e7` (period 2), then `e6`,
//! `e3`, `e2`, and `e1`. The vertex at index `i` has:
//!
//! ```text
//! d45 = (i mod 3) − 1
//! e7  = (i /  3) mod 2
//! e6  = (i /  6) mod 2
//! e3  = (i / 12) mod 2
//! e2  = (i / 24) mod 2
//! e1  = (i / 48) mod 2
//! ```
//!
//! ## Bijection encoding
//!
//! Two arrays establish the bijection:
//!
//! - [`R_TO_ATLAS_INDEX`] — for each `i ∈ [0, R_CARDINALITY)`, the
//!   Atlas vertex index assigned to `r360::ELEMENTS[i]`.
//! - [`ATLAS_TO_R_INDEX`] — the inverse permutation.
//!
//! Phase 1 ships with the **identity** permutation as the default
//! map, marked clearly: this is the simplest bijection consistent
//! with Alex's confirmation that the two 96-element sets correspond.
//! The graph-edge-preserving map (`R_TO_ATLAS_INDEX[i] = π(i)` for
//! some non-identity permutation `π`) is the structurally meaningful
//! one. **Phase 2 O2 progress:** the Atlas-96 adjacency is now
//! formally specified in
//! `lean4/PrimeFoundations/AtlasCorrespondence.lean` (theorems
//! `degree_is_5_or_6`, `total_edges_is_256`,
//! `mirror_pairs_are_not_neighbours`). The corresponding adjacency
//! on `R(360)` — and hence the actual edge-preserving permutation —
//! remains open and is escalated to Alex; see the `Phase 2 O2 —
//! R(360) side, open` section in that Lean module for the question
//! statement and three candidate adjacencies to test.
//!
//! ## Domain note
//!
//! Phase 1 verifies only: (a) the Atlas label set has cardinality
//! exactly 96, (b) the iteration order matches atlas-embeddings's
//! canonical order, (c) mirror symmetry τ : `e7 → 1 − e7` satisfies
//! τ² = identity, (d) the bijection is well-defined (round-trip).
//! Edge-preservation is **not** verified — it requires importing or
//! reproducing `atlas-embeddings::Atlas::build_adjacency`, which is
//! Phase 2 work.

use crate::parameters::R_CARDINALITY;
use crate::r360::ELEMENTS;

/// Atlas vertex label — mirrors `atlas-embeddings::atlas::Label`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AtlasLabel {
    /// `e1 ∈ {0, 1}`.
    pub e1: u8,
    /// `e2 ∈ {0, 1}`.
    pub e2: u8,
    /// `e3 ∈ {0, 1}`.
    pub e3: u8,
    /// `d45 ∈ {−1, 0, +1}` — the `e4 − e5` ternary difference.
    pub d45: i8,
    /// `e6 ∈ {0, 1}`.
    pub e6: u8,
    /// `e7 ∈ {0, 1}` — flipped by mirror symmetry τ.
    pub e7: u8,
}

impl AtlasLabel {
    /// Construct a label, with the standard atlas-embeddings range
    /// constraints enforced via `assert!`.
    #[must_use]
    pub const fn new(e1: u8, e2: u8, e3: u8, d45: i8, e6: u8, e7: u8) -> Self {
        assert!(
            e1 <= 1 && e2 <= 1 && e3 <= 1 && e6 <= 1 && e7 <= 1,
            "binary coordinates must be 0 or 1"
        );
        assert!(d45 >= -1 && d45 <= 1, "d45 must be in {{-1, 0, +1}}");
        Self {
            e1,
            e2,
            e3,
            d45,
            e6,
            e7,
        }
    }

    /// Apply mirror symmetry τ — flip `e7`. `τ² = identity`.
    #[must_use]
    pub const fn mirror(self) -> Self {
        Self {
            e1: self.e1,
            e2: self.e2,
            e3: self.e3,
            d45: self.d45,
            e6: self.e6,
            e7: 1 - self.e7,
        }
    }
}

// ─── Derived: the 96 Atlas labels in canonical iteration order ───────

/// The 96 Atlas vertex labels in `atlas-embeddings::generate_labels`
/// iteration order.
///
/// **Derived** at compile time. Iteration matches
/// `atlas-embeddings/src/atlas/mod.rs::generate_labels` exactly.
pub const ATLAS_LABELS: [AtlasLabel; R_CARDINALITY] = compute_atlas_labels();

const fn compute_atlas_labels() -> [AtlasLabel; R_CARDINALITY] {
    let mut out = [AtlasLabel::new(0, 0, 0, -1, 0, 0); R_CARDINALITY];
    let mut idx = 0usize;

    // The same six nested loops as `generate_labels`, written
    // const-fn-style with explicit while loops.
    let mut e1: u8 = 0;
    while e1 <= 1 {
        let mut e2: u8 = 0;
        while e2 <= 1 {
            let mut e3: u8 = 0;
            while e3 <= 1 {
                let mut e6: u8 = 0;
                while e6 <= 1 {
                    let mut e7: u8 = 0;
                    while e7 <= 1 {
                        let mut d45_idx: u8 = 0;
                        while d45_idx <= 2 {
                            // d45_idx = 0,1,2 → d45 = -1, 0, +1
                            #[allow(clippy::cast_possible_wrap)]
                            let d45 = (d45_idx as i8) - 1;
                            out[idx] = AtlasLabel::new(e1, e2, e3, d45, e6, e7);
                            idx += 1;
                            d45_idx += 1;
                        }
                        e7 += 1;
                    }
                    e6 += 1;
                }
                e3 += 1;
            }
            e2 += 1;
        }
        e1 += 1;
    }
    out
}

// Compile-time check: exactly 96 labels were emitted.
const _: () = {
    let mut count = 0usize;
    let mut i = 0usize;
    // Distinguishability: every label is reachable; all 96 slots filled.
    // Use the `e1 == 0 && d45 == -1 && e7 == 0` "first" label to
    // identify uninitialised vs initialised slots only at index 0;
    // every other slot must differ from the seed pattern in at least
    // one coordinate. We instead verify by checking idx hit 96 above
    // — but const-fn return forces idx == R_CARDINALITY by array size.
    while i < ATLAS_LABELS.len() {
        count += 1;
        i += 1;
    }
    assert!(count == R_CARDINALITY, "ATLAS_LABELS slot count");
};

// Compile-time check: pairwise distinct labels. Two labels are equal
// iff all six coordinates match.
const _: () = {
    let mut i = 0usize;
    while i < R_CARDINALITY {
        let mut j = i + 1;
        while j < R_CARDINALITY {
            let a = ATLAS_LABELS[i];
            let b = ATLAS_LABELS[j];
            let same = a.e1 == b.e1
                && a.e2 == b.e2
                && a.e3 == b.e3
                && a.d45 == b.d45
                && a.e6 == b.e6
                && a.e7 == b.e7;
            assert!(!same, "duplicate Atlas label found");
            j += 1;
        }
        i += 1;
    }
};

// ─── Bijection R(360) ↔ Atlas-96 ─────────────────────────────────────

/// Permutation `R(360)-index → Atlas-vertex-index`.
///
/// **Phase 1 placeholder: identity permutation.** Set so that
/// `R_TO_ATLAS_INDEX[i] = i` — the i-th element of
/// [`r360::ELEMENTS`] maps to the i-th Atlas vertex in the
/// canonical iteration order. The graph-edge-preserving permutation
/// is a Phase 2 verification target; until then, this identity map
/// stands as the simplest bijection consistent with Alex's
/// vertex-set correspondence claim.
pub const R_TO_ATLAS_INDEX: [usize; R_CARDINALITY] = {
    let mut out = [0usize; R_CARDINALITY];
    let mut i = 0usize;
    while i < R_CARDINALITY {
        out[i] = i;
        i += 1;
    }
    out
};

/// Inverse permutation `Atlas-vertex-index → R(360)-index`.
///
/// Computed as `ATLAS_TO_R_INDEX[R_TO_ATLAS_INDEX[i]] = i` for
/// every `i`. With the identity permutation this is also the
/// identity, but the const-fn computation works for any permutation
/// and so survives the Phase 2 update without code change.
pub const ATLAS_TO_R_INDEX: [usize; R_CARDINALITY] = {
    let mut out = [0usize; R_CARDINALITY];
    let mut i = 0usize;
    while i < R_CARDINALITY {
        let j = R_TO_ATLAS_INDEX[i];
        out[j] = i;
        i += 1;
    }
    out
};

// Compile-time enforcement: the two permutations are mutual inverses.
const _: () = {
    let mut i = 0usize;
    while i < R_CARDINALITY {
        assert!(
            ATLAS_TO_R_INDEX[R_TO_ATLAS_INDEX[i]] == i,
            "ATLAS_TO_R_INDEX is not the left inverse of R_TO_ATLAS_INDEX"
        );
        i += 1;
    }
    let mut j = 0usize;
    while j < R_CARDINALITY {
        assert!(
            R_TO_ATLAS_INDEX[ATLAS_TO_R_INDEX[j]] == j,
            "R_TO_ATLAS_INDEX is not the right inverse of ATLAS_TO_R_INDEX"
        );
        j += 1;
    }
};

// Compile-time enforcement: the **identity-placeholder contract**.
// Mutual-inverse alone accepts any bijection, including a sabotage
// swap that silently replaces the placeholder. This block pins the
// stronger property that `R_TO_ATLAS_INDEX[i] == i` for every `i`,
// matching the Lean theorem
// `PrimeFoundations.AtlasCorrespondence.rToAtlasIndex_is_identity_placeholder`.
//
// When the Phase 2 O2 edge-preserving bijection lands, this block
// must be removed (or updated to assert the new permutation) in the
// same change that updates the Lean `rToAtlasIndex` definition.
// Failing to remove it will cause a build break — exactly the
// lockstep we want.
const _: () = {
    let mut i = 0usize;
    while i < R_CARDINALITY {
        assert!(
            R_TO_ATLAS_INDEX[i] == i,
            "R_TO_ATLAS_INDEX has drifted from the identity-placeholder contract; \
             update Lean's `rToAtlasIndex` in the same change."
        );
        i += 1;
    }
};

// ─── Public API: forward and inverse maps ────────────────────────────

/// Forward map: `n ∈ R(360)` to its Atlas vertex label.
///
/// Returns `None` if `n` is not in `R(360)`.
#[must_use]
pub const fn n_to_atlas_label(n: u32) -> Option<AtlasLabel> {
    let r_idx = match find_r_index(n) {
        Some(i) => i,
        None => return None,
    };
    let atlas_idx = R_TO_ATLAS_INDEX[r_idx];
    Some(ATLAS_LABELS[atlas_idx])
}

/// Inverse map: an Atlas vertex label to the corresponding `n ∈ R(360)`.
///
/// Returns `None` if the label is not one of the 96 canonical labels.
#[must_use]
pub const fn atlas_label_to_n(label: AtlasLabel) -> Option<u32> {
    let atlas_idx = match find_atlas_index(label) {
        Some(i) => i,
        None => return None,
    };
    let r_idx = ATLAS_TO_R_INDEX[atlas_idx];
    Some(ELEMENTS[r_idx])
}

const fn find_r_index(n: u32) -> Option<usize> {
    let mut i = 0usize;
    while i < R_CARDINALITY {
        if ELEMENTS[i] == n {
            return Some(i);
        }
        i += 1;
    }
    None
}

const fn find_atlas_index(label: AtlasLabel) -> Option<usize> {
    let mut i = 0usize;
    while i < R_CARDINALITY {
        let l = ATLAS_LABELS[i];
        if l.e1 == label.e1
            && l.e2 == label.e2
            && l.e3 == label.e3
            && l.d45 == label.d45
            && l.e6 == label.e6
            && l.e7 == label.e7
        {
            return Some(i);
        }
        i += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

    use super::*;

    /// Exactly 96 Atlas labels.
    #[test]
    fn atlas_label_count_is_96() {
        assert_eq!(ATLAS_LABELS.len(), R_CARDINALITY);
        assert_eq!(R_CARDINALITY, 96);
    }

    /// All Atlas labels are distinct.
    #[test]
    fn atlas_labels_pairwise_distinct() {
        for i in 0..R_CARDINALITY {
            for j in (i + 1)..R_CARDINALITY {
                assert_ne!(ATLAS_LABELS[i], ATLAS_LABELS[j], "duplicate at ({i}, {j})");
            }
        }
    }

    /// Every coordinate range respected.
    #[test]
    fn atlas_label_coordinate_ranges() {
        for &l in &ATLAS_LABELS {
            assert!((0..=1).contains(&l.e1));
            assert!((0..=1).contains(&l.e2));
            assert!((0..=1).contains(&l.e3));
            assert!((0..=1).contains(&l.e6));
            assert!((0..=1).contains(&l.e7));
            assert!((-1..=1).contains(&l.d45));
        }
    }

    /// The iteration order matches the index decomposition documented
    /// at the top of this module: `d45` cycles fastest.
    #[test]
    fn iteration_order_matches_index_decomposition() {
        for (i, &l) in ATLAS_LABELS.iter().enumerate() {
            let expected_d45 = (i as i32 % 3) - 1;
            let expected_e7 = (i / 3) % 2;
            let expected_e6 = (i / 6) % 2;
            let expected_e3 = (i / 12) % 2;
            let expected_e2 = (i / 24) % 2;
            let expected_e1 = (i / 48) % 2;
            assert_eq!(l.d45 as i32, expected_d45, "d45 at index {i}");
            assert_eq!(l.e7 as usize, expected_e7, "e7 at index {i}");
            assert_eq!(l.e6 as usize, expected_e6, "e6 at index {i}");
            assert_eq!(l.e3 as usize, expected_e3, "e3 at index {i}");
            assert_eq!(l.e2 as usize, expected_e2, "e2 at index {i}");
            assert_eq!(l.e1 as usize, expected_e1, "e1 at index {i}");
        }
    }

    /// Mirror symmetry `τ² = identity`.
    #[test]
    fn mirror_is_involution() {
        for &l in &ATLAS_LABELS {
            assert_eq!(l.mirror().mirror(), l);
        }
    }

    /// Mirror symmetry `τ` is a bijection on the Atlas vertex set —
    /// applying τ to every label gives a permutation of the same set.
    #[test]
    fn mirror_permutes_atlas_labels() {
        for &l in &ATLAS_LABELS {
            let m = l.mirror();
            // m must also be one of the 96 labels.
            assert!(
                find_atlas_index(m).is_some(),
                "mirror of {l:?} not in ATLAS_LABELS"
            );
        }
    }

    /// The R-to-Atlas and Atlas-to-R index permutations are mutual
    /// inverses (cross-checks the compile-time assert).
    #[test]
    fn permutations_are_mutual_inverses() {
        for i in 0..R_CARDINALITY {
            assert_eq!(ATLAS_TO_R_INDEX[R_TO_ATLAS_INDEX[i]], i);
        }
        for j in 0..R_CARDINALITY {
            assert_eq!(R_TO_ATLAS_INDEX[ATLAS_TO_R_INDEX[j]], j);
        }
    }

    /// Round-trip `n → label → n` is the identity on `R(360)`.
    #[test]
    fn n_to_label_to_n_is_identity() {
        for &n in &ELEMENTS {
            let label = n_to_atlas_label(n).expect("R(360) element must map");
            let n_back = atlas_label_to_n(label).expect("valid label must map back");
            assert_eq!(n, n_back, "round-trip failure at n = {n}");
        }
    }

    /// Round-trip `label → n → label` is the identity on Atlas-96.
    #[test]
    fn label_to_n_to_label_is_identity() {
        for &label in &ATLAS_LABELS {
            let n = atlas_label_to_n(label).expect("Atlas vertex must map");
            let label_back = n_to_atlas_label(n).expect("R(360) element must map back");
            assert_eq!(label, label_back, "round-trip failure at {label:?}");
        }
    }

    /// `n_to_atlas_label` rejects integers not in `R(360)`.
    #[test]
    fn n_to_label_rejects_non_r360() {
        for n in [0u32, 2, 3, 5, 6, 30, 60, 360, 361, 1000] {
            assert!(
                n_to_atlas_label(n).is_none(),
                "{n} should not have an Atlas label"
            );
        }
    }
}
