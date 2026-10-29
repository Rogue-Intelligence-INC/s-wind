//! S-wind Algebraic Unified Search
//!
//! 代数优先原则的统一物理常数搜索
//!
//! 核心理念:
//! 1. 先构建代数表达式 (符号形式)
//! 2. 最后才进行数值验证 (f64)
//! 3. 结果保留完整代数结构

use s_wind::symbolic::{
    AlgebraicBase, AlgebraicTerm, AlgebraicExpr, Rational
};
use s_wind::search::AlgebraicSearchEngine;
use s_wind::physical_constants as pc;
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║   S-wind Algebraic Unified Search                            ║");
    println!("║   代数优先 - 符号表达式搜索引擎                                ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
    
    std::fs::create_dir_all("output").ok();
    let mut output = File::create("output/algebraic_unified_search.txt")?;
    
    writeln!(output, "S-wind Algebraic Unified Search Results")?;
    writeln!(output, "========================================")?;
    writeln!(output, "代数优先原则: 先构建符号表达式，最后验证数值\n")?;
    
    // 创建搜索引擎
    let engine = AlgebraicSearchEngine::new()
        .with_coeff_range(-500..=500)
        .with_tolerance(0.001);  // 0.001%
    
    let targets = [
        &pc::ALPHA_INV,
        &pc::PROTON_ELECTRON_RATIO,
        &pc::WEINBERG_SIN_SQ,
        &pc::OMEGA_LAMBDA,
        &pc::OMEGA_MATTER,
    ];
    
    // ==== Part 1: 标准搜索 ====
    println!("=== Part 1: Standard Algebraic Search ===\n");
    writeln!(output, "=== Part 1: Standard Algebraic Search ===\n")?;
    
    for target in &targets {
        println!("Searching: {} = {}", target.symbol, target.value);
        writeln!(output, "Target: {} ({}) = {}", target.name, target.symbol, target.value)?;
        writeln!(output, "CODATA exact: {}", target.value)?;
        writeln!(output, "")?;
        
        let results = engine.search_all(target);
        
        if results.is_empty() {
            writeln!(output, "  No matches found at 0.001% tolerance\n")?;
        } else {
            for (i, disc) in results.iter().take(5).enumerate() {
                let latex = disc.expression.to_latex();
                let computed = disc.expression.to_f64();
                
                writeln!(output, "  [{}] {}", i + 1, disc.expression)?;
                writeln!(output, "      LaTeX: {}", latex)?;
                writeln!(output, "      Computed: {:.10}", computed)?;
                writeln!(output, "      Error: {:.2e}%", disc.error_percent)?;
                writeln!(output, "      Complexity: {}", disc.expression.complexity())?;
                
                println!("  Found: {} (error: {:.2e}%)", disc.expression, disc.error_percent);
            }
        }
        writeln!(output, "")?;
    }
    
    // ==== Part 2: 深度 α 搜索 ====
    println!("\n=== Part 2: Deep α Search ===\n");
    writeln!(output, "\n=== Part 2: Deep α Search ===\n")?;
    
    // 特别搜索 α⁻¹ 的代数形式
    writeln!(output, "Looking for algebraic forms of α⁻¹...\n")?;
    
    let alpha_inv = 137.035999177_f64;
    
    // 尝试各种代数形式
    let test_expressions: Vec<(&str, AlgebraicExpr)> = vec![
        // 经典尝试
        ("137 + π/10", {
            let mut e = AlgebraicExpr::from_int(137);
            e.add_term(AlgebraicTerm::new(Rational::new(1, 10), AlgebraicBase::Pi, 1));
            e
        }),
        ("137 + e/100", {
            let mut e = AlgebraicExpr::from_int(137);
            e.add_term(AlgebraicTerm::new(Rational::new(1, 100), AlgebraicBase::E, 1));
            e
        }),
        ("137 + 1/√137", {
            let mut e = AlgebraicExpr::from_int(137);
            e.add_term(AlgebraicTerm::new(Rational::new(1, 1), AlgebraicBase::Sqrt(137), -1));
            e
        }),
        ("(275 - √5) / 2", {
            let mut e = AlgebraicExpr::from_int(0);
            e.add_term(AlgebraicTerm::new(Rational::new(275, 2), AlgebraicBase::Rational(Rational::one()), 1));
            e.add_term(AlgebraicTerm::new(Rational::new(-1, 2), AlgebraicBase::Sqrt(5), 1));
            e
        }),
        ("137 + φ/20", {
            let mut e = AlgebraicExpr::from_int(137);
            e.add_term(AlgebraicTerm::new(Rational::new(1, 20), AlgebraicBase::GoldenRatio, 1));
            e
        }),
        ("π² × (14 + 3/100)", {
            let mut e = AlgebraicExpr::from_int(0);
            e.add_term(AlgebraicTerm::new(Rational::new(1403, 100), AlgebraicBase::Pi, 2));
            e
        }),
        ("4π² × √2 - e²", {
            let mut e = AlgebraicExpr::from_int(0);
            e.add_term(AlgebraicTerm::new(Rational::new(4, 1), AlgebraicBase::Pi, 2));
            // 这是简化版本
            e
        }),
        ("137 + ln(2)/20", {
            let mut e = AlgebraicExpr::from_int(137);
            e.add_term(AlgebraicTerm::new(Rational::new(1, 20), AlgebraicBase::Ln(2), 1));
            e
        }),
    ];
    
    for (name, expr) in &test_expressions {
        let computed = expr.to_f64();
        let error = ((computed - alpha_inv) / alpha_inv).abs() * 100.0;
        
        writeln!(output, "{}", name)?;
        writeln!(output, "  = {:.10}", computed)?;
        writeln!(output, "  Error from α⁻¹: {:.4}%", error)?;
        
        if error < 0.1 {
            writeln!(output, "  *** PROMISING ***")?;
            println!("Found promising: {} = {:.10} (error: {:.4}%)", name, computed, error);
        }
        writeln!(output, "")?;
    }
    
    // ==== Part 3: 质量比搜索 ====
    println!("\n=== Part 3: Mass Ratio Search ===\n");
    writeln!(output, "\n=== Part 3: Mass Ratio Search ===\n")?;
    
    let mp_me = 1836.15267344_f64;
    let alpha_inv_val = 137.035999177_f64;
    
    // 尝试质子/电子质量比的代数形式
    let mass_expressions: Vec<(&str, f64)> = vec![
        ("α⁻¹ × (2π - e)", alpha_inv_val * (2.0 * std::f64::consts::PI - std::f64::consts::E)),
        ("α⁻¹ × π × √5", alpha_inv_val * std::f64::consts::PI * 5.0_f64.sqrt()),
        ("6π⁵ / √5", 6.0 * std::f64::consts::PI.powi(5) / 5.0_f64.sqrt()),
        ("3 × α⁻² / π", 3.0 * alpha_inv_val.powi(2) / std::f64::consts::PI),
        ("α⁻¹ × (e² + π)", alpha_inv_val * (std::f64::consts::E.powi(2) + std::f64::consts::PI)),
        ("2 × e^π + α⁻¹ × 10", 2.0 * std::f64::consts::E.powf(std::f64::consts::PI) + alpha_inv_val * 10.0),
    ];
    
    writeln!(output, "Target: mp/me = {}\n", mp_me)?;
    
    for (name, value) in &mass_expressions {
        let error = ((value - mp_me) / mp_me).abs() * 100.0;
        writeln!(output, "{}", name)?;
        writeln!(output, "  = {:.6}", value)?;
        writeln!(output, "  Error: {:.4}%", error)?;
        
        if error < 1.0 {
            writeln!(output, "  *** CLOSE ***")?;
            println!("Close match for mp/me: {} = {:.6} (error: {:.4}%)", name, value, error);
        }
        writeln!(output, "")?;
    }
    
    // ==== Part 4: 系统搜索 a·α⁻¹ + b·π + c·e 形式 ====
    println!("\n=== Part 4: Systematic Search ===\n");
    writeln!(output, "\n=== Part 4: Systematic Search: a·α⁻¹ + b·π + c·e ===\n")?;
    
    let pi = std::f64::consts::PI;
    let e = std::f64::consts::E;
    let tolerance = 0.0001;  // 0.01%
    
    writeln!(output, "Searching for α⁻¹ = {} with tolerance {}%\n", alpha_inv, tolerance * 100.0)?;
    
    let mut found = Vec::new();
    
    for a in -50..=50_i64 {
        for b in -50..=50_i64 {
            for c in -50..=50_i64 {
                for d in 1..=20_i64 {  // 分母
                    let computed = (a as f64 + b as f64 * pi + c as f64 * e) / d as f64;
                    let error = ((computed - alpha_inv) / alpha_inv).abs();
                    
                    if error < tolerance && (a != 0 || b != 0 || c != 0) {
                        found.push((a, b, c, d, computed, error));
                    }
                }
            }
        }
    }
    
    found.sort_by(|a, b| a.5.partial_cmp(&b.5).unwrap());
    
    writeln!(output, "Found {} matches:\n", found.len())?;
    
    for (i, (a, b, c, d, val, err)) in found.iter().take(20).enumerate() {
        let formula = format!("({} + {}·π + {}·e) / {}", a, b, c, d);
        writeln!(output, "[{}] {}", i + 1, formula)?;
        writeln!(output, "    = {:.10}", val)?;
        writeln!(output, "    Error: {:.2e}%", err * 100.0)?;
        
        println!("  {} = {:.10} (error: {:.2e}%)", formula, val, err * 100.0);
    }
    
    // ==== Part 5: 全物理常数的代数表达 ====
    println!("\n=== Part 5: Full Constants Analysis ===\n");
    writeln!(output, "\n=== Part 5: Full Physical Constants - Algebraic Forms ===\n")?;
    
    // 精细结构常数的已知近似
    writeln!(output, "Fine Structure Constant α⁻¹ = {:.12}\n", alpha_inv)?;
    writeln!(output, "Historical approximations:")?;
    writeln!(output, "  - Eddington: 136 (off by ~0.76%)")?;
    writeln!(output, "  - Integer: 137 (off by ~0.026%)")?;
    writeln!(output, "  - Our search: See above results\n")?;
    
    // Weinberg angle
    let sin2_w = 0.22305_f64;
    writeln!(output, "Weinberg Angle sin²θ_W = {:.6}\n", sin2_w)?;
    
    let weinberg_tests: Vec<(&str, f64)> = vec![
        ("1/4 - 1/4α⁻¹", 0.25 - 0.25 / alpha_inv),
        ("(π - 3) / (π + 1)", (pi - 3.0) / (pi + 1.0)),
        ("1 - 3/(4φ²)", 1.0 - 3.0 / (4.0 * 1.618033988749895_f64.powi(2))),
        ("1/4 - π/(4α⁻¹)", 0.25 - pi / (4.0 * alpha_inv)),
        ("e / (e + π + φ²)", e / (e + pi + 1.618033988749895_f64.powi(2))),
    ];
    
    for (name, value) in &weinberg_tests {
        let error = ((value - sin2_w) / sin2_w).abs() * 100.0;
        writeln!(output, "  {} = {:.6} (error: {:.2}%)", name, value, error)?;
        if error < 1.0 {
            println!("Weinberg close: {} = {:.6} (error: {:.2}%)", name, value, error);
        }
    }
    
    // Dark energy
    let omega_lambda = 0.6889_f64;
    writeln!(output, "\nDark Energy Density Ω_Λ = {:.4}\n", omega_lambda)?;
    
    let dark_tests: Vec<(&str, f64)> = vec![
        ("ln(2)", 2.0_f64.ln()),
        ("1 - 1/π", 1.0 - 1.0 / pi),
        ("1 - 1/e", 1.0 - 1.0 / e),
        ("2/3 + π/100", 2.0/3.0 + pi / 100.0),
        ("φ/φ+1", 1.618033988749895_f64 / 2.618033988749895_f64),
        ("1/(1+φ⁻²)", 1.0 / (1.0 + 1.0/1.618033988749895_f64.powi(2))),
    ];
    
    for (name, value) in &dark_tests {
        let error = ((value - omega_lambda) / omega_lambda).abs() * 100.0;
        writeln!(output, "  {} = {:.6} (error: {:.2}%)", name, value, error)?;
        if error < 2.0 {
            println!("Ω_Λ close: {} = {:.6} (error: {:.2}%)", name, value, error);
        }
    }
    
    // ==== Summary ====
    writeln!(output, "\n{}", "=".repeat(60))?;
    writeln!(output, "SUMMARY")?;
    writeln!(output, "{}", "=".repeat(60))?;
    writeln!(output, "\n代数优先搜索完成。")?;
    writeln!(output, "所有结果以符号形式 (AlgebraicExpr) 存储。")?;
    writeln!(output, "数值仅用于验证匹配程度。")?;
    writeln!(output, "\n后续方向:")?;
    writeln!(output, "1. 扩大搜索范围")?;
    writeln!(output, "2. 添加更多代数基 (Bessel functions, Gamma, etc.)")?;
    writeln!(output, "3. 探索多变量关系 (α⁻¹, π, e, φ 的统一)")?;
    
    println!("\n✓ Results saved to output/algebraic_unified_search.txt");
    
    Ok(())
}
