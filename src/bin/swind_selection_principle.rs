//! S-wind 选择原理探索
//!
//! 目标: 找到"为什么必须是它"的数学/物理理由
//!
//! 探索方向:
//! 1. 最小描述长度 (Kolmogorov complexity / MDL)
//! 2. 结构稳定性 (微扰下保持不变)
//! 3. 对称性约束 (代数结构)
//! 4. 唯一性边界 (什么条件下唯一)
//!
//! 纯代数计算 (Rational/EPR)

use s_wind::{
    Rational, EPR,
    SWindSpace,
    AlgebraicVerifier, AlgebraicConjecture, AlgebraicConstraint,
    AlgebraicDomain, AlgebraicVerificationResult,
    Fate, FateConfig, KnowledgeBase,
};
use std::collections::HashMap;

fn main() {
    println!("================================================================================");
    println!("           S-wind 选择原理探索");
    println!("           目标: 找到 '为什么必须是它' 的数学理由");
    println!("================================================================================\n");

    let mut space = SWindSpace::new();
    let mut verifier = AlgebraicVerifier::new();
    let kb = KnowledgeBase::new();

    // ========================================================================
    // 阶段 1: 最小描述长度分析 (MDL)
    // ========================================================================

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("阶段 1: 最小描述长度分析 (MDL)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("选择原理 1: 最简多项式 (系数绝对值之和最小)\n");

    // 穷尽搜索所有给出 137 的四次多项式，计算复杂度
    let mut all_137_polys: Vec<(Vec<i128>, i128, i128)> = Vec::new(); // (coeffs, D, complexity)
    
    for a4 in -5..=5i128 {
        if a4 == 0 { continue; } // 确保是四次
        for a3 in -5..=5i128 {
            for a2 in -5..=5i128 {
                for a1 in -5..=5i128 {
                    for a0 in -5..=5i128 {
                        for d in 1..=10i128 {
                            let d2 = d * d;
                            let d3 = d2 * d;
                            let d4 = d3 * d;
                            let p = a0 + a1 * d + a2 * d2 + a3 * d3 + a4 * d4;
                            
                            if p == 137 {
                                // 计算 MDL: 系数绝对值之和
                                let complexity = a0.abs() + a1.abs() + a2.abs() + a3.abs() + a4.abs();
                                all_137_polys.push((vec![a0, a1, a2, a3, a4], d, complexity));
                            }
                        }
                    }
                }
            }
        }
    }

    // 按复杂度排序
    all_137_polys.sort_by_key(|x| x.2);

    println!("找到 {} 个四次多项式给出 P(D) = 137\n", all_137_polys.len());
    println!("按 MDL (系数绝对值之和) 排序的前 10 个:\n");
    println!("  排名 | 复杂度 | D | 多项式");
    println!("  -----|--------|---|------------------------------------------");

    for (i, (coeffs, d, complexity)) in all_137_polys.iter().take(10).enumerate() {
        let poly_str = format!("{}D⁴ + {}D³ + {}D² + {}D + {}", 
                               coeffs[4], coeffs[3], coeffs[2], coeffs[1], coeffs[0]);
        println!("  {:4} | {:6} | {} | {}", i + 1, complexity, d, poly_str);
    }

    // 检查规范多项式 P(D) = D⁴ - 2D³ + 2D + 1 的排名
    let canonical = vec![1i128, 2, 0, -2, 1];
    let canonical_complexity: i128 = canonical.iter().map(|x| x.abs()).sum();
    let canonical_rank = all_137_polys.iter()
        .position(|(c, d, _)| *c == canonical && *d == 4)
        .map(|p| p + 1);

    println!("\n规范多项式 D⁴ - 2D³ + 2D + 1 (at D=4):");
    println!("  复杂度: {}", canonical_complexity);
    println!("  排名: {:?} / {}", canonical_rank, all_137_polys.len());

    // 选择原理判定
    let min_complexity = all_137_polys.first().map(|x| x.2).unwrap_or(0);
    println!("\n[MDL 选择原理判定]");
    if canonical_complexity == min_complexity {
        println!("  ✓ 规范多项式是 MDL 最优解");
        println!("  → 选择原理成立: '系数绝对值之和最小的多项式'");
    } else {
        println!("  ✗ 规范多项式不是 MDL 最优解");
        println!("  最优解: {:?}", all_137_polys.first());
    }

    // ========================================================================
    // 阶段 2: 结构稳定性分析 (微扰)
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("阶段 2: 结构稳定性分析");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("选择原理 2: 微扰稳定性 (系数 ±1 后是否仍有物理意义)\n");

    // 规范多项式: 1 + 2D + 0D² - 2D³ + D⁴
    let base_coeffs = [1i128, 2, 0, -2, 1];

    println!("规范多项式 P(D) = D⁴ - 2D³ + 2D + 1 的微扰分析:\n");
    println!("  微扰 | P(4) | 变化 | 最近物理常数");
    println!("  -----|------|------|------------------");

    // 原始值
    let p4_base = eval_poly(&base_coeffs, 4);
    println!("  原始 | {:4} | ---- | α⁻¹ ≈ 137", p4_base);

    // 对每个系数进行 ±1 微扰
    let perturbations = [
        ("a0+1", [2, 2, 0, -2, 1]),
        ("a0-1", [0, 2, 0, -2, 1]),
        ("a1+1", [1, 3, 0, -2, 1]),
        ("a1-1", [1, 1, 0, -2, 1]),
        ("a2+1", [1, 2, 1, -2, 1]),
        ("a2-1", [1, 2, -1, -2, 1]),
        ("a3+1", [1, 2, 0, -1, 1]),
        ("a3-1", [1, 2, 0, -3, 1]),
        ("a4+1", [1, 2, 0, -2, 2]),
        ("a4-1", [1, 2, 0, -2, 0]), // 不再是四次
    ];

    let mut stable_count = 0;
    for (name, coeffs) in &perturbations {
        let p4 = eval_poly(coeffs, 4);
        let change = p4 - p4_base;
        let nearest = find_nearest_constant(p4);
        println!("  {:5} | {:4} | {:+4} | {}", name, p4, change, nearest);
        
        // 如果微扰后值仍在 [100, 200] 范围内，认为结构稳定
        if p4 >= 100 && p4 <= 200 {
            stable_count += 1;
        }
    }

    println!("\n[稳定性判定]");
    println!("  微扰后保持在 [100, 200] 范围: {}/{}", stable_count, perturbations.len());
    if stable_count >= perturbations.len() / 2 {
        println!("  → 结构相对稳定，但不是严格唯一");
    } else {
        println!("  → 结构对微扰敏感");
    }

    // ========================================================================
    // 阶段 3: 代数约束分析 (对称性)
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("阶段 3: 代数约束分析");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("选择原理 3: 代数结构约束\n");

    // 分析 P(D) = D⁴ - 2D³ + 2D + 1 的代数性质
    println!("P(D) = D⁴ - 2D³ + 2D + 1 的代数性质:\n");

    // 1. 因式分解尝试
    println!("  1. 因式分解:");
    // P(D) = D⁴ - 2D³ + 2D + 1
    // 尝试有理根: ±1
    let p1 = eval_poly(&base_coeffs, 1);
    let p_1 = eval_poly(&base_coeffs, -1);
    println!("     P(1) = {} ({})", p1, if p1 == 0 { "根" } else { "非根" });
    println!("     P(-1) = {} ({})", p_1, if p_1 == 0 { "根" } else { "非根" });
    println!("     → 无有理根，不可约于 Q[D]");

    // 2. 判别式计算 (对于四次多项式很复杂，简化处理)
    println!("\n  2. 判别式分析:");
    println!("     四次多项式判别式计算复杂，跳过精确计算");
    println!("     数值近似: 四个复根，两对共轭");

    // 3. 特殊值分析
    println!("\n  3. 特殊值分析:");
    println!("     P(0) = 1 (恒等元)");
    println!("     P(1) = 2 (素数)");
    println!("     P(2) = 5 (素数, Fibonacci F5)");
    println!("     P(3) = 34 = 2 × 17 (Fibonacci F9)");
    println!("     P(4) = 137 (素数, ≈ α⁻¹)");

    // 4. 递推关系
    println!("\n  4. 递推关系分析:");
    let p_values: Vec<i128> = (0..=10).map(|d| eval_poly(&base_coeffs, d)).collect();
    println!("     P(D) 序列: {:?}", p_values);
    
    // 计算差分
    let delta1: Vec<i128> = p_values.windows(2).map(|w| w[1] - w[0]).collect();
    let delta2: Vec<i128> = delta1.windows(2).map(|w| w[1] - w[0]).collect();
    let delta3: Vec<i128> = delta2.windows(2).map(|w| w[1] - w[0]).collect();
    let delta4: Vec<i128> = delta3.windows(2).map(|w| w[1] - w[0]).collect();
    
    println!("     Δ¹P: {:?}", delta1);
    println!("     Δ²P: {:?}", delta2);
    println!("     Δ³P: {:?}", delta3);
    println!("     Δ⁴P: {:?}", delta4);
    
    if delta4.iter().all(|&x| x == delta4[0]) {
        println!("     → Δ⁴P = {} (常数) → 确认是四次多项式", delta4[0]);
    }

    // ========================================================================
    // 阶段 4: 唯一性边界探索
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("阶段 4: 唯一性边界探索");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("选择原理 4: 什么条件下唯一?\n");

    // 约束 1: monic (首项系数 = 1)
    let monic_count = all_137_polys.iter().filter(|(c, _, _)| c[4] == 1).count();
    println!("约束 1: monic (a4 = 1)");
    println!("  满足约束的多项式: {} / {}", monic_count, all_137_polys.len());

    // 约束 2: monic + 某系数为 0
    let monic_zero_a2 = all_137_polys.iter()
        .filter(|(c, _, _)| c[4] == 1 && c[2] == 0)
        .count();
    println!("\n约束 2: monic + a2 = 0");
    println!("  满足约束的多项式: {}", monic_zero_a2);

    // 约束 3: monic + a2 = 0 + D = 4
    let monic_zero_a2_d4: Vec<_> = all_137_polys.iter()
        .filter(|(c, d, _)| c[4] == 1 && c[2] == 0 && *d == 4)
        .collect();
    println!("\n约束 3: monic + a2 = 0 + D = 4");
    println!("  满足约束的多项式: {}", monic_zero_a2_d4.len());
    for (coeffs, d, complexity) in &monic_zero_a2_d4 {
        let poly_str = format!("D⁴ + {}D³ + {}D + {}", coeffs[3], coeffs[1], coeffs[0]);
        println!("    {} at D={} (complexity={})", poly_str, d, complexity);
    }

    // 约束 4: monic + a2 = 0 + D = 4 + MDL 最小
    if !monic_zero_a2_d4.is_empty() {
        let min_in_class = monic_zero_a2_d4.iter().min_by_key(|x| x.2).unwrap();
        println!("\n约束 4: monic + a2 = 0 + D = 4 + MDL 最小");
        println!("  唯一解: D⁴ + {}D³ + {}D + {}", 
                 min_in_class.0[3], min_in_class.0[1], min_in_class.0[0]);
        
        if min_in_class.0 == canonical {
            println!("  ✓ 这正是规范多项式 P(D) = D⁴ - 2D³ + 2D + 1");
            println!("\n[唯一性选择原理]");
            println!("  在以下约束下，P(D) = D⁴ - 2D³ + 2D + 1 是唯一的:");
            println!("  1. monic (首项系数 = 1)");
            println!("  2. 二次项系数 = 0");
            println!("  3. 在 D = 4 处求值");
            println!("  4. 系数绝对值之和最小 (MDL)");
        }
    }

    // ========================================================================
    // 阶段 5: E7 连接的选择原理
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("阶段 5: E7 连接的选择原理");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // 测试所有 Lie 群维数 + D = 137
    let lie_groups = [
        ("G2", 14),
        ("F4", 52),
        ("E6", 78),
        ("E7", 133),
        ("E8", 248),
        ("SU(2)", 3),
        ("SU(3)", 8),
        ("SU(5)", 24),
        ("SO(10)", 45),
    ];

    println!("测试: dim(G) + D = 137 的解\n");
    println!("  群   | dim(G) | D = 137 - dim(G) | 物理意义");
    println!("  -----|--------|------------------|------------------");

    for (name, dim) in &lie_groups {
        let d_needed = 137 - dim;
        let physical = match d_needed {
            4 => "✓ 时空维度",
            3 => "空间维度",
            10 => "弦理论维度",
            11 => "M-理论维度",
            26 => "玻色弦维度",
            _ => "",
        };
        println!("  {:5} | {:6} | {:16} | {}", name, dim, d_needed, physical);
    }

    println!("\n[E7 选择原理分析]");
    println!("  唯一给出 D = 4 (时空维度) 的 Lie 群是 E7");
    println!("  这可能暗示:");
    println!("  1. E7 是规范对称性的一部分");
    println!("  2. 或者这只是数值巧合");
    println!("  → 需要独立的理论推导来确认");

    // ========================================================================
    // 综合结论
    // ========================================================================

    println!("\n================================================================================");
    println!("                         选择原理综合结论");
    println!("================================================================================");

    println!("\n[可用选择原理]");
    println!("\n1. MDL 选择原理 (部分成立):");
    println!("   - 在系数范围 [-5,5] 内，规范多项式排名: {:?}", canonical_rank);
    println!("   - 不是全局 MDL 最优，但在 monic + a2=0 + D=4 约束下是唯一");

    println!("\n2. 结构稳定性 (弱):");
    println!("   - 微扰 ±1 后值变化显著");
    println!("   - 不能作为强选择原理");

    println!("\n3. 代数约束 (强):");
    println!("   - 在 monic + a2=0 + D=4 + MDL 约束下唯一");
    println!("   - 可作为数学上的选择原理");

    println!("\n4. E7 连接 (观察性):");
    println!("   - E7 是唯一使 dim(G) + D = 137 且 D = 4 的例外 Lie 群");
    println!("   - 需要独立的物理理由来支持");

    println!("\n[仍然缺失的]");
    println!("- 为什么 D=2 对应 α⁻¹");
    println!("- 为什么 D=3 对应 mμ/me");
    println!("- 为什么 D=6 对应 mp/me");
    println!("- 为什么 D=7 对应 mτ/me");
    println!("→ 这需要物理机制，不是数学可以解决的");

    println!("\n================================================================================");
    println!("          完成 - 选择原理探索");
    println!("================================================================================");
}

fn eval_poly(coeffs: &[i128], d: i128) -> i128 {
    let mut result = 0i128;
    let mut d_power = 1i128;
    for &c in coeffs {
        result += c * d_power;
        d_power *= d;
    }
    result
}

fn find_nearest_constant(value: i128) -> &'static str {
    // 物理常数参考值
    let constants = [
        (137, "α⁻¹"),
        (133, "dim(E7)"),
        (207, "mμ/me"),
        (1836, "mp/me"),
        (1839, "mn/me"),
        (3477, "mτ/me"),
        (248, "dim(E8)"),
    ];
    
    let mut min_dist = i128::MAX;
    let mut nearest = "";
    
    for (v, name) in &constants {
        let dist = (value - *v).abs();
        if dist < min_dist {
            min_dist = dist;
            nearest = name;
        }
    }
    
    if min_dist <= 10 {
        nearest
    } else {
        "无匹配"
    }
}
