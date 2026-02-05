//! S-wind Algebraic Unified Theory Exploration (EPR Only)
//!
//! CORE PRINCIPLE: ALL computation uses EPR (infinite precision rational)
//! - NO floating point (f64)
//! - ALL physical constants as exact rationals
//! - ALL polynomials evaluated algebraically
//! - Uses Fate engine for pattern detection
//!
//! This is what S-wind is designed for.

use s_wind::{
    Rational, EPR,
    Fate,
    KnowledgeBase,
};

// ============================================================================
// PHYSICAL CONSTANTS AS EXACT RATIONALS
// ============================================================================

/// α⁻¹ = 137.035999177 ≈ 137035999177/1000000000
fn alpha_inv_rational() -> Rational {
    Rational::new(137035999177, 1000000000)
}

/// Integer target: α⁻¹ ≈ 137
fn alpha_inv_integer() -> Rational {
    Rational::from_i128(137)
}

/// sin²θW = 0.23122 = 23122/100000
fn sin2_theta_w() -> Rational {
    Rational::new(23122, 100000)
}

/// MW/MZ = 0.88145 = 88145/100000
fn mw_mz_ratio() -> Rational {
    Rational::new(88145, 100000)
}

/// mμ/me = 206.7682827 ≈ 2067682827/10000000
fn mu_me_ratio() -> Rational {
    Rational::new(2067682827, 10000000)
}

/// mp/me = 1836.15267343 ≈ 183615267343/100000000
fn mp_me_ratio() -> Rational {
    Rational::new(183615267343, 100000000)
}

/// mτ/me = 3477.23 = 347723/100
fn mtau_me_ratio() -> Rational {
    Rational::new(347723, 100)
}

/// ΩΛ = 0.6889 = 6889/10000
fn omega_lambda() -> Rational {
    Rational::new(6889, 10000)
}

/// Ωm = 0.3111 = 3111/10000
fn omega_matter() -> Rational {
    Rational::new(3111, 10000)
}

/// dim(E7) = 133
fn dim_e7() -> Rational {
    Rational::from_i128(133)
}

// ============================================================================
// ALGEBRAIC POLYNOMIAL EVALUATION (EXACT)
// ============================================================================

/// Polynomial P(D) = a4*D^4 + a3*D^3 + a2*D^2 + a1*D + a0
/// ALL operations are exact Rational arithmetic
fn eval_polynomial(coeffs: &[i128; 5], d: i128) -> Rational {
    let a4 = Rational::from_i128(coeffs[0]);
    let a3 = Rational::from_i128(coeffs[1]);
    let a2 = Rational::from_i128(coeffs[2]);
    let a1 = Rational::from_i128(coeffs[3]);
    let a0 = Rational::from_i128(coeffs[4]);
    let d_rat = Rational::from_i128(d);
    
    // D^4, D^3, D^2, D
    let d2 = d_rat * d_rat;
    let d3 = d2 * d_rat;
    let d4 = d3 * d_rat;
    
    // P(D) = a4*D^4 + a3*D^3 + a2*D^2 + a1*D + a0
    a4 * d4 + a3 * d3 + a2 * d2 + a1 * d_rat + a0
}

/// Check if polynomial value equals target (EXACT comparison)
fn poly_equals_target(coeffs: &[i128; 5], d: i128, target: &Rational) -> bool {
    let result = eval_polynomial(coeffs, d);
    result == *target
}

/// Check if polynomial value is within relative tolerance of target
fn poly_within_tolerance(coeffs: &[i128; 5], d: i128, target: &Rational, tol_num: i128, tol_den: i128) -> bool {
    let result = eval_polynomial(coeffs, d);
    let diff = if result > *target {
        result - *target
    } else {
        *target - result
    };
    let tolerance = *target * Rational::new(tol_num, tol_den);
    let tolerance_abs = if tolerance.is_negative() { -tolerance } else { tolerance };
    diff < tolerance_abs
}

// ============================================================================
// MAIN EXPLORATION
// ============================================================================

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║     S-wind ALGEBRAIC UNIFIED THEORY EXPLORATION                              ║");
    println!("║     ALL COMPUTATION USING EPR (INFINITE PRECISION RATIONAL)                  ║");
    println!("║     NO FLOATING POINT - PURE ALGEBRAIC                                       ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();
    
    // ========================================================================
    // PHASE 1: Exact Integer Target Search (α⁻¹ ≈ 137)
    // ========================================================================
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PHASE 1: EXACT INTEGER POLYNOMIAL SEARCH");
    println!("Target: P(D) = 137 (exact integer)");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    let target_137 = Rational::from_i128(137);
    let mut exact_matches: Vec<([i128; 5], i128)> = Vec::new();
    
    // Exhaustive search: monic quartic with small coefficients
    for a3 in -10i128..=10 {
        for a2 in -10i128..=10 {
            for a1 in -10i128..=10 {
                for a0 in -50i128..=50 {
                    let coeffs = [1, a3, a2, a1, a0];
                    
                    for d in 1i128..=20 {
                        if poly_equals_target(&coeffs, d, &target_137) {
                            exact_matches.push((coeffs, d));
                        }
                    }
                }
            }
        }
    }
    
    println!("Found {} EXACT solutions: P(D) = 137", exact_matches.len());
    println!();
    println!("First 20 solutions:");
    for (i, (coeffs, d)) in exact_matches.iter().take(20).enumerate() {
        let poly_str = format_polynomial(coeffs);
        let result = eval_polynomial(coeffs, *d);
        println!("  {}. {} at D={} → {}", i+1, poly_str, d, result);
    }
    
    // ========================================================================
    // PHASE 2: Verify P(D) = D^4 - 2D^3 + 2D + 1
    // ========================================================================
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PHASE 2: VERIFY CANONICAL POLYNOMIAL");
    println!("P(D) = D⁴ - 2D³ + 2D + 1");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    let canonical = [1i128, -2, 0, 2, 1];
    println!("Polynomial: {}", format_polynomial(&canonical));
    println!();
    println!("Evaluation table (EXACT Rational):");
    println!("{:>4} | {:>15} | Integer?", "D", "P(D)");
    println!("{:-<4}-+-{:-<15}-+-{:-<8}", "", "", "");
    
    for d in 1i128..=12 {
        let result = eval_polynomial(&canonical, d);
        let is_int = if result.is_integer() { "YES" } else { "no" };
        println!("{:>4} | {:>15} | {}", d, result, is_int);
    }
    
    // Verify P(4) = 137 exactly
    let p4 = eval_polynomial(&canonical, 4);
    println!();
    println!("VERIFICATION: P(4) = {}", p4);
    println!("P(4) == 137: {}", p4 == target_137);
    
    // ========================================================================
    // PHASE 3: E7 Group Theory Connection (EXACT)
    // ========================================================================
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PHASE 3: E7 GROUP THEORY CONNECTION (ALGEBRAIC)");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    let e7 = dim_e7();
    let four = Rational::from_i128(4);
    let e7_plus_4 = e7 + four;
    
    println!("dim(E7) = {}", e7);
    println!("dim(E7) + 4 = {}", e7_plus_4);
    println!();
    println!("EXACT equality: dim(E7) + 4 = 137: {}", e7_plus_4 == target_137);
    println!();
    println!("Interpretation:");
    println!("  α⁻¹ ≈ 137 = dim(E7) + D where D = 4 (spacetime dimension)");
    
    // ========================================================================
    // PHASE 4: Multi-Constant Search (Algebraic)
    // ========================================================================
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PHASE 4: MULTI-CONSTANT POLYNOMIAL SEARCH (ALGEBRAIC)");
    println!("Tolerance: 1% relative error (expressed as rational 1/100)");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    let constants: Vec<(&str, Rational)> = vec![
        ("α⁻¹ (integer)", alpha_inv_integer()),
        ("mμ/me", mu_me_ratio()),
        ("mp/me", mp_me_ratio()),
        ("mτ/me", mtau_me_ratio()),
    ];
    
    let mut multi_matches: Vec<([i128; 5], Vec<(i128, String)>)> = Vec::new();
    
    // Search non-monic quartics (optimized range)
    for a4 in [-2i128, -1, 1, 2] {
        for a3 in -5i128..=5 {
            for a2 in -5i128..=5 {
                for a1 in -10i128..=10 {
                    for a0 in -50i128..=50 {
                        let coeffs = [a4, a3, a2, a1, a0];
                        let mut matches_for_poly: Vec<(i128, String)> = Vec::new();
                        
                        for d in 1i128..=15 {
                            for (name, target) in &constants {
                                // 1% tolerance = 1/100
                                if poly_within_tolerance(&coeffs, d, target, 1, 100) {
                                    matches_for_poly.push((d, name.to_string()));
                                }
                            }
                        }
                        
                        // Only keep if matches 2+ distinct constants
                        let unique_constants: std::collections::HashSet<_> = 
                            matches_for_poly.iter().map(|(_, n)| n.clone()).collect();
                        
                        if unique_constants.len() >= 2 {
                            multi_matches.push((coeffs, matches_for_poly));
                        }
                    }
                }
            }
        }
    }
    
    // Sort by number of matches
    multi_matches.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    println!("Found {} polynomials matching 2+ constants (1% tolerance)", multi_matches.len());
    println!();
    println!("Top 15 multi-constant polynomials:");
    for (i, (coeffs, matches)) in multi_matches.iter().take(15).enumerate() {
        let poly_str = format_polynomial(coeffs);
        println!("  {}. {}", i+1, poly_str);
        for (d, name) in matches {
            let val = eval_polynomial(coeffs, *d);
            println!("       P({}) = {} ≈ {}", d, val, name);
        }
        println!();
    }
    
    // ========================================================================
    // PHASE 5: Use Fate Engine for Pattern Detection
    // ========================================================================
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PHASE 5: FATE ENGINE PATTERN DETECTION (EPR)");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    // Create Fate with KnowledgeBase
    let kb = KnowledgeBase::new();
    let mut fate = Fate::new();
    
    // Feed polynomial sequence to Fate
    let canonical_poly = [1i128, -2, 0, 2, 1];
    let poly_values: Vec<EPR> = (1i128..=20)
        .map(|d| {
            let val = eval_polynomial(&canonical_poly, d);
            EPR::from_ratio(val.numerator, val.denominator)
        })
        .collect();
    
    println!("Feeding P(D) = D⁴ - 2D³ + 2D + 1 sequence to Fate...");
    println!("Sequence (D=1..20):");
    for (i, val) in poly_values.iter().enumerate() {
        print!("  P({}) = {}", i+1, val);
        if (i + 1) % 5 == 0 { println!(); }
    }
    println!();
    
    // Use Fate's explore method with EPR signal
    let result = fate.explore(&poly_values, &kb);
    
    println!();
    println!("Fate analysis result:");
    println!("  Status: {:?}", result.status);
    println!("  Is Discovery: {}", result.is_discovery);
    if let Some(pattern) = &result.detected_pattern {
        println!("  Detected Pattern: {}", pattern);
    }
    if let Some(formula) = &result.formula {
        println!("  Formula: {}", formula);
    }
    if let Some(fit_level) = &result.fit_level {
        println!("  Fit Level: {:?}", fit_level);
    }
    println!("  Derivation Steps:");
    for step in &result.derivation_steps {
        println!("    - {}", step);
    }
    
    // ========================================================================
    // PHASE 6: Inter-Constant Relations (ALGEBRAIC)
    // ========================================================================
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PHASE 6: INTER-CONSTANT ALGEBRAIC RELATIONS");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    // Check: sin²θW = 34/(α⁻¹ + 10) approximately
    let alpha = alpha_inv_rational();
    let ten = Rational::from_i128(10);
    let thirty_four = Rational::from_i128(34);
    
    let predicted_sin2 = thirty_four / (alpha + ten);
    let actual_sin2 = sin2_theta_w();
    
    let diff = if predicted_sin2 > actual_sin2 {
        predicted_sin2 - actual_sin2
    } else {
        actual_sin2 - predicted_sin2
    };
    
    println!("Testing: sin²θW = 34/(α⁻¹ + 10)");
    println!("  α⁻¹ = {}", alpha);
    println!("  Predicted sin²θW = 34/({} + 10) = {}", alpha, predicted_sin2);
    println!("  Actual sin²θW = {}", actual_sin2);
    println!("  Difference = {}", diff);
    
    // Test: MW/MZ relation
    println!();
    let mw_mz = mw_mz_ratio();
    let one = Rational::one();
    let mw_mz_sq = mw_mz * mw_mz;
    let one_minus_sin2 = one - actual_sin2;
    
    println!("Testing: (MW/MZ)² = 1 - sin²θW");
    println!("  (MW/MZ)² = {} × {} = {}", mw_mz, mw_mz, mw_mz_sq);
    println!("  1 - sin²θW = {}", one_minus_sin2);
    println!("  Difference = {}", if mw_mz_sq > one_minus_sin2 { mw_mz_sq - one_minus_sin2 } else { one_minus_sin2 - mw_mz_sq });
    
    // ========================================================================
    // FINAL SUMMARY
    // ========================================================================
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("FINAL SUMMARY");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("1. EXACT solutions for P(D) = 137:");
    println!("   Found {} monic quartic polynomials", exact_matches.len());
    println!("   Including: D⁴ - 2D³ + 2D + 1 at D = 4");
    println!();
    println!("2. E7 connection (ALGEBRAIC EXACT):");
    println!("   dim(E7) + 4 = 133 + 4 = 137 ✓");
    println!();
    println!("3. Multi-constant polynomials:");
    println!("   Found {} polynomials matching 2+ constants", multi_matches.len());
    println!();
    println!("4. ALL computations performed in EXACT Rational arithmetic");
    println!("   NO floating point approximations used");
    println!();
    println!("S-wind EPR core: FULLY UTILIZED");
}

/// Format polynomial as string
fn format_polynomial(coeffs: &[i128; 5]) -> String {
    let mut terms = Vec::new();
    
    if coeffs[0] != 0 {
        if coeffs[0] == 1 {
            terms.push("D⁴".to_string());
        } else if coeffs[0] == -1 {
            terms.push("-D⁴".to_string());
        } else {
            terms.push(format!("{}D⁴", coeffs[0]));
        }
    }
    
    if coeffs[1] != 0 {
        if coeffs[1] == 1 {
            terms.push("+D³".to_string());
        } else if coeffs[1] == -1 {
            terms.push("-D³".to_string());
        } else if coeffs[1] > 0 {
            terms.push(format!("+{}D³", coeffs[1]));
        } else {
            terms.push(format!("{}D³", coeffs[1]));
        }
    }
    
    if coeffs[2] != 0 {
        if coeffs[2] == 1 {
            terms.push("+D²".to_string());
        } else if coeffs[2] == -1 {
            terms.push("-D²".to_string());
        } else if coeffs[2] > 0 {
            terms.push(format!("+{}D²", coeffs[2]));
        } else {
            terms.push(format!("{}D²", coeffs[2]));
        }
    }
    
    if coeffs[3] != 0 {
        if coeffs[3] == 1 {
            terms.push("+D".to_string());
        } else if coeffs[3] == -1 {
            terms.push("-D".to_string());
        } else if coeffs[3] > 0 {
            terms.push(format!("+{}D", coeffs[3]));
        } else {
            terms.push(format!("{}D", coeffs[3]));
        }
    }
    
    if coeffs[4] != 0 {
        if coeffs[4] > 0 {
            terms.push(format!("+{}", coeffs[4]));
        } else {
            terms.push(format!("{}", coeffs[4]));
        }
    }
    
    if terms.is_empty() {
        "0".to_string()
    } else {
        let result = terms.join("");
        if result.starts_with('+') {
            result[1..].to_string()
        } else {
            result
        }
    }
}
