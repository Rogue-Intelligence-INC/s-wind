//! Fate 实验 - 使用统一 Fate (EPR 无限精度)

use s_wind::{
    Fate, KnowledgeBase, EPR,
    experiments::run_all_experiments,
};

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║       S-wind Fate 实验 (EPR 无限精度)                          ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
    
    // 初始化
    let mut fate = Fate::new();
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    
    println!("知识库状态:");
    println!("  - 已知序列: {}", kb.known_sequences.len());
    println!("  - 模式类: {}", kb.pattern_classes.len());
    println!();
    
    // 运行标准实验
    println!("═══════════════════════════════════════════════════════════════════");
    println!("运行标准实验...\n");
    
    let results = run_all_experiments();
    
    let mut success_count = 0;
    let mut known_count = 0;
    let mut pattern_count = 0;
    let mut unknown_count = 0;
    
    for (name, result) in &results {
        let status_str = match &result.status {
            s_wind::FateStatus::KnownSequence { id, name } => {
                known_count += 1;
                success_count += 1;
                format!("✓ 已知序列: {} ({})", name, id)
            }
            s_wind::FateStatus::PatternDetected { class_id, formula } => {
                pattern_count += 1;
                success_count += 1;
                format!("✓ 检测模式: {} [{}]", formula, class_id)
            }
            s_wind::FateStatus::Unknown => {
                unknown_count += 1;
                "? 未知模式".to_string()
            }
            s_wind::FateStatus::Mystery { iterations_exhausted, ref reason, .. } => {
                unknown_count += 1;
                format!("🔮 谜团: {} 轮探索后穷尽, {}", iterations_exhausted, reason)
            }
        };
        
        println!("[{}] {}", name, status_str);
    }
    
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("实验统计:");
    println!("  总计: {}", results.len());
    println!("  成功: {} (已知: {}, 检测: {})", success_count, known_count, pattern_count);
    println!("  未知: {}", unknown_count);
    println!();
    
    // 测试 EPR 高精度能力
    println!("═══════════════════════════════════════════════════════════════════");
    println!("EPR 高精度测试...\n");
    
    // 计算 50!
    let mut factorial = EPR::one();
    for i in 1..=50 {
        factorial = factorial * EPR::from_i128(i);
    }
    println!("50! = {}", factorial);
    println!("50! 位数 ≈ {} bits", factorial.bits());
    
    // 阶乘序列检测
    let factorial_signal: Vec<EPR> = (1..=10)
        .scan(EPR::one(), |acc, i| {
            *acc = acc.clone() * EPR::from_i128(i);
            Some(acc.clone())
        })
        .collect();
    
    println!("\n阶乘信号 (1! to 10!):");
    for (i, f) in factorial_signal.iter().enumerate() {
        println!("  {}! = {}", i + 1, f);
    }
    
    let result = fate.explore(&factorial_signal, &kb);
    println!("\n检测结果: {:?}", result.status);
    if let Some(formula) = &result.formula {
        println!("公式: {}", formula);
    }
    
    // Fate 统计
    let stats = fate.stats();
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("Fate 统计:");
    println!("  tick: {}", stats.tick);
    println!("  信号数: {}", stats.signal_count);
    println!("  发现数: {}", stats.discovery_count);
    println!("  模式多样性: {}", stats.pattern_diversity);
    
    println!("\n完成!");
}
