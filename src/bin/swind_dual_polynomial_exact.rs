//! S-wind 双多项式统一理论 - 纯代数精确计算
//!
//! 核心原则: 所有计算使用整数/有理数，零浮点
//!
//! 使用 S-wind 的 Rational 类型进行精确运算

use s_wind::Rational;

fn main() {
    println!("================================================================================");
    println!("           S-wind 双多项式统一理论 (纯代数精确版)");
    println!("           所有计算: 整数/有理数，零浮点");
    println!("================================================================================\n");

    // 两个核心多项式 - 整数系数
    let p_coeffs: [i128; 5] = [2, -5, 4, 15, 99];    // 质量谱多项式
    let q_coeffs: [i128; 5] = [1, -2, 0, 2, 1];      // 耦合常数多项式

    // ========================================================================
    // 阶段 1: P(D)/Q(D) 精确有理数计算
    // ========================================================================

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("阶段 1: P(D)/Q(D) 精确有理数分析");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("  D | P(D)   | Q(D)  | P/Q (精确有理数)       | 整数部分 | 余数");
    println!("  --|--------|-------|------------------------|---------|------");

    for d in 1..=12i128 {
        let p = eval_poly_exact(&p_coeffs, d);
        let q = eval_poly_exact(&q_coeffs, d);
        
        // 精确整数除法
        let quotient = p / q;  // 整数部分
        let remainder = p % q; // 余数
        
        // 有理数表示
        let g = gcd(p.abs(), q.abs());
        let p_reduced = p / g;
        let q_reduced = q / g;
        
        let marker = if remainder.abs() * 100 < p.abs() { " ★" } else { "" };
        
        println!("  {:2} | {:6} | {:5} | {:6}/{:<6} = {:<6}/{:<6} | {:7} | {:5}{}",
                 d, p, q, p, q, p_reduced, q_reduced, quotient, remainder, marker);
    }

    // ========================================================================
    // 阶段 2: 精确整数比分析
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("阶段 2: P(D) = n × Q(D) + r 的精确分解");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("寻找: P(D) = n × Q(D) + r, 其中 |r| 最小\n");

    for d in 1..=10i128 {
        let p = eval_poly_exact(&p_coeffs, d);
        let q = eval_poly_exact(&q_coeffs, d);
        
        // P = n*Q + r
        let n = p / q;
        let r = p - n * q;
        
        // 也检查 n+1
        let r_plus = p - (n + 1) * q;
        
        let (best_n, best_r) = if r.abs() <= r_plus.abs() {
            (n, r)
        } else {
            (n + 1, r_plus)
        };
        
        // 计算 |r|/P 的有理数
        let error_num = best_r.abs();
        let error_den = p;
        let g = gcd(error_num, error_den);
        
        println!("  D={:2}: P({}) = {} × Q({}) + {}", d, d, best_n, d, best_r);
        println!("         {} = {} × {} + {}", p, best_n, q, best_r);
        println!("         |r|/P = {}/{} = {}/{}", 
                 error_num, error_den, error_num/g, error_den/g);
        println!();
    }

    // ========================================================================
    // 阶段 3: Q(D) 与 Fibonacci 的精确关系
    // ========================================================================

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("阶段 3: Q(D) 与 Fibonacci 数列 (精确整数)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Fibonacci 数列 (精确整数)
    let fibs: Vec<i128> = {
        let mut f = vec![1i128, 1];
        for _ in 2..25 {
            let next = f[f.len()-1] + f[f.len()-2];
            f.push(next);
        }
        f
    };

    println!("Fibonacci 数列 F_n (n=1,2,...):");
    println!("  {:?}\n", &fibs[..15]);

    println!("Q(D) 与 Fibonacci 的精确比较:");
    for d in 1..=10i128 {
        let q = eval_poly_exact(&q_coeffs, d);
        
        // 检查是否精确等于某个 Fibonacci 数
        let mut match_info = String::new();
        for (i, &f) in fibs.iter().enumerate() {
            if f == q {
                match_info = format!("= F_{} ★★★ 精确匹配", i + 1);
                break;
            }
        }
        
        // 如果不是精确匹配，找最近的
        if match_info.is_empty() {
            let mut nearest_idx = 0;
            let mut nearest_diff = i128::MAX;
            for (i, &f) in fibs.iter().enumerate() {
                let diff = (q - f).abs();
                if diff < nearest_diff {
                    nearest_diff = diff;
                    nearest_idx = i;
                }
            }
            match_info = format!("≈ F_{} = {} (差 = {})", 
                                nearest_idx + 1, fibs[nearest_idx], q - fibs[nearest_idx]);
        }
        
        println!("  Q({:2}) = {:6} {}", d, q, match_info);
    }

    // ========================================================================
    // 阶段 4: 多项式的代数性质
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("阶段 4: 多项式代数性质 (精确)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // P(D) 的差分序列
    println!("P(D) 差分序列 (精确整数):");
    let p_values: Vec<i128> = (1..=12).map(|d| eval_poly_exact(&p_coeffs, d)).collect();
    println!("  P(D):  {:?}", p_values);
    
    let delta1: Vec<i128> = p_values.windows(2).map(|w| w[1] - w[0]).collect();
    let delta2: Vec<i128> = delta1.windows(2).map(|w| w[1] - w[0]).collect();
    let delta3: Vec<i128> = delta2.windows(2).map(|w| w[1] - w[0]).collect();
    let delta4: Vec<i128> = delta3.windows(2).map(|w| w[1] - w[0]).collect();
    
    println!("  Δ¹P:   {:?}", delta1);
    println!("  Δ²P:   {:?}", delta2);
    println!("  Δ³P:   {:?}", delta3);
    println!("  Δ⁴P:   {:?}", delta4);
    println!("  → Δ⁴P = {} (常数，因为 P 是 4 次多项式)", delta4[0]);
    println!("  → Δ⁴P = 4! × a₄ = 24 × 2 = 48 ✓");

    // Q(D) 的差分序列
    println!("\nQ(D) 差分序列 (精确整数):");
    let q_values: Vec<i128> = (1..=12).map(|d| eval_poly_exact(&q_coeffs, d)).collect();
    println!("  Q(D):  {:?}", q_values);
    
    let q_delta1: Vec<i128> = q_values.windows(2).map(|w| w[1] - w[0]).collect();
    let q_delta2: Vec<i128> = q_delta1.windows(2).map(|w| w[1] - w[0]).collect();
    let q_delta3: Vec<i128> = q_delta2.windows(2).map(|w| w[1] - w[0]).collect();
    let q_delta4: Vec<i128> = q_delta3.windows(2).map(|w| w[1] - w[0]).collect();
    
    println!("  Δ¹Q:   {:?}", q_delta1);
    println!("  Δ²Q:   {:?}", q_delta2);
    println!("  Δ³Q:   {:?}", q_delta3);
    println!("  Δ⁴Q:   {:?}", q_delta4);
    println!("  → Δ⁴Q = {} (常数)", q_delta4[0]);
    println!("  → Δ⁴Q = 4! × a₄ = 24 × 1 = 24 ✓");

    // ========================================================================
    // 阶段 5: P - n×Q 的精确分析
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("阶段 5: 不变量 I_n(D) = P(D) - n×Q(D) 的精确值");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // I_n(D) = P(D) - n × Q(D)
    // 这是一个新的多项式，系数为 (2-n, -5+2n, 4, 15-2n, 99-n)
    
    for n in 1..=4i128 {
        println!("I_{}(D) = P(D) - {} × Q(D):", n, n);
        println!("       = (2-{})D⁴ + (-5+{})D³ + 4D² + (15-{})D + (99-{})", n, 2*n, 2*n, n);
        let new_coeffs: [i128; 5] = [2-n, -5+2*n, 4, 15-2*n, 99-n];
        println!("       = {}D⁴ + ({})D³ + 4D² + ({})D + {}", 
                 new_coeffs[0], new_coeffs[1], new_coeffs[3], new_coeffs[4]);
        
        print!("       值: ");
        for d in 1..=7i128 {
            let inv = eval_poly_exact(&new_coeffs, d);
            print!("I_{}({})={}, ", n, d, inv);
        }
        println!("\n");
    }

    // 特别关注 n=2 的情况
    println!("特别分析 I_2(D) = P(D) - 2×Q(D):");
    let i2_coeffs: [i128; 5] = [0, -1, 4, 11, 97];  // 2-2, -5+4, 4, 15-4, 99-2
    println!("  I_2(D) = -D³ + 4D² + 11D + 97 (注意: 变成 3 次多项式!)");
    
    for d in 1..=10i128 {
        let i2 = eval_poly_exact(&i2_coeffs, d);
        println!("  I_2({:2}) = {}", d, i2);
    }

    // ========================================================================
    // 阶段 6: 寻找精确整数关系
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("阶段 6: 精确整数关系搜索");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("寻找: P(D₁) = k × Q(D₂) 的精确整数关系\n");

    let mut exact_relations: Vec<(i128, i128, i128, i128, i128)> = Vec::new();
    
    for d1 in 1..=10i128 {
        let p = eval_poly_exact(&p_coeffs, d1);
        for d2 in 1..=10i128 {
            let q = eval_poly_exact(&q_coeffs, d2);
            if q != 0 && p % q == 0 {
                let k = p / q;
                exact_relations.push((d1, d2, p, q, k));
            }
        }
    }

    if exact_relations.is_empty() {
        println!("  无精确整数关系 P(D₁) = k × Q(D₂)");
    } else {
        println!("  发现精确整数关系:");
        for (d1, d2, p, q, k) in &exact_relations {
            println!("    P({}) = {} × Q({})  即 {} = {} × {}", d1, k, d2, p, k, q);
        }
    }

    // 寻找 P(D₁) + P(D₂) = k × Q(D₃) 形式
    println!("\n寻找: P(D₁) + P(D₂) = k × Q(D₃) 的精确整数关系\n");
    
    for d1 in 1..=7i128 {
        for d2 in d1..=7i128 {
            let p_sum = eval_poly_exact(&p_coeffs, d1) + eval_poly_exact(&p_coeffs, d2);
            for d3 in 1..=7i128 {
                let q = eval_poly_exact(&q_coeffs, d3);
                if q != 0 && p_sum % q == 0 {
                    let k = p_sum / q;
                    if k > 0 && k < 100 {
                        println!("  P({}) + P({}) = {} × Q({})", d1, d2, k, d3);
                        println!("  {} + {} = {} × {}", 
                                 eval_poly_exact(&p_coeffs, d1),
                                 eval_poly_exact(&p_coeffs, d2),
                                 k, q);
                    }
                }
            }
        }
    }

    // ========================================================================
    // 阶段 7: 物理常数的精确验证
    // ========================================================================

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("阶段 7: 物理常数精确验证");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // 使用有理数表示物理常数 (CODATA 精确值)
    // α⁻¹ = 137.035999177(21) ≈ 137035999177/1000000000
    
    println!("物理常数 (有理数近似):");
    println!("  α⁻¹ ≈ 137035999/1000000 (精确到 ppm 级)");
    println!("  mμ/me ≈ 2067682827/10000000");
    println!("  mp/me ≈ 183615267343/100000000");
    println!("  mn/me ≈ 183868366173/100000000");
    println!("  mτ/me ≈ 347723/100");

    println!("\n多项式精确值:");
    println!("  P(2) = {} (对比 α⁻¹ ≈ 137.036)", eval_poly_exact(&p_coeffs, 2));
    println!("  P(3) = {} (对比 mμ/me ≈ 206.768)", eval_poly_exact(&p_coeffs, 3));
    println!("  P(6) = {} (对比 mp/me ≈ 1836.15)", eval_poly_exact(&p_coeffs, 6));
    println!("  P(7) = {} (对比 mτ/me ≈ 3477.23)", eval_poly_exact(&p_coeffs, 7));
    
    println!("\n精确偏差 (整数):");
    // P(2) - 137 = 0
    println!("  P(2) - 137 = {}", eval_poly_exact(&p_coeffs, 2) - 137);
    // P(3) - 207 = 0
    println!("  P(3) - 207 = {}", eval_poly_exact(&p_coeffs, 3) - 207);
    // P(6) - 1836 = 9
    println!("  P(6) - 1836 = {}", eval_poly_exact(&p_coeffs, 6) - 1836);
    // P(6) - 1845 = 0
    println!("  P(6) - 1845 = {}", eval_poly_exact(&p_coeffs, 6) - 1845);
    // P(7) - 3477 = 10
    println!("  P(7) - 3477 = {}", eval_poly_exact(&p_coeffs, 7) - 3477);
    // P(7) - 3487 = 0
    println!("  P(7) - 3487 = {}", eval_poly_exact(&p_coeffs, 7) - 3487);

    // ========================================================================
    // 综合
    // ========================================================================

    println!("\n================================================================================");
    println!("                    S-wind 双多项式理论 - 精确代数结论");
    println!("================================================================================");

    println!("\n[精确整数结果]");
    println!("1. P(D) 在 D=2,3,6,7 处给出精确整数 137, 207, 1845, 3487");
    println!("2. Q(D) 在 D=1,2,3 处精确等于 Fibonacci 数: F_3=2, F_5=5, F_9=34");
    println!("3. Δ⁴P = 48 = 24×2, Δ⁴Q = 24 = 24×1 (精确)");
    println!("4. I_2(D) = P(D) - 2Q(D) 退化为 3 次多项式");

    println!("\n[精确有理数结果]");
    println!("  D=1: P/Q = 115/2 (余数 1)");
    println!("  D=4: P/Q = 415/137 (余数 4)");
    println!("  D=7: P/Q = 3487/1730 (余数 27)");

    println!("\n[新发现的精确关系]");
    println!("  P(D) - 2Q(D) = -D³ + 4D² + 11D + 97 (3次多项式!)");

    println!("\n================================================================================");
    println!("          完成 - S-wind 双多项式精确代数分析");
    println!("================================================================================");
}

/// 精确整数多项式求值
fn eval_poly_exact(coeffs: &[i128], d: i128) -> i128 {
    coeffs[0] * d.pow(4) + coeffs[1] * d.pow(3) + coeffs[2] * d.pow(2) + coeffs[3] * d + coeffs[4]
}

/// 最大公约数 (欧几里得算法)
fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 { a } else { gcd(b, a % b) }
}
