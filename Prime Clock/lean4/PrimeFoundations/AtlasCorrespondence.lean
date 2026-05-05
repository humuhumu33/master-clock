/-
Copyright (c) 2026 UOR Foundation. All rights reserved.
Released under Apache-2.0 license.

# Atlas correspondence — R(360) ↔ Resonance-96 vertex set.

Mirrors `src/atlas_correspondence.rs`. Phase 2 — Lean side.

**Builds on:** `PrimeFoundations.Parameters` for `rCardinality`,
`PrimeFoundations.R360` for the lattice. Mirrors the Phase 1 Rust
`AtlasLabel` struct, `ATLAS_LABELS` enumeration in canonical
iteration order, mirror symmetry τ² = id, pairwise distinctness,
and coordinate-range constraints.

The edge-preserving permutation `R_TO_ATLAS_INDEX` (currently the
identity placeholder in the Rust crate) is Phase 2 objective O2;
this module establishes the vertex-set structure on which that
permutation will act.

**NO `sorry` POLICY** — every theorem closed by `rfl`, `decide`,
or constructive proof.
-/

import PrimeFoundations.Parameters
import PrimeFoundations.R360
import Mathlib.Data.List.Basic

namespace PrimeFoundations.AtlasCorrespondence

open PrimeFoundations.Parameters

/-! ## Atlas vertex labels -/

/-- Atlas vertex label — mirror of `crate::atlas_correspondence::AtlasLabel`.

Six coordinates: five binary (`e1, e2, e3, e6, e7 ∈ {0, 1}`) and one
ternary (`d45 = e4 − e5 ∈ {−1, 0, +1}`). Total label space:
`2⁵ × 3 = 96`, matching `R_CARDINALITY`. -/
structure AtlasLabel where
  e1 : Nat
  e2 : Nat
  e3 : Nat
  d45 : Int
  e6 : Nat
  e7 : Nat
  deriving DecidableEq, Repr

namespace AtlasLabel

/-- Apply mirror symmetry τ: flip `e7`. Mirror of
`crate::atlas_correspondence::AtlasLabel::mirror`. -/
def mirror (l : AtlasLabel) : AtlasLabel :=
  { l with e7 := 1 - l.e7 }

end AtlasLabel

/-! ## Enumeration in `atlas-embeddings` canonical order -/

/-- All 96 Atlas labels in `atlas-embeddings` canonical iteration
order: `d45` cycles fastest (3 values), then `e7`, `e6`, `e3`, `e2`,
`e1`. Total `2 · 2 · 2 · 2 · 2 · 3 = 96`.

Mirror of `crate::atlas_correspondence::ATLAS_LABELS` and the
`generate_labels` function in
`atlas-embeddings/src/atlas/mod.rs:431-451`. -/
def atlasLabels : List AtlasLabel :=
  (List.range 2).flatMap fun e1 =>
  (List.range 2).flatMap fun e2 =>
  (List.range 2).flatMap fun e3 =>
  (List.range 2).flatMap fun e6 =>
  (List.range 2).flatMap fun e7 =>
  ([(-1 : Int), 0, 1]).map fun d45 =>
    { e1 := e1, e2 := e2, e3 := e3, d45 := d45, e6 := e6, e7 := e7 }

/-! ## Cardinality and structural theorems -/

/-- **Theorem.** `|atlasLabels| = R_CARDINALITY = 96`. Mirror of
`atlas_correspondence::tests::atlas_label_count_is_96`. -/
theorem atlas_labels_length : atlasLabels.length = rCardinality := by
  decide

/-- **Theorem.** No two labels in the enumeration are equal.
Mirror of `atlas_correspondence::tests::atlas_labels_pairwise_distinct`. -/
theorem atlas_labels_nodup : atlasLabels.Nodup := by
  decide

/-- **Theorem.** Every label respects the coordinate-range constraints.
Mirror of `atlas_correspondence::tests::atlas_label_coordinate_ranges`. -/
theorem atlas_label_ranges :
    atlasLabels.all (fun l =>
      l.e1 ≤ 1 && l.e2 ≤ 1 && l.e3 ≤ 1 && l.e6 ≤ 1 && l.e7 ≤ 1 &&
      -1 ≤ l.d45 && l.d45 ≤ 1) = true := by
  decide

/-! ## Mirror symmetry τ -/

/-- **Theorem.** Mirror is an involution: `τ² = id` on every label
in `atlasLabels`. Mirror of
`atlas_correspondence::tests::mirror_is_involution`.

Proven by `decide` over the 96 specific labels: each has
`l.e7 ∈ {0, 1}`, so `1 − (1 − l.e7) = l.e7` in `Nat`. -/
theorem all_mirror_is_involution :
    atlasLabels.all (fun l => decide (l.mirror.mirror = l)) = true := by
  decide

/-- **Theorem.** Pointwise involution for every label in the list.
Equivalent statement to `all_mirror_is_involution` but in
quantifier form, useful for downstream proofs. -/
theorem mirror_is_involution (l : AtlasLabel) (h : l ∈ atlasLabels) :
    l.mirror.mirror = l := by
  fin_cases h <;> rfl

/-- **Theorem.** Mirror permutes the label set: applying τ to any
label in `atlasLabels` yields another label in `atlasLabels`.
Mirror of `atlas_correspondence::tests::mirror_permutes_atlas_labels`. -/
theorem all_mirror_in_labels :
    atlasLabels.all (fun l => decide (l.mirror ∈ atlasLabels)) = true := by
  decide

/-- **Theorem.** Pointwise mirror-membership: for every label `l` in
`atlasLabels`, `τ(l)` is also in the list. -/
theorem mirror_permutes (l : AtlasLabel) (h : l ∈ atlasLabels) :
    l.mirror ∈ atlasLabels := by
  fin_cases h <;> decide

/-! ## Spot checks -/

/-- **Theorem.** First label in canonical order: `(0, 0, 0, -1, 0, 0)`. -/
theorem first_label :
    atlasLabels.head? = some { e1 := 0, e2 := 0, e3 := 0, d45 := -1, e6 := 0, e7 := 0 } := by
  decide

/-- **Theorem.** Last label in canonical order: `(1, 1, 1, +1, 1, 1)`. -/
theorem last_label :
    atlasLabels.getLast? = some { e1 := 1, e2 := 1, e3 := 1, d45 := 1, e6 := 1, e7 := 1 } := by
  decide

/-! ## Atlas-96 adjacency (Phase 2 O2 — Atlas side formalised)

The adjacency on Atlas-96 is Hamming-1 flips, **excluding** `e7`
(mirror is a global symmetry, not an edge). The exact rule, as
implemented by `atlas-embeddings/src/atlas/mod.rs::compute_neighbors`:

- Flip `e1`, `e2`, `e3`, or `e6` (four binary flips, four neighbours)
- Flip `e4` or `e5` via `d45` canonicalisation:
  - `flip_e4`: `{-1, +1} ↦ 0`, `0 ↦ +1`
  - `flip_e5`: `{-1, +1} ↦ 0`, `0 ↦ -1`

When `d45 = 0`, the two ternary flips give distinct neighbours
(`+1` and `-1`), yielding degree 6.
When `d45 = ±1`, both ternary flips collapse to `d45 = 0`,
yielding degree 5.
-/

/-- The d45 transformation when `e4` is flipped. -/
def flipD45ByE4 (d : Int) : Int :=
  if d = 0 then 1 else 0

/-- The d45 transformation when `e5` is flipped. -/
def flipD45ByE5 (d : Int) : Int :=
  if d = 0 then -1 else 0

/-- Up-to-6 candidate neighbours of a label under Hamming-1 flips
(excluding `e7`). The list may contain duplicates when `d45 = ±1`;
de-duplicate via `List.eraseDup` to get the actual neighbour set. -/
def neighbourCandidates (l : AtlasLabel) : List AtlasLabel :=
  [ { l with e1 := 1 - l.e1 },
    { l with e2 := 1 - l.e2 },
    { l with e3 := 1 - l.e3 },
    { l with e6 := 1 - l.e6 },
    { l with d45 := flipD45ByE4 l.d45 },
    { l with d45 := flipD45ByE5 l.d45 } ]

/-- Distinct neighbours of `l`, with `l` itself removed (a vertex is
not its own neighbour) and duplicates merged. -/
def neighbours (l : AtlasLabel) : List AtlasLabel :=
  ((neighbourCandidates l).eraseDups).filter (· ≠ l)

/-- Vertex degree = number of distinct neighbours. -/
def degree (l : AtlasLabel) : Nat := (neighbours l).length

/-- **Theorem.** Every vertex has degree 5 or 6. Mirror of the
`atlas-embeddings/src/atlas/mod.rs:96-103` doc-test claim
`assert!(deg == 5 || deg == 6)`. -/
theorem degree_is_5_or_6 :
    atlasLabels.all (fun l => decide (degree l = 5 ∨ degree l = 6)) = true := by
  decide

/-- **Theorem.** A vertex has degree 6 iff `d45 = 0`. The 32
"interior" ternary vertices (where both `+1` and `-1` are reachable
via single flips) versus the 64 "boundary" vertices at `d45 = ±1`. -/
theorem degree_6_iff_d45_zero :
    atlasLabels.all (fun l => decide (degree l = 6 ↔ l.d45 = 0)) = true := by
  decide

/-- **Theorem.** Total edges in the Atlas-96 graph = 256.

Sum of degrees = `32·5 + 32·6 + 32·5 = 512`. Each undirected edge
contributes 2 to the degree sum, so edge count = `512 / 2 = 256`. -/
theorem total_edges_is_256 :
    (atlasLabels.map degree).sum / 2 = 256 := by
  decide

/-- **Theorem.** Mirror symmetry τ does not introduce edges:
no vertex is adjacent to its own τ-image. (Mirror pairs are not
neighbours per `atlas-embeddings/src/atlas/mod.rs:559-562`.) -/
theorem mirror_pairs_are_not_neighbours :
    atlasLabels.all (fun l => decide (l.mirror ∉ neighbours l)) = true := by
  decide

/-! ## R_TO_ATLAS_INDEX — identity-placeholder contract

The Rust crate's `crate::atlas_correspondence::R_TO_ATLAS_INDEX` is an
identity permutation (`R_TO_ATLAS_INDEX[i] = i`) until the Phase 2 O2
edge-preserving bijection is supplied. The compile-time mutual-inverse
assert in Rust catches non-bijective values but accepts any bijective
permutation — including a non-identity sabotage swap that silently
changes the contract.

This module pins the contract on the Lean side: a theorem stating the
placeholder is the identity. If the Rust crate later supplies a
non-identity permutation, this theorem must be updated in lockstep
with the Rust constant. The mismatch — Lean still asserting identity
while Rust reports a permutation — surfaces in the conformance gauntlet.
-/

/-- The Lean mirror of `crate::atlas_correspondence::R_TO_ATLAS_INDEX`.
Currently the identity permutation; updated when Phase 2 O2 supplies
the edge-preserving bijection. -/
def rToAtlasIndex (i : Nat) : Nat := i

/-- **Theorem.** `rToAtlasIndex` is the identity on `[0, R_CARDINALITY)`.
Mirrors the Rust contract for the Phase 1/2 placeholder.

When the Phase 2 O2 work supplies a non-identity bijection, this
theorem flips to a `decide` over the new table, and the Rust constant
and this Lean definition must be updated in the same change. -/
theorem rToAtlasIndex_is_identity_placeholder :
    ∀ i, i < rCardinality → rToAtlasIndex i = i := by
  intro _ _
  rfl

/-- **Theorem.** `rToAtlasIndex` is bijective on `[0, R_CARDINALITY)`.
Trivial corollary of the identity-placeholder, but stated separately
so the inverse pairing assertion survives a Phase 2 O2 swap unchanged. -/
theorem rToAtlasIndex_bijective_on_range :
    ∀ i j, i < rCardinality → j < rCardinality →
      rToAtlasIndex i = rToAtlasIndex j → i = j := by
  intro _ _ _ _ h
  exact h

/-! ## Phase 2 O2 — R(360) side, open

The Atlas-96 adjacency is now formally specified above. To complete
the edge-preserving correspondence, the **next step** is to
identify the natural adjacency relation on `R(360)` whose graph is
isomorphic to the Atlas-96 graph defined here.

Constraints any candidate must satisfy:

- 96 vertices (matches `R360.elements.length`).
- Same degree sequence: 32 vertices of degree 6 and 64 vertices of
  degree 5.
- Same edge count: 256 undirected edges.
- The edge-preserving bijection `π : R(360) → atlasLabels` must
  send adjacent R(360) elements to adjacent atlasLabels and
  vice versa.

Candidate adjacencies on R(360) to test:

1. **CRT-coordinate adjacency.** Decompose `n ∈ R(360)` via
   `U(360) ≅ U(8) × U(9) × U(5)`, then declare two integers
   adjacent iff they differ in exactly one CRT factor by a
   "minimal step" in that factor's natural cyclic structure.
2. **Multiplicative generator adjacency.** Pick a small set of
   multiplicative generators of `U(360)` (e.g., `7, 11`); declare
   `n ~ m` iff `n · g ≡ m (mod 360)` for some `g` in the set.
3. **(u_index, layer)-coordinate adjacency.** Decompose `n` via the
   Phase 1 bijection `R360.decompose`; declare adjacency by
   Hamming-1 flips on the bit-encoding of the U(30)-index plus
   cyclic moves on the layer.

Each candidate generates a graph; comparing the resulting degree
sequence and edge structure to Atlas-96 narrows the search.

The work is genuinely open mathematics. The committee handing
`SPEC_REQUEST.md` to Gary handles the prime-counting side; the
analogue for this question is to escalate to Alex when
communication resumes:

> What natural adjacency relation on `R(360) = U(360)` corresponds
> to Hamming-1 flips on `Atlas::Label::(e1, e2, e3, d45, e6)` in
> `atlas-embeddings`?

Until that question is answered, `R_TO_ATLAS_INDEX` in the Rust
crate retains its identity placeholder, with the disclaimer in
`atlas_correspondence.rs` updated to cite this Lean module's
adjacency formalisation as the Phase 2 progress. -/

end PrimeFoundations.AtlasCorrespondence
