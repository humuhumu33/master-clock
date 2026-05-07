// Combined rigor verifier for the Phase 4 batch:
//   index_to_residue.rs       — bijection k ↔ n-th U_30(∞) member
//   fibonacci_reciprocal.rs   — 1/89 = Σ F_n / 10^(n+1) generating-function identity
//   factorization_dyads.rs    — 576 = 24² ordered dyads, 24 fibers of 24
//
// Re-implements every const-fn, runs cross-checks, confirms zero defects.
//
// Compile: rustc -O verify_phase4_batch.rs -o verify_phase4_batch

const MODULUS_U: u64 = 30;
const MODULUS_U90: u64 = 90;
const NUMERAL_BASE: u64 = 10;
const U30_CARDINALITY: usize = 8;
const RESIDUES: [u64; 8] = [1, 7, 11, 13, 17, 19, 23, 29];

fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }

fn fib(k: u64) -> u64 {
    if k == 0 { return 0; }
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    let mut i: u64 = 1;
    while i < k {
        let next = a + b;
        a = b;
        b = next;
        i += 1;
    }
    b
}

fn u90_residues() -> Vec<u64> {
    (1..=MODULUS_U90).filter(|&n| gcd(n, MODULUS_U) == 1).collect()
}

fn header(s: &str) { println!("\n=== {} ===", s); }
fn pass(s: &str) { println!("  [OK] {}", s); }
fn check(name: &str, c: bool) {
    if c { pass(name); } else { panic!("[FAIL] {}", name); }
}

// ── Phase 4.1 — index_to_residue ──────────────────────────────────

fn nth_u30_member(k: u64) -> u64 {
    if k == 0 { return 0; }
    let zero_idx = k - 1;
    let block = zero_idx / U30_CARDINALITY as u64;
    let offset = zero_idx % U30_CARDINALITY as u64;
    block * MODULUS_U + RESIDUES[offset as usize]
}

fn index_of_u30_member(n: u64) -> u64 {
    if n == 0 || gcd(n, MODULUS_U) != 1 { return 0; }
    let block = n / MODULUS_U;
    let offset = n % MODULUS_U;
    let pos = RESIDUES.iter().position(|&r| r == offset).unwrap();
    block * U30_CARDINALITY as u64 + pos as u64 + 1
}

// ── Phase 4.2 — fibonacci_reciprocal ──────────────────────────────

fn pow10_u128(k: u32) -> u128 {
    let mut p: u128 = 1;
    for _ in 0..k { p *= NUMERAL_BASE as u128; }
    p
}

fn partial_numerator(n: u32) -> u128 {
    let mut sum: u128 = 0;
    for k in 1..=n {
        let exp = (n - k) as u32;
        let f_k = fib(k as u64) as u128;
        sum += f_k * pow10_u128(exp);
    }
    sum
}

fn tail_gap(n: u32) -> u128 {
    let denom = pow10_u128(n + 1);
    let s = partial_numerator(n);
    let scaled = 89u128 * s;
    if denom > scaled { denom - scaled } else { 0 }
}

fn tail_bound(n: u32) -> u128 {
    178u128 * (fib((n + 1) as u64) as u128)
}

// ── Phase 4.3 — factorization_dyads ───────────────────────────────

fn compute_dyads() -> Vec<(u64, u64, u64)> {
    let u90 = u90_residues();
    let mut out = Vec::with_capacity(576);
    for &a in &u90 {
        for &b in &u90 {
            out.push((a, b, (a * b) % MODULUS_U90));
        }
    }
    out
}

fn main() {
    println!("Phase 4 batch verifier");
    println!("Two parameters in (MODULUS_U=30, NUMERAL_BASE=10); every claim out.\n");

    // ─── 4.1 ─────────────────────────────────────────────────────
    header("4.1: bijection k ↔ n-th U_30(∞) member");
    let anchors: &[(u64, u64)] = &[
        (1, 1), (2, 7), (3, 11), (8, 29), (9, 31), (24, 89), (2112, 7919),
    ];
    for &(k, n) in anchors {
        check(&format!("nth_u30_member({}) == {}", k, n), nth_u30_member(k) == n);
        check(&format!("index_of_u30_member({}) == {}", n, k), index_of_u30_member(n) == k);
    }

    let mut count = 0u64;
    for n in 1u64..=10_000 {
        if gcd(n, MODULUS_U) == 1 {
            count += 1;
            assert_eq!(index_of_u30_member(n), count, "inverse map drift at n={}", n);
            assert_eq!(nth_u30_member(count), n, "forward map drift at k={}", count);
        }
    }
    println!("  [OK] round-trip on all {} U_30(∞) members up to 10,000", count);

    // ─── 4.2 ─────────────────────────────────────────────────────
    header("4.2: 1/89 = Σ F_n / 10^(n+1) — generating-function identity");
    check("100 - 10 - 1 == 89", NUMERAL_BASE * NUMERAL_BASE - NUMERAL_BASE - 1 == 89);
    check("partial_numerator(10) == 1,123,595,495", partial_numerator(10) == 1_123_595_495);
    println!("  [info] partial_numerator(10) = {}", partial_numerator(10));
    println!("  [info] true 1/89 × 10^11 ≈ 1,123,595,505.617");

    println!("  [info] N | 89·S(N) ≤ 10^(N+1) | gap | bound");
    for n in 5..=17u32 {
        let denom = pow10_u128(n + 1);
        let gap = tail_gap(n);
        let bound = tail_bound(n);
        let scaled = 89u128 * partial_numerator(n);
        println!("       {:>2} | {:>40} | {:>15} | {:>15}", n, scaled, gap, bound);
        check(&format!("N={}: 89·S(N) < 10^(N+1)", n), scaled < denom);
        check(&format!("N={}: gap > 0", n), gap > 0);
        check(&format!("N={}: gap < tail_bound", n), gap < bound);
    }
    // Convergence (integer-only): gap(N+1) < 10 * gap(N).
    for n in 5..17u32 {
        let g0 = tail_gap(n);
        let g1 = tail_gap(n + 1);
        check(&format!("gap shrinks 10×: gap({}) < 10·gap({})", n + 1, n), g1 < 10 * g0);
    }

    // ─── 4.3 ─────────────────────────────────────────────────────
    header("4.3: 576 = 24² ordered dyads in U_30(90) × U_30(90)");
    let dyads = compute_dyads();
    check("dyad count == 576", dyads.len() == 576);

    let u90 = u90_residues();
    for &(_, _, p) in &dyads {
        assert!(u90.contains(&p), "product {} escapes U_30(90)", p);
    }
    pass("every product remains in U_30(90)");

    let mut counts = std::collections::BTreeMap::<u64, usize>::new();
    for &(_, _, p) in &dyads {
        *counts.entry(p).or_insert(0) += 1;
    }
    check("24 distinct product residues", counts.len() == 24);
    for (&r, &c) in &counts {
        assert_eq!(c, 24, "fiber over {} has size {}, expected 24", r, c);
    }
    pass("every fiber has size 24");

    // Diagonal: a · a mod 90 over a ∈ U_30(90)
    let diag: Vec<u64> = u90.iter().map(|&a| (a * a) % MODULUS_U90).collect();
    let diag_distinct: std::collections::BTreeSet<u64> = diag.iter().copied().collect();
    check("diagonal has 6 distinct values", diag_distinct.len() == 6);
    let expected_diag: std::collections::BTreeSet<u64> = [1u64, 19, 31, 49, 61, 79].iter().copied().collect();
    check("diagonal matches square residue values", diag_distinct == expected_diag);

    println!("\n=== ALL THREE PHASE 4 MODULES VERIFIED ===");
    println!("  4.1: bijection round-trip on {} members", count);
    println!("  4.2: 1/89 identity holds at N ∈ [5, 17] within Fibonacci tail bound");
    println!("  4.3: 576 dyads, 24 fibers of 24, group closure under mod-90 multiplication");
}
