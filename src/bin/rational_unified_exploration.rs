//! S-wind 有理数大一统理论穷尽探索
//! 
//! 扩展搜索空间：从纯整数到有理数(Rational)
//! 保持完全代数精确性，不使用f64
//! 
//! 目标：找到统一的代数结构连接所有基本物理常数

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// ============================================================================
// EPR (Extended Precision Rational) - 精确有理数
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EPR {
    numerator: i128,
    denominator: i128,
}

impl EPR {
    fn new(n: i128, d: i128) -> Self {
        if d == 0 { panic!("Division by zero"); }
        let sign = if (n < 0) != (d < 0) { -1 } else { 1 };
        let n_abs = n.abs();
        let d_abs = d.abs();
        let g = gcd(n_abs, d_abs);
        EPR { 
            numerator: sign * (n_abs / g), 
            denominator: d_abs / g 
        }
    }
    
    fn from_i128(n: i128) -> Self { EPR::new(n, 1) }
    
    fn is_integer(&self) -> bool { self.denominator == 1 }
    fn is_zero(&self) -> bool { self.numerator == 0 }
    
    fn abs(&self) -> Self { EPR::new(self.numerator.abs(), self.denominator) }
    
    fn pow(&self, exp: u32) -> Self {
        if exp == 0 { return EPR::from_i128(1); }
        let mut result = self.clone();
        for _ in 1..exp {
            result = result * self.clone();
        }
        result
    }
    
    fn to_f64(&self) -> f64 { self.numerator as f64 / self.denominator as f64 }
}

impl std::ops::Add for EPR {
    type Output = EPR;
    fn add(self, other: EPR) -> EPR {
        EPR::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator
        )
    }
}

impl std::ops::Sub for EPR {
    type Output = EPR;
    fn sub(self, other: EPR) -> EPR {
        EPR::new(
            self.numerator * other.denominator - other.numerator * self.denominator,
            self.denominator * other.denominator
        )
    }
}

impl std::ops::Mul for EPR {
    type Output = EPR;
    fn mul(self, other: EPR) -> EPR {
        EPR::new(self.numerator * other.numerator, self.denominator * other.denominator)
    }
}

impl std::ops::Div for EPR {
    type Output = EPR;
    fn div(self, other: EPR) -> EPR {
        if other.is_zero() { panic!("Division by zero"); }
        EPR::new(self.numerator * other.denominator, self.denominator * other.numerator)
    }
}

impl std::fmt::Display for EPR {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 { a.max(1) } else { gcd(b, a % b) }
}

// ============================================================================
// 物理常数数据库 (精确有理数表示)
// ============================================================================

struct PhysicsConstants {
    constants: HashMap<String, EPR>,
    // 高精度原始值用于比较
    high_precision: HashMap<String, (i128, i128)>, // (value * 10^precision, precision)
}

impl PhysicsConstants {
    fn new() -> Self {
        let mut c = HashMap::new();
        let mut hp = HashMap::new();
        
        // α⁻¹ = 137.035999084 (CODATA 2018)
        // 精确表示为有理数：137035999084 / 1000000000
        c.insert("alpha_inv".into(), EPR::new(137035999084, 1000000000));
        hp.insert("alpha_inv".into(), (137035999084i128, 9));
        
        // 整数 137
        c.insert("I137".into(), EPR::from_i128(137));
        
        // 质量比 (CODATA 2018)
        // mμ/me = 206.7682830
        c.insert("m_mu_me".into(), EPR::new(2067682830, 10000000));
        hp.insert("m_mu_me".into(), (2067682830i128, 7));
        
        // mτ/me = 3477.23
        c.insert("m_tau_me".into(), EPR::new(347723, 100));
        hp.insert("m_tau_me".into(), (347723i128, 2));
        
        // mp/me = 1836.15267343
        c.insert("m_p_me".into(), EPR::new(183615267343, 100000000));
        hp.insert("m_p_me".into(), (183615267343i128, 8));
        
        // mn/me = 1838.68366173
        c.insert("m_n_me".into(), EPR::new(183868366173, 100000000));
        hp.insert("m_n_me".into(), (183868366173i128, 8));
        
        // mW/me = 157299 (约)
        c.insert("m_W_me".into(), EPR::new(157299, 1));
        
        // mZ/me = 178450 (约)
        c.insert("m_Z_me".into(), EPR::new(178450, 1));
        
        // mH/me = 244954 (约)
        c.insert("m_H_me".into(), EPR::new(244954, 1));
        
        // mt/me = 338962 (约)
        c.insert("m_t_me".into(), EPR::new(338962, 1));
        
        // Weinberg角 sin²θ_W = 0.23122
        c.insert("sin2_theta_W".into(), EPR::new(23122, 100000));
        hp.insert("sin2_theta_W".into(), (23122i128, 5));
        
        // 宇宙学常数
        c.insert("Omega_Lambda".into(), EPR::new(6889, 10000));
        c.insert("Omega_m".into(), EPR::new(3111, 10000));
        
        // Koide公式目标 2/3
        c.insert("koide_2_3".into(), EPR::new(2, 3));
        
        // 关键整数
        c.insert("I133".into(), EPR::from_i128(133)); // dim(E7)
        c.insert("I4".into(), EPR::from_i128(4));     // D=4
        
        PhysicsConstants { constants: c, high_precision: hp }
    }
    
    fn get(&self, name: &str) -> Option<&EPR> {
        self.constants.get(name)
    }
}

// ============================================================================
// 有理数多项式
// ============================================================================

#[derive(Debug, Clone)]
struct RationalPolynomial {
    /// 系数 [a0, a1, a2, ...] 对应 a0 + a1*D + a2*D² + ...
    coefficients: Vec<EPR>,
    name: String,
}

impl RationalPolynomial {
    fn from_integers(coeffs: &[i128]) -> Self {
        RationalPolynomial {
            coefficients: coeffs.iter().map(|&c| EPR::from_i128(c)).collect(),
            name: format!("IntPoly{:?}", coeffs),
        }
    }
    
    fn from_rationals(coeffs: Vec<EPR>) -> Self {
        RationalPolynomial {
            coefficients: coeffs,
            name: "RatPoly".into(),
        }
    }
    
    fn evaluate(&self, d: &EPR) -> EPR {
        let mut result = EPR::from_i128(0);
        let mut d_power = EPR::from_i128(1);
        
        for coeff in &self.coefficients {
            result = result + coeff.clone() * d_power.clone();
            d_power = d_power * d.clone();
        }
        result
    }
    
    fn evaluate_i128(&self, d: i128) -> EPR {
        self.evaluate(&EPR::from_i128(d))
    }
    
    fn to_latex(&self) -> String {
        let mut terms = Vec::new();
        for (i, c) in self.coefficients.iter().enumerate().rev() {
            if c.is_zero() { continue; }
            let coeff_str = if c.denominator == 1 {
                if c.numerator.abs() == 1 && i > 0 {
                    if c.numerator < 0 { "-".into() } else { "".into() }
                } else {
                    format!("{}", c.numerator)
                }
            } else {
                format!("\\frac{{{}}}{{{}}}", c.numerator, c.denominator)
            };
            
            match i {
                0 => terms.push(coeff_str),
                1 => terms.push(format!("{}D", coeff_str)),
                _ => terms.push(format!("{}D^{}", coeff_str, i)),
            }
        }
        if terms.is_empty() { "0".into() } else { terms.join(" + ").replace("+ -", "- ") }
    }
}

// ============================================================================
// 核心多项式定义
// ============================================================================

/// P(D) = D⁴ - 2D³ + 2D + 1 (耦合多项式)
fn coupling_polynomial() -> RationalPolynomial {
    RationalPolynomial::from_integers(&[1, 2, 0, -2, 1])
}

/// Q(D) = 2D⁴ - 5D³ + 4D² + 15D + 99 (质量多项式)
fn mass_polynomial() -> RationalPolynomial {
    RationalPolynomial::from_integers(&[99, 15, 4, -5, 2])
}

/// I(D) = Q(D) - 2P(D) = -D³ + 4D² + 11D + 97 (残差多项式)
fn residual_polynomial() -> RationalPolynomial {
    RationalPolynomial::from_integers(&[97, 11, 4, -1, 0])
}

// ============================================================================
// 穷尽搜索引擎
// ============================================================================

#[derive(Debug, Clone)]
struct UnifiedRelation {
    formula: String,
    latex: String,
    lhs_value: EPR,
    rhs_value: EPR,
    residual: EPR,
    is_exact: bool,
    category: String,
}

/// 穷尽搜索有理数线性组合 aP(x) + bQ(y) = target
fn search_rational_combinations(
    target_name: &str,
    target: &EPR,
    output: &mut Vec<String>,
) -> Vec<UnifiedRelation> {
    let p = coupling_polynomial();
    let q = mass_polynomial();
    
    let mut results = Vec::new();
    let mut found_exact = false;
    
    output.push(format!("\n## Searching combinations for {} = {}\n", target_name, target));
    
    // 搜索范围：a, b ∈ [-100, 100], x, y ∈ [-20, 20]
    // 包括有理数系数 a/c, b/d where c,d ∈ {1,2,3,4,5,6}
    
    let denominators = [1i128, 2, 3, 4, 5, 6];
    
    for &denom_a in &denominators {
        for num_a in -50i128..=50 {
            let a = EPR::new(num_a, denom_a);
            if a.is_zero() { continue; }
            
            for x in -15i128..=15 {
                let p_x = p.evaluate_i128(x);
                let term_a = a.clone() * p_x.clone();
                
                for &denom_b in &denominators {
                    for num_b in -50i128..=50 {
                        let b = EPR::new(num_b, denom_b);
                        
                        for y in -15i128..=15 {
                            let q_y = q.evaluate_i128(y);
                            let term_b = b.clone() * q_y.clone();
                            let result = term_a.clone() + term_b.clone();
                            
                            // 检查是否精确匹配
                            if result == *target {
                                found_exact = true;
                                let formula = format!("{}*P({}) + {}*Q({}) = {}", a, x, b, y, target);
                                let latex = format!("{}P({}) + {}Q({}) = {}", a, x, b, y, target);
                                
                                output.push(format!("  EXACT: {}", formula));
                                
                                results.push(UnifiedRelation {
                                    formula,
                                    latex,
                                    lhs_value: result.clone(),
                                    rhs_value: target.clone(),
                                    residual: EPR::from_i128(0),
                                    is_exact: true,
                                    category: target_name.into(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    
    // 单项搜索: a*P(x) = target 或 b*Q(y) = target
    for &denom in &denominators {
        for num in -200i128..=200 {
            if num == 0 { continue; }
            let coeff = EPR::new(num, denom);
            
            for d in -20i128..=20 {
                let p_d = p.evaluate_i128(d);
                let result_p = coeff.clone() * p_d;
                if result_p == *target {
                    let formula = format!("{}*P({}) = {}", coeff, d, target);
                    output.push(format!("  EXACT (single P): {}", formula));
                    results.push(UnifiedRelation {
                        formula: formula.clone(),
                        latex: formula,
                        lhs_value: result_p,
                        rhs_value: target.clone(),
                        residual: EPR::from_i128(0),
                        is_exact: true,
                        category: target_name.into(),
                    });
                }
                
                let q_d = q.evaluate_i128(d);
                let result_q = coeff.clone() * q_d;
                if result_q == *target {
                    let formula = format!("{}*Q({}) = {}", coeff, d, target);
                    output.push(format!("  EXACT (single Q): {}", formula));
                    results.push(UnifiedRelation {
                        formula: formula.clone(),
                        latex: formula,
                        lhs_value: result_q,
                        rhs_value: target.clone(),
                        residual: EPR::from_i128(0),
                        is_exact: true,
                        category: target_name.into(),
                    });
                }
            }
        }
    }
    
    if !found_exact {
        output.push(format!("  No exact rational combination found for {}", target_name));
    }
    
    output.push(format!("  Found {} exact relations", results.len()));
    results
}

/// 搜索 α² 相关关系 (用于电弱粒子)
fn search_alpha_squared_relations(
    target_name: &str,
    target: &EPR,
    output: &mut Vec<String>,
) -> Vec<UnifiedRelation> {
    let p = coupling_polynomial();
    let q = mass_polynomial();
    let alpha_inv = EPR::from_i128(137); // 整数近似
    let alpha_inv_sq = alpha_inv.clone() * alpha_inv.clone(); // 137² = 18769
    
    let mut results = Vec::new();
    
    output.push(format!("\n## Searching α² relations for {} = {}\n", target_name, target));
    
    // 搜索 a*α⁻² + b*P(x) + c*Q(y) = target
    for a in 1i128..=30 {
        let term_alpha = EPR::from_i128(a) * alpha_inv_sq.clone();
        let remainder = target.clone() - term_alpha.clone();
        
        if remainder.is_integer() && remainder.numerator > 0 {
            // 检查remainder是否可以用P或Q表示
            for d in -15i128..=15 {
                let p_d = p.evaluate_i128(d);
                let q_d = q.evaluate_i128(d);
                
                // 检查 remainder = b*P(d)
                if !p_d.is_zero() {
                    let b_candidate = remainder.clone() / p_d.clone();
                    if b_candidate.is_integer() && b_candidate.numerator.abs() <= 200 {
                        let formula = format!("{}*α⁻² + {}*P({}) = {}", a, b_candidate, d, target);
                        output.push(format!("  EXACT: {}", formula));
                        results.push(UnifiedRelation {
                            formula: formula.clone(),
                            latex: format!("{}\\alpha^{{-2}} + {}P({}) = {}", a, b_candidate, d, target),
                            lhs_value: target.clone(),
                            rhs_value: target.clone(),
                            residual: EPR::from_i128(0),
                            is_exact: true,
                            category: target_name.into(),
                        });
                    }
                }
                
                // 检查 remainder = c*Q(d)
                if !q_d.is_zero() {
                    let c_candidate = remainder.clone() / q_d.clone();
                    if c_candidate.is_integer() && c_candidate.numerator.abs() <= 200 {
                        let formula = format!("{}*α⁻² + {}*Q({}) = {}", a, c_candidate, d, target);
                        output.push(format!("  EXACT: {}", formula));
                        results.push(UnifiedRelation {
                            formula: formula.clone(),
                            latex: format!("{}\\alpha^{{-2}} + {}Q({}) = {}", a, c_candidate, d, target),
                            lhs_value: target.clone(),
                            rhs_value: target.clone(),
                            residual: EPR::from_i128(0),
                            is_exact: true,
                            category: target_name.into(),
                        });
                    }
                }
            }
        }
    }
    
    output.push(format!("  Found {} α² relations", results.len()));
    results
}

/// 搜索质量比的有理数关系
fn search_mass_ratio_relations(output: &mut Vec<String>) -> Vec<UnifiedRelation> {
    let p = coupling_polynomial();
    let q = mass_polynomial();
    
    let mut results = Vec::new();
    
    output.push("\n## Mass Ratio Relations (Rational Coefficients)\n".into());
    
    // 质量比目标（整数近似）
    let targets = [
        ("m_mu/m_e", 207i128),
        ("m_tau/m_e", 3477i128),
        ("m_p/m_e", 1836i128),
        ("m_n/m_e", 1839i128),
        ("m_W/m_e", 157299i128),
        ("m_Z/m_e", 178450i128),
        ("m_H/m_e", 244954i128),
        ("m_t/m_e", 338962i128),
    ];
    
    for (name, target) in &targets {
        let target_epr = EPR::from_i128(*target);
        output.push(format!("\n### {} ≈ {}\n", name, target));
        
        // 搜索简单组合
        for d in -20i128..=20 {
            let p_d = p.evaluate_i128(d);
            let q_d = q.evaluate_i128(d);
            
            // P(d) = target?
            if p_d == target_epr {
                output.push(format!("  P({}) = {} EXACT", d, target));
                results.push(UnifiedRelation {
                    formula: format!("P({}) = {}", d, name),
                    latex: format!("P({}) = {}", d, name),
                    lhs_value: p_d.clone(),
                    rhs_value: target_epr.clone(),
                    residual: EPR::from_i128(0),
                    is_exact: true,
                    category: name.to_string(),
                });
            }
            
            // Q(d) = target?
            if q_d == target_epr {
                output.push(format!("  Q({}) = {} EXACT", d, target));
                results.push(UnifiedRelation {
                    formula: format!("Q({}) = {}", d, name),
                    latex: format!("Q({}) = {}", d, name),
                    lhs_value: q_d.clone(),
                    rhs_value: target_epr.clone(),
                    residual: EPR::from_i128(0),
                    is_exact: true,
                    category: name.to_string(),
                });
            }
        }
        
        // 搜索 a*P(x) + b*Q(y) = target (整数系数)
        for a in -20i128..=20 {
            for x in -10i128..=10 {
                let p_x = p.evaluate_i128(x);
                let term_a = EPR::from_i128(a) * p_x;
                
                for b in -20i128..=20 {
                    for y in -10i128..=10 {
                        let q_y = q.evaluate_i128(y);
                        let term_b = EPR::from_i128(b) * q_y;
                        let result = term_a.clone() + term_b.clone();
                        
                        if result == target_epr {
                            let formula = format!("{}*P({}) + {}*Q({}) = {}", a, x, b, y, name);
                            output.push(format!("  EXACT: {}", formula));
                            results.push(UnifiedRelation {
                                formula,
                                latex: format!("{}P({}) + {}Q({}) = {}", a, x, b, y, name),
                                lhs_value: result,
                                rhs_value: target_epr.clone(),
                                residual: EPR::from_i128(0),
                                is_exact: true,
                                category: name.to_string(),
                            });
                        }
                    }
                }
            }
        }
    }
    
    results
}

/// 验证核心代数恒等式
fn verify_core_identities(output: &mut Vec<String>) -> Vec<UnifiedRelation> {
    let p = coupling_polynomial();
    let q = mass_polynomial();
    let i = residual_polynomial();
    
    let mut results = Vec::new();
    
    output.push("\n================================================================================".into());
    output.push("                    CORE ALGEBRAIC IDENTITIES".into());
    output.push("================================================================================\n".into());
    
    // P(2) = Q(4) = 137
    let p2 = p.evaluate_i128(2);
    let q4 = q.evaluate_i128(4);
    let i137 = EPR::from_i128(137);
    
    output.push(format!("P(2) = {}", p2));
    output.push(format!("Q(4) = {}", q4));
    output.push(format!("P(2) = Q(4) = 137: {}", p2 == q4 && q4 == i137));
    
    if p2 == i137 {
        results.push(UnifiedRelation {
            formula: "P(2) = 137".into(),
            latex: "P(2) = 137".into(),
            lhs_value: p2.clone(),
            rhs_value: i137.clone(),
            residual: EPR::from_i128(0),
            is_exact: true,
            category: "core".into(),
        });
    }
    
    if q4 == i137 {
        results.push(UnifiedRelation {
            formula: "Q(4) = 137".into(),
            latex: "Q(4) = 137".into(),
            lhs_value: q4.clone(),
            rhs_value: i137.clone(),
            residual: EPR::from_i128(0),
            is_exact: true,
            category: "core".into(),
        });
    }
    
    // I(D) = Q(D) - 2P(D) 验证
    output.push("\n--- Residual Polynomial I(D) = Q(D) - 2P(D) ---\n".into());
    for d in -5i128..=10 {
        let p_d = p.evaluate_i128(d);
        let q_d = q.evaluate_i128(d);
        let i_d = i.evaluate_i128(d);
        let computed = q_d.clone() - EPR::from_i128(2) * p_d.clone();
        
        let verified = computed == i_d;
        output.push(format!("D={}: I({}) = {} | Q({}) - 2P({}) = {} | Verified: {}", 
            d, d, i_d, d, d, computed, verified));
    }
    
    // Fibonacci连接
    output.push("\n--- Fibonacci Connections ---\n".into());
    let fib = [1i128, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987];
    
    for d in 0i128..=10 {
        let p_d = p.evaluate_i128(d);
        for (idx, &f) in fib.iter().enumerate() {
            if p_d.numerator == f && p_d.denominator == 1 {
                output.push(format!("  P({}) = {} = F_{}", d, p_d, idx + 1));
            }
        }
    }
    
    results
}

/// 搜索新的有理数多项式族
fn search_new_polynomial_families(output: &mut Vec<String>) -> Vec<UnifiedRelation> {
    let mut results = Vec::new();
    
    output.push("\n================================================================================".into());
    output.push("               NEW POLYNOMIAL FAMILY SEARCH".into());
    output.push("================================================================================\n".into());
    
    // 搜索满足 P(a) = target 的新多项式
    // 形式: P(D) = c4*D⁴ + c3*D³ + c2*D² + c1*D + c0
    // 约束: P(4) = 137
    
    let target = 137i64;
    let d = 4i64;
    let d2 = d * d;
    let d3 = d2 * d;
    let d4 = d3 * d;
    
    output.push("Searching polynomials P(D) with P(4) = 137...\n".into());
    
    let mut count = 0;
    let mut special_polynomials = Vec::new();
    
    // 系数范围 [-5, 5]
    for c4 in -5i64..=5 {
        for c3 in -5i64..=5 {
            for c2 in -5i64..=5 {
                for c1 in -5i64..=5 {
                    let val = c4 * d4 + c3 * d3 + c2 * d2 + c1 * d;
                    let c0 = target - val;
                    
                    if c0.abs() <= 100 {
                        count += 1;
                        
                        // 检查这个多项式是否有其他有趣性质
                        let coeffs = vec![c0 as i128, c1 as i128, c2 as i128, c3 as i128, c4 as i128];
                        let poly = RationalPolynomial::from_integers(&coeffs);
                        
                        // 检查 P(2) 和 P(3)
                        let p2 = poly.evaluate_i128(2);
                        let p3 = poly.evaluate_i128(3);
                        
                        // 检查是否和物理常数相关
                        let is_interesting = 
                            p2.numerator == 137 ||  // P(2) = 137
                            p3.numerator == 207 ||  // mμ/me ≈ 207
                            (p2.numerator > 0 && p3.numerator > 0 && 
                             p2.numerator < 200 && p3.numerator < 300);
                        
                        if is_interesting && c4 != 0 {
                            special_polynomials.push((coeffs.clone(), p2.clone(), p3.clone()));
                            output.push(format!(
                                "  Poly[{},{},{},{},{}]: P(2)={}, P(3)={}, P(4)=137",
                                c0, c1, c2, c3, c4, p2, p3
                            ));
                        }
                    }
                }
            }
        }
    }
    
    output.push(format!("\nTotal polynomials with P(4)=137: {}", count));
    output.push(format!("Interesting polynomials found: {}", special_polynomials.len()));
    
    // 特别标记我们的原始多项式
    let original_p = coupling_polynomial();
    let orig_p2 = original_p.evaluate_i128(2);
    let orig_p4 = original_p.evaluate_i128(4);
    output.push(format!("\nOriginal P(D) = D⁴ - 2D³ + 2D + 1: P(2)={}, P(4)={}", orig_p2, orig_p4));
    
    results
}

/// 主函数：运行完整的有理数统一探索
fn main() {
    println!("================================================================================");
    println!("     S-WIND RATIONAL UNIFIED THEORY EXPLORATION");
    println!("     有理数大一统理论穷尽探索");
    println!("================================================================================\n");
    
    let mut output: Vec<String> = Vec::new();
    let mut all_relations: Vec<UnifiedRelation> = Vec::new();
    
    output.push("# S-wind Rational Unified Exploration Report\n".into());
    output.push("## 有理数大一统理论穷尽探索报告\n".into());
    output.push("All computations use exact rational arithmetic (EPR).\n".into());
    
    // 加载物理常数
    let constants = PhysicsConstants::new();
    
    // 1. 验证核心恒等式
    println!("Phase 1: Verifying core identities...");
    let core_results = verify_core_identities(&mut output);
    all_relations.extend(core_results);
    
    // 2. 搜索质量比关系
    println!("Phase 2: Searching mass ratio relations...");
    let mass_results = search_mass_ratio_relations(&mut output);
    all_relations.extend(mass_results);
    
    // 3. 搜索整数137的组合
    println!("Phase 3: Searching combinations for α⁻¹ ≈ 137...");
    let alpha_results = search_rational_combinations(
        "α⁻¹",
        &EPR::from_i128(137),
        &mut output
    );
    all_relations.extend(alpha_results);
    
    // 4. 搜索电弱粒子的 α² 关系
    println!("Phase 4: Searching α² relations for electroweak particles...");
    let ew_masses = [
        ("m_W/m_e", 157299i128),
        ("m_Z/m_e", 178450i128),
        ("m_H/m_e", 244954i128),
        ("m_t/m_e", 338962i128),
    ];
    
    for (name, val) in &ew_masses {
        let ew_results = search_alpha_squared_relations(name, &EPR::from_i128(*val), &mut output);
        all_relations.extend(ew_results);
    }
    
    // 5. 搜索新多项式族
    println!("Phase 5: Searching new polynomial families...");
    let new_poly_results = search_new_polynomial_families(&mut output);
    all_relations.extend(new_poly_results);
    
    // 6. 生成统计摘要
    output.push("\n================================================================================".into());
    output.push("                         SUMMARY".into());
    output.push("================================================================================\n".into());
    
    let exact_count = all_relations.iter().filter(|r| r.is_exact).count();
    output.push(format!("Total exact relations found: {}", exact_count));
    
    // 按类别统计
    let mut category_counts: HashMap<String, usize> = HashMap::new();
    for r in &all_relations {
        *category_counts.entry(r.category.clone()).or_insert(0) += 1;
    }
    
    output.push("\nRelations by category:".into());
    for (cat, count) in &category_counts {
        output.push(format!("  {}: {}", cat, count));
    }
    
    // 7. 输出到文件
    let report_content = output.join("\n");
    println!("\n{}", report_content);
    
    // 保存到文件
    let mut file = File::create("/home/jouly/AI/Valhalla/s-wind-sim/output/rational_unified_exploration.txt")
        .expect("Cannot create output file");
    file.write_all(report_content.as_bytes()).expect("Cannot write to file");
    
    println!("\n================================================================================");
    println!("Report saved to: output/rational_unified_exploration.txt");
    println!("================================================================================");
    
    // 输出关键发现的LaTeX
    if !all_relations.is_empty() {
        println!("\n=== KEY EXACT RELATIONS (LaTeX) ===\n");
        for (i, r) in all_relations.iter().filter(|r| r.is_exact).take(20).enumerate() {
            println!("{}. ${}$", i + 1, r.latex);
        }
    }
}
