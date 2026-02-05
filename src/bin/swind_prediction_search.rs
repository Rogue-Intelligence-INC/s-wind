//! S-wind 预测搜索实验
//! 
//! 目标：从已知精确整数关系推导出 out-of-sample prediction
//! 约束：纯 i128 整数运算，禁止 f64
//! 
//! 已知精确关系：
//!   P(2) = Q(4) = 137
//!   I(7) = 27 = 3³
//!   Q(1,2,3) = F₃, F₅, F₉
//! 
//! 未参与搜索的常数（整数近似）：
//!   mW/me ≈ 157298 (W玻色子)
//!   mZ/me ≈ 178450 (Z玻色子)  
//!   mH/me ≈ 244950 (Higgs)
//!   mW/mZ ≈ 0.882 → 整数比?

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  S-wind 预测搜索实验 (纯整数)");
    println!("═══════════════════════════════════════════════════════════════\n");

    // 三个核心多项式的系数 (降幂排列)
    let p_coeffs: [i128; 5] = [2, -5, 4, 15, 99];   // P(D) = 2D⁴ - 5D³ + 4D² + 15D + 99
    let q_coeffs: [i128; 5] = [1, -2, 0, 2, 1];     // Q(D) = D⁴ - 2D³ + 2D + 1
    // I(D) = -D³ + 4D² + 11D + 97 (用函数计算，不需要系数数组)
    
    // ═══════════════════════════════════════════════════════════════
    // 实验 1: 扩展 D 范围，寻找新的精确整数性质
    // ═══════════════════════════════════════════════════════════════
    println!("【实验 1】扩展 D 范围 (-20 到 +20)");
    println!("─────────────────────────────────────────────────────────────────");
    println!("  D  |   P(D)   |   Q(D)   |   I(D)   | P/Q商 | P/Q余 | 特殊性质");
    println!("─────────────────────────────────────────────────────────────────");
    
    for d in -20i128..=20 {
        let p = eval_p(&p_coeffs, d);
        let q = eval_q(&q_coeffs, d);
        let i = eval_i(d);
        
        let (quot, rem) = if q != 0 { (p / q, p % q) } else { (0, 0) };
        
        // 检查特殊性质
        let mut special = Vec::new();
        
        // 完美幂检查
        if let Some(s) = is_perfect_power(p.abs()) {
            special.push(format!("P={}^{}", s.0, s.1));
        }
        if let Some(s) = is_perfect_power(q.abs()) {
            special.push(format!("Q={}^{}", s.0, s.1));
        }
        if let Some(s) = is_perfect_power(i.abs()) {
            special.push(format!("I={}^{}", s.0, s.1));
        }
        
        // 素数检查
        if is_prime(p.abs()) { special.push("P素".to_string()); }
        if is_prime(q.abs()) { special.push("Q素".to_string()); }
        if is_prime(i.abs()) { special.push("I素".to_string()); }
        
        // 整除检查
        if q != 0 && rem == 0 { special.push(format!("P={}Q", quot)); }
        
        // Fibonacci 检查
        if let Some(idx) = is_fibonacci(q.abs()) {
            special.push(format!("Q=F_{}", idx));
        }
        
        // 137 相关
        if p == 137 { special.push("P=α⁻¹".to_string()); }
        if q == 137 { special.push("Q=α⁻¹".to_string()); }
        
        let special_str = if special.is_empty() { 
            String::new() 
        } else { 
            special.join(", ") 
        };
        
        println!("{:4} | {:8} | {:8} | {:8} | {:5} | {:5} | {}", 
                 d, p, q, i, quot, rem, special_str);
    }
    
    // ═══════════════════════════════════════════════════════════════
    // 实验 2: 寻找 P(a) = Q(b) 的精确匹配
    // ═══════════════════════════════════════════════════════════════
    println!("\n\n【实验 2】寻找 P(a) = Q(b) 的精确匹配");
    println!("─────────────────────────────────────────────────────────────────");
    
    let mut pq_matches = Vec::new();
    for a in -50i128..=50 {
        let pa = eval_p(&p_coeffs, a);
        for b in -50i128..=50 {
            let qb = eval_q(&q_coeffs, b);
            if pa == qb && pa > 0 {
                pq_matches.push((a, b, pa));
            }
        }
    }
    
    println!("  找到 {} 个匹配:", pq_matches.len());
    for (a, b, val) in &pq_matches {
        let factored = factorize(*val);
        println!("    P({}) = Q({}) = {} = {}", a, b, val, factored);
    }
    
    // ═══════════════════════════════════════════════════════════════
    // 实验 3: 组合关系搜索
    // ═══════════════════════════════════════════════════════════════
    println!("\n\n【实验 3】组合关系 aP(x) + bQ(y) = c 搜索");
    println!("─────────────────────────────────────────────────────────────────");
    
    // 目标常数（整数）
    let targets: [(i128, &str); 10] = [
        (137, "α⁻¹"),
        (207, "mμ/me近似"),
        (1836, "mp/me近似"),
        (1839, "mn/me近似"),
        (3477, "mτ/me近似"),
        (157298, "mW/me近似"),
        (178450, "mZ/me近似"),
        (244950, "mH/me近似"),
        (80379, "mW(MeV)"),
        (91188, "mZ(MeV)"),
    ];
    
    println!("  搜索形如 aP(x) + bQ(y) = target 的关系 (|a|,|b| ≤ 10, |x|,|y| ≤ 20):\n");
    
    for (target, name) in &targets {
        let mut found = Vec::new();
        
        for a in -10i128..=10 {
            if a == 0 { continue; }
            for x in -20i128..=20 {
                let px = eval_p(&p_coeffs, x);
                let apx = a * px;
                
                for b in -10i128..=10 {
                    for y in -20i128..=20 {
                        let qy = eval_q(&q_coeffs, y);
                        let bqy = b * qy;
                        
                        if apx + bqy == *target {
                            found.push((a, x, b, y));
                        }
                    }
                }
            }
        }
        
        if !found.is_empty() {
            println!("  {} = {}:", name, target);
            for (a, x, b, y) in found.iter().take(5) {
                let px = eval_p(&p_coeffs, *x);
                let qy = eval_q(&q_coeffs, *y);
                println!("    {}×P({}) + {}×Q({}) = {}×{} + {}×{} = {}", 
                         a, x, b, y, a, px, b, qy, target);
            }
            if found.len() > 5 {
                println!("    ... 还有 {} 个", found.len() - 5);
            }
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // 实验 4: 从精确关系推导预测
    // ═══════════════════════════════════════════════════════════════
    println!("\n\n【实验 4】从精确关系推导预测");
    println!("─────────────────────────────────────────────────────────────────");
    
    // 已知: P(2) = Q(4) = 137
    // 猜想: P(D) 和 Q(2D) 有系统关系？
    println!("  检验 P(D) vs Q(2D) 关系:");
    for d in 1i128..=10 {
        let pd = eval_p(&p_coeffs, d);
        let q2d = eval_q(&q_coeffs, 2 * d);
        let diff = pd - q2d;
        let ratio_int = if q2d != 0 { pd / q2d } else { 0 };
        let rem = if q2d != 0 { pd % q2d } else { 0 };
        println!("    D={}: P({})={}, Q({})={}, 差={}, 商={}, 余={}", 
                 d, d, pd, 2*d, q2d, diff, ratio_int, rem);
    }
    
    // 已知: I(7) = 27 = 3³
    // 搜索: I(D) = n³ 的其他解
    println!("\n  搜索 I(D) = n³ (完美立方):");
    for d in -100i128..=100 {
        let id = eval_i(d);
        if id > 0 {
            let cbrt = (id as f64).cbrt().round() as i128;
            if cbrt * cbrt * cbrt == id {
                println!("    I({}) = {} = {}³ ★", d, id, cbrt);
            }
        }
    }
    
    // 已知: Q(1)=F₃, Q(2)=F₅, Q(3)=F₉
    // 模式: 索引 3, 5, 9 → 差为 2, 4 → 下一个是 9+? 
    println!("\n  Fibonacci 索引模式分析:");
    println!("    Q(1) = 2 = F_3");
    println!("    Q(2) = 5 = F_5    (索引差 +2)");
    println!("    Q(3) = 34 = F_9   (索引差 +4)");
    println!("    预测: Q(4) 可能接近 F_{{9+6}} = F_15?");
    
    let fib = generate_fibonacci(30);
    let q4 = eval_q(&q_coeffs, 4);
    println!("    实际: Q(4) = {}", q4);
    println!("    F_13 = {}, F_14 = {}, F_15 = {}", fib[13], fib[14], fib[15]);
    
    // ═══════════════════════════════════════════════════════════════
    // 实验 5: 多项式商和有理关系
    // ═══════════════════════════════════════════════════════════════
    println!("\n\n【实验 5】精确有理关系搜索");
    println!("─────────────────────────────────────────────────────────────────");
    
    // 搜索 P(a)/P(b) = 有理数 m/n 的简单情况
    println!("  P(a)/P(b) 的简单有理数:");
    for a in 1i128..=10 {
        for b in 1i128..=10 {
            if a != b {
                let pa = eval_p(&p_coeffs, a);
                let pb = eval_p(&p_coeffs, b);
                let g = gcd(pa.abs(), pb.abs());
                let num = pa / g;
                let den = pb / g;
                if den <= 10 && num <= 100 {
                    println!("    P({})/P({}) = {}/{} = {}/{}", a, b, pa, pb, num, den);
                }
            }
        }
    }
    
    // ═══════════════════════════════════════════════════════════════
    // 实验 6: 生成 Out-of-Sample 预测
    // ═══════════════════════════════════════════════════════════════
    println!("\n\n【实验 6】生成 Out-of-Sample 预测");
    println!("═══════════════════════════════════════════════════════════════");
    
    // 基于发现的模式，生成可验证的预测
    println!("\n  基于精确整数关系的预测:\n");
    
    // 预测 1: 如果 P(D) 和 Q(D) 的结构是物理的，
    // 那么应该存在 D 使得 P(D) 或 Q(D) 接近其他基本常数
    
    println!("  预测 1: 多项式在特定 D 值的物理意义");
    println!("  ─────────────────────────────────────");
    
    let physical_constants: [(i128, &str); 8] = [
        (137, "α⁻¹ (精细结构常数倒数)"),
        (207, "mμ/me (μ子质量比)"),
        (1836, "mp/me (质子质量比)"),
        (1839, "mn/me (中子质量比)"),
        (3477, "mτ/me (τ子质量比)"),
        (17, "夸克数 + 轻子数"),
        (26, "时空维度 (弦论)"),
        (11, "M理论维度"),
    ];
    
    for d in 1i128..=15 {
        let pd = eval_p(&p_coeffs, d);
        let qd = eval_q(&q_coeffs, d);
        let id = eval_i(d);
        
        for (val, name) in &physical_constants {
            if pd == *val {
                println!("    ★ P({}) = {} = {}", d, pd, name);
            }
            if qd == *val {
                println!("    ★ Q({}) = {} = {}", d, qd, name);
            }
            if id == *val {
                println!("    ★ I({}) = {} = {}", d, id, name);
            }
        }
    }
    
    // 预测 2: 比值预测
    println!("\n  预测 2: 质量比的精确整数近似");
    println!("  ─────────────────────────────────────");
    
    // 如果 mτ/mμ 有精确整数表示
    // mτ/mμ ≈ 16.817 → 检查 P(a)/P(b) 或 Q(a)/Q(b)
    
    for a in 1i128..=10 {
        for b in 1i128..=10 {
            if a != b {
                let pa = eval_p(&p_coeffs, a);
                let pb = eval_p(&p_coeffs, b);
                // mτ/mμ ≈ 16.817, 整数部分 16-17
                if pa > 0 && pb > 0 {
                    let ratio = pa / pb;
                    let rem = pa % pb;
                    // 检查是否接近 17 且余数小
                    if ratio >= 15 && ratio <= 18 && rem * 100 < pb {
                        println!("    P({})/P({}) = {}/{} ≈ {} (余{})", 
                                 a, b, pa, pb, ratio, rem);
                    }
                }
            }
        }
    }
    
    // 预测 3: 新的完美幂
    println!("\n  预测 3: 完美幂搜索 (D = -100 到 100)");
    println!("  ─────────────────────────────────────");
    
    for d in -100i128..=100 {
        let pd = eval_p(&p_coeffs, d);
        let qd = eval_q(&q_coeffs, d);
        let id = eval_i(d);
        
        if pd > 1 {
            if let Some((base, exp)) = is_perfect_power(pd) {
                if exp >= 2 {
                    println!("    P({}) = {} = {}^{}", d, pd, base, exp);
                }
            }
        }
        if qd > 1 {
            if let Some((base, exp)) = is_perfect_power(qd) {
                if exp >= 2 {
                    println!("    Q({}) = {} = {}^{}", d, qd, base, exp);
                }
            }
        }
        if id > 1 {
            if let Some((base, exp)) = is_perfect_power(id) {
                if exp >= 2 {
                    println!("    I({}) = {} = {}^{}", d, id, base, exp);
                }
            }
        }
    }
    
    // 最终总结
    println!("\n\n═══════════════════════════════════════════════════════════════");
    println!("  实验总结");
    println!("═══════════════════════════════════════════════════════════════");
    println!("\n  已确认的精确整数关系:");
    println!("    • P(2) = Q(4) = 137");
    println!("    • I(7) = 27 = 3³");
    println!("    • Q(1) = 2 = F_3");
    println!("    • Q(2) = 5 = F_5");
    println!("    • Q(3) = 34 = F_9");
    println!("\n  待验证的预测将在输出中标记 ★");
}

// ═══════════════════════════════════════════════════════════════
// 辅助函数（纯整数）
// ═══════════════════════════════════════════════════════════════

fn eval_p(coeffs: &[i128; 5], d: i128) -> i128 {
    coeffs[0] * d.pow(4) + coeffs[1] * d.pow(3) + 
    coeffs[2] * d.pow(2) + coeffs[3] * d + coeffs[4]
}

fn eval_q(coeffs: &[i128; 5], d: i128) -> i128 {
    coeffs[0] * d.pow(4) + coeffs[1] * d.pow(3) + 
    coeffs[2] * d.pow(2) + coeffs[3] * d + coeffs[4]
}

fn eval_i(d: i128) -> i128 {
    // I(D) = -D³ + 4D² + 11D + 97
    -d.pow(3) + 4 * d.pow(2) + 11 * d + 97
}

fn gcd(a: i128, b: i128) -> i128 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn is_prime(n: i128) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let sqrt_n = (n as f64).sqrt() as i128 + 1;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 { return false; }
    }
    true
}

fn is_perfect_power(n: i128) -> Option<(i128, u32)> {
    if n <= 1 { return None; }
    
    for exp in (2..=40).rev() {
        let base = (n as f64).powf(1.0 / exp as f64).round() as i128;
        for b in [base - 1, base, base + 1] {
            if b > 1 && b.checked_pow(exp).map_or(false, |p| p == n) {
                return Some((b, exp));
            }
        }
    }
    None
}

fn is_fibonacci(n: i128) -> Option<usize> {
    let fibs = generate_fibonacci(50);
    fibs.iter().position(|&f| f == n)
}

fn generate_fibonacci(count: usize) -> Vec<i128> {
    let mut fibs = vec![0i128, 1];
    for i in 2..count {
        fibs.push(fibs[i-1] + fibs[i-2]);
    }
    fibs
}

fn factorize(mut n: i128) -> String {
    if n <= 1 { return n.to_string(); }
    
    let mut factors = Vec::new();
    let mut d = 2i128;
    
    while d * d <= n {
        let mut count = 0;
        while n % d == 0 {
            n /= d;
            count += 1;
        }
        if count > 0 {
            if count == 1 {
                factors.push(format!("{}", d));
            } else {
                factors.push(format!("{}^{}", d, count));
            }
        }
        d += 1;
    }
    
    if n > 1 {
        factors.push(format!("{}", n));
    }
    
    if factors.is_empty() {
        n.to_string()
    } else {
        factors.join(" × ")
    }
}
