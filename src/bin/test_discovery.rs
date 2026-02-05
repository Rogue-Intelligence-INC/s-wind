//! 测试 Fate 的新发现机制

use s_wind::{
    Fate, KnowledgeBase, EPR, DiscoveryType,
    fate::{SignalDomain},
};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           Fate 新发现机制测试                                    ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    let mut fate = Fate::new();

    // 1. 测试已知模式检测 (MathPattern)
    println!("═══════════════════════════════════════════════════════════════════");
    println!("1. 已知模式检测 (MathPattern)");
    println!("═══════════════════════════════════════════════════════════════════");
    
    let squares: Vec<EPR> = vec![1, 4, 9, 16, 25, 36, 49].into_iter()
        .map(EPR::from_i128).collect();
    let result = fate.explore(&squares, &kb);
    println!("输入: [1, 4, 9, 16, 25, 36, 49]");
    println!("状态: {:?}", result.status);
    println!("公式: {:?}", result.formula);
    println!();

    // 2. 测试未知但有规律的序列 (NewRule)
    println!("═══════════════════════════════════════════════════════════════════");
    println!("2. 未知但有规律的序列 (候选 NewRule)");
    println!("═══════════════════════════════════════════════════════════════════");
    
    // 一个不匹配标准模式但有某种规律的序列
    let mystery1: Vec<EPR> = vec![1, 3, 7, 15, 31, 63, 127].into_iter()  // 2^n - 1
        .map(EPR::from_i128).collect();
    let result = fate.explore(&mystery1, &kb);
    println!("输入: [1, 3, 7, 15, 31, 63, 127]  (2^n - 1)");
    println!("状态: {:?}", result.status);
    for line in &result.exploration_log {
        if line.contains("发现") || line.contains("候选") || line.contains("结构") {
            println!("  {}", line);
        }
    }
    println!();

    // 3. 测试跨域同构 (SystemPattern)
    println!("═══════════════════════════════════════════════════════════════════");
    println!("3. 跨域同构检测 (SystemPattern)");
    println!("═══════════════════════════════════════════════════════════════════");
    
    // 先从数学域输入
    let math_signal: Vec<EPR> = vec![1, 2, 4, 8, 16, 32].into_iter()
        .map(EPR::from_i128).collect();
    let _ = fate.explore_with_domain(&math_signal, &kb, SignalDomain::Mathematics);
    println!("数学域: [1, 2, 4, 8, 16, 32]");
    
    // 再从物理域输入相同结构
    let physics_signal: Vec<EPR> = vec![1, 2, 4, 8, 16, 32].into_iter()
        .map(EPR::from_i128).collect();
    let result = fate.explore_with_domain(&physics_signal, &kb, SignalDomain::PhysicsQuantum);
    println!("物理域: [1, 2, 4, 8, 16, 32]");
    println!("状态: {:?}", result.status);
    for line in &result.exploration_log {
        if line.contains("跨域") || line.contains("SystemPattern") {
            println!("  {}", line);
        }
    }
    println!();

    // 4. 测试结构提取
    println!("═══════════════════════════════════════════════════════════════════");
    println!("4. 结构提取测试");
    println!("═══════════════════════════════════════════════════════════════════");
    
    let test_sequences = vec![
        ("线性", vec![2, 4, 6, 8, 10, 12]),
        ("二次", vec![1, 4, 9, 16, 25, 36]),
        ("指数", vec![1, 2, 4, 8, 16, 32]),
        ("阶乘", vec![1, 2, 6, 24, 120, 720]),
        ("调和分子", vec![1, 3, 11, 25, 137, 49]),
        ("Pi连分数", vec![3, 7, 15, 1, 292, 1]),
    ];
    
    for (name, data) in test_sequences {
        let signal: Vec<EPR> = data.into_iter().map(EPR::from_i128).collect();
        let structure = fate.extract_structure(&signal, SignalDomain::External);
        println!("{}: {:?} (一致性: {})", name, structure.structure_type, structure.consistency_score);
    }
    println!();

    // 5. 检查所有发现
    println!("═══════════════════════════════════════════════════════════════════");
    println!("5. 所有发现统计");
    println!("═══════════════════════════════════════════════════════════════════");
    
    let discoveries = fate.discoveries();
    
    let math_patterns = discoveries.iter().filter(|d| d.discovery_type == DiscoveryType::MathPattern).count();
    let system_patterns = discoveries.iter().filter(|d| d.discovery_type == DiscoveryType::SystemPattern).count();
    let new_rules = discoveries.iter().filter(|d| d.discovery_type == DiscoveryType::NewRule).count();
    let level_ups = discoveries.iter().filter(|d| d.discovery_type == DiscoveryType::LevelUp).count();
    
    println!("总发现数: {}", discoveries.len());
    println!("  - MathPattern (已知模式): {}", math_patterns);
    println!("  - SystemPattern (跨域同构): {}", system_patterns);
    println!("  - NewRule (新规则候选): {}", new_rules);
    println!("  - LevelUp (泛化发现): {}", level_ups);
    println!();
    
    println!("详细发现列表:");
    for d in discoveries {
        println!("  [{:?}] {} - {}", d.discovery_type, d.id, d.description);
    }
    println!();

    // Fate 统计
    let stats = fate.stats();
    println!("Fate 统计:");
    println!("  - tick: {}", stats.tick);
    println!("  - 信号数: {}", stats.signal_count);
    println!("  - 发现数: {}", stats.discovery_count);
    println!("  - 模式多样性: {}", stats.pattern_diversity);
}
