//! S-wind 电弱粒子深度搜索
//! 
//! 专门搜索 W, Z, Higgs, Top 质量的精确代数表达式
//! 使用有理数系数和 α² 关系

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// ============================================================================
// 基础多项式 P(D) = D⁴ - 2D³ + 2D + 1
// ============================================================================

fn p(d: i128) -> i128 {
    let d2 = d * d;
    let d3 = d2 * d;
    let d4 = d3 * d;
    d4 - 2 * d3 + 2 * d + 1
}

// ============================================================================
// 精确有理数
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rat {
    n: i128,
    d: i128,
}

impl Rat {
    fn new(n: i128, d: i128) -> Self {
        if d == 0 { panic!("Zero denominator"); }
        let g = gcd(n.abs(), d.abs());
        let sign = if (n < 0) != (d < 0) { -1 } else { 1 };
        Rat { n: sign * n.abs() / g, d: d.abs() / g }
    }
    
    fn from_int(n: i128) -> Self { Rat { n, d: 1 } }
    fn is_int(&self) -> bool { self.d == 1 }
}

impl std::fmt::Display for Rat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.d == 1 { write!(f, "{}", self.n) }
        else { write!(f, "{}/{}", self.n, self.d) }
    }
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 { a.max(1) } else { gcd(b, a % b) }
}

// ============================================================================
// 物理常数
// ============================================================================

const ALPHA_INV: i128 = 137;
const ALPHA_INV_SQ: i128 = 137 * 137;  // 18769

// 质量比 (整数近似)
const M_MU_ME: i128 = 207;
const M_TAU_ME: i128 = 3477;
const M_P_ME: i128 = 1836;
const M_N_ME: i128 = 1839;
const M_W_ME: i128 = 157299;   // 80.377 GeV / 0.511 MeV
const M_Z_ME: i128 = 178450;   // 91.188 GeV / 0.511 MeV  
const M_H_ME: i128 = 244954;   // 125.1 GeV / 0.511 MeV
const M_T_ME: i128 = 338962;   // 173.1 GeV / 0.511 MeV

// ============================================================================
// 搜索策略
// ============================================================================

struct ExactRelation {
    formula: String,
    latex: String,
    target: i128,
    target_name: String,
    components: Vec<(String, i128)>,
}

/// 搜索形如 a*α⁻¹ + b*α⁻² + c*P(x) + d*P(y) = target 的关系
fn search_alpha_polynomial_relations(target: i128, target_name: &str) -> Vec<ExactRelation> {
    let mut results = Vec::new();
    
    // 搜索 a*α⁻² + b*P(x) = target
    for a in 0i128..=20 {
        let term_alpha = a * ALPHA_INV_SQ;
        if term_alpha > target { break; }
        
        let remainder = target - term_alpha;
        
        for x in -20i128..=20 {
            let p_x = p(x);
            if p_x == 0 { continue; }
            
            if remainder % p_x == 0 {
                let b = remainder / p_x;
                if b.abs() <= 500 {
                    results.push(ExactRelation {
                        formula: format!("{}*α⁻² + {}*P({}) = {}", a, b, x, target_name),
                        latex: format!("{}\\alpha^{{-2}} + {}P({}) = {}", a, b, x, target_name),
                        target,
                        target_name: target_name.into(),
                        components: vec![
                            (format!("{}*α⁻²", a), term_alpha),
                            (format!("{}*P({})", b, x), b * p_x),
                        ],
                    });
                }
            }
        }
    }
    
    // 搜索 a*α⁻¹ + b*α⁻² + c*P(x) = target
    for a in 0i128..=30 {
        let term_a = a * ALPHA_INV;
        if term_a > target { continue; }
        
        for b in 0i128..=20 {
            let term_b = b * ALPHA_INV_SQ;
            let sum_ab = term_a + term_b;
            if sum_ab > target { break; }
            
            let remainder = target - sum_ab;
            
            for x in -20i128..=20 {
                let p_x = p(x);
                if p_x == 0 { continue; }
                
                if remainder % p_x == 0 {
                    let c = remainder / p_x;
                    if c.abs() <= 500 && (a > 0 || b > 0) {
                        results.push(ExactRelation {
                            formula: format!("{}*α⁻¹ + {}*α⁻² + {}*P({}) = {}", a, b, c, x, target_name),
                            latex: format!("{}\\alpha^{{-1}} + {}\\alpha^{{-2}} + {}P({}) = {}", a, b, c, x, target_name),
                            target,
                            target_name: target_name.into(),
                            components: vec![
                                (format!("{}*α⁻¹", a), term_a),
                                (format!("{}*α⁻²", b), term_b),
                                (format!("{}*P({})", c, x), c * p_x),
                            ],
                        });
                    }
                }
            }
        }
    }
    
    results
}

/// 搜索双P组合 a*P(x) + b*P(y) = target
fn search_double_p_relations(target: i128, target_name: &str) -> Vec<ExactRelation> {
    let mut results = Vec::new();
    
    for a in -100i128..=100 {
        if a == 0 { continue; }
        for x in -15i128..=15 {
            let term_a = a * p(x);
            
            for b in -100i128..=100 {
                for y in -15i128..=15 {
                    if x == y && a + b == 0 { continue; }
                    
                    let term_b = b * p(y);
                    
                    if term_a + term_b == target {
                        results.push(ExactRelation {
                            formula: format!("{}*P({}) + {}*P({}) = {}", a, x, b, y, target_name),
                            latex: format!("{}P({}) + {}P({}) = {}", a, x, b, y, target_name),
                            target,
                            target_name: target_name.into(),
                            components: vec![
                                (format!("{}*P({})", a, x), term_a),
                                (format!("{}*P({})", b, y), term_b),
                            ],
                        });
                    }
                }
            }
        }
    }
    
    results
}

/// 搜索三P组合
fn search_triple_p_relations(target: i128, target_name: &str) -> Vec<ExactRelation> {
    let mut results = Vec::new();
    
    for a in -30i128..=30 {
        if a == 0 { continue; }
        for x in -10i128..=10 {
            let term_a = a * p(x);
            
            for b in -30i128..=30 {
                for y in -10i128..=10 {
                    let term_b = b * p(y);
                    let sum_ab = term_a + term_b;
                    
                    for c in -30i128..=30 {
                        for z in -10i128..=10 {
                            let term_c = c * p(z);
                            
                            if sum_ab + term_c == target {
                                // 过滤平凡解
                                if (a == 0 && b == 0) || (b == 0 && c == 0) { continue; }
                                
                                results.push(ExactRelation {
                                    formula: format!("{}*P({}) + {}*P({}) + {}*P({}) = {}", 
                                        a, x, b, y, c, z, target_name),
                                    latex: format!("{}P({}) + {}P({}) + {}P({}) = {}", 
                                        a, x, b, y, c, z, target_name),
                                    target,
                                    target_name: target_name.into(),
                                    components: vec![
                                        (format!("{}*P({})", a, x), term_a),
                                        (format!("{}*P({})", b, y), term_b),
                                        (format!("{}*P({})", c, z), term_c),
                                    ],
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    
    results
}

/// 搜索有理数系数 (p/q)*P(x) = target
fn search_rational_p_relations(target: i128, target_name: &str) -> Vec<ExactRelation> {
    let mut results = Vec::new();
    
    for q in 1i128..=20 {
        for p_coeff in 1i128..=500 {
            for x in -20i128..=20 {
                let p_x = p(x);
                // (p/q) * P(x) = target
                // p * P(x) = target * q
                if p_coeff * p_x == target * q {
                    let rat = Rat::new(p_coeff, q);
                    results.push(ExactRelation {
                        formula: format!("({})*P({}) = {}", rat, x, target_name),
                        latex: format!("\\frac{{{}}}{{{}}}P({}) = {}", rat.n, rat.d, x, target_name),
                        target,
                        target_name: target_name.into(),
                        components: vec![
                            (format!("({})*P({})", rat, x), target),
                        ],
                    });
                }
            }
        }
    }
    
    results
}

/// 主函数
fn main() {
    println!("================================================================================");
    println!("     S-WIND ELECTROWEAK DEEP SEARCH");
    println!("     电弱粒子精确公式深度搜索");
    println!("================================================================================\n");
    
    let mut output: Vec<String> = Vec::new();
    
    output.push("# S-wind Electroweak Deep Search Report\n".into());
    output.push("## 电弱粒子精确公式深度搜索报告\n".into());
    output.push("Base polynomial: P(D) = D⁴ - 2D³ + 2D + 1\n".into());
    
    // P(D) 值表
    output.push("### P(D) Values\n".into());
    output.push("| D | P(D) |".into());
    output.push("|---|------|".into());
    for d in -10i128..=15 {
        output.push(format!("| {} | {} |", d, p(d)));
    }
    
    // 定义目标
    let targets = [
        ("α⁻¹", ALPHA_INV),
        ("mμ/me", M_MU_ME),
        ("mτ/me", M_TAU_ME),
        ("mp/me", M_P_ME),
        ("mn/me", M_N_ME),
        ("mW/me", M_W_ME),
        ("mZ/me", M_Z_ME),
        ("mH/me", M_H_ME),
        ("mt/me", M_T_ME),
    ];
    
    let mut all_results: HashMap<String, Vec<ExactRelation>> = HashMap::new();
    
    for (name, target) in &targets {
        output.push(format!("\n## {} = {}\n", name, target));
        
        let mut found = Vec::new();
        
        // 1. α + α² + P 组合搜索
        println!("Searching α+α²+P relations for {} = {}...", name, target);
        let alpha_results = search_alpha_polynomial_relations(*target, name);
        for r in &alpha_results {
            if found.len() < 20 {
                output.push(format!("  {}", r.formula));
            }
        }
        if !alpha_results.is_empty() {
            output.push(format!("  ... total {} α-polynomial relations", alpha_results.len()));
            found.extend(alpha_results);
        }
        
        // 2. 双P组合搜索
        println!("Searching double-P relations for {} = {}...", name, target);
        let double_p = search_double_p_relations(*target, name);
        let unique_double: Vec<_> = double_p.iter()
            .filter(|r| !r.formula.contains("0*P"))
            .take(10)
            .collect();
        for r in &unique_double {
            output.push(format!("  {}", r.formula));
        }
        if !double_p.is_empty() {
            output.push(format!("  ... total {} double-P relations", double_p.len()));
            found.extend(double_p);
        }
        
        // 3. 有理数系数搜索
        println!("Searching rational relations for {} = {}...", name, target);
        let rational = search_rational_p_relations(*target, name);
        for r in rational.iter().take(10) {
            output.push(format!("  {}", r.formula));
        }
        if !rational.is_empty() {
            output.push(format!("  ... total {} rational relations", rational.len()));
            found.extend(rational);
        }
        
        // 4. 三P组合（仅对较大的常数）
        if *target > 10000 {
            println!("Searching triple-P relations for {} = {}...", name, target);
            let triple_p = search_triple_p_relations(*target, name);
            for r in triple_p.iter().take(5) {
                output.push(format!("  {}", r.formula));
            }
            if !triple_p.is_empty() {
                output.push(format!("  ... total {} triple-P relations", triple_p.len()));
                found.extend(triple_p);
            }
        }
        
        output.push(format!("\n  **Total for {}: {} exact relations**\n", name, found.len()));
        all_results.insert(name.to_string(), found);
    }
    
    // 统计摘要
    output.push("\n================================================================================".into());
    output.push("                         SUMMARY".into());
    output.push("================================================================================\n".into());
    
    output.push("| Constant | Total Relations | Best Form |".into());
    output.push("|----------|-----------------|-----------|".into());
    
    for (name, _target) in &targets {
        if let Some(relations) = all_results.get(*name) {
            let best = relations.first().map(|r| r.formula.clone()).unwrap_or("-".into());
            output.push(format!("| {} | {} | {} |", name, relations.len(), best));
        }
    }
    
    // 输出关键发现
    output.push("\n## Key Discoveries (LaTeX)\n".into());
    
    for (name, _target) in &targets {
        if let Some(relations) = all_results.get(*name) {
            if let Some(r) = relations.first() {
                output.push(format!("- ${}$", r.latex));
            }
        }
    }
    
    // 保存报告
    let report = output.join("\n");
    println!("\n{}", report);
    
    let mut file = File::create("/home/jouly/AI/Valhalla/s-wind-sim/output/electroweak_deep_search.txt")
        .expect("Cannot create file");
    file.write_all(report.as_bytes()).expect("Cannot write");
    
    println!("\n================================================================================");
    println!("Report saved to: output/electroweak_deep_search.txt");
    println!("================================================================================");
}
