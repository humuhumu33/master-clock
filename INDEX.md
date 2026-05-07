# Prime Clock — searchable derivation index

Three navigation axes mirror the substrate's triality:
**by name** (alphabetical, fastest lookup), **by value**
(convergence map — every place a structurally meaningful number
appears), **by phase** (chronological derivation walk).

Two parameters in: `MODULUS_U = 30`, `NUMERAL_BASE = 10`.
Everything below derives from these two.

---

## §1 — By name (alphabetical)

| Name | Module | Value / Type | Derivation | Lean theorem |
|---|---|---|---|---|
| `ANCHOR_24` | `triangular_numbers` | `89` | `nth_u30_member(24)` | `TriangularNumbers.anchor24_eq_89` |
| `ANCHOR_30` | `triangular_numbers` | `109` | `nth_u30_member(30)` | `TriangularNumbers.anchor30_eq_109` |
| `DIAGONAL_PAIR_SUM` | `u90` | `2160` | primary 960 + secondary 1200 = `6 · MODULUS_R` | `U90.combined_diagonal_sum_eq_six_modulus_r` |
| `DIAGONAL_SUM` | `u90` | `960` | `Σ (n²) mod 90` over `U_30(90)` = `4 · 240` | `U90.diagonal_sum_eq_960` |
| `DR_CYCLE` | `u90` | `[1,7,2,4,8,1,5,2,4,1,5,7,2,4,8,5,7,4,8,1,5,7,2,8]` | `dr` applied to `RESIDUES_U90` | `U90.dr_cycle_matches_pdf` |
| `DR_ROW_SUM` | `period24_dr_sequences` | `108` | `4 · UNITS_Z9_SUM` = `4 · 27` | (compile-time assert) |
| `DR_SEQUENCES` | `period24_dr_sequences` | `[[u64; 24]; 24]` | `dr((r · a) mod 90)` for `r, a ∈ U_30(90)` | `Period24DrSequences.every_row_sums_to_108` |
| `DR_TABLE_TOTAL` | `period24_dr_sequences` | `2592` | `24 · DR_ROW_SUM = 6 · SUM_DR` | `Period24DrSequences.total_eq_2592` |
| `DUPLICATE_FACTORIZATIONS_AT_10000` | `replicant_factorization` | `408` | total pairs − unique products | (test) |
| `DUPLICATE_FACTORIZATIONS_AT_1000` | `replicant_factorization` | `5` | same | `ReplicantFactorization.duplicateFactorizationsAt1000` |
| `CLASS_COUNT` | `dyad_classes` | `36` | `\|(Z/9Z)*\|²` | `DyadClasses.class_count_eq_36` |
| `CLASS_COUNTS` | `dyad_classes` | `[usize; 36]` (all 16) | dyad count per dr-pair class | `DyadClasses.each_class_has_16_dyads` |
| `CLASS_SIZE` | `dyad_classes` | `16` | `4²` | (compile-time assert) |
| `DYAD_COUNT` | `factorization_dyads` | `576` | `U90_CARDINALITY²` | `FactorizationDyads.dyad_count_eq_576` |
| `DYADS_TABLE` | `factorization_dyads` | `[(u64,u64,u64); 576]` | every `(a, b, (a·b) mod 90)` | (compile-time assert) |
| `ELEMENTS` | `r360` | `[u64; 96]` | enumerated `n ∈ [1, 360]` with `gcd(n, 30) = 1` | `R360.cardinality_r360` |
| `F49` | `fibonacci` | `7,778,742,049` | `fib(49)` | `Fibonacci.f49_equals_seven_billion` |
| `FIBER_SIZE` | `factorization_dyads` | `24` | `U90_CARDINALITY` | `FactorizationDyads.fiber_size_eq_24` |
| `LATTICE_LAYERS` | `parameters` | `12` | `MODULUS_R / MODULUS_U` | `Parameters.latticeLayers_eq_12` |
| `MODULUS_DR` | `parameters` | `9` | `NUMERAL_BASE − 1` | `Parameters.modulusDR_eq_9` |
| `MODULUS_R` | `parameters` | `360` | `lcm(MODULUS_U, MODULUS_DR, PISANO_PERIOD_MOD_DR)` | `Parameters.modulusR_eq_360` |
| `MODULUS_U` | `parameters` | `30` | **parameter** | (axiom) |
| `MODULUS_U90` | `u90` | `90` | `3 · MODULUS_U` | (compile-time assert) |
| `MULT_TABLE_ROW_SUM` | `units_z9` | `27` | `Σ UNITS_Z9` | (compile-time assert) |
| `MULT_TABLE_TOTAL_SUM` | `units_z9` | `162` | `6 · 27 = UNITS_Z9_CARDINALITY · MULT_TABLE_ROW_SUM` | `UnitsZ9.mult_table_total_eq_162` |
| `MULT_TABLE_Z9` | `units_z9` | `[[u64; 6]; 6]` | `(a · b) mod 9` over `(Z/9Z)*` | `UnitsZ9.mult_table_canonical` |
| `NUMERAL_BASE` | `parameters` | `10` | **parameter** | (axiom) |
| `PISANO_PERIOD_MOD_DR` | `parameters` | `24` | minimal period of `(F_n mod 9)` | `Fibonacci.pisano_period_mod_dr_is_minimal` |
| `POWERS_OF_2_MOD_9` | `units_z9` | `[2,4,8,7,5,1]` | `2^k mod 9` for `k = 1..6` | `UnitsZ9.two_generates_units_z9` |
| `PYRAMID_PRODUCT` | `triangular_numbers` | `8,363,520` | `2112 · 11 · MODULUS_R` | `TriangularNumbers.pyramid_product` |
| `R_CARDINALITY` | `parameters` | `96` | `U30_CARDINALITY · LATTICE_LAYERS` | `Parameters.rCardinality_eq_96` |
| `RESIDUES` | `u30` | `[1,7,11,13,17,19,23,29]` | `gcd(n, 30) = 1` for `n ∈ [1, 30)` | `U30.u30_canonical` |
| `RESIDUES_U90` | `u90` | `[1,7,11,…,89]` (24 entries) | `gcd(n, 30) = 1` for `n ∈ [1, 90]` | `U90.cardinality_u90` |
| `ROW_SUM_LINEAR` | `u90` | `1080` | `Σ U_30(90) = 3 · MODULUS_R` | `U90.linear_sum_eq_three_modulus_r` |
| `SECONDARY_DIAGONAL_SUM` | `u90` | `1200` | `4 · 300 = 4 · (Σ {11,29,41,59,71,89})` | `U90.secondary_diagonal_sum_eq_1200` |
| `SECONDARY_DIAGONAL_VALUES` | `u90` | `[11,29,41,59,71,89]` | `90 − (n² mod 90)` for `n ∈ U_30(90)` | `U90.secondary_diagonal_residues_eq_six` |
| `SQUARE_DIFFERENCE_30_24` | `triangular_numbers` | `3,960` | `109² − 89² = 11 · MODULUS_R` | `TriangularNumbers.square_difference_30_24` |
| `SQUARE_DR_VALUES` | `perfect_squares_mod30` | `[1,4,7]` | `dr(n²)` for `n ∈ U(30)` | `PerfectSquaresMod30.squares_dr_in_one_four_seven` |
| `SQUARE_MINUS_ONE_AT_24` | `triangular_numbers` | `7,920` | `89² − 1 = 22 · MODULUS_R` | `TriangularNumbers.square_minus_one_at_24` |
| `SQUARE_MOD30_RESIDUES` | `perfect_squares_mod30` | `[1, 19]` | `n² mod 30` for `n ∈ U(30)` | `PerfectSquaresMod30.squares_mod_30_in_one_or_nineteen` |
| `SQUARE_RESIDUE_VALUES` | `u90` | `[1,19,31,49,61,79]` | distinct values of `n² mod 90` over `U_30(90)` | `U90.diagonal_residues_eq_six` |
| `SUM_DR` | `appendix_a` | `432` | `Σ dr(n)` over `R(360)` = `LATTICE_LAYERS · 36` | `AppendixA.sum_dr_eq_432` |
| `SUM_FIBO_DR` | `appendix_a` | `432` | `Σ dr(F_n)` over `R(360)` = `LATTICE_LAYERS · 36` | `AppendixA.sum_fibo_dr_eq_432` |
| `TILES_INTO_R360` | `u90` | `96` | `U90_CARDINALITY · 4` | `U90.tiles_into_r360` |
| `TRIANGULAR_AT_24TH` | `triangular_numbers` | `4,005` | `triangular(89) = 89 · 90 / 2` | `TriangularNumbers.triangular_at_89` |
| `TRIANGULAR_AT_30TH` | `triangular_numbers` | `5,995` | `triangular(109) = 109 · 110 / 2` | `TriangularNumbers.triangular_at_109` |
| `TRIANGULAR_AT_THOUSANDTH_PRIME` | `triangular_numbers` | `31,359,240` | `triangular(7919)` | `TriangularNumbers.triangular_at_7919` |
| `TWIN_PRIME_LOWER_RESIDUES` | `twin_prime_residues` | `[11, 17, 29]` | `r ∈ U(30)` with `gcd((r+2) mod 30, 30) = 1` | `TwinPrimeResidues.twin_lower_has_coprime_successor` |
| `TWIN_PRIME_RESIDUE_PAIRS` | `twin_prime_residues` | `[(11,13),(17,19),(29,1)]` | derived from `TWIN_PRIME_LOWER_RESIDUES` | `TwinPrimeResidues.pairs_offset_by_two` |
| `U30_CARDINALITY` | `parameters` | `8` | `φ(MODULUS_U)` | `Parameters.u30Cardinality_eq_8` |
| `U90_CARDINALITY` | `u90` | `24` | enumerated `n ∈ [1, 90]` with `gcd(n, 30) = 1` | `U90.cardinality_u90` |
| `UNITS_Z9` | `units_z9` | `[1, 2, 4, 5, 7, 8]` | `gcd(n, 9) = 1` for `n ∈ [1, 9)` | `UnitsZ9.units_z9_canonical` |
| `UNITS_Z9_CARDINALITY` | `units_z9` | `6` | `\|UNITS_Z9\|` | `UnitsZ9.units_z9_cardinality` |
| `UNITS_Z9_SUM` | `units_z9` | `27` | `Σ UNITS_Z9` | `UnitsZ9.units_z9_sum_eq_27` |

Functions: `gcd`, `lcm`, `euler_phi`, `digital_root`, `fib`,
`dr_fib`, `triangular`, `nth_u30_member`, `index_of_u30_member`,
`pi_via_u30_sieve`, `replicant_count_distinct`,
`duplicate_factorization_count`, plus eight `is_*_candidate`
predicates.

---

## §2 — By value (the convergence map)

The substrate's elegance is that the same structurally meaningful
numbers arise from independent paths. Each row below is a witness
that the modules cohere.

| Number | Derivations |
|---:|---|
| `8` | `U30_CARDINALITY = φ(30)`; `RESIDUES.len()`; `\|U(30)\|` |
| `9` | `MODULUS_DR = NUMERAL_BASE − 1` |
| `12` | `LATTICE_LAYERS = MODULUS_R / MODULUS_U`; `\|R(360)\| / \|U(30)\| = 96/8` |
| `24` | `PISANO_PERIOD_MOD_DR`; `U90_CARDINALITY`; `FIBER_SIZE`; `LATTICE_LAYERS · 2`; `U30_CARDINALITY · 3` |
| `27` | `UNITS_Z9_SUM`; `MULT_TABLE_ROW_SUM = 3 · MODULUS_DR`; `Σ {1,2,4,5,7,8}` |
| `30` | `MODULUS_U` (parameter); `MODULUS_U90 / 3`; `MODULUS_R / LATTICE_LAYERS` |
| `36` | per-layer Fibo-DR sum (Phase 1 structural decomposition); `SUM_FIBO_DR / LATTICE_LAYERS` |
| `49 / 19 / 31 / 61 / 79 / 1` | `SQUARE_RESIDUE_VALUES` (`n² mod 90` over `U_30(90)`) |
| `89` | `ANCHOR_24 = nth_u30_member(24)`; `24th element of U_30(∞)` |
| `90` | `MODULUS_U90 = 3 · MODULUS_U`; `MODULUS_R / 4` |
| `96` | `R_CARDINALITY`; `U30_CARDINALITY · LATTICE_LAYERS`; `U90_CARDINALITY · 4`; `ELEMENTS.len()` |
| `108` | `DR_ROW_SUM = 4 · 27 = 4 · UNITS_Z9_SUM` |
| `109` | `ANCHOR_30 = nth_u30_member(30)`; `30th element of U_30(∞)` |
| `162` | `MULT_TABLE_TOTAL_SUM = 6 · 27` |
| `360` | `MODULUS_R = lcm(30, 9, 24)`; `4 · MODULUS_U90` |
| `432` | `SUM_DR`; `SUM_FIBO_DR`; `LATTICE_LAYERS · 36`; `4 · DR_ROW_SUM`; `DR_TABLE_TOTAL / 6`; `DIAGONAL_PAIR_SUM / 5` |
| `576` | `DYAD_COUNT = U90_CARDINALITY²`; `FIBER_SIZE · U90_CARDINALITY`; `CLASS_COUNT · CLASS_SIZE = 36 · 16` |
| `960` | `DIAGONAL_SUM = 4 · 240`; `4 · Σ SQUARE_RESIDUE_VALUES` |
| `1,080` | `ROW_SUM_LINEAR = 3 · MODULUS_R`; `Σ U_30(90)` |
| `1,200` | `SECONDARY_DIAGONAL_SUM = 4 · 300`; `4 · Σ SECONDARY_DIAGONAL_VALUES` |
| `2,160` | `DIAGONAL_PAIR_SUM = 6 · MODULUS_R = 5 · SUM_DR` |
| `2,592` | `DR_TABLE_TOTAL = 24 · DR_ROW_SUM = 6 · SUM_DR` |
| `7,919` | `nth_u30_member(2112)`; the 1000th prime |
| `7,920` | `89² − 1 = 22 · MODULUS_R` |
| `7,921` | `89² = 7919 + 2`; `F_10 · F_12 + 1` |
| `8,363,520` | `PYRAMID_PRODUCT = 2112 · 11 · MODULUS_R` |

---

## §3 — By phase (timeline)

Each phase added a typed layer to the substrate. Walking top to
bottom traces the discovery path. Every entry derives from the
phases above.

### Phase 1 — substrate (12 modules)

`parameters`, `arithmetic`, `digital_root`, `fibonacci`, `u30`,
`r360`, `appendix_a`, `dirichlet`, `atlas_correspondence`,
`ring_substrate`, `li`, `ground_truth`. Two parameters, eight
derived constants, the 96-row Appendix A matrix, two `432`
invariants, the Atlas-96 mirror.

### Phase 1.5 — `(Z/9Z)*` extension (2 modules)

`u90`, `units_z9`. Twenty-four-element substrate, period-24 dr
cycle, six-element multiplicative group of `Z/9Z`, the 6×6
multiplication table.

### Phase 2 — wheel-30 sieve (2 modules)

`sieve`, `pi_function`. `π(N)` exact at every tested decimal power
through 10¹⁰. The structural identity `π = D₃₀ − C₃₀ + 2` and the
additive form `π = primes_in_u30(N) + 3` proven equivalent.

### Phase 3 — residue constraints (4 modules)

`twin_prime_residues`, `sophie_germain_residues`,
`siamese_prime_pattern`, `perfect_squares_mod30`. Necessary
mod-30 constraints on twin primes (`{11,17,29}`), Sophie Germain
primes (`{11,23,29}`), Siamese generators (`{3,9,15,21,27} = 6k+3`),
and squares of `U(30)` members (`{1,19} mod 30`, dr `{1,4,7}`).

### Phase 4 — bijection / generating function / dyads (3 modules)

`index_to_residue`, `fibonacci_reciprocal`, `factorization_dyads`.
Closed-form bijection `k ↔ U_30(∞)[k]`, the `1/89 = Σ F_n / 10^(n+1)`
identity, and 576 dyads grouped into 24 fibers of 24.

### Phase 5 — invariants and amplification (3 modules)

`triangular_numbers`, `period24_dr_sequences`, `replicant_factorization`.
The 89 / 109 / 7919 anchor identities, the 24×24 dr table with row
sum 108 and total 2592, and the duplicate-factorization counter
that tabulates `RF(1000) = 5` and `RF(10000) = 408`.

### Phase 6 — dyad classes (1 module)

`dyad_classes`. The 576-dyad partition `36 = |(Z/9Z)*|²` classes ×
`16 = 4²` dyads each. Closes the final coverage gap from the
website audit.

---

## Searching this index

| Looking for… | Open §… |
|---|---|
| a constant by name | §1 |
| every place a number appears | §2 |
| the derivation chain or context | §3 |
| the convergence proofs | `tests/cross_phase_invariants.rs` |
| the visual map | `TOPOLOGY.md` |
| the Phase 1 strict-anchoring proof | `RESULTS.md` |
| the audit reports (cuts and renames) | `../jargon_audit.md`, `../strict_audit.md` |
