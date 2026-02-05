//! S-wind π-e 大一统深度搜索
//! 
//! 基于发现: mt/me = 18α⁻² + 222π, mH/me = 13α⁻² + 249e
//! 搜索形如: m/me = a*α⁻ⁿ + b*π + c*e + d 的统一结构

use std::fs::File;
use std::io::Write;

const PI: f64 = std::f64::consts::PI;
const E: f64 = std::f64::consts::E;
const SQRT2: f64 = std::f64::consts::SQRT_2;
const PHI: f64 = 1.618033988749895; // 黄金比
const LN2: f64 = 0.6931471805599453;

// 精确物理常数
const ALPHA_INV: f64 = 137.035999177;
const ALPHA_INV_SQ: f64 = 18778.863712;

fn main() {
    println!("================================================================================");
    println!("     S-WIND π-e UNIFIED DEEP SEARCH");
    println!("     π-e 大一统深度搜索");
    println!("================================================================================\n");
    
    let mut output: Vec<String> = Vec::new();
    
    output.push("# S-wind π-e Unified Deep Search Report\n".into());
    output.push("## 基于发现的关键模式:\n".into());
    output.push("- mt/me = 18α⁻² + 222π".into());
    output.push("- mH/me = 13α⁻² + 249e\n".into());
    
    // 目标常数
    let targets = [
        ("α⁻¹", 137.035999177),
        ("mμ/me", 206.7682827),
        ("mτ/me", 3477.23),
        ("mp/me", 1836.15267343),
        ("mn/me", 1838.68366173),
        ("mW/me", 157298.8),
        ("mZ/me", 178450.3),
        ("mH/me", 244802.0),
        ("mt/me", 338717.0),
        ("sin²θ_W", 0.23122),
        ("Ω_Λ", 0.6889),
    ];
    
    let tolerance = 0.0001; // 0.0001% 严格阈值
    
    // ========================================================================
    // 搜索形如 a*α⁻¹ + b*π + c*e + d 的表达式
    // ========================================================================
    output.push("\n================================================================================".into());
    output.push("## Form: a*α⁻¹ + b*π + c*e + d".into());
    output.push("================================================================================\n".into());
    
    for (name, target) in &targets {
        if *target > 10.0 { // 只搜索大值
            let results = search_alpha_pi_e(*target, tolerance);
            if !results.is_empty() {
                output.push(format!("\n### {} = {:.10}", name, target));
                for (i, (formula, value, error)) in results.iter().take(5).enumerate() {
                    output.push(format!("{}. {} = {:.10} (error: {:.8}%)", i+1, formula, value, error));
                }
            }
        }
    }
    
    // ========================================================================
    // 搜索形如 a*α⁻² + b*π + c*e + d 的表达式
    // ========================================================================
    output.push("\n================================================================================".into());
    output.push("## Form: a*α⁻² + b*π + c*e + d".into());
    output.push("================================================================================\n".into());
    
    for (name, target) in &targets {
        if *target > 1000.0 { // 只搜索大值
            let results = search_alpha_sq_pi_e(*target, tolerance);
            if !results.is_empty() {
                output.push(format!("\n### {} = {:.10}", name, target));
                for (i, (formula, value, error)) in results.iter().take(5).enumerate() {
                    output.push(format!("{}. {} = {:.10} (error: {:.8}%)", i+1, formula, value, error));
                }
            }
        }
    }
    
    // ========================================================================
    // 搜索形如 a*π^n + b*e^m + c 的纯超越数表达式
    // ========================================================================
    output.push("\n================================================================================".into());
    output.push("## Form: Pure Transcendental (π and e powers)".into());
    output.push("================================================================================\n".into());
    
    for (name, target) in &targets {
        let results = search_pure_transcendental(*target, tolerance * 10.0);
        if !results.is_empty() {
            output.push(format!("\n### {} = {:.10}", name, target));
            for (i, (formula, value, error)) in results.iter().take(5).enumerate() {
                output.push(format!("{}. {} = {:.10} (error: {:.8}%)", i+1, formula, value, error));
            }
        }
    }
    
    // ========================================================================
    // 搜索 α⁻¹ 的精确超越数表达式
    // ========================================================================
    output.push("\n================================================================================".into());
    output.push("## Special: Exact α⁻¹ expressions".into());
    output.push("================================================================================\n".into());
    
    let alpha_results = search_alpha_inv_exact(0.00001);
    for (formula, value, error) in &alpha_results {
        output.push(format!("{} = {:.12} (error: {:.10}%)", formula, value, error));
    }
    
    // ========================================================================
    // 搜索质量比之间的统一关系
    // ========================================================================
    output.push("\n================================================================================".into());
    output.push("## Unified Mass Ratio Relations".into());
    output.push("================================================================================\n".into());
    
    let mass_ratios = [
        ("mμ/me", 206.7682827),
        ("mτ/me", 3477.23),
        ("mp/me", 1836.15267343),
        ("mW/me", 157298.8),
        ("mZ/me", 178450.3),
        ("mH/me", 244802.0),
        ("mt/me", 338717.0),
    ];
    
    // 搜索质量比之间的简单关系
    output.push("\n### Ratios between mass ratios:".into());
    for i in 0..mass_ratios.len() {
        for j in (i+1)..mass_ratios.len() {
            let ratio = mass_ratios[j].1 / mass_ratios[i].1;
            let results = find_simple_ratio(ratio, 0.001);
            if !results.is_empty() {
                output.push(format!("\n{} / {} = {:.6}:", mass_ratios[j].0, mass_ratios[i].0, ratio));
                for (formula, value, error) in results.iter().take(3) {
                    output.push(format!("  {} = {:.6} (error: {:.6}%)", formula, value, error));
                }
            }
        }
    }
    
    // ========================================================================
    // 寻找黄金比φ相关公式
    // ========================================================================
    output.push("\n================================================================================".into());
    output.push("## Golden Ratio (φ) Relations".into());
    output.push("================================================================================\n".into());
    
    for (name, target) in &targets {
        let results = search_phi_relations(*target, tolerance * 100.0);
        if !results.is_empty() {
            output.push(format!("\n### {} = {:.10}", name, target));
            for (i, (formula, value, error)) in results.iter().take(3).enumerate() {
                output.push(format!("{}. {} = {:.10} (error: {:.8}%)", i+1, formula, value, error));
            }
        }
    }
    
    // 输出
    let report = output.join("\n");
    println!("{}", report);
    
    let mut file = File::create("/home/jouly/AI/Valhalla/s-wind-sim/output/pi_e_unified_deep.txt")
        .expect("Cannot create file");
    file.write_all(report.as_bytes()).expect("Cannot write");
    
    println!("\n================================================================================");
    println!("Report saved to: output/pi_e_unified_deep.txt");
    println!("================================================================================");
}

/// 搜索 a*α⁻¹ + b*π + c*e + d
fn search_alpha_pi_e(target: f64, tolerance: f64) -> Vec<(String, f64, f64)> {
    let mut results = Vec::new();
    
    for a in -50i64..=50 {
        for b in -500i64..=500 {
            for c in -500i64..=500 {
                for d in -500i64..=500 {
                    if a == 0 && b == 0 && c == 0 { continue; }
                    
                    let computed = a as f64 * ALPHA_INV 
                        + b as f64 * PI 
                        + c as f64 * E 
                        + d as f64;
                    let error = ((computed - target) / target).abs() * 100.0;
                    
                    if error < tolerance {
                        let formula = format!("{}α⁻¹ + {}π + {}e + {}", a, b, c, d);
                        results.push((formula, computed, error));
                    }
                }
            }
        }
    }
    
    results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    results.dedup_by(|a, b| (a.1 - b.1).abs() < 1e-6);
    results
}

/// 搜索 a*α⁻² + b*π + c*e + d
fn search_alpha_sq_pi_e(target: f64, tolerance: f64) -> Vec<(String, f64, f64)> {
    let mut results = Vec::new();
    
    for a in -30i64..=30 {
        for b in -500i64..=500 {
            for c in -500i64..=500 {
                for d in -1000i64..=1000 {
                    if a == 0 && b == 0 && c == 0 { continue; }
                    
                    let computed = a as f64 * ALPHA_INV_SQ 
                        + b as f64 * PI 
                        + c as f64 * E 
                        + d as f64;
                    let error = ((computed - target) / target).abs() * 100.0;
                    
                    if error < tolerance {
                        let formula = format!("{}α⁻² + {}π + {}e + {}", a, b, c, d);
                        results.push((formula, computed, error));
                    }
                }
            }
        }
    }
    
    results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    results.dedup_by(|a, b| (a.1 - b.1).abs() < 1e-6);
    results
}

/// 搜索纯超越数表达式
fn search_pure_transcendental(target: f64, tolerance: f64) -> Vec<(String, f64, f64)> {
    let mut results = Vec::new();
    
    // a*π^n + b*e^m + c
    for a in -100i64..=100 {
        for n in 1i32..=5 {
            for b in -100i64..=100 {
                for m in 0i32..=5 {
                    for c in -1000i64..=1000 {
                        if a == 0 && b == 0 { continue; }
                        
                        let computed = a as f64 * PI.powi(n) 
                            + b as f64 * E.powi(m) 
                            + c as f64;
                        let error = ((computed - target) / target).abs() * 100.0;
                        
                        if error < tolerance && (n > 1 || m > 1) {
                            let formula = format!("{}π^{} + {}e^{} + {}", a, n, b, m, c);
                            results.push((formula, computed, error));
                        }
                    }
                }
            }
        }
    }
    
    results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    results.dedup_by(|a, b| (a.1 - b.1).abs() < 1e-6);
    results
}

/// 搜索α⁻¹的精确表达式
fn search_alpha_inv_exact(tolerance: f64) -> Vec<(String, f64, f64)> {
    let mut results = Vec::new();
    let target = ALPHA_INV;
    
    // 形如 a + b*π + c*π² + d*π³
    for a in 100i64..=150 {
        for b in -50i64..=50 {
            for c in -20i64..=20 {
                for d in -10i64..=10 {
                    let computed = a as f64 + b as f64 * PI + c as f64 * PI * PI + d as f64 * PI.powi(3);
                    let error = ((computed - target) / target).abs() * 100.0;
                    
                    if error < tolerance && (b != 0 || c != 0 || d != 0) {
                        let formula = format!("{} + {}π + {}π² + {}π³", a, b, c, d);
                        results.push((formula, computed, error));
                    }
                }
            }
        }
    }
    
    // 形如 a + b*e + c*e²
    for a in 100i64..=150 {
        for b in -100i64..=100 {
            for c in -50i64..=50 {
                let computed = a as f64 + b as f64 * E + c as f64 * E * E;
                let error = ((computed - target) / target).abs() * 100.0;
                
                if error < tolerance && (b != 0 || c != 0) {
                    let formula = format!("{} + {}e + {}e²", a, b, c);
                    results.push((formula, computed, error));
                }
            }
        }
    }
    
    // 形如 (a + b*π) / (c + d*π)
    for a in -200i64..=200 {
        for b in -100i64..=100 {
            for c in 1i64..=20 {
                for d in -20i64..=20 {
                    let numer = a as f64 + b as f64 * PI;
                    let denom = c as f64 + d as f64 * PI;
                    if denom.abs() < 0.001 { continue; }
                    
                    let computed = numer / denom;
                    let error = ((computed - target) / target).abs() * 100.0;
                    
                    if error < tolerance && (b != 0 || d != 0) {
                        let formula = format!("({} + {}π) / ({} + {}π)", a, b, c, d);
                        results.push((formula, computed, error));
                    }
                }
            }
        }
    }
    
    // 形如 a*√(b + c*π)
    for a in 1i64..=50 {
        for b in 1i64..=500 {
            for c in -100i64..=100 {
                if c == 0 { continue; }
                let inner = b as f64 + c as f64 * PI;
                if inner < 0.0 { continue; }
                
                let computed = a as f64 * inner.sqrt();
                let error = ((computed - target) / target).abs() * 100.0;
                
                if error < tolerance {
                    let formula = format!("{}√({} + {}π)", a, b, c);
                    results.push((formula, computed, error));
                }
            }
        }
    }
    
    results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    results.dedup_by(|a, b| (a.1 - b.1).abs() < 1e-10);
    results
}

/// 寻找简单有理数和无理数比例
fn find_simple_ratio(target: f64, tolerance: f64) -> Vec<(String, f64, f64)> {
    let mut results = Vec::new();
    
    // 简单有理数 a/b
    for a in 1i64..=1000 {
        for b in 1i64..=1000 {
            let ratio = a as f64 / b as f64;
            let error = ((ratio - target) / target).abs() * 100.0;
            
            if error < tolerance {
                let formula = format!("{}/{}", a, b);
                results.push((formula, ratio, error));
            }
        }
    }
    
    // a*π/b
    for a in 1i64..=500 {
        for b in 1i64..=500 {
            let ratio = a as f64 * PI / b as f64;
            let error = ((ratio - target) / target).abs() * 100.0;
            
            if error < tolerance {
                let formula = format!("{}π/{}", a, b);
                results.push((formula, ratio, error));
            }
        }
    }
    
    // a*φ/b (黄金比)
    for a in 1i64..=500 {
        for b in 1i64..=500 {
            let ratio = a as f64 * PHI / b as f64;
            let error = ((ratio - target) / target).abs() * 100.0;
            
            if error < tolerance {
                let formula = format!("{}φ/{}", a, b);
                results.push((formula, ratio, error));
            }
        }
    }
    
    results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    results.dedup_by(|a, b| (a.1 - b.1).abs() < 1e-6);
    results
}

/// 搜索黄金比相关公式
fn search_phi_relations(target: f64, tolerance: f64) -> Vec<(String, f64, f64)> {
    let mut results = Vec::new();
    
    // a + b*φ + c*φ²
    for a in -1000i64..=1000 {
        for b in -500i64..=500 {
            for c in -200i64..=200 {
                if b == 0 && c == 0 { continue; }
                
                let computed = a as f64 + b as f64 * PHI + c as f64 * PHI * PHI;
                let error = ((computed - target) / target).abs() * 100.0;
                
                if error < tolerance {
                    let formula = format!("{} + {}φ + {}φ²", a, b, c);
                    results.push((formula, computed, error));
                }
            }
        }
    }
    
    // a*α⁻¹ + b*φ
    for a in -50i64..=50 {
        for b in -500i64..=500 {
            if a == 0 && b == 0 { continue; }
            
            let computed = a as f64 * ALPHA_INV + b as f64 * PHI;
            let error = ((computed - target) / target).abs() * 100.0;
            
            if error < tolerance {
                let formula = format!("{}α⁻¹ + {}φ", a, b);
                results.push((formula, computed, error));
            }
        }
    }
    
    results.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    results.dedup_by(|a, b| (a.1 - b.1).abs() < 1e-6);
    results
}
