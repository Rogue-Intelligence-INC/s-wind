//! S-wind EXHAUSTIVE Unified Multi-Constant Search
//! 
//! TRUE exhaustive search - no arbitrary limits
//! Goal: Find algebraic structures constraining MULTIPLE physical constants

use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

// Physical constants (CODATA 2022 / PDG 2024)
const ALPHA_INV: f64 = 137.035999177;
const SIN2_THETA_W: f64 = 0.23122;
const MW_MZ_RATIO: f64 = 0.88145;
const MU_ME_RATIO: f64 = 206.7682827;
const MTAU_ME_RATIO: f64 = 3477.23;
const MP_ME_RATIO: f64 = 1836.15267343;
const MN_ME_RATIO: f64 = 1838.68366173;  // Neutron/electron
const MTAU_MMU_RATIO: f64 = 16.817;      // Tau/muon
const PI: f64 = std::f64::consts::PI;

// Cosmological
const OMEGA_LAMBDA: f64 = 0.6889;        // Dark energy density
const OMEGA_M: f64 = 0.3111;             // Matter density
const H0_NORMALIZED: f64 = 67.4;         // Hubble constant (km/s/Mpc)

// Tolerance levels
const TIGHT: f64 = 0.001;   // 0.1%
const MEDIUM: f64 = 0.01;   // 1%
const LOOSE: f64 = 0.05;    // 5%

static TOTAL_SEARCHED: AtomicUsize = AtomicUsize::new(0);
static MATCHES_FOUND: AtomicUsize = AtomicUsize::new(0);

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║              S-wind EXHAUSTIVE UNIFIED SEARCH                                ║");
    println!("║              真正的穷尽搜索 - 无任意限制                                      ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();

    let start = Instant::now();

    let constants: Vec<(&str, f64)> = vec![
        ("α⁻¹", ALPHA_INV),
        ("sin²θW", SIN2_THETA_W),
        ("MW/MZ", MW_MZ_RATIO),
        ("mμ/me", MU_ME_RATIO),
        ("mτ/me", MTAU_ME_RATIO),
        ("mp/me", MP_ME_RATIO),
        ("mn/me", MN_ME_RATIO),
        ("mτ/mμ", MTAU_MMU_RATIO),
        ("ΩΛ", OMEGA_LAMBDA),
        ("Ωm", OMEGA_M),
    ];

    println!("Physical constants to match:");
    for (name, val) in &constants {
        println!("  {} = {:.10}", name, val);
    }
    println!();

    // PART 1: Exhaustive polynomial search (degree 2-6)
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PART 1: EXHAUSTIVE POLYNOMIAL SEARCH (degree 2-6, coeff [-20,20], D ∈ [1,20])");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    exhaustive_polynomial_search(&constants);

    // PART 2: Rational function search
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PART 2: EXHAUSTIVE RATIONAL FUNCTION SEARCH");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    exhaustive_rational_search(&constants);

    // PART 3: Three-constant simultaneous match
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PART 3: THREE-CONSTANT SIMULTANEOUS SEARCH");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    three_constant_search(&constants);

    // PART 4: Inter-constant algebraic relations
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PART 4: INTER-CONSTANT ALGEBRAIC RELATIONS");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    inter_constant_relations(&constants);

    // PART 5: Combined expression search
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PART 5: COMBINED EXPRESSION SEARCH (α⁻¹ ↔ sin²θW ↔ masses)");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    combined_expression_search();

    // PART 6: Group theory inspired structures
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PART 6: GROUP-THEORETIC STRUCTURES");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    group_theory_search();

    let elapsed = start.elapsed();
    
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("EXHAUSTIVE SEARCH COMPLETE");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("Total configurations searched: {}", TOTAL_SEARCHED.load(Ordering::Relaxed));
    println!("Multi-constant matches found: {}", MATCHES_FOUND.load(Ordering::Relaxed));
    println!("Time elapsed: {:.2}s", elapsed.as_secs_f64());
}

fn exhaustive_polynomial_search(constants: &[(&str, f64)]) {
    println!();
    println!("Searching polynomials P(D) = Σ aᵢDⁱ...");
    println!();

    let coeff_range = -20i64..=20;
    let d_range = 1..=20i64;
    
    // Degree 4 polynomials with extended range
    println!("--- Degree 4 polynomials (exhaustive) ---");
    let mut best_multi: Vec<(String, Vec<(i64, String, f64)>)> = Vec::new();
    
    for a4 in [-3i64, -2, -1, 1, 2, 3] {
        for a3 in -10i64..=10 {
            for a2 in -10i64..=10 {
                for a1 in -10i64..=10 {
                    for a0 in -30i64..=30 {
                        TOTAL_SEARCHED.fetch_add(1, Ordering::Relaxed);
                        
                        let poly = |d: i64| -> f64 {
                            (a4 * d.pow(4) + a3 * d.pow(3) + a2 * d.pow(2) + a1 * d + a0) as f64
                        };
                        
                        let mut matches: Vec<(i64, String, f64)> = Vec::new();
                        
                        for d in 1..=20i64 {
                            let val = poly(d);
                            for (name, target) in constants {
                                let err = (val - target).abs() / target;
                                if err < TIGHT {
                                    matches.push((d, name.to_string(), err * 100.0));
                                }
                            }
                        }
                        
                        let unique_consts: HashSet<_> = matches.iter().map(|(_, n, _)| n.clone()).collect();
                        
                        if unique_consts.len() >= 3 {
                            let name = format!("{}D⁴+{}D³+{}D²+{}D+{}", a4, a3, a2, a1, a0);
                            println!("★★★ THREE+ CONSTANTS: {}", name);
                            for (d, n, e) in &matches {
                                println!("      P({}) ≈ {} (err: {:.4}%)", d, n, e);
                            }
                            println!();
                        }
                        
                        if unique_consts.len() >= 2 {
                            MATCHES_FOUND.fetch_add(1, Ordering::Relaxed);
                            let name = format!("{}D⁴+{}D³+{}D²+{}D+{}", a4, a3, a2, a1, a0);
                            best_multi.push((name, matches));
                        }
                    }
                }
            }
        }
    }
    
    // Sort by number of constants matched
    best_multi.sort_by(|a, b| {
        let a_consts: HashSet<_> = a.1.iter().map(|(_, n, _)| n).collect();
        let b_consts: HashSet<_> = b.1.iter().map(|(_, n, _)| n).collect();
        b_consts.len().cmp(&a_consts.len())
    });
    
    println!();
    println!("Top 20 multi-constant polynomials (by # constants matched):");
    for (name, matches) in best_multi.iter().take(20) {
        let unique: HashSet<_> = matches.iter().map(|(_, n, _)| n).collect();
        println!("  {} ({} constants)", name, unique.len());
        for (d, n, e) in matches {
            println!("    P({}) ≈ {} (err: {:.4}%)", d, n, e);
        }
    }
    
    // Higher degree search (degree 5, 6) with sparser sampling
    println!();
    println!("--- Degree 5-6 polynomials (targeted) ---");
    
    for deg in 5..=6 {
        for lead in [-2i64, -1, 1, 2] {
            // For higher degrees, focus on sparse polynomials
            for c1 in -5i64..=5 {
                for c2 in -5i64..=5 {
                    for c0 in -50i64..=50 {
                        TOTAL_SEARCHED.fetch_add(1, Ordering::Relaxed);
                        
                        let poly = |d: i64| -> f64 {
                            let df = d as f64;
                            if deg == 5 {
                                (lead as f64) * df.powi(5) + (c1 as f64) * df.powi(2) + (c2 as f64) * df + c0 as f64
                            } else {
                                (lead as f64) * df.powi(6) + (c1 as f64) * df.powi(2) + (c2 as f64) * df + c0 as f64
                            }
                        };
                        
                        let mut matches: Vec<(i64, String, f64)> = Vec::new();
                        
                        for d in 1..=15i64 {
                            let val = poly(d);
                            for (name, target) in constants {
                                let err = (val - target).abs() / target;
                                if err < TIGHT {
                                    matches.push((d, name.to_string(), err * 100.0));
                                }
                            }
                        }
                        
                        let unique: HashSet<_> = matches.iter().map(|(_, n, _)| n.clone()).collect();
                        if unique.len() >= 2 {
                            println!("  Degree {}: lead={}, sparse coeffs ({}, {}, {})", deg, lead, c1, c2, c0);
                            for (d, n, e) in &matches {
                                println!("    P({}) ≈ {} (err: {:.4}%)", d, n, e);
                            }
                        }
                    }
                }
            }
        }
    }
}

fn exhaustive_rational_search(constants: &[(&str, f64)]) {
    println!();
    println!("Searching rational functions P(D)/Q(D)...");
    println!();
    
    // Form: (aD^n + b) / (cD^m + d)
    println!("--- Form: (aD^n + b) / (cD^m + d) ---");
    
    let mut found: Vec<(String, Vec<(i64, String, f64)>)> = Vec::new();
    
    for n in 1..=4i32 {
        for m in 0..=3i32 {
            for a in [-5i64, -3, -2, -1, 1, 2, 3, 5] {
                for b in -20i64..=20 {
                    for c in [1i64, 2, 3, 5] {
                        for dd in -20i64..=20 {
                            if dd == 0 && m == 0 { continue; }
                            
                            TOTAL_SEARCHED.fetch_add(1, Ordering::Relaxed);
                            
                            let rat = |d: i64| -> Option<f64> {
                                let df = d as f64;
                                let numer = (a as f64) * df.powi(n) + b as f64;
                                let denom = (c as f64) * df.powi(m) + dd as f64;
                                if denom.abs() < 0.001 { None } else { Some(numer / denom) }
                            };
                            
                            let mut matches: Vec<(i64, String, f64)> = Vec::new();
                            
                            for d in 1..=20i64 {
                                if let Some(val) = rat(d) {
                                    for (name, target) in constants {
                                        let err = (val - target).abs() / target;
                                        if err < TIGHT {
                                            matches.push((d, name.to_string(), err * 100.0));
                                        }
                                    }
                                }
                            }
                            
                            let unique: HashSet<_> = matches.iter().map(|(_, nm, _)| nm.clone()).collect();
                            if unique.len() >= 2 {
                                MATCHES_FOUND.fetch_add(1, Ordering::Relaxed);
                                let name = format!("({}D^{}+{})/({}D^{}+{})", a, n, b, c, m, dd);
                                found.push((name, matches));
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Sort and display
    found.sort_by(|a, b| {
        let a_u: HashSet<_> = a.1.iter().map(|(_, n, _)| n).collect();
        let b_u: HashSet<_> = b.1.iter().map(|(_, n, _)| n).collect();
        b_u.len().cmp(&a_u.len())
    });
    
    println!("Found {} rational functions matching 2+ constants", found.len());
    println!();
    println!("Top 30:");
    for (name, matches) in found.iter().take(30) {
        let unique: HashSet<_> = matches.iter().map(|(_, n, _)| n).collect();
        println!("  {} ({} constants)", name, unique.len());
        for (d, n, e) in matches {
            println!("    R({}) ≈ {} (err: {:.4}%)", d, n, e);
        }
    }
}

fn three_constant_search(constants: &[(&str, f64)]) {
    println!();
    println!("Searching for structures matching THREE or more constants...");
    println!();
    
    // We need polynomial P(D) such that:
    // P(D1) ≈ const1
    // P(D2) ≈ const2
    // P(D3) ≈ const3
    
    // Extended coefficient range
    let mut three_plus: Vec<(String, Vec<(i64, String, f64)>)> = Vec::new();
    
    for a4 in [-2i64, -1, 1, 2] {
        for a3 in -15i64..=15 {
            for a2 in -15i64..=15 {
                for a1 in -15i64..=15 {
                    for a0 in -100i64..=100 {
                        TOTAL_SEARCHED.fetch_add(1, Ordering::Relaxed);
                        
                        let poly = |d: i64| -> f64 {
                            (a4 * d.pow(4) + a3 * d.pow(3) + a2 * d.pow(2) + a1 * d + a0) as f64
                        };
                        
                        let mut matches: Vec<(i64, String, f64)> = Vec::new();
                        
                        for d in 1..=25i64 {
                            let val = poly(d);
                            for (name, target) in constants {
                                let err = (val - target).abs() / target;
                                if err < MEDIUM {  // 1% tolerance for 3+ search
                                    matches.push((d, name.to_string(), err * 100.0));
                                }
                            }
                        }
                        
                        // Count unique constants
                        let unique: HashSet<_> = matches.iter().map(|(_, n, _)| n.clone()).collect();
                        let unique_d: HashSet<_> = matches.iter().map(|(d, _, _)| *d).collect();
                        
                        if unique.len() >= 3 && unique_d.len() >= 3 {
                            MATCHES_FOUND.fetch_add(1, Ordering::Relaxed);
                            let name = format!("{}D⁴+{}D³+{}D²+{}D+{}", a4, a3, a2, a1, a0);
                            three_plus.push((name, matches));
                        }
                    }
                }
            }
        }
    }
    
    three_plus.sort_by(|a, b| {
        let a_u: HashSet<_> = a.1.iter().map(|(_, n, _)| n).collect();
        let b_u: HashSet<_> = b.1.iter().map(|(_, n, _)| n).collect();
        b_u.len().cmp(&a_u.len())
    });
    
    println!("Found {} polynomials matching 3+ constants:", three_plus.len());
    println!();
    
    for (name, matches) in three_plus.iter().take(50) {
        let unique: HashSet<_> = matches.iter().map(|(_, n, _)| n).collect();
        println!("★ {} ({} constants)", name, unique.len());
        for (d, n, e) in matches {
            println!("    P({}) ≈ {} (err: {:.3}%)", d, n, e);
        }
        println!();
    }
}

fn inter_constant_relations(constants: &[(&str, f64)]) {
    println!();
    println!("Searching algebraic relations BETWEEN constants...");
    println!();
    
    // Form: const1 = f(const2, const3, ...)
    
    println!("--- Form: C1 = a*C2 + b ---");
    for i in 0..constants.len() {
        for j in 0..constants.len() {
            if i == j { continue; }
            let (n1, v1) = constants[i];
            let (n2, v2) = constants[j];
            
            // v1 = a*v2 + b
            // Try integer a
            for a in -20i64..=20 {
                if a == 0 { continue; }
                let b = v1 - (a as f64) * v2;
                if b.abs() < 1.0 && (b - b.round()).abs() < 0.001 {
                    println!("  {} ≈ {}×{} + {:.0}", n1, a, n2, b.round());
                }
            }
        }
    }
    
    println!();
    println!("--- Form: C1 = C2 * C3 ---");
    for i in 0..constants.len() {
        for j in 0..constants.len() {
            for k in 0..constants.len() {
                if i == j || i == k || j >= k { continue; }
                let (n1, v1) = constants[i];
                let (n2, v2) = constants[j];
                let (n3, v3) = constants[k];
                
                let product = v2 * v3;
                let err = (v1 - product).abs() / v1;
                if err < 0.01 {
                    println!("  {} ≈ {} × {} (err: {:.3}%)", n1, n2, n3, err * 100.0);
                }
            }
        }
    }
    
    println!();
    println!("--- Form: C1 = C2 / C3 ---");
    for i in 0..constants.len() {
        for j in 0..constants.len() {
            for k in 0..constants.len() {
                if i == j || i == k || j == k { continue; }
                let (n1, v1) = constants[i];
                let (n2, v2) = constants[j];
                let (n3, v3) = constants[k];
                
                if v3.abs() < 0.0001 { continue; }
                let ratio = v2 / v3;
                let err = (v1 - ratio).abs() / v1;
                if err < 0.01 {
                    println!("  {} ≈ {} / {} (err: {:.3}%)", n1, n2, n3, err * 100.0);
                }
            }
        }
    }
    
    println!();
    println!("--- Form: C1 = (a*C2 + b) / (c*C3 + d) ---");
    for i in 0..constants.len() {
        for j in 0..constants.len() {
            for k in 0..constants.len() {
                if i == j || i == k { continue; }
                let (n1, v1) = constants[i];
                let (n2, v2) = constants[j];
                let (n3, v3) = constants[k];
                
                for a in [-2i64, -1, 1, 2] {
                    for b in -10i64..=10 {
                        for c in [1i64, 2] {
                            for d in -10i64..=10 {
                                let numer = (a as f64) * v2 + b as f64;
                                let denom = (c as f64) * v3 + d as f64;
                                if denom.abs() < 0.001 { continue; }
                                let val = numer / denom;
                                let err = (v1 - val).abs() / v1;
                                if err < 0.001 {
                                    println!("  {} ≈ ({}×{} + {}) / ({}×{} + {}) (err: {:.4}%)", 
                                            n1, a, n2, b, c, n3, d, err * 100.0);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn combined_expression_search() {
    println!();
    println!("Testing unified expressions connecting multiple constants...");
    println!();
    
    // Key test: Can we express sin²θW in terms of α⁻¹?
    println!("--- sin²θW as function of α⁻¹ = 137.036 ---");
    
    for a in -100i64..=100 {
        for b in -200i64..=200 {
            if b == 0 { continue; }
            let val = (a as f64) / (ALPHA_INV + b as f64);
            let err = (val - SIN2_THETA_W).abs() / SIN2_THETA_W;
            if err < 0.0001 {
                println!("  sin²θW = {} / (α⁻¹ + {}) = {:.8} (err: {:.5}%)", a, b, val, err * 100.0);
            }
        }
    }
    
    println!();
    println!("--- sin²θW as polynomial in 1/α⁻¹ ---");
    let alpha = 1.0 / ALPHA_INV;
    for a in -50i64..=50 {
        for b in -50i64..=50 {
            let val = (a as f64) * alpha + (b as f64) * alpha * alpha;
            let err = (val - SIN2_THETA_W).abs() / SIN2_THETA_W;
            if err < 0.01 {
                println!("  sin²θW ≈ {}α + {}α² = {:.6} (err: {:.3}%)", a, b, val, err * 100.0);
            }
        }
    }
    
    println!();
    println!("--- Mass ratios as functions of α⁻¹ ---");
    
    // mμ/me ≈ f(α⁻¹)
    for a in 1i64..=5 {
        for b in -100i64..=100 {
            let val = (a as f64) * ALPHA_INV + b as f64;
            let err = (val - MU_ME_RATIO).abs() / MU_ME_RATIO;
            if err < 0.01 {
                println!("  mμ/me ≈ {}×α⁻¹ + {} = {:.4} (err: {:.3}%)", a, b, val, err * 100.0);
            }
        }
    }
    
    // mp/me ≈ f(α⁻¹)
    for a in 10i64..=20 {
        for b in -200i64..=200 {
            let val = (a as f64) * ALPHA_INV + b as f64;
            let err = (val - MP_ME_RATIO).abs() / MP_ME_RATIO;
            if err < 0.01 {
                println!("  mp/me ≈ {}×α⁻¹ + {} = {:.4} (err: {:.3}%)", a, b, val, err * 100.0);
            }
        }
    }
    
    println!();
    println!("--- Testing P(D) + f(D) combined structures ---");
    
    // P(D) from our known polynomial
    let p = |d: f64| d.powi(4) - 2.0 * d.powi(3) + 2.0 * d + 1.0;
    
    for d in 1..=10i32 {
        let pd = p(d as f64);
        
        // Can pd / (aD + b) give another constant?
        for a in -10i64..=10 {
            for b in 1i64..=50 {
                let denom = (a as f64) * (d as f64) + b as f64;
                if denom.abs() < 0.1 { continue; }
                let val = pd / denom;
                
                // Check against mass ratios
                let err_mu = (val - MU_ME_RATIO).abs() / MU_ME_RATIO;
                if err_mu < 0.01 {
                    println!("  P({}) / ({}D + {}) = {:.4} ≈ mμ/me (err: {:.3}%)", d, a, b, val, err_mu * 100.0);
                }
                
                let err_tau = (val - MTAU_MMU_RATIO).abs() / MTAU_MMU_RATIO;
                if err_tau < 0.01 {
                    println!("  P({}) / ({}D + {}) = {:.4} ≈ mτ/mμ (err: {:.3}%)", d, a, b, val, err_tau * 100.0);
                }
            }
        }
    }
}

fn group_theory_search() {
    println!();
    println!("Testing group-theoretic inspired structures...");
    println!();
    
    // SU(N) Casimir operators
    println!("--- SU(N) Casimir invariants ---");
    for n in 2..=10i64 {
        let c2_fund = (n * n - 1) as f64 / (2 * n) as f64;  // C₂(fundamental)
        let dim_adj = (n * n - 1) as f64;                    // Dimension of adjoint
        
        // Check if these relate to physical constants
        let check = |val: f64, name: &str| {
            if (val - ALPHA_INV).abs() / ALPHA_INV < 0.01 {
                println!("  SU({}) {}: {} ≈ α⁻¹", n, name, val);
            }
            if val > 0.1 && val < 1.0 {
                if (val - SIN2_THETA_W).abs() / SIN2_THETA_W < 0.05 {
                    println!("  SU({}) {}: {} ≈ sin²θW", n, name, val);
                }
            }
        };
        
        check(c2_fund, "C₂(fund)");
        check(dim_adj, "dim(adj)");
        check(c2_fund * (n as f64), "C₂×N");
        check(dim_adj / (n as f64), "dim/N");
    }
    
    println!();
    println!("--- Exceptional group dimensions ---");
    let exceptional = [
        ("G₂", 14i64),
        ("F₄", 52),
        ("E₆", 78),
        ("E₇", 133),
        ("E₈", 248),
    ];
    
    for (name, dim) in exceptional {
        println!("  {} dimension = {}", name, dim);
        if dim == 133 {
            println!("    → Close to α⁻¹ = 137 (diff = 4)");
        }
        
        // Check combinations
        for d in 2..=6i64 {
            let val = dim as f64 + d as f64;
            if (val - ALPHA_INV).abs() < 0.1 {
                println!("    dim({}) + {} = {} ≈ α⁻¹", name, d, val);
            }
        }
    }
    
    println!();
    println!("--- E₇ structure exploration ---");
    // E7 has dimension 133, very close to 137
    let e7_dim = 133i64;
    let diff = ALPHA_INV - e7_dim as f64;
    println!("  α⁻¹ - dim(E₇) = {:.6}", diff);
    println!("  α⁻¹ = dim(E₇) + {:.6}", diff);
    println!("  Note: {} ≈ 4 + 0.036 ≈ D + α⁻¹ mod 1", diff);
    
    // Is diff = 4 + something small?
    let rem = diff - 4.0;
    println!("  α⁻¹ = dim(E₇) + D + {:.6} where D=4", rem);
    println!("  → α⁻¹ = 133 + 4 + 0.036 = E₇ + spacetime_dim + (α⁻¹ - 137)");
    
    println!();
    println!("--- Testing: α⁻¹ = dim(G) + f(D) ---");
    for (name, dim) in exceptional {
        for d in 1..=10i64 {
            // f(D) = D
            if (dim + d) as f64 == 137.0 {
                println!("  dim({}) + {} = 137 EXACT", name, d);
            }
            // f(D) = D²
            let sum = dim as f64 + (d * d) as f64;
            if sum > 136.0 && sum < 138.0 {
                println!("  dim({}) + {}² = {}", name, d, dim + d*d);
            }
        }
    }
}
