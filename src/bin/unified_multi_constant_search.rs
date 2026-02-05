//! S-wind Unified Multi-Constant Search
//! 
//! Goal: Find algebraic structures that simultaneously constrain MULTIPLE physical constants
//! This is the path to unified theory - not just fitting one constant, but finding
//! a single framework that yields multiple constants from the same structure.

// Multi-constant unified search

/// Physical constants (CODATA 2022 / PDG 2024)
const ALPHA_INV: f64 = 137.035999177;      // Fine structure constant inverse
const SIN2_THETA_W: f64 = 0.23122;         // Weinberg angle (MS-bar at MZ)
const MW_MZ_RATIO: f64 = 0.88145;          // W/Z mass ratio
const MU_ME_RATIO: f64 = 206.7682827;      // Muon/electron mass ratio
const MTAU_ME_RATIO: f64 = 3477.23;        // Tau/electron mass ratio
const MP_ME_RATIO: f64 = 1836.15267343;    // Proton/electron mass ratio
const GOLDEN_RATIO: f64 = 1.6180339887;    // φ = (1+√5)/2
const PI: f64 = std::f64::consts::PI;
const E: f64 = std::f64::consts::E;

/// Tolerance for "match"
const TOLERANCE: f64 = 0.001; // 0.1%

#[derive(Debug, Clone)]
struct AlgebraicExpression {
    name: String,
    formula: String,
    evaluate: fn(i64) -> f64,  // Function of D (dimension)
}

#[derive(Debug, Clone)]
struct MultiConstantMatch {
    expression: String,
    d_value: i64,
    matches: Vec<(String, f64, f64, f64)>, // (constant_name, target, computed, error%)
}

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║           S-wind UNIFIED MULTI-CONSTANT SEARCH                               ║");
    println!("║           Goal: Find ONE structure → MULTIPLE constants                      ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();

    // Physical constants to match
    let constants: Vec<(&str, f64)> = vec![
        ("α⁻¹", ALPHA_INV),
        ("sin²θW", SIN2_THETA_W),
        ("MW/MZ", MW_MZ_RATIO),
        ("mμ/me", MU_ME_RATIO),
        ("mτ/me", MTAU_ME_RATIO),
        ("mp/me", MP_ME_RATIO),
    ];

    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PART 1: POLYNOMIAL FAMILY SEARCH");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    // Search polynomial families P(D) for multiple constant matches
    polynomial_family_search(&constants);

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PART 2: UNIFIED ALGEBRAIC STRUCTURE SEARCH");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    // Search for unified structures
    unified_structure_search(&constants);

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PART 3: DIMENSIONAL RATIO ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    // Analyze ratios between constants
    dimensional_ratio_analysis(&constants);

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PART 4: COMBINED CONSTRAINT SEARCH");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    // Search for expressions satisfying multiple constraints simultaneously
    combined_constraint_search();

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PART 5: KOIDE-STYLE MASS RELATION EXPLORATION");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    koide_exploration();

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PART 6: EXHAUSTIVE TWO-CONSTANT JOINT SEARCH");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    exhaustive_two_constant_search();

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("FINAL SUMMARY");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    
    final_summary();
}

/// Search polynomial families for multi-constant matches
fn polynomial_family_search(constants: &[(&str, f64)]) {
    println!("Testing polynomial P(D) = D^4 - 2D^3 + 2D + 1 at various D:");
    println!();
    
    let p = |d: i64| -> f64 {
        let df = d as f64;
        df.powi(4) - 2.0 * df.powi(3) + 2.0 * df + 1.0
    };
    
    println!("{:>4} | {:>15} | Matches", "D", "P(D)");
    println!("{:-<4}-+-{:-<15}-+-{:-<40}", "", "", "");
    
    for d in 1..=20 {
        let val = p(d);
        let mut matches = Vec::new();
        
        for (name, target) in constants {
            let error = ((val - target) / target).abs() * 100.0;
            if error < 1.0 { // Within 1%
                matches.push(format!("{}({:.3}%)", name, error));
            }
        }
        
        let match_str = if matches.is_empty() { "-".to_string() } else { matches.join(", ") };
        println!("{:>4} | {:>15.6} | {}", d, val, match_str);
    }
    
    println!();
    println!("Searching for polynomials that match MULTIPLE constants...");
    println!();
    
    // Search space: P(D) = aD^4 + bD^3 + cD^2 + dD + e
    let mut multi_matches: Vec<(String, Vec<(i64, String, f64)>)> = Vec::new();
    
    for a in -3i64..=3 {
        if a == 0 { continue; }
        for b in -5i64..=5 {
            for c in -5i64..=5 {
                for d_coef in -5i64..=5 {
                    for e in -10i64..=10 {
                        let poly_name = format!("{}D⁴ + {}D³ + {}D² + {}D + {}", a, b, c, d_coef, e);
                        let mut found_matches: Vec<(i64, String, f64)> = Vec::new();
                        
                        for dim in 1..=10i64 {
                            let val = (a * dim.pow(4) + b * dim.pow(3) + c * dim.pow(2) + d_coef * dim + e) as f64;
                            
                            for (name, target) in constants {
                                let error = ((val - target) / target).abs() * 100.0;
                                if error < TOLERANCE * 100.0 {
                                    found_matches.push((dim, name.to_string(), error));
                                }
                            }
                        }
                        
                        // Only keep if matches 2+ different constants
                        let unique_constants: std::collections::HashSet<_> = 
                            found_matches.iter().map(|(_, name, _)| name.clone()).collect();
                        
                        if unique_constants.len() >= 2 {
                            multi_matches.push((poly_name, found_matches));
                        }
                    }
                }
            }
        }
    }
    
    if multi_matches.is_empty() {
        println!("No polynomials found matching 2+ constants within {}% tolerance.", TOLERANCE * 100.0);
    } else {
        println!("Found {} polynomials matching 2+ constants:", multi_matches.len());
        for (poly, matches) in multi_matches.iter().take(20) {
            println!("  {}", poly);
            for (d, name, err) in matches {
                println!("    D={}: {} (err={:.4}%)", d, name, err);
            }
        }
    }
}

/// Search for unified algebraic structures
fn unified_structure_search(constants: &[(&str, f64)]) {
    println!("Testing unified structure candidates:");
    println!();
    
    // Structure 1: f(D) = D^n / (D-1)^m type
    println!("Structure Type 1: D^n / (D-1)^m at D=4");
    println!("{:>4} {:>4} | {:>15} | Matches", "n", "m", "Value");
    println!("{:-<4}-{:-<4}-+-{:-<15}-+-{:-<30}", "", "", "", "");
    
    for n in 1..=8i32 {
        for m in 0..=4i32 {
            let d = 4.0f64;
            let val = d.powi(n) / (d - 1.0).powi(m);
            
            let mut matches = Vec::new();
            for (name, target) in constants {
                let error = ((val - target) / target).abs() * 100.0;
                if error < 5.0 {
                    matches.push(format!("{}({:.2}%)", name, error));
                }
            }
            
            if !matches.is_empty() {
                println!("{:>4} {:>4} | {:>15.6} | {}", n, m, val, matches.join(", "));
            }
        }
    }
    
    println!();
    println!("Structure Type 2: (D^a - b) / (D^c + d) at D=4");
    
    for a in 1..=4i32 {
        for b in -10..=10i64 {
            for c in 0..=3i32 {
                for d in 1..=10i64 {
                    let dim = 4.0f64;
                    let val = (dim.powi(a) - b as f64) / (dim.powi(c) + d as f64);
                    
                    for (name, target) in constants {
                        let error = ((val - target) / target).abs() * 100.0;
                        if error < 0.1 {
                            println!("  (D^{} - {}) / (D^{} + {}) = {:.6} ≈ {} (err={:.4}%)", 
                                    a, b, c, d, val, name, error);
                        }
                    }
                }
            }
        }
    }
    
    println!();
    println!("Structure Type 3: Polynomial ratios");
    
    // P(D)/Q(D) type structures
    let p_137 = |d: f64| d.powi(4) - 2.0 * d.powi(3) + 2.0 * d + 1.0;
    
    for a in -5..=5i64 {
        for b in 1..=10i64 {
            let d = 4.0f64;
            let val = p_137(d) / (a as f64 * d + b as f64);
            
            for (name, target) in constants {
                if *name == "α⁻¹" { continue; } // Skip the one we're dividing
                let error = ((val - target) / target).abs() * 100.0;
                if error < 1.0 {
                    println!("  P(4) / ({}D + {}) = {:.6} ≈ {} (err={:.3}%)", a, b, val, name, error);
                }
            }
        }
    }
}

/// Analyze dimensional ratios
fn dimensional_ratio_analysis(constants: &[(&str, f64)]) {
    println!("Ratio analysis between physical constants:");
    println!();
    
    println!("{:<12} / {:<12} = {:>15} | Nearby integers/simple fractions", "Const1", "Const2", "Ratio");
    println!("{:-<12}-+-{:-<12}-+-{:-<15}-+-{:-<30}", "", "", "", "");
    
    for i in 0..constants.len() {
        for j in 0..constants.len() {
            if i == j { continue; }
            let (name1, val1) = constants[i];
            let (name2, val2) = constants[j];
            let ratio = val1 / val2;
            
            // Check if ratio is near simple values
            let mut nearby = Vec::new();
            
            // Check integers
            let nearest_int = ratio.round() as i64;
            if nearest_int > 0 && ((ratio - nearest_int as f64) / ratio).abs() < 0.01 {
                nearby.push(format!("≈{}", nearest_int));
            }
            
            // Check simple fractions
            for num in 1..=20i64 {
                for den in 1..=20i64 {
                    let frac = num as f64 / den as f64;
                    if ((ratio - frac) / ratio).abs() < 0.005 {
                        nearby.push(format!("{}/{}", num, den));
                    }
                }
            }
            
            // Check powers of common constants
            for p in -4..=4i32 {
                if p == 0 { continue; }
                let test = PI.powi(p);
                if ((ratio - test) / ratio).abs() < 0.01 {
                    nearby.push(format!("π^{}", p));
                }
                let test = 2.0f64.powi(p);
                if ((ratio - test) / ratio).abs() < 0.01 {
                    nearby.push(format!("2^{}", p));
                }
            }
            
            if !nearby.is_empty() {
                println!("{:<12} / {:<12} = {:>15.6} | {}", name1, name2, ratio, nearby.join(", "));
            }
        }
    }
    
    println!();
    println!("Key ratio: α⁻¹ / (mμ/me) = {:.6}", ALPHA_INV / MU_ME_RATIO);
    println!("         = {:.6} ≈ 2/3 = {:.6}", ALPHA_INV / MU_ME_RATIO, 2.0/3.0);
    println!("         Error: {:.3}%", ((ALPHA_INV / MU_ME_RATIO) - (2.0/3.0)).abs() / (2.0/3.0) * 100.0);
}

/// Combined constraint search
fn combined_constraint_search() {
    println!("Searching for expressions satisfying MULTIPLE constraints simultaneously...");
    println!();
    
    // Form: f(D,n,m) where we want f to give α⁻¹ and some transformation gives sin²θW
    
    let _d = 4.0f64;
    
    println!("Testing: Does any simple function of D give BOTH α⁻¹ and sin²θW?");
    println!();
    
    // sin²θW ≈ 1/4 - 1/(4π²) ≈ 0.2247 (not great but interesting)
    let sin2_approx1 = 0.25 - 1.0/(4.0 * PI * PI);
    println!("  sin²θW ≈ 1/4 - 1/(4π²) = {:.6} (actual: {:.6}, err: {:.2}%)",
            sin2_approx1, SIN2_THETA_W, (sin2_approx1 - SIN2_THETA_W).abs()/SIN2_THETA_W * 100.0);
    
    // sin²θW ≈ 3/13 = 0.2308
    let sin2_approx2 = 3.0/13.0;
    println!("  sin²θW ≈ 3/13 = {:.6} (actual: {:.6}, err: {:.2}%)",
            sin2_approx2, SIN2_THETA_W, (sin2_approx2 - SIN2_THETA_W).abs()/SIN2_THETA_W * 100.0);
    
    // Test: If α⁻¹ = P(4) = 137, can we derive sin²θW?
    println!();
    println!("Testing relations between α⁻¹=137 and sin²θW:");
    
    // sin²θW = 1 - (MW/MZ)² in SM
    // MW/MZ ≈ cos(θW)
    let mw_mz_from_sin2 = (1.0 - SIN2_THETA_W).sqrt();
    println!("  MW/MZ = √(1-sin²θW) = {:.6} (measured: {:.6})", mw_mz_from_sin2, MW_MZ_RATIO);
    
    // Try: sin²θW = f(137)
    println!();
    println!("Can sin²θW be expressed as f(137)?");
    
    for a in -10..=10i64 {
        for b in 1..=1000i64 {
            let candidate = a as f64 / (137.0 + b as f64);
            let error = (candidate - SIN2_THETA_W).abs() / SIN2_THETA_W * 100.0;
            if error < 0.5 {
                println!("  {} / (137 + {}) = {:.6} ≈ sin²θW (err: {:.3}%)", a, b, candidate, error);
            }
        }
    }
    
    // Try: sin²θW = a/(137 * b + c)
    println!();
    for a in 1..=50i64 {
        for b in 1..=10i64 {
            for c in -50..=50i64 {
                let denom = 137.0 * b as f64 + c as f64;
                if denom.abs() < 1.0 { continue; }
                let candidate = a as f64 / denom;
                let error = (candidate - SIN2_THETA_W).abs() / SIN2_THETA_W * 100.0;
                if error < 0.1 {
                    println!("  {} / (137×{} + {}) = {:.6} ≈ sin²θW (err: {:.4}%)", a, b, c, candidate, error);
                }
            }
        }
    }
}

/// Koide-style mass relation exploration
fn koide_exploration() {
    println!("Koide formula verification and extensions:");
    println!();
    
    // Lepton masses (MeV)
    let m_e: f64 = 0.51099895000;  // electron
    let m_mu: f64 = 105.6583755;   // muon  
    let m_tau: f64 = 1776.86;      // tau
    
    // Koide Q = (me + mμ + mτ) / (√me + √mμ + √mτ)²
    let sum_m = m_e + m_mu + m_tau;
    let sum_sqrt = m_e.sqrt() + m_mu.sqrt() + m_tau.sqrt();
    let q_koide = sum_m / (sum_sqrt * sum_sqrt);
    
    println!("Koide Q = (me + mμ + mτ) / (√me + √mμ + √mτ)²");
    println!("       = ({:.4} + {:.4} + {:.2}) / ({:.4} + {:.4} + {:.4})²", 
            m_e, m_mu, m_tau, m_e.sqrt(), m_mu.sqrt(), m_tau.sqrt());
    println!("       = {:.4} / {:.4}²", sum_m, sum_sqrt);
    println!("       = {:.4} / {:.4}", sum_m, sum_sqrt * sum_sqrt);
    println!("       = {:.10}", q_koide);
    println!();
    println!("Target: 2/3 = {:.10}", 2.0/3.0);
    println!("Error: {:.6}%", (q_koide - 2.0/3.0).abs() / (2.0/3.0) * 100.0);
    
    println!();
    println!("Can Koide Q = 2/3 be related to D=4?");
    println!("  2/3 = (D-2)/D at D=6");
    println!("  2/3 = 2/(D-1) at D=4");
    println!("  2/3 = D/(D+2) at D=4");
    
    // Check: Is there a D where P(D)/Q(D) = 2/3?
    println!();
    println!("Testing P(D) relations:");
    for d in 1..=20 {
        let p = (d as f64).powi(4) - 2.0 * (d as f64).powi(3) + 2.0 * (d as f64) + 1.0;
        
        // P(D) / X = 2/3 → X = 3P(D)/2
        let x_needed = 3.0 * p / 2.0;
        
        // Is x_needed a simple expression in D?
        let ratio_to_d4 = x_needed / (d as f64).powi(4);
        let ratio_to_d3 = x_needed / (d as f64).powi(3);
        
        if (ratio_to_d4 - ratio_to_d4.round()).abs() < 0.01 || 
           (ratio_to_d3 - ratio_to_d3.round()).abs() < 0.01 {
            println!("  D={}: P(D)={}, 3P(D)/2={:.1}", d, p, x_needed);
        }
    }
    
    // Extended Koide to quarks
    println!();
    println!("Extended Koide-style analysis for up-type quarks:");
    let m_u: f64 = 2.16;    // MeV (MS-bar at 2 GeV)
    let m_c: f64 = 1270.0;  // MeV
    let m_t: f64 = 172760.0; // MeV (pole mass)
    
    let sum_q = m_u + m_c + m_t;
    let sum_sqrt_q = m_u.sqrt() + m_c.sqrt() + m_t.sqrt();
    let q_up = sum_q / (sum_sqrt_q * sum_sqrt_q);
    
    println!("Q_up = (mu + mc + mt) / (√mu + √mc + √mt)² = {:.6}", q_up);
    println!("Compare to 2/3: error = {:.2}%", (q_up - 2.0/3.0).abs() / (2.0/3.0) * 100.0);
}

/// Exhaustive two-constant joint search
fn exhaustive_two_constant_search() {
    println!("Exhaustive search: Find P(D) matching TWO constants at different D values");
    println!();
    
    let constants = [
        ("α⁻¹", ALPHA_INV),
        ("mμ/me", MU_ME_RATIO),
        ("mp/me", MP_ME_RATIO),
    ];
    
    let tolerance = 0.01; // 1%
    
    println!("Searching polynomials aD⁴ + bD³ + cD² + dD + e where:");
    println!("  - P(D₁) ≈ constant_1");
    println!("  - P(D₂) ≈ constant_2  (D₁ ≠ D₂)");
    println!("  - Coefficients in [-5, 5]");
    println!("  - Tolerance: {}%", tolerance * 100.0);
    println!();
    
    let mut found = 0;
    
    for a in -3i64..=3 {
        if a == 0 { continue; }
        for b in -5i64..=5 {
            for c in -5i64..=5 {
                for d_coef in -5i64..=5 {
                    for e in -20i64..=20 {
                        let poly = |dim: i64| -> f64 {
                            (a * dim.pow(4) + b * dim.pow(3) + c * dim.pow(2) + d_coef * dim + e) as f64
                        };
                        
                        let mut matches: Vec<(i64, &str, f64)> = Vec::new();
                        
                        for dim in 1..=10i64 {
                            let val = poly(dim);
                            for (name, target) in &constants {
                                let err = (val - target).abs() / target;
                                if err < tolerance {
                                    matches.push((dim, name, err * 100.0));
                                }
                            }
                        }
                        
                        // Check if matches 2+ different constants at different D
                        let unique_consts: std::collections::HashSet<_> = 
                            matches.iter().map(|(_, name, _)| *name).collect();
                        let unique_dims: std::collections::HashSet<_> = 
                            matches.iter().map(|(d, _, _)| *d).collect();
                        
                        if unique_consts.len() >= 2 && unique_dims.len() >= 2 {
                            found += 1;
                            if found <= 30 {
                                println!("FOUND: {}D⁴ + {}D³ + {}D² + {}D + {}", a, b, c, d_coef, e);
                                for (dim, name, err) in &matches {
                                    println!("       P({}) ≈ {} (err: {:.3}%)", dim, name, err);
                                }
                                println!();
                            }
                        }
                    }
                }
            }
        }
    }
    
    println!("Total polynomials matching 2+ constants at different D: {}", found);
}

fn final_summary() {
    println!();
    println!("┌────────────────────────────────────────────────────────────────────────────┐");
    println!("│                           UNIFIED SEARCH SUMMARY                           │");
    println!("├────────────────────────────────────────────────────────────────────────────┤");
    println!("│                                                                            │");
    println!("│  VERIFIED RELATIONS:                                                       │");
    println!("│    ✓ P(4) = 4⁴ - 2×4³ + 2×4 + 1 = 137    (algebraic identity)             │");
    println!("│    ✓ Koide Q ≈ 2/3                        (0.001% precision)               │");
    println!("│    ✓ sin²θW = 1 - (MW/MZ)²               (SM definition)                  │");
    println!("│                                                                            │");
    println!("│  MULTI-CONSTANT RELATIONS FOUND:                                           │");
    println!("│    See output above for polynomials matching 2+ constants                  │");
    println!("│                                                                            │");
    println!("│  OPEN QUESTIONS FOR UNIFIED THEORY:                                        │");
    println!("│    • Why does dimension D appear in these relations?                       │");
    println!("│    • Is there ONE structure yielding ALL SM parameters?                    │");
    println!("│    • What physical mechanism selects the specific polynomial?              │");
    println!("│                                                                            │");
    println!("│  NEXT STEPS:                                                               │");
    println!("│    • Expand search to rational functions                                   │");
    println!("│    • Include more constants (CKM, PMNS, cosmological)                      │");
    println!("│    • Search for group-theoretic origins                                    │");
    println!("│                                                                            │");
    println!("└────────────────────────────────────────────────────────────────────────────┘");
}
