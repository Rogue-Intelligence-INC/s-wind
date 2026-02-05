//! S-wind: 23 的代数结构和 W 玻色子预测
//!
//! 关键发现:
//! - P(1) = 5 × 23, P(3) = 9 × 23
//! - 379 × P(4) = 157285 ≈ mW/me (误差 13)
//! - P(2) = Q(4) = 137 是唯一匹配

fn main() {
    println!("═══════════════════════════════════════════════════════════════════════");
    println!("  S-wind: 23 的代数结构和 W/Z 玻色子预测");
    println!("═══════════════════════════════════════════════════════════════════════\n");

    let p_coeffs: [i128; 5] = [2, -5, 4, 15, 99];
    let q_coeffs: [i128; 5] = [1, -2, 0, 2, 1];

    // ═══════════════════════════════════════════════════════════════════════
    // 分析 23 的代数结构
    // ═══════════════════════════════════════════════════════════════════════
    println!("【分析 1】23 的代数结构");
    println!("───────────────────────────────────────────────────────────────────────");
    
    println!("  P(1) = 115 = 5 × 23");
    println!("  P(3) = 207 = 9 × 23");
    println!("  因此 P(1) : P(3) = 5 : 9");
    
    println!("\n  检验: P(D) mod 23 的周期性");
    println!("  D  | P(D) | P(D) mod 23 | P(D)/23 (若整除)");
    println!("  ---|------|-------------|------------------");
    
    for d in -10i128..=15 {
        let pd = eval_p(&p_coeffs, d);
        let r = ((pd % 23) + 23) % 23;
        let div_str = if pd > 0 && pd % 23 == 0 {
            format!("{}", pd / 23)
        } else {
            String::new()
        };
        println!("  {:3} | {:6} | {:11} | {}", d, pd, r, div_str);
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // 23 的物理意义
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n\n【分析 2】23 可能的物理意义");
    println!("───────────────────────────────────────────────────────────────────────");
    
    println!("  李群维数检查:");
    println!("    dim(SU(2)) = 3");
    println!("    dim(SU(3)) = 8");
    println!("    dim(SU(4)) = 15");
    println!("    dim(SU(5)) = 24 ≈ 23 + 1");
    println!("    dim(SO(7)) = 21");
    println!("    dim(SO(8)) = 28");
    println!("    dim(G₂) = 14");
    
    println!("\n  标准模型粒子数:");
    println!("    夸克: 6 种 × 3 色 = 18");
    println!("    轻子: 6 种");
    println!("    总计: 24 种（费米子）≈ 23 + 1");
    
    println!("\n  23 = 素数");
    println!("  23 = 2³ - 1 + 16 = 7 + 16");
    println!("  23 = 137 - 114 = α⁻¹ - 114");
    
    // ═══════════════════════════════════════════════════════════════════════
    // W 玻色子质量的精确表达式搜索
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n\n【分析 3】W 玻色子质量的精确表达式");
    println!("───────────────────────────────────────────────────────────────────────");
    
    // mW ≈ 80.3692 GeV, me ≈ 0.510999 MeV
    // mW/me ≈ 157276 (更精确)
    let mw_me: i128 = 157276;
    
    println!("  mW/me ≈ {} (CODATA 2022)", mw_me);
    
    println!("\n  搜索形如 a×P(x) + b×Q(y) = {} 的精确表达式:", mw_me);
    println!("  (|a|, |b| ≤ 1500, |x|, |y| ≤ 20)");
    
    let mut w_expressions = Vec::new();
    
    for x in -20i128..=20 {
        let px = eval_p(&p_coeffs, x);
        if px <= 0 { continue; }
        
        for a in 1i128..=1500 {
            let apx = a * px;
            if apx > mw_me + 5000 { break; }
            
            // 检查 a×P(x) 是否直接等于 mW/me
            if apx == mw_me {
                w_expressions.push((a, x, 0i128, 0i128, mw_me));
            }
            
            // 检查 a×P(x) + b×Q(y)
            for y in -20i128..=20 {
                let qy = eval_q(&q_coeffs, y);
                if qy == 0 { continue; }
                
                let diff = mw_me - apx;
                if diff % qy == 0 {
                    let b = diff / qy;
                    if b.abs() <= 100 {
                        w_expressions.push((a, x, b, y, apx + b * qy));
                    }
                }
            }
        }
    }
    
    // 按复杂度排序（|a| + |b| 最小）
    w_expressions.sort_by_key(|(a, _, b, _, _)| a.abs() + b.abs());
    w_expressions.dedup();
    
    println!("\n  找到的精确表达式 (前 20 个):");
    for (a, x, b, y, val) in w_expressions.iter().take(20) {
        let px = eval_p(&p_coeffs, *x);
        let qy = if *b != 0 { eval_q(&q_coeffs, *y) } else { 0 };
        
        if *b == 0 {
            println!("    {}×P({}) = {}×{} = {}", a, x, a, px, val);
        } else {
            println!("    {}×P({}) + {}×Q({}) = {}×{} + {}×{} = {}", 
                     a, x, b, y, a, px, b, qy, val);
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // Z 玻色子质量
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n\n【分析 4】Z 玻色子质量的精确表达式");
    println!("───────────────────────────────────────────────────────────────────────");
    
    // mZ ≈ 91.1876 GeV, me ≈ 0.510999 MeV
    // mZ/me ≈ 178450
    let mz_me: i128 = 178450;
    
    println!("  mZ/me ≈ {} (CODATA 2022)", mz_me);
    
    println!("\n  搜索形如 a×P(x) + b×Q(y) = {} 的精确表达式:", mz_me);
    
    let mut z_expressions = Vec::new();
    
    for x in -20i128..=20 {
        let px = eval_p(&p_coeffs, x);
        if px <= 0 { continue; }
        
        for a in 1i128..=1500 {
            let apx = a * px;
            if apx > mz_me + 5000 { break; }
            
            if apx == mz_me {
                z_expressions.push((a, x, 0i128, 0i128, mz_me));
            }
            
            for y in -20i128..=20 {
                let qy = eval_q(&q_coeffs, y);
                if qy == 0 { continue; }
                
                let diff = mz_me - apx;
                if diff % qy == 0 {
                    let b = diff / qy;
                    if b.abs() <= 100 {
                        z_expressions.push((a, x, b, y, apx + b * qy));
                    }
                }
            }
        }
    }
    
    z_expressions.sort_by_key(|(a, _, b, _, _)| a.abs() + b.abs());
    z_expressions.dedup();
    
    println!("\n  找到的精确表达式 (前 20 个):");
    for (a, x, b, y, val) in z_expressions.iter().take(20) {
        let px = eval_p(&p_coeffs, *x);
        let qy = if *b != 0 { eval_q(&q_coeffs, *y) } else { 0 };
        
        if *b == 0 {
            println!("    {}×P({}) = {}×{} = {}", a, x, a, px, val);
        } else {
            println!("    {}×P({}) + {}×Q({}) = {}×{} + {}×{} = {}", 
                     a, x, b, y, a, px, b, qy, val);
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // mW/mZ 比值
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n\n【分析 5】mW/mZ 比值的代数结构");
    println!("───────────────────────────────────────────────────────────────────────");
    
    // mW/mZ ≈ 0.88145 → cos(θW)
    // 整数比: 157276/178450
    let g = gcd(mw_me, mz_me);
    println!("  mW/mZ = {}/{} = {}/{}", mw_me, mz_me, mw_me/g, mz_me/g);
    println!("  约化比: {}/{}", mw_me/g, mz_me/g);
    
    // 检查 P(a)/P(b) 或 Q(a)/Q(b) 是否接近这个比值
    println!("\n  搜索 P(a)/P(b) ≈ mW/mZ:");
    for a in 1i128..=20 {
        for b in 1i128..=20 {
            if a != b {
                let pa = eval_p(&p_coeffs, a);
                let pb = eval_p(&p_coeffs, b);
                if pa > 0 && pb > 0 {
                    // 检查 pa * mz_me 是否接近 pb * mw_me
                    let lhs = pa * mz_me;
                    let rhs = pb * mw_me;
                    let diff = (lhs - rhs).abs();
                    // 允许 0.5% 误差
                    if diff * 200 < lhs {
                        println!("    P({})/P({}) = {}/{} ≈ {:.6}", 
                                 a, b, pa, pb, pa as f64 / pb as f64);
                        println!("      mW/mZ ≈ {:.6}", mw_me as f64 / mz_me as f64);
                    }
                }
            }
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // Higgs 质量
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n\n【分析 6】Higgs 玻色子质量");
    println!("───────────────────────────────────────────────────────────────────────");
    
    // mH ≈ 125.25 GeV, me ≈ 0.510999 MeV
    // mH/me ≈ 245100
    let mh_me: i128 = 245100;
    
    println!("  mH/me ≈ {} (CODATA 2022)", mh_me);
    
    println!("\n  搜索形如 a×P(x) + b×Q(y) = {} 的精确表达式:", mh_me);
    
    let mut h_expressions = Vec::new();
    
    for x in -20i128..=20 {
        let px = eval_p(&p_coeffs, x);
        if px <= 0 { continue; }
        
        for a in 1i128..=2000 {
            let apx = a * px;
            if apx > mh_me + 5000 { break; }
            
            if apx == mh_me {
                h_expressions.push((a, x, 0i128, 0i128, mh_me));
            }
            
            for y in -20i128..=20 {
                let qy = eval_q(&q_coeffs, y);
                if qy == 0 { continue; }
                
                let diff = mh_me - apx;
                if diff % qy == 0 {
                    let b = diff / qy;
                    if b.abs() <= 100 {
                        h_expressions.push((a, x, b, y, apx + b * qy));
                    }
                }
            }
        }
    }
    
    h_expressions.sort_by_key(|(a, _, b, _, _)| a.abs() + b.abs());
    h_expressions.dedup();
    
    println!("\n  找到的精确表达式 (前 15 个):");
    for (a, x, b, y, val) in h_expressions.iter().take(15) {
        let px = eval_p(&p_coeffs, *x);
        let qy = if *b != 0 { eval_q(&q_coeffs, *y) } else { 0 };
        
        if *b == 0 {
            println!("    {}×P({}) = {}×{} = {}", a, x, a, px, val);
        } else {
            println!("    {}×P({}) + {}×Q({}) = {}×{} + {}×{} = {}", 
                     a, x, b, y, a, px, b, qy, val);
        }
    }
    
    // ═══════════════════════════════════════════════════════════════════════
    // 总结: OUT-OF-SAMPLE 预测
    // ═══════════════════════════════════════════════════════════════════════
    println!("\n\n═══════════════════════════════════════════════════════════════════════");
    println!("  ★ 玻色子质量的代数预测总结 ★");
    println!("═══════════════════════════════════════════════════════════════════════\n");
    
    println!("  这些玻色子质量是 OUT-OF-SAMPLE (未参与原始搜索)");
    println!("  如果能找到简洁的精确表达式，将是强有力的验证\n");
    
    // 找最简洁的表达式
    if let Some((a, x, b, y, _)) = w_expressions.first() {
        let px = eval_p(&p_coeffs, *x);
        if *b == 0 {
            println!("  mW/me 最简表达: {}×P({}) = {}×{}", a, x, a, px);
        } else {
            let qy = eval_q(&q_coeffs, *y);
            println!("  mW/me 最简表达: {}×P({}) + {}×Q({}) = {}×{} + {}×{}", 
                     a, x, b, y, a, px, b, qy);
        }
    }
    
    if let Some((a, x, b, y, _)) = z_expressions.first() {
        let px = eval_p(&p_coeffs, *x);
        if *b == 0 {
            println!("  mZ/me 最简表达: {}×P({}) = {}×{}", a, x, a, px);
        } else {
            let qy = eval_q(&q_coeffs, *y);
            println!("  mZ/me 最简表达: {}×P({}) + {}×Q({}) = {}×{} + {}×{}", 
                     a, x, b, y, a, px, b, qy);
        }
    }
    
    if let Some((a, x, b, y, _)) = h_expressions.first() {
        let px = eval_p(&p_coeffs, *x);
        if *b == 0 {
            println!("  mH/me 最简表达: {}×P({}) = {}×{}", a, x, a, px);
        } else {
            let qy = eval_q(&q_coeffs, *y);
            println!("  mH/me 最简表达: {}×P({}) + {}×Q({}) = {}×{} + {}×{}", 
                     a, x, b, y, a, px, b, qy);
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// 辅助函数
// ═══════════════════════════════════════════════════════════════════════

fn eval_p(coeffs: &[i128; 5], d: i128) -> i128 {
    coeffs[0] * d.pow(4) + coeffs[1] * d.pow(3) + 
    coeffs[2] * d.pow(2) + coeffs[3] * d + coeffs[4]
}

fn eval_q(coeffs: &[i128; 5], d: i128) -> i128 {
    coeffs[0] * d.pow(4) + coeffs[1] * d.pow(3) + 
    coeffs[2] * d.pow(2) + coeffs[3] * d + coeffs[4]
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}
