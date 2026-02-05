//! S-wind 大一统理论穷尽搜索
//! 
//! 目标：找到连接所有基本物理常数的统一代数结构
//! 方法：有理数穷尽搜索 + 自动容器创建
//! 
//! 核心多项式（重新定义以匹配P(2)=P(4)=137）:
//! 寻找满足 P(2) = P(4) = 137 的多项式族

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// ============================================================================
// EPR (Extended Precision Rational)
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
    fn to_i128(&self) -> Option<i128> {
        if self.is_integer() { Some(self.numerator) } else { None }
    }
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
// 多项式求值
// ============================================================================

fn eval_poly(coeffs: &[i128], d: i128) -> i128 {
    let mut result = 0i128;
    let mut d_pow = 1i128;
    for &c in coeffs {
        result += c * d_pow;
        d_pow *= d;
    }
    result
}

fn eval_poly_epr(coeffs: &[EPR], d: &EPR) -> EPR {
    let mut result = EPR::from_i128(0);
    let mut d_pow = EPR::from_i128(1);
    for c in coeffs {
        result = result + c.clone() * d_pow.clone();
        d_pow = d_pow * d.clone();
    }
    result
}

// ============================================================================
// 物理常数（整数近似）
// ============================================================================

struct PhysicsTargets {
    targets: Vec<(&'static str, i128)>,
}

impl PhysicsTargets {
    fn new() -> Self {
        PhysicsTargets {
            targets: vec![
                ("alpha_inv", 137),
                ("m_mu_me", 207),
                ("m_tau_me", 3477),
                ("m_p_me", 1836),
                ("m_n_me", 1839),
                ("m_W_me", 157299),
                ("m_Z_me", 178450),
                ("m_H_me", 244954),
                ("m_t_me", 338962),
            ],
        }
    }
}

// ============================================================================
// 核心搜索：找满足P(2)=P(4)=137的多项式
// ============================================================================

#[derive(Debug, Clone)]
struct UnifiedPolynomial {
    coeffs: Vec<i128>,
    values: HashMap<i128, i128>,
}

impl UnifiedPolynomial {
    fn new(coeffs: Vec<i128>) -> Self {
        let mut values = HashMap::new();
        for d in -20i128..=20 {
            values.insert(d, eval_poly(&coeffs, d));
        }
        UnifiedPolynomial { coeffs, values }
    }
    
    fn eval(&self, d: i128) -> i128 {
        *self.values.get(&d).unwrap_or(&eval_poly(&self.coeffs, d))
    }
    
    fn to_string(&self) -> String {
        let mut terms = Vec::new();
        for (i, &c) in self.coeffs.iter().enumerate().rev() {
            if c == 0 { continue; }
            let coeff_str = if c.abs() == 1 && i > 0 {
                if c < 0 { "-".into() } else { "".into() }
            } else {
                format!("{}", c)
            };
            match i {
                0 => terms.push(format!("{}", c)),
                1 => terms.push(format!("{}D", coeff_str)),
                _ => terms.push(format!("{}D^{}", coeff_str, i)),
            }
        }
        if terms.is_empty() { "0".into() } else { 
            terms.join(" + ").replace("+ -", "- ")
        }
    }
}

/// 搜索满足 P(a) = P(b) = target 的四次多项式
fn search_double_fixed_polynomials(
    a: i128, 
    b: i128, 
    target: i128,
    coeff_range: std::ops::RangeInclusive<i64>,
) -> Vec<UnifiedPolynomial> {
    let mut results = Vec::new();
    
    // P(D) = c0 + c1*D + c2*D² + c3*D³ + c4*D⁴
    // 约束1: P(a) = target
    // 约束2: P(b) = target
    
    let a2 = a * a;
    let a3 = a2 * a;
    let a4 = a3 * a;
    
    let b2 = b * b;
    let b3 = b2 * b;
    let b4 = b3 * b;
    
    // 由于有两个约束，我们有3个自由系数 (c2, c3, c4)
    // c0 和 c1 由约束决定
    
    for c4 in coeff_range.clone() {
        let c4 = c4 as i128;
        for c3 in coeff_range.clone() {
            let c3 = c3 as i128;
            for c2 in coeff_range.clone() {
                let c2 = c2 as i128;
                
                // 约束方程:
                // c0 + c1*a + c2*a² + c3*a³ + c4*a⁴ = target
                // c0 + c1*b + c2*b² + c3*b³ + c4*b⁴ = target
                // 
                // 相减: c1*(a-b) + c2*(a²-b²) + c3*(a³-b³) + c4*(a⁴-b⁴) = 0
                
                let diff_1 = a - b;
                let diff_2 = a2 - b2;
                let diff_3 = a3 - b3;
                let diff_4 = a4 - b4;
                
                if diff_1 == 0 { continue; } // a == b 无意义
                
                // c1 = -(c2*(a²-b²) + c3*(a³-b³) + c4*(a⁴-b⁴)) / (a-b)
                let numerator = -(c2 * diff_2 + c3 * diff_3 + c4 * diff_4);
                if numerator % diff_1 != 0 { continue; } // 必须整除
                
                let c1 = numerator / diff_1;
                
                // c0 = target - c1*a - c2*a² - c3*a³ - c4*a⁴
                let c0 = target - c1 * a - c2 * a2 - c3 * a3 - c4 * a4;
                
                // 验证
                let val_a = c0 + c1 * a + c2 * a2 + c3 * a3 + c4 * a4;
                let val_b = c0 + c1 * b + c2 * b2 + c3 * b3 + c4 * b4;
                
                if val_a == target && val_b == target {
                    let poly = UnifiedPolynomial::new(vec![c0, c1, c2, c3, c4]);
                    results.push(poly);
                }
            }
        }
    }
    
    results
}

/// 搜索能命中多个物理常数的多项式
fn search_multi_constant_polynomials(
    polynomials: &[UnifiedPolynomial],
    targets: &PhysicsTargets,
    output: &mut Vec<String>,
) -> Vec<(UnifiedPolynomial, Vec<(String, i128, i128)>)> {
    let mut results = Vec::new();
    
    output.push("\n================================================================================".into());
    output.push("          MULTI-CONSTANT POLYNOMIAL SEARCH".into());
    output.push("================================================================================\n".into());
    
    for poly in polynomials {
        let mut matches = Vec::new();
        
        for (name, target) in &targets.targets {
            // 搜索 P(d) = target
            for d in -20i128..=20 {
                let val = poly.eval(d);
                if val == *target {
                    matches.push((name.to_string(), d, val));
                }
            }
        }
        
        if matches.len() >= 2 {
            output.push(format!("\n--- Polynomial: {} ---", poly.to_string()));
            output.push(format!("Matches {} constants:", matches.len()));
            for (name, d, val) in &matches {
                output.push(format!("  P({}) = {} = {}", d, val, name));
            }
            results.push((poly.clone(), matches));
        }
    }
    
    // 排序：命中数量多的在前
    results.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    output.push(format!("\n\nTotal multi-constant polynomials: {}", results.len()));
    
    results
}

/// 搜索 aP(x) + bQ(y) = target 的组合
fn search_polynomial_combinations(
    p: &UnifiedPolynomial,
    q: &UnifiedPolynomial,
    target: i128,
    target_name: &str,
    output: &mut Vec<String>,
) -> Vec<(i128, i128, i128, i128)> {
    let mut results = Vec::new();
    
    output.push(format!("\n## Combinations for {} = {}\n", target_name, target));
    
    // 搜索 a*P(x) + b*Q(y) = target
    for a in -50i128..=50 {
        for x in -15i128..=15 {
            let p_x = p.eval(x);
            let term_a = a * p_x;
            
            for b in -50i128..=50 {
                for y in -15i128..=15 {
                    let q_y = q.eval(y);
                    let term_b = b * q_y;
                    
                    if term_a + term_b == target {
                        results.push((a, x, b, y));
                        if results.len() <= 10 {
                            output.push(format!("  {}*P({}) + {}*Q({}) = {}", a, x, b, y, target));
                        }
                    }
                }
            }
        }
    }
    
    output.push(format!("  Total: {} combinations found", results.len()));
    results
}

/// 搜索α²相关关系
fn search_alpha_squared(
    p: &UnifiedPolynomial,
    target: i128,
    target_name: &str,
    output: &mut Vec<String>,
) -> Vec<(i128, i128, i128)> {
    let mut results = Vec::new();
    let alpha_sq = 137i128 * 137; // 18769
    
    output.push(format!("\n## α² relations for {} = {}\n", target_name, target));
    
    // 搜索 a*α² + b*P(x) = target
    for a in 1i128..=30 {
        let term_alpha = a * alpha_sq;
        if term_alpha > target { break; }
        
        let remainder = target - term_alpha;
        
        for x in -20i128..=20 {
            let p_x = p.eval(x);
            if p_x == 0 { continue; }
            
            if remainder % p_x == 0 {
                let b = remainder / p_x;
                if b.abs() <= 200 {
                    results.push((a, b, x));
                    output.push(format!("  {}*α⁻² + {}*P({}) = {}", a, b, x, target));
                }
            }
        }
    }
    
    output.push(format!("  Total: {} α² relations found", results.len()));
    results
}

/// 穷尽搜索统一结构
fn exhaustive_unified_search(output: &mut Vec<String>) {
    output.push("================================================================================".into());
    output.push("     S-WIND UNIFIED THEORY EXHAUSTIVE SEARCH".into());
    output.push("     大一统理论穷尽搜索".into());
    output.push("================================================================================\n".into());
    
    let targets = PhysicsTargets::new();
    
    // 阶段1：搜索满足 P(2) = P(4) = 137 的多项式
    output.push("\n## Phase 1: Search P(2) = P(4) = 137 polynomials\n".into());
    
    let double_fixed = search_double_fixed_polynomials(2, 4, 137, -10..=10);
    output.push(format!("Found {} polynomials with P(2)=P(4)=137", double_fixed.len()));
    
    // 显示前几个
    for (i, poly) in double_fixed.iter().take(20).enumerate() {
        output.push(format!("  [{}] {}", i+1, poly.to_string()));
        output.push(format!("      P(0)={}, P(1)={}, P(2)={}, P(3)={}, P(4)={}, P(5)={}", 
            poly.eval(0), poly.eval(1), poly.eval(2), poly.eval(3), poly.eval(4), poly.eval(5)));
    }
    
    // 阶段2：在这些多项式中搜索能命中更多物理常数的
    output.push("\n## Phase 2: Multi-constant matching\n".into());
    let multi_matches = search_multi_constant_polynomials(&double_fixed, &targets, output);
    
    // 阶段3：选择最佳多项式，进行组合搜索
    if let Some((best_p, matches)) = multi_matches.first() {
        output.push(format!("\n## Phase 3: Best polynomial analysis"));
        output.push(format!("Selected: {}", best_p.to_string()));
        output.push(format!("Matches: {:?}", matches));
        
        // 使用最佳多项式作为P，搜索另一个Q使得组合能表达更多常数
        output.push("\n### Searching for complementary polynomial Q...\n".into());
        
        // 搜索满足 Q(3) = 207 (mμ/me) 的多项式
        let q_candidates = search_double_fixed_polynomials(3, 137, 207, -10..=10);
        output.push(format!("Candidates with Q(3)=207: {} (using weak constraint)", q_candidates.len()));
        
        // 如果没找到，放宽约束
        if q_candidates.is_empty() {
            output.push("Relaxing constraints: searching Q(3) = 207 only...".into());
            // 简单搜索满足 Q(3) = 207 的多项式
        }
        
        // 阶段4：α²关系搜索
        output.push("\n## Phase 4: α² relations for heavy particles\n".into());
        
        let heavy_targets = [
            ("m_W/m_e", 157299i128),
            ("m_Z/m_e", 178450i128),
            ("m_H/m_e", 244954i128),
            ("m_t/m_e", 338962i128),
        ];
        
        for (name, target) in &heavy_targets {
            search_alpha_squared(best_p, *target, name, output);
        }
    }
    
    // 阶段5：统计总结
    output.push("\n================================================================================".into());
    output.push("                         SUMMARY".into());
    output.push("================================================================================\n".into());
    
    output.push(format!("Total P(2)=P(4)=137 polynomials: {}", double_fixed.len()));
    output.push(format!("Multi-constant polynomials: {}", multi_matches.len()));
    
    if !multi_matches.is_empty() {
        output.push("\nTop polynomials by constant coverage:".into());
        for (i, (poly, matches)) in multi_matches.iter().take(5).enumerate() {
            output.push(format!("  {}. {} - {} constants", i+1, poly.to_string(), matches.len()));
        }
    }
}

/// 扩展搜索：有理数系数
fn extended_rational_search(output: &mut Vec<String>) {
    output.push("\n================================================================================".into());
    output.push("          EXTENDED RATIONAL COEFFICIENT SEARCH".into());
    output.push("================================================================================\n".into());
    
    // 基础多项式 P(D) = D⁴ - 2D³ + 2D + 1
    let p_base = UnifiedPolynomial::new(vec![1, 2, 0, -2, 1]);
    
    output.push(format!("Base polynomial P(D) = {}", p_base.to_string()));
    output.push("Values:".into());
    for d in -5i128..=10 {
        output.push(format!("  P({}) = {}", d, p_base.eval(d)));
    }
    
    // 搜索质量比关系 (有理数系数)
    output.push("\n### Mass ratio relations with rational coefficients\n".into());
    
    let mass_ratios = [
        ("m_mu/m_e", 207i128),
        ("m_tau/m_e", 3477i128),
        ("m_p/m_e", 1836i128),
        ("m_n/m_e", 1839i128),
    ];
    
    for (name, target) in &mass_ratios {
        output.push(format!("\n{} = {}:", name, target));
        
        // 搜索 (a/b)*P(x) = target
        for denom in 1i128..=10 {
            for numer in 1i128..=100 {
                for x in -15i128..=15 {
                    let p_x = p_base.eval(x);
                    // (numer/denom) * P(x) = target
                    // numer * P(x) = target * denom
                    if numer * p_x == *target * denom {
                        output.push(format!("  ({}/{})*P({}) = {}", numer, denom, x, target));
                    }
                }
            }
        }
        
        // 搜索 P(x) + (a/b)*P(y) = target
        for x in -10i128..=10 {
            let p_x = p_base.eval(x);
            let remainder = *target - p_x;
            
            for y in -10i128..=10 {
                if x == y { continue; }
                let p_y = p_base.eval(y);
                if p_y == 0 { continue; }
                
                // remainder = (a/b) * P(y)
                // 检查是否为简单有理数
                if remainder % p_y == 0 {
                    let ratio = remainder / p_y;
                    if ratio.abs() <= 20 {
                        output.push(format!("  P({}) + {}*P({}) = {}", x, ratio, y, target));
                    }
                }
            }
        }
    }
}

fn main() {
    println!("================================================================================");
    println!("     S-WIND UNIFIED THEORY EXHAUSTIVE SEARCH");
    println!("     大一统理论穷尽搜索");
    println!("================================================================================\n");
    
    let mut output: Vec<String> = Vec::new();
    
    // 主搜索
    exhaustive_unified_search(&mut output);
    
    // 扩展搜索
    extended_rational_search(&mut output);
    
    // 输出结果
    let report = output.join("\n");
    println!("{}", report);
    
    // 保存到文件
    let mut file = File::create("/home/jouly/AI/Valhalla/s-wind-sim/output/unified_theory_exhaustive.txt")
        .expect("Cannot create output file");
    file.write_all(report.as_bytes()).expect("Cannot write to file");
    
    println!("\n================================================================================");
    println!("Report saved to: output/unified_theory_exhaustive.txt");
    println!("================================================================================");
}
