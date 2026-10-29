//! S-wind 符号无理数搜索
//! 
//! 使用 S-wind 的 Expr 符号系统，而不是独立的 f64 搜索
//! 
//! 特性：
//! 1. 使用真正的物理常数精确值（非整数近似）
//! 2. 构建符号表达式，然后数值验证
//! 3. 支持 π, e, √n 等超越数
//! 4. 保留符号形式用于输出

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// ============================================================================
// 精确物理常数 (CODATA 2022 精确值)
// ============================================================================

/// 精确物理常数表
struct PhysicsConstants {
    constants: HashMap<String, f64>,
}

impl PhysicsConstants {
    fn new() -> Self {
        let mut c = HashMap::new();
        
        // 精细结构常数 (CODATA 2022)
        c.insert("alpha_inv".into(), 137.035999177);
        c.insert("alpha".into(), 1.0 / 137.035999177);
        
        // 轻子质量比 (精确值)
        c.insert("m_mu_me".into(), 206.7682827);    // CODATA 2022
        c.insert("m_tau_me".into(), 3477.23);       // PDG 2022
        
        // 重子质量比
        c.insert("m_p_me".into(), 1836.15267343);   // CODATA 2022
        c.insert("m_n_me".into(), 1838.68366173);   // CODATA 2022
        
        // 电弱玻色子质量比 (PDG 2022)
        c.insert("m_W_me".into(), 157298.8);        // 80.377 GeV / 0.51099895 MeV
        c.insert("m_Z_me".into(), 178450.3);        // 91.1876 GeV / 0.51099895 MeV
        c.insert("m_H_me".into(), 244802.0);        // 125.1 GeV / 0.51099895 MeV
        c.insert("m_t_me".into(), 338717.0);        // 173.07 GeV / 0.51099895 MeV
        
        // Weinberg角
        c.insert("sin2_theta_W".into(), 0.23122);
        
        // 宇宙学参数
        c.insert("Omega_Lambda".into(), 0.6889);
        c.insert("Omega_m".into(), 0.3111);
        
        PhysicsConstants { constants: c }
    }
    
    fn get(&self, name: &str) -> Option<f64> {
        self.constants.get(name).copied()
    }
}

// ============================================================================
// 符号无理数常数
// ============================================================================

/// 无理数常数（符号 + 数值）
#[derive(Clone)]
struct IrrationalConst {
    name: String,       // 符号名称 (e.g., "π")
    latex: String,      // LaTeX 表示 (e.g., "\\pi")
    value: f64,         // 数值近似
}

impl IrrationalConst {
    fn new(name: &str, latex: &str, value: f64) -> Self {
        IrrationalConst {
            name: name.to_string(),
            latex: latex.to_string(),
            value,
        }
    }
}

/// 无理数常数库
struct IrrationalLibrary {
    constants: Vec<IrrationalConst>,
}

impl IrrationalLibrary {
    fn new() -> Self {
        let mut constants = Vec::new();
        
        // 数学常数
        constants.push(IrrationalConst::new("π", "\\pi", std::f64::consts::PI));
        constants.push(IrrationalConst::new("e", "e", std::f64::consts::E));
        constants.push(IrrationalConst::new("√2", "\\sqrt{2}", std::f64::consts::SQRT_2));
        constants.push(IrrationalConst::new("√3", "\\sqrt{3}", 3.0_f64.sqrt()));
        constants.push(IrrationalConst::new("√5", "\\sqrt{5}", 5.0_f64.sqrt()));
        constants.push(IrrationalConst::new("φ", "\\phi", (1.0 + 5.0_f64.sqrt()) / 2.0));
        constants.push(IrrationalConst::new("ln2", "\\ln 2", 2.0_f64.ln()));
        constants.push(IrrationalConst::new("ln10", "\\ln 10", 10.0_f64.ln()));
        
        // π 的幂次
        constants.push(IrrationalConst::new("π²", "\\pi^2", std::f64::consts::PI.powi(2)));
        constants.push(IrrationalConst::new("π³", "\\pi^3", std::f64::consts::PI.powi(3)));
        constants.push(IrrationalConst::new("π⁴", "\\pi^4", std::f64::consts::PI.powi(4)));
        
        // e 的幂次
        constants.push(IrrationalConst::new("e²", "e^2", std::f64::consts::E.powi(2)));
        
        IrrationalLibrary { constants }
    }
    
    fn all(&self) -> &[IrrationalConst] {
        &self.constants
    }
}

// ============================================================================
// 符号表达式 (简化版，兼容 S-wind Expr 设计)
// ============================================================================

/// 符号表达式
#[derive(Clone)]
enum SymExpr {
    /// 有理数
    Rational(i64, i64),
    /// 无理数常数
    Irrational(IrrationalConst),
    /// 物理常数引用 (α⁻¹, α⁻², etc.)
    PhysicsConst(String, f64),
    /// 加法
    Add(Box<SymExpr>, Box<SymExpr>),
    /// 乘法
    Mul(Box<SymExpr>, Box<SymExpr>),
    /// 除法
    Div(Box<SymExpr>, Box<SymExpr>),
}

impl SymExpr {
    /// 数值求值
    fn eval(&self) -> f64 {
        match self {
            SymExpr::Rational(n, d) => *n as f64 / *d as f64,
            SymExpr::Irrational(c) => c.value,
            SymExpr::PhysicsConst(_, v) => *v,
            SymExpr::Add(a, b) => a.eval() + b.eval(),
            SymExpr::Mul(a, b) => a.eval() * b.eval(),
            SymExpr::Div(a, b) => a.eval() / b.eval(),
        }
    }
    
    /// 转换为符号字符串
    fn to_symbolic(&self) -> String {
        match self {
            SymExpr::Rational(n, d) => {
                if *d == 1 { format!("{}", n) }
                else { format!("{}/{}", n, d) }
            }
            SymExpr::Irrational(c) => c.name.clone(),
            SymExpr::PhysicsConst(name, _) => name.clone(),
            SymExpr::Add(a, b) => format!("{} + {}", a.to_symbolic(), b.to_symbolic()),
            SymExpr::Mul(a, b) => format!("{}·{}", a.to_symbolic(), b.to_symbolic()),
            SymExpr::Div(a, b) => format!("({})/({})", a.to_symbolic(), b.to_symbolic()),
        }
    }
    
    /// 转换为 LaTeX
    fn to_latex(&self) -> String {
        match self {
            SymExpr::Rational(n, d) => {
                if *d == 1 { format!("{}", n) }
                else { format!("\\frac{{{}}}{{{}}}", n, d) }
            }
            SymExpr::Irrational(c) => c.latex.clone(),
            SymExpr::PhysicsConst(name, _) => {
                if name == "α⁻¹" { "\\alpha^{-1}".to_string() }
                else if name == "α⁻²" { "\\alpha^{-2}".to_string() }
                else { name.clone() }
            }
            SymExpr::Add(a, b) => format!("{} + {}", a.to_latex(), b.to_latex()),
            SymExpr::Mul(a, b) => format!("{} \\cdot {}", a.to_latex(), b.to_latex()),
            SymExpr::Div(a, b) => format!("\\frac{{{}}}{{{}}}", a.to_latex(), b.to_latex()),
        }
    }
}

// ============================================================================
// 搜索发现结果
// ============================================================================

struct Discovery {
    target_name: String,
    target_value: f64,
    expression: SymExpr,
    computed_value: f64,
    error_percent: f64,
}

impl Discovery {
    fn new(target_name: &str, target_value: f64, expression: SymExpr) -> Self {
        let computed_value = expression.eval();
        let error_percent = ((computed_value - target_value) / target_value).abs() * 100.0;
        Discovery {
            target_name: target_name.to_string(),
            target_value,
            expression,
            computed_value,
            error_percent,
        }
    }
}

// ============================================================================
// 符号搜索引擎
// ============================================================================

/// 搜索 a + b*X 形式的表达式
fn search_linear_irrational(
    target_name: &str,
    target: f64,
    irr: &IrrationalConst,
    tolerance: f64,
) -> Vec<Discovery> {
    let mut results = Vec::new();
    
    for a in -2000i64..=2000 {
        for b in -1000i64..=1000 {
            if b == 0 { continue; }
            
            let expr = SymExpr::Add(
                Box::new(SymExpr::Rational(a, 1)),
                Box::new(SymExpr::Mul(
                    Box::new(SymExpr::Rational(b, 1)),
                    Box::new(SymExpr::Irrational(irr.clone())),
                )),
            );
            
            let computed = expr.eval();
            let error = ((computed - target) / target).abs() * 100.0;
            
            if error < tolerance {
                results.push(Discovery::new(target_name, target, expr));
            }
        }
    }
    
    results
}

/// 搜索 a + b*X + c*X² 形式的表达式
fn search_quadratic_irrational(
    target_name: &str,
    target: f64,
    irr: &IrrationalConst,
    tolerance: f64,
) -> Vec<Discovery> {
    let mut results = Vec::new();
    let x = irr.value;
    let x2 = x * x;
    
    for a in -500i64..=500 {
        for b in -200i64..=200 {
            for c in -100i64..=100 {
                if b == 0 && c == 0 { continue; }
                
                let computed = a as f64 + b as f64 * x + c as f64 * x2;
                let error = ((computed - target) / target).abs() * 100.0;
                
                if error < tolerance && c != 0 {
                    // 构建符号表达式
                    let x_term = SymExpr::Mul(
                        Box::new(SymExpr::Rational(b, 1)),
                        Box::new(SymExpr::Irrational(irr.clone())),
                    );
                    let x2_const = IrrationalConst::new(
                        &format!("{}²", irr.name),
                        &format!("{}^2", irr.latex),
                        x2,
                    );
                    let x2_term = SymExpr::Mul(
                        Box::new(SymExpr::Rational(c, 1)),
                        Box::new(SymExpr::Irrational(x2_const)),
                    );
                    let expr = SymExpr::Add(
                        Box::new(SymExpr::Rational(a, 1)),
                        Box::new(SymExpr::Add(Box::new(x_term), Box::new(x2_term))),
                    );
                    
                    results.push(Discovery::new(target_name, target, expr));
                }
            }
        }
    }
    
    results
}

/// 搜索 a*α⁻¹ + b*X 形式
fn search_alpha_irrational(
    target_name: &str,
    target: f64,
    alpha_inv: f64,
    irr: &IrrationalConst,
    tolerance: f64,
) -> Vec<Discovery> {
    let mut results = Vec::new();
    
    for a in -100i64..=100 {
        for b in -1000i64..=1000 {
            if a == 0 && b == 0 { continue; }
            
            let computed = a as f64 * alpha_inv + b as f64 * irr.value;
            let error = ((computed - target) / target).abs() * 100.0;
            
            if error < tolerance && a != 0 && b != 0 {
                let alpha_term = SymExpr::Mul(
                    Box::new(SymExpr::Rational(a, 1)),
                    Box::new(SymExpr::PhysicsConst("α⁻¹".to_string(), alpha_inv)),
                );
                let irr_term = SymExpr::Mul(
                    Box::new(SymExpr::Rational(b, 1)),
                    Box::new(SymExpr::Irrational(irr.clone())),
                );
                let expr = SymExpr::Add(Box::new(alpha_term), Box::new(irr_term));
                
                results.push(Discovery::new(target_name, target, expr));
            }
        }
    }
    
    results
}

/// 搜索 a*α⁻² + b*X 形式 (重粒子)
fn search_alpha_sq_irrational(
    target_name: &str,
    target: f64,
    alpha_inv_sq: f64,
    irr: &IrrationalConst,
    tolerance: f64,
) -> Vec<Discovery> {
    let mut results = Vec::new();
    
    for a in -30i64..=30 {
        for b in -1000i64..=1000 {
            if a == 0 && b == 0 { continue; }
            
            let computed = a as f64 * alpha_inv_sq + b as f64 * irr.value;
            let error = ((computed - target) / target).abs() * 100.0;
            
            if error < tolerance && a != 0 && b != 0 {
                let alpha_term = SymExpr::Mul(
                    Box::new(SymExpr::Rational(a, 1)),
                    Box::new(SymExpr::PhysicsConst("α⁻²".to_string(), alpha_inv_sq)),
                );
                let irr_term = SymExpr::Mul(
                    Box::new(SymExpr::Rational(b, 1)),
                    Box::new(SymExpr::Irrational(irr.clone())),
                );
                let expr = SymExpr::Add(Box::new(alpha_term), Box::new(irr_term));
                
                results.push(Discovery::new(target_name, target, expr));
            }
        }
    }
    
    results
}

/// 搜索有理函数 (a + b*X) / (c + d*X)
fn search_rational_function(
    target_name: &str,
    target: f64,
    irr: &IrrationalConst,
    tolerance: f64,
) -> Vec<Discovery> {
    let mut results = Vec::new();
    let x = irr.value;
    
    for a in -100i64..=100 {
        for b in -50i64..=50 {
            for c in 1i64..=50 {
                for d in -30i64..=30 {
                    let numer = a as f64 + b as f64 * x;
                    let denom = c as f64 + d as f64 * x;
                    if denom.abs() < 0.01 { continue; }
                    
                    let computed = numer / denom;
                    let error = ((computed - target) / target).abs() * 100.0;
                    
                    if error < tolerance && (b != 0 || d != 0) {
                        let numer_expr = SymExpr::Add(
                            Box::new(SymExpr::Rational(a, 1)),
                            Box::new(SymExpr::Mul(
                                Box::new(SymExpr::Rational(b, 1)),
                                Box::new(SymExpr::Irrational(irr.clone())),
                            )),
                        );
                        let denom_expr = SymExpr::Add(
                            Box::new(SymExpr::Rational(c, 1)),
                            Box::new(SymExpr::Mul(
                                Box::new(SymExpr::Rational(d, 1)),
                                Box::new(SymExpr::Irrational(irr.clone())),
                            )),
                        );
                        let expr = SymExpr::Div(Box::new(numer_expr), Box::new(denom_expr));
                        
                        results.push(Discovery::new(target_name, target, expr));
                    }
                }
            }
        }
    }
    
    results
}

// ============================================================================
// Main
// ============================================================================

fn main() {
    println!("================================================================================");
    println!("     S-WIND SYMBOLIC IRRATIONAL NUMBER SEARCH");
    println!("     使用 S-wind 符号系统的无理数搜索");
    println!("================================================================================\n");
    
    let mut output: Vec<String> = Vec::new();
    let physics = PhysicsConstants::new();
    let irrationals = IrrationalLibrary::new();
    
    output.push("# S-wind Symbolic Irrational Number Search\n".into());
    output.push("Using S-wind Expr-compatible symbolic system with exact CODATA constants.\n".into());
    
    let alpha_inv = physics.get("alpha_inv").unwrap();
    let alpha_inv_sq = alpha_inv * alpha_inv;
    
    output.push(format!("## Physical Constants (CODATA 2022 exact values)\n"));
    output.push(format!("- α⁻¹ = {:.12}", alpha_inv));
    output.push(format!("- α⁻² = {:.6}", alpha_inv_sq));
    output.push(format!("- mμ/me = {:.10}", physics.get("m_mu_me").unwrap()));
    output.push(format!("- mτ/me = {:.6}", physics.get("m_tau_me").unwrap()));
    output.push(format!("- mp/me = {:.10}", physics.get("m_p_me").unwrap()));
    output.push(format!("- mW/me = {:.1}", physics.get("m_W_me").unwrap()));
    output.push(format!("- mZ/me = {:.1}", physics.get("m_Z_me").unwrap()));
    output.push(format!("- mH/me = {:.1}", physics.get("m_H_me").unwrap()));
    output.push(format!("- mt/me = {:.1}\n", physics.get("m_t_me").unwrap()));
    
    // 目标常数列表
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
    
    let tolerance = 0.001; // 0.001% 精度
    
    for (target_name, target_value) in &targets {
        output.push(format!("\n================================================================================"));
        output.push(format!("## {} = {:.12}", target_name, target_value));
        output.push(format!("================================================================================\n"));
        
        println!("Searching for {} = {}...", target_name, target_value);
        
        let mut all_discoveries: Vec<Discovery> = Vec::new();
        
        // 1. 线性无理数搜索 a + b*X
        for irr in irrationals.all() {
            let results = search_linear_irrational(target_name, *target_value, irr, tolerance);
            all_discoveries.extend(results);
        }
        
        // 2. 二次无理数搜索 a + b*X + c*X²
        for irr in irrationals.all() {
            let results = search_quadratic_irrational(target_name, *target_value, irr, tolerance);
            all_discoveries.extend(results);
        }
        
        // 3. α + 无理数组合
        for irr in irrationals.all() {
            let results = search_alpha_irrational(target_name, *target_value, alpha_inv, irr, tolerance);
            all_discoveries.extend(results);
        }
        
        // 4. α² + 无理数组合 (重粒子)
        if *target_value > 10000.0 {
            for irr in irrationals.all() {
                let results = search_alpha_sq_irrational(target_name, *target_value, alpha_inv_sq, irr, tolerance);
                all_discoveries.extend(results);
            }
        }
        
        // 5. 有理函数 (小值常数)
        if *target_value < 1000.0 {
            for irr in irrationals.all() {
                let results = search_rational_function(target_name, *target_value, irr, tolerance);
                all_discoveries.extend(results);
            }
        }
        
        // 排序并去重
        all_discoveries.sort_by(|a, b| a.error_percent.partial_cmp(&b.error_percent).unwrap());
        
        // 输出最佳结果
        output.push(format!("Found {} expressions with error < {}%\n", all_discoveries.len(), tolerance));
        
        if !all_discoveries.is_empty() {
            output.push("### Best expressions (sorted by precision):".into());
            
            let mut seen_formulas = std::collections::HashSet::new();
            let mut count = 0;
            
            for disc in &all_discoveries {
                let formula = disc.expression.to_symbolic();
                if seen_formulas.contains(&formula) { continue; }
                seen_formulas.insert(formula.clone());
                
                count += 1;
                if count > 10 { break; }
                
                output.push(format!(
                    "{}. {} = {:.12} (error: {:.8}%)",
                    count, formula, disc.computed_value, disc.error_percent
                ));
                output.push(format!("   LaTeX: ${}$", disc.expression.to_latex()));
            }
        } else {
            output.push("No expressions found within tolerance.".into());
        }
    }
    
    // 特殊：著名的 α⁻¹ 近似公式验证
    output.push("\n================================================================================".into());
    output.push("## Famous α⁻¹ approximations (verification)".into());
    output.push("================================================================================\n".into());
    
    let alpha = physics.get("alpha_inv").unwrap();
    let pi = std::f64::consts::PI;
    let e = std::f64::consts::E;
    
    let famous = [
        ("137 (integer)", 137.0),
        ("4π³ + π² + π", 4.0 * pi.powi(3) + pi.powi(2) + pi),
        ("(e^π - 1)/2 + 132", (e.powf(pi) - 1.0) / 2.0 + 132.0),
        ("29/212 * π^4", 29.0 / 212.0 * pi.powi(4)),
        ("(π⁵/15)^(1/4) + 137", (pi.powi(5) / 15.0).powf(0.25) + 137.0),
    ];
    
    for (formula, value) in &famous {
        let error = ((value - alpha) / alpha).abs() * 100.0;
        output.push(format!("- {} = {:.12} (error: {:.8}%)", formula, value, error));
    }
    
    // 输出报告
    let report = output.join("\n");
    println!("\n{}", report);
    
    std::fs::create_dir_all("output").ok();
    let mut file = File::create("output/swind_symbolic_search.txt")
        .expect("Cannot create file");
    file.write_all(report.as_bytes()).expect("Cannot write");
    
    println!("\n================================================================================");
    println!("Report saved to: output/swind_symbolic_search.txt");
    println!("================================================================================");
}
