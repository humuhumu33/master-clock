# Prime Clock — topology of the substrate

Three layers (atoms / substrate / invariants) and three convergence
witnesses (`432`, `96`, `24`) — every node below derives from the
two atoms via standard number theory.

```mermaid
flowchart TD
    classDef atom        fill:#0a3,stroke:#fff,color:#fff,stroke-width:2px;
    classDef substrate   fill:#048,stroke:#fff,color:#fff;
    classDef invariant   fill:#963,stroke:#fff,color:#fff;
    classDef convergent  fill:#a30,stroke:#fff,color:#fff,stroke-width:3px;

    %% Atoms — irreducible parameters
    P1[MODULUS_U = 30]:::atom
    P2[NUMERAL_BASE = 10]:::atom

    %% Atomic helpers
    GCD[gcd / lcm / euler_phi]:::atom
    DR_FN[digital_root]:::atom
    FIB_FN[fib / fib_mod_dr / dr_fib]:::atom

    P1 --> GCD
    P2 --> DR_FN
    GCD --> FIB_FN
    DR_FN --> FIB_FN

    %% Derived parameters
    MD[MODULUS_DR = 9]:::invariant
    PIS[PISANO_PERIOD_MOD_DR = 24]:::invariant
    UC[U30_CARDINALITY = 8]:::invariant
    MR[MODULUS_R = 360]:::invariant
    LL[LATTICE_LAYERS = 12]:::invariant
    RC[R_CARDINALITY = 96]:::convergent

    P2 --> MD
    FIB_FN --> PIS
    P1 --> UC
    GCD --> UC
    P1 --> MR
    MD --> MR
    PIS --> MR
    MR --> LL
    P1 --> LL
    UC --> RC
    LL --> RC

    %% Substrate — sets and tables
    U30[u30::RESIDUES<br/>{1,7,11,13,17,19,23,29}]:::substrate
    U90[u90::RESIDUES_U90<br/>24 elements]:::substrate
    R360[r360::ELEMENTS<br/>96 elements]:::substrate
    UZ9[units_z9::UNITS_Z9<br/>{1,2,4,5,7,8}]:::substrate
    APX[appendix_a::ROWS<br/>96-row matrix]:::substrate
    DRC[u90::DR_CYCLE<br/>period-24]:::convergent
    DYADS[factorization_dyads::DYADS_TABLE<br/>576 entries]:::substrate
    DCLASS[dyad_classes::CLASS_COUNTS<br/>36 × 16 partition]:::invariant
    DRSEQ[period24_dr_sequences::DR_SEQUENCES<br/>24×24]:::substrate
    MULT[units_z9::MULT_TABLE_Z9<br/>6×6 Latin square]:::substrate

    P1 --> U30
    GCD --> U30
    P1 --> U90
    GCD --> U90
    MR --> R360
    GCD --> R360
    MD --> UZ9
    GCD --> UZ9
    R360 --> APX
    DR_FN --> APX
    FIB_FN --> APX
    U90 --> DRC
    DR_FN --> DRC
    U90 --> DYADS
    U90 --> DRSEQ
    DR_FN --> DRSEQ
    UZ9 --> MULT

    %% Invariants — the convergent numbers
    SUM432[SUM_DR = 432]:::convergent
    SFIB[SUM_FIBO_DR = 432]:::convergent
    DRROW[DR_ROW_SUM = 108]:::invariant
    DRTOT[DR_TABLE_TOTAL = 2592 = 6·432]:::invariant
    DPS[DIAGONAL_PAIR_SUM = 2160 = 6·MODULUS_R]:::invariant
    SQS[SQUARE_RESIDUE_VALUES<br/>{1,19,31,49,61,79}]:::invariant
    SDS[SECONDARY_DIAGONAL_VALUES<br/>{11,29,41,59,71,89}]:::invariant
    DCC[DYAD_COUNT = 576]:::invariant
    FIBSZ[FIBER_SIZE = 24]:::invariant
    UZSUM[UNITS_Z9_SUM = 27]:::invariant
    MTROW[MULT_TABLE_ROW_SUM = 27]:::invariant

    APX --> SUM432
    APX --> SFIB
    LL --> SUM432
    LL --> SFIB
    DRSEQ --> DRROW
    DRROW --> DRTOT
    SUM432 --> DRTOT
    R360 --> DPS
    MR --> DPS
    SUM432 --> DPS
    U90 --> SQS
    U90 --> SDS
    U90 --> DCC
    DYADS --> FIBSZ
    UZ9 --> UZSUM
    UZSUM --> MTROW
    MULT --> MTROW
    DYADS --> DCLASS
    UZ9 --> DCLASS
    DRC --> DCLASS

    %% Phase 3 predicates and Phase 5 anchors
    TWIN[twin_prime_residues<br/>{11,17,29}]:::invariant
    SG[sophie_germain_residues<br/>{11,23,29}]:::invariant
    SIAM[siamese_prime_pattern<br/>{3,9,15,21,27} = 6k+3]:::invariant
    PSQ[perfect_squares_mod30<br/>n² mod 30 ∈ {1,19}]:::invariant
    NTH[index_to_residue::nth_u30_member]:::substrate
    A24[ANCHOR_24 = 89]:::invariant
    A30[ANCHOR_30 = 109]:::invariant
    PYR[PYRAMID_PRODUCT = 8,363,520<br/>= 2112·11·MODULUS_R]:::invariant
    F49N[F49 = 7,778,742,049]:::invariant
    SIEVE[sieve::pi_via_u30_sieve]:::substrate
    PIN[pi_function::π(N)]:::invariant
    FIB89[fibonacci_reciprocal<br/>1/89 = Σ F_n/10^(n+1)]:::invariant
    REPS[replicant_factorization<br/>RF(1000)=5, RF(10000)=408]:::invariant

    U30 --> TWIN
    U30 --> SG
    U30 --> SIAM
    U30 --> PSQ
    DR_FN --> PSQ
    P1 --> NTH
    U30 --> NTH
    NTH --> A24
    NTH --> A30
    A24 --> PYR
    A30 --> PYR
    MR --> PYR
    FIB_FN --> F49N
    U30 --> SIEVE
    SIEVE --> PIN
    FIB_FN --> FIB89
    P2 --> FIB89
    U30 --> REPS
    GCD --> REPS
```

## How to read this

- **Green nodes** are the two irreducible parameters (`atoms`).
- **Blue nodes** are derived structural objects (`substrate`).
- **Brown nodes** are invariants and predicates (`invariants`).
- **Red-bordered nodes** are *convergent numbers* — they admit
  three or more independent derivations through the substrate.
  See `tests/cross_phase_invariants.rs` for the machine-checked
  proofs of convergence.

Three convergent numbers anchor the topology:

```
432   = SUM_DR = SUM_FIBO_DR = LATTICE_LAYERS · 36
       = 4 · DR_ROW_SUM = DR_TABLE_TOTAL / 6 = DIAGONAL_PAIR_SUM / 5

96    = R_CARDINALITY = U30_CARDINALITY · LATTICE_LAYERS
       = U90_CARDINALITY · 4 = |R360_ELEMENTS|

24    = PISANO_PERIOD_MOD_DR = U90_CARDINALITY = FIBER_SIZE
       = LATTICE_LAYERS · 2 = U30_CARDINALITY · 3
```

## Triality of the substrate, summarized

The graph above exhibits triality at three nesting levels:

1. **The two atoms generate three orbits.** `MODULUS_U = 30 = 2·3·5`
   — three primes generate the residue substrate. `(Z/9Z)*` partitions
   into three cosets `{1,4,7} ∪ {2,5,8} ∪ {3,6,9}` mod 3.

2. **The substrate has three structural layers.** Eight-element
   `U(30)` (Phase 1 atom-level) → 24-element `U_30(90)`
   (Phase 1.5 substrate-level) → 96-element `R(360)` (Phase 1
   lattice-level). Each layer triples the previous: `8 · 3 = 24`,
   `24 · 4 = 96`. The closed form is `R_CARDINALITY = 8 · 12 =
   24 · 4`.

3. **Three Phase 3 prime constraints.** `twin / Sophie Germain /
   Siamese` — three independent residue filters, each a partition
   of `U(30)` into the residues that survive their respective
   structural test and the residues that are isolated. The three
   filters together cover the website's three categorical claims
   about prime structure mod 30.

## How to navigate from here

| Want to… | Open… |
|---|---|
| look up a single constant | `INDEX.md` §1 |
| find every place a number appears | `INDEX.md` §2 |
| see the derivation walk in chronological order | `INDEX.md` §3 |
| read the strict Phase 1 anchoring evidence | `RESULTS.md` |
| see the convergence proofs run | `tests/cross_phase_invariants.rs` |
| reproduce all numerical claims standalone | `validate_*.rs`, `verify_*_batch.rs` |
| check the Lean theorems | `lean4/PrimeFoundations/*.lean` |
| import the canonical public API | `use toroidal_prime_clock::prelude::*` |
