//! Algebraic Search Engine for S-wind
//!
//! 代数优先原则的搜索引擎:
//! 1. 生成符号代数表达式
//! 2. 评估复杂度
//! 3. 最后才进行数值验证
//!
//! 核心理念: 先构建代数结构，再验证数值

use crate::symbolic::{
    AlgebraicBase, AlgebraicTerm, AlgebraicExpr, 
    AlgebraicBuilder, AlgebraicDiscovery, Rational
};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

// ============================================================================
// 物理常数定义 (CODATA 2022)
// ============================================================================

/// 物理常数，包含符号名称和 CODATA 精确值
pub struct PhysicalConstant {
    pub name: &'static str,
    pub symbol: &'static str,
    pub value: f64,
    pub latex: &'static str,
}

/// CODATA 2022 物理常数
pub mod constants {
    use super::PhysicalConstant;
    
    pub const ALPHA_INV: PhysicalConstant = PhysicalConstant {
        name: "Fine structure constant inverse",
        symbol: "α⁻¹",
        value: 137.035999177,
        latex: "\\alpha^{-1}",
    };
    
    pub const PROTON_ELECTRON_RATIO: PhysicalConstant = PhysicalConstant {
        name: "Proton-electron mass ratio",
        symbol: "mp/me",
        value: 1836.15267344,
        latex: "m_p/m_e",
    };
    
    pub const W_ELECTRON_RATIO: PhysicalConstant = PhysicalConstant {
        name: "W boson-electron mass ratio",
        symbol: "mW/me",
        value: 157339.7,  // 80.377 GeV / 0.511 MeV
        latex: "m_W/m_e",
    };
    
    pub const Z_ELECTRON_RATIO: PhysicalConstant = PhysicalConstant {
        name: "Z boson-electron mass ratio",
        symbol: "mZ/me",
        value: 178450.0,  // 91.1876 GeV / 0.511 MeV
        latex: "m_Z/m_e",
    };
    
    pub const HIGGS_ELECTRON_RATIO: PhysicalConstant = PhysicalConstant {
        name: "Higgs-electron mass ratio",
        symbol: "mH/me",
        value: 244888.7,  // 125.25 GeV / 0.511 MeV
        latex: "m_H/m_e",
    };
    
    pub const TOP_ELECTRON_RATIO: PhysicalConstant = PhysicalConstant {
        name: "Top quark-electron mass ratio",
        symbol: "mt/me",
        value: 338023.0,  // 172.69 GeV / 0.511 MeV
        latex: "m_t/m_e",
    };
    
    pub const WEINBERG_SIN_SQ: PhysicalConstant = PhysicalConstant {
        name: "Weinberg angle sin squared",
        symbol: "sin²θ_W",
        value: 0.22305,
        latex: "\\sin^2\\theta_W",
    };
    
    pub const OMEGA_LAMBDA: PhysicalConstant = PhysicalConstant {
        name: "Dark energy density",
        symbol: "Ω_Λ",
        value: 0.6889,
        latex: "\\Omega_\\Lambda",
    };
    
    pub const OMEGA_MATTER: PhysicalConstant = PhysicalConstant {
        name: "Matter density",
        symbol: "Ω_m",
        value: 0.3111,
        latex: "\\Omega_m",
    };
    
    /// 获取所有常数
    pub fn all() -> Vec<&'static PhysicalConstant> {
        vec![
            &ALPHA_INV, &PROTON_ELECTRON_RATIO,
            &W_ELECTRON_RATIO, &Z_ELECTRON_RATIO,
            &HIGGS_ELECTRON_RATIO, &TOP_ELECTRON_RATIO,
            &WEINBERG_SIN_SQ, &OMEGA_LAMBDA, &OMEGA_MATTER,
        ]
    }
}

// ============================================================================
// 搜索结果排序 (按复杂度 + 误差)
// ============================================================================

/// 可排序的发现（最小复杂度 + 最小误差 优先）
#[derive(Debug, Clone)]
struct RankedDiscovery {
    discovery: AlgebraicDiscovery,
    score: f64,  // 越低越好
}

impl RankedDiscovery {
    fn new(discovery: AlgebraicDiscovery) -> Self {
        // Score = complexity + log(error) * 10
        // 这样既考虑代数简洁性，又考虑精度
        let complexity = discovery.expression.complexity() as f64;
        let error_factor = if discovery.error_percent > 0.0 {
            discovery.error_percent.log10() * 10.0
        } else {
            -100.0  // 完美匹配
        };
        let score = complexity + error_factor.max(-50.0);
        RankedDiscovery { discovery, score }
    }
}

impl PartialEq for RankedDiscovery {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for RankedDiscovery {}

impl PartialOrd for RankedDiscovery {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RankedDiscovery {
    fn cmp(&self, other: &Self) -> Ordering {
        // 反向：score 越低优先级越高
        other.score.partial_cmp(&self.score).unwrap_or(Ordering::Equal)
    }
}

// ============================================================================
// 代数搜索引擎
// ============================================================================

/// 代数搜索引擎
pub struct AlgebraicSearchEngine {
    /// 搜索的代数基集合
    bases: Vec<AlgebraicBase>,
    /// 系数搜索范围
    coeff_range: std::ops::RangeInclusive<i128>,
    /// 容差阈值 (%)
    tolerance_percent: f64,
    /// 最大结果数
    max_results: usize,
}

impl Default for AlgebraicSearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AlgebraicSearchEngine {
    pub fn new() -> Self {
        AlgebraicSearchEngine {
            bases: vec![
                AlgebraicBase::Pi,
                AlgebraicBase::E,
                AlgebraicBase::Sqrt(2),
                AlgebraicBase::Sqrt(3),
                AlgebraicBase::Sqrt(5),
                AlgebraicBase::GoldenRatio,
                AlgebraicBase::Ln(2),
                AlgebraicBase::Ln(10),
                AlgebraicBase::AlphaInv,
            ],
            coeff_range: -500..=500,
            tolerance_percent: 0.001,  // 0.001% = 1e-5
            max_results: 10,
        }
    }
    
    /// 设置搜索范围
    pub fn with_coeff_range(mut self, range: std::ops::RangeInclusive<i128>) -> Self {
        self.coeff_range = range;
        self
    }
    
    /// 设置容差
    pub fn with_tolerance(mut self, percent: f64) -> Self {
        self.tolerance_percent = percent;
        self
    }
    
    /// 设置代数基
    pub fn with_bases(mut self, bases: Vec<AlgebraicBase>) -> Self {
        self.bases = bases;
        self
    }
    
    /// 搜索: a + b·X 形式
    pub fn search_linear(&self, target: &PhysicalConstant) -> Vec<AlgebraicDiscovery> {
        let mut heap: BinaryHeap<RankedDiscovery> = BinaryHeap::new();
        
        for base in &self.bases {
            for a in self.coeff_range.clone() {
                for b in self.coeff_range.clone() {
                    if b == 0 { continue; }
                    
                    let mut expr = AlgebraicExpr::from_int(a);
                    expr.add_term(AlgebraicTerm::new(
                        Rational::from_i128(b),
                        base.clone(),
                        1,
                    ));
                    
                    let discovery = AlgebraicDiscovery::new(
                        target.symbol,
                        target.value,
                        expr,
                    );
                    
                    if discovery.is_exact(self.tolerance_percent) {
                        heap.push(RankedDiscovery::new(discovery));
                        
                        // 保持堆大小
                        while heap.len() > self.max_results * 2 {
                            heap.pop();
                        }
                    }
                }
            }
        }
        
        // 提取最佳结果
        let mut results: Vec<_> = heap.into_iter()
            .take(self.max_results)
            .map(|r| r.discovery)
            .collect();
        results.sort_by(|a, b| a.error_percent.partial_cmp(&b.error_percent).unwrap());
        results
    }
    
    /// 搜索: a + b·X + c·X² 形式
    pub fn search_quadratic(&self, target: &PhysicalConstant) -> Vec<AlgebraicDiscovery> {
        let mut heap: BinaryHeap<RankedDiscovery> = BinaryHeap::new();
        let small_range = -100..=100_i128;
        
        for base in &self.bases {
            for a in small_range.clone() {
                for b in small_range.clone() {
                    for c in small_range.clone() {
                        if b == 0 && c == 0 { continue; }
                        
                        let mut expr = AlgebraicExpr::from_int(a);
                        if b != 0 {
                            expr.add_term(AlgebraicTerm::new(
                                Rational::from_i128(b),
                                base.clone(),
                                1,
                            ));
                        }
                        if c != 0 {
                            expr.add_term(AlgebraicTerm::new(
                                Rational::from_i128(c),
                                base.clone(),
                                2,
                            ));
                        }
                        
                        let discovery = AlgebraicDiscovery::new(
                            target.symbol,
                            target.value,
                            expr,
                        );
                        
                        if discovery.is_exact(self.tolerance_percent) {
                            heap.push(RankedDiscovery::new(discovery));
                            while heap.len() > self.max_results * 2 {
                                heap.pop();
                            }
                        }
                    }
                }
            }
        }
        
        let mut results: Vec<_> = heap.into_iter()
            .take(self.max_results)
            .map(|r| r.discovery)
            .collect();
        results.sort_by(|a, b| a.error_percent.partial_cmp(&b.error_percent).unwrap());
        results
    }
    
    /// 搜索: a·α⁻¹ + b·X 形式 (α 与超越数混合)
    pub fn search_alpha_mixed(&self, target: &PhysicalConstant) -> Vec<AlgebraicDiscovery> {
        let mut heap: BinaryHeap<RankedDiscovery> = BinaryHeap::new();
        
        let transcendental_bases = vec![
            AlgebraicBase::Pi,
            AlgebraicBase::E,
            AlgebraicBase::Sqrt(2),
            AlgebraicBase::Sqrt(3),
            AlgebraicBase::Sqrt(5),
            AlgebraicBase::GoldenRatio,
            AlgebraicBase::Ln(2),
            AlgebraicBase::Ln(10),
        ];
        
        for base in &transcendental_bases {
            for a in self.coeff_range.clone() {
                for b in self.coeff_range.clone() {
                    // a·α⁻¹ + b·X
                    let expr = AlgebraicBuilder::alpha_inv_combo(a, b, base.clone());
                    let discovery = AlgebraicDiscovery::new(target.symbol, target.value, expr);
                    
                    if discovery.is_exact(self.tolerance_percent) {
                        heap.push(RankedDiscovery::new(discovery));
                    }
                    
                    // a·α⁻² + b·X
                    let expr2 = AlgebraicBuilder::alpha_inv_sq_combo(a, b, base.clone());
                    let discovery2 = AlgebraicDiscovery::new(target.symbol, target.value, expr2);
                    
                    if discovery2.is_exact(self.tolerance_percent) {
                        heap.push(RankedDiscovery::new(discovery2));
                    }
                    
                    while heap.len() > self.max_results * 2 {
                        heap.pop();
                    }
                }
            }
        }
        
        let mut results: Vec<_> = heap.into_iter()
            .take(self.max_results)
            .map(|r| r.discovery)
            .collect();
        results.sort_by(|a, b| a.error_percent.partial_cmp(&b.error_percent).unwrap());
        results
    }
    
    /// 搜索: (a + b·X) / (c + d·X) 有理函数形式
    pub fn search_rational_function(&self, target: &PhysicalConstant) -> Vec<AlgebraicDiscovery> {
        let mut results = Vec::new();
        let small_range = -100..=100_i64;
        
        for base in &self.bases {
            let base_val = base.to_f64();
            
            for a in small_range.clone() {
                for b in small_range.clone() {
                    for c in small_range.clone() {
                        for d in small_range.clone() {
                            if c == 0 && d == 0 { continue; }
                            
                            let numer = a as f64 + b as f64 * base_val;
                            let denom = c as f64 + d as f64 * base_val;
                            
                            if denom.abs() < 1e-10 { continue; }
                            
                            let computed = numer / denom;
                            let error = ((computed - target.value) / target.value).abs() * 100.0;
                            
                            if error < self.tolerance_percent {
                                // 构建符号表达式 (简化表示)
                                let mut numer_expr = AlgebraicExpr::from_int(a as i128);
                                numer_expr.add_term(AlgebraicTerm::new(
                                    Rational::from_i128(b as i128),
                                    base.clone(),
                                    1,
                                ));
                                
                                // 用字符串表示有理函数
                                let mut denom_expr = AlgebraicExpr::from_int(c as i128);
                                denom_expr.add_term(AlgebraicTerm::new(
                                    Rational::from_i128(d as i128),
                                    base.clone(),
                                    1,
                                ));
                                
                                // 创建一个特殊的发现表示
                                results.push(AlgebraicDiscovery {
                                    target_name: target.symbol.to_string(),
                                    target_value: target.value,
                                    expression: numer_expr,  // 分子
                                    error_percent: error,
                                });
                                
                                if results.len() >= self.max_results {
                                    return results;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        results.sort_by(|a, b| a.error_percent.partial_cmp(&b.error_percent).unwrap());
        results.truncate(self.max_results);
        results
    }
    
    /// 全面搜索目标常数
    pub fn search_all(&self, target: &PhysicalConstant) -> Vec<AlgebraicDiscovery> {
        let mut all_results = Vec::new();
        
        // 线性搜索
        all_results.extend(self.search_linear(target));
        
        // 二次搜索
        all_results.extend(self.search_quadratic(target));
        
        // α 混合搜索
        all_results.extend(self.search_alpha_mixed(target));
        
        // 去重和排序
        all_results.sort_by(|a, b| {
            a.error_percent.partial_cmp(&b.error_percent).unwrap()
        });
        
        // 按复杂度去重（保留误差最小的）
        let mut seen_complexity = std::collections::HashSet::new();
        all_results.retain(|d| {
            let key = d.expression.complexity();
            if seen_complexity.contains(&key) && d.error_percent > 1e-6 {
                false
            } else {
                seen_complexity.insert(key);
                true
            }
        });
        
        all_results.truncate(self.max_results);
        all_results
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_search_alpha_inv() {
        let engine = AlgebraicSearchEngine::new()
            .with_coeff_range(-200..=200)
            .with_tolerance(0.01);
        
        let results = engine.search_quadratic(&constants::ALPHA_INV);
        
        // 应该能找到一些近似
        assert!(!results.is_empty());
        
        // 最佳结果应该非常接近
        if let Some(best) = results.first() {
            println!("Best: {} (error: {}%)", best.expression, best.error_percent);
            assert!(best.error_percent < 0.1);
        }
    }
    
    #[test]
    fn test_search_engine_basic() {
        let engine = AlgebraicSearchEngine::new();
        assert!(!engine.bases.is_empty());
    }
}
