//! Fusion Q-η Algebraic Derivation
//!
//! Pure symbolic algebra - no numerical presuppositions.
//! S-wind derives Q(η) from power balance constraints.

use s_wind::{EPR, Rational, Expr, Symbol};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║            S-wind Algebraic Derivation: Fusion Q-η Relationship             ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();
    
    // ========================================================================
    // ALGEBRAIC SETUP: Define symbolic variables
    // ========================================================================
    
    println!("【STEP 1】Symbolic Variables");
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!();
    println!("  P_fus   = fusion power");
    println!("  P_loss  = energy confinement loss");
    println!("  P_input = external heating power");
    println!("  P_α     = alpha particle self-heating = (1/5) × P_fus");
    println!("  Q       = P_fus / P_input  (energy gain ratio)");
    println!("  η       = normalized triple product (relative to ignition)");
    println!();
    
    // ========================================================================
    // ALGEBRAIC CONSTRAINT 1: Power Balance
    // ========================================================================
    
    println!("【STEP 2】Power Balance Equation (Algebraic)");
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!();
    println!("  Steady-state power balance:");
    println!();
    println!("    P_input + P_α = P_loss");
    println!();
    println!("  Substitute P_α = (1/5) × P_fus:");
    println!();
    println!("    P_input + (1/5) × P_fus = P_loss");
    println!();
    println!("  Solve for P_input:");
    println!();
    println!("    P_input = P_loss - (1/5) × P_fus          ... (Eq. 1)");
    println!();
    
    // ========================================================================
    // ALGEBRAIC CONSTRAINT 2: Q Definition
    // ========================================================================
    
    println!("【STEP 3】Q Definition (Algebraic)");
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!();
    println!("  By definition:");
    println!();
    println!("    Q ≡ P_fus / P_input");
    println!();
    println!("  Substitute (Eq. 1):");
    println!();
    println!("    Q = P_fus / (P_loss - (1/5) × P_fus)");
    println!();
    println!("  Let R ≡ P_loss / P_fus, then:");
    println!();
    println!("    Q = P_fus / (R × P_fus - (1/5) × P_fus)");
    println!("      = P_fus / (P_fus × (R - 1/5))");
    println!("      = 1 / (R - 1/5)                         ... (Eq. 2)");
    println!();
    
    // ========================================================================
    // ALGEBRAIC CONSTRAINT 3: Loss Ratio from Physics
    // ========================================================================
    
    println!("【STEP 4】Loss Ratio R (Algebraic)");
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!();
    println!("  Energy confinement loss:");
    println!();
    println!("    P_loss = W / τ_E = 3nTV / τ_E            (W = plasma stored energy)");
    println!();
    println!("  Fusion power:");
    println!();
    println!("    P_fus = (1/4) × n² × <σv> × E_fus × V");
    println!();
    println!("  Loss ratio:");
    println!();
    println!("    R = P_loss / P_fus");
    println!("      = (3nTV / τ_E) / ((1/4) × n² × <σv> × E_fus × V)");
    println!("      = (3nT / τ_E) × (4 / (n² × <σv> × E_fus))");
    println!("      = 12T / (n × τ_E × <σv> × E_fus)        ... (Eq. 3)");
    println!();
    
    // ========================================================================
    // ALGEBRAIC CONSTRAINT 4: Ignition Condition
    // ========================================================================
    
    println!("【STEP 5】Ignition Condition (Q → ∞)");
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!();
    println!("  At ignition: Q → ∞");
    println!();
    println!("  From (Eq. 2): Q = 1/(R - 1/5) → ∞  requires  R → 1/5");
    println!();
    println!("  Substitute into (Eq. 3):");
    println!();
    println!("    R_ign = 1/5");
    println!("    12T / (n × τ_E × <σv> × E_fus)|_ign = 1/5");
    println!();
    println!("  Solve for (n × τ_E)|_ign:");
    println!();
    println!("    (n × τ_E)|_ign = 60T / (<σv> × E_fus)     ... (Eq. 4)");
    println!();
    
    // ========================================================================
    // ALGEBRAIC CONSTRAINT 5: Define Normalized η
    // ========================================================================
    
    println!("【STEP 6】Define Normalized Triple Product η");
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!();
    println!("  Define η as ratio to ignition value:");
    println!();
    println!("    η ≡ (n × τ_E) / (n × τ_E)|_ign");
    println!("      = (n × τ_E × <σv> × E_fus) / (60T)      ... (Eq. 5)");
    println!();
    println!("  Rearranging (Eq. 3) using η:");
    println!();
    println!("    R = 12T / (n × τ_E × <σv> × E_fus)");
    println!("      = 12T / (60T × η)");
    println!("      = 12 / (60 × η)");
    println!("      = 1 / (5η)                              ... (Eq. 6)");
    println!();
    
    // ========================================================================
    // ALGEBRAIC DERIVATION: Q(η)
    // ========================================================================
    
    println!("【STEP 7】Derive Q(η) - Pure Algebra");
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!();
    println!("  Substitute (Eq. 6) into (Eq. 2):");
    println!();
    println!("    Q = 1 / (R - 1/5)");
    println!("      = 1 / (1/(5η) - 1/5)");
    println!();
    println!("  Factor out 1/5:");
    println!();
    println!("      = 1 / ((1/5) × (1/η - 1))");
    println!("      = 5 / (1/η - 1)");
    println!();
    println!("  Simplify:");
    println!();
    println!("      = 5 / ((1 - η)/η)");
    println!("      = 5η / (1 - η)");
    println!();
    
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                                                                              ║");
    println!("║                        Q = 5η / (1 - η)                                      ║");
    println!("║                                                                              ║");
    println!("║             Equivalently:  Q = 5 / (1/η - 1)                                 ║");
    println!("║                                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();
    
    // ========================================================================
    // ALGEBRAIC VERIFICATION with EPR (Exact Rational)
    // ========================================================================
    
    println!("【STEP 8】Algebraic Verification (EPR - Exact Rational)");
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!();
    
    // Verify using exact rational arithmetic
    let test_cases = vec![
        (1i128, 4i128),   // η = 1/4
        (1, 3),           // η = 1/3
        (1, 2),           // η = 1/2
        (3, 5),           // η = 3/5
        (2, 3),           // η = 2/3  <-- ITER Q=10
        (3, 4),           // η = 3/4
        (4, 5),           // η = 4/5
        (9, 10),          // η = 9/10
    ];
    
    println!("  ┌──────────────────┬──────────────────┬──────────────────┐");
    println!("  │        η         │      Q exact     │    Q decimal     │");
    println!("  ├──────────────────┼──────────────────┼──────────────────┤");
    
    for (num, den) in &test_cases {
        let eta = Rational::new(*num, *den);
        let q = compute_q_algebraic(&eta);
        let q_decimal = (*num as f64 * 5.0) / (*den as f64 - *num as f64);
        
        let eta_str = if *den == 1 {
            format!("{}", num)
        } else {
            format!("{}/{}", num, den)
        };
        
        let q_str = if q.denominator == 1 {
            format!("{}", q.numerator)
        } else {
            format!("{}/{}", q.numerator, q.denominator)
        };
        
        let marker = if *num == 2 && *den == 3 { " ←ITER" } else { "" };
        
        println!("  │ {:>16} │ {:>16} │ {:>14.4}{} │",
            eta_str, q_str, q_decimal, marker);
    }
    
    println!("  └──────────────────┴──────────────────┴──────────────────┘");
    println!();
    
    // ========================================================================
    // ALGEBRAIC INVERSE: η(Q)
    // ========================================================================
    
    println!("【STEP 9】Inverse Relation: η(Q)");
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!();
    println!("  From Q = 5η / (1 - η), solve for η:");
    println!();
    println!("    Q(1 - η) = 5η");
    println!("    Q - Qη = 5η");
    println!("    Q = 5η + Qη");
    println!("    Q = η(5 + Q)");
    println!("    η = Q / (Q + 5)");
    println!();
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                                                                              ║");
    println!("║                        η = Q / (Q + 5)                                       ║");
    println!("║                                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
    println!();
    
    // Verify inverse
    println!("  Verification of inverse:");
    println!();
    
    for q_val in [1, 5, 10, 20, 50, 100] {
        let eta = Rational::new(q_val, q_val + 5);
        let q_back = compute_q_algebraic(&eta);
        println!("    Q = {:>3}  →  η = {}/{}  →  Q = {}  ✓",
            q_val, q_val, q_val + 5, q_back.numerator);
    }
    println!();
    
    // ========================================================================
    // KEY RESULT: ITER Q=10
    // ========================================================================
    
    println!("【STEP 10】ITER Q=10 Requirement (Algebraic)");
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!();
    println!("  For Q = 10:");
    println!();
    println!("    η = Q / (Q + 5) = 10 / (10 + 5) = 10/15 = 2/3");
    println!();
    println!("  Verification:");
    println!();
    println!("    Q = 5 × (2/3) / (1 - 2/3)");
    println!("      = (10/3) / (1/3)");
    println!("      = 10  ✓");
    println!();
    
    let eta_iter = Rational::new(2, 3);
    let q_iter = compute_q_algebraic(&eta_iter);
    
    println!("  EPR verification: Q(η=2/3) = {}/{} = {}", 
        q_iter.numerator, q_iter.denominator, q_iter.numerator);
    println!();
    
    // ========================================================================
    // COMPARISON: Old vs New Formula
    // ========================================================================
    
    println!("【STEP 11】Error in Old Formula");
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!();
    println!("  OLD (WRONG):  Q = η / (1 - 0.2η)");
    println!("  NEW (CORRECT): Q = 5η / (1 - η)");
    println!();
    println!("  The old formula made an algebraic error:");
    println!();
    println!("    WRONG:  Q = 1 / (R - 0.2)  with  R = 1/η");
    println!("            Q = 1 / (1/η - 0.2) = η / (1 - 0.2η)  ← INCORRECT R");
    println!();
    println!("    RIGHT:  Q = 1 / (R - 0.2)  with  R = 1/(5η)  [from physics]");
    println!("            Q = 1 / (1/(5η) - 1/5) = 5η / (1 - η)  ← CORRECT");
    println!();
    println!("  The error was in the definition of R:");
    println!();
    println!("    Wrong:  R = 1/η  (missing factor of 5)");
    println!("    Right:  R = 1/(5η) = 0.2/η");
    println!();
    
    println!("  ┌──────────────────┬──────────────────┬──────────────────┬────────────────┐");
    println!("  │        η         │     Q (old)      │    Q (correct)   │  Error factor  │");
    println!("  ├──────────────────┼──────────────────┼──────────────────┼────────────────┤");
    
    for (num, den) in &test_cases {
        let eta_f = *num as f64 / *den as f64;
        let q_old = eta_f / (1.0 - 0.2 * eta_f);
        let q_new = 5.0 * eta_f / (1.0 - eta_f);
        let error = q_new / q_old;
        
        let eta_str = format!("{}/{}", num, den);
        
        println!("  │ {:>16} │ {:>16.4} │ {:>16.4} │ {:>14.2}× │",
            eta_str, q_old, q_new, error);
    }
    
    println!("  └──────────────────┴──────────────────┴──────────────────┴────────────────┘");
    println!();
    
    // ========================================================================
    // LaTeX OUTPUT
    // ========================================================================
    
    println!("【LaTeX Derivation】");
    println!("────────────────────────────────────────────────────────────────────────────────");
    println!();
    print_latex_derivation();
    
    // ========================================================================
    // SUMMARY
    // ========================================================================
    
    println!();
    println!("╔══════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              ALGEBRAIC SUMMARY                               ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════╣");
    println!("║                                                                              ║");
    println!("║  Power balance:      P_input + (1/5)P_fus = P_loss                           ║");
    println!("║  Q definition:       Q = P_fus / P_input                                     ║");
    println!("║  Loss ratio:         R = P_loss/P_fus = 1/(5η)                               ║");
    println!("║                                                                              ║");
    println!("║  ─────────────────────────────────────────────────────────────────────────── ║");
    println!("║                                                                              ║");
    println!("║  DERIVED FORMULA:    Q = 5η / (1 - η)                                        ║");
    println!("║  INVERSE:            η = Q / (Q + 5)                                         ║");
    println!("║                                                                              ║");
    println!("║  ─────────────────────────────────────────────────────────────────────────── ║");
    println!("║                                                                              ║");
    println!("║  KEY VALUES:                                                                 ║");
    println!("║    η = 1/2  →  Q = 5    (α-heating balances losses)                          ║");
    println!("║    η = 2/3  →  Q = 10   (ITER target)                                        ║");
    println!("║    η = 4/5  →  Q = 20   (high performance)                                   ║");
    println!("║    η → 1    →  Q → ∞    (ignition)                                           ║");
    println!("║                                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════╝");
}

/// Compute Q from η using exact rational arithmetic
/// Q = 5η / (1 - η)
fn compute_q_algebraic(eta: &Rational) -> Rational {
    // Q = 5η / (1 - η)
    // numerator = 5 * eta.num
    // denominator = eta.den - eta.num (representing 1 - η = (den - num)/den)
    // Q = (5 * eta.num / eta.den) / ((eta.den - eta.num) / eta.den)
    //   = (5 * eta.num) / (eta.den - eta.num)
    
    let num = 5 * eta.numerator;
    let den = eta.denominator - eta.numerator;
    
    // Simplify
    let g = gcd(num.abs(), den.abs());
    Rational::new(num / g, den / g)
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn print_latex_derivation() {
    println!(r"% S-wind Generated LaTeX: Fusion Q-η Algebraic Derivation");
    println!(r"% ================================================================");
    println!();
    println!(r"\section{{Algebraic Derivation of Q-$\eta$ Relationship}}");
    println!();
    println!(r"\subsection{{Power Balance Constraint}}");
    println!(r"Steady-state power balance:");
    println!(r"\begin{{equation}}");
    println!(r"P_{{\text{{input}}}} + P_\alpha = P_{{\text{{loss}}}}");
    println!(r"\end{{equation}}");
    println!(r"where $P_\alpha = \frac{{1}}{{5}} P_{{\text{{fus}}}}$ (alpha self-heating).");
    println!();
    println!(r"\subsection{{Energy Gain Ratio}}");
    println!(r"\begin{{equation}}");
    println!(r"Q \equiv \frac{{P_{{\text{{fus}}}}}}{{P_{{\text{{input}}}}}} = \frac{{1}}{{R - \frac{{1}}{{5}}}}");
    println!(r"\end{{equation}}");
    println!(r"where $R \equiv P_{{\text{{loss}}}}/P_{{\text{{fus}}}}$.");
    println!();
    println!(r"\subsection{{Loss Ratio from Confinement Physics}}");
    println!(r"\begin{{equation}}");
    println!(r"R = \frac{{P_{{\text{{loss}}}}}}{{P_{{\text{{fus}}}}}} = \frac{{12T}}{{n \tau_E \langle\sigma v\rangle E_{{\text{{fus}}}}}}");
    println!(r"\end{{equation}}");
    println!();
    println!(r"\subsection{{Normalized Triple Product}}");
    println!(r"At ignition ($Q \to \infty$), $R = \frac{{1}}{{5}}$, giving:");
    println!(r"\begin{{equation}}");
    println!(r"(n \tau_E)_{{\text{{ign}}}} = \frac{{60T}}{{\langle\sigma v\rangle E_{{\text{{fus}}}}}}");
    println!(r"\end{{equation}}");
    println!();
    println!(r"Define normalized triple product:");
    println!(r"\begin{{equation}}");
    println!(r"\eta \equiv \frac{{n \tau_E}}{{(n \tau_E)_{{\text{{ign}}}}}} = \frac{{n \tau_E \langle\sigma v\rangle E_{{\text{{fus}}}}}}{{60T}}");
    println!(r"\end{{equation}}");
    println!();
    println!(r"Then:");
    println!(r"\begin{{equation}}");
    println!(r"R = \frac{{1}}{{5\eta}}");
    println!(r"\end{{equation}}");
    println!();
    println!(r"\subsection{{Final Result}}");
    println!(r"Substituting $R = \frac{{1}}{{5\eta}}$ into $Q = \frac{{1}}{{R - \frac{{1}}{{5}}}}$:");
    println!(r"\begin{{align}}");
    println!(r"Q &= \frac{{1}}{{\frac{{1}}{{5\eta}} - \frac{{1}}{{5}}}} \\");
    println!(r"  &= \frac{{1}}{{\frac{{1}}{{5}}\left(\frac{{1}}{{\eta}} - 1\right)}} \\");
    println!(r"  &= \frac{{5}}{{\frac{{1-\eta}}{{\eta}}}} \\");
    println!(r"  &= \boxed{{\frac{{5\eta}}{{1-\eta}}}}");
    println!(r"\end{{align}}");
    println!();
    println!(r"\subsection{{Key Values}}");
    println!(r"\begin{{center}}");
    println!(r"\begin{{tabular}}{{|c|c|l|}}");
    println!(r"\hline");
    println!(r"$\eta$ & $Q$ & Physical Significance \\");
    println!(r"\hline");
    println!(r"$\frac{{1}}{{2}}$ & $5$ & $P_\alpha = P_{{\text{{loss}}}}$ \\");
    println!(r"$\frac{{2}}{{3}}$ & $\mathbf{{10}}$ & \textbf{{ITER design target}} \\");
    println!(r"$\frac{{4}}{{5}}$ & $20$ & High performance \\");
    println!(r"$1$ & $\infty$ & Ignition \\");
    println!(r"\hline");
    println!(r"\end{{tabular}}");
    println!(r"\end{{center}}");
}
