//! Algebraic Number System for S-wind
//!
//! 代数优先原则：
//! 1. 所有计算尽可能保持符号形式
//! 2. 只在最终验证时才转换为数值
//! 3. 区分：精确代数 vs 数值近似
//!
//! 支持：
//! - 有理数 Q
//! - 代数数 (√n, 黄金比 φ, etc.)
//! - 超越数 (π, e, ln(n))
//! - 符号表达式组合

use super::Rational;
use std::fmt;

// ============================================================================
// 代数基 (Algebraic Base) - 不可约的代数/超越常数
// ============================================================================

/// 代数基类型 - 构建代数表达式的原子单元
#[derive(Debug, Clone, PartialEq)]
pub enum AlgebraicBase {
    /// 有理数 (精确)
    Rational(Rational),
    
    /// 平方根 √n (代数数, n 必须是非完全平方数)
    Sqrt(i128),
    
    /// n 次根 ⁿ√m
    NthRoot { radicand: i128, index: u32 },
    
    /// 黄金比 φ = (1+√5)/2
    GoldenRatio,
    
    /// 圆周率 π (超越数)
    Pi,
    
    /// 自然常数 e (超越数)
    E,
    
    /// 自然对数 ln(n)
    Ln(i128),
    
    /// Euler-Mascheroni 常数 γ
    EulerGamma,
    
    /// 精细结构常数 α (实验值，非代数)
    Alpha,
    
    /// 精细结构常数的逆 α⁻¹
    AlphaInv,
}

impl AlgebraicBase {
    /// 获取数值近似 (仅用于验证)
    pub fn to_f64(&self) -> f64 {
        match self {
            AlgebraicBase::Rational(r) => r.numerator as f64 / r.denominator as f64,
            AlgebraicBase::Sqrt(n) => (*n as f64).sqrt(),
            AlgebraicBase::NthRoot { radicand, index } => {
                (*radicand as f64).powf(1.0 / *index as f64)
            }
            AlgebraicBase::GoldenRatio => (1.0 + 5.0_f64.sqrt()) / 2.0,
            AlgebraicBase::Pi => std::f64::consts::PI,
            AlgebraicBase::E => std::f64::consts::E,
            AlgebraicBase::Ln(n) => (*n as f64).ln(),
            AlgebraicBase::EulerGamma => 0.5772156649015329,
            AlgebraicBase::Alpha => 1.0 / 137.035999177,
            AlgebraicBase::AlphaInv => 137.035999177,
        }
    }
    
    /// 是否是精确代数数 (vs 超越数/实验值)
    pub fn is_algebraic(&self) -> bool {
        matches!(self, 
            AlgebraicBase::Rational(_) | 
            AlgebraicBase::Sqrt(_) | 
            AlgebraicBase::NthRoot { .. } | 
            AlgebraicBase::GoldenRatio
        )
    }
    
    /// 是否是超越数
    pub fn is_transcendental(&self) -> bool {
        matches!(self, 
            AlgebraicBase::Pi | 
            AlgebraicBase::E | 
            AlgebraicBase::Ln(_) |
            AlgebraicBase::EulerGamma
        )
    }
    
    /// 是否是实验测量值
    pub fn is_experimental(&self) -> bool {
        matches!(self, AlgebraicBase::Alpha | AlgebraicBase::AlphaInv)
    }
    
    /// 转换为 LaTeX
    pub fn to_latex(&self) -> String {
        match self {
            AlgebraicBase::Rational(r) => {
                if r.denominator == 1 {
                    format!("{}", r.numerator)
                } else {
                    format!("\\frac{{{}}}{{{}}}", r.numerator, r.denominator)
                }
            }
            AlgebraicBase::Sqrt(n) => format!("\\sqrt{{{}}}", n),
            AlgebraicBase::NthRoot { radicand, index } => {
                format!("\\sqrt[{}]{{{}}}", index, radicand)
            }
            AlgebraicBase::GoldenRatio => "\\phi".to_string(),
            AlgebraicBase::Pi => "\\pi".to_string(),
            AlgebraicBase::E => "e".to_string(),
            AlgebraicBase::Ln(n) => format!("\\ln {}", n),
            AlgebraicBase::EulerGamma => "\\gamma".to_string(),
            AlgebraicBase::Alpha => "\\alpha".to_string(),
            AlgebraicBase::AlphaInv => "\\alpha^{-1}".to_string(),
        }
    }
}

impl fmt::Display for AlgebraicBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlgebraicBase::Rational(r) => write!(f, "{}", r),
            AlgebraicBase::Sqrt(n) => write!(f, "√{}", n),
            AlgebraicBase::NthRoot { radicand, index } => write!(f, "{}√{}", index, radicand),
            AlgebraicBase::GoldenRatio => write!(f, "φ"),
            AlgebraicBase::Pi => write!(f, "π"),
            AlgebraicBase::E => write!(f, "e"),
            AlgebraicBase::Ln(n) => write!(f, "ln({})", n),
            AlgebraicBase::EulerGamma => write!(f, "γ"),
            AlgebraicBase::Alpha => write!(f, "α"),
            AlgebraicBase::AlphaInv => write!(f, "α⁻¹"),
        }
    }
}

// ============================================================================
// 代数项 (Algebraic Term) - 系数 × 基的幂次
// ============================================================================

/// 代数项: coefficient × base^power
/// 例如: 3π², -2√5, 5/7·e
#[derive(Debug, Clone)]
pub struct AlgebraicTerm {
    /// 有理数系数
    pub coefficient: Rational,
    /// 代数基
    pub base: AlgebraicBase,
    /// 整数幂次 (可以是负数)
    pub power: i32,
}

impl AlgebraicTerm {
    /// 创建新项
    pub fn new(coefficient: Rational, base: AlgebraicBase, power: i32) -> Self {
        AlgebraicTerm { coefficient, base, power }
    }
    
    /// 从整数创建
    pub fn from_int(n: i128) -> Self {
        AlgebraicTerm {
            coefficient: Rational::from_i128(n),
            base: AlgebraicBase::Rational(Rational::one()),
            power: 1,
        }
    }
    
    /// 从有理数创建
    pub fn from_rational(r: Rational) -> Self {
        AlgebraicTerm {
            coefficient: r,
            base: AlgebraicBase::Rational(Rational::one()),
            power: 1,
        }
    }
    
    /// 创建 n·√m 形式
    pub fn sqrt_multiple(coefficient: i128, radicand: i128) -> Self {
        AlgebraicTerm {
            coefficient: Rational::from_i128(coefficient),
            base: AlgebraicBase::Sqrt(radicand),
            power: 1,
        }
    }
    
    /// 创建 n·π^k 形式
    pub fn pi_power(coefficient: i128, power: i32) -> Self {
        AlgebraicTerm {
            coefficient: Rational::from_i128(coefficient),
            base: AlgebraicBase::Pi,
            power,
        }
    }
    
    /// 创建 n·e^k 形式
    pub fn e_power(coefficient: i128, power: i32) -> Self {
        AlgebraicTerm {
            coefficient: Rational::from_i128(coefficient),
            base: AlgebraicBase::E,
            power,
        }
    }
    
    /// 创建 n·α⁻¹ 形式
    pub fn alpha_inv_multiple(coefficient: i128) -> Self {
        AlgebraicTerm {
            coefficient: Rational::from_i128(coefficient),
            base: AlgebraicBase::AlphaInv,
            power: 1,
        }
    }
    
    /// 创建 n·α⁻² 形式
    pub fn alpha_inv_sq_multiple(coefficient: i128) -> Self {
        AlgebraicTerm {
            coefficient: Rational::from_i128(coefficient),
            base: AlgebraicBase::AlphaInv,
            power: 2,
        }
    }
    
    /// 是否是零
    pub fn is_zero(&self) -> bool {
        self.coefficient.is_zero()
    }
    
    /// 数值近似
    pub fn to_f64(&self) -> f64 {
        let coeff = self.coefficient.numerator as f64 / self.coefficient.denominator as f64;
        let base_val = self.base.to_f64();
        coeff * base_val.powi(self.power)
    }
    
    /// 转换为 LaTeX
    pub fn to_latex(&self) -> String {
        let coeff_str = if self.coefficient == Rational::one() {
            String::new()
        } else if self.coefficient == Rational::new(-1, 1) {
            "-".to_string()
        } else if self.coefficient.denominator == 1 {
            format!("{}", self.coefficient.numerator)
        } else {
            format!("\\frac{{{}}}{{{}}}", self.coefficient.numerator, self.coefficient.denominator)
        };
        
        let base_str = self.base.to_latex();
        
        if self.power == 0 {
            if coeff_str.is_empty() { "1".to_string() } else { coeff_str }
        } else if self.power == 1 {
            format!("{}{}", coeff_str, base_str)
        } else {
            format!("{}{}^{{{}}}", coeff_str, base_str, self.power)
        }
    }
}

impl fmt::Display for AlgebraicTerm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.coefficient.is_zero() {
            return write!(f, "0");
        }
        
        let coeff_str = if self.coefficient == Rational::one() {
            String::new()
        } else if self.coefficient == Rational::new(-1, 1) {
            "-".to_string()
        } else {
            format!("{}", self.coefficient)
        };
        
        if self.power == 0 {
            if coeff_str.is_empty() { write!(f, "1") } else { write!(f, "{}", coeff_str) }
        } else if self.power == 1 {
            write!(f, "{}·{}", coeff_str, self.base)
        } else {
            write!(f, "{}·{}^{}", coeff_str, self.base, self.power)
        }
    }
}

// ============================================================================
// 代数表达式 (Algebraic Expression) - 项的和
// ============================================================================

/// 代数表达式: 多个代数项的和
/// 例如: 3 + 2π - √5 + 4e²
#[derive(Debug, Clone)]
pub struct AlgebraicExpr {
    /// 项列表
    pub terms: Vec<AlgebraicTerm>,
}

impl AlgebraicExpr {
    /// 创建空表达式 (=0)
    pub fn zero() -> Self {
        AlgebraicExpr { terms: Vec::new() }
    }
    
    /// 从单个项创建
    pub fn from_term(term: AlgebraicTerm) -> Self {
        if term.is_zero() {
            Self::zero()
        } else {
            AlgebraicExpr { terms: vec![term] }
        }
    }
    
    /// 从整数创建
    pub fn from_int(n: i128) -> Self {
        if n == 0 {
            Self::zero()
        } else {
            Self::from_term(AlgebraicTerm::from_int(n))
        }
    }
    
    /// 从有理数创建
    pub fn from_rational(r: Rational) -> Self {
        if r.is_zero() {
            Self::zero()
        } else {
            Self::from_term(AlgebraicTerm::from_rational(r))
        }
    }
    
    /// 添加项
    pub fn add_term(&mut self, term: AlgebraicTerm) {
        if !term.is_zero() {
            // TODO: 合并同类项
            self.terms.push(term);
        }
    }
    
    /// 表达式相加
    pub fn add(&self, other: &AlgebraicExpr) -> AlgebraicExpr {
        let mut result = self.clone();
        for term in &other.terms {
            result.add_term(term.clone());
        }
        result.simplify()
    }
    
    /// 简化（合并同类项）
    pub fn simplify(&self) -> AlgebraicExpr {
        // TODO: 实现同类项合并
        self.clone()
    }
    
    /// 是否是零
    pub fn is_zero(&self) -> bool {
        self.terms.is_empty() || self.terms.iter().all(|t| t.is_zero())
    }
    
    /// 是否是纯有理数
    pub fn is_rational(&self) -> bool {
        self.terms.iter().all(|t| {
            matches!(t.base, AlgebraicBase::Rational(_)) || t.power == 0
        })
    }
    
    /// 是否包含超越数
    pub fn has_transcendental(&self) -> bool {
        self.terms.iter().any(|t| t.base.is_transcendental() && t.power != 0)
    }
    
    /// 是否包含实验值
    pub fn has_experimental(&self) -> bool {
        self.terms.iter().any(|t| t.base.is_experimental() && t.power != 0)
    }
    
    /// 尝试精确求值为有理数 (仅当表达式是纯有理数时)
    pub fn try_eval_exact(&self) -> Option<Rational> {
        if !self.is_rational() {
            return None;
        }
        
        let mut sum = Rational::zero();
        for term in &self.terms {
            if let AlgebraicBase::Rational(r) = &term.base {
                if term.power == 0 {
                    sum = sum + term.coefficient;
                } else if term.power == 1 {
                    sum = sum + term.coefficient * *r;
                } else {
                    // 有理数的整数幂
                    let mut power_val = Rational::one();
                    for _ in 0..term.power.abs() {
                        power_val = power_val * *r;
                    }
                    if term.power < 0 {
                        power_val = Rational::new(power_val.denominator, power_val.numerator);
                    }
                    sum = sum + term.coefficient * power_val;
                }
            }
        }
        Some(sum)
    }
    
    /// 数值近似 (用于验证)
    pub fn to_f64(&self) -> f64 {
        self.terms.iter().map(|t| t.to_f64()).sum()
    }
    
    /// 转换为 LaTeX
    pub fn to_latex(&self) -> String {
        if self.terms.is_empty() {
            return "0".to_string();
        }
        
        let mut parts = Vec::new();
        for (i, term) in self.terms.iter().enumerate() {
            let term_latex = term.to_latex();
            if i == 0 {
                parts.push(term_latex);
            } else if term.coefficient.numerator < 0 {
                // 负数，减号已在 to_latex 中
                parts.push(term_latex);
            } else {
                parts.push(format!("+ {}", term_latex));
            }
        }
        parts.join(" ")
    }
    
    /// 复杂度评分 (越低越好)
    pub fn complexity(&self) -> u32 {
        let mut score = 0u32;
        for term in &self.terms {
            // 系数复杂度
            score += (term.coefficient.numerator.abs() as u32).min(100);
            if term.coefficient.denominator != 1 {
                score += 10;
            }
            // 基的复杂度
            score += match &term.base {
                AlgebraicBase::Rational(_) => 0,
                AlgebraicBase::Sqrt(_) => 5,
                AlgebraicBase::NthRoot { .. } => 10,
                AlgebraicBase::GoldenRatio => 3,
                AlgebraicBase::Pi => 2,
                AlgebraicBase::E => 2,
                AlgebraicBase::Ln(_) => 8,
                AlgebraicBase::EulerGamma => 15,
                AlgebraicBase::Alpha | AlgebraicBase::AlphaInv => 1,
            };
            // 幂次复杂度
            score += (term.power.abs() as u32).saturating_sub(1) * 3;
        }
        score
    }
}

impl fmt::Display for AlgebraicExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.terms.is_empty() {
            return write!(f, "0");
        }
        
        for (i, term) in self.terms.iter().enumerate() {
            if i == 0 {
                write!(f, "{}", term)?;
            } else if term.coefficient.numerator < 0 {
                write!(f, " {}", term)?;
            } else {
                write!(f, " + {}", term)?;
            }
        }
        Ok(())
    }
}

// ============================================================================
// 代数表达式构建器
// ============================================================================

/// 代数表达式构建器 - 便捷 API
pub struct AlgebraicBuilder;

impl AlgebraicBuilder {
    /// a + b·π
    pub fn a_plus_b_pi(a: i128, b: i128) -> AlgebraicExpr {
        let mut expr = AlgebraicExpr::from_int(a);
        expr.add_term(AlgebraicTerm::pi_power(b, 1));
        expr
    }
    
    /// a + b·π + c·π²
    pub fn quadratic_pi(a: i128, b: i128, c: i128) -> AlgebraicExpr {
        let mut expr = AlgebraicExpr::from_int(a);
        expr.add_term(AlgebraicTerm::pi_power(b, 1));
        expr.add_term(AlgebraicTerm::pi_power(c, 2));
        expr
    }
    
    /// a + b·e + c·e²
    pub fn quadratic_e(a: i128, b: i128, c: i128) -> AlgebraicExpr {
        let mut expr = AlgebraicExpr::from_int(a);
        expr.add_term(AlgebraicTerm::e_power(b, 1));
        expr.add_term(AlgebraicTerm::e_power(c, 2));
        expr
    }
    
    /// a + b·√n
    pub fn a_plus_b_sqrt(a: i128, b: i128, n: i128) -> AlgebraicExpr {
        let mut expr = AlgebraicExpr::from_int(a);
        expr.add_term(AlgebraicTerm::sqrt_multiple(b, n));
        expr
    }
    
    /// a·α⁻¹ + b·X (X 是任意代数基)
    pub fn alpha_inv_combo(a: i128, b: i128, base: AlgebraicBase) -> AlgebraicExpr {
        let mut expr = AlgebraicExpr::zero();
        expr.add_term(AlgebraicTerm::alpha_inv_multiple(a));
        expr.add_term(AlgebraicTerm::new(Rational::from_i128(b), base, 1));
        expr
    }
    
    /// a·α⁻² + b·X
    pub fn alpha_inv_sq_combo(a: i128, b: i128, base: AlgebraicBase) -> AlgebraicExpr {
        let mut expr = AlgebraicExpr::zero();
        expr.add_term(AlgebraicTerm::alpha_inv_sq_multiple(a));
        expr.add_term(AlgebraicTerm::new(Rational::from_i128(b), base, 1));
        expr
    }
    
    /// a·ln(n) + b
    pub fn ln_linear(a: i128, n: i128, b: i128) -> AlgebraicExpr {
        let mut expr = AlgebraicExpr::from_int(b);
        expr.add_term(AlgebraicTerm::new(
            Rational::from_i128(a),
            AlgebraicBase::Ln(n),
            1,
        ));
        expr
    }
}

// ============================================================================
// 代数搜索结果
// ============================================================================

/// 搜索发现
#[derive(Debug, Clone)]
pub struct AlgebraicDiscovery {
    /// 目标常数名称
    pub target_name: String,
    /// 目标数值
    pub target_value: f64,
    /// 发现的代数表达式
    pub expression: AlgebraicExpr,
    /// 相对误差百分比
    pub error_percent: f64,
}

impl AlgebraicDiscovery {
    pub fn new(target_name: &str, target_value: f64, expression: AlgebraicExpr) -> Self {
        let computed = expression.to_f64();
        let error_percent = ((computed - target_value) / target_value).abs() * 100.0;
        AlgebraicDiscovery {
            target_name: target_name.to_string(),
            target_value,
            expression,
            error_percent,
        }
    }
    
    /// 是否是精确匹配（在给定容差内）
    pub fn is_exact(&self, tolerance_percent: f64) -> bool {
        self.error_percent < tolerance_percent
    }
}

impl fmt::Display for AlgebraicDiscovery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ≈ {} (error: {:.8}%)", 
            self.target_name, self.expression, self.error_percent)
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_algebraic_base_values() {
        assert!((AlgebraicBase::Pi.to_f64() - std::f64::consts::PI).abs() < 1e-15);
        assert!((AlgebraicBase::E.to_f64() - std::f64::consts::E).abs() < 1e-15);
        assert!((AlgebraicBase::Sqrt(2).to_f64() - std::f64::consts::SQRT_2).abs() < 1e-15);
        assert!((AlgebraicBase::GoldenRatio.to_f64() - 1.618033988749895).abs() < 1e-10);
    }
    
    #[test]
    fn test_term_to_f64() {
        // 2π
        let term = AlgebraicTerm::pi_power(2, 1);
        assert!((term.to_f64() - 2.0 * std::f64::consts::PI).abs() < 1e-10);
        
        // 3√5
        let term = AlgebraicTerm::sqrt_multiple(3, 5);
        assert!((term.to_f64() - 3.0 * 5.0_f64.sqrt()).abs() < 1e-10);
    }
    
    #[test]
    fn test_expression_to_f64() {
        // 3 + 2π
        let expr = AlgebraicBuilder::a_plus_b_pi(3, 2);
        let expected = 3.0 + 2.0 * std::f64::consts::PI;
        assert!((expr.to_f64() - expected).abs() < 1e-10);
    }
    
    #[test]
    fn test_exact_rational() {
        let expr = AlgebraicExpr::from_int(137);
        assert!(expr.is_rational());
        assert_eq!(expr.try_eval_exact(), Some(Rational::from_i128(137)));
    }
    
    #[test]
    fn test_complexity() {
        let simple = AlgebraicExpr::from_int(137);
        let complex = AlgebraicBuilder::quadratic_pi(100, 50, 25);
        assert!(simple.complexity() < complex.complexity());
    }
}
