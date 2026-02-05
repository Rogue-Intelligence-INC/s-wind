//! S-wind 无理数大一统理论搜索
//! 
//! 支持：
//! - 精确物理常数（非整数）
//! - 无理数常数（π, e, √2, φ, √3, √5, ln2 等）
//! - 高精度有理数逼近
//! - 代数组合搜索

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// ============================================================================
// 高精度有理数 (支持大分母)
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
struct HighPrecRat {
    // 使用 i128 支持高精度
    num: i128,
    den: i128,
    // 符号表示（用于无理数）
    symbolic: Option<String>,
}

impl HighPrecRat {
    fn new(num: i128, den: i128) -> Self {
        if den == 0 { panic!("Zero denominator"); }
        let g = gcd(num.abs(), den.abs());
        let sign = if (num < 0) != (den < 0) { -1 } else { 1 };
        HighPrecRat {
            num: sign * num.abs() / g,
            den: den.abs() / g,
            symbolic: None,
        }
    }
    
    fn from_int(n: i128) -> Self {
        HighPrecRat { num: n, den: 1, symbolic: None }
    }
    
    fn from_float_approx(f: f64, precision: i128) -> Self {
        // 将浮点数转换为有理数近似
        let scaled = (f * precision as f64).round() as i128;
        HighPrecRat::new(scaled, precision)
    }
    
    fn to_f64(&self) -> f64 {
        self.num as f64 / self.den as f64
    }
    
    fn is_close_to(&self, other: &HighPrecRat, tolerance: f64) -> bool {
        (self.to_f64() - other.to_f64()).abs() < tolerance
    }
}

impl std::ops::Add for HighPrecRat {
    type Output = HighPrecRat;
    fn add(self, other: HighPrecRat) -> HighPrecRat {
        HighPrecRat::new(
            self.num * other.den + other.num * self.den,
            self.den * other.den
        )
    }
}

impl std::ops::Sub for HighPrecRat {
    type Output = HighPrecRat;
    fn sub(self, other: HighPrecRat) -> HighPrecRat {
        HighPrecRat::new(
            self.num * other.den - other.num * self.den,
            self.den * other.den
        )
    }
}

impl std::ops::Mul for HighPrecRat {
    type Output = HighPrecRat;
    fn mul(self, other: HighPrecRat) -> HighPrecRat {
        HighPrecRat::new(self.num * other.num, self.den * other.den)
    }
}

impl std::ops::Div for HighPrecRat {
    type Output = HighPrecRat;
    fn div(self, other: HighPrecRat) -> HighPrecRat {
        HighPrecRat::new(self.num * other.den, self.den * other.num)
    }
}

impl std::fmt::Display for HighPrecRat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(ref sym) = self.symbolic {
            write!(f, "{}", sym)
        } else if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 { a.max(1) } else { gcd(b, a % b) }
}

// ============================================================================
// 无理数常数库
// ============================================================================

struct IrrationalConstants {
    constants: HashMap<String, (f64, String)>, // (value, latex)
}

impl IrrationalConstants {
    fn new() -> Self {
        let mut c = HashMap::new();
        
        // 数学常数
        c.insert("pi".into(), (std::f64::consts::PI, "\\pi".into()));
        c.insert("e".into(), (std::f64::consts::E, "e".into()));
        c.insert("sqrt2".into(), (std::f64::consts::SQRT_2, "\\sqrt{2}".into()));
        c.insert("sqrt3".into(), (3.0_f64.sqrt(), "\\sqrt{3}".into()));
        c.insert("sqrt5".into(), (5.0_f64.sqrt(), "\\sqrt{5}".into()));
        c.insert("phi".into(), ((1.0 + 5.0_f64.sqrt()) / 2.0, "\\phi".into())); // 黄金比
        c.insert("ln2".into(), (2.0_f64.ln(), "\\ln 2".into()));
        c.insert("ln10".into(), (10.0_f64.ln(), "\\ln 10".into()));
        
        // π 的幂次
        c.insert("pi2".into(), (std::f64::consts::PI.powi(2), "\\pi^2".into()));
        c.insert("pi3".into(), (std::f64::consts::PI.powi(3), "\\pi^3".into()));
        c.insert("pi4".into(), (std::f64::consts::PI.powi(4), "\\pi^4".into()));
        
        // 常用组合
        c.insert("2pi".into(), (2.0 * std::f64::consts::PI, "2\\pi".into()));
        c.insert("4pi".into(), (4.0 * std::f64::consts::PI, "4\\pi".into()));
        c.insert("pi_sqrt2".into(), (std::f64::consts::PI * std::f64::consts::SQRT_2, "\\pi\\sqrt{2}".into()));
        
        IrrationalConstants { constants: c }
    }
    
    fn get(&self, name: &str) -> Option<f64> {
        self.constants.get(name).map(|(v, _)| *v)
    }
    
    fn latex(&self, name: &str) -> Option<&str> {
        self.constants.get(name).map(|(_, l)| l.as_str())
    }
    
    fn all(&self) -> Vec<(&str, f64, &str)> {
        self.constants.iter()
            .map(|(k, (v, l))| (k.as_str(), *v, l.as_str()))
            .collect()
    }
}

// ============================================================================
// 精确物理常数 (CODATA 2022)
// ============================================================================

struct PhysicsConstants {
    constants: HashMap<String, f64>,
}

impl PhysicsConstants {
    fn new() -> Self {
        let mut c = HashMap::new();
        
        // 精细结构常数
        c.insert("alpha_inv".into(), 137.035999177);
        
        // 质量比
        c.insert("m_mu_me".into(), 206.7682827);
        c.insert("m_tau_me".into(), 3477.23);
        c.insert("m_p_me".into(), 1836.15267343);
        c.insert("m_n_me".into(), 1838.68366173);
        
        // 电弱质量比
        c.insert("m_W_me".into(), 157298.8);   // 80.377 GeV / 0.51099895 MeV
        c.insert("m_Z_me".into(), 178450.3);   // 91.1876 GeV / 0.51099895 MeV
        c.insert("m_H_me".into(), 244802.0);   // 125.1 GeV / 0.51099895 MeV (更精确)
        c.insert("m_t_me".into(), 338717.0);   // 173.07 GeV / 0.51099895 MeV
        
        // Weinberg角
        c.insert("sin2_theta_W".into(), 0.23122);
        c.insert("cos2_theta_W".into(), 0.76878);
        
        // 宇宙学参数
        c.insert("Omega_Lambda".into(), 0.6889);
        c.insert("Omega_m".into(), 0.3111);
        c.insert("Omega_b".into(), 0.0486);
        
        // Koide 参数
        c.insert("koide_ratio".into(), 0.666661); // ≈ 2/3
        
        // 质子/中子质量差
        c.insert("m_n_m_p".into(), 1.00137841931);
        
        PhysicsConstants { constants: c }
    }
    
    fn get(&self, name: &str) -> Option<f64> {
        self.constants.get(name).copied()
    }
    
    fn all(&self) -> Vec<(&str, f64)> {
        self.constants.iter().map(|(k, v)| (k.as_str(), *v)).collect()
    }
}

// ============================================================================
// 代数表达式结构
// ============================================================================

#[derive(Debug, Clone)]
struct AlgebraicExpression {
    formula: String,
    latex: String,
    computed_value: f64,
    target_name: String,
    target_value: f64,
    error_percent: f64,
}

// ============================================================================
// 搜索引擎
// ============================================================================

/// 搜索形如 a + b*X + c*X² 的表达式，其中X是无理数常数
fn search_polynomial_in_irrational(
    target_name: &str,
    target: f64,
    irr_name: &str,
    irr_value: f64,
    irr_latex: &str,
    tolerance: f64,
) -> Vec<AlgebraicExpression> {
    let mut results = Vec::new();
    
    // 搜索 a + b*X
    for a in -1000i64..=1000 {
        for b in -500i64..=500 {
            if b == 0 && a == 0 { continue; }
            
            let computed = a as f64 + b as f64 * irr_value;
            let error = ((computed - target) / target).abs() * 100.0;
            
            if error < tolerance {
                let formula = if a == 0 {
                    format!("{}*{}", b, irr_name)
                } else if b > 0 {
                    format!("{} + {}*{}", a, b, irr_name)
                } else {
                    format!("{} - {}*{}", a, -b, irr_name)
                };
                
                let latex = if a == 0 {
                    format!("{}{}", b, irr_latex)
                } else if b > 0 {
                    format!("{} + {}{}", a, b, irr_latex)
                } else {
                    format!("{} - {}{}", a, -b, irr_latex)
                };
                
                results.push(AlgebraicExpression {
                    formula,
                    latex,
                    computed_value: computed,
                    target_name: target_name.into(),
                    target_value: target,
                    error_percent: error,
                });
            }
        }
    }
    
    // 搜索 a + b*X + c*X²
    let irr_sq = irr_value * irr_value;
    for a in -500i64..=500 {
        for b in -100i64..=100 {
            for c in -100i64..=100 {
                if a == 0 && b == 0 && c == 0 { continue; }
                
                let computed = a as f64 + b as f64 * irr_value + c as f64 * irr_sq;
                let error = ((computed - target) / target).abs() * 100.0;
                
                if error < tolerance && c != 0 {
                    let formula = format!("{} + {}*{} + {}*{}^2", a, b, irr_name, c, irr_name);
                    let latex = format!("{} + {}{} + {}{}^2", a, b, irr_latex, c, irr_latex);
                    
                    results.push(AlgebraicExpression {
                        formula,
                        latex,
                        computed_value: computed,
                        target_name: target_name.into(),
                        target_value: target,
                        error_percent: error,
                    });
                }
            }
        }
    }
    
    // 搜索 (a + b*X) / (c + d*X) 有理函数
    for a in -100i64..=100 {
        for b in -50i64..=50 {
            for c in 1i64..=50 {
                for d in -20i64..=20 {
                    let numer = a as f64 + b as f64 * irr_value;
                    let denom = c as f64 + d as f64 * irr_value;
                    if denom.abs() < 0.001 { continue; }
                    
                    let computed = numer / denom;
                    let error = ((computed - target) / target).abs() * 100.0;
                    
                    if error < tolerance && (b != 0 || d != 0) {
                        let formula = format!("({} + {}*{}) / ({} + {}*{})", a, b, irr_name, c, d, irr_name);
                        let latex = format!("\\frac{{{} + {}{}}}{{{}  + {}{}}}", a, b, irr_latex, c, d, irr_latex);
                        
                        results.push(AlgebraicExpression {
                            formula,
                            latex,
                            computed_value: computed,
                            target_name: target_name.into(),
                            target_value: target,
                            error_percent: error,
                        });
                    }
                }
            }
        }
    }
    
    results
}

/// 搜索混合无理数表达式 a*X + b*Y
fn search_mixed_irrational(
    target_name: &str,
    target: f64,
    irr1: (&str, f64, &str),
    irr2: (&str, f64, &str),
    tolerance: f64,
) -> Vec<AlgebraicExpression> {
    let mut results = Vec::new();
    let (name1, val1, latex1) = irr1;
    let (name2, val2, latex2) = irr2;
    
    for a in -200i64..=200 {
        for b in -200i64..=200 {
            if a == 0 && b == 0 { continue; }
            
            let computed = a as f64 * val1 + b as f64 * val2;
            let error = ((computed - target) / target).abs() * 100.0;
            
            if error < tolerance && a != 0 && b != 0 {
                let formula = format!("{}*{} + {}*{}", a, name1, b, name2);
                let latex = format!("{}{} + {}{}", a, latex1, b, latex2);
                
                results.push(AlgebraicExpression {
                    formula,
                    latex,
                    computed_value: computed,
                    target_name: target_name.into(),
                    target_value: target,
                    error_percent: error,
                });
            }
        }
    }
    
    results
}

/// 搜索 a*α⁻¹ + b*X 的组合
fn search_alpha_irrational_combo(
    target_name: &str,
    target: f64,
    alpha_inv: f64,
    irr_name: &str,
    irr_value: f64,
    irr_latex: &str,
    tolerance: f64,
) -> Vec<AlgebraicExpression> {
    let mut results = Vec::new();
    let alpha_inv_sq = alpha_inv * alpha_inv;
    
    // a*α⁻¹ + b*X
    for a in -50i64..=50 {
        for b in -500i64..=500 {
            let computed = a as f64 * alpha_inv + b as f64 * irr_value;
            let error = ((computed - target) / target).abs() * 100.0;
            
            if error < tolerance && a != 0 && b != 0 {
                let formula = format!("{}*α⁻¹ + {}*{}", a, b, irr_name);
                let latex = format!("{}\\alpha^{{-1}} + {}{}", a, b, irr_latex);
                
                results.push(AlgebraicExpression {
                    formula,
                    latex,
                    computed_value: computed,
                    target_name: target_name.into(),
                    target_value: target,
                    error_percent: error,
                });
            }
        }
    }
    
    // a*α⁻² + b*X
    for a in -30i64..=30 {
        for b in -500i64..=500 {
            let computed = a as f64 * alpha_inv_sq + b as f64 * irr_value;
            let error = ((computed - target) / target).abs() * 100.0;
            
            if error < tolerance && a != 0 && b != 0 {
                let formula = format!("{}*α⁻² + {}*{}", a, b, irr_name);
                let latex = format!("{}\\alpha^{{-2}} + {}{}", a, b, irr_latex);
                
                results.push(AlgebraicExpression {
                    formula,
                    latex,
                    computed_value: computed,
                    target_name: target_name.into(),
                    target_value: target,
                    error_percent: error,
                });
            }
        }
    }
    
    results
}

/// 搜索纯无理数比例 target = a * X / Y
fn search_irrational_ratio(
    target_name: &str,
    target: f64,
    irr1: (&str, f64, &str),
    irr2: (&str, f64, &str),
    tolerance: f64,
) -> Vec<AlgebraicExpression> {
    let mut results = Vec::new();
    let (name1, val1, latex1) = irr1;
    let (name2, val2, latex2) = irr2;
    
    if val2.abs() < 1e-10 { return results; }
    
    for a in 1i64..=500 {
        for b in 1i64..=500 {
            let computed = (a as f64 * val1) / (b as f64 * val2);
            let error = ((computed - target) / target).abs() * 100.0;
            
            if error < tolerance {
                let formula = format!("{}*{} / ({}*{})", a, name1, b, name2);
                let latex = format!("\\frac{{{}{}}}{{{}{}}}", a, latex1, b, latex2);
                
                results.push(AlgebraicExpression {
                    formula,
                    latex,
                    computed_value: computed,
                    target_name: target_name.into(),
                    target_value: target,
                    error_percent: error,
                });
            }
        }
    }
    
    results
}

fn main() {
    println!("================================================================================");
    println!("     S-WIND IRRATIONAL NUMBER UNIFIED SEARCH");
    println!("     无理数大一统理论搜索");
    println!("================================================================================\n");
    
    let mut output: Vec<String> = Vec::new();
    let physics = PhysicsConstants::new();
    let irrational = IrrationalConstants::new();
    
    output.push("# S-wind Irrational Number Unified Search Report\n".into());
    output.push("## 无理数大一统理论搜索报告\n".into());
    output.push("Search includes: π, e, √2, √3, √5, φ (golden ratio), ln2, and their combinations\n".into());
    
    let alpha_inv = physics.get("alpha_inv").unwrap();
    output.push(format!("\nα⁻¹ = {:.10}\n", alpha_inv));
    
    // 主要目标常数
    let targets = [
        ("α⁻¹", physics.get("alpha_inv").unwrap()),
        ("mμ/me", physics.get("m_mu_me").unwrap()),
        ("mτ/me", physics.get("m_tau_me").unwrap()),
        ("mp/me", physics.get("m_p_me").unwrap()),
        ("mn/me", physics.get("m_n_me").unwrap()),
        ("mW/me", physics.get("m_W_me").unwrap()),
        ("mZ/me", physics.get("m_Z_me").unwrap()),
        ("mH/me", physics.get("m_H_me").unwrap()),
        ("mt/me", physics.get("m_t_me").unwrap()),
        ("sin²θ_W", physics.get("sin2_theta_W").unwrap()),
        ("Ω_Λ", physics.get("Omega_Lambda").unwrap()),
    ];
    
    let irr_list = irrational.all();
    let tolerance = 0.01; // 0.01% 误差阈值
    
    for (target_name, target_value) in &targets {
        output.push(format!("\n================================================================================"));
        output.push(format!("## {} = {:.10}", target_name, target_value));
        output.push(format!("================================================================================\n"));
        
        let mut all_results: Vec<AlgebraicExpression> = Vec::new();
        
        // 1. 单个无理数的多项式搜索
        println!("Searching polynomial expressions for {} = {}...", target_name, target_value);
        for (irr_name, irr_value, irr_latex) in &irr_list {
            let results = search_polynomial_in_irrational(
                target_name, *target_value, irr_name, *irr_value, irr_latex, tolerance
            );
            all_results.extend(results);
        }
        
        // 2. α与无理数的组合搜索
        println!("Searching α + irrational combinations for {}...", target_name);
        for (irr_name, irr_value, irr_latex) in &irr_list {
            let results = search_alpha_irrational_combo(
                target_name, *target_value, alpha_inv, irr_name, *irr_value, irr_latex, tolerance
            );
            all_results.extend(results);
        }
        
        // 3. 混合无理数搜索 (选择几个重要组合)
        println!("Searching mixed irrational expressions for {}...", target_name);
        let key_pairs = [
            ("pi", "e"),
            ("pi", "sqrt2"),
            ("pi2", "e"),
            ("phi", "pi"),
            ("sqrt5", "pi"),
        ];
        
        for (n1, n2) in &key_pairs {
            if let (Some((v1, l1)), Some((v2, l2))) = (
                irrational.constants.get(*n1),
                irrational.constants.get(*n2)
            ) {
                let results = search_mixed_irrational(
                    target_name, *target_value,
                    (n1, *v1, l1),
                    (n2, *v2, l2),
                    tolerance
                );
                all_results.extend(results);
            }
        }
        
        // 排序并输出最佳结果
        all_results.sort_by(|a, b| a.error_percent.partial_cmp(&b.error_percent).unwrap());
        all_results.dedup_by(|a, b| a.formula == b.formula);
        
        output.push(format!("Found {} expressions with error < {}%\n", all_results.len(), tolerance));
        
        if !all_results.is_empty() {
            output.push("### Best expressions (sorted by precision):".into());
            for (i, expr) in all_results.iter().take(15).enumerate() {
                output.push(format!(
                    "{}. {} = {:.10} (error: {:.6}%)",
                    i + 1, expr.formula, expr.computed_value, expr.error_percent
                ));
                output.push(format!("   LaTeX: ${}$", expr.latex));
            }
        } else {
            output.push("No expressions found within tolerance.".into());
        }
    }
    
    // 特殊搜索：精细结构常数的精确表达式
    output.push("\n================================================================================".into());
    output.push("## Special: Exact expressions for α⁻¹".into());
    output.push("================================================================================\n".into());
    
    let alpha = physics.get("alpha_inv").unwrap();
    
    // 著名的近似公式
    let famous_formulas = [
        ("137", 137.0),
        ("4π³ + π² + π", 4.0 * std::f64::consts::PI.powi(3) + std::f64::consts::PI.powi(2) + std::f64::consts::PI),
        ("(π⁵/2¹⁵)^(1/4) * 2^7", ((std::f64::consts::PI.powi(5) / 32768.0).powf(0.25)) * 128.0),
        ("137 + π/2790", 137.0 + std::f64::consts::PI / 2790.0),
        ("e^(π/2) * 27 + e", std::f64::consts::E.powf(std::f64::consts::PI / 2.0) * 27.0 + std::f64::consts::E),
    ];
    
    output.push("Known approximations:".into());
    for (formula, value) in &famous_formulas {
        let error = ((value - alpha) / alpha).abs() * 100.0;
        output.push(format!("  {} = {:.10} (error: {:.6}%)", formula, value, error));
    }
    
    // 输出
    let report = output.join("\n");
    println!("\n{}", report);
    
    let mut file = File::create("/home/jouly/AI/Valhalla/s-wind-sim/output/irrational_unified_search.txt")
        .expect("Cannot create file");
    file.write_all(report.as_bytes()).expect("Cannot write");
    
    println!("\n================================================================================");
    println!("Report saved to: output/irrational_unified_search.txt");
    println!("================================================================================");
}
