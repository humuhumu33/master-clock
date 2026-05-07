use std::collections::BTreeMap;

const MODULUS_U: u64 = 30;
const MODULUS_U90: u64 = 90;
const MODULUS_R: u64 = 360;
const RESIDUES: [u64; 8] = [1, 7, 11, 13, 17, 19, 23, 29];

fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
fn dr(n: u64) -> u64 { if n == 0 { 0 } else { let m = n % 9; if m == 0 { 9 } else { m } } }
fn u90_residues() -> Vec<u64> { (1..=MODULUS_U90).filter(|&n| gcd(n, MODULUS_U) == 1).collect() }
fn nth_u30_member(k: u64) -> u64 {
    if k == 0 { return 0; }
    let zero_idx = k - 1;
    let block = zero_idx / 8;
    let offset = zero_idx % 8;
    block * MODULUS_U + RESIDUES[offset as usize]
}
fn triangular(n: u64) -> u64 { n * (n + 1) / 2 }

fn pass(s: &str) { println!("  [OK] {}", s); }
fn check(name: &str, c: bool) { if c { pass(name); } else { panic!("[FAIL] {}", name); } }
fn header(s: &str) { println!("\n=== {} ===", s); }

fn factorization_table(n: u64) -> BTreeMap<u64, u64> {
    let mut counts: BTreeMap<u64, u64> = BTreeMap::new();
    if n < 49 { return counts; }
    let mut a = 7u64;
    while a * a <= n {
        if gcd(a, MODULUS_U) == 1 {
            let mut b = a;
            while a * b <= n {
                if gcd(b, MODULUS_U) == 1 { *counts.entry(a * b).or_insert(0) += 1; }
                b += 2;
            }
        }
        a += 2;
    }
    counts
}

fn replicant_products(n: u64) -> Vec<u64> {
    let mut out: Vec<u64> = factorization_table(n).into_iter()
        .filter(|&(_, c)| c >= 2).map(|(p, _)| p).collect();
    out.sort();
    out
}

fn duplicate_factorization_count(n: u64) -> u64 {
    factorization_table(n).values().filter(|&&c| c >= 2).map(|&c| c - 1).sum()
}

fn main() {
    println!("Phase 5 batch verifier — triangular / period-24 dr / replicants\n");

    header("5.1: triangular k(n) = n(n+1)/2 anchors");
    let a24 = nth_u30_member(24);
    let a30 = nth_u30_member(30);
    check("nth_u30_member(24) == 89", a24 == 89);
    check("nth_u30_member(30) == 109", a30 == 109);
    check("k(89) == 4,005", triangular(a24) == 4_005);
    check("k(109) == 5,995", triangular(a30) == 5_995);
    check("k(7919) == 31,359,240", triangular(7_919) == 31_359_240);
    check("89² − 1 == 22 · MODULUS_R", a24 * a24 - 1 == 22 * MODULUS_R);
    check("109² − 89² == 11 · MODULUS_R", a30 * a30 - a24 * a24 == 11 * MODULUS_R);
    check("pyramid product 2112·(109²−89²) == 8,363,520",
        2_112u64 * (a30 * a30 - a24 * a24) == 8_363_520);

    header("5.2: 24×24 DR sequences over U_30(90)");
    let u90 = u90_residues();
    let mut seq = vec![vec![0u64; 24]; 24];
    let mut total = 0u64;
    for (i, &r) in u90.iter().enumerate() {
        let mut row_sum = 0u64;
        for (k, &a) in u90.iter().enumerate() {
            seq[i][k] = dr((r * a) % MODULUS_U90);
            row_sum += seq[i][k];
            total += seq[i][k];
        }
        if row_sum != 108 { panic!("[FAIL] row {} sum {} != 108", i, row_sum); }
    }
    pass("every row sums to 108");
    check("table total == 2592 == 6·432", total == 2592 && total == 6 * 432);
    for k in 0..24 {
        let col_sum: u64 = (0..24).map(|i| seq[i][k]).sum();
        if col_sum != 108 { panic!("[FAIL] col {} sum {} != 108", k, col_sum); }
    }
    pass("every column sums to 108");
    let units_z9: std::collections::BTreeSet<u64> = [1u64, 2, 4, 5, 7, 8].iter().copied().collect();
    for row in &seq { for &v in row { assert!(units_z9.contains(&v)); } }
    pass("every entry in (Z/9Z)*");
    let dr_cycle: Vec<u64> = u90.iter().map(|&n| dr(n)).collect();
    check("first row matches dr cycle", seq[0] == dr_cycle);
    let mut canonical = dr_cycle.clone(); canonical.sort();
    for row in &seq { let mut s = row.clone(); s.sort(); assert_eq!(s, canonical); }
    pass("every row is a permutation of the dr cycle");

    header("5.3: replicant factorization counts");
    check("RF distinct(100) == 0", replicant_products(100).is_empty());
    let r1000 = replicant_products(1_000);
    println!("  replicants in [1, 1000]: {:?}", r1000);
    check("RF distinct(1000) == 5", r1000.len() == 5);
    check("RF distinct(1000) products == [539, 637, 833, 847, 931]",
        r1000 == vec![539, 637, 833, 847, 931]);

    let dup_1000 = duplicate_factorization_count(1_000);
    let dup_10000 = duplicate_factorization_count(10_000);
    println!("  duplicate factorizations: RF(1000)={}, RF(10000)={}", dup_1000, dup_10000);
    check("duplicate_factorization_count(1000) == 5", dup_1000 == 5);
    check("duplicate_factorization_count(10000) == 408", dup_10000 == 408);

    let dist_10000 = replicant_products(10_000).len();
    println!("  distinct replicant integers at N=10000: {}", dist_10000);
    check("distinct count diverges from duplicate count at 10000",
        dist_10000 == 248 && dup_10000 == 408);

    let cases: &[(u64, u64, u64, u64, u64)] = &[
        (539, 7, 77, 11, 49), (637, 7, 91, 13, 49), (833, 7, 119, 17, 49),
        (847, 7, 121, 11, 77), (931, 7, 133, 19, 49),
    ];
    for &(p, a1, b1, a2, b2) in cases {
        check(&format!("{} = {}·{} = {}·{}", p, a1, b1, a2, b2),
            a1 * b1 == p && a2 * b2 == p);
    }

    println!("\n=== ALL THREE PHASE 5 MODULES VERIFIED ===");
    println!("  5.1: triangular anchors {{89, 109, 7919}} + algebraic identities to MODULUS_R");
    println!("  5.2: 24×24 DR table — rows/cols 108, total 2592, permutations of dr cycle");
    println!("  5.3: distinct(1000)=5; duplicate(1000)=5, duplicate(10000)=408 (matches website)");
}
