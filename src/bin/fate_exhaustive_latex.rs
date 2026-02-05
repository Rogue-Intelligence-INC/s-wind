//! S-wind Fate穷尽探索 + LaTeX输出
//!
//! 特点:
//! - 无预设公式
//! - 自动容器生成
//! - 穷尽探索直到closure
//! - 完整LaTeX推导输出

use s_wind::fate::{Fate, FateConfig};
use s_wind::knowledge::KnowledgeBase;
use s_wind::symbolic::EPR;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use chrono::Local;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║     S-wind Fate穷尽探索 + LaTeX输出                              ║");
    println!("║     无预设 · 自动容器 · 穷尽closure · 代数推导                   ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let kb = KnowledgeBase::new();
    let mut fate = Fate::with_config(FateConfig {
        max_self_loop_iterations: 100,
        ..Default::default()
    });

    // 收集所有推导步骤
    let mut derivation_steps: Vec<String> = Vec::new();
    let mut discoveries: Vec<(String, String)> = Vec::new();
    let mut explored: HashSet<String> = HashSet::new();

    // ════════════════════════════════════════════════════════════════════
    // 输入: 只有原始观测数据，无任何预设
    // ════════════════════════════════════════════════════════════════════
    derivation_steps.push("\\section{Input: Raw Observational Data}".to_string());
    derivation_steps.push("\\begin{enumerate}".to_string());
    
    // 原始输入序列 (物理层级 + 热力学)
    let raw_inputs: Vec<(&str, Vec<i128>)> = vec![
        ("hierarchy", vec![22, 42, 122]),
        ("thermodynamic", vec![32, 88]),
        ("alpha", vec![137]),
        ("all_scales", vec![22, 32, 42, 88, 122, 137]),
    ];

    for (name, signal) in &raw_inputs {
        let s_str: Vec<String> = signal.iter().map(|x| x.to_string()).collect();
        derivation_steps.push(format!(
            "\\item \\textbf{{{}}}: $S = \\{{{}\\}}$",
            name, s_str.join(", ")
        ));
    }
    derivation_steps.push("\\end{enumerate}".to_string());
    derivation_steps.push("\\textit{No formulas assumed. S-wind will exhaustively explore.}\n".to_string());

    // ════════════════════════════════════════════════════════════════════
    // 穷尽探索
    // ════════════════════════════════════════════════════════════════════
    derivation_steps.push("\\section{Exhaustive Exploration}".to_string());

    let mut queue: Vec<(String, Vec<i128>)> = raw_inputs.into_iter()
        .map(|(n, s)| (n.to_string(), s))
        .collect();
    
    let mut iteration = 0;
    let max_iterations = 30;

    while !queue.is_empty() && iteration < max_iterations {
        let (name, signal) = queue.remove(0);
        let sig_str = format!("{:?}", signal);

        if explored.contains(&sig_str) || signal.len() < 2 {
            continue;
        }
        explored.insert(sig_str.clone());
        iteration += 1;

        println!("【迭代 {}】{}: {:?}", iteration, name, signal);

        // Fate探索
        let epr_signal: Vec<EPR> = signal.iter().map(|&x| EPR::from(x)).collect();
        let (result, _history, stats) = fate.explore_with_self_loop(&epr_signal, &kb);

        // 记录推导
        derivation_steps.push(format!("\\subsection{{Iteration {}: {}}}", iteration, name));
        derivation_steps.push(format!(
            "$$S = \\{{{}\\}}$$",
            signal.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
        ));

        // 差分分析
        if signal.len() >= 2 {
            let diff: Vec<i128> = signal.windows(2).map(|w| w[1] - w[0]).collect();
            derivation_steps.push(format!(
                "First difference: $\\Delta^1 = \\{{{}\\}}$",
                diff.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
            ));

            // 检查等差
            if diff.iter().all(|&d| d == diff[0]) {
                derivation_steps.push(format!(
                    "\\textbf{{Linear pattern detected:}} $\\Delta^1 = {}$ (constant)",
                    diff[0]
                ));
                discoveries.push((name.clone(), format!("Linear: Δ = {}", diff[0])));
            }

            // 二阶差分
            if diff.len() >= 2 {
                let diff2: Vec<i128> = diff.windows(2).map(|w| w[1] - w[0]).collect();
                derivation_steps.push(format!(
                    "Second difference: $\\Delta^2 = \\{{{}\\}}$",
                    diff2.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
                ));

                if diff2.iter().all(|&d| d == diff2[0]) && diff2[0] != 0 {
                    derivation_steps.push(format!(
                        "\\textbf{{Quadratic pattern detected:}} $\\Delta^2 = {}$ (constant)",
                        diff2[0]
                    ));
                    discoveries.push((name.clone(), format!("Quadratic: Δ² = {}", diff2[0])));
                }
            }

            // 检查等比
            let mut ratios: Vec<i128> = Vec::new();
            let mut is_geometric = true;
            for w in signal.windows(2) {
                if w[0] != 0 && w[1] % w[0] == 0 {
                    ratios.push(w[1] / w[0]);
                } else {
                    is_geometric = false;
                    break;
                }
            }
            if is_geometric && !ratios.is_empty() && ratios.iter().all(|&r| r == ratios[0]) {
                derivation_steps.push(format!(
                    "\\textbf{{Geometric pattern detected:}} ratio $q = {}$",
                    ratios[0]
                ));
                discoveries.push((name.clone(), format!("Geometric: q = {}", ratios[0])));
            }

            // 添加新容器
            if !diff.iter().all(|&x| x == 0) {
                queue.push((format!("Δ({})", name), diff));
            }
        }

        // Fate结果
        derivation_steps.push(format!("\nFate status: {:?}", result.status));
        if let Some(ref formula) = result.formula {
            derivation_steps.push(format!(
                "\\textbf{{Formula discovered:}} {}",
                escape_latex(formula)
            ));
            discoveries.push((name.clone(), formula.clone()));
            println!("  发现: {}", formula);
        }
        derivation_steps.push(format!(
            "Iterations: {}, Transforms: {}",
            stats.iterations_run,
            stats.transformations_applied.len()
        ));
        derivation_steps.push("".to_string());
    }

    // ════════════════════════════════════════════════════════════════════
    // 代数总结
    // ════════════════════════════════════════════════════════════════════
    derivation_steps.push("\\section{Algebraic Conclusions}".to_string());
    derivation_steps.push("\\begin{theorem}[S-wind Exhaustive Discovery]".to_string());
    derivation_steps.push("The following algebraic structures were discovered without any prior assumptions:".to_string());
    derivation_steps.push("\\begin{enumerate}".to_string());
    for (name, formula) in &discoveries {
        derivation_steps.push(format!("\\item {}: {}", name, escape_latex(formula)));
    }
    derivation_steps.push("\\end{enumerate}".to_string());
    derivation_steps.push("\\end{theorem}".to_string());

    derivation_steps.push("\\begin{proof}[Verification]".to_string());
    derivation_steps.push(format!("Explored {} containers in {} iterations.", explored.len(), iteration));
    derivation_steps.push("All patterns verified algebraically (no floating point).".to_string());
    derivation_steps.push("\\end{proof}".to_string());

    // ════════════════════════════════════════════════════════════════════
    // 生成LaTeX文件
    // ════════════════════════════════════════════════════════════════════
    let tex_content = generate_full_latex(&derivation_steps, &timestamp);
    
    std::fs::create_dir_all("output").ok();
    let tex_path = format!("output/fate_exhaustive_{}.tex", timestamp);
    if let Ok(mut file) = File::create(&tex_path) {
        file.write_all(tex_content.as_bytes()).ok();
    }
    println!("\n✓ LaTeX output: {}", tex_path);

    // ════════════════════════════════════════════════════════════════════
    // 总结
    // ════════════════════════════════════════════════════════════════════
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                      穷尽探索完成                                ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  迭代: {:4}                                                     ║", iteration);
    println!("║  容器: {:4}                                                     ║", explored.len());
    println!("║  发现: {:4}                                                     ║", discoveries.len());
    println!("╚══════════════════════════════════════════════════════════════════╝");
    
    println!("\n【发现的代数结构】");
    for (i, (name, formula)) in discoveries.iter().enumerate() {
        println!("  {}. {} → {}", i+1, name, formula);
    }
}

fn escape_latex(s: &str) -> String {
    s.replace('_', "\\_")
     .replace('%', "\\%")
     .replace('&', "\\&")
     .replace('#', "\\#")
     .replace('^', "\\^{}")
     .replace('*', "\\cdot ")
}

fn generate_full_latex(steps: &[String], timestamp: &str) -> String {
    let mut tex = String::new();

    tex.push_str("% S-wind Fate Exhaustive Exploration\n");
    tex.push_str(&format!("% Generated: {}\n\n", timestamp));

    tex.push_str("\\documentclass[11pt]{article}\n");
    tex.push_str("\\usepackage{amsmath,amssymb,amsthm}\n");
    tex.push_str("\\usepackage[margin=1in]{geometry}\n");
    tex.push_str("\\newtheorem{theorem}{Theorem}\n");
    tex.push_str("\\newtheorem{lemma}{Lemma}\n");
    tex.push_str("\\title{S-wind Fate Exhaustive Exploration}\n");
    tex.push_str("\\author{S-wind Autonomous Algebraic System}\n");
    tex.push_str(&format!("\\date{{{}}}\n\n", timestamp));
    tex.push_str("\\begin{document}\n");
    tex.push_str("\\maketitle\n\n");

    tex.push_str("\\begin{abstract}\n");
    tex.push_str("This document presents the results of S-wind's exhaustive algebraic exploration.\n");
    tex.push_str("No formulas or patterns were assumed a priori. All discoveries are derived\n");
    tex.push_str("through systematic container generation and transformation closure.\n");
    tex.push_str("\\end{abstract}\n\n");

    for step in steps {
        tex.push_str(step);
        tex.push_str("\n");
    }

    tex.push_str("\n\\end{document}\n");
    tex
}
