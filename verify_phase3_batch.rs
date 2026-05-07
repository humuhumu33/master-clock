// Combined rigor verifier for the Phase 3 batch:
//   sophie_germain_residues.rs
//   siamese_prime_pattern.rs
//   perfect_squares_mod30.rs
//   u90.rs (secondary diagonal extension)
//
// Re-implements every const-fn, runs an empirical sweep against an
// independent primality oracle, and confirms zero residue violations.
//
// Compile: rustc -O verify_phase3_batch.rs -o verify_phase3_batch
// Run:     ./verify_phase3_batch

const MODULUS_U: u64 = 30;
const MODULUS_U90: u64 = 90;
const RESIDUES: [u64; 8] = [1, 7, 11, 13, 17, 19, 23, 29];

fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }

fn isqrt(n: u64) -> u64 {
    if n < 2 { return n; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x { x = y; y = (x + n / x) / 2; }
    x
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n < 4 { return true; }
    if n % 2 == 0 { return false; }
    let r = isqrt(n);
    let mut k = 3u64;
    while k <= r { if n % k == 0 { return false; } k += 2; }
    true
}

fn dr(n: u64) -> u64 {
    if n == 0 { 0 } else { let m = n % 9; if m == 0 { 9 } else { m } }
}

fn pass(s: &str) { println!("  [OK] {}", s); }
fn check(name: &str, c: bool) {
    if c { pass(name); } else { panic!("[FAIL] {}", name); }
}
fn header(s: &str) { println!("\n=== {} ===", s); }

fn u90_residues() -> Vec<u64> {
    (1..=MODULUS_U90).filter(|&n| gcd(n, MODULUS_U) == 1).collect()
}

fn main() {
    println!("Phase 3 batch verifier — Sophie Germain / Siamese / squares / u90 secondary");
    println!("Two parameters in (MODULUS_U=30, gcd); every claim out.\n");

    // ─── Sophie Germain ──────────────────────────────────────────────
    header("SG-1: derivation of SG lower residues");
    let mut sg_lower: Vec<u64> = Vec::new();
    let mut sg_iso: Vec<u64> = Vec::new();
    for &r in &RESIDUES {
        let s = (2 * r + 1) % MODULUS_U;
        if gcd(s, MODULUS_U) == 1 { sg_lower.push(r); }
        else { sg_iso.push(r); }
    }
    println!("sg_lower    = {:?}", sg_lower);
    println!("sg_isolated = {:?}", sg_iso);
    check("sg_lower == [11, 23, 29]", sg_lower == vec![11, 23, 29]);
    check("sg_isolated == [1, 7, 13, 17, 19]", sg_iso == vec![1, 7, 13, 17, 19]);
    check("partition covers U(30)", sg_lower.len() + sg_iso.len() == 8);

    header("SG-2: empirical sweep — every SG prime in [7, 10000]");
    let mut sg_count = 0u64;
    let mut sg_violations = 0u64;
    let mut sg_dist: [u64; 30] = [0; 30];
    let mut p = 7u64;
    while p <= 10_000 {
        if is_prime(p) && is_prime(2 * p + 1) {
            sg_count += 1;
            sg_dist[(p % MODULUS_U) as usize] += 1;
            if !matches!(p % MODULUS_U, 11 | 23 | 29) { sg_violations += 1; }
        }
        p += 2;
    }
    println!("SG primes in [7, 10000]: {}", sg_count);
    print!("distribution: ");
    for r in 0..30 { if sg_dist[r] > 0 { print!("[r={}: {}] ", r, sg_dist[r]); } }
    println!();
    check("zero SG violations", sg_violations == 0);

    // ─── Siamese ─────────────────────────────────────────────────────
    header("SI-1: derivation of Siamese generator residues mod 30");
    let mut sia: Vec<u64> = Vec::new();
    for n in 0u64..MODULUS_U {
        let nsq = (n * n) % MODULUS_U;
        let m = if nsq >= 2 { nsq - 2 } else { nsq + MODULUS_U - 2 };
        let p = (nsq + 2) % MODULUS_U;
        if gcd(m, MODULUS_U) == 1 && gcd(p, MODULUS_U) == 1 {
            sia.push(n);
        }
    }
    println!("siamese_residues = {:?}", sia);
    check("siamese_residues == [3, 9, 15, 21, 27]", sia == vec![3, 9, 15, 21, 27]);
    let as_6k_plus_3: Vec<u64> = (0..5u64).map(|k| 6 * k + 3).collect();
    check("matches 6k+3 form", sia == as_6k_plus_3);

    header("SI-2: empirical sweep — every Siamese pair in [3, 1000]");
    let mut si_count = 0u64;
    let mut si_violations = 0u64;
    for n in 3u64..=1000 {
        if n * n < 2 { continue; }
        let lo = n * n - 2;
        let hi = n * n + 2;
        if is_prime(lo) && is_prime(hi) {
            si_count += 1;
            if !matches!(n % MODULUS_U, 3 | 9 | 15 | 21 | 27) { si_violations += 1; }
        }
    }
    println!("Siamese pairs in [3, 1000]: {}", si_count);
    check("zero Siamese violations", si_violations == 0);

    // ─── Perfect squares mod 30 ──────────────────────────────────────
    header("PS-1: derivation of square residues mod 30");
    let mut sq_mod30: Vec<u64> = Vec::new();
    for &n in &RESIDUES {
        let sq = (n * n) % MODULUS_U;
        if !sq_mod30.contains(&sq) { sq_mod30.push(sq); }
    }
    sq_mod30.sort();
    println!("square mod 30 residues = {:?}", sq_mod30);
    check("== [1, 19]", sq_mod30 == vec![1, 19]);

    header("PS-2: derivation of square digital roots");
    let mut sq_dr: Vec<u64> = Vec::new();
    for &n in &RESIDUES {
        let d = dr(n * n);
        if !sq_dr.contains(&d) { sq_dr.push(d); }
    }
    sq_dr.sort();
    println!("square dr values = {:?}", sq_dr);
    check("== [1, 4, 7]", sq_dr == vec![1, 4, 7]);

    header("PS-3: each square residue hit exactly 4 times");
    let mut hits_1 = 0u64;
    let mut hits_19 = 0u64;
    for &n in &RESIDUES {
        let sq = (n * n) % MODULUS_U;
        if sq == 1 { hits_1 += 1; } else if sq == 19 { hits_19 += 1; }
    }
    check("residue 1 hit 4 times", hits_1 == 4);
    check("residue 19 hit 4 times", hits_19 == 4);

    header("PS-4: empirical sweep — every prime > 5 has square in {1, 19} mod 30");
    let mut ps_violations = 0u64;
    let mut prime_count = 0u64;
    let mut p = 7u64;
    while p <= 10_000 {
        if is_prime(p) {
            prime_count += 1;
            let sq_mod = (p * p) % MODULUS_U;
            if sq_mod != 1 && sq_mod != 19 { ps_violations += 1; }
            let sq_dr = dr(p * p);
            if !matches!(sq_dr, 1 | 4 | 7) { ps_violations += 1; }
        }
        p += 2;
    }
    println!("primes tested in [7, 10000]: {}", prime_count);
    check("zero square-residue violations", ps_violations == 0);

    // ─── U(90) secondary diagonal ────────────────────────────────────
    header("UD-1: secondary diagonal residues from U_30(90)");
    let u90 = u90_residues();
    let mut sec_diag: Vec<u64> = Vec::new();
    let mut sec_sum = 0u64;
    for &a in &u90 {
        let sq = (a * a) % MODULUS_U90;
        let entry = if sq == 0 { 0 } else { MODULUS_U90 - sq };
        sec_sum += entry;
        if !sec_diag.contains(&entry) { sec_diag.push(entry); }
    }
    sec_diag.sort();
    println!("secondary diagonal distinct values = {:?}", sec_diag);
    check("== [11, 29, 41, 59, 71, 89]",
        sec_diag == vec![11, 29, 41, 59, 71, 89]);
    println!("secondary diagonal sum = {}", sec_sum);
    check("sum == 1200", sec_sum == 1200);

    header("UD-2: every secondary diagonal value is prime");
    for &v in &sec_diag {
        check(&format!("{} is prime", v), is_prime(v));
    }

    header("UD-3: combined diagonal sum (primary + secondary) = 6·MODULUS_R");
    let primary_sum: u64 = u90.iter().map(|&a| (a * a) % MODULUS_U90).sum();
    let combined = primary_sum + sec_sum;
    println!("primary={}, secondary={}, combined={}", primary_sum, sec_sum, combined);
    check("primary + secondary == 2160", combined == 2160);
    check("== 6 · 360 = 6 · MODULUS_R", combined == 6 * 360);

    // ─── Final ───────────────────────────────────────────────────────
    println!("\n=== ALL FOUR BATCH MODULES VERIFIED ===");
    println!("  Sophie Germain: {} primes, 0 violations", sg_count);
    println!("  Siamese pairs:  {} pairs,  0 violations", si_count);
    println!("  Squares mod 30: {} primes tested, 0 violations", prime_count);
    println!("  U(90) sec diag: 6 distinct primes, sum 1200");
}
