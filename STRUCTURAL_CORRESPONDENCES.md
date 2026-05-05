# Structural correspondences — Phase 2 O3 verdict

This document closes Phase 2 objective O3 by ruling on the two
structural-correspondence claims Phase 1 disclaimed as "numerical
match, suggestive only":

1. **`OctonionAlgebra ↔ U(30)`** — `dim(𝕆) = 8 = |U(30)|`.
2. **`TriadProjection ↔ Z/3 factor of Atlas-96`** — both have
   cardinality 3.

For each, the verdict is one of: **proven structural** (a specific
canonical bijection or homomorphism is exhibited), **refuted**
(an obstruction is exhibited), or **classified-as-coincidence**
(no obstruction found, but no canonical map either; cardinalities
agree by independent constructions).

The output of O3 is this document plus updates to the disclaimer
language in `src/lib.rs`, `src/ring_substrate.rs`, and `src/r360.rs`.

---

## Claim 1 — `OctonionAlgebra ↔ U(30)`

**Verdict: classified-as-coincidence.**

### The numerical match

`|U(30)| = φ(30) = φ(2)·φ(3)·φ(5) = 1·2·4 = 8`.
`dim_ℝ(𝕆) = 8` (the octonion algebra over the reals).

Both 8s are real. The question is whether a canonical structure-
preserving bijection between the 8-element multiplicative group
`U(30)` and the 8-element basis of `𝕆` exists.

### The obstruction at the multiplicative-group level

`U(30)` is an abelian group of order 8 under multiplication mod 30.
By the Chinese Remainder Theorem applied to `30 = 2·3·5`:

```
U(30)  ≅  U(2) × U(3) × U(5)
       ≅  {1} × ℤ/2 × ℤ/4
       ≅  ℤ/2 × ℤ/4
```

In particular, `U(30)` contains an element of order 4 (any generator
of the `U(5)` factor — for example, `7 mod 30`, since `7² = 49 ≡ 19`,
`7³ = 343 ≡ 13`, `7⁴ = 2401 ≡ 1 (mod 30)`).

The octonion basis `{1, e₁, e₂, …, e₇}` does **not** form a group
under octonion multiplication. The multiplicative closure
`{±1, ±e₁, …, ±e₇}` is a 16-element non-associative loop (the Cayley
group), not a group of order 8. So there is no group of order 8
canonically associated with the octonions to compare `U(30)`
against.

If we stretch and say "compare `U(30)` against the **set** of 8
basis elements," then the question reduces to: is there a natural
bijection between the eight residues `{1, 7, 11, 13, 17, 19, 23, 29}`
and the eight basis elements `{1, e₁, e₂, e₃, e₄, e₅, e₆, e₇}`
that preserves any structure? No such map is canonical; any
bijection works at the set level, and structure-preserving maps
require choosing a multiplication operation, and the natural
candidates do not match.

### The independent origins of the two 8s

The match is best understood as two unrelated 8s:

- `|U(30)| = 8` because 30 is the smallest non-trivial squarefree
  product of primes (`30 = 2·3·5`). Specifically, `φ(30) = 8` is
  forced by Euler's totient on three coprime prime factors.
- `dim(𝕆) = 8` because of **Hurwitz's theorem**: the only normed
  division algebras over ℝ have dimensions 1, 2, 4, 8 — and 8 is
  the maximum, beyond which the doubling Cayley-Dickson construction
  loses the alternativity property. This is a fact about quadratic
  forms and division-algebra structure, not about primes.

Both 8s are forced by their respective contexts; neither
construction references the other. The match is mathematically
compatible (no contradiction) but does not arise from a deeper
common cause — at least, none we can identify without further
input.

### What would change the verdict

A specific canonical map `π: U(30) → {1, e₁, …, e₇}` that:
- Is bijective (set-level), and
- Carries some named structure on `U(30)` (multiplicative cosets,
  Galois action, twin-prime channels, etc.) onto a named structure
  on the octonion basis (Fano-plane multiplication rules, the G₂
  automorphism action, Cayley-Dickson levels).

Until such a `π` is exhibited and proved structure-preserving in
both directions, the match remains a numerical coincidence.

### Source updates

- `src/lib.rs` — research-target language updated to record the
  verdict.
- `src/ring_substrate.rs` — companion-structures section updated
  similarly.
- `STRUCTURAL_CORRESPONDENCES.md` (this file) is the canonical
  reference cited in source comments.

---

## Claim 2 — `TriadProjection ↔ Z/3 factor of Atlas-96`

**Verdict: cardinality-only, structural verdict open.**

### The cardinality match

UOR's `query::TriadProjection` is a typed enum with three named
individuals (per `spec/src/namespaces/query.rs:259–293` and the
two adjacent declarations):

- `Stratum` — vertical layering in the ring geometry
- `Spectrum` — spectral component
- `Address` — content-addressable position

The Atlas-96 graph, per `atlas-embeddings::Atlas::Label`, has
six coordinates `(e₁, e₂, e₃, d₄₅, e₆, e₇)` where `d₄₅ ∈ {−1, 0, +1}`
is the ternary coordinate. Atlas-96 ≅ `(ℤ/2)⁵ × ℤ/3` as a vertex
set with cardinality `32 × 3 = 96`.

Both sides carry a real Z/3 (three Stratum/Spectrum/Address values;
three d₄₅ values). The cardinalities match.

### Why this is more promising than the octonion claim

Unlike the octonion case, the Z/3 here is genuine on both sides:

- TriadProjection's three variants are not arbitrary — they are
  three specific coordinate-system projections that the UOR
  `CoordinateQuery` can extract. The **typed** nature of this
  enumeration means there's structural content: each variant has
  a specific role in the query pipeline.
- Atlas-96's Z/3 factor is also typed: `d₄₅` represents the `e₄ − e₅`
  difference, which carries algebraic meaning (it's the parameter
  along which the triality of Spin(8) acts in `atlas-embeddings`'s
  formalism).

A canonical bijection might exist:

```
Stratum   ↔   d₄₅ = ?   (ring-geometry layer index)
Spectrum  ↔   d₄₅ = ?   (spectral / harmonic component)
Address   ↔   d₄₅ = ?   (content-addressable position)
```

But the specific assignment requires either (a) an explicit
statement in the `atlas-embeddings` source documenting which
TriadProjection corresponds to which `d₄₅` value, or (b) a derivation
from the Spin(8)-triality outer automorphism action on
`{vector, half-spin⁺, half-spin⁻}` representations.

This crate does not currently have either, so the match remains
**cardinality-only**.

### Why this is not yet proven structural

Three things are missing:

1. **The map.** Which TriadProjection corresponds to which
   `d₄₅ ∈ {−1, 0, +1}`?
2. **The structural property preserved.** Z/3 has a natural cyclic
   action; TriadProjection variants do not obviously have a cyclic
   ordering. If the bijection is to be canonical, the cyclic
   structure on the Z/3 side must correspond to some natural
   permutation of `{Stratum, Spectrum, Address}`.
3. **Independent verification.** A theorem in either Lean or
   `atlas-embeddings`'s formal corpus stating the bijection and the
   preserved structure.

### What would change the verdict

A statement in `atlas-embeddings`'s spec or in UOR Foundation's
`query` namespace documentation linking specific d₄₅ values to
specific TriadProjection variants, plus a stated invariant
(e.g., "the triality outer automorphism cycles the three projections
in the same order as `+1 → 0 → −1`"). Such a statement would close
the verdict to **proven structural**. Until then it stays
**cardinality-only**.

### Source updates

- `src/r360.rs` — research-note language updated.
- `STRUCTURAL_CORRESPONDENCES.md` (this file) is the canonical
  reference.

### Open question to escalate

This is suitable to ask Alex directly when communication resumes
(separately from the four `SPEC_REQUEST.md` items, which target
Gary):

> Does `atlas-embeddings::Atlas::Label::d45` correspond to UOR's
> `query::TriadProjection` enum? Specifically, is there an intended
> map `{−1, 0, +1} → {Stratum, Spectrum, Address}` that preserves
> the Spin(8)-triality cyclic action?

---

## Summary table

| Correspondence | Verdict | Strongest evidence |
|---|---|---|
| `OctonionAlgebra ↔ U(30)` | **Classified-as-coincidence** | `U(30) ≅ ℤ/2 × ℤ/4` has element of order 4; octonion basis under multiplication is non-associative loop of order 16. The two 8s arise from independent constructions (totient vs Hurwitz). |
| `TriadProjection ↔ Z/3 of Atlas` | **Cardinality-only** | Both sides have real, typed Z/3 structure. A canonical bijection plausibly exists but is not currently documented; needs Alex's input or a derivation from Spin(8)-triality. |

Phase 2 O3 is closed by this verdict. The disclaimer language in
the source files is updated to reflect "classified-as-coincidence"
or "cardinality-only" rather than "research target," matching the
honesty discipline established in Phase 1.
