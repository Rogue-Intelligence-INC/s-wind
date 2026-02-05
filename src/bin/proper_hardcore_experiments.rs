//! S-wind 纯探索实验
//!
//! 只输入原始数据，不预设任何公式
//! 让S-wind穷尽探索并输出LaTeX

use s_wind::interface::{
    SignalBuilder, ExperimentRunner, ExperimentConfig,
    print_capabilities,
};
use std::io::{self, BufRead, Write};

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════════════════╗");
    println!("║   S-wind 纯探索模式                                                              ║");
    println!("║                                                                                  ║");
    println!("║   输入任意序列 → 自动穷尽 → LaTeX输出                                           ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════════╝\n");

    print_capabilities();
    println!();

    // 交互式模式
    println!("┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓");
    println!("┃ 输入格式:                                                                       ┃");
    println!("┃   整数: 1, 2, 3, 4, 5                                                          ┃");
    println!("┃   分数: 1/2, 1, 3/2, 2                                                         ┃");
    println!("┃   小数: 0.5, 1.0, 1.5, 2.0                                                     ┃");
    println!("┃                                                                                 ┃");
    println!("┃ 输入 'q' 退出                                                                  ┃");
    println!("┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛\n");

    let stdin = io::stdin();
    let mut experiment_count = 0;

    loop {
        print!("\n实验名称 (或 'q' 退出): ");
        io::stdout().flush().unwrap();
        
        let mut name = String::new();
        stdin.lock().read_line(&mut name).unwrap();
        let name = name.trim();
        
        if name == "q" || name == "quit" {
            break;
        }
        
        if name.is_empty() {
            experiment_count += 1;
            format!("experiment_{}", experiment_count);
        }
        
        print!("输入序列: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();
        
        if input.is_empty() {
            println!("  ⚠ 空输入，跳过");
            continue;
        }

        // 解析输入
        match SignalBuilder::from_string(input) {
            Ok(builder) => {
                let signal = builder.build();
                
                println!("\n  解析结果: {} 个数据点", signal.len());
                println!("  序列: {:?}", signal.iter().map(|r| r.to_string()).collect::<Vec<_>>());
                
                // 运行实验
                let config = ExperimentConfig {
                    name: if name.is_empty() { 
                        format!("experiment_{}", experiment_count) 
                    } else { 
                        name.to_string() 
                    },
                    output_dir: "output".to_string(),
                    generate_latex: true,
                    generate_markdown: true,
                    verbose: true,
                };
                
                let mut runner = ExperimentRunner::new(config.clone());
                let result = runner.run(signal);
                
                println!("\n  ┌──────────────────────────────────────────────────────────────┐");
                println!("  │ 探索结果                                                      │");
                println!("  ├──────────────────────────────────────────────────────────────┤");
                println!("  │ 检测成功: {:50} │", if result.is_success() { "✅ YES" } else { "❌ NO" });
                println!("  │ 推导步数: {:50} │", result.step_count());
                
                if let Some(eq_type) = result.equation_type() {
                    println!("  │ 方程类型: {:50} │", format!("{:?}", eq_type).chars().take(50).collect::<String>());
                }
                
                println!("  └──────────────────────────────────────────────────────────────┘");
                
                println!("\n  生成文件:");
                println!("    output/{}_answer_*.tex", config.name);
                println!("    output/{}_derivation_*.tex", config.name);
                println!("    output/{}_reverse_*.tex", config.name);
                println!("    output/{}_report_*.md", config.name);
            }
            Err(e) => {
                println!("  ⚠ 解析错误: {}", e);
            }
        }
    }

    println!("\n再见！");
}
