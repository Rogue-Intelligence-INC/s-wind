//! S-wind Conjecture Explorer
//!
//! Demonstrates the auto-generation of physics experiment containers
//! to systematically test conjectures.

use s_wind::conjecture::{
    Conjecture, ConjectureVerifier, Constraint, Domain, 
    ExperimentContainer, PhysicsModel, VerificationResult,
};
use std::collections::HashMap;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║              S-wind Conjecture Explorer & Auto-Model Generator              ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();
    
    // ========================================================================
    // FEATURE 1: Reusable Experiment Containers
    // ========================================================================
    
    println!("【FEATURE 1】Reusable Experiment Containers");
    println!("════════════════════════════════════════════════════════════════════════════════");
    println!();
    
    // Create QED running coupling container
    let qed_container = ExperimentContainer::new(
        "QED_Running",
        PhysicsModel::QEDRunning {
            beta_coefficients: vec![4.0/3.0, 4.0, 20.0/9.0],
            thresholds: vec![
                ("m_e".to_string(), 0.000511),
                ("m_μ".to_string(), 0.1057),
                ("m_τ".to_string(), 1.777),
                ("m_c".to_string(), 1.27),
                ("m_b".to_string(), 4.18),
                ("m_t".to_string(), 173.0),
            ],
        }
    );
    
    // Create cosmology container
    let cosmo_container = ExperimentContainer::new(
        "Cosmology_ΛCDM",
        PhysicsModel::CosmologicalEvolution {
            hubble_constant: 67.4,
            omega_matter: 0.315,
            omega_lambda: 0.685,
        }
    );
    
    // Create algebraic container
    let algebra_container = ExperimentContainer::new(
        "Algebraic_D4",
        PhysicsModel::Algebraic {
            expressions: vec![
                "D^4 - 2*D^3 + 2*D + 1".to_string(),
                "D*(D+1)".to_string(),
                "(D^2+D+2)".to_string(),
            ],
        }
    );
    
    println!("  Created containers:");
    println!("    ✓ QED_Running (α running with thresholds)");
    println!("    ✓ Cosmology_ΛCDM (Hubble evolution)");
    println!("    ✓ Algebraic_D4 (polynomial identities)");
    println!();
    
    // ========================================================================
    // FEATURE 2: Unified Interface for Conjecture Testing
    // ========================================================================
    
    println!("【FEATURE 2】Unified Conjecture Testing Interface");
    println!("════════════════════════════════════════════════════════════════════════════════");
    println!();
    
    let mut verifier = ConjectureVerifier::new();
    
    // Register containers
    verifier.register_container(qed_container);
    verifier.register_container(cosmo_container);
    verifier.register_container(algebra_container);
    
    // Define conjectures to test
    
    // Conjecture 1: D=4 polynomial gives 137
    let conj_d4_137 = Conjecture {
        id: "D4_POLY_137".to_string(),
        statement: "D^4 - 2D^3 + 2D + 1 = 137 for D=4".to_string(),
        formula: Some("D^4 - 2D^3 + 2D + 1".to_string()),
        domain: Domain::PureMathematics,
        parameters: {
            let mut p = HashMap::new();
            p.insert("D".to_string(), 4.0);
            p
        },
        constraints: vec![
            Constraint {
                name: "result".to_string(),
                condition: "equals".to_string(),
                value: 137.0,
                tolerance: 0.001,
            }
        ],
    };
    
    // Conjecture 2: α⁻¹ = 137 at some scale
    let conj_alpha_137 = Conjecture {
        id: "ALPHA_INV_137".to_string(),
        statement: "∃ μ such that α⁻¹(μ) = 137 exactly".to_string(),
        formula: Some("α⁻¹(μ) = 137".to_string()),
        domain: Domain::QuantumElectrodynamics,
        parameters: {
            let mut p = HashMap::new();
            p.insert("alpha_inv_0".to_string(), 137.036);
            p.insert("mu_0".to_string(), 0.000511);
            p.insert("mu_target".to_string(), 0.0001); // Try below m_e
            p
        },
        constraints: vec![
            Constraint {
                name: "alpha_inv_target".to_string(),
                condition: "equals".to_string(),
                value: 137.0,
                tolerance: 0.001,
            }
        ],
    };
    
    // Conjecture 3: D=3 polynomial value
    let conj_d3 = Conjecture {
        id: "D3_POLY".to_string(),
        statement: "D^4 - 2D^3 + 2D + 1 for D=3".to_string(),
        formula: Some("D^4 - 2D^3 + 2D + 1".to_string()),
        domain: Domain::PureMathematics,
        parameters: {
            let mut p = HashMap::new();
            p.insert("D".to_string(), 3.0);
            p
        },
        constraints: vec![
            Constraint {
                name: "result".to_string(),
                condition: "equals".to_string(),
                value: 28.0, // 81 - 54 + 6 + 1 = 34, not 28
                tolerance: 0.001,
            }
        ],
    };
    
    println!("  Testing conjectures:");
    println!();
    
    // Test each conjecture
    let result1 = verifier.verify(&conj_d4_137);
    print_result("D4_POLY_137", &result1);
    
    let result2 = verifier.verify(&conj_alpha_137);
    print_result("ALPHA_INV_137", &result2);
    
    let result3 = verifier.verify(&conj_d3);
    print_result("D3_POLY", &result3);
    
    println!();
    
    // ========================================================================
    // FEATURE 3: Batch Testing with Auto-Generated Conjectures
    // ========================================================================
    
    println!("【FEATURE 3】Batch Testing & Auto-Generation");
    println!("════════════════════════════════════════════════════════════════════════════════");
    println!();
    
    // Auto-generate conjectures for different D values
    let base_conjecture = Conjecture {
        id: "D_POLY".to_string(),
        statement: "D^4 - 2D^3 + 2D + 1 evaluation".to_string(),
        formula: Some("D^4 - 2D^3 + 2D + 1".to_string()),
        domain: Domain::PureMathematics,
        parameters: HashMap::new(),
        constraints: vec![],
    };
    
    let variations = vec![
        ("D", vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]),
    ];
    
    let generated = verifier.generate_conjectures(&base_conjecture, &variations);
    
    println!("  Auto-generated {} conjectures for D = 2..8", generated.len());
    println!();
    
    println!("  ┌────────────┬────────────────────────────────┬────────────────────┐");
    println!("  │     D      │   D⁴ - 2D³ + 2D + 1            │   Near integer?    │");
    println!("  ├────────────┼────────────────────────────────┼────────────────────┤");
    
    for d in 2..=8 {
        let d_f = d as f64;
        let result = d_f.powi(4) - 2.0 * d_f.powi(3) + 2.0 * d_f + 1.0;
        let near_int = if (result - result.round()).abs() < 0.001 { "✓ exact" } else { "" };
        println!("  │ {:>10} │ {:>30.0} │ {:>18} │", d, result, near_int);
    }
    
    println!("  └────────────┴────────────────────────────────┴────────────────────┘");
    println!();
    
    // Show which D values give "interesting" results
    println!("  Interesting polynomial values:");
    for d in 2..=10 {
        let d_f = d as f64;
        let result = d_f.powi(4) - 2.0 * d_f.powi(3) + 2.0 * d_f + 1.0;
        
        // Check if result is a "special" number
        let special = check_special_number(result as i64);
        if !special.is_empty() {
            println!("    D={}: {} → {}", d, result as i64, special);
        }
    }
    println!();
    
    // ========================================================================
    // FEATURE 4: Auto-Generate New Physics Models
    // ========================================================================
    
    println!("【FEATURE 4】Auto-Generate New Physics Models from Discoveries");
    println!("════════════════════════════════════════════════════════════════════════════════");
    println!();
    
    println!("  Based on the discovery that D⁴ - 2D³ + 2D + 1 = 137 at D=4,");
    println!("  S-wind can auto-generate new physics model containers:");
    println!();
    
    // Generate new model ideas
    let new_models = vec![
        ("DimensionalCoupling", 
         "Model where α⁻¹ is determined by spacetime dimension D",
         "α⁻¹(D) = D⁴ - 2D³ + 2D + 1"),
        
        ("QuantizedScales",
         "Model where physical scales are quantized to polynomial values",
         "s_n = (5×4ⁿ + 46)/3"),
        
        ("RecurrencePhysics",
         "Model based on linear recurrence s_{n+1} = 4s_n - 46",
         "Predicts sequence: 22, 42, 122, 442, ..."),
         
        ("IntegerRunning",
         "Model where running crosses integer values at special scales",
         "Find μ* where α⁻¹(μ*) ∈ ℤ"),
    ];
    
    println!("  ┌─────────────────────────────────────────────────────────────────────────────┐");
    println!("  │                    AUTO-GENERATED PHYSICS MODELS                           │");
    println!("  ├─────────────────────────────────────────────────────────────────────────────┤");
    
    for (name, description, formula) in &new_models {
        println!("  │                                                                             │");
        println!("  │  Model: {:<67} │", name);
        println!("  │  Desc:  {:<67} │", description);
        println!("  │  Form:  {:<67} │", formula);
    }
    
    println!("  │                                                                             │");
    println!("  └─────────────────────────────────────────────────────────────────────────────┘");
    println!();
    
    // ========================================================================
    // DEMONSTRATION: Use New Model to Make Predictions
    // ========================================================================
    
    println!("【DEMONSTRATION】New Model Predictions");
    println!("════════════════════════════════════════════════════════════════════════════════");
    println!();
    
    println!("  Using 'DimensionalCoupling' model:");
    println!("    α⁻¹(D) = D⁴ - 2D³ + 2D + 1");
    println!();
    
    println!("  Predictions for other dimensions:");
    println!();
    
    for d in 2..=6 {
        let d_f = d as f64;
        let alpha_inv = d_f.powi(4) - 2.0 * d_f.powi(3) + 2.0 * d_f + 1.0;
        let marker = if d == 4 { " ← OUR UNIVERSE" } else { "" };
        println!("    D = {}:  α⁻¹ = {:.0}{}", d, alpha_inv, marker);
    }
    println!();
    
    println!("  Testable prediction:");
    println!("    IF spacetime dimension affects α, THEN string theory compactifications");
    println!("    with effective D ≠ 4 should show different α values.");
    println!();
    
    // ========================================================================
    // SUMMARY
    // ========================================================================
    
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              SYSTEM SUMMARY                                  ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════╣");
    println!("║                                                                              ║");
    println!("║  FEATURE 1: Reusable Experiment Containers                                   ║");
    println!("║    • QED running, Cosmology, Algebraic models                                ║");
    println!("║    • Plug-and-play physics computations                                      ║");
    println!("║                                                                              ║");
    println!("║  FEATURE 2: Unified Conjecture Interface                                     ║");
    println!("║    • Input: Conjecture (statement + constraints)                             ║");
    println!("║    • Output: Verified/Falsified + Evidence                                   ║");
    println!("║                                                                              ║");
    println!("║  FEATURE 3: Batch Testing & Auto-Generation                                  ║");
    println!("║    • Generate parameter variations automatically                             ║");
    println!("║    • Systematic exploration of conjecture space                              ║");
    println!("║                                                                              ║");
    println!("║  FEATURE 4: Auto-Generate New Physics Models                                 ║");
    println!("║    • Transform discoveries into testable models                              ║");
    println!("║    • Generate predictions for verification                                   ║");
    println!("║                                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();
    
    // Print verifier summary
    println!("{}", verifier.summary());
}

fn print_result(id: &str, result: &VerificationResult) {
    match result {
        VerificationResult::Verified { evidence, confidence } => {
            println!("    ✓ {} - VERIFIED (confidence: {:.0}%)", id, confidence * 100.0);
            for e in evidence {
                println!("      Evidence: {}", e);
            }
        }
        VerificationResult::Falsified { counter_evidence, confidence } => {
            println!("    ✗ {} - FALSIFIED (confidence: {:.0}%)", id, confidence * 100.0);
            for e in counter_evidence {
                println!("      Counter-evidence: {}", e);
            }
        }
        VerificationResult::Undecidable { reason } => {
            println!("    ? {} - UNDECIDABLE: {}", id, reason);
        }
        VerificationResult::RequiresNewPhysics { suggestion } => {
            println!("    ! {} - REQUIRES NEW PHYSICS: {}", id, suggestion);
        }
    }
}

fn check_special_number(n: i64) -> String {
    let mut specials = Vec::new();
    
    // Check if prime
    if is_prime(n) {
        specials.push("prime");
    }
    
    // Check known physics constants
    match n {
        137 => specials.push("≈ α⁻¹"),
        1836 | 1837 => specials.push("≈ m_p/m_e"),
        207 => specials.push("≈ m_μ/m_e"),
        _ => {}
    }
    
    // Check if perfect power
    if is_perfect_square(n) {
        specials.push("perfect square");
    }
    if is_perfect_cube(n) {
        specials.push("perfect cube");
    }
    
    // Check Fibonacci
    if is_fibonacci(n) {
        specials.push("Fibonacci");
    }
    
    specials.join(", ")
}

fn is_prime(n: i64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let sqrt_n = (n as f64).sqrt() as i64;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

fn is_perfect_square(n: i64) -> bool {
    let sqrt = (n as f64).sqrt() as i64;
    sqrt * sqrt == n
}

fn is_perfect_cube(n: i64) -> bool {
    let cbrt = (n as f64).cbrt().round() as i64;
    cbrt * cbrt * cbrt == n
}

fn is_fibonacci(n: i64) -> bool {
    // A number is Fibonacci if one of 5n²+4 or 5n²-4 is a perfect square
    let check1 = 5 * n * n + 4;
    let check2 = 5 * n * n - 4;
    is_perfect_square(check1) || is_perfect_square(check2)
}
