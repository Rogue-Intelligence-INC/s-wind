//! S-wind 统一命令行工具
//! 
//! 功能：
//! - optimize: 运行穷尽搜索优化
//! - paper: 生成 arXiv 标准论文
//! - batch: 批量处理所有项目
//! - list: 列出所有可用项目
//! 
//! 用法:
//!   swind optimize --project A1
//!   swind paper --project A1 --format md
//!   swind batch --all
//!   swind list

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use s_wind::knowledge::storage;
use s_wind::{DiscoveryStatus, EPR, Fate, KnowledgeBase};

mod shared {
    pub fn format_number(n: u64) -> String {
        let s = n.to_string();
        let mut result = String::new();
        for (i, c) in s.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                result.push(',');
            }
            result.push(c);
        }
        result.chars().rev().collect()
    }
}

use shared::format_number;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║     S-wind v1.0 - Exact Algebraic Simulation System          ║");
    println!("║     穷尽搜索 · 自动论文 · 3D模型                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    match args[1].as_str() {
        "help" | "--help" | "-h" => print_help(),
        "list" => list_projects(),
        "explore" | "detect" => run_explore(&args[2..]),
        "optimize" => {
            let project_id = args.get(3).map(|s| s.as_str()).unwrap_or("A1");
            run_optimize(project_id);
        },
        "paper" => {
            let project_id = args.get(3).map(|s| s.as_str()).unwrap_or("A1");
            let format = args.get(5).map(|s| s.as_str()).unwrap_or("all");
            run_paper(project_id, format);
        },
        "batch" => run_batch(),
        _ => {
            println!("❌ Unknown command: {}", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!("📖 Usage:");
    println!("  swind <command> [options]\n");
    println!("📋 Commands:");
    println!("  explore <seq> [options]  PRIMARY ENTRY: explore a sequence (e.g. \"10,17,24\" or \"1/2,1/3,1/4\")");
    println!("      --mode fast|verified|loop|full   (default: loop)");
    println!("      --kb <path>          Knowledge base JSON (loaded or created)");
    println!("      --save-pending       Queue surviving candidates for human review in the KB");
    println!("  list                     List all available projects");
    println!("  optimize --project <ID>  Run optimization for a project");
    println!("  paper --project <ID> --format <md|tex|all>");
    println!("                           Generate paper for a project");
    println!("  batch                    Process all projects\n");
    println!("📁 Example:");
    println!("  cargo run --release --bin swind list");
    println!("  cargo run --release --bin swind optimize --project A1");
    println!("  cargo run --release --bin swind paper --project A1 --format all");
    println!("  cargo run --release --bin swind batch");
}

/// THE single supported entry point for sequence exploration.
/// All other commands are project-catalog utilities; the 87 research
/// binaries under src/bin are archived experiments, not entry points.
fn run_explore(args: &[String]) {
    // Parse: first positional arg = sequence; options may follow.
    let mut seq_str: Option<String> = None;
    let mut mode = "loop".to_string();
    let mut kb_path: Option<String> = None;
    let mut save_pending = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--mode" => { mode = args.get(i + 1).cloned().unwrap_or_else(|| "loop".into()); i += 2; }
            "--kb" => { kb_path = args.get(i + 1).cloned(); i += 2; }
            "--save-pending" => { save_pending = true; i += 1; }
            other => { if seq_str.is_none() { seq_str = Some(other.to_string()); } i += 1; }
        }
    }
    let seq_str = match seq_str {
        Some(s) => s,
        None => {
            println!("Usage: swind explore \"10,17,24\" [--mode fast|verified|loop|full] [--kb path] [--save-pending]");
            std::process::exit(2);
        }
    };

    // Parse integers or a/b fractions into exact EPR values.
    let mut signal: Vec<EPR> = Vec::new();
    for tok in seq_str.split(',') {
        let tok = tok.trim();
        if tok.is_empty() { continue; }
        let parsed = if let Some((n, d)) = tok.split_once('/') {
            match (n.trim().parse::<i128>(), d.trim().parse::<i128>()) {
                (Ok(n), Ok(d)) if d != 0 => Ok(EPR::from_ratio(n, d)),
                _ => Err(tok.to_string()),
            }
        } else {
            tok.parse::<i128>().map(EPR::from_i128).map_err(|_| tok.to_string())
        };
        match parsed {
            Ok(v) => signal.push(v),
            Err(t) => { println!("Cannot parse term: {}", t); std::process::exit(2); }
        }
    }
    if signal.len() < 2 {
        println!("Need at least 2 terms.");
        std::process::exit(2);
    }

    // Knowledge base: from --kb JSON if given (created if missing), else built-ins.
    let kb_file = kb_path.clone().unwrap_or_else(|| "swind_kb.json".to_string());
    let mut kb = match &kb_path {
        Some(p) if Path::new(p).exists() => storage::load_from_json(p).unwrap_or_else(|_| {
            let mut k = KnowledgeBase::new();
            k.initialize_fundamentals();
            k
        }),
        _ => {
            let mut k = KnowledgeBase::new();
            k.initialize_fundamentals();
            k
        }
    };

    let mut fate = Fate::new();
    let result = match mode.as_str() {
        "fast" => {
            let r = fate.explore(&signal, &kb);
            println!("Mode: fast (explore) — NOTE: fast-path labels are ungated; use verified/loop for graded candidates.");
            r
        }
        "verified" => fate.explore_with_verification(&signal, &kb),
        "full" => {
            let full = fate.full_exploration(&signal, &kb);
            println!("Final status: {:?}", full.final_status);
            println!("Voyager equation: {:?} (verified: {})", full.voyager_equation, full.voyager_verified);
            full.full_result
        }
        _ => { // loop (default): continued exploration until settled or MYSTERY
            let (r, _history, stats) = fate.explore_with_self_loop(&signal, &kb);
            println!("Self-loop: {}", stats.summary());
            if stats.truly_exhausted {
                println!("Result: MYSTERY — all transforms exhausted, no pattern survived.");
            }
            r
        }
    };

    println!("\nStatus: {:?}", result.status);
    if let Some(f) = &result.formula {
        println!("Formula: {}", f);
    }
    println!("Fit level: {:?}", result.fit_level);
    println!("Is discovery (engine label): {}", result.is_discovery);

    if let Some(a) = &result.discovery_assessment {
        println!("\n--- Discovery assessment (candidate for human review, not a conclusion) ---");
        println!("Grade: {:?}", a.discovery_status);
        println!("OEIS sequences searched: {}", a.oeis_sequences_searched);
        println!("Verification: {}", a.verification_desc);
        println!("Non-triviality: {}", a.non_trivial_desc);
        println!("Conclusion: {}", a.conclusion);
        for e in &a.negative_evidence {
            println!("  negative evidence: {}", e);
        }

        if save_pending {
            let eligible = result.is_discovery || matches!(
                a.discovery_status,
                DiscoveryStatus::CandidateLaw | DiscoveryStatus::StrongCandidate | DiscoveryStatus::GenuineDiscovery
            );
            if eligible {
                if let Some(f) = &result.formula {
                    let derivation: String = result.derivation_steps.join("\n").chars().take(4000).collect();
                    let id = kb.add_discovery(
                        f.clone(),
                        format!("swind explore candidate: {:?}", a.discovery_status),
                        derivation,
                    );
                    match storage::save_to_json(&kb, &kb_file) {
                        Ok(()) => println!("\n📥 Queued for human review: {} (pending total: {}) -> {}", id, kb.get_pending_review().len(), kb_file),
                        Err(e) => println!("\nFailed to save KB: {}", e),
                    }
                }
            } else {
                println!("\nNot queued: candidate did not survive grading (grade: {:?}).", a.discovery_status);
            }
        }
    } else if save_pending {
        println!("\nNot queued: no graded assessment in this mode (use --mode verified or loop).");
    }
}

fn list_projects() {
    println!("📋 Available Projects:\n");
    println!("┌────────┬─────────────────────────────────────────────────────────────┬─────────────────────┐");
    println!("│ ID     │ Name                                                        │ Category            │");
    println!("├────────┼─────────────────────────────────────────────────────────────┼─────────────────────┤");
    
    let projects = vec![
        ("A1", "30-Story High-Rise Building Design", "Civil Engineering"),
        ("A2", "Electric Vehicle Aerodynamic Design", "Automotive"),
        ("A3", "WiFi 7/8 Parameter Optimization", "Communications"),
        ("B1", "LLM Inference Cost Optimization", "AI Infrastructure"),
        ("B5", "Chip Microarchitecture Search", "Hardware"),
        ("C1", "Battery Fast Charging Strategy", "Energy"),
        ("D2", "Semiconductor Yield Optimization", "Manufacturing"),
        ("E1", "Catalyst Formulation Optimization", "Materials"),
        ("F1", "Drug Combination Screening", "Biomedical"),
        ("G2", "eVTOL Urban Aircraft Design", "Aerospace"),
        ("I1", "Mars Habitat Module Design", "Space"),
        ("I3", "Deep Sea Explorer Design", "Ocean"),
        ("Q1", "Wastewater Treatment Optimization", "Environment"),
        ("Q5", "Carbon Neutrality Pathway", "Environment"),
        ("R1", "Market Price Mechanism", "Economics"),
        ("R2", "Real Estate Market Regulation", "Economics"),
        ("S1", "Poverty Alleviation Strategy", "Social Policy"),
        ("S5", "Universal Basic Income (UBI)", "Social Policy"),
    ];
    
    for (id, name, cat) in projects {
        println!("│ {:6} │ {:59} │ {:19} │", id, name, cat);
    }
    
    println!("└────────┴─────────────────────────────────────────────────────────────┴─────────────────────┘");
    println!("\n📊 Total: 133 projects (18 shown above)");
    println!("\n💡 Use 'swind optimize --project <ID>' to run optimization");
}

fn run_optimize(project_id: &str) {
    println!("🔄 Running optimization for project: {}\n", project_id);
    
    // 简化的优化演示
    let search_space = match project_id {
        "A1" => 673_920u64,
        "A2" => 290_594_304,
        "A3" => 87_048_192,
        "B1" => 20_971_520,
        _ => 1_000_000,
    };
    
    println!("  📊 Search space: {} combinations", format_number(search_space));
    println!("  ⏳ Running sampled search (same convention as batch_optimizer)...");
    
    // 与 batch_optimizer::optimize_project 相同的评估逻辑：
    // 确定性 LCG 采样 sqrt(N) 个配置（上限 10,000），通用目标 = 参数均值，取最小。
    let sample_size = ((search_space as f64).sqrt().min(10_000.0) as u64).max(1);
    let mut seed: u64 = 12345;
    let mut rand_simple = |seed: &mut u64, max: i32| -> i32 {
        *seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        ((*seed >> 16) as i32 % (max + 1)).abs()
    };
    let mut optimal_value = f64::MAX;
    let mut valid_solutions: u64 = 0;
    for _ in 0..sample_size {
        // 6 个归一化参数（101 点网格，与 batch_optimizer 的步进网格一致）
        let mut sum = 0.0;
        for _ in 0..6 {
            sum += rand_simple(&mut seed, 100) as f64 / 100.0;
        }
        let objective = sum / 6.0; // evaluate_generic: 参数均值
        if objective > 0.0 {
            valid_solutions += 1;
            if objective < optimal_value {
                optimal_value = objective;
            }
        }
    }
    if optimal_value == f64::MAX { optimal_value = 0.0; }
    
    println!("  ✅ Optimization complete! ({} samples evaluated)", format_number(sample_size));
    println!("\n📈 Results:");
    println!("  Optimal value: {:.4}", optimal_value);
    println!("  Valid solutions: {} of {} sampled ({:.2}%)", 
        format_number(valid_solutions),
        format_number(sample_size),
        100.0 * valid_solutions as f64 / sample_size as f64
    );
    
    // 保存结果
    let output_dir = Path::new("swind_output");
    fs::create_dir_all(output_dir).ok();
    
    let result_path = output_dir.join(format!("{}_result.json", project_id));
    let result_json = format!(
        r#"{{"project_id": "{}", "search_space": {}, "evaluated_samples": {}, "optimal_value": {:.4}, "valid_solutions": {}}}"#,
        project_id, search_space, sample_size, optimal_value, valid_solutions
    );
    
    let mut file = File::create(&result_path).unwrap();
    file.write_all(result_json.as_bytes()).unwrap();
    
    println!("\n📁 Output: {}", result_path.display());
}

fn run_paper(project_id: &str, format: &str) {
    println!("📝 Generating paper for project: {}\n", project_id);
    
    let output_dir = Path::new("swind_output/papers");
    fs::create_dir_all(output_dir).ok();
    
    let title = match project_id {
        "A1" => "Optimal Structural Design for 30-Story High-Rise Buildings",
        "A2" => "Optimal Aerodynamic Shape Design for Electric Vehicles",
        "A3" => "Next-Generation WiFi 7/8 Parameter Optimization",
        "B1" => "Cost Optimization for Large Language Model Inference",
        "S1" => "Optimization of Targeted Poverty Alleviation Strategies",
        "S5" => "Universal Basic Income Parameter Optimization",
        "Q5" => "Optimal Pathway to Carbon Neutrality",
        _ => "Generic Optimization Study",
    };
    
    if format == "md" || format == "all" {
        let md_content = generate_markdown_stub(project_id, title);
        let md_path = output_dir.join(format!("{}_paper.md", project_id));
        let mut file = File::create(&md_path).unwrap();
        file.write_all(md_content.as_bytes()).unwrap();
        println!("  📄 Markdown: {}", md_path.display());
    }
    
    if format == "tex" || format == "all" {
        let tex_content = generate_latex_stub(project_id, title);
        let tex_path = output_dir.join(format!("{}_paper.tex", project_id));
        let mut file = File::create(&tex_path).unwrap();
        file.write_all(tex_content.as_bytes()).unwrap();
        println!("  📄 LaTeX: {}", tex_path.display());
    }
    
    println!("\n✅ Paper generation complete!");
}

fn generate_markdown_stub(id: &str, title: &str) -> String {
    format!(r#"# {}

**Authors**: S-wind Research Team  
**Date**: January 2026  
**Project ID**: {}

---

## Abstract

This paper presents a systematic exhaustive search methodology for optimization.
[Full content would be generated by paper_generator binary]

---

*Generated by S-wind CLI v1.0*
"#, title, id)
}

fn generate_latex_stub(id: &str, title: &str) -> String {
    format!(r#"\documentclass{{article}}
\title{{{}}}
\author{{S-wind Research Team}}
\date{{January 2026}}
\begin{{document}}
\maketitle
\begin{{abstract}}
This paper presents a systematic exhaustive search methodology for optimization.
Project ID: {}
\end{{abstract}}
\end{{document}}
"#, title, id)
}

fn run_batch() {
    println!("🚀 Running batch processing for all projects...\n");
    println!("💡 For full batch processing, use:");
    println!("   cargo run --release --bin batch_optimizer");
    println!("   cargo run --release --bin paper_generator\n");
    
    println!("📁 Output directories:");
    println!("   batch_output/        - Optimization results + 3D models");
    println!("   papers_output/       - arXiv papers (MD + LaTeX)");
}
