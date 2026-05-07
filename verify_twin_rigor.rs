// Extended rigor verification — re-implements the actual src/twin_prime_residues.rs
// const-fn logic verbatim, then exercises every claim and every theorem the module
// states. Cross-checks against an independent reference implementation (sieve-based
// primality + brute residue enumeration). Sweeps up to N=100,000.
//
// The point of this file is to catch any defect that the in-crate `const _: () =
// assert!` blocks would miss: off-by-ones, hidden coupling, predicate inconsistency,
// edge cases at the boundaries, and large-scale empirical violations.
//
// Compile: rustc -O verify_twin_rigor.rs -o verify_twin_rigor
// Run:     ./verify_twin_rigor

const MODULUS_U: u64 = 30;
const U30_CARDINALITY: usize = 8;
const RESIDUES: [u64; 8] = [1, 7, 11, 13, 17, 19, 23, 29];

fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }

// ── Verbatim mirrors of the production const-fn logic ──────────────────────

const fn compute_twin_prime_lower() -> [u64; 3] {
    let mut out = [0u64; 3];
    let mut idx = 0usize;
    let mut i = 0usize;
    while i < U30_CARDINALITY {
        let r = RESIDUES[i];
        let s = (r + 2) % MODULUS_U;
        if gcd_const(s, MODULUS_U) == 1 {
            out[idx] = r;
            idx += 1;
        }
        i += 1;
    }
    out
}

const fn compute_isolated_lower() -> [u64; 5] {
    let mut out = [0u64; 5];
    let mut idx = 0usize;
    let mut i = 0usize;
    while i < U30_CARDINALITY {
        let r = RESIDUES[i];
        let s = (r + 2) % MODULUS_U;
        if gcd_const(s, MODULUS_U) > 1 {
            out[idx] = r;
            idx += 1;
        }
        i += 1;
    }
    out
}

const fn gcd_const(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd_const(b, a % b) } }

const TWIN_PRIME_LOWER_RESIDUES: [u64; 3] = compute_twin_prime_lower();
const ISOLATED_LOWER_RESIDUES: [u64; 5] = compute_isolated_lower();

const fn is_twin_prime_candidate_lower(n: u64) -> bool {
    let r = n % MODULUS_U;
    let mut i = 0usize;
    while i < 3 {
        if TWIN_PRIME_LOWER_RESIDUES[i] == r {
            return true;
        }
        i += 1;
    }
    false
}

// ── Independent reference oracle ───────────────────────────────────────────

fn isqrt(n: u64) -> u64 {
    if n < 2 { return n; }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x { x = y; y = (x + n / x) / 2; }
    x
}

fn is_prime_oracle(n: u64) -> bool {
    if n < 2 { return false; }
    if n < 4 { return true; }
    if n % 2 == 0 { return false; }
    let r = isqrt(n);
    let mut k = 3u64;
    while k <= r { if n % k == 0 { return false; } k += 2; }
    true
}

fn pass(s: &str) { println!("  [OK] {}", s); }
fn check(name: &str, condition: bool) {
    if condition { pass(name); }
    else { panic!("[FAIL] {}", name); }
}
fn header(s: &str) { println!("\n=== {} ===", s); }

// ── Rigor checks ──────────────────────────────────────────────────────────

fn main() {
    println!("Extended rigor verification of src/twin_prime_residues.rs");
    println!("Two parameters in (MODULUS_U=30, gcd); every claim out.\n");

    header("R1: production const-fns produce the canonical arrays");
    check("TWIN_PRIME_LOWER_RESIDUES == [11, 17, 29]",
        TWIN_PRIME_LOWER_RESIDUES == [11, 17, 29]);
    check("ISOLATED_LOWER_RESIDUES == [1, 7, 13, 19, 23]",
        ISOLATED_LOWER_RESIDUES == [1, 7, 13, 19, 23]);
    check("|twin_lower| == 3", TWIN_PRIME_LOWER_RESIDUES.len() == 3);
    check("|isolated| == 5", ISOLATED_LOWER_RESIDUES.len() == 5);
    check("partition covers U(30)",
        TWIN_PRIME_LOWER_RESIDUES.len() + ISOLATED_LOWER_RESIDUES.len() == U30_CARDINALITY);

    header("R2: arrays are disjoint subsets of U(30)");
    for &r in &TWIN_PRIME_LOWER_RESIDUES {
        check(&format!("twin lower {} ∈ U(30)", r),
            RESIDUES.contains(&r));
        check(&format!("twin lower {} ∉ isolated", r),
            !ISOLATED_LOWER_RESIDUES.contains(&r));
    }
    for &r in &ISOLATED_LOWER_RESIDUES {
        check(&format!("isolated {} ∈ U(30)", r),
            RESIDUES.contains(&r));
        check(&format!("isolated {} ∉ twin lower", r),
            !TWIN_PRIME_LOWER_RESIDUES.contains(&r));
    }

    header("R3: defining property — gcd((r+2) mod 30, 30) = 1 iff r ∈ twin_lower");
    for &r in &RESIDUES {
        let s = (r + 2) % MODULUS_U;
        let coprime = gcd(s, MODULUS_U) == 1;
        let in_twin = TWIN_PRIME_LOWER_RESIDUES.contains(&r);
        let in_isolated = ISOLATED_LOWER_RESIDUES.contains(&r);
        check(&format!("r={}: coprime({}, 30)={} ↔ twin={}", r, s, coprime, in_twin),
            coprime == in_twin && coprime != in_isolated);
    }

    header("R4: twin residue pairs (r, (r+2) mod 30) — both in U(30)");
    for &r in &TWIN_PRIME_LOWER_RESIDUES {
        let s = (r + 2) % MODULUS_U;
        check(&format!("({}, {}) both coprime to 30", r, s),
            gcd(r, MODULUS_U) == 1 && gcd(s, MODULUS_U) == 1);
    }

    header("R5: predicate is_twin_prime_candidate_lower — agrees with array");
    for n in 0u64..120 {
        let r = n % MODULUS_U;
        let pred = is_twin_prime_candidate_lower(n);
        let in_set = TWIN_PRIME_LOWER_RESIDUES.contains(&r);
        if pred != in_set {
            panic!("predicate disagrees at n={}: pred={}, set={}", n, pred, in_set);
        }
    }
    pass("predicate ↔ array agreement on n ∈ [0, 120)");

    header("R6: edge cases — small n, primes ≤ 5, n=0");
    // The predicate is purely residue-based; it accepts certain non-primes (e.g., n=11)
    // and may accept n=0 + offset. That's correct: predicate is necessary, not sufficient.
    check("predicate(0) = false  (0 mod 30 = 0, not in {11,17,29})",
        !is_twin_prime_candidate_lower(0));
    check("predicate(1) = false  (1 mod 30 = 1)",
        !is_twin_prime_candidate_lower(1));
    check("predicate(11) = true",
        is_twin_prime_candidate_lower(11));
    check("predicate(2) = false  (2 not in U(30))",
        !is_twin_prime_candidate_lower(2));
    check("predicate(MODULUS_U) = false  (30 mod 30 = 0)",
        !is_twin_prime_candidate_lower(MODULUS_U));
    check("predicate(MODULUS_U + 11) = true",
        is_twin_prime_candidate_lower(MODULUS_U + 11));

    header("R7: non-overlap with isolated set on the predicate");
    for n in 0u64..120 {
        let r = n % MODULUS_U;
        if ISOLATED_LOWER_RESIDUES.contains(&r) {
            check(&format!("n={} (residue {}): predicate must reject", n, r),
                !is_twin_prime_candidate_lower(n));
        }
    }

    header("R8: empirical sweep — every twin prime in [7, 100_000] passes the predicate");
    let n_max: u64 = 100_000;
    let mut twins = 0u64;
    let mut violations = 0u64;
    let mut by_residue = [0u64; 30];
    let mut p = 7u64;
    while p <= n_max {
        if is_prime_oracle(p) && is_prime_oracle(p + 2) {
            twins += 1;
            let r = (p % MODULUS_U) as usize;
            by_residue[r] += 1;
            if !is_twin_prime_candidate_lower(p) {
                violations += 1;
                println!("  VIOLATION at p={} (residue {})", p, r);
            }
        }
        p += 2;
    }
    println!("  twin pairs in [7, {}]: {}", n_max, twins);
    print!("  distribution: ");
    for r in 0..30 {
        if by_residue[r] > 0 { print!("[r={}: {}] ", r, by_residue[r]); }
    }
    println!();
    check("zero violations across 100,000 candidates", violations == 0);
    check("every recorded twin lands in {11, 17, 29}",
        by_residue.iter().enumerate()
            .filter(|&(_, &c)| c > 0)
            .all(|(r, _)| matches!(r, 11 | 17 | 29)));

    header("R9: residue distribution is balanced (Dirichlet at 100k)");
    let total = by_residue[11] + by_residue[17] + by_residue[29];
    let mean = total as f64 / 3.0;
    let max_dev = [by_residue[11], by_residue[17], by_residue[29]].iter()
        .map(|&c| ((c as f64) - mean).abs() / mean)
        .fold(0.0_f64, f64::max);
    println!("  total={}, mean per class≈{:.0}, max deviation = {:.2}%",
        total, mean, max_dev * 100.0);
    check("every class within 5% of mean", max_dev < 0.05);

    header("R10: ISOLATED residues really can't be lower twin — independent check");
    // For each residue r in ISOLATED_LOWER_RESIDUES, sweep all p ≡ r (mod 30)
    // up to 1000 and verify NO twin pair (p, p+2) where both are prime.
    for &r in &ISOLATED_LOWER_RESIDUES {
        let mut p = r;
        while p < 1000 {
            if p > 5 && is_prime_oracle(p) && is_prime_oracle(p + 2) {
                panic!("[FAIL] residue {} produced twin pair ({}, {}) — \
                       isolated claim broken", r, p, p + 2);
            }
            p += MODULUS_U;
        }
    }
    pass("isolated residues {1,7,13,19,23}: zero twin pairs in [1, 1000]");
    // ... but note: (3,5) and (5,7) are twin pairs with p=3, p=5 (both ≤ 5).
    // The claim is for p > 5 only — verified separately.
    check("(3, 5) is twin but p=3 ≤ 5 — outside the claim", is_prime_oracle(3) && is_prime_oracle(5));
    check("(5, 7) is twin but p=5 ≤ 5 — outside the claim", is_prime_oracle(5) && is_prime_oracle(7));

    header("R11: cross-check — `gcd(r, 30) = 1` is necessary for any candidate");
    // r ∉ U(30) ⟹ predicate must reject n with n ≡ r (mod 30).
    for r in 0u64..MODULUS_U {
        if gcd(r, MODULUS_U) != 1 {
            for offset in [0u64, 30, 60, 90].iter() {
                let n = r + offset;
                check(&format!("n={} (residue {} not coprime): predicate rejects", n, r),
                    !is_twin_prime_candidate_lower(n));
            }
        }
    }

    println!("\n=== ALL RIGOR CHECKS PASSED ===");
    println!("11 check classes, 100,000-element empirical sweep, zero violations.");
    println!("Claim is rigorous and verifiable from MODULUS_U=30 and gcd alone.");
}
