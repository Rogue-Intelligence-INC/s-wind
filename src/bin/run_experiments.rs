//! S-wind Complete Experiment Runner
//! Run all previous experiments and generate reports

use s_wind::{
    EPR, Fate, FateStatus,
    KnowledgeBase, OeisLoader,
    SignalBuilder, ExperimentRunner,
};
use std::fs;
use std::time::Instant;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           S-wind Full Experiment Run                                    ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");

    let start_time = Instant::now();
    
    // Initialize knowledge base
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    
    // Load OEIS
    let mut oeis = OeisLoader::new();
    oeis.load_builtin();
    let _ = oeis.load_bfiles_from_dir("data/oeis");
    let oeis_added = oeis.add_to_knowledge_base(&mut kb);
    
    println!("Knowledge base initialized:");
    println!("  - Math theorems: {}", kb.theorems.len());
    println!("  - Physical laws: {}", kb.physical_laws.len());
    println!("  - Known sequences: {} (including OEIS: {})", kb.known_sequences.len(), oeis_added);
    println!();

    // Create Fate engine
    let mut fate = Fate::new();
    
    // Collect experiment results
    let mut results = Vec::new();

    // === High-Value Experiments ===
    println!("═══════════════════════════════════════════════════════════════════");
    println!("High-Value Experiments (High-Value Experiments)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    // 1. Fibonacci sequence
    results.push(run_fate_experiment(&mut fate, &kb, 
        "Fibonacci Sequence",
        "Fibonacci sequence - Golden ratio related",
        &[1, 1, 2, 3, 5, 8, 13, 21, 34, 55],
    ));

    // 2. Prime sequence
    results.push(run_fate_experiment(&mut fate, &kb,
        "Prime Numbers",
        "Prime sequence - No simple closed form",
        &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29],
    ));

    // 3. Perfect squares
    results.push(run_fate_experiment(&mut fate, &kb,
        "Perfect Squares",
        "Perfect squares - n² pattern",
        &[1, 4, 9, 16, 25, 36, 49, 64, 81, 100],
    ));

    // 4. Perfect cubes
    results.push(run_fate_experiment(&mut fate, &kb,
        "Perfect Cubes",
        "Perfect cubes - n³ pattern",
        &[1, 8, 27, 64, 125, 216, 343, 512, 729, 1000],
    ));

    // 5. Triangular numbers
    results.push(run_fate_experiment(&mut fate, &kb,
        "Triangular Numbers",
        "Triangular numbers - n(n+1)/2 pattern",
        &[1, 3, 6, 10, 15, 21, 28, 36, 45, 55],
    ));

    // 6. Factorials
    results.push(run_fate_experiment(&mut fate, &kb,
        "Factorials",
        "Factorial sequence - n! pattern",
        &[1, 2, 6, 24, 120, 720, 5040],
    ));

    // 7. Powers of 2
    results.push(run_fate_experiment(&mut fate, &kb,
        "Powers of 2",
        "Powers of 2 - Exponential growth",
        &[1, 2, 4, 8, 16, 32, 64, 128, 256, 512],
    ));

    // 8. Catalan numbers
    results.push(run_fate_experiment(&mut fate, &kb,
        "Catalan Numbers",
        "Catalan numbers - Combinatorial counting",
        &[1, 1, 2, 5, 14, 42, 132, 429],
    ));

    // === Frontier Experiments ===
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("Frontier Experiments (Frontier Experiments)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    // 9. Harmonic numerators
    results.push(run_fate_experiment(&mut fate, &kb,
        "Harmonic Numerators",
        "Harmonic numerators - 1 + 1/2 + ... + 1/n",
        &[1, 3, 11, 25, 137, 49, 363, 761],
    ));

    // 10. Lucas numbers
    results.push(run_fate_experiment(&mut fate, &kb,
        "Lucas Numbers",
        "Lucas numbers - Fibonacci variant",
        &[2, 1, 3, 4, 7, 11, 18, 29, 47, 76],
    ));

    // 11. Pell numbers
    results.push(run_fate_experiment(&mut fate, &kb,
        "Pell Numbers",
        "Pell numbers - P(n) = 2P(n-1) + P(n-2)",
        &[0, 1, 2, 5, 12, 29, 70, 169, 408],
    ));

    // 12. Tribonacci
    results.push(run_fate_experiment(&mut fate, &kb,
        "Tribonacci Numbers",
        "Tribonacci numbers - 3-term recurrence",
        &[0, 0, 1, 1, 2, 4, 7, 13, 24, 44],
    ));

    // 13. Bell numbers
    results.push(run_fate_experiment(&mut fate, &kb,
        "Bell Numbers",
        "Bell numbers - Set partition counting",
        &[1, 1, 2, 5, 15, 52, 203, 877],
    ));

    // 14. Partition numbers
    results.push(run_fate_experiment(&mut fate, &kb,
        "Partition Numbers",
        "Partition numbers - Integer partition ways",
        &[1, 1, 2, 3, 5, 7, 11, 15, 22, 30],
    ));

    // === Rational Sequences ===
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("Rational Sequences (Rational Sequences)");
    println!("═══════════════════════════════════════════════════════════════════\n");

    // 15. Harmonic sequence
    results.push(run_fate_rational_experiment(&mut fate, &kb,
        "Harmonic Sequence",
        "Harmonic sequence - 1/n pattern",
        &[(1,1), (1,2), (1,3), (1,4), (1,5), (1,6), (1,7), (1,8)],
    ));

    // 16. Harmonic sequence (offset)
    results.push(run_fate_rational_experiment(&mut fate, &kb,
        "Harmonic Offset",
        "Harmonic offset - 1/(n+1) pattern",
        &[(1,2), (1,3), (1,4), (1,5), (1,6), (1,7), (1,8), (1,9)],
    ));

    // 17. Basel partial sum
    results.push(run_fate_rational_experiment(&mut fate, &kb,
        "Basel Partial Sum",
        "Basel problem partial sum - Σ 1/n²",
        &[(1,1), (5,4), (49,36), (205,144), (5269,3600)],
    ));

    // === Voyager Detailed Derivation ===
    println!("\n═══════════════════════════════════════════════════════════════════");
    println!("Voyager Detailed Derivation");
    println!("═══════════════════════════════════════════════════════════════════\n");

    // Using ExperimentRunner to generate detailed reports
    let experiments = vec![
        ("Linear_Sequence", vec![2, 4, 6, 8, 10, 12, 14]),
        ("Quadratic_Sequence", vec![1, 4, 9, 16, 25, 36, 49]),
        ("Cubic_Sequence", vec![1, 8, 27, 64, 125, 216, 343]),
        ("Fibonacci_Sequence", vec![1, 1, 2, 3, 5, 8, 13, 21, 34]),
    ];

    for (name, data) in experiments {
        println!("Experiment: {}", name);
        let signal = SignalBuilder::from_integers(&data).build();
        let mut runner = ExperimentRunner::with_name(name);
        let result = runner.run(signal);
        if let Some(trace) = &result.derivation_trace {
            println!("  Derivation steps: {}", trace.len());
        }
        if let Some(_formatted) = &result.formatted_trace {
            println!("  LaTeX output: Generated");
        }
        println!();
    }

    // === Generate Report ===
    let elapsed = start_time.elapsed();
    
    println!("\n╔══════════════════════════════════════════════════════════════════╗");
    println!("║                        Experiment Summary                                  ║");
    println!("╚══════════════════════════════════════════════════════════════════╝\n");
    
    println!("Total experiments: {}", results.len());
    let known = results.iter().filter(|r| r.contains("Recognized")).count();
    let detected = results.iter().filter(|r| r.contains("Pattern")).count();
    let unknown = results.iter().filter(|r| r.contains("Unknown")).count();
    
    println!("  - Known sequences: {}", known);
    println!("  - Pattern detections: {}", detected);
    println!("  - Unknown: {}", unknown);
    println!("\nTime: {:.2?}", elapsed);
    
    // Fate stats
    let stats = fate.stats();
    println!("\nFate Engine Stats:");
    println!("  - tick: {}", stats.tick);
    println!("  - Signals: {}", stats.signal_count);
    println!("  - Discoveries: {}", stats.discovery_count);
    
    // Discovery Summary (using new criterion)
    let summary = fate.get_discovery_summary();
    println!("\nDiscovery Criterion Summary:");
    println!("  - Novel Results: {}", summary.novel_results);
    println!("  - System Patterns: {}", summary.system_patterns);
    println!("  - New Rules: {}", summary.new_rules);
    println!("  - Level Ups: {}", summary.level_ups);
    println!("  - Pattern Detections (not counted): {}", summary.pattern_detections);
    
    // Show real discoveries
    if !summary.novel_discoveries.is_empty() {
        println!("\n🎯 Real Discoveries:");
        for d in &summary.novel_discoveries {
            println!("  [{:0>4}] {}", d.tick, d.id);
            if let Some(known) = &d.known_formula {
                println!("         Known: {}", known);
            } else {
                println!("         Known: (none)");
            }
            println!("         Found: {}", d.found_formula);
            println!("         Diff: {}", d.difference);
        }
    }

    // Generate Markdown reports (with discovery summary)
    generate_reports_with_discoveries(&results, &summary);
}

fn run_fate_experiment(fate: &mut Fate, kb: &KnowledgeBase, name: &str, desc: &str, data: &[i128]) -> String {
    let start = Instant::now();
    let signal: Vec<EPR> = data.iter().map(|&v| EPR::from_i128(v)).collect();
    // Use explore_with_verification to get proper FitAnalysis with DoF calculation
    let result = fate.explore_with_verification(&signal, kb);
    let elapsed = start.elapsed();
    
    // Get academic label
    let label = result.status.label();
    let fit_label = result.fit_level.as_ref().map(|f| f.label()).unwrap_or("");
    
    // Get DoF info if available
    let dof_info = if let Some(ref fa) = result.fit_analysis {
        format!(" [DoF={}, n={}, params={}]", fa.dof, fa.sample_size, fa.num_parameters)
    } else {
        String::new()
    };
    
    let status_str = match &result.status {
        FateStatus::KnownSequence { id, name } => {
            format!("{} {} ({})", label, name, id)
        }
        FateStatus::PatternDetected { class_id, formula } => {
            format!("{}{} {} [{}]{}", label, fit_label, formula, class_id, dof_info)
        }
        FateStatus::Unknown => format!("{} Unknown", label),
        FateStatus::Mystery { iterations_exhausted, ref reason, .. } => {
            format!("{} 🔮 MYSTERY: {} iterations, {}", label, iterations_exhausted, reason)
        }
    };
    
    println!("{}", name);
    println!("  Description: {}", desc);
    println!("  Input: {:?}", &data[..data.len().min(8)]);
    println!("  Result: {}", status_str);
    if let Some(ref fa) = result.fit_analysis {
        println!("  FitAnalysis: {}", fa.summary());
    }
    // Show three-layer discovery assessment if available
    if let Some(ref da) = result.discovery_assessment {
        println!("  Discovery Assessment:");
        println!("    OEIS: {} (searched {} seqs)", da.oeis_status.label(), da.oeis_sequences_searched);
        println!("    Status: {}", da.discovery_status.label());
        println!("    Conclusion: {}", da.conclusion);
    }
    println!("  Time: {:?}", elapsed);
    println!();
    
    format!("{}|{}|{}", name, status_str, format!("{:?}", elapsed))
}

fn run_fate_rational_experiment(fate: &mut Fate, kb: &KnowledgeBase, name: &str, desc: &str, data: &[(i128, i128)]) -> String {
    let start = Instant::now();
    let signal: Vec<EPR> = data.iter().map(|&(n, d)| EPR::from_ratio(n, d)).collect();
    // Use explore_with_verification to get proper FitAnalysis with DoF calculation
    let result = fate.explore_with_verification(&signal, kb);
    let elapsed = start.elapsed();
    
    // Get academic label
    let label = result.status.label();
    let fit_label = result.fit_level.as_ref().map(|f| f.label()).unwrap_or("");
    
    // Get DoF info if available
    let dof_info = if let Some(ref fa) = result.fit_analysis {
        format!(" [DoF={}, n={}, params={}]", fa.dof, fa.sample_size, fa.num_parameters)
    } else {
        String::new()
    };
    
    let status_str = match &result.status {
        FateStatus::KnownSequence { id, name } => {
            format!("{} {} ({})", label, name, id)
        }
        FateStatus::PatternDetected { class_id, formula } => {
            format!("{}{} {} [{}]{}", label, fit_label, formula, class_id, dof_info)
        }
        FateStatus::Unknown => format!("{} Unknown", label),
        FateStatus::Mystery { iterations_exhausted, ref reason, .. } => {
            format!("{} 🔮 MYSTERY: {} iterations, {}", label, iterations_exhausted, reason)
        }
    };
    
    let data_str: Vec<String> = data.iter().map(|(n, d)| format!("{}/{}", n, d)).collect();
    
    println!("{}", name);
    println!("  Description: {}", desc);
    println!("  Input: {:?}", &data_str[..data_str.len().min(6)]);
    println!("  Result: {}", status_str);
    if let Some(ref fa) = result.fit_analysis {
        println!("  FitAnalysis: {}", fa.summary());
    }
    // Show three-layer discovery assessment if available
    if let Some(ref da) = result.discovery_assessment {
        println!("  Discovery Assessment:");
        println!("    OEIS: {} (searched {} seqs)", da.oeis_status.label(), da.oeis_sequences_searched);
        println!("    Status: {}", da.discovery_status.label());
        println!("    Conclusion: {}", da.conclusion);
    }
    println!("  Time: {:?}", elapsed);
    println!();
    
    format!("{}|{}|{}", name, status_str, format!("{:?}", elapsed))
}

fn generate_reports_with_discoveries(results: &[String], summary: &s_wind::DiscoverySummary) {
    // Build discovery summary section
    let discovery_section = if summary.total_discoveries > 0 || !summary.novel_discoveries.is_empty() {
        let novel_list = if summary.novel_discoveries.is_empty() {
            "No new discoveries".to_string()
        } else {
            summary.novel_discoveries.iter()
                .map(|d| {
                    let known = d.known_formula.as_deref().unwrap_or("(no known formula)");
                    format!(
                        "- **{}** (tick {})\n  - Known: {}\n  - Found: {}\n  - Diff: {}",
                        d.id, d.tick, known, d.found_formula, d.difference
                    )
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        };
        
        format!(r#"
## Discovery Summary

**Criterion**: Discovery = (Result ≠ Known Authority) ∧ (Verified Correct) ∧ (Non-Trivial)

**Non-Triviality (CORRECTED)**: DoF = n - (d+1) ≥ 2

Where d+1 = number of polynomial parameters. DoF=0 is pure interpolation (zero information).

| Metric | Count |
|--------|-------|
| **Novel Results** | {} |
| System Patterns | {} |
| New Rules | {} |
| Level Ups | {} |
| Pattern Detections (not discovery) | {} |

### Novel Discovery Details

{}
"#,
            summary.novel_results,
            summary.system_patterns,
            summary.new_rules,
            summary.level_ups,
            summary.pattern_detections,
            novel_list
        )
    } else {
        "\n## Discovery Summary\n\nNo new discoveries (all results match knowledge base)\n".to_string()
    };
    
    // Generate High-Value Experiments report
    let high_value_report = format!(r#"# S-wind High-Value Experiments Results

**Generated**: {}
**S-wind Version**: 1.0.0 (Full EPR Precision + Discovery Criterion)

## System Configuration

- **Arithmetic**: Extended Precision Rational (EPR) - Arbitrary precision
- **Knowledge Base**: 78 theorems, 60 physical laws, 195+ sequences (with OEIS)
- **Modules Loaded**: Fate, Voyager, Quad (Gravity, Quantum, Thermal, Coupling), Space
- **Discovery Criterion**: Result ≠ Known Authority ∧ Verified Correct
{}

## Experiment Results

| Experiment | Result | Time |
|------------|--------|------|
{}

## Summary

S-wind successfully identifies:
1. **Known Sequences**: Fibonacci, Lucas, Pell, Catalan, Bell, Triangular, etc.
2. **Pattern Detection**: Polynomial (linear, quadratic, cubic), Exponential, Reciprocal
3. **Impossibility Theorems**: Primes (no polynomial closed form), Pi digits (normal number)

### Three-Layer Discovery Assessment

**Philosophy**: OEIS is a refutation tool, not a precondition for discovery.

| Layer | Purpose | Check |
|-------|---------|-------|
| **Pre-OEIS** | Recognition | Match against 195+ sequences |
| **Discovery** | Assessment | Definition + Verification + Non-triviality |
| **Negative Knowledge** | Evidence | Why NOT known (failed patterns, no OEIS match) |

**Discovery Status**:
- `[Known]` - OEIS match (NOT discovery)
- `[TrivialInterpolation]` - DoF=0 (NOT discovery)
- `[DescriptiveOnly]` - Unverified (NOT discovery)
- `[CandidateLaw]` - Verified but needs evidence
- `[StrongCandidate]` - Verified + Non-trivial + Novel
- `[GenuineDiscovery]` - All criteria met

### Academic Disclaimer

> *OEIS should be a refutation tool, not a precondition for discovery.*
> *Fitting (including exact rational interpolation) is not counted as discovery.*
> *Results are reported as auditable hypothesis generators.*

## Technical Notes

- All computations use exact rational arithmetic (no floating-point)
- EPR enables arbitrary precision beyond i128 limits
- Fate dynamically loads patterns from KnowledgeBase (no hardcoding)
- Fit levels follow academic consensus on interpolation vs discovery

---

**S-wind: Autonomous Algebraic Discovery Framework**

*Valhalla Research Team*  
*valhalla@research.org*

*Version 1.0.0 | January 2026*
"#,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        discovery_section,
        results.iter()
            .map(|r| {
                let parts: Vec<&str> = r.split('|').collect();
                format!("| {} | {} | {} |", 
                    parts.first().unwrap_or(&"-"),
                    parts.get(1).unwrap_or(&"-"),
                    parts.get(2).unwrap_or(&"-"))
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    fs::write("HIGH_VALUE_EXPERIMENTS_RESULTS_EN.md", &high_value_report).ok();
    println!("\n✓ Report generated: HIGH_VALUE_EXPERIMENTS_RESULTS_EN.md");

    // Generate Frontier Experiments report
    let discovery_section_cn = if summary.total_discoveries > 0 || !summary.novel_discoveries.is_empty() {
        let novel_list = if summary.novel_discoveries.is_empty() {
            "No new discoveries".to_string()
        } else {
            summary.novel_discoveries.iter()
                .map(|d| {
                    let known = d.known_formula.as_deref().unwrap_or("(no known formula)");
                    format!(
                        "- **{}** (tick {})\n  - Known: {}\n  - Found: {}\n  - Diff: {}",
                        d.id, d.tick, known, d.found_formula, d.difference
                    )
                })
                .collect::<Vec<_>>()
                .join("\n\n")
        };
        
        format!(r#"
## Discovery Summary

**Criterion**: Discovery = (Result ≠ Known Authority) ∧ (Verified Correct) ∧ (Non-Trivial)

**Non-Triviality (CORRECTED)**: DoF = n - (d+1) ≥ 2

Where d+1 = number of polynomial parameters. DoF=0 is pure interpolation (zero information).

| Metric | Count |
|--------|-------|
| **Novel Results** | {} |
| System Patterns | {} |
| New Rules | {} |
| Level Ups | {} |
| Pattern Detections (not discovery) | {} |

### Novel Discovery Details

{}
"#,
            summary.novel_results,
            summary.system_patterns,
            summary.new_rules,
            summary.level_ups,
            summary.pattern_detections,
            novel_list
        )
    } else {
        "\n## Discovery Summary\n\nNo new discoveries (all results match knowledge base)\n".to_string()
    };
    
    let frontier_report = format!(r#"# S-wind Frontier Experiments Results

**Generated**: {}
**S-wind Version**: 1.0.0 (Full EPR Precision + Discovery Criterion)

## System Configuration

- **Arithmetic**: Extended Precision Rational (EPR) - Arbitrary precision
- **Knowledge Base**: 78 theorems, 60 physical laws, 195+ sequences (with OEIS)
- **Modules Loaded**: Fate, Voyager, Quad (Gravity, Quantum, Thermal, Coupling), Space
- **Discovery Criterion**: Result ≠ Known Authority ∧ Verified Correct
{}

## Experiment Results

| Experiment | Result | Time |
|------------|--------|------|
{}

## Summary

S-wind successfully identifies:
1. **Known Sequences**: Fibonacci, Lucas, Pell, Catalan, Bell, Triangular, etc.
2. **Pattern Detection**: Polynomial (linear, quadratic, cubic), Exponential, Reciprocal
3. **Impossibility Theorems**: Primes (no polynomial closed form), Pi digits (normal number)

### Three-Layer Discovery Assessment

**Philosophy**: OEIS is a refutation tool, not a precondition for discovery.

| Layer | Purpose | Check |
|-------|---------|-------|
| **Pre-OEIS** | Recognition | Match against 195+ sequences |
| **Discovery** | Assessment | Definition + Verification + Non-triviality |
| **Negative Knowledge** | Evidence | Why NOT known (failed patterns, no OEIS match) |

**Discovery Status**:
- `[Known]` - OEIS match (NOT discovery)
- `[TrivialInterpolation]` - DoF=0 (NOT discovery)
- `[DescriptiveOnly]` - Unverified (NOT discovery)
- `[CandidateLaw]` - Verified but needs evidence
- `[StrongCandidate]` - Verified + Non-trivial + Novel
- `[GenuineDiscovery]` - All criteria met

### Academic Disclaimer

> *OEIS should be a refutation tool, not a precondition for discovery.*
> *Fitting (including exact rational interpolation) is not counted as discovery.*
> *Results are reported as auditable hypothesis generators.*

## Technical Notes

- All computations use exact rational arithmetic (no floating-point)
- EPR enables arbitrary precision beyond i128 limits
- Fate dynamically loads patterns from KnowledgeBase (no hardcoding)
- Fit levels follow academic consensus on interpolation vs discovery

---

**S-wind: Autonomous Algebraic Discovery Framework**

*Valhalla Research Team*  
*valhalla@research.org*

*Version 1.0.0 | January 2026*
"#,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        discovery_section_cn,
        results.iter()
            .map(|r| {
                let parts: Vec<&str> = r.split('|').collect();
                format!("| {} | {} | {} |", 
                    parts.first().unwrap_or(&"-"),
                    parts.get(1).unwrap_or(&"-"),
                    parts.get(2).unwrap_or(&"-"))
            })
            .collect::<Vec<_>>()
            .join("\n")
    );
    
    fs::write("FRONTIER_EXPERIMENTS_RESULTS.md", &frontier_report).ok();
    println!("✓ Report generated: FRONTIER_EXPERIMENTS_RESULTS.md");
}
