# Results — Phase 1 strict, two parameters in, every fact out

## Anchoring

Every constant, every theorem, and every numerical claim in this
crate derives from exactly two irreducible parameters:

```
MODULUS_U     = 30      (Gary's residue modulus, = 2·3·5)
NUMERAL_BASE  = 10      (decimal numeral system)
```

Anything that cannot be derived from these two values via the
standard helpers (gcd, lcm, totient, digital root, Fibonacci
recurrence, finite enumeration) has been removed. The Master-Clock
identity `π(N) = D(N) − TF(N) + RF(N) + 3` and its calibration
constants χ, ζ, δ, k were audited and cut — they appear only in
private correspondence, not in any of Gary's published artifacts,
and the formulas that introduce them do not arithmetically
reproduce their own tabulated values across five iterations of his
spec. See `sheets_findings.md` and `primesdemystified_scrape.md`.

What remains is strict and tight.

## Phase 1 — derived constants

| Constant | Derivation | Value |
|---|---|---|
| `MODULUS_DR` | `NUMERAL_BASE − 1` | 9 |
| `PISANO_PERIOD_MOD_DR` | minimal `p` with `F_p ≡ 0` and `F_{p+1} ≡ 1 (mod 9)` | 24 |
| `U30_CARDINALITY` | `φ(MODULUS_U)` | 8 |
| `MODULUS_R` | `lcm(MODULUS_U, MODULUS_DR, PISANO_PERIOD_MOD_DR)` | 360 |
| `LATTICE_LAYERS` | `MODULUS_R / MODULUS_U` | 12 |
| `R_CARDINALITY` | `U30_CARDINALITY · LATTICE_LAYERS` | 96 |
| `MODULUS_U90` | `3 · MODULUS_U` | 90 |
| `U90_CARDINALITY` | enumerate `n ≤ 90` with `gcd(n, 30) = 1` | 24 |
| `F49` | `Nat.fib 49` | 7,778,742,049 |
| `SUM_DR` | `Σ dr(n)` over `R(360)` | 432 |
| `SUM_FIBO_DR` | `Σ dr(F_n)` over `R(360)` | 432 = `LATTICE_LAYERS · 36` |

Every value is computed in `const fn` at compile time and
cross-checked by `const _: () = assert!(…)`. The build fails on any
derivation drift.

## Phase 1.5 — `U_30(90)` substrate and `(Z/9Z)*`

The 24-element residue set `U_30(90)` and the 6-element multiplicative
group `(Z/9Z)*` together account for every structural symmetry that
the source website displays as the "Magic Mirror Matrix" and "Vedic
Square." Both reduce to standard finite-group facts, anchored to one
parameter each (`MODULUS_U = 30` and `MODULUS_DR = 9`).

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 1 | `\|U_30(90)\| = 24` | `u90::U90_CARDINALITY` | `U90.cardinality_u90` |
| 2 | `U_30(90) = {1,7,11,…,89}` matches reference table | `u90::RESIDUES_U90` | `U90.residues_match_gary` |
| 3 | digital-root cycle has period 24 over `U_30(90)` | `u90::DR_CYCLE` | `U90.dr_cycle_matches_pdf` |
| 4 | dr cycle is 4-uniform over `(Z/9Z)*`, zero on `{3,6,9}` | `u90::DR_CYCLE_UNIFORMITY` | `U90.dr_cycle_*_count` |
| 5 | `(Z/9Z)* = {1,2,4,5,7,8}` cyclic of order 6, generator 2 | `units_z9::POWERS_OF_2_MOD_9` | `UnitsZ9.two_generates_units_z9` |
| 6 | 6×6 multiplication table mod 9 closed; row/col sum 27; total 162 | `units_z9::MULT_TABLE_Z9` | `UnitsZ9.mult_table_total_eq_162` |
| 7 | `Σ U_30(90) = 1080 = 3 · MODULUS_R` (row sum of 24×24 mod-90 matrix) | `u90::ROW_SUM_LINEAR` | `U90.linear_sum_eq_three_modulus_r` |
| 8 | Diagonal of 24×24 → 6 distinct residues `{1,19,31,49,61,79}`, each ×4, sum 960 | `u90::SQUARE_RESIDUE_VALUES`, `u90::DIAGONAL_SUM` | `U90.diagonal_*` |

Tiling identity: `|U_30(90)| · (MODULUS_R / MODULUS_U90) = 24 · 4 = 96 = |R(360)|`.

Standalone reproduction:

```sh
rustc -O validate_units_z9.rs -o validate_units_z9
./validate_units_z9
```

All eight facts validated in under a millisecond.

## Phase 3 — twin prime residue constraints

Necessary mod-30 conditions on twin prime pairs `(p, p + 2)`,
derived from `U(30)` and `gcd` only.

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 1 | Twin-prime lower residues mod 30 = `{11, 17, 29}` | `twin_prime_residues::TWIN_PRIME_LOWER_RESIDUES` | `TwinPrimeResidues.twin_lower_has_coprime_successor` |
| 2 | Isolated lower residues mod 30 = `{1, 7, 13, 19, 23}` | `twin_prime_residues::ISOLATED_LOWER_RESIDUES` | `TwinPrimeResidues.isolated_lower_has_noncoprime_successor` |
| 3 | The two sets partition `U(30)`: `3 + 5 = 8` | `twin_prime_residues::PARTITION_CHECK` | `TwinPrimeResidues.partition_covers_u30` |
| 4 | Twin residue pairs: `(11,13)`, `(17,19)`, `(29,1)` | `twin_prime_residues::TWIN_PRIME_RESIDUE_PAIRS` | `TwinPrimeResidues.pairs_offset_by_two` |
| 5 | The predicate `is_twin_prime_candidate_lower` accepts exactly the residues `{11, 17, 29}` | `twin_prime_residues::is_twin_prime_candidate_lower` | `TwinPrimeResidues.predicate_matches_residue_set` |

Empirical anchor — 33 twin prime pairs found in `[7, 1000]`, every
single one with `p mod 30 ∈ {11, 17, 29}`. Distribution roughly
balanced (13 / 9 / 11), consistent with Dirichlet equidistribution.

Standalone reproduction (extended rigor verifier — 11 check classes,
100,000-element sweep):

```sh
rustc -O verify_twin_rigor.rs -o verify_twin_rigor
./verify_twin_rigor
```

Validates the five claims plus six additional rigor classes
(production-const-fn replay, edge cases, predicate-array agreement,
non-coprime rejection, large-N empirical sweep) in 5 ms.

This is a **necessary** condition — passing the predicate doesn't
certify that `(p, p+2)` is twin prime; it just rules out 5/8 of
candidate `p` before any primality test. The wheel-30 sieve in
[`crate::sieve`] still does the actual primality work.

### Phase 3 — Sophie Germain residue constraints

`(p, 2p+1)` both prime, `p > 5` ⟹ `p mod 30 ∈ {11, 23, 29}`. Same
shape as the twin constraint, derived by enumerating `r ∈ U(30)`
keeping those with `gcd((2r+1) mod 30, 30) = 1`.

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 1 | SG lower residues mod 30 = `{11, 23, 29}` | `sophie_germain_residues::SOPHIE_GERMAIN_LOWER_RESIDUES` | `SophieGermainResidues.sg_lower_has_coprime_doubled_successor` |
| 2 | SG isolated residues mod 30 = `{1, 7, 13, 17, 19}` | `sophie_germain_residues::SG_ISOLATED_RESIDUES` | `SophieGermainResidues.sg_isolated_has_noncoprime_doubled_successor` |
| 3 | Partition `3 + 5 = 8` | `sophie_germain_residues::SG_PARTITION_CHECK` | `SophieGermainResidues.sg_partition_covers_u30` |
| 4 | Pairs `(11, 23)`, `(23, 17)`, `(29, 29)` | `sophie_germain_residues::SG_RESIDUE_PAIRS` | `SophieGermainResidues.sg_pairs_form_2p_plus_1` |

Empirical anchor: 187 Sophie Germain primes in `[7, 10000]`, every
one with `p mod 30 ∈ {11, 23, 29}`. Distribution `60 / 65 / 62`.

### Phase 3 — Siamese prime generator pattern

`(n²−2, n²+2)` both prime ⟹ `n mod 30 ∈ {3, 9, 15, 21, 27}`. The
generators are NOT in U(30) — they are odd multiples of 3, the
sequence `6k + 3` for `k ∈ {0, 1, 2, 3, 4}` (OEIS
[A016945](https://oeis.org/A016945)).

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 1 | Siamese generator residues = `{3, 9, 15, 21, 27}` | `siamese_prime_pattern::SIAMESE_GENERATOR_RESIDUES` | `SiamesePrimePattern.siamese_generator_minus_two_coprime_to_30` |
| 2 | Closed form `6k + 3` for `k ∈ {0..4}` | `siamese_prime_pattern::SIAMESE_AS_6K_PLUS_3` | `SiamesePrimePattern.siamese_residues_match_6k_plus_3` |
| 3 | Equivalent predicate `n mod 6 == 3` | `siamese_prime_pattern::is_siamese_generator_via_mod6` | `SiamesePrimePattern.siamese_predicates_equivalent_mod_30` |

Empirical anchor: 20 Siamese pairs in `[3, 1000]`, every one with
`n mod 30 ∈ {3, 9, 15, 21, 27}`.

### Phase 3 — Perfect squares mod 30 and mod 9

For every `n ∈ U(30)`, the square `n²` reduces to one of two
mod-30 residues and one of three digital roots:

```
n² mod 30 ∈ {1, 19}     (each value hit by 4 of the 8 U(30) members)
dr(n²)    ∈ {1, 4, 7}    (the squares of (Z/9Z)*)
```

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 1 | `n² mod 30 ∈ {1, 19}` for `n ∈ U(30)` | `perfect_squares_mod30::SQUARE_MOD30_RESIDUES` | `PerfectSquaresMod30.squares_mod_30_in_one_or_nineteen` |
| 2 | `dr(n²) ∈ {1, 4, 7}` for `n ∈ U(30)` | `perfect_squares_mod30::SQUARE_DR_VALUES` | `PerfectSquaresMod30.squares_dr_in_one_four_seven` |
| 3 | Each mod-30 square residue hit 4 times | `perfect_squares_mod30::SQUARE_MOD30_MULTIPLICITY` | `PerfectSquaresMod30.each_square_residue_hit_4_times` |
| 4 | DR cosets `{1,4,7} ⊔ {2,5,8} ⊔ {3,6,9}` partition `Z/9Z` | `perfect_squares_mod30::SQUARE_DR_COSET` etc. | (compile-time assert) |

Empirical anchor: 1,226 primes tested in `[7, 10000]`, every square
satisfies both predicates. Zero violations.

### Phase 3 — U_30(90) secondary diagonal of the 24×24 mod-90 matrix

Extension to `u90.rs`. The 24×24 mod-90 multiplication matrix has
two structural diagonals:

```
primary diagonal:   (n²) mod 90       distinct = {1, 19, 31, 49, 61, 79}    sum = 960
secondary diagonal: (90 − n²) mod 90  distinct = {11, 29, 41, 59, 71, 89}   sum = 1200
combined:                                                                     sum = 2160 = 6 · MODULUS_R
```

The secondary-diagonal residues are all prime: `{11, 29, 41, 59, 71, 89}`.

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 5 | Secondary diagonal distinct values = `{11, 29, 41, 59, 71, 89}` | `u90::SECONDARY_DIAGONAL_VALUES` | `U90.secondary_diagonal_residues_eq_six` |
| 6 | Secondary diagonal sum = 1200 | `u90::SECONDARY_DIAGONAL_SUM` | `U90.secondary_diagonal_sum_eq_1200` |
| 7 | Combined diagonal sum = `6 · MODULUS_R` = 2160 | `u90::DIAGONAL_PAIR_SUM` | `U90.combined_diagonal_sum_eq_six_modulus_r` |

Standalone reproduction of the entire Phase 3 batch:

```sh
rustc -O verify_phase3_batch.rs -o verify_phase3_batch
./verify_phase3_batch
```

Validates all four modules empirically in milliseconds.

## Phase 4 — index bijection, Fibonacci reciprocal, factorization dyads

Three structural identities, each derived from existing Phase 1
modules. Every claim compile-time-checked, every theorem decidable.

### Phase 4.1 — bijection `k ↔ k`-th member of `U_30(∞)`

Closed-form bijection between 1-indexed positions and the ascending
sequence of integers coprime to 30:

```
nth_u30_member(k) = MODULUS_U · ⌊(k − 1) / 8⌋ + RESIDUES[(k − 1) mod 8]
```

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 1 | Forward map is a `const fn` from `(MODULUS_U, RESIDUES)` | `index_to_residue::nth_u30_member` | `IndexToResidue.nthU30Member` |
| 2 | Inverse map via mod-30 lookup | `index_to_residue::index_of_u30_member` | `IndexToResidue.indexOfU30Member` |
| 3 | Anchor: `nth_u30_member(2112) = 7919` (1000th prime) | (compile-time assert) | `IndexToResidue.nth_at_2112_eq_7919` |
| 4 | Anchor: `nth_u30_member(24) = 89` | (compile-time assert) | `IndexToResidue.nth_at_twentyfour` |
| 5 | Round-trip on every `k ∈ [1, 96]` and every `n ≤ 360` coprime to 30 | (compile-time assert) | `IndexToResidue.{forward_inverse,inverse_forward}_round_trip` |

Empirical anchor: 2,666 round-trip checks across every `U_30(∞)`
member up to 10,000, zero defects.

### Phase 4.2 — `1/89 = Σ_{n≥1} F_n / 10^(n+1)`

The Fibonacci generating function `f(x) = x / (1 − x − x²)` evaluated
at `x = 1/10` gives `f(1/10) = 10/89`, so `Σ F_n / 10^n = 10/89` and
the headline form is `Σ F_n / 10^(n+1) = 1/89`.

In integer arithmetic the partial sum
`S(N) = Σ_{n=1..N} F_n · 10^(N−n)` satisfies `89 · S(N) ≤ 10^(N+1)`
with gap bounded by `178 · F_(N+1)`.

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 1 | `89` derived as `b² − b − 1` from `b = NUMERAL_BASE = 10` | `fibonacci_reciprocal::RECIPROCAL_DENOMINATOR` | `FibonacciReciprocal.reciprocal_denominator_eq_89` |
| 2 | `partial_numerator(10) = 1,123,595,495` (matches website's 10-digit truncation) | (compile-time assert) | `FibonacciReciprocal.partial_at_10` |
| 3 | Gap `10^(N+1) − 89·S(N)` positive at `N ∈ {5, 10, 15, 17}` | (compile-time assert) | `FibonacciReciprocal.gap_positive_at_*` |
| 4 | Gap bounded by `178·F_(N+1)` at the same `N` | (compile-time assert) | `FibonacciReciprocal.gap_below_bound_at_*` |
| 5 | Gap shrinks faster than 10× per step (integer convergence) | (test) | `FibonacciReciprocal.gap_shrinks_per_step_5_to_10` |

Empirical anchor at `N = 17` (`u128`): `89·S(17) = 999,999,999,999,972,563`
vs `10^18 = 10^(N+1)`, gap = 27,437, well under bound 459,952.

### Phase 4.3 — 576 = 24² factorization dyads

Every ordered pair `(a, b) ∈ U_30(90) × U_30(90)` produces
`(a · b) mod 90 ∈ U_30(90)` (group closure). The 576 dyads
partition into 24 fibers of size 24, one per residue class.

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 1 | Total dyad count = `24² = 576` | `factorization_dyads::DYAD_COUNT` | `FactorizationDyads.dyad_count_eq_576` |
| 2 | Group closure: every product in `U_30(90)` | (compile-time assert) | `FactorizationDyads.closure_under_mod90_multiplication` |
| 3 | Each fiber has size 24 | `factorization_dyads::FIBER_SIZE` | `FactorizationDyads.fiber_size_eq_24` |
| 4 | Diagonal residues match `SQUARE_RESIDUE_VALUES = {1, 19, 31, 49, 61, 79}` | `factorization_dyads::DIAGONAL_DYADS` | (test) |

Standalone reproduction of the entire Phase 4 batch:

```sh
rustc -O verify_phase4_batch.rs -o verify_phase4_batch
./verify_phase4_batch
```

Combined run: 2,666 bijection round-trips + 13 partial-sum gap
checks + 576 dyad enumeration. All in under one second.

## Phase 5 — triangular numbers, period-24 dr sequences, replicants

Three more website-derived modules, each anchored back to existing
Phase 1 / 1.5 / 4 substrate.

### Phase 5.1 — triangular numbers and the `MODULUS_R` algebra

`k(n) = n · (n + 1) / 2` (OEIS A000217) at the U_30(∞) anchors
`89` (24th member), `109` (30th member), and `7919` (1000th prime
at index 2112).

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 1 | `k(89) = 4,005` | `triangular_numbers::TRIANGULAR_AT_24TH` | `TriangularNumbers.triangular_at_89` |
| 2 | `k(109) = 5,995` | `triangular_numbers::TRIANGULAR_AT_30TH` | `TriangularNumbers.triangular_at_109` |
| 3 | `k(7919) = 31,359,240` | `triangular_numbers::TRIANGULAR_AT_THOUSANDTH_PRIME` | `TriangularNumbers.triangular_at_7919` |
| 4 | `89² − 1 = 22 · MODULUS_R` | `triangular_numbers::SQUARE_MINUS_ONE_AT_24` | `TriangularNumbers.square_minus_one_at_24` |
| 5 | `109² − 89² = 11 · MODULUS_R` | `triangular_numbers::SQUARE_DIFFERENCE_30_24` | `TriangularNumbers.square_difference_30_24` |
| 6 | Pyramid product `2112 · 11 · MODULUS_R = 8,363,520` | `triangular_numbers::PYRAMID_PRODUCT` | `TriangularNumbers.pyramid_product` |

### Phase 5.2 — period-24 digital-root sequences across `U_30(90)`

For each `r ∈ U_30(90)`, the row `s_r(k) = dr(r · U_30(90)[k] mod 90)`
is a permutation of [`crate::u90::DR_CYCLE`]. The 24×24 table
inherits the row sum 108 and column sum 108 from the dr cycle's
4-uniformity over `(Z/9Z)*`.

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 1 | Every row sums to 108 | `period24_dr_sequences::DR_ROW_SUM` | `Period24DrSequences.every_row_sums_to_108` |
| 2 | Every column sums to 108 | (compile-time assert) | (test) |
| 3 | Total table = 2592 = 6·432 | `period24_dr_sequences::DR_TABLE_TOTAL` | `Period24DrSequences.total_eq_2592` |
| 4 | Every entry in `(Z/9Z)*` | (compile-time assert) | `Period24DrSequences.every_entry_in_units_z9` |
| 5 | Row at `r = 1` matches `DR_CYCLE` | (compile-time assert) | (test) |
| 6 | Every row is a permutation of `DR_CYCLE` | (test) | — |

### Phase 5.3 — replicant factorization counts

Two metrics over factorizations `(a, b)` with `a, b ∈ U_30(∞)`,
`7 ≤ a ≤ b`, `a · b ≤ N`:
- **Distinct replicants** — integers with ≥ 2 such factorizations.
- **Duplicate factorizations** — `Σ_{m ≤ N} (count(m) − 1)` over
  `m` with `count(m) ≥ 2` (= `total_pairs − unique_products`).

primesdemystified.com tabulates the second metric.

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 1 | No replicants below 100 | `replicant_factorization::replicant_count_distinct(100) == 0` | (test) |
| 2 | Five replicants at `N = 1000` | `replicant_factorization::REPLICANTS_AT_1000` | `ReplicantFactorization.replicantsAt1000` |
| 3 | `duplicate_factorization_count(1000) = 5` | `replicant_factorization::DUPLICATE_FACTORIZATIONS_AT_1000` | `ReplicantFactorization.duplicateFactorizationsAt1000` |
| 4 | `duplicate_factorization_count(10000) = 408` | `replicant_factorization::DUPLICATE_FACTORIZATIONS_AT_10000` | `ReplicantFactorization.duplicateFactorizationsAt10000` |
| 5 | Distinct replicants at `N = 10000` = 248 (diverges from 408) | (test) | — |
| 6 | Each replicant at `N = 1000` factors via two `U_30(∞)` pairs | (test) | `ReplicantFactorization.replicant_*` |

The two metrics coincide at `N ≤ 1000` because every replicant has
exactly two factorizations there. They diverge at `N = 10000`
because some products acquire three or more factorizations: the
website's `408` counts extras, our `248` counts distinct integers.

Standalone reproduction of the entire Phase 5 batch:

```sh
rustc -O verify_phase5_batch.rs -o verify_phase5_batch
./verify_phase5_batch
```

Combined run: 6 algebraic identities + 24×24 = 576 dr-table cells
+ replicant counts at 100/1000/10000. All in milliseconds.

## Phase 6 — dyad classes: `576 = 36 × 16` partition

The 576 ordered factorization dyads in `U_30(90) × U_30(90)`
partition into exactly 36 classes by digital-root pair, each with
16 members.

```
|U_30(90)|              = 24
|(Z/9Z)*|               = 6
|(Z/9Z)*|²              = 36
each (Z/9Z)* dr appears = 4 times in DR_CYCLE
⟹ class size            = 4 · 4 = 16
⟹ 36 · 16               = 576 = |U_30(90)|²  ✓
```

| # | Statement | Rust | Lean |
|---:|---|---|---|
| 1 | `CLASS_COUNT = 36 = |(Z/9Z)*|²` | `dyad_classes::CLASS_COUNT` | `DyadClasses.class_count_eq_36` |
| 2 | `CLASS_SIZE = 16 = 4²` | `dyad_classes::CLASS_SIZE` | (compile-time assert) |
| 3 | `CLASS_COUNT · CLASS_SIZE = 576` | `dyad_classes::CLASS_COUNTS` total | `DyadClasses.partition_arithmetic` |
| 4 | Every class has size exactly 16 | `dyad_classes::CLASS_COUNTS` | `DyadClasses.each_class_has_16_dyads` |
| 5 | `class_index(dr1, dr2)` is a `const fn` lex-order coordinate query | `dyad_classes::class_index` | (test) |

Verified end-to-end: all 36 classes inspected, each contains
exactly 16 dyads, summing to 576.

This closes the website's coverage. `final_coverage_audit.md`
classifies 23 of 31 distinct claims as ENCODED, 3 as
MISSING-DERIVABLE (two of which were already covered by
`tests/triality_witnesses.rs`'s `{1,4,7}` and `{2,5,8}` cosets,
the third addressed by this Phase 6 module), 2 as
MISSING-EMPIRICAL (literature reference; out of Phase 1 scope),
and 3 as OUT-OF-SCOPE (numerological).

## Phase 2 — wheel-30 sieve gives exact `π(N)`

The published Prime Spiral Sieve, scaled to large `N`, produces
canonical literature `π(N)` at every tested decimal power.

| `k` | `N = 10^k` | `π(N)` computed | OEIS A006880 | match | runtime |
|---:|---:|---:|---:|:---:|---:|
| 4 | 10⁴ | 1,229 | 1,229 | OK | 0.000 s |
| 5 | 10⁵ | 9,592 | 9,592 | OK | 0.000 s |
| 6 | 10⁶ | 78,498 | 78,498 | OK | 0.001 s |
| 7 | 10⁷ | 664,579 | 664,579 | OK | 0.007 s |
| 8 | 10⁸ | 5,761,455 | 5,761,455 | OK | 0.089 s |
| 9 | 10⁹ | 50,847,534 | 50,847,534 | OK | 1.159 s |
| 10 | 10¹⁰ | 455,052,511 | 455,052,511 | OK | 17.65 s |

The published structural identity:

```
π(N) = #{ primes in U(30) ∩ [1, N] } + 3
     = D₃₀(N) − C₃₀(N) + 2
```

Both forms compute identically at every `N` tested. Pure integer
arithmetic on the eight residue classes coprime to 30. No
transcendental functions, no calibration constants, no residual.

Standalone reproduction:

```sh
rustc -O validate_sieve.rs -o validate_sieve
./validate_sieve
```

## What was cut and why

The previous Phase 4 superstructure depended on calibration
constants (χ, ζ, δ, k) that do not derive from `MODULUS_U` or
`NUMERAL_BASE` and do not appear in any published artifact:

| Removed file | Reason |
|---|---|
| `src/master_clock.rs` | defines `CHI_DERIVED`, `CHI_FITTED`, `OMEGA`, `F49` ratchet trait surface |
| `src/tf_function.rs` | uses χ in `(N/lnN) / (χ · ln(lnN))` |
| `src/rf_function.rs` | uses χ, k, F49 in piecewise `e^{-k(N-F49)}` |
| `src/harness.rs` | Master-Clock evaluation harness |
| `src/d_function.rs` | "lattice tally" — orphan once TF/RF go |
| `src/cauchy.rs`, `elementary.rs`, `rational.rs`, `categorical/` | exact-arithmetic infrastructure motivated only by exp/ln in TF/RF |
| Lean: `DFunction.lean`, `Elementary.lean`, `Cauchy.lean`, `Rational.lean` | mirrors of the above |
| `src/li.rs::GARY_CLAIMS`, `compare_li`, `LiAgreement` | tabulated his Master-Clock predictions |
| `Li.lean::garyMasterClockDelta`, `master_falsifier_surface` | structural anchors that no longer derive |

The numerical coincidence at 10¹¹ and 10¹² (Gary's claimed residuals
432 and 96 happen to equal `SUM_DR` and `R_CARDINALITY`) is
preserved as a curiosity in this file but is no longer load-bearing
in the proof architecture. Without a closed-form formula that
arithmetically produces those residuals from `MODULUS_U`, the
coincidence is not derivation.

## Discipline

- `#![no_std]` core, `#![forbid(unsafe_code)]`
- `clippy::float_arithmetic = "deny"` — all arithmetic exact integer or rational
- All clippy warnings as errors
- Every Rust constant in `const fn` with `const _: () = assert!(…)` cross-check
- Every Lean theorem closed by `decide`, `native_decide`, `rfl`, or constructive proof
- Zero `sorry`

## Bottom line

Two parameters: `MODULUS_U = 30`, `NUMERAL_BASE = 10`. Eight Phase 1
constants derive. Eight Phase 1.5 substrate facts follow. The
wheel-30 sieve produces canonical `π(N)` exactly at every tested
decimal power. Everything else has been cut. Nothing in this crate
asserts a fact that cannot be reproduced from the two parameters
and standard number theory.
