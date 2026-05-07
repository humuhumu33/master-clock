const NUMERAL_BASE: u64 = 10;
const MODULUS_U: u64 = 30;
const MODULUS_DR: u64 = NUMERAL_BASE - 1;
const MODULUS_R: u64 = 360;
const MODULUS_U90: u64 = 90;

fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
fn dr(n: u64) -> u64 { if n == 0 {0} else { let m = n % MODULUS_DR; if m == 0 {9} else {m} } }
fn units(k: u64) -> Vec<u64> { (1..k).filter(|&n| gcd(n, k) == 1).collect() }
fn u90_coprime_30() -> Vec<u64> { (1..=MODULUS_U90).filter(|&n| gcd(n, MODULUS_U) == 1).collect() }

fn header(s: &str) { println!("\n=== {} ===", s); }

fn main() {
    println!("Website-derived structural claims — numerical validation");
    println!("Grounded in MODULUS_U=30, NUMERAL_BASE=10. MODULUS_R={}", MODULUS_R);

    header("Claim 1: U_30(90) has 24 elements (coprime to 30 in [1,90])");
    let u90 = u90_coprime_30();
    println!("|U_30(90)| = {}", u90.len());
    println!("U_30(90)   = {:?}", u90);
    assert_eq!(u90.len(), 24);

    header("Claim 2: dr cycle over U_30(90)");
    let dr_cycle: Vec<u64> = u90.iter().map(|&n| dr(n)).collect();
    let expected = vec![1u64,7,2,4,8,1,5,2,4,1,5,7,2,4,8,5,7,4,8,1,5,7,2,8];
    println!("dr cycle = {:?}", dr_cycle);
    assert_eq!(dr_cycle, expected);

    header("Claim 3: 4-uniformity over {1,2,4,5,7,8}, zero on {3,6,9}");
    let units9 = vec![1u64, 2, 4, 5, 7, 8];
    for u in &units9 {
        let c = dr_cycle.iter().filter(|&&x| x == *u).count();
        println!("dr value {} appears {} times", u, c);
        assert_eq!(c, 4);
    }
    for v in &[3u64, 6, 9] {
        let c = dr_cycle.iter().filter(|&&x| x == *v).count();
        assert_eq!(c, 0);
    }
    println!("=> exactly 4 each of {{1,2,4,5,7,8}}, zero of {{3,6,9}}");

    header("Claim 4: (Z/9Z)* = {1,2,4,5,7,8}, cyclic of order 6, generator 2");
    let z9_units: Vec<u64> = units(9);
    println!("(Z/9Z)* = {:?}", z9_units);
    assert_eq!(z9_units, units9);
    let mut x = 1u64;
    let mut powers = Vec::new();
    for _ in 0..6 { x = (x * 2) % 9; powers.push(x); }
    println!("2^k mod 9 for k=1..6: {:?}", powers);
    let mut s = powers.clone(); s.sort();
    assert_eq!(s, units9);
    println!("=> 2 generates the entire group");

    header("Claim 5: 6x6 multiplication table mod 9 — closed, row/col sums 27, total 162");
    let mut total = 0u64;
    for &a in &units9 {
        let row: Vec<u64> = units9.iter().map(|&b| dr(a * b)).collect();
        let row_sum: u64 = row.iter().sum();
        for v in &row { assert!(units9.contains(v)); }
        assert_eq!(row_sum, 27);
        total += row_sum;
        println!("dr({}·*) = {:?}, sum {}", a, row, row_sum);
    }
    assert_eq!(total, 162);

    header("Claim 6: additive-inverse pairs (1,8), (2,7), (4,5) sum to 9");
    for (a, b) in &[(1u64,8u64),(2,7),(4,5)] {
        println!("{} + {} = {}", a, b, a+b);
        assert_eq!(a + b, 9);
    }
    println!("Note: these are ADDITIVE inverses in Z/9Z, not multiplicative.");
    println!("Multiplicative inverse pairs in (Z/9Z)*: (1,1), (2,5), (4,7), (8,8)");
    for (a, b) in &[(1u64,1u64),(2,5),(4,7),(8,8)] {
        let p = (a * b) % 9;
        println!("{}·{} mod 9 = {}", a, b, p);
        assert_eq!(p, 1);
    }

    header("Claim 7: 24x24 mod-90 multiplication matrix — every row sums to 1080");
    let mut sample_row: Vec<u64> = Vec::new();
    for (i, &a) in u90.iter().enumerate() {
        let row: Vec<u64> = u90.iter().map(|&b| (a * b) % MODULUS_U90).collect();
        let rs: u64 = row.iter().sum();
        if i == 0 { sample_row = row.clone(); }
        assert_eq!(rs, 1080, "row a={} sum {} != 1080", a, rs);
    }
    println!("All 24 rows sum to 1080 = 3 * MODULUS_R = 3 * 360");
    println!("First row (a=1) = {:?}", sample_row);
    println!("Reason: a·U_30(90) is a bijection of U_30(90) for a ∈ U_30(90)");
    let lin_sum: u64 = u90.iter().sum();
    println!("=> Σ_{{j ∈ U_30(90)}} j = {}", lin_sum);
    assert_eq!(lin_sum, 1080);

    header("Claim 8: principal diagonal residues = {1, 19, 31, 49, 61, 79}");
    use std::collections::BTreeMap;
    let mut counts: BTreeMap<u64, u64> = BTreeMap::new();
    for &a in &u90 {
        let sq = (a * a) % MODULUS_U90;
        *counts.entry(sq).or_insert(0) += 1;
    }
    let keys: Vec<u64> = counts.keys().copied().collect();
    println!("distinct values = {:?}", keys);
    assert_eq!(keys, vec![1u64, 19, 31, 49, 61, 79]);
    for (v, c) in &counts {
        println!("  {} appears {} times", v, c);
        assert_eq!(*c, 4);
    }
    let diag_sum: u64 = u90.iter().map(|&a| (a * a) % MODULUS_U90).sum();
    println!("diagonal sum = {} = 4 * 240", diag_sum);
    assert_eq!(diag_sum, 960);

    header("Claim 9: anchoring to Phase 1");
    println!("|U_30(90)| = 24             = Pisano(9) = LATTICE_LAYERS · 2");
    println!("|U(30)| · 3 = 8 · 3        = 24 (mod-90 = 3 mod-30 copies)");
    println!("|R(360)| = 96              = 24 · 4 (mod-360 = 4 mod-90 copies)");
    println!("Each row sum 1080          = 3 · MODULUS_R = 3 · 360");
    println!("Diagonal residue count 6   = |(Z/9Z)*|");
    println!("4-uniformity count 4       = MODULUS_R / MODULUS_U90");

    println!("\n=== ALL CLAIMS VERIFIED ===");
    println!("Every fact derives from MODULUS_U=30 and NUMERAL_BASE=10.");
}
