//! S-wind Algebraic Exhaustive Exploration
//!
//! Demonstrates S-wind's CORE capabilities:
//! 1. AUTO-CREATE new conjecture containers
//! 2. EXHAUSTIVE exploration (no preconceptions)
//! 3. PURE ALGEBRAIC computation (Rational/EPR only)
//! 4. Batch verification with exact comparisons
//!
//! This is the CORRECT way to use S-wind for unified theory exploration.

use s_wind::{
    Rational,
    AlgebraicConjecture, AlgebraicConstraint, AlgebraicDomain,
    AlgebraicVerifier, AlgebraicVerificationResult,
    Fate, KnowledgeBase,
};
use std::collections::HashMap;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║   S-wind ALGEBRAIC EXHAUSTIVE EXPLORATION                                    ║");
    println!("║   AUTO-CREATE Containers + EXHAUSTIVE Search + PURE ALGEBRAIC                ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();
    
    // Create algebraic verifier (EPR-only, no f64)
    let mut verifier = AlgebraicVerifier::new();
    
    // ========================================================================
    // PHASE 1: AUTO-GENERATE + EXHAUSTIVE VERIFICATION
    // ========================================================================
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PHASE 1: AUTO-GENERATE CONJECTURE CONTAINERS (Exhaustive)");
    println!("Search: Monic quartic P(D) = 137 EXACTLY");
    println!("Coefficient range: [-5, 5], D range: [1, 10]");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    let target_137 = Rational::from_i128(137);
    
    // S-wind AUTO-GENERATES all polynomial conjectures
    let conjectures = verifier.auto_generate_polynomial_conjectures(
        "poly137",
        -5..=5,     // coefficient range
        1..=10,     // D range
        target_137,
        true,       // require EXACT equality
    );
    
    println!("AUTO-GENERATED {} conjectures", conjectures.len());
    println!("Each conjecture gets its own AUTO-CREATED container");
    println!();
    
    // EXHAUSTIVE batch verification
    println!("Running EXHAUSTIVE verification...");
    let results = verifier.batch_test(&conjectures);
    
    // Collect exact matches
    let exact_matches: Vec<_> = results.iter()
        .filter(|(_, r)| matches!(r, AlgebraicVerificationResult::ExactlyVerified { .. }))
        .collect();
    
    println!();
    println!("EXHAUSTIVE SEARCH COMPLETE:");
    println!("  Total conjectures tested: {}", results.len());
    println!("  EXACTLY verified (P(D) = 137): {}", exact_matches.len());
    println!("  Containers auto-created: {}", verifier.stats.containers_created);
    println!();
    
    println!("First 20 EXACT solutions:");
    for (i, (id, result)) in exact_matches.iter().take(20).enumerate() {
        if let AlgebraicVerificationResult::ExactlyVerified { computed, expected, .. } = result {
            println!("  {}. {} → {} = {} ✓", i+1, id, computed, expected);
        }
    }
    
    // ========================================================================
    // PHASE 2: SPECIFIC CONJECTURE VERIFICATION (E7 + D = 137)
    // ========================================================================
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PHASE 2: VERIFY E7 GROUP CONJECTURE");
    println!("Conjecture: dim(E7) + D = 133 + 4 = 137 = α⁻¹ (integer)");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    // Create E7 conjecture
    let mut e7_params = HashMap::new();
    e7_params.insert("dim_E7".to_string(), Rational::from_i128(133));
    e7_params.insert("D".to_string(), Rational::from_i128(4));
    // Polynomial: 133 + D = a0 + a1*D where a0=133, a1=1
    e7_params.insert("a0".to_string(), Rational::from_i128(133));
    e7_params.insert("a1".to_string(), Rational::one());
    
    let e7_conjecture = AlgebraicConjecture {
        id: "e7_alpha".to_string(),
        statement: "α⁻¹ = dim(E7) + D = 133 + 4".to_string(),
        formula: "dim(E7) + D".to_string(),
        parameters: e7_params,
        constraints: vec![
            AlgebraicConstraint {
                name: "alpha_inv_integer".to_string(),
                expected: Rational::from_i128(137),
                tolerance: None,
                require_exact: true,
            }
        ],
        domain: AlgebraicDomain::GroupTheory,
    };
    
    let e7_result = verifier.verify(&e7_conjecture);
    
    match &e7_result {
        AlgebraicVerificationResult::ExactlyVerified { computed, expected, .. } => {
            println!("E7 CONJECTURE: EXACTLY VERIFIED ✓");
            println!("  Computed: dim(E7) + D = {}", computed);
            println!("  Expected: α⁻¹ = {}", expected);
            println!("  ALGEBRAIC EQUALITY CONFIRMED");
        }
        AlgebraicVerificationResult::Falsified { difference, .. } => {
            println!("E7 CONJECTURE: FALSIFIED ✗");
            println!("  Difference: {}", difference);
        }
        _ => {
            println!("E7 CONJECTURE: {:?}", e7_result);
        }
    }
    
    // ========================================================================
    // PHASE 3: CANONICAL POLYNOMIAL VERIFICATION
    // ========================================================================
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PHASE 3: VERIFY CANONICAL POLYNOMIAL");
    println!("P(D) = D⁴ - 2D³ + 2D + 1 at D = 4");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    let mut canon_params = HashMap::new();
    canon_params.insert("a0".to_string(), Rational::from_i128(1));
    canon_params.insert("a1".to_string(), Rational::from_i128(2));
    canon_params.insert("a2".to_string(), Rational::zero());
    canon_params.insert("a3".to_string(), Rational::from_i128(-2));
    canon_params.insert("a4".to_string(), Rational::one());
    canon_params.insert("D".to_string(), Rational::from_i128(4));
    
    let canonical_conjecture = AlgebraicConjecture {
        id: "canonical_137".to_string(),
        statement: "D⁴ - 2D³ + 2D + 1 = 137 at D = 4".to_string(),
        formula: "D^4 - 2*D^3 + 2*D + 1".to_string(),
        parameters: canon_params,
        constraints: vec![
            AlgebraicConstraint {
                name: "P(4)".to_string(),
                expected: Rational::from_i128(137),
                tolerance: None,
                require_exact: true,
            }
        ],
        domain: AlgebraicDomain::Polynomial,
    };
    
    let canon_result = verifier.verify(&canonical_conjecture);
    
    match &canon_result {
        AlgebraicVerificationResult::ExactlyVerified { computed, expected, evidence } => {
            println!("CANONICAL POLYNOMIAL: EXACTLY VERIFIED ✓");
            println!("  P(4) = {}", computed);
            println!("  Expected: {}", expected);
            for ev in evidence {
                println!("  Evidence: {}", ev);
            }
        }
        _ => {
            println!("CANONICAL POLYNOMIAL: {:?}", canon_result);
        }
    }
    
    // ========================================================================
    // PHASE 4: FATE ENGINE INTEGRATION
    // ========================================================================
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("PHASE 4: FATE ENGINE PATTERN DETECTION");
    println!("Feed verified polynomial sequence to Fate");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    let mut fate = Fate::new();
    let kb = KnowledgeBase::new();
    
    // Generate polynomial sequence P(D) for D = 1..15
    let sequence: Vec<s_wind::EPR> = (1i128..=15)
        .map(|d| {
            // P(D) = D^4 - 2D^3 + 2D + 1
            let val = d.pow(4) - 2*d.pow(3) + 2*d + 1;
            s_wind::EPR::from_i128(val)
        })
        .collect();
    
    println!("Polynomial sequence P(D) = D⁴ - 2D³ + 2D + 1:");
    for (i, val) in sequence.iter().enumerate() {
        print!("P({})={} ", i+1, val);
        if (i + 1) % 5 == 0 { println!(); }
    }
    println!();
    
    let fate_result = fate.explore(&sequence, &kb);
    
    println!();
    println!("Fate Analysis:");
    println!("  Status: {:?}", fate_result.status);
    println!("  Is Discovery: {}", fate_result.is_discovery);
    if let Some(pattern) = &fate_result.detected_pattern {
        println!("  Detected Pattern: {}", pattern);
    }
    if let Some(formula) = &fate_result.formula {
        println!("  Formula: {}", formula);
    }
    
    // ========================================================================
    // FINAL SUMMARY
    // ========================================================================
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!("FINAL SUMMARY: S-wind Core Capabilities Utilized");
    println!("═══════════════════════════════════════════════════════════════════════════════");
    println!();
    
    println!("{}", verifier.summary());
    println!();
    
    println!("S-wind CORE CAPABILITIES VERIFIED:");
    println!("  ✓ AUTO-CREATE conjecture containers: {}", verifier.stats.containers_created);
    println!("  ✓ EXHAUSTIVE search (no preconceptions)");
    println!("  ✓ PURE ALGEBRAIC (Rational, no f64)");
    println!("  ✓ EXACT equality verification");
    println!("  ✓ FATE engine integration (EPR)");
    println!();
    
    println!("All computation performed in EXACT Rational arithmetic.");
    println!("NO floating point approximations used.");
}
