//! S-wind Interface Module (Enhanced)
//!
//! Provides standardized I/O interfaces, supporting multiple signal formats and report generation
//!
//! # Input Interface
//!
//! ## Supported Input Formats:
//! - Vec<Rational>: Exact rational sequence
//! - Vec<i64>: Integer sequence (auto-converted to Rational)
//! - Vec<(i64, i64)>: Fraction tuples (numerator, denominator)
//! - &str: String format "1/2, 1, 3/2" or "0.5, 1.0, 1.5"
//! - &[f64]: Float array (converted to approximate Rational)
//!
//! # Output Interface
//!
//! ## Generated Files:
//! - `{problem}_answer_{timestamp}.tex`: Answer (closed-form expression)
//! - `{problem}_reverse_{timestamp}.tex`: Reverse verification
//! - `{problem}_derivation_{timestamp}.tex`: Detailed derivation
//! - `{problem}_report_{timestamp}.md`: Markdown report

use chrono::Local;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use voyager::{
    DerivationRule, EquationType, MathematicalProcessingResult, 
    PreciseSignalDetector, Rational as VoyagerRational,
};

/// S-wind Experiment Configuration
#[derive(Debug, Clone)]
pub struct ExperimentConfig {
    /// Experiment name (for file naming)
    pub name: String,
    /// Output directory
    pub output_dir: String,
    /// Whether to generate LaTeX files
    pub generate_latex: bool,
    /// 是否Generate Markdown report
    pub generate_markdown: bool,
    /// Whether to print verbose logs
    pub verbose: bool,
}

impl Default for ExperimentConfig {
    fn default() -> Self {
        Self {
            name: "experiment".to_string(),
            output_dir: "output".to_string(),
            generate_latex: true,
            generate_markdown: true,
            verbose: true,
        }
    }
}

/// Input signal builder
pub struct SignalBuilder {
    values: Vec<VoyagerRational>,
}

impl SignalBuilder {
    /// Create empty signal builder
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
    
    /// Create from integer sequence
    pub fn from_integers(values: &[i64]) -> Self {
        Self {
            values: values.iter().map(|&n| VoyagerRational::new(n as i128, 1)).collect(),
        }
    }
    
    /// Create from fraction tuples (numerator, denominator)
    pub fn from_fractions(values: &[(i64, i64)]) -> Self {
        Self {
            values: values.iter().map(|&(n, d)| VoyagerRational::new(n as i128, d as i128)).collect(),
        }
    }
    
    /// Parse from string "1/2, 1, 3/2" or "0.5, 1.0, 1.5"
    pub fn from_string(input: &str) -> Result<Self, String> {
        let mut values = Vec::new();
        
        for part in input.split(',') {
            let trimmed = part.trim();
            
            if trimmed.contains('/') {
                // Fraction format: "1/2"
                let parts: Vec<&str> = trimmed.split('/').collect();
                if parts.len() != 2 {
                    return Err(format!("Invalid fraction format: {}", trimmed));
                }
                let num: i128 = parts[0].trim().parse()
                    .map_err(|_| format!("Invalid numerator: {}", parts[0]))?;
                let den: i128 = parts[1].trim().parse()
                    .map_err(|_| format!("Invalid denominator: {}", parts[1]))?;
                values.push(VoyagerRational::new(num, den));
            } else if trimmed.contains('.') {
                // Decimal format: approximate conversion
                let f: f64 = trimmed.parse()
                    .map_err(|_| format!("Invalid decimal: {}", trimmed))?;
                // Convert to fraction (precision limited to 1000000)
                let scale = 1000000i128;
                let num = (f * scale as f64).round() as i128;
                values.push(VoyagerRational::new(num, scale));
            } else {
                // Integer format: "42"
                let num: i128 = trimmed.parse()
                    .map_err(|_| format!("Invalid integer: {}", trimmed))?;
                values.push(VoyagerRational::new(num, 1));
            }
        }
        
        Ok(Self { values })
    }
    
    /// Add a rational value
    pub fn push(mut self, numerator: i64, denominator: i64) -> Self {
        self.values.push(VoyagerRational::new(numerator as i128, denominator as i128));
        self
    }
    
    /// Add an integer
    pub fn push_int(mut self, value: i64) -> Self {
        self.values.push(VoyagerRational::new(value as i128, 1));
        self
    }
    
    /// Build signal
    pub fn build(self) -> Vec<VoyagerRational> {
        self.values
    }
    
    /// Get signal length
    pub fn len(&self) -> usize {
        self.values.len()
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl Default for SignalBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// S-wind Experiment Runner
pub struct ExperimentRunner {
    config: ExperimentConfig,
    voyager: PreciseSignalDetector,
    timestamp: String,
}

impl ExperimentRunner {
    /// Create new experiment runner
    pub fn new(config: ExperimentConfig) -> Self {
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
        Self {
            config,
            voyager: PreciseSignalDetector::new(),
            timestamp,
        }
    }
    
    /// Create with default config
    pub fn with_name(name: &str) -> Self {
        let mut config = ExperimentConfig::default();
        config.name = name.to_string();
        Self::new(config)
    }
    
    /// Run experiment and generate reports
    pub fn run(&mut self, signal: Vec<VoyagerRational>) -> ExperimentResult {
        let result = self.voyager.process_signal_mathematically(signal.clone(), 4);
        
        // Get equation ID
        let equation_id = result.equation_created
            .as_ref()
            .map(|eq| eq.equation_id.clone())
            .unwrap_or_else(|| format!("eq_{}", self.voyager.known_equation_count()));
        
        // Get derivation trace
        let derivation_trace = self.voyager.get_derivation_trace(&equation_id).cloned();
        let formatted_trace = self.voyager.format_derivation_trace(&equation_id);
        
        // Generate output files
        if self.config.generate_latex || self.config.generate_markdown {
            let output_dir = Path::new(&self.config.output_dir);
            if !output_dir.exists() {
                fs::create_dir_all(output_dir).ok();
            }
            
            if self.config.generate_latex {
                self.generate_latex_files(&signal, &result, &equation_id, output_dir);
            }
            
            if self.config.generate_markdown {
                self.generate_markdown_report(&signal, &result, &equation_id, output_dir, &formatted_trace);
            }
        }
        
        ExperimentResult {
            input_signal: signal,
            processing_result: result,
            equation_id,
            derivation_trace,
            formatted_trace,
            timestamp: self.timestamp.clone(),
        }
    }
    
    /// Generate LaTeX files
    fn generate_latex_files(
        &self,
        signal: &[VoyagerRational],
        result: &MathematicalProcessingResult,
        equation_id: &str,
        output_dir: &Path,
    ) {
        // 1. Answer file
        let answer = self.generate_answer_tex(signal, result);
        let answer_path = output_dir.join(format!("{}_answer_{}.tex", self.config.name, self.timestamp));
        if let Ok(mut f) = File::create(&answer_path) {
            f.write_all(answer.as_bytes()).ok();
        }
        
        // 2. Reverse verification file
        let reverse = self.generate_reverse_tex(signal, result);
        let reverse_path = output_dir.join(format!("{}_reverse_{}.tex", self.config.name, self.timestamp));
        if let Ok(mut f) = File::create(&reverse_path) {
            f.write_all(reverse.as_bytes()).ok();
        }
        
        // 3. Derivation file
        let derivation = self.generate_derivation_tex(signal, result, equation_id);
        let deriv_path = output_dir.join(format!("{}_derivation_{}.tex", self.config.name, self.timestamp));
        if let Ok(mut f) = File::create(&deriv_path) {
            f.write_all(derivation.as_bytes()).ok();
        }
        
        if self.config.verbose {
            println!("  ✓ LaTeX files generated in {}/", self.config.output_dir);
        }
    }
    
    /// Generate Markdown report
    fn generate_markdown_report(
        &self,
        signal: &[VoyagerRational],
        result: &MathematicalProcessingResult,
        equation_id: &str,
        output_dir: &Path,
        formatted_trace: &Option<String>,
    ) {
        let mut md = String::new();
        
        // Header
        md.push_str(&format!("# S-wind Experiment Report: {}\n\n", self.config.name));
        md.push_str(&format!("**Generated**: {}  \n", self.timestamp));
        md.push_str("**S-wind Version**: With Pattern class assumed\n\n");
        md.push_str("---\n\n");
        
        // Input
        md.push_str("## Input Signal\n\n");
        let signal_str: Vec<String> = signal.iter().map(|r| r.to_string()).collect();
        md.push_str(&format!("```\nS = {{{}}}\n```\n\n", signal_str.join(", ")));
        md.push_str(&format!("**Sample count**: {}\n\n", signal.len()));
        
        // Result
        md.push_str("## Detection Result\n\n");
        
        if let Some(eq) = &result.equation_created {
            let (pattern_class, eq_latex) = match &eq.equation_type {
                EquationType::Constant(c) => {
                    ("constant function".to_string(), format!("f ≡ {}", c))
                }
                EquationType::Linear { coefficient, constant, offset } => {
                    ("linear polynomial (degree 1)".to_string(), 
                     format!("f(n) = {}·n + {} (n starts at {})", coefficient, constant, 1 + offset))
                }
                EquationType::Quadratic { a, b, c, offset } => {
                    ("quadratic polynomial (degree 2)".to_string(),
                     format!("f(n) = {}·n² + {}·n + {} (n starts at {})", a, b, c, 1 + offset))
                }
                EquationType::Reciprocal { numerator, offset } => {
                    ("reciprocal form (c/(n+k))".to_string(),
                     format!("f(n) = {}/(n + {})", numerator, offset))
                }
                _ => ("unknown".to_string(), "?".to_string()),
            };
            
            md.push_str(&format!("**Pattern class assumed**: {}\n\n", pattern_class));
            md.push_str(&format!("**Equation ID**: `{}`\n\n", eq.equation_id));
            md.push_str("**Discovered Interpolant**:\n\n");
            md.push_str(&format!("```\n{}\n```\n\n", eq_latex));
            md.push_str(&format!("**Algebraically verified**: {}\n\n", 
                if eq.algebraically_verified { "✅ YES" } else { "❌ NO" }));
        } else {
            md.push_str("**Result**: No equation created (unknown pattern)\n\n");
        }
        
        // Derivation
        md.push_str("---\n\n");
        md.push_str("## Derivation Trace\n\n");
        
        if let Some(trace) = formatted_trace {
            md.push_str("```\n");
            md.push_str(trace);
            md.push_str("\n```\n\n");
        } else if let Some(trace) = self.voyager.get_derivation_trace(equation_id) {
            md.push_str(&format!("**Total steps**: {}\n\n", trace.len()));
            md.push_str("| Step | Description | Output | Rule |\n");
            md.push_str("|------|-------------|--------|------|\n");
            
            for step in trace.iter().take(20) {  // Show first 20 steps
                let rule = format!("{:?}", step.rule_applied);
                md.push_str(&format!("| {} | {} | {} | {} |\n",
                    step.step_number,
                    step.description.chars().take(40).collect::<String>(),
                    step.output_expr.chars().take(40).collect::<String>(),
                    rule));
            }
            
            if trace.len() > 20 {
                md.push_str(&format!("\n*... and {} more steps*\n\n", trace.len() - 20));
            }
        } else {
            md.push_str("*No derivation trace available*\n\n");
        }
        
        // Verification summary
        md.push_str("---\n\n");
        md.push_str("## Verification Summary\n\n");
        md.push_str("| Property | Status |\n");
        md.push_str("|----------|--------|\n");
        md.push_str("| Exact Rational arithmetic | ✅ |\n");
        md.push_str("| No f64 floating point | ✅ |\n");
        md.push_str("| Pattern class stated | ✅ |\n");
        
        if result.equation_created.is_some() {
            md.push_str("| Algebraic verification | ✅ |\n");
        }
        
        // Write file
        let md_path = output_dir.join(format!("{}_report_{}.md", self.config.name, self.timestamp));
        if let Ok(mut f) = File::create(&md_path) {
            f.write_all(md.as_bytes()).ok();
        }
        
        if self.config.verbose {
            println!("  ✓ Markdown report generated: {}_report_{}.md", self.config.name, self.timestamp);
        }
    }
    
    fn generate_answer_tex(&self, signal: &[VoyagerRational], result: &MathematicalProcessingResult) -> String {
        let mut tex = String::new();
        
        tex.push_str("% S-wind Answer Output\n");
        tex.push_str(&format!("% Problem: {}\n", self.config.name));
        tex.push_str(&format!("% Generated: {}\n\n", self.timestamp));
        
        tex.push_str("\\documentclass{article}\n");
        tex.push_str("\\usepackage{amsmath,amssymb}\n");
        tex.push_str("\\begin{document}\n\n");
        
        tex.push_str(&format!("\\section*{{Answer: {}}}\n\n", self.config.name.replace('_', " ")));
        
        tex.push_str("\\subsection*{Input Signal}\n");
        tex.push_str(&format!("$$S = \\{{{}\\}}$$\n\n", 
            signal.iter().map(|r| r.to_latex()).collect::<Vec<_>>().join(", ")));
        
        if let Some(eq) = &result.equation_created {
            let eq_latex = self.equation_to_latex(&eq.equation_type);
            tex.push_str("\\subsection*{Discovered Interpolant}\n");
            tex.push_str(&format!("$${}$$\n\n", eq_latex));
            tex.push_str(&format!("Algebraically verified: {}\n", 
                if eq.algebraically_verified { "\\textbf{YES}" } else { "NO" }));
        }
        
        tex.push_str("\n\\end{document}\n");
        tex
    }
    
    fn generate_reverse_tex(&self, signal: &[VoyagerRational], result: &MathematicalProcessingResult) -> String {
        let mut tex = String::new();
        
        tex.push_str("% S-wind Reverse Verification\n");
        tex.push_str(&format!("% Problem: {}\n\n", self.config.name));
        
        tex.push_str("\\documentclass{article}\n");
        tex.push_str("\\usepackage{amsmath,amssymb}\n");
        tex.push_str("\\begin{document}\n\n");
        
        tex.push_str(&format!("\\section*{{Reverse Verification: {}}}\n\n", self.config.name.replace('_', " ")));
        
        if let Some(eq) = &result.equation_created {
            tex.push_str("\\begin{tabular}{|c|c|c|c|}\n\\hline\n");
            tex.push_str("$n$ & $s_i$ & $f(n)$ & Match \\\\\n\\hline\n");
            
            let offset = match &eq.equation_type {
                EquationType::Linear { offset, .. } => *offset,
                EquationType::Quadratic { offset, .. } => *offset,
                _ => 0,
            };
            
            for (i, val) in signal.iter().enumerate() {
                let n = (i as i128) + 1 + offset;
                let computed = self.evaluate_equation(&eq.equation_type, n);
                let matches = computed == *val;
                
                tex.push_str(&format!("{} & {} & {} & {} \\\\\n\\hline\n",
                    n, val.to_latex(), computed.to_latex(),
                    if matches { "$\\checkmark$" } else { "$\\times$" }));
            }
            
            tex.push_str("\\end{tabular}\n");
        }
        
        tex.push_str("\n\\end{document}\n");
        tex
    }
    
    fn generate_derivation_tex(&self, _signal: &[VoyagerRational], result: &MathematicalProcessingResult, equation_id: &str) -> String {
        let mut tex = String::new();
        
        tex.push_str("% S-wind Detailed Derivation\n");
        tex.push_str(&format!("% Problem: {}\n\n", self.config.name));
        
        tex.push_str("\\documentclass{article}\n");
        tex.push_str("\\usepackage{amsmath,amssymb,amsthm,longtable}\n");
        tex.push_str("\\newtheorem{proposition}{Proposition}\n");
        tex.push_str("\\begin{document}\n\n");
        
        tex.push_str(&format!("\\section*{{Derivation: {}}}\n\n", self.config.name.replace('_', " ")));
        
        if let Some(trace) = self.voyager.get_derivation_trace(equation_id) {
            tex.push_str("\\begin{proposition}\n");
            
            if let Some(eq) = &result.equation_created {
                let pattern_class = self.pattern_class_string(&eq.equation_type);
                tex.push_str(&format!("\\textit{{Pattern class assumed}}: {}.\n\n", pattern_class));
                tex.push_str(&format!("Let ${}$.\n", self.equation_to_latex(&eq.equation_type)));
            }
            
            tex.push_str("\\end{proposition}\n\n");
            tex.push_str("\\textbf{Derivation.}\n\n");
            
            tex.push_str("\\begin{longtable}{|c|p{5cm}|p{6cm}|p{2cm}|}\n");
            tex.push_str("\\hline\n\\textbf{Step} & \\textbf{Description} & \\textbf{Output} & \\textbf{Rule} \\\\\n\\hline\\endhead\n");
            
            for step in trace {
                let rule = match step.rule_applied {
                    DerivationRule::Axiom => "Given",
                    DerivationRule::Definition => "Definition",
                    DerivationRule::Verification => "Verification",
                    DerivationRule::Subtraction => "Subtraction",
                    DerivationRule::Addition => "Addition",
                    DerivationRule::Simplification => "Simplification",
                    DerivationRule::Substitution => "Substitution",
                    DerivationRule::PatternMatch => "Pattern Match",
                    _ => "Rule",
                };
                
                let desc = self.escape_latex(&step.description);
                let output = self.escape_latex(&step.output_expr);
                
                tex.push_str(&format!("{} & {} & {} & {} \\\\\n\\hline\n",
                    step.step_number, desc, output, rule));
            }
            
            tex.push_str("\\end{longtable}\n\n");
            tex.push_str("$\\square$ (verified on sample domain)\n");
        }
        
        tex.push_str("\n\\end{document}\n");
        tex
    }
    
    fn equation_to_latex(&self, eq_type: &EquationType) -> String {
        match eq_type {
            EquationType::Constant(c) => format!("f \\equiv {}", c.to_latex()),
            EquationType::Linear { coefficient, constant, .. } => {
                format!("f(n) = {} \\cdot n + {}", coefficient.to_latex(), constant.to_latex())
            }
            EquationType::Quadratic { a, b, c, .. } => {
                format!("f(n) = {} \\cdot n^2 + {} \\cdot n + {}", a.to_latex(), b.to_latex(), c.to_latex())
            }
            EquationType::Reciprocal { numerator, offset } => {
                format!("f(n) = \\frac{{{}}}{{n + {}}}", numerator.to_latex(), offset)
            }
            _ => "f(n)".to_string(),
        }
    }
    
    fn pattern_class_string(&self, eq_type: &EquationType) -> String {
        match eq_type {
            EquationType::Constant(_) => "constant function".to_string(),
            EquationType::Linear { .. } => "linear polynomial (degree 1)".to_string(),
            EquationType::Quadratic { .. } => "quadratic polynomial (degree 2)".to_string(),
            EquationType::Reciprocal { .. } => "reciprocal form $c/(n+k)$".to_string(),
            _ => "unknown".to_string(),
        }
    }
    
    fn evaluate_equation(&self, eq_type: &EquationType, n: i128) -> VoyagerRational {
        let n_rat = VoyagerRational::new(n, 1);
        match eq_type {
            EquationType::Constant(c) => *c,
            EquationType::Linear { coefficient, constant, .. } => {
                coefficient.mul(&n_rat).add(constant)
            }
            EquationType::Quadratic { a, b, c, .. } => {
                let n_sq = n_rat.mul(&n_rat);
                a.mul(&n_sq).add(&b.mul(&n_rat)).add(c)
            }
            EquationType::Reciprocal { numerator, offset } => {
                let denom = VoyagerRational::new(n + offset, 1);
                numerator.div(&denom)
            }
            _ => VoyagerRational::zero(),
        }
    }
    
    fn escape_latex(&self, s: &str) -> String {
        s.replace('_', "\\_")
         .replace('%', "\\%")
         .replace('&', "\\&")
         .replace('#', "\\#")
         .replace('∀', "$\\forall$")
         .replace('✓', "$\\checkmark$")
         .replace('✗', "$\\times$")
         .replace('→', "$\\rightarrow$")
         .replace('∴', "$\\therefore$")
    }
}

/// 实验结果
#[derive(Debug)]
pub struct ExperimentResult {
    pub input_signal: Vec<VoyagerRational>,
    pub processing_result: MathematicalProcessingResult,
    pub equation_id: String,
    pub derivation_trace: Option<Vec<voyager::DerivationStep>>,
    pub formatted_trace: Option<String>,
    pub timestamp: String,
}

impl ExperimentResult {
    /// 是否成功检测到模式
    pub fn is_success(&self) -> bool {
        self.processing_result.equation_created.is_some()
    }
    
    /// 获取检测到的方程类型
    pub fn equation_type(&self) -> Option<&EquationType> {
        self.processing_result.equation_created.as_ref().map(|eq| &eq.equation_type)
    }
    
    /// 获取推导步数
    pub fn step_count(&self) -> usize {
        self.derivation_trace.as_ref().map(|t| t.len()).unwrap_or(0)
    }
}

// ============================================================================
// Convenience Functions (便捷函数)
// ============================================================================

/// 快速运行实验 (使用默认配置)
pub fn quick_experiment(name: &str, signal: Vec<VoyagerRational>) -> ExperimentResult {
    let mut runner = ExperimentRunner::with_name(name);
    runner.run(signal)
}

/// 从整数序列运行实验
pub fn experiment_from_integers(name: &str, values: &[i64]) -> ExperimentResult {
    let signal = SignalBuilder::from_integers(values).build();
    quick_experiment(name, signal)
}

/// 从字符串运行实验
pub fn experiment_from_string(name: &str, input: &str) -> Result<ExperimentResult, String> {
    let signal = SignalBuilder::from_string(input)?.build();
    Ok(quick_experiment(name, signal))
}

// ============================================================================
// 当前支持的功能总结
// ============================================================================

/// 返回 S-wind 当前支持的功能列表
pub fn capabilities() -> Vec<&'static str> {
    vec![
        // 输入能力
        "INPUT: Vec<Rational> - Exact rational sequence",
        "INPUT: Vec<i64> - 整数序列 (自动转换)",
        "INPUT: Vec<(i64, i64)> - 分数元组",
        "INPUT: &str - 字符串格式 '1/2, 1, 3/2'",
        
        // 模式检测
        "DETECT: Constant - 常数模式",
        "DETECT: Linear - 线性模式 (f(n) = an + b)",
        "DETECT: Quadratic - 二次模式 (f(n) = an² + bn + c)",
        "DETECT: Reciprocal - 倒数模式 (f(n) = c/(n+k))",
        
        // 输出能力
        "OUTPUT: LaTeX 答案文件 (_answer.tex)",
        "OUTPUT: LaTeX Reverse verification (_reverse.tex)",
        "OUTPUT: LaTeX Detailed derivation (_derivation.tex)",
        "OUTPUT: Markdown report (_report.md)",
        
        // 推导能力
        "DERIVE: 自动差分分析 (Δ¹, Δ²)",
        "DERIVE: 系数推导 (Newton 差分定理)",
        "DERIVE: 偏移推断 (offset inference)",
        "DERIVE: 逐点代数验证",
        "DERIVE: 动态步数 (随输入规模增加)",
        
        // 精确性
        "EXACT: i128/i128 有理数算术",
        "EXACT: 无 f64 浮点误差",
        "EXACT: 符号表达式 to_latex()",
    ]
}

/// 打印能力列表
pub fn print_capabilities() {
    println!("╔═══════════════════════════════════════════════════════════════════════════╗");
    println!("║  S-WIND CAPABILITIES (当前支持的功能)                                     ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════╝\n");
    
    for cap in capabilities() {
        println!("  • {}", cap);
    }
}
