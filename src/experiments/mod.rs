//! S-wind 实验模块
//!
//! 使用统一 Fate (EPR 无限精度) 进行数学/物理探索

use crate::fate::{Fate, ExplorationResult};
use crate::knowledge::KnowledgeBase;
use crate::symbolic::EPR;

/// 简化的实验接口
pub struct Experiment {
    pub name: String,
    pub description: String,
    pub signal: Vec<EPR>,
}

impl Experiment {
    /// 创建新实验
    pub fn new(name: &str, description: &str, signal: Vec<EPR>) -> Self {
        Experiment {
            name: name.to_string(),
            description: description.to_string(),
            signal,
        }
    }
    
    /// 从整数创建
    pub fn from_integers(name: &str, description: &str, values: &[i128]) -> Self {
        Experiment {
            name: name.to_string(),
            description: description.to_string(),
            signal: values.iter().map(|&v| EPR::from_i128(v)).collect(),
        }
    }
    
    /// 从分数创建
    pub fn from_fractions(name: &str, description: &str, fractions: &[(i128, i128)]) -> Self {
        Experiment {
            name: name.to_string(),
            description: description.to_string(),
            signal: fractions.iter().map(|&(n, d)| EPR::from_ratio(n, d)).collect(),
        }
    }
    
    /// 运行实验
    pub fn run(&self, fate: &mut Fate, kb: &KnowledgeBase) -> ExplorationResult {
        fate.explore(&self.signal, kb)
    }
}

/// 预定义实验集
pub fn get_standard_experiments() -> Vec<Experiment> {
    vec![
        // 基础序列
        Experiment::from_integers(
            "Fibonacci",
            "斐波那契数列: 1, 1, 2, 3, 5, 8, ...",
            &[1, 1, 2, 3, 5, 8, 13, 21, 34]
        ),
        Experiment::from_integers(
            "Primes",
            "素数序列: 2, 3, 5, 7, 11, ...",
            &[2, 3, 5, 7, 11, 13, 17, 19, 23]
        ),
        Experiment::from_integers(
            "Squares",
            "平方数: 1, 4, 9, 16, 25, ...",
            &[1, 4, 9, 16, 25, 36, 49, 64, 81]
        ),
        Experiment::from_integers(
            "Cubes",
            "立方数: 1, 8, 27, 64, ...",
            &[1, 8, 27, 64, 125, 216, 343]
        ),
        Experiment::from_integers(
            "Powers_of_2",
            "2的幂: 1, 2, 4, 8, 16, ...",
            &[1, 2, 4, 8, 16, 32, 64, 128]
        ),
        Experiment::from_integers(
            "Factorial",
            "阶乘: 1, 1, 2, 6, 24, 120, ...",
            &[1, 1, 2, 6, 24, 120, 720, 5040]
        ),
        Experiment::from_integers(
            "Triangular",
            "三角数: 1, 3, 6, 10, 15, ...",
            &[1, 3, 6, 10, 15, 21, 28, 36, 45]
        ),
        Experiment::from_integers(
            "Catalan",
            "卡塔兰数: 1, 1, 2, 5, 14, 42, ...",
            &[1, 1, 2, 5, 14, 42, 132, 429]
        ),
        
        // 分数序列
        Experiment::from_fractions(
            "Harmonic",
            "调和序列: 1, 1/2, 1/3, 1/4, ...",
            &[(1, 1), (1, 2), (1, 3), (1, 4), (1, 5)]
        ),
        Experiment::from_fractions(
            "Basel",
            "巴塞尔序列: 1, 1/4, 1/9, 1/16, ...",
            &[(1, 1), (1, 4), (1, 9), (1, 16), (1, 25)]
        ),
        
        // 高级序列
        Experiment::from_integers(
            "Bell",
            "贝尔数: 1, 1, 2, 5, 15, 52, ...",
            &[1, 1, 2, 5, 15, 52, 203, 877]
        ),
        Experiment::from_integers(
            "Partition",
            "分割数: 1, 1, 2, 3, 5, 7, 11, ...",
            &[1, 1, 2, 3, 5, 7, 11, 15, 22, 30]
        ),
        Experiment::from_integers(
            "Pell",
            "佩尔数: 0, 1, 2, 5, 12, 29, ...",
            &[0, 1, 2, 5, 12, 29, 70, 169]
        ),
        Experiment::from_integers(
            "Lucas",
            "卢卡斯数: 2, 1, 3, 4, 7, 11, ...",
            &[2, 1, 3, 4, 7, 11, 18, 29, 47]
        ),
    ]
}

/// 运行所有标准实验
pub fn run_all_experiments() -> Vec<(String, ExplorationResult)> {
    let mut fate = Fate::new();
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    
    let experiments = get_standard_experiments();
    let mut results = Vec::new();
    
    for exp in experiments {
        let result = exp.run(&mut fate, &kb);
        results.push((exp.name, result));
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_experiment_fibonacci() {
        let mut fate = Fate::new();
        let mut kb = KnowledgeBase::new();
        kb.initialize_fundamentals();
        
        let exp = Experiment::from_integers(
            "Fibonacci",
            "Fibonacci sequence",
            &[1, 1, 2, 3, 5, 8]
        );
        
        let result = exp.run(&mut fate, &kb);
        assert!(result.matched_known.is_some());
    }
    
    #[test]
    fn test_experiment_harmonic() {
        let mut fate = Fate::new();
        let mut kb = KnowledgeBase::new();
        kb.initialize_fundamentals();
        
        let exp = Experiment::from_fractions(
            "Harmonic",
            "Harmonic sequence",
            &[(1, 2), (1, 3), (1, 4), (1, 5)]
        );
        
        let result = exp.run(&mut fate, &kb);
        // 应该检测到倒数模式
        assert!(result.formula.is_some() || result.matched_known.is_some());
    }
}
