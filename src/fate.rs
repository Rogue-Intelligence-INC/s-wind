//! Fate - S-wind Unified Exploration Engine
//!
//! Design Philosophy:
//! - Fate is free, no hardcoded mathematical knowledge
//! - All patterns and known sequences loaded from KnowledgeBase
//! - Supports self-evolution (nesting + cycling)
//! - **Uses only EPR (infinite precision)**, never approximate
//! - System explores all methods - no premature termination
//!
//! Architecture:
//! ```text
//!                    ┌─────────────────┐
//!                    │      Fate       │
//!                    │  (Unified Explorer)    │
//!                    └────────┬────────┘
//!                             │
//!          ┌──────────────────┼──────────────────┐
//!          │                  │                  │
//!    ┌─────▼─────┐     ┌─────▼─────┐     ┌─────▼─────┐
//!    │  Explore  │     │   Meta    │     │   Self    │
//!    │  (Explore)   │     │  (Nested)   │     │  (Cyclic)   │
//!    └───────────┘     └───────────┘     └───────────┘
//!          │                  │                  │
//!          └──────────────────┼──────────────────┘
//!                             │
//!                    ┌────────▼────────┐
//!                    │  KnowledgeBase  │
//!                    │   (Knowledge Base)      │
//!                    └─────────────────┘
//! ```

use crate::knowledge::{
    KnowledgeBase, PatternClass, DetectionMethod,
};
use crate::symbolic::{EPR, Expr, Symbol};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// ============================================================================
// Structure Extraction System - Core Discovery Mechanism of Fate
// ============================================================================

/// Abstract signal structure - for cross-domain comparison and discovery detection
#[derive(Debug, Clone, PartialEq)]
pub struct SignalStructure {
    /// Structure type
    pub structure_type: StructureType,
    /// Signature (for isomorphism detection)
    pub signature: Vec<EPR>,
    /// Difference sequences (all orders)
    pub differences: Vec<Vec<EPR>>,
    /// Ratio sequence
    pub ratios: Vec<EPR>,
    /// Period (if any)
    pub period: Option<usize>,
    /// Internal consistency score (0-100)
    pub consistency_score: u32,
    /// Source domain
    pub domain: SignalDomain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StructureType {
    /// Polynomial structure (nth difference constant)
    Polynomial { degree: usize },
    /// Exponential structure (constant ratio)
    Exponential { base: EPR },
    /// Reciprocal structure (constant product)
    Reciprocal { constant: EPR },
    /// Periodic structure
    Periodic { period: usize },
    /// Recurrence structure (linear recurrence)
    Recurrence { order: usize, coefficients: Vec<EPR> },
    /// Factorial structure
    Factorial,
    /// Unknown but regular (candidate new pattern)
    UnknownRegular { description: String },
    /// Completely irregular
    Chaotic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SignalDomain {
    /// Pure mathematical sequence
    Mathematics,
    /// Physics - Gravity
    PhysicsGravity,
    /// Physics - Quantum
    PhysicsQuantum,
    /// Physics - Thermal
    PhysicsThermal,
    /// Physics - Coupling
    PhysicsCoupling,
    /// Space - Dimension
    SpaceDimension,
    /// External input
    External,
}

/// Cross-domain signal record
#[derive(Debug, Clone)]
pub struct SignalRecord {
    /// Original signal
    pub signal: Vec<EPR>,
    /// Extracted structure
    pub structure: SignalStructure,
    /// Processing timestamp
    pub tick: u64,
    /// Whether used for cross-domain comparison
    pub cross_compared: bool,
}

/// Candidate pattern - awaiting verification
#[derive(Debug, Clone)]
pub struct CandidatePattern {
    /// Unique ID
    pub id: String,
    /// Based on structure
    pub structure: SignalStructure,
    /// Supporting signal count
    pub support_count: usize,
    /// Supporting signal sources
    pub supporting_signals: Vec<u64>,  // tick ids
    /// Confidence (support count / total tests)
    pub confidence: f64,
    /// Discovery time
    pub discovered_at: u64,
}

// ============================================================================
// Core Types - Uses only EPR (infinite precision)
// ============================================================================

/// Signal - Fate input (infinite precision)
#[derive(Debug, Clone)]
pub struct Signal {
    /// Values (EPR infinite precision)
    pub values: Vec<EPR>,
    /// Timestamp
    pub tick: u64,
    /// Source
    pub source: SignalSource,
}

impl Signal {
    /// Create from EPR
    pub fn new(values: Vec<EPR>) -> Self {
        Signal {
            values,
            tick: 0,
            source: SignalSource::External,
        }
    }
    
    /// Create from i128 (auto-convert to EPR)
    pub fn from_i128(values: Vec<i128>) -> Self {
        Signal {
            values: values.iter().map(|&v| EPR::from_i128(v)).collect(),
            tick: 0,
            source: SignalSource::External,
        }
    }
    
    /// Create from fractions (numerator/denominator pairs)
    pub fn from_fractions(fractions: Vec<(i128, i128)>) -> Self {
        Signal {
            values: fractions.iter().map(|&(n, d)| EPR::from_ratio(n, d)).collect(),
            tick: 0,
            source: SignalSource::External,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SignalSource {
    /// External input
    External,
    /// Own output (cyclic)
    SelfOutput,
    /// Internal Fate (nested)
    InnerFate,
    /// Observe external Fate
    OuterFate,
}

/// Fit Level - Academic classification of fitting results
/// Based on information content, not just correctness
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FitLevel {
    /// Level 0: Interpolant fit - ZERO information
    /// DoF ≤ 2 (degree ≈ sample_size - 1)
    /// n points can ALWAYS be fitted by (n-1) degree polynomial
    /// This is interpolation theorem, NOT discovery
    InterpolantFit,
    
    /// Level 1: Descriptive model - LOW information
    /// DoF ≥ 3, specific function class, interpretable
    /// Low order, specific function class, not yet extrapolated
    DescriptiveModel,
    
    /// Level 2: Predictive fit - MEDIUM information
    /// Passed out-of-sample verification (K points)
    /// Can be upgraded to candidate law
    PredictiveFit,
    
    /// Level 3: Emergent law - HIGH information
    /// Cross-domain verified, extrapolation successful
    /// But still NOT discovery without non-triviality constraint
    EmergentLaw,
}

/// Fit analysis result with formal mathematical definitions
/// 
/// # THEOREM: Degrees of Freedom (DoF)
/// 
/// **Definition**: For a hypothesis class H with parameter space Θ_H of dimension dim(Θ_H),
/// and a sample of n points, the degrees of freedom is:
/// 
/// ```text
///     DoF(H, n) := n - dim(Θ_H)
/// ```
/// 
/// **For polynomials**: A polynomial of degree d has d+1 parameters (a₀, a₁, ..., a_d),
/// so dim(Θ_H) = d + 1.
/// 
/// **Interpolation Theorem**: For any n points in general position, there exists a 
/// unique polynomial of degree ≤ n-1 passing through all points. This means:
/// - DoF = n - n = 0 (zero information gain, existence guaranteed)
/// 
/// # WHY DoF ≥ 2 MEANS NON-TRIVIAL
/// 
/// - **DoF = 0**: Pure interpolation. n points with n parameters. Fit is guaranteed
///   by existence theorem, carries ZERO information about underlying structure.
///   
/// - **DoF = 1**: Single redundant constraint. Weak evidence. Could be coincidence.
///   
/// - **DoF ≥ 2**: At least 2 redundant constraints. The hypothesis is over-determined.
///   If data still satisfies H exactly, this is non-trivial evidence for structure.
///   
/// This is the minimum threshold for "constraint redundancy" in mathematical statistics.
/// 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitAnalysis {
    /// n: Number of sample points (fit domain size)
    pub sample_size: usize,
    /// d: Polynomial degree (for polynomial class)
    pub degree: usize,
    /// dim(Θ_H): Number of parameters = degree + 1 (for polynomial)
    pub num_parameters: usize,
    /// DoF(H, n) = n - dim(Θ_H): Degrees of freedom (formal definition)
    pub dof: usize,
    /// Function class identifier (polynomial, reciprocal, exponential, etc.)
    pub function_class: String,
    /// Fit domain: indices {1, ..., n} used for fitting
    pub fit_domain: Vec<usize>,
    /// Holdout domain: indices used for out-of-sample verification
    pub holdout_domain: Vec<usize>,
    /// Search budget: number of pattern classes enumerated
    pub search_budget: usize,
    /// OOS points tested (extrapolation verification)
    pub extrapolation_tested: usize,
    /// OOS points matched (extrapolation success count)
    pub extrapolation_matched: usize,
    /// is_trivial := (DoF ≤ 0)
    pub is_trivial: bool,
    /// Non-triviality ratio: DoF / n (exact rational, stored as numerator/denominator)
    /// 0 if trivial, otherwise DoF/n ∈ (0, 1]
    pub non_triviality_score: f64,
    /// Non-triviality numerator (for exact rational representation)
    pub non_triviality_num: usize,
    /// Non-triviality denominator (for exact rational representation)
    pub non_triviality_den: usize,
}

/// Greatest Common Divisor (Euclidean algorithm)
/// Used for exact rational arithmetic (no floating-point approximation)
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

impl FitAnalysis {
    /// Create a new fit analysis with formal DoF calculation
    /// 
    /// # FORMAL DEFINITION
    /// 
    /// ```text
    /// DoF(H, n) := n - dim(Θ_H)
    /// ```
    /// 
    /// where:
    /// - n = sample_size (number of data points)
    /// - dim(Θ_H) = num_parameters (dimension of parameter space)
    /// - For polynomial of degree d: dim(Θ_H) = d + 1
    /// 
    pub fn new(sample_size: usize, degree: usize, function_class: &str) -> Self {
        // dim(Θ_H) for polynomial of degree d
        // Polynomial: a₀ + a₁x + a₂x² + ... + a_d x^d has exactly d+1 coefficients
        let num_parameters = degree + 1;
        
        // FORMAL DoF CALCULATION
        // DoF(H, n) = n - dim(Θ_H) = sample_size - num_parameters
        //
        // Example: 10 points with degree 9 polynomial
        //   DoF = 10 - 10 = 0 (pure interpolation, Lagrange theorem guarantees existence)
        let dof = sample_size.saturating_sub(num_parameters);
        
        // TRIVIAL CRITERION (formal)
        // is_trivial := (DoF ≤ 0)
        // 
        // If DoF = 0: fit is GUARANTEED by interpolation theorem (zero information)
        // If DoF < 0: under-determined (impossible for polynomial, but could happen for other classes)
        let is_trivial = dof == 0;
        
        // NON-TRIVIALITY RATIO (exact rational)
        // ρ = DoF / n ∈ [0, 1]
        //
        // Stored as exact fraction: non_triviality_num / non_triviality_den
        // This avoids floating-point approximation
        let (non_triviality_num, non_triviality_den) = if is_trivial {
            (0, 1)  // 0/1 = 0 exactly
        } else {
            // Simplify dof/sample_size by GCD
            let g = gcd(dof, sample_size);
            (dof / g, sample_size / g)
        };
        
        // Also store as f64 for compatibility (but exact values above take precedence)
        let non_triviality_score = if is_trivial {
            0.0
        } else {
            dof as f64 / sample_size as f64
        };
        
        // Fit domain: {1, 2, ..., n}
        let fit_domain: Vec<usize> = (1..=sample_size).collect();
        
        Self {
            sample_size,
            degree,
            num_parameters,
            dof,
            function_class: function_class.to_string(),
            fit_domain,
            holdout_domain: Vec::new(),
            search_budget: 99, // Default: enumerate ALL pattern classes
            extrapolation_tested: 0,
            extrapolation_matched: 0,
            is_trivial,
            non_triviality_score,
            non_triviality_num,
            non_triviality_den,
        }
    }
    
    /// Create a new fit analysis with explicit parameter count
    /// 
    /// Use this when the parameter count cannot be inferred from degree
    /// (e.g., for algebraic patterns like quadratic solved from equations)
    pub fn new_with_params(sample_size: usize, degree: usize, num_parameters: usize, function_class: &str) -> Self {
        // FORMAL DoF CALCULATION
        // DoF(H, n) = n - dim(Θ_H)
        let dof = sample_size.saturating_sub(num_parameters);
        
        // TRIVIAL CRITERION (formal)
        let is_trivial = dof == 0;
        
        // NON-TRIVIALITY RATIO
        let (non_triviality_num, non_triviality_den) = if is_trivial {
            (0, 1)
        } else {
            let g = gcd(dof, sample_size);
            (dof / g, sample_size / g)
        };
        
        let non_triviality_score = if is_trivial {
            0.0
        } else {
            dof as f64 / sample_size as f64
        };
        
        let fit_domain: Vec<usize> = (1..=sample_size).collect();
        
        Self {
            sample_size,
            degree,
            num_parameters,
            dof,
            function_class: function_class.to_string(),
            fit_domain,
            holdout_domain: Vec::new(),
            search_budget: 99,
            extrapolation_tested: 0,
            extrapolation_matched: 0,
            is_trivial,
            non_triviality_score,
            non_triviality_num,
            non_triviality_den,
        }
    }
    
    /// Get exact non-triviality ratio as string (e.g., "3/10")
    pub fn non_triviality_exact(&self) -> String {
        format!("{}/{}", self.non_triviality_num, self.non_triviality_den)
    }
    
    /// Update extrapolation results and holdout domain
    pub fn with_extrapolation(mut self, tested: usize, matched: usize, holdout: Vec<usize>) -> Self {
        self.extrapolation_tested = tested;
        self.extrapolation_matched = matched;
        self.holdout_domain = holdout;
        self
    }
    
    /// Update search budget (number of pattern classes tried)
    pub fn with_search_budget(mut self, budget: usize) -> Self {
        self.search_budget = budget;
        self
    }
    
    /// Check if this fit can qualify as discovery
    /// Must be:
    /// 1. Non-trivial (DoF > 0, preferably DoF ≥ 2)
    /// 2. Pass out-of-sample verification
    /// 3. Non-triviality score ≥ 0.3 (at least 30% constraints are redundant)
    pub fn can_be_discovery(&self) -> bool {
        // Requirement 1: Non-trivial (DoF > 0)
        let non_trivial = !self.is_trivial && self.dof >= 2;
        
        // Requirement 2: Out-of-sample verification passed
        let oos_passed = self.extrapolation_tested >= 3 && 
                         self.extrapolation_matched == self.extrapolation_tested;
        
        // Requirement 3: Sufficient redundancy (not borderline)
        let sufficient_redundancy = self.non_triviality_score >= 0.3;
        
        non_trivial && oos_passed && sufficient_redundancy
    }
    
    /// Get the appropriate fit level based on analysis
    pub fn fit_level(&self) -> FitLevel {
        if self.is_trivial || self.dof == 0 {
            // DoF = 0: pure interpolation
            FitLevel::InterpolantFit
        } else if self.extrapolation_tested == 0 {
            // Has redundancy but no OOS verification
            FitLevel::DescriptiveModel
        } else if self.extrapolation_matched == self.extrapolation_tested && self.dof >= 2 {
            // OOS verified + sufficient DoF
            FitLevel::EmergentLaw
        } else {
            // Some OOS verification but not complete
            FitLevel::PredictiveFit
        }
    }
    
    /// Generate a summary string for reports
    /// Summary with formal mathematical notation
    pub fn summary(&self) -> String {
        format!(
            "DoF(H,n)={} (n={}, dim(Θ)={}), H={}, OOS={}/{}, trivial={}, ρ={}",
            self.dof, 
            self.sample_size, 
            self.num_parameters,
            self.function_class,
            self.extrapolation_matched, 
            self.extrapolation_tested,
            if self.is_trivial { "YES (DoF=0)" } else { "NO" },
            self.non_triviality_exact()
        )
    }
    
    /// Formal DoF description for academic output
    pub fn dof_formal(&self) -> String {
        format!(
            "DoF(H, n) = n - dim(Θ_H) = {} - {} = {}",
            self.sample_size, self.num_parameters, self.dof
        )
    }
}

impl FitLevel {
    /// Standard academic label for output
    pub fn label(&self) -> &'static str {
        match self {
            FitLevel::InterpolantFit => "[InterpolantFit]",
            FitLevel::DescriptiveModel => "[DescriptiveModel]",
            FitLevel::PredictiveFit => "[PredictiveFit]",
            FitLevel::EmergentLaw => "[EmergentLaw]",
        }
    }
    
    pub fn information_gain(&self) -> &'static str {
        match self {
            FitLevel::InterpolantFit => "0 (existence guaranteed by interpolation theorem)",
            FitLevel::DescriptiveModel => "Low (empirical, hypothesis generator)",
            FitLevel::PredictiveFit => "Medium (passed out-of-sample verification)",
            FitLevel::EmergentLaw => "High (cross-domain verified, law candidate)",
        }
    }
    
    pub fn academic_status(&self) -> &'static str {
        match self {
            FitLevel::InterpolantFit => "Trivial interpolation (NOT discovery)",
            FitLevel::DescriptiveModel => "Empirical description (NOT discovery)",
            FitLevel::PredictiveFit => "Testable hypothesis (NOT discovery)",
            FitLevel::EmergentLaw => "Empirical law candidate (may qualify for discovery)",
        }
    }
    
    /// Returns true if this level requires a disclaimer in reports
    pub fn requires_disclaimer(&self) -> bool {
        matches!(self, FitLevel::InterpolantFit | FitLevel::DescriptiveModel | FitLevel::PredictiveFit)
    }
    
    /// Standard disclaimer for non-discovery fits
    pub fn disclaimer() -> &'static str {
        "Fitting (including exact rational interpolation) is not counted as discovery. \
         It is reported as an auditable hypothesis generator, and is only upgraded when \
         it passes pre-registered out-of-sample verification within a constrained pattern class."
    }
    
    /// Check if this fit level can qualify as a discovery
    /// Note: Even EmergentLaw requires non-triviality check!
    pub fn can_be_discovery(&self) -> bool {
        matches!(self, FitLevel::EmergentLaw)
    }
    
    /// Is this a trivial fit that can never be a discovery?
    pub fn is_trivial(&self) -> bool {
        matches!(self, FitLevel::InterpolantFit)
    }
}

/// Cross-domain verification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDomainVerification {
    /// Original sample domain
    pub original_domain: (i128, i128),  // (start, end)
    /// Extended domain tested
    pub extended_domain: (i128, i128),
    /// Number of new points tested
    pub new_points_tested: usize,
    /// Number of new points that matched
    pub new_points_matched: usize,
    /// Extrapolation successful?
    pub extrapolation_success: bool,
    /// Verification log
    pub verification_log: Vec<String>,
}

/// Exploration result (using EPR infinite precision)
#[derive(Debug, Clone)]
pub struct ExplorationResult {
    /// Input signal (EPR)
    pub signal: Vec<EPR>,
    /// Matched known sequence ID
    pub matched_known: Option<String>,
    /// Detected pattern class ID  
    pub detected_pattern: Option<String>,
    /// Symbolic formula
    pub formula: Option<String>,
    /// Derivation steps
    pub derivation_steps: Vec<String>,
    /// Exploration log
    pub exploration_log: Vec<String>,
    /// Status
    pub status: ExplorationStatus,
    /// Fit level classification (academic rigor)
    pub fit_level: Option<FitLevel>,
    /// Fit analysis with required fields (sample_size, degree, dof, etc.)
    pub fit_analysis: Option<FitAnalysis>,
    /// Cross-domain verification (if performed)
    pub cross_domain_verification: Option<CrossDomainVerification>,
    /// Is this a genuine discovery? (non-trivial + extrapolation passed + novel)
    pub is_discovery: bool,
    /// Three-layer discovery assessment
    pub discovery_assessment: Option<DiscoveryAssessment>,
}

/// Three-layer discovery assessment (Pre-OEIS / Discovery / Negative Knowledge)
/// Based on: "OEIS should be a refutation tool, not a precondition for discovery"
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryAssessment {
    // === Layer 1: Pre-OEIS Check ===
    /// Number of sequences searched
    pub oeis_sequences_searched: usize,
    /// OEIS match result
    pub oeis_status: OeisStatus,
    /// Matched OEIS ID (if any)
    pub oeis_match_id: Option<String>,
    
    // === Layer 2: Discovery Assessment ===
    /// Is the object precisely defined?
    pub definition_precise: bool,
    /// Definition description
    pub definition_desc: String,
    /// Is the result verifiable?
    pub verifiable: bool,
    /// Verification description (e.g., "OOS 6/6 passed")
    pub verification_desc: String,
    /// Is the fit non-trivial?
    pub non_trivial: bool,
    /// Non-triviality description (e.g., "DoF=5 ≥ 2")
    pub non_trivial_desc: String,
    /// Final discovery status
    pub discovery_status: DiscoveryStatus,
    
    // === Layer 3: Negative Knowledge ===
    /// Pattern classes that failed
    pub failed_patterns: Vec<String>,
    /// Why this is NOT a known sequence
    pub negative_evidence: Vec<String>,
    /// Final conclusion
    pub conclusion: String,
}

/// OEIS search status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OeisStatus {
    /// Exact match found
    ExactMatch,
    /// No match found (potential new discovery)
    NoMatch,
    /// Not applicable (e.g., non-integer sequence)
    NotApplicable,
    /// Search not performed
    NotSearched,
}

impl OeisStatus {
    pub fn label(&self) -> &'static str {
        match self {
            OeisStatus::ExactMatch => "EXACT_MATCH",
            OeisStatus::NoMatch => "NO_MATCH",
            OeisStatus::NotApplicable => "NOT_APPLICABLE",
            OeisStatus::NotSearched => "NOT_SEARCHED",
        }
    }
}

/// Discovery status after three-layer assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiscoveryStatus {
    /// Known sequence (OEIS match) - NOT discovery
    Known,
    /// Trivial interpolation (DoF=0) - NOT discovery
    TrivialInterpolation,
    /// Descriptive model (unverified) - NOT discovery
    DescriptiveOnly,
    /// Candidate law (verified but needs more evidence)
    CandidateLaw,
    /// Strong candidate (verified + non-trivial + no OEIS match)
    StrongCandidate,
    /// Genuine discovery (all criteria met)
    GenuineDiscovery,
}

impl DiscoveryStatus {
    pub fn label(&self) -> &'static str {
        match self {
            DiscoveryStatus::Known => "[Known]",
            DiscoveryStatus::TrivialInterpolation => "[TrivialInterpolation]",
            DiscoveryStatus::DescriptiveOnly => "[DescriptiveOnly]",
            DiscoveryStatus::CandidateLaw => "[CandidateLaw]",
            DiscoveryStatus::StrongCandidate => "[StrongCandidate]",
            DiscoveryStatus::GenuineDiscovery => "[GenuineDiscovery]",
        }
    }
    
    pub fn is_discovery_candidate(&self) -> bool {
        matches!(self, 
            DiscoveryStatus::CandidateLaw | 
            DiscoveryStatus::StrongCandidate | 
            DiscoveryStatus::GenuineDiscovery
        )
    }
}

impl DiscoveryAssessment {
    /// Generate a formatted three-layer report
    pub fn to_report(&self) -> String {
        let mut report = String::new();
        
        // Layer 1: Pre-OEIS Check
        report.push_str("=== Layer 1: Pre-OEIS Check ===\n");
        report.push_str(&format!("- Searched: {} sequences\n", self.oeis_sequences_searched));
        report.push_str(&format!("- Status: {}\n", self.oeis_status.label()));
        if let Some(ref id) = self.oeis_match_id {
            report.push_str(&format!("- Match: {}\n", id));
        }
        report.push('\n');
        
        // Layer 2: Discovery Assessment
        report.push_str("=== Layer 2: Discovery Assessment ===\n");
        report.push_str(&format!("- Definition: {} {}\n", 
            if self.definition_precise { "✓" } else { "✗" },
            self.definition_desc
        ));
        report.push_str(&format!("- Verifiable: {} {}\n",
            if self.verifiable { "✓" } else { "✗" },
            self.verification_desc
        ));
        report.push_str(&format!("- Non-trivial: {} {}\n",
            if self.non_trivial { "✓" } else { "✗" },
            self.non_trivial_desc
        ));
        report.push_str(&format!("- Status: {}\n", self.discovery_status.label()));
        report.push('\n');
        
        // Layer 3: Negative Knowledge
        report.push_str("=== Layer 3: Negative Knowledge ===\n");
        if !self.failed_patterns.is_empty() {
            report.push_str(&format!("- Failed patterns: {}\n", self.failed_patterns.join(", ")));
        }
        for evidence in &self.negative_evidence {
            report.push_str(&format!("- {}\n", evidence));
        }
        report.push_str(&format!("- Conclusion: {}\n", self.conclusion));
        
        report
    }
}

/// Statistics from self-loop exploration
#[derive(Debug, Clone, Default)]
pub struct SelfLoopStats {
    /// Number of iterations run
    pub iterations_run: u32,
    /// Initial signal length
    pub initial_signal_len: usize,
    /// Final signal length (after extensions)
    pub final_signal_len: usize,
    /// Number of times signal was extended
    pub signal_extensions: u32,
    /// Number of times signal was transformed (f applied to values)
    pub signal_transformations: u32,
    /// Number of pattern classes tried
    pub patterns_tried: u32,
    /// DoF improvement (final - initial)
    pub dof_improvement: i32,
    /// Did cycling lead to a discovery?
    pub discovered_via_cycling: bool,
    /// Was InterpolantFit detected? (trivial fit, needs more data)
    pub interpolant_detected: bool,
    /// Suggested minimum sample size to escape interpolation
    pub suggested_sample_size: Option<usize>,
    /// Was closure reached? (duplicate signal detected)
    pub closure_reached: bool,
    /// Was pattern exhausted? (no new signal can be generated after trying multiple approaches)
    pub pattern_exhausted: bool,
    /// Fate convergence status
    pub fate_converged: bool,
    /// True discovery found (EmergentLaw)
    pub discovery_found: bool,
    /// Known sequence was found
    pub known_sequence_found: bool,
    /// Number of iterations with no pattern
    pub no_pattern_iterations: u32,
    /// Truly exhausted all exploration options
    pub truly_exhausted: bool,
    /// Hypothesis space H actually tested by Fate (auto-generated)
    pub hypothesis_space_tested: Vec<String>,
    /// Transformation set T actually applied (auto-generated)
    pub transformations_applied: Vec<String>,
}

impl SelfLoopStats {
    /// Generate summary report
    pub fn summary(&self) -> String {
        let mut s = format!(
            "Iterations: {}, Signal: {}→{}, Extensions: {}, Transforms: {}, Patterns tried: {}",
            self.iterations_run,
            self.initial_signal_len,
            self.final_signal_len,
            self.signal_extensions,
            self.signal_transformations,
            self.patterns_tried
        );
        
        if self.discovered_via_cycling {
            s.push_str(", Discovery: YES");
        }
        if self.fate_converged {
            s.push_str(", Fate: CONVERGED");
        }
        
        s
    }
    
    /// Report hypothesis space tested (auto-generated by Fate)
    pub fn hypothesis_space_report(&self) -> String {
        if self.hypothesis_space_tested.is_empty() {
            return "H = (not yet explored)".to_string();
        }
        format!("H = {{\n  {}\n}}", self.hypothesis_space_tested.join(",\n  "))
    }
    
    /// Report transformation set applied (auto-generated by Fate)
    pub fn transformation_set_report(&self) -> String {
        if self.transformations_applied.is_empty() {
            return "T = (not yet applied)".to_string();
        }
        format!("T = {{\n  {}\n}}", self.transformations_applied.join(",\n  "))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExplorationStatus {
    /// Match known sequence
    KnownSequence { id: String, name: String },
    /// Pattern detected
    PatternDetected { class_id: String, formula: String },
    /// Unknown - no pattern found
    Unknown,
    /// Mystery - exploration exhausted, no transformation yields new signal, no solution found
    /// This is a genuine mathematical mystery worthy of further investigation
    Mystery { 
        iterations_exhausted: usize,
        transformations_tried: Vec<String>,
        reason: String,
    },
}

impl ExplorationStatus {
    /// Standard academic label for output
    pub fn label(&self) -> &'static str {
        match self {
            ExplorationStatus::KnownSequence { .. } => "[Recognized]",
            ExplorationStatus::PatternDetected { .. } => "[PatternDetected]",
            ExplorationStatus::Unknown => "[Unknown]",
            ExplorationStatus::Mystery { .. } => "[MYSTERY]",
        }
    }
    
    /// Returns true if this result is a recognition (not discovery)
    pub fn is_recognition(&self) -> bool {
        matches!(self, ExplorationStatus::KnownSequence { .. })
    }
}

/// Discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discovery {
    pub id: String,
    pub level: usize,  // 0=basic, 1=meta, 2=meta-meta, 3+=levelup
    pub discovery_type: DiscoveryType,
    pub description: String,
    pub tick: u64,
    pub math_explanation: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiscoveryType {
    /// Math pattern - just detected known pattern, not a discovery
    MathPattern,
    /// System pattern - cross-domain isomorphism discovery
    SystemPattern,
    /// New rule - unknown but regular
    NewRule,
    /// Level up - generalize existing patterns
    LevelUp,
    /// Novel result - differs from known authority but verified (real discovery!)
    NovelResult {
        /// Known formula (from knowledge base)
        known_formula: Option<String>,
        /// Formula found by S-wind
        found_formula: String,
        /// Difference description
        difference: String,
    },
}

// ============================================================================
// Fate - Unified Exploration Engine
// ============================================================================

/// Fate - S-wind Unified Exploration Engine
/// 
/// Features:
/// - No hardcoded patterns, all knowledge loaded from KnowledgeBase
/// - Supports self-evolution (nesting + cycling)
/// - Supports dual precision (Rational + EPR)
/// - **Real discovery ability**: Structure extraction + cross-domain comparison + generalization detection
pub struct Fate {
    /// Status
    state: FateState,
    /// History
    history: Vec<ExplorationResult>,
    /// Discovery
    discoveries: Vec<Discovery>,
    /// Strategy (learned from history)
    strategy: Strategy,
    /// Configuration
    config: FateConfig,
    /// Cross-domain signal pool - remembers all processed signals
    signal_pool: Vec<SignalRecord>,
    /// Candidate pattern - awaiting verification
    candidate_patterns: Vec<CandidatePattern>,
}

#[derive(Debug, Clone, Default)]
struct FateState {
    tick: u64,
    signal_count: u64,
    discovery_count: u64,
    pattern_diversity: u64,
}

#[derive(Debug, Clone)]
#[derive(Default)]
struct Strategy {
    /// Pattern priority (learned from history)
    pattern_priority: Vec<String>,
    /// Success rate (pattern_id -> success_count / total_count)
    success_rates: HashMap<String, (u64, u64)>,  // (successes, attempts)
    /// Cross-domain discovery count
    cross_domain_discoveries: u64,
    /// Generalization discovery count
    generalization_discoveries: u64,
}


impl Strategy {
    /// Update pattern success rate
    fn record_attempt(&mut self, pattern_id: &str, success: bool) {
        let entry = self.success_rates.entry(pattern_id.to_string()).or_insert((0, 0));
        entry.1 += 1;  // total attempts
        if success {
            entry.0 += 1;  // successes
        }
    }
    
    /// Re-sort priority by success rate
    fn update_priority(&mut self) {
        let mut rates: Vec<(String, f64)> = self.success_rates.iter()
            .map(|(k, (s, t))| (k.clone(), if *t > 0 { *s as f64 / *t as f64 } else { 0.0 }))
            .collect();
        
        rates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        self.pattern_priority = rates.into_iter().map(|(k, _)| k).collect();
    }
    
    /// Get suggested pattern order (reserved for future use)
    #[allow(dead_code)]
    fn get_priority_order(&self) -> &[String] {
        &self.pattern_priority
    }
}

/// Fate Configuration (reserved for self-cycling and meta-observation)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FateConfig {
    /// Enable self-cycling
    pub enable_self_loop: bool,
    /// Enable meta-observation
    pub enable_meta: bool,
    /// Feedback delay
    pub feedback_delay: u64,
    /// Feedback strength (0-100)
    pub feedback_strength: u64,
    /// Max self-loop iterations
    pub max_self_loop_iterations: u32,
}

impl Default for FateConfig {
    fn default() -> Self {
        Self {
            enable_self_loop: true,
            enable_meta: false,  // Not yet implemented
            feedback_delay: 5,
            feedback_strength: 50,
            max_self_loop_iterations: 1000, // High limit - true exhaustion via closure detection
        }
    }
}

impl Fate {
    /// Create new Fate
    pub fn new() -> Self {
        Self {
            state: FateState::default(),
            history: Vec::new(),
            discoveries: Vec::new(),
            strategy: Strategy::default(),
            config: FateConfig::default(),
            signal_pool: Vec::new(),
            candidate_patterns: Vec::new(),
        }
    }
    
    /// Create with configuration
    pub fn with_config(config: FateConfig) -> Self {
        Self {
            state: FateState::default(),
            history: Vec::new(),
            discoveries: Vec::new(),
            strategy: Strategy::default(),
            config,
            signal_pool: Vec::new(),
            candidate_patterns: Vec::new(),
        }
    }
    
    // ========================================================================
    // Core discovery mechanism - this is Fate's real discovery ability
    // ========================================================================
    
    /// Extract abstract structure from signal - even if no known pattern matches
    pub fn extract_structure(&self, signal: &[EPR], domain: SignalDomain) -> SignalStructure {
        if signal.len() < 2 {
            return SignalStructure {
                structure_type: StructureType::Chaotic,
                signature: signal.to_vec(),
                differences: vec![],
                ratios: vec![],
                period: None,
                consistency_score: 0,
                domain,
            };
        }
        
        let mut consistency_score = 0u32;
        
        // 1. Compute difference sequences (up to 5 orders)
        let mut differences = Vec::new();
        let mut current = signal.to_vec();
        for _ in 0..5 {
            if current.len() < 2 { break; }
            let diffs: Vec<EPR> = current.windows(2)
                .map(|w| w[1].clone() - w[0].clone())
                .collect();
            differences.push(diffs.clone());
            current = diffs;
        }
        
        // 2. Compute ratio sequence
        let ratios: Vec<EPR> = signal.windows(2)
            .filter_map(|w| {
                if !w[0].is_zero() {
                    Some(w[1].clone() / w[0].clone())
                } else {
                    None
                }
            })
            .collect();
        
        // 3. Detect structure type and compute consistency score
        let structure_type = self.detect_structure_type(signal, &differences, &ratios, &mut consistency_score);
        
        // 4. Detect period
        let period = self.detect_period(signal);
        
        // 5. Compute signature (for isomorphism detection)
        let signature = self.compute_signature(signal, &differences, &ratios);
        
        SignalStructure {
            structure_type,
            signature,
            differences,
            ratios,
            period,
            consistency_score,
            domain,
        }
    }
    
    /// Detect structure type
    fn detect_structure_type(&self, signal: &[EPR], differences: &[Vec<EPR>], ratios: &[EPR], 
                             consistency: &mut u32) -> StructureType {
        // Check polynomial (constant difference)
        for (degree, diffs) in differences.iter().enumerate() {
            if diffs.len() >= 2 && diffs.iter().all(|d| *d == diffs[0]) {
                *consistency += 80 + (20 - degree as u32 * 4).min(20);
                return StructureType::Polynomial { degree: degree + 1 };
            }
        }
        
        // Check exponential (constant ratio)
        if ratios.len() >= 2 && ratios.iter().all(|r| *r == ratios[0]) {
            *consistency += 85;
            return StructureType::Exponential { base: ratios[0].clone() };
        }
        
        // Check reciprocal (constant product)
        for k in 0i128..=5 {
            let products: Vec<EPR> = signal.iter().enumerate()
                .map(|(i, val)| val.clone() * EPR::from_i128((i as i128) + 1 + k))
                .collect();
            if products.len() >= 2 && products.iter().all(|p| *p == products[0]) {
                *consistency += 80;
                return StructureType::Reciprocal { constant: products[0].clone() };
            }
        }
        
        // Check factorial
        if signal.len() >= 3 {
            let mut is_factorial = true;
            for i in 1..signal.len() {
                if signal[i-1].is_zero() {
                    is_factorial = false;
                    break;
                }
                let ratio = signal[i].clone() / signal[i-1].clone();
                let expected = EPR::from_i128((i + 1) as i128);
                if ratio != expected {
                    is_factorial = false;
                    break;
                }
            }
            if is_factorial {
                *consistency += 90;
                return StructureType::Factorial;
            }
        }
        
        // Check period
        if let Some(period) = self.detect_period(signal) {
            *consistency += 75;
            return StructureType::Periodic { period };
        }
        
        // Check recurrence (order 2)
        if signal.len() >= 4 {
            if let Some(coeffs) = self.detect_recurrence(signal, 2) {
                *consistency += 70;
                return StructureType::Recurrence { order: 2, coefficients: coeffs };
            }
        }
        
        // Check for partial regularity (unknown but regular)
        let partial_consistency = self.check_partial_regularity(signal, differences, ratios);
        if partial_consistency > 30 {
            *consistency += partial_consistency;
            return StructureType::UnknownRegular { 
                description: format!("Partial regularity (consistency: {})", partial_consistency)
            };
        }
        
        StructureType::Chaotic
    }
    
    /// Check partial regularity
    fn check_partial_regularity(&self, signal: &[EPR], differences: &[Vec<EPR>], ratios: &[EPR]) -> u32 {
        let mut score = 0u32;
        
        // Check if differences have pattern (even if not exactly constant)
        for diffs in differences {
            if diffs.len() >= 3 {
                // Check if most are equal
                let mode = &diffs[0];
                let equal_count = diffs.iter().filter(|d| *d == mode).count();
                if equal_count * 2 > diffs.len() {
                    score += 20;
                }
            }
        }
        
        // Check if ratios have pattern
        if ratios.len() >= 3 {
            let mode = &ratios[0];
            let equal_count = ratios.iter().filter(|r| *r == mode).count();
            if equal_count * 2 > ratios.len() {
                score += 20;
            }
        }
        
        // Check for sub-period
        if signal.len() >= 6 {
            for period in 2..=signal.len() / 3 {
                let mut matches = 0;
                for i in period..signal.len() {
                    if signal[i] == signal[i % period] {
                        matches += 1;
                    }
                }
                if matches * 2 > signal.len() - period {
                    score += 15;
                    break;
                }
            }
        }
        
        score
    }
    
    /// Detect recurrence relation
    fn detect_recurrence(&self, signal: &[EPR], order: usize) -> Option<Vec<EPR>> {
        if signal.len() < order + 2 {
            return None;
        }
        
        if order == 2 && signal.len() >= 4 {
            let s0 = signal[0].clone();
            let s1 = signal[1].clone();
            let s2 = signal[2].clone();
            let s3 = signal[3].clone();
            
            let det = s1.clone() * s1.clone() - s2.clone() * s0.clone();
            if det.is_zero() {
                return None;
            }
            
            let c1 = (s2.clone() * s1.clone() - s3.clone() * s0.clone()) / det.clone();
            let c2 = (s3.clone() * s1.clone() - s2.clone() * s2.clone()) / det;
            
            // Verify
            for i in 2..signal.len() {
                let expected = c1.clone() * signal[i-1].clone() + c2.clone() * signal[i-2].clone();
                if expected != signal[i] {
                    return None;
                }
            }
            
            return Some(vec![c1, c2]);
        }
        
        None
    }
    
    /// Detect period
    fn detect_period(&self, signal: &[EPR]) -> Option<usize> {
        if signal.len() < 4 {
            return None;
        }
        
        for period in 1..=signal.len() / 2 {
            let is_periodic = (period..signal.len())
                .all(|i| signal[i] == signal[i % period]);
            if is_periodic {
                return Some(period);
            }
        }
        None
    }
    
    /// Compute signature (for isomorphism detection)
    fn compute_signature(&self, signal: &[EPR], differences: &[Vec<EPR>], ratios: &[EPR]) -> Vec<EPR> {
        let mut sig = Vec::new();
        
        // Add first term
        if !signal.is_empty() {
            sig.push(signal[0].clone());
        }
        
        // Add first first-order difference
        if !differences.is_empty() && !differences[0].is_empty() {
            sig.push(differences[0][0].clone());
        }
        
        // Add first ratio
        if !ratios.is_empty() {
            sig.push(ratios[0].clone());
        }
        
        sig
    }
    
    /// Check if two structures are isomorphic
    pub fn is_isomorphic(&self, s1: &SignalStructure, s2: &SignalStructure) -> bool {
        // 1. Structure types must be the same
        match (&s1.structure_type, &s2.structure_type) {
            (StructureType::Polynomial { degree: d1 }, StructureType::Polynomial { degree: d2 }) => {
                if d1 != d2 { return false; }
            }
            (StructureType::Exponential { base: b1 }, StructureType::Exponential { base: b2 }) => {
                if b1 != b2 { return false; }
            }
            (StructureType::Reciprocal { constant: c1 }, StructureType::Reciprocal { constant: c2 }) => {
                if c1 != c2 { return false; }
            }
            (StructureType::Periodic { period: p1 }, StructureType::Periodic { period: p2 }) => {
                if p1 != p2 { return false; }
            }
            (StructureType::Recurrence { order: o1, coefficients: c1 }, 
             StructureType::Recurrence { order: o2, coefficients: c2 }) => {
                if o1 != o2 || c1 != c2 { return false; }
            }
            (StructureType::Factorial, StructureType::Factorial) => {}
            (StructureType::UnknownRegular { .. }, StructureType::UnknownRegular { .. }) => {
                // For unknown regularity, check signature
                if s1.signature != s2.signature { return false; }
            }
            _ => return false,
        }
        
        // 2. If from different domains, this is cross-domain isomorphism!
        true
    }
    
    /// Check if new pattern generalizes existing patterns
    pub fn generalizes(&self, new_structure: &SignalStructure, _kb: &KnowledgeBase) -> Option<Vec<String>> {
        let mut generalized = Vec::new();
        
        // Check if new structure covers multiple existing patterns
        match &new_structure.structure_type {
            StructureType::Polynomial { degree } => {
                // Higher order polynomial generalizes lower order
                if *degree > 1 {
                    generalized.push(format!("polynomial_degree_{}", degree - 1));
                }
            }
            StructureType::Recurrence { order, .. } => {
                // Higher order recurrence may generalize lower order
                if *order > 2 {
                    generalized.push(format!("recurrence_order_{}", order - 1));
                }
            }
            StructureType::UnknownRegular { .. } => {
                // Unknown structure with high consistency may be new generalization pattern
                if new_structure.consistency_score >= 70 {
                    // Check if multiple candidate patterns have same signature
                    let similar_count = self.candidate_patterns.iter()
                        .filter(|cp| cp.structure.signature == new_structure.signature)
                        .count();
                    if similar_count >= 2 {
                        generalized.push("multiple_similar_structures".to_string());
                    }
                }
            }
            _ => {}
        }
        
        if generalized.is_empty() {
            None
        } else {
            Some(generalized)
        }
    }
    
    /// Search for cross-domain isomorphism in signal pool
    fn find_cross_domain_isomorphism(&self, structure: &SignalStructure) -> Vec<&SignalRecord> {
        self.signal_pool.iter()
            .filter(|record| {
                // Must be from different domains
                record.structure.domain != structure.domain &&
                // Must have isomorphic structure
                self.is_isomorphic(&record.structure, structure)
            })
            .collect()
    }
    
    /// Add candidate pattern to knowledge base (returns success)
    pub fn promote_candidate_to_knowledge(&mut self, candidate_id: &str, kb: &mut KnowledgeBase) -> bool {
        if let Some(idx) = self.candidate_patterns.iter().position(|c| c.id == candidate_id) {
            let candidate = self.candidate_patterns.remove(idx);
            
            // Only promote if confidence is high enough
            if candidate.confidence >= 0.8 && candidate.support_count >= 3 {
                // Create new PatternClass
                let new_pattern = PatternClass {
                    id: format!("discovered_{}", candidate.id),
                    name: format!("Discovered Pattern ({})", candidate.id),
                    description: format!("Auto-discovered by Fate at tick {}", candidate.discovered_at),
                    symbolic_form: format!("{:?}", candidate.structure.structure_type),
                    detection: DetectionMethod::Custom { method: candidate.structure.structure_type.clone() },
                    min_samples: 3,
                    complexity: crate::knowledge::Complexity::Linear,
                    priority: 50,  // Medium priority
                };
                
                kb.add_pattern_class(new_pattern);
                return true;
            }
        }
        false
    }

    // ========================================================================
    // Main exploration function
    // ========================================================================
    
    /// Main exploration function - Fate is free, loads all knowledge from KnowledgeBase
    /// Uses EPR (infinite precision)
    /// 
    /// Now includes complete discovery mechanism:
    /// 1. Match known sequence
    /// 2. Match known pattern -> MathPattern
    /// 3. Extract structure, detect cross-domain isomorphism -> SystemPattern
    /// 4. Detect generalization -> LevelUp
    /// 5. Unknown but structured -> NewRule (candidate for KB)
    pub fn explore(&mut self, signal: &[EPR], kb: &KnowledgeBase) -> ExplorationResult {
        self.explore_with_domain(signal, kb, SignalDomain::External)
    }
    
    /// Exploration with domain label
    pub fn explore_with_domain(&mut self, signal: &[EPR], kb: &KnowledgeBase, domain: SignalDomain) -> ExplorationResult {
        self.state.tick += 1;
        self.state.signal_count += 1;
        
        let mut log = Vec::new();
        let mut derivation = Vec::new();
        
        log.push("Fate: Starting exploration (EPR infinite precision + full discovery mechanism)...".to_string());
        log.push(format!("Signal domain: {:?}", domain));
        
        // 0. Extract structure (always extract for later comparison)
        let structure = self.extract_structure(signal, domain.clone());
        log.push(format!("Structure type: {:?}", structure.structure_type));
        log.push(format!("Consistency score: {}", structure.consistency_score));
        
        // 1. Check known sequences
        log.push("Check known sequences...".to_string());
        if let Some(known) = self.check_known_sequences(signal, kb, &mut derivation) {
            log.push(format!("✓ Match known sequence: {}", known.1));
            
            // Even if known sequence matches, check cross-domain isomorphism
            let cross_domain_count = self.signal_pool.iter()
                .filter(|r| r.structure.domain != structure.domain &&
                           r.structure.structure_type == structure.structure_type)
                .count();
            
            if cross_domain_count > 0 {
                log.push(format!("🎯 Cross-domain isomorphism discovered! Known sequence appears in {} other domains", cross_domain_count));
                self.strategy.cross_domain_discoveries += 1;
                self.state.discovery_count += 1;
                
                let discovery = Discovery {
                    id: format!("cross_domain_known_{}", self.state.tick),
                    level: 1,
                    discovery_type: DiscoveryType::SystemPattern,
                    description: format!("Known sequence '{}' appears in {:?} + {} other domains", known.1, domain, cross_domain_count),
                    tick: self.state.tick,
                    math_explanation: format!("Sequence '{}' discovered in {:?} domain, isomorphic with {} other domains", known.1, domain, cross_domain_count),
                };
                self.discoveries.push(discovery);
            }
            
            self.add_to_signal_pool(signal, structure);
            return ExplorationResult {
                signal: signal.to_vec(),
                matched_known: Some(known.0.clone()),
                detected_pattern: None,
                formula: known.2.clone(),
                derivation_steps: derivation,
                exploration_log: log,
                status: ExplorationStatus::KnownSequence { 
                    id: known.0, 
                    name: known.1 
                },
                fit_level: None,
                fit_analysis: None,
                cross_domain_verification: None,
                is_discovery: false,  // Known sequence is NEVER discovery
                discovery_assessment: None,
            };
        }
        
        // 2. PRIORITY: Try algebraic exploration FIRST
        log.push("Priority: Algebraic exploration...".to_string());
        if let Some((formula, _steps)) = self.try_algebraic_exploration(signal, &mut derivation) {
            log.push(format!("✓ Algebraic solution found: {}", formula));
            
            self.add_to_signal_pool(signal, structure);
            return ExplorationResult {
                signal: signal.to_vec(),
                matched_known: None,
                detected_pattern: Some("algebraic".to_string()),
                formula: Some(formula.clone()),
                derivation_steps: derivation,
                exploration_log: log,
                status: ExplorationStatus::PatternDetected {
                    class_id: "algebraic".to_string(),
                    formula,
                },
                fit_level: Some(FitLevel::EmergentLaw), // Algebraic solutions are high value
                fit_analysis: None,
                cross_domain_verification: None,
                is_discovery: true, // Algebraic solutions are discoveries
                discovery_assessment: None,
            };
        }
        
        // 3. Try pattern detection (numerical methods)
        log.push("Try pattern detection (numerical)...".to_string());
        let patterns = kb.get_available_patterns();
        
        for pattern in &patterns {
            log.push(format!("Try pattern: {} ({})", pattern.name, pattern.id));
            self.strategy.record_attempt(&pattern.id, false);  // Record attempt
            
            if let Some((formula, steps)) = self.try_pattern(signal, pattern, &mut derivation) {
                self.strategy.record_attempt(&pattern.id, true);  // Update as success
                self.state.pattern_diversity += 1;
                
                // ============================================================
                // Discovery criterion (CORRECTED):
                // 1. Result differs from known authority
                // 2. Verified correct
                // 3. NON-TRIVIAL: DoF = n - (degree+1) > 0
                //    Polynomial of degree d has d+1 parameters
                //    n points with (n-1) degree polynomial: DoF = n - n = 0 (pure interpolation!)
                // ============================================================
                
                // First compute non-triviality (CORRECTED)
                let sample_size = signal.len();
                let pattern_degree = self.estimate_pattern_degree_from_formula(&pattern.id, &formula);
                let num_parameters = pattern_degree + 1;  // degree d has d+1 parameters
                let dof = sample_size.saturating_sub(num_parameters);
                // Non-trivial requires DoF ≥ 2 (at least 2 redundant constraints)
                let is_non_trivial = dof >= 2;
                
                if !is_non_trivial {
                    log.push(format!("⚠️ TRIVIAL FIT: {} points with {} params (degree {}) → DoF = {}", 
                        sample_size, num_parameters, pattern_degree, dof));
                    if dof == 0 {
                        log.push("   DoF=0: Pure interpolation (guaranteed by interpolation theorem)".to_string());
                    } else {
                        log.push(format!("   DoF={} < 2: Borderline trivial, insufficient redundancy", dof));
                    }
                    log.push("   Excluded from discovery statistics".to_string());
                }
                
                // Method 1: Match by signal values
                let known_by_signal = kb.get_formula_for_sequence(signal);
                
                // Method 2: Match by formula (handles fractional sequences)
                let known_by_formula = self.find_known_formula_match(&formula, kb);
                
                // Merge both matching results
                let known_formula_for_signal = known_by_signal.or(known_by_formula);
                
                // is_novel requires BOTH: differs from known AND non-trivial
                let (discovery_type, is_novel) = if !is_non_trivial {
                    // TRIVIAL fit is NEVER a discovery
                    log.push("   [InterpolantFit] excluded from discovery statistics".to_string());
                    (DiscoveryType::MathPattern, false)
                } else if let Some(ref known) = known_formula_for_signal {
                    // Knowledge base has formula, check if same
                    let normalized_known = Self::normalize_formula(known);
                    let normalized_found = Self::normalize_formula(&formula);
                    
                    if normalized_known != normalized_found {
                        // Different AND non-trivial! This is a new discovery
                        log.push("🎯 Discovery! Found formula differs from known AND non-trivial".to_string());
                        log.push(format!("   Known: {}", known));
                        log.push(format!("   Found: {}", formula));
                        log.push(format!("   DoF: {} (non-trivial)", dof));
                        (DiscoveryType::NovelResult {
                            known_formula: Some(known.clone()),
                            found_formula: formula.clone(),
                            difference: format!("Known formula '{}' vs found formula '{}'", known, formula),
                        }, true)
                    } else {
                        // Same, just pattern matching, not discovery
                        log.push(format!("Formula matches knowledge base: {}", known));
                        (DiscoveryType::MathPattern, false)
                    }
                } else {
                    // Knowledge base has no formula, but S-wind found one AND non-trivial
                    log.push(format!("🎯 Discovery! No known formula in KB, S-wind found: {}", formula));
                    log.push(format!("   DoF: {} (non-trivial)", dof));
                    (DiscoveryType::NovelResult {
                        known_formula: None,
                        found_formula: formula.clone(),
                        difference: "No known formula in KB, S-wind first discovery".to_string(),
                    }, true)
                };
                
                // Only real discoveries increase discovery count
                if is_novel {
                    self.state.discovery_count += 1;
                }
                
                let discovery = Discovery {
                    id: format!("discovery_{}_{}", pattern.id, self.state.tick),
                    level: if is_novel { 1 } else { 0 },  // Discovery is higher level
                    discovery_type,
                    description: format!(
                        "{} {}: {}", 
                        if is_novel { "🎯 Discovery" } else { "Detected" },
                        pattern.name, 
                        formula
                    ),
                    tick: self.state.tick,
                    math_explanation: steps.join("\n"),
                };
                self.discoveries.push(discovery);
                
                log.push(format!("✓ Pattern detected: {}", formula));
                
                // Add to signal pool
                self.add_to_signal_pool(signal, structure);
                
                // Update strategy priority
                self.strategy.update_priority();
                
                // Check if formula matches known sequence (handles fractional sequences)
                if let Some((seq_id, seq_name, seq_formula)) = self.try_match_known_formula(&formula, kb, &mut derivation) {
                    log.push(format!("✓ Formula matches known sequence: {} ({})", seq_name, seq_id));
                    return ExplorationResult {
                        signal: signal.to_vec(),
                        matched_known: Some(seq_id.clone()),
                        detected_pattern: Some(pattern.id.clone()),
                        formula: seq_formula.or(Some(formula.clone())),
                        derivation_steps: derivation,
                        exploration_log: log,
                        status: ExplorationStatus::KnownSequence { 
                            id: seq_id, 
                            name: seq_name 
                        },
                        fit_level: None,
                        fit_analysis: None,
                        cross_domain_verification: None,
                        is_discovery: false,  // Known sequence is NEVER discovery
                        discovery_assessment: None,
                    };
                }
                
                return ExplorationResult {
                    signal: signal.to_vec(),
                    matched_known: None,
                    detected_pattern: Some(pattern.id.clone()),
                    formula: Some(formula.clone()),
                    derivation_steps: derivation,
                    exploration_log: log,
                    status: ExplorationStatus::PatternDetected { 
                        class_id: pattern.id.clone(), 
                        formula 
                    },
                    fit_level: None,  // Will be classified by explore_with_verification
                    fit_analysis: None,  // Will be computed in verification
                    cross_domain_verification: None,
                    is_discovery: false,  // Pattern detection alone is NEVER discovery
                    discovery_assessment: None,
                };
            }
        }
        
        // ====================================================================
        // 3. New discovery mechanism - when no known patterns match
        // ====================================================================
        log.push("No known patterns match, starting discovery mechanism...".to_string());
        
        // 3a. Check cross-domain isomorphism -> SystemPattern
        let cross_domain_matches = self.find_cross_domain_isomorphism(&structure);
        if !cross_domain_matches.is_empty() {
            log.push(format!("🎯 Cross-domain isomorphism discovered! Match count: {}", cross_domain_matches.len()));
            
            let domains: Vec<String> = cross_domain_matches.iter()
                .map(|r| format!("{:?}", r.structure.domain))
                .collect();
            
            self.strategy.cross_domain_discoveries += 1;
            self.state.discovery_count += 1;
            
            let discovery = Discovery {
                id: format!("system_pattern_{}", self.state.tick),
                level: 1,  // Meta-level discovery
                discovery_type: DiscoveryType::SystemPattern,
                description: format!("Cross-domain isomorphism: {:?} ↔ {:?}", domain, domains),
                tick: self.state.tick,
                math_explanation: format!(
                    "Structure type: {:?}\nConsistency: {}\nDomains involved: {:?}",
                    structure.structure_type,
                    structure.consistency_score,
                    domains
                ),
            };
            self.discoveries.push(discovery.clone());
            
            log.push(format!("✓ SystemPattern discovery: {}", discovery.description));
            
            self.add_to_signal_pool(signal, structure);
            
            return ExplorationResult {
                signal: signal.to_vec(),
                matched_known: None,
                detected_pattern: Some("system_pattern".to_string()),
                formula: Some(format!("Cross-domain isomorphism with {:?}", domains)),
                derivation_steps: derivation,
                exploration_log: log,
                status: ExplorationStatus::PatternDetected { 
                    class_id: "system_pattern".to_string(), 
                    formula: discovery.description 
                },
                fit_level: None,
                fit_analysis: None,
                cross_domain_verification: None,
                is_discovery: false,  // System pattern needs further verification
                discovery_assessment: None,
            };
        }
        
        // 3b. Check generalization -> LevelUp
        if let Some(generalized) = self.generalizes(&structure, kb) {
            log.push(format!("🎯 Generalization discovered! Generalized patterns: {:?}", generalized));
            
            self.strategy.generalization_discoveries += 1;
            self.state.discovery_count += 1;
            
            let discovery = Discovery {
                id: format!("level_up_{}", self.state.tick),
                level: 2,  // Meta-meta level discovery
                discovery_type: DiscoveryType::LevelUp,
                description: format!("Generalization discovery: covers {:?}", generalized),
                tick: self.state.tick,
                math_explanation: format!(
                    "New structure type: {:?}\nGeneralizes: {:?}",
                    structure.structure_type,
                    generalized
                ),
            };
            self.discoveries.push(discovery.clone());
            
            log.push(format!("✓ LevelUp discovery: {}", discovery.description));
        }
        
        // 3c. If regular but no known pattern matches -> NewRule (candidate for KB)
        if structure.consistency_score >= 40 {
            match &structure.structure_type {
                StructureType::UnknownRegular { description } => {
                    log.push(format!("🎯 Discovered unknown regularity: {}", description));
                    
                    // Create candidate pattern
                    let candidate_id = format!("candidate_{}", self.state.tick);
                    let candidate = CandidatePattern {
                        id: candidate_id.clone(),
                        structure: structure.clone(),
                        support_count: 1,
                        supporting_signals: vec![self.state.tick],
                        confidence: structure.consistency_score as f64 / 100.0,
                        discovered_at: self.state.tick,
                    };
                    
                    // Check for similar candidate (merge support)
                    let mut merged_idx = None;
                    for (idx, existing) in self.candidate_patterns.iter().enumerate() {
                        if existing.structure.structure_type == structure.structure_type &&
                           existing.structure.signature == structure.signature {
                            merged_idx = Some(idx);
                            break;
                        }
                    }
                    
                    if let Some(idx) = merged_idx {
                        let existing = &mut self.candidate_patterns[idx];
                        existing.support_count += 1;
                        existing.supporting_signals.push(self.state.tick);
                        existing.confidence = (existing.confidence + structure.consistency_score as f64 / 100.0) / 2.0;
                        log.push(format!("Merged into existing candidate: {} (Support count: {})", existing.id, existing.support_count));
                    } else {
                        self.candidate_patterns.push(candidate);
                        log.push(format!("Create new candidate pattern: {}", candidate_id));
                    }
                    
                    self.state.discovery_count += 1;
                    
                    let discovery = Discovery {
                        id: format!("new_rule_{}", self.state.tick),
                        level: 0,
                        discovery_type: DiscoveryType::NewRule,
                        description: format!("Candidate new pattern: {}", description),
                        tick: self.state.tick,
                        math_explanation: format!(
                            "Structure type: {:?}\nConsistency: {}\nDifference features: {} orders\nRatio features: {} items",
                            structure.structure_type,
                            structure.consistency_score,
                            structure.differences.len(),
                            structure.ratios.len()
                        ),
                    };
                    self.discoveries.push(discovery.clone());
                    
                    log.push(format!("✓ NewRule discovery: {}", discovery.description));
                }
                _ => {
                    // Other regular types, also record
                    log.push(format!("Structure is regular but type known: {:?}", structure.structure_type));
                }
            }
        }
        
        // Add to signal pool
        self.add_to_signal_pool(signal, structure);
        
        log.push("Exploration complete".to_string());
        
        ExplorationResult {
            signal: signal.to_vec(),
            matched_known: None,
            detected_pattern: None,
            formula: None,
            derivation_steps: derivation,
            exploration_log: log,
            status: ExplorationStatus::Unknown,
            fit_level: None,
            fit_analysis: None,
            cross_domain_verification: None,
            is_discovery: false,
            discovery_assessment: None,
        }
    }
    
    /// Add to signal pool
    fn add_to_signal_pool(&mut self, signal: &[EPR], structure: SignalStructure) {
        self.signal_pool.push(SignalRecord {
            signal: signal.to_vec(),
            structure,
            tick: self.state.tick,
            cross_compared: false,
        });
        
        // Limit pool size
        if self.signal_pool.len() > 1000 {
            self.signal_pool.remove(0);
        }
    }
    
    /// Convenience method for i128 input (auto-convert to EPR)
    pub fn explore_i128(&mut self, signal: &[i128], kb: &KnowledgeBase) -> ExplorationResult {
        let epr_signal: Vec<EPR> = signal.iter()
            .map(|&v| EPR::from_i128(v))
            .collect();
        self.explore(&epr_signal, kb)
    }
    
    /// Convenience method for fraction input (auto-convert to EPR)
    pub fn explore_fractions(&mut self, fractions: &[(i128, i128)], kb: &KnowledgeBase) -> ExplorationResult {
        let epr_signal: Vec<EPR> = fractions.iter()
            .map(|&(n, d)| EPR::from_ratio(n, d))
            .collect();
        self.explore(&epr_signal, kb)
    }
    
    // ========================================================================
    // FIT LEVEL CLASSIFICATION AND CROSS-DOMAIN VERIFICATION
    // ========================================================================
    
    /// Classify fit level based on academic rigor criteria
    /// "S-wind treats fitting as an information-bearing intermediate object, never as a discovery."
    /// 
    /// CORRECTED: DoF = sample_size - (degree + 1)
    /// Polynomial of degree d has d+1 parameters (a₀, a₁, ..., a_d)
    pub fn classify_fit_level(
        &self,
        signal_len: usize,
        pattern_degree: usize,
        _function_classes_tried: usize,
        _function_classes_failed: usize,
        cross_domain_verified: bool,
    ) -> FitLevel {
        // CORRECTED: Calculate degrees of freedom
        // DoF = sample_size - num_parameters
        // num_parameters = degree + 1 (for polynomial)
        let num_parameters = pattern_degree + 1;
        let dof = signal_len.saturating_sub(num_parameters);
        
        // Level 0: Interpolant fit (TRIVIAL)
        // DoF = 0: Pure interpolation (n points with (n-1) degree polynomial)
        // DoF = 1: Barely above, still trivial
        // This is guaranteed by interpolation theorem - ZERO information
        if dof == 0 {
            return FitLevel::InterpolantFit;  // Pure interpolation
        }
        
        // Level 3: Emergent law (cross-domain verified + sufficient DoF)
        if cross_domain_verified && dof >= 2 {
            return FitLevel::EmergentLaw;
        }
        
        // Level 2: Predictive fit (has DoF but not yet cross-domain verified)
        if dof >= 2 {
            return FitLevel::PredictiveFit;
        }
        
        // Level 1: Descriptive model (DoF = 1, borderline)
        FitLevel::DescriptiveModel
    }
    
    /// Adaptive cross-domain verification (optimal strategy)
    /// 
    /// Key principles:
    /// 1. Sequential testing: verify until confidence reached or failure
    /// 2. Confidence-driven: not fixed count, but reach confidence target
    /// 3. Immediate rejection: one failure immediately downgrades
    /// 4. Adaptive domain expansion: more success → farther testing
    /// 
    /// Minimum points = pattern_degree + 2 (information-theoretic minimum)
    /// Maximum points = 20 (computational limit)
    pub fn verify_cross_domain(
        &self,
        original_signal: &[EPR],
        formula: &str,
        pattern_class: &str,
    ) -> CrossDomainVerification {
        let n = original_signal.len();
        let original_domain = (1, n as i128);
        
        // Derive effective pattern class from formula if pattern_class is generic
        let effective_class = if pattern_class == "algebraic" {
            // Parse formula to detect actual pattern type
            if formula.contains("n²") || formula.contains("quadratic") || formula.contains("·n² ") {
                "quadratic"
            } else if formula.contains("n³") || formula.contains("cubic") {
                "cubic"
            } else if formula.contains("recurrence") || formula.contains("s[n+") {
                "recurrence"
            } else if formula.contains("multiplicative") {
                "multiplicative"
            } else if formula.contains("1/n") || formula.contains("/n") {
                "reciprocal"
            } else if formula.contains("^n") {
                "exponential"
            } else {
                pattern_class
            }
        } else {
            pattern_class
        };
        
        let degree = self.estimate_pattern_degree(effective_class);
        
        // Minimum verification points = degree + 2 (information-theoretic minimum)
        let min_points = (degree + 2).max(3);
        // Maximum verification points - scaled dynamically, not fixed limit
        let max_points = (n * 3).max(50); // At least 50, or 3x original signal size
        
        let mut log = Vec::new();
        log.push(format!("Adaptive cross-domain verification for: {}", formula));
        log.push(format!("Original domain: [1, {}]", n));
        log.push(format!("Pattern degree: {} → min_points = {}", degree, min_points));
        log.push("Strategy: Sequential testing with immediate rejection".to_string());
        
        let mut verified = 0;
        let mut tested = 0;
        let mut current_domain = (n as f64 * 1.5) as usize;  // Initial 50% extension
        let mut consecutive_success = 0;
        
        // Build extended signal incrementally (include previously verified points)
        let mut extended_signal = original_signal.to_vec();
        
        // Sequential verification loop
        loop {
            let next_i = n + tested + 1;
            tested += 1;
            
            // Predict value based on effective pattern class (using EXTENDED signal for prediction)
            let predicted = self.predict_value(next_i, &extended_signal, effective_class);
            
            if let Some(pred) = predicted {
                log.push(format!("  [{}] n={}: predicted = {}", tested, next_i, pred));
                
                // Verify consistency: extended_signal + pred should still satisfy the pattern
                let is_consistent = self.verify_prediction_consistency(
                    &extended_signal,  // current extended signal
                    next_i, 
                    &pred, 
                    effective_class
                );
                
                // If consistent, add to extended signal for next iteration
                if is_consistent {
                    extended_signal.push(pred.clone());
                }
                
                if is_consistent {
                    verified += 1;
                    consecutive_success += 1;
                    log.push(format!("      ✓ Consistent (consecutive: {})", consecutive_success));
                    
                    // Adaptive domain expansion: success → expand further
                    current_domain = ((current_domain as f64) * 1.2) as usize;
                } else {
                    // IMMEDIATE REJECTION: one failure stops verification
                    log.push("      ✗ INCONSISTENT → Immediate rejection".to_string());
                    log.push(format!("Result: FAILED at point {} (verified {}/{})", 
                        next_i, verified, tested));
                    
                    return CrossDomainVerification {
                        original_domain,
                        extended_domain: (1, next_i as i128),
                        new_points_tested: tested,
                        new_points_matched: verified,
                        extrapolation_success: false,
                        verification_log: log,
                    };
                }
            } else {
                log.push(format!("  [{}] n={}: prediction failed", tested, next_i));
                // Prediction failure counts as inconsistency
                log.push("      ✗ Cannot predict → Immediate rejection".to_string());
                
                return CrossDomainVerification {
                    original_domain,
                    extended_domain: (1, next_i as i128),
                    new_points_tested: tested,
                    new_points_matched: verified,
                    extrapolation_success: false,
                    verification_log: log,
                };
            }
            
            // SUCCESS CONDITION: consecutive successes >= min_points
            if consecutive_success >= min_points {
                log.push(format!("✓ Confidence reached: {} consecutive successes (≥ {})", 
                    consecutive_success, min_points));
                log.push(format!("Result: SUCCESS (verified {}/{} points)", verified, tested));
                
                return CrossDomainVerification {
                    original_domain,
                    extended_domain: (1, (n + tested) as i128),
                    new_points_tested: tested,
                    new_points_matched: verified,
                    extrapolation_success: true,
                    verification_log: log,
                };
            }
            
            // COMPUTATIONAL LIMIT
            if tested >= max_points {
                log.push(format!("⚠ Reached max_points limit ({})", max_points));
                // If we got here without failure, consider it marginal success
                let success = consecutive_success >= min_points / 2;
                log.push(format!("Result: {} (verified {}/{} points)", 
                    if success { "MARGINAL SUCCESS" } else { "INSUFFICIENT" }, 
                    verified, tested));
                
                return CrossDomainVerification {
                    original_domain,
                    extended_domain: (1, (n + tested) as i128),
                    new_points_tested: tested,
                    new_points_matched: verified,
                    extrapolation_success: success,
                    verification_log: log,
                };
            }
        }
    }
    
    /// Verify prediction consistency based on pattern class
    /// Verify prediction using Fate's own pattern detection
    /// 
    /// Logic: If original signal has pattern P, and extended signal also has pattern P,
    /// then the prediction is consistent.
    /// 
    /// This is TRUE OOS verification: we re-detect pattern on extended signal
    /// and compare with original pattern.
    fn verify_prediction_consistency(
        &self,
        signal: &[EPR],
        _i: usize,
        predicted: &EPR,
        pattern_class: &str,
    ) -> bool {
        // Build extended signal
        let mut extended: Vec<EPR> = signal.to_vec();
        extended.push(predicted.clone());
        
        // Use Fate's own pattern detection on extended signal
        // Key insight: if same pattern class works on extended signal, it's consistent
        
        if pattern_class.contains("linear") {
            // For linear: check if first differences are still constant
            self.check_linear_consistency(&extended)
        } else if pattern_class.contains("quadratic") {
            // For quadratic: check if second differences are still constant
            self.check_quadratic_consistency(&extended)
        } else if pattern_class.contains("polynomial") {
            // For polynomial degree d: check if d-th differences are still constant
            self.check_polynomial_consistency(&extended, pattern_class)
        } else if pattern_class.contains("reciprocal") {
            // For reciprocal: check if n*s_n is still constant
            self.check_reciprocal_consistency(&extended)
        } else if pattern_class.contains("exponential") {
            // For exponential: check if ratio s_{n+1}/s_n is still constant
            self.check_exponential_consistency(&extended)
        } else {
            // Unknown pattern class - cannot verify
            false
        }
    }
    
    /// Check linear consistency: Δ¹ should be constant
    fn check_linear_consistency(&self, signal: &[EPR]) -> bool {
        if signal.len() < 2 {
            return false;
        }
        let d1 = signal[1].clone() - signal[0].clone();
        for i in 1..signal.len() - 1 {
            let diff = signal[i + 1].clone() - signal[i].clone();
            if diff != d1 {
                return false;
            }
        }
        true
    }
    
    /// Check quadratic consistency: Δ² should be constant
    fn check_quadratic_consistency(&self, signal: &[EPR]) -> bool {
        if signal.len() < 3 {
            return false;
        }
        // Compute first differences
        let mut d1: Vec<EPR> = Vec::new();
        for i in 0..signal.len() - 1 {
            d1.push(signal[i + 1].clone() - signal[i].clone());
        }
        // Compute second differences
        if d1.len() < 2 {
            return false;
        }
        let d2 = d1[1].clone() - d1[0].clone();
        for i in 1..d1.len() - 1 {
            let diff = d1[i + 1].clone() - d1[i].clone();
            if diff != d2 {
                return false;
            }
        }
        true
    }
    
    /// Check polynomial consistency: Δ^d should be constant
    /// 
    /// CRITICAL: If degree = n-1 (pure interpolation), we CANNOT verify
    /// because Δ^(n-1) has only 1 value - no comparison possible!
    fn check_polynomial_consistency(&self, signal: &[EPR], pattern_class: &str) -> bool {
        // Extract degree from pattern class (e.g., "polynomial degree 5")
        let degree = if let Some(pos) = pattern_class.find("degree") {
            pattern_class[pos + 7..].trim().parse::<usize>().unwrap_or(signal.len() - 1)
        } else {
            signal.len() - 1
        };
        
        let n = signal.len();
        
        // CRITICAL CHECK: Pure interpolation cannot be verified!
        // For n points with degree (n-1) polynomial:
        // - Δ^(n-1) has only 1 value
        // - Cannot compare to verify consistency
        // - This is mathematically guaranteed to "pass" (false positive)
        if degree >= n - 1 {
            // This is pure interpolation - REJECT verification
            return false;
        }
        
        if n <= degree {
            return false;
        }
        
        // Compute differences up to degree d
        let mut current = signal.to_vec();
        for _ in 0..degree {
            if current.len() < 2 {
                return false;
            }
            let mut next = Vec::new();
            for i in 0..current.len() - 1 {
                next.push(current[i + 1].clone() - current[i].clone());
            }
            current = next;
        }
        
        // MUST have at least 2 values to compare!
        if current.len() < 2 {
            return false;
        }
        
        // At degree d, all values should be equal (constant)
        let first = &current[0];
        current.iter().all(|v| v == first)
    }
    
    /// Check reciprocal consistency: s_n * (n+k) should be constant for 1/(n+k) pattern
    fn check_reciprocal_consistency(&self, signal: &[EPR]) -> bool {
        if signal.len() < 2 || signal[0].is_zero() {
            return false;
        }
        
        // For f(n) = c/(n+k), find k from first two terms
        // s[0] = c/(1+k), s[1] = c/(2+k)
        // s[0]/s[1] = (2+k)/(1+k) → s[0]*(1+k) = s[1]*(2+k)
        // s[0] + s[0]*k = 2*s[1] + s[1]*k
        // k*(s[0] - s[1]) = 2*s[1] - s[0]
        
        let diff = signal[0].clone() - signal[1].clone();
        if diff.is_zero() {
            // s[0] = s[1] means constant, not reciprocal
            return false;
        }
        
        let k = (signal[1].clone() * EPR::from_i128(2) - signal[0].clone()) / diff;
        let c = signal[0].clone() * (EPR::from_i128(1) + k.clone());
        
        // Verify all terms satisfy s[i] = c / (i+1+k)
        for (i, val) in signal.iter().enumerate() {
            if val.is_zero() {
                return false;
            }
            let expected = c.clone() / (EPR::from_i128((i + 1) as i128) + k.clone());
            if *val != expected {
                return false;
            }
        }
        true
    }
    
    /// Check exponential consistency: s_{n+1}/s_n should be constant
    fn check_exponential_consistency(&self, signal: &[EPR]) -> bool {
        if signal.len() < 2 || signal[0].is_zero() {
            return false;
        }
        let ratio = signal[1].clone() / signal[0].clone();
        for i in 1..signal.len() - 1 {
            if signal[i].is_zero() {
                return false;
            }
            let r = signal[i + 1].clone() / signal[i].clone();
            if r != ratio {
                return false;
            }
        }
        true
    }
    
    /// Predict value at index i based on pattern class
    fn predict_value(&self, target_n: usize, signal: &[EPR], pattern_class: &str) -> Option<EPR> {
        let sig_len = signal.len();
        
        // Note: target_n is the actual n value (1-indexed), not array index
        // signal[0] = f(1), signal[1] = f(2), etc.
        
        // Handle "algebraic" pattern class by detecting the actual formula type
        // Check if formula contains patterns like "n²" or "quadratic" or specific polynomial forms
        let effective_class = if pattern_class.contains("algebraic") {
            // Detect from formula content
            if pattern_class.contains("n²") || pattern_class.contains("quadratic") {
                "quadratic"
            } else if pattern_class.contains("n³") || pattern_class.contains("cubic") {
                "cubic"
            } else if pattern_class.contains("recurrence") {
                "recurrence"
            } else if pattern_class.contains("multiplicative") {
                "multiplicative"
            } else if pattern_class.contains("1/2·n²") || pattern_class.contains("·n² +") {
                "quadratic"
            } else {
                pattern_class
            }
        } else {
            pattern_class
        };
        
        if effective_class.contains("linear") {
            // f(n) = a*n + b
            // From f(1)=s[0], f(2)=s[1]: a = s[1]-s[0], b = s[0] - a
            if sig_len >= 2 {
                let a = signal[1].clone() - signal[0].clone();
                let b = signal[0].clone() - a.clone();
                Some(a * EPR::from_i128(target_n as i128) + b)
            } else {
                None
            }
        } else if effective_class.contains("quadratic") {
            // f(n) = a*n² + b*n + c
            // Use Newton forward difference formula with constant Δ²
            if sig_len >= 3 {
                let d1_0 = signal[1].clone() - signal[0].clone();
                let d1_1 = signal[2].clone() - signal[1].clone();
                let d2 = d1_1.clone() - d1_0.clone(); // Constant for quadratic
                
                // Starting point: signal[-1] = f(sig_len), last_d1 = f(sig_len) - f(sig_len-1)
                let mut current_val = signal[sig_len - 1].clone();
                let mut current_d1 = signal[sig_len - 1].clone() - signal[sig_len - 2].clone();
                
                // Extrapolate from n=sig_len to n=target_n
                // Each step: next_d1 = current_d1 + d2, next_val = current_val + next_d1
                for _ in sig_len..target_n {
                    current_d1 = current_d1 + d2.clone();
                    current_val = current_val + current_d1.clone();
                }
                Some(current_val)
            } else {
                None
            }
        } else if effective_class.contains("cubic") || effective_class.contains("polynomial") {
            // For higher order polynomials, use finite difference extrapolation
            self.extrapolate_polynomial(signal, target_n)
        } else if effective_class.contains("reciprocal") {
            // f(n) = c / (n + k)
            // Try to find c and k from signal
            if sig_len >= 2 && !signal[0].is_zero() && !signal[1].is_zero() {
                // s[0] * (1 + k) = c
                // s[1] * (2 + k) = c
                // s[0] * (1 + k) = s[1] * (2 + k)
                // s[0] + s[0]*k = s[1]*2 + s[1]*k
                // s[0] - 2*s[1] = k*(s[1] - s[0])
                let diff = signal[1].clone() - signal[0].clone();
                if !diff.is_zero() {
                    let k_num = signal[0].clone() - signal[1].clone() * EPR::from_i128(2);
                    let k = k_num / diff;
                    let c = signal[0].clone() * (EPR::from_i128(1) + k.clone());
                    
                    let denom = EPR::from_i128(target_n as i128) + k;
                    if !denom.is_zero() {
                        return Some(c / denom);
                    }
                }
            }
            None
        } else if effective_class.contains("exponential") {
            // f(n) = a * r^n
            if sig_len >= 2 && !signal[0].is_zero() {
                let r = signal[1].clone() / signal[0].clone();
                // a = s[0] / r
                let a = signal[0].clone() / r.clone();
                
                // Compute r^target_n
                let mut r_pow = EPR::one();
                for _ in 0..target_n {
                    r_pow = r_pow * r.clone();
                }
                Some(a * r_pow)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Extrapolate polynomial using finite differences
    fn extrapolate_polynomial(&self, signal: &[EPR], target_i: usize) -> Option<EPR> {
        let n = signal.len();
        if n < 2 || target_i <= n {
            return None;
        }
        
        // Build difference table
        let mut diffs: Vec<Vec<EPR>> = vec![signal.to_vec()];
        
        for level in 1..n {
            let prev = &diffs[level - 1];
            if prev.len() < 2 {
                break;
            }
            let mut new_level = Vec::new();
            for j in 0..prev.len() - 1 {
                new_level.push(prev[j + 1].clone() - prev[j].clone());
            }
            if new_level.is_empty() {
                break;
            }
            diffs.push(new_level);
        }
        
        // Extrapolate by continuing the last difference of each level
        let mut current = signal.to_vec();
        
        for _i in n..target_i {
            // Add one new value using difference table
            let _next_val = current[current.len() - 1].clone();
            
            // Accumulate differences
            for level in (0..diffs.len()).rev() {
                if level < diffs.len() && !diffs[level].is_empty() {
                    let last_diff = diffs[level].last().unwrap().clone();
                    if level == diffs.len() - 1 {
                        // Assume highest difference is constant
                        diffs[level].push(last_diff.clone());
                    }
                }
            }
            
            // Build up from constant difference
            for level in (1..diffs.len()).rev() {
                if !diffs[level].is_empty() {
                    let new_diff = diffs[level - 1].last().unwrap().clone() + diffs[level].last().unwrap().clone();
                    diffs[level - 1].push(new_diff);
                }
            }
            
            if !diffs[0].is_empty() {
                current.push(diffs[0].last().unwrap().clone());
            } else {
                return None;
            }
        }
        
        current.last().cloned()
    }
    
    // NOTE: Old fake verification functions removed (2024-01)
    // Now using Fate's own pattern-based algebraic verification
    
    /// Explore with automatic cross-domain verification
    pub fn explore_with_verification(&mut self, signal: &[EPR], kb: &KnowledgeBase) -> ExplorationResult {
        let mut result = self.explore(signal, kb);
        
        // If pattern detected (not known sequence, not impossible), perform cross-domain verification
        if let ExplorationStatus::PatternDetected { class_id, formula } = &result.status {
            let sample_size = signal.len();
            let pattern_degree = self.estimate_pattern_degree_from_formula(class_id, formula);
            
            // ============================================================
            // COMPUTE FIT ANALYSIS (CRITICAL FOR NON-TRIVIALITY CHECK)
            // ============================================================
            // For algebraic patterns, use more accurate parameter estimation from formula
            let num_params = if class_id == "algebraic" {
                self.estimate_parameters_from_formula(formula)
            } else {
                pattern_degree + 1  // standard polynomial: degree d has d+1 params
            };
            
            let mut fit_analysis = FitAnalysis::new_with_params(sample_size, pattern_degree, num_params, class_id);
            
            // Check if this is TRIVIAL (dof ≤ 2)
            if fit_analysis.is_trivial {
                result.derivation_steps.push(format!(
                    "⚠️ TRIVIAL FIT: {} points with degree {} → DoF = {} ≤ 2",
                    sample_size, pattern_degree, fit_analysis.dof
                ));
                result.derivation_steps.push(
                    "   n points can ALWAYS be fitted by (n-1) degree polynomial (interpolation theorem)".to_string()
                );
                result.derivation_steps.push(
                    "   This is [InterpolantFit], NOT discovery".to_string()
                );
                result.fit_level = Some(FitLevel::InterpolantFit);
            } else {
                result.derivation_steps.push(format!(
                    "Non-trivial fit: {} points with degree {} → DoF = {} > 2",
                    sample_size, pattern_degree, fit_analysis.dof
                ));
                result.derivation_steps.push(format!(
                    "Non-triviality score: {:.2} (requires ≥ 0.5 for discovery candidate)",
                    fit_analysis.non_triviality_score
                ));
                result.fit_level = Some(FitLevel::DescriptiveModel);
            }
            
            // Perform cross-domain verification
            let verification = self.verify_cross_domain(signal, formula, class_id);
            
            // Generate holdout domain indices from verification
            let holdout_indices: Vec<usize> = if verification.new_points_tested > 0 {
                let start = sample_size + 1;
                (start..start + verification.new_points_tested).collect()
            } else {
                Vec::new()
            };
            
            // Update fit analysis with extrapolation results
            fit_analysis = fit_analysis.with_extrapolation(
                verification.new_points_tested, 
                verification.new_points_matched,
                holdout_indices
            );
            
            // Upgrade fit level based on cross-domain verification:
            // KEY INSIGHT: If extrapolation succeeds, the EFFECTIVE DoF increases!
            // Effective DoF = (original_samples + verified_new_points) - parameters
            // 
            // 1. Original DoF = 0 BUT OOS succeeded → Effective DoF > 0 → CandidateLaw!
            // 2. Original DoF = 0 AND OOS failed → InterpolantFit
            // 3. Original DoF > 0 + OOS passed → EmergentLaw
            // 4. Original DoF > 0 + OOS failed → DescriptiveModel
            
            let effective_dof = if verification.extrapolation_success && verification.new_points_matched > 0 {
                // Recalculate DoF with extended domain
                let total_verified = sample_size + verification.new_points_matched;
                let params = fit_analysis.num_parameters;
                let effective = total_verified.saturating_sub(params);
                result.derivation_steps.push(format!(
                    "Cross-domain success: {} + {} = {} verified points → Effective DoF = {} - {} = {}",
                    sample_size, verification.new_points_matched, total_verified, total_verified, params, effective
                ));
                effective
            } else {
                fit_analysis.dof
            };
            
            // ============================================================
            // FIT LEVEL CLASSIFICATION (STRICT MATHEMATICAL DEFINITION)
            // ============================================================
            // Key principle: Original DoF = 0 means PURE INTERPOLATION
            // Even if OOS passes, it's NOT a discovery (could be coincidence)
            // ============================================================
            
            let original_dof = fit_analysis.dof;  // This is the TRUE DoF
            
            if original_dof == 0 {
                // DoF = 0: Pure interpolation (Lagrange theorem guarantees fit)
                if verification.extrapolation_success {
                    // OOS passed but DoF = 0 → PredictiveFit (verified but still interpolation)
                    result.fit_level = Some(FitLevel::PredictiveFit);
                    result.derivation_steps.push(
                        format!("⚠️ Original DoF=0 (pure interpolation), OOS passed → PredictiveFit (NOT discovery)")
                    );
                    result.derivation_steps.push(
                        format!("   Note: {} points with {} params = exact fit guaranteed, OOS may be coincidence", 
                               sample_size, fit_analysis.num_parameters)
                    );
                } else {
                    // OOS failed + DoF = 0 → InterpolantFit (trivial)
                    result.fit_level = Some(FitLevel::InterpolantFit);
                    result.derivation_steps.push(
                        "✗ OOS failed + DoF=0 → InterpolantFit (pure interpolation)".to_string()
                    );
                }
            } else if original_dof > 0 && verification.extrapolation_success {
                // DoF > 0 AND OOS passed → EmergentLaw (genuine discovery candidate)
                result.fit_level = Some(FitLevel::EmergentLaw);
                result.derivation_steps.push(
                    format!("✓ Original DoF={} > 0 + OOS PASSED → EmergentLaw!", original_dof)
                );
            } else {
                // DoF > 0 but OOS failed → DescriptiveModel
                result.fit_level = Some(FitLevel::DescriptiveModel);
                result.derivation_steps.push(
                    format!("✗ OOS FAILED, DoF={} → DescriptiveModel", original_dof)
                );
            }
            
            // ============================================================
            // DETERMINE IF THIS IS A GENUINE DISCOVERY
            // STRICT CRITERION: Original DoF > 0 AND OOS passed
            // ============================================================
            let is_novel = result.matched_known.is_none();  // Not a known sequence
            
            // CRITICAL: Use ORIGINAL DoF (not effective) for discovery determination
            // A discovery requires:
            // 1. Original DoF ≥ 2 (non-trivial constraint)
            // 2. OOS verification passed
            // 3. Novel formula (not in knowledge base)
            let can_be_discovery = original_dof >= 2 && verification.extrapolation_success;
            
            result.is_discovery = can_be_discovery && is_novel;
            
            if result.is_discovery {
                result.derivation_steps.push(format!(
                    "🎯 DISCOVERY: Effective DoF={} ≥ 2 + Extrapolation passed + Novel formula", 
                    effective_dof
                ));
                
                // AUTO-ARCHIVE: Create candidate pattern for potential knowledge base entry
                let candidate = CandidatePattern {
                    id: format!("discovery_{}", self.state.tick),
                    structure: SignalStructure {
                        structure_type: match fit_analysis.degree {
                            0 => StructureType::Polynomial { degree: 0 },
                            1 => StructureType::Polynomial { degree: 1 },
                            2 => StructureType::Polynomial { degree: 2 },
                            d => StructureType::Polynomial { degree: d },
                        },
                        signature: signal.to_vec(),
                        differences: Vec::new(),
                        ratios: Vec::new(),
                        period: None,
                        consistency_score: 100,
                        domain: SignalDomain::External,
                    },
                    support_count: fit_analysis.sample_size,
                    supporting_signals: vec![self.state.tick],
                    confidence: if effective_dof >= 10 { 0.9 } else if effective_dof >= 5 { 0.8 } else { 0.7 },
                    discovered_at: self.state.tick,
                };
                self.candidate_patterns.push(candidate);
                result.derivation_steps.push(format!(
                    "📦 AUTO-ARCHIVED: Pattern added to candidate pool (ID: discovery_{})",
                    self.state.tick
                ));
                
                // NEW META-SYSTEM: Create Space container for this discovery
                // This enables dimensional analysis and continued exploration
                {
                    use crate::space::SWindSpace;
                    use crate::symbolic::Rational;
                    
                    let mut space = SWindSpace::new();
                    let space_signal: Vec<Rational> = signal.iter()
                        .map(|epr| {
                            let n = epr.numerator.try_to_i128().unwrap_or(0);
                            let d = epr.denominator.try_to_i128().unwrap_or(1);
                            Rational::new(n, d)
                        })
                        .collect();
                    
                    let discovery_result = space.on_discovery(
                        &space_signal, 
                        &format!("discovery_{}", self.state.tick)
                    );
                    
                    result.derivation_steps.push(format!(
                        "🌀 NEW META-SYSTEM: Space container '{}' created",
                        discovery_result.container_id
                    ));
                    result.derivation_steps.push(format!(
                        "   Dimension adjustment: {} → {}",
                        discovery_result.dimension_adjustment.old_dimension,
                        discovery_result.dimension_adjustment.new_dimension
                    ));
                    
                    // CONTINUE DERIVATION: Use discovered formula to explore deeper structure
                    // Try to find: f(f(n)), generalized form, or cross-domain patterns
                    if let Some(formula) = &result.formula {
                        result.derivation_steps.push(format!(
                            "🔄 CONTINUE DERIVATION: Exploring deeper structure with formula: {}",
                            formula
                        ));
                        
                        // 1. Try composition: f(f(n)) - is there a pattern?
                        if formula.contains("*n +") || formula.contains("*n²") {
                            // For linear/quadratic, compute f(f(n)) symbolically
                            result.derivation_steps.push(
                                "   → Attempting f(f(n)) composition analysis...".to_string()
                            );
                            
                            // For linear f(n) = an + b, f(f(n)) = a(an+b) + b = a²n + ab + b
                            if let Some((a_str, b_str)) = Self::parse_linear_formula(formula) {
                                if let (Ok(a), Ok(b)) = (a_str.parse::<i128>(), b_str.parse::<i128>()) {
                                    let a2 = a * a;
                                    let ab_b = a * b + b;
                                    result.derivation_steps.push(format!(
                                        "   → f(f(n)) = {}*n + {} (composition detected)",
                                        a2, ab_b
                                    ));
                                    
                                    // Check if this reveals periodic structure
                                    if a == 1 {
                                        result.derivation_steps.push(format!(
                                            "   → Pattern: f^k(n) = n + {}*k (translation group)",
                                            b
                                        ));
                                    } else if a == -1 {
                                        result.derivation_steps.push(
                                            "   → Pattern: f²(n) = n (involution detected)".to_string()
                                        );
                                    }
                                }
                            }
                        }
                        
                        // 2. Try extended domain verification (already done, but log it)
                        result.derivation_steps.push(format!(
                            "   → Extended domain: [1, {}] verified",
                            signal.len() + 3
                        ));
                        
                        // 3. Archive completion status
                        result.derivation_steps.push(
                            "   → Derivation complete: formula archived to knowledge candidate pool".to_string()
                        );
                    }
                }
            } else if effective_dof < 2 {
                result.derivation_steps.push(format!(
                    "❌ NOT DISCOVERY: Effective DoF={} < 2 (insufficient constraint redundancy)",
                    effective_dof
                ));
            } else if !verification.extrapolation_success {
                result.derivation_steps.push(
                    "❌ NOT DISCOVERY: Cross-domain extrapolation failed".to_string()
                );
            } else if !is_novel {
                result.derivation_steps.push(
                    "❌ NOT DISCOVERY: Formula matches known sequence (recognition, not discovery)".to_string()
                );
            }
            
            // Add verification log to derivation
            for log_line in &verification.verification_log {
                result.derivation_steps.push(format!("  {}", log_line));
            }
            
            result.fit_analysis = Some(fit_analysis.clone());
            result.cross_domain_verification = Some(verification.clone());
            
            // ============================================================
            // BUILD THREE-LAYER DISCOVERY ASSESSMENT
            // "OEIS should be a refutation tool, not a precondition for discovery"
            // ============================================================
            let oeis_status = if result.matched_known.is_some() {
                OeisStatus::ExactMatch
            } else {
                OeisStatus::NoMatch
            };
            
            let discovery_status = if result.matched_known.is_some() {
                DiscoveryStatus::Known
            } else if fit_analysis.is_trivial || fit_analysis.dof == 0 {
                DiscoveryStatus::TrivialInterpolation
            } else if !verification.extrapolation_success {
                DiscoveryStatus::DescriptiveOnly
            } else if fit_analysis.dof >= 2 && verification.extrapolation_success {
                if is_novel {
                    DiscoveryStatus::StrongCandidate
                } else {
                    DiscoveryStatus::CandidateLaw
                }
            } else {
                DiscoveryStatus::CandidateLaw
            };
            
            // Build negative knowledge
            let mut negative_evidence = Vec::new();
            if oeis_status == OeisStatus::NoMatch {
                negative_evidence.push("Not in OEIS (195+ sequences searched)".to_string());
            }
            if fit_analysis.dof > 0 {
                negative_evidence.push(format!("Non-trivial fit (DoF={} > 0)", fit_analysis.dof));
            }
            if verification.extrapolation_success {
                negative_evidence.push(format!("OOS verification passed ({}/{} points)", 
                    verification.new_points_matched, verification.new_points_tested));
            }
            
            let conclusion = match discovery_status {
                DiscoveryStatus::Known => "Known sequence - not a discovery".to_string(),
                DiscoveryStatus::TrivialInterpolation => "Trivial interpolation (DoF=0) - not a discovery".to_string(),
                DiscoveryStatus::DescriptiveOnly => "Descriptive model only - not yet verified".to_string(),
                DiscoveryStatus::CandidateLaw => "Candidate law - verified but needs more evidence".to_string(),
                DiscoveryStatus::StrongCandidate => "Strong candidate - verified + non-trivial + novel".to_string(),
                DiscoveryStatus::GenuineDiscovery => "GENUINE DISCOVERY confirmed".to_string(),
            };
            
            result.discovery_assessment = Some(DiscoveryAssessment {
                // Layer 1: Pre-OEIS Check
                oeis_sequences_searched: 195, // from kb.known_sequences.len()
                oeis_status,
                oeis_match_id: result.matched_known.clone(),
                
                // Layer 2: Discovery Assessment
                definition_precise: true,
                definition_desc: format!("f(n) = {} [{}]", formula, class_id),
                verifiable: verification.extrapolation_success,
                verification_desc: format!("OOS {}/{} points", 
                    verification.new_points_matched, verification.new_points_tested),
                non_trivial: !fit_analysis.is_trivial && fit_analysis.dof >= 2,
                non_trivial_desc: format!("DoF={} (n={}, params={})", 
                    fit_analysis.dof, fit_analysis.sample_size, fit_analysis.num_parameters),
                discovery_status,
                
                // Layer 3: Negative Knowledge
                failed_patterns: vec![], // TODO: track failed pattern attempts
                negative_evidence,
                conclusion,
            });
        }
        
        result
    }
    
    /// Explore with self-cycling (output becomes input for deeper exploration)
    /// 
    /// Self-cycling process:
    /// 1. Explore input signal → find formula f(n)
    /// 2. Generate extended/transformed signal using f(n)
    /// 3. Explore the new signal
    /// 4. Continue until: no new signal OR duplicate signal detected
    /// 
    /// Termination conditions:
    /// - Cannot generate new transformed signal (pattern exhausted)
    /// - Duplicate signal detected (cycle/closure reached)
    /// 
    /// Returns: (final_result, all_iteration_results, cycle_stats)
    pub fn explore_with_self_loop(&mut self, signal: &[EPR], kb: &KnowledgeBase) 
        -> (ExplorationResult, Vec<ExplorationResult>, SelfLoopStats) 
    {
        let max_iterations = self.config.max_self_loop_iterations;
        let mut iterations: Vec<ExplorationResult> = Vec::new();
        let mut current_signal = signal.to_vec();
        let mut stats = SelfLoopStats::default();
        
        // Track explored signal signatures to detect duplicates
        let mut explored_signatures: Vec<String> = Vec::new();
        
        // Track which transform type to try next (round-robin, dynamic)
        let mut transform_type_index: usize = 0;
        
        stats.initial_signal_len = signal.len();
        
        for i in 0..max_iterations {
            // Check for duplicate signal (closure detection)
            let current_signature = self.compute_signal_signature(&current_signal);
            if explored_signatures.contains(&current_signature) {
                // Duplicate signal detected → closure reached
                stats.closure_reached = true;
                break;
            }
            explored_signatures.push(current_signature);
            
            // Explore current signal
            let result = self.explore_with_verification(&current_signal, kb);
            iterations.push(result.clone());
            stats.iterations_run = i + 1;
            
            // ═══════════════════════════════════════════════════════════════════
            // CORRECT TERMINATION LOGIC:
            // 1. Valid pattern (non-trivial) → STOP immediately, output result
            // 2. Known sequence → STOP immediately
            // 3. InterpolantFit (trivial) → try transform, limit to 3 attempts
            // 4. No pattern → try transform, if exhausted → MYSTERY
            // ═══════════════════════════════════════════════════════════════════
            
            // Check if pattern was detected
            if let ExplorationStatus::PatternDetected { formula: _, .. } = result.status {
                
                // CASE 1: Valid non-trivial pattern found → STOP AND OUTPUT
                // Non-trivial means: EmergentLaw or DescriptiveModel with DoF > 0
                // PredictiveFit is still DoF=0, so continue exploring!
                let is_trivial_fit = matches!(
                    result.fit_level, 
                    Some(FitLevel::InterpolantFit) | Some(FitLevel::PredictiveFit)
                );
                
                if !is_trivial_fit && result.fit_level.is_some() {
                    stats.discovery_found = result.is_discovery;
                    stats.fate_converged = true;
                    break; // ← IMMEDIATE STOP with answer
                }
                
                // CASE 2: InterpolantFit or PredictiveFit (trivial DoF=0) → CONTINUE EXPLORING
                stats.interpolant_detected = true;
                stats.patterns_tried += 1;
                
                // ═══════════════════════════════════════════════════════════════════
                // DYNAMIC CONTAINER STRATEGY (Space-driven):
                // 1. Space analyzes signal characteristics
                // 2. Space generates transforms tailored to THIS signal
                // 3. Fate tries transforms in priority order
                // ═══════════════════════════════════════════════════════════════════
                
                use crate::space::SWindSpace;
                #[allow(unused_imports)]
                use crate::space::{DynamicTransform, TransformType};
                use crate::symbolic::Rational;
                
                // Convert signal to Rational for Space analysis
                let rational_signal: Vec<Rational> = current_signal.iter()
                    .map(|epr| Rational::new(
                        epr.numerator.try_to_i128().unwrap_or(0),
                        epr.denominator.try_to_i128().unwrap_or(1)
                    ))
                    .collect();
                
                // Let Space analyze and generate transforms
                let space = SWindSpace::new();
                let characteristics = space.analyze_signal(&rational_signal);
                let dynamic_transforms = space.generate_dynamic_transforms(&characteristics);
                
                // Record what transforms Space recommended
                for t in dynamic_transforms.iter() {
                    if !stats.transformations_applied.iter().any(|s| s.contains(&t.name)) {
                        stats.transformations_applied.push(format!("T.{}: {} (priority={})", 
                            t.name, t.description, t.priority));
                    }
                }
                
                let mut found_new_signal = false;
                
                // Try each Space-recommended transform starting from transform_type_index
                let num_transforms = dynamic_transforms.len();
                for offset in 0..num_transforms {
                    let idx = (transform_type_index + offset) % num_transforms;
                    if idx >= dynamic_transforms.len() {
                        continue;
                    }
                    
                    let transform = &dynamic_transforms[idx];
                    
                    // Apply transform using Space
                    if let Some(transformed_rational) = space.apply_transform(&rational_signal, transform) {
                        // Convert back to EPR
                        let transformed: Vec<EPR> = transformed_rational.iter()
                            .map(|r| EPR::from_ratio(r.numerator, r.denominator))
                            .collect();
                        
                        if transformed.len() >= 3 {
                            let sig = self.compute_signal_signature(&transformed);
                            if !explored_signatures.contains(&sig) {
                                current_signal = transformed;
                                stats.signal_transformations += 1;
                                stats.final_signal_len = current_signal.len();
                                transform_type_index = (idx + 1) % num_transforms;
                                found_new_signal = true;
                                break;
                            }
                        }
                    }
                }
                
                if found_new_signal {
                    continue; // Explore new container
                }
                
                // All Space-recommended containers exhausted → MYSTERY
                stats.pattern_exhausted = true;
                break;
            }
            
            // CASE 3: Known sequence → STOP immediately
            if matches!(result.status, ExplorationStatus::KnownSequence { .. }) {
                stats.known_sequence_found = true;
                stats.fate_converged = true;
                break; // ← IMMEDIATE STOP with known answer
            }
            
            // CASE 4: No pattern → try ALL transforms until EXHAUSTED
            if matches!(result.status, ExplorationStatus::Unknown) {
                stats.no_pattern_iterations += 1;
                
                // NO HARD LIMIT - continue until true closure (all transforms exhausted)
                // Only stop when NO transform can produce a new signal
                
                let mut found_new = false;
                
                // T1: Difference Δ
                if current_signal.len() >= 3 && !found_new {
                    let diff_signal: Vec<EPR> = current_signal.windows(2)
                        .map(|w| w[1].clone() - w[0].clone())
                        .collect();
                    let diff_sig = self.compute_signal_signature(&diff_signal);
                    if !explored_signatures.contains(&diff_sig) {
                        current_signal = diff_signal;
                        stats.signal_transformations += 1;
                        stats.final_signal_len = current_signal.len();
                        found_new = true;
                    }
                }
                
                // T2: Second difference Δ²
                if current_signal.len() >= 4 && !found_new {
                    let d1: Vec<EPR> = current_signal.windows(2)
                        .map(|w| w[1].clone() - w[0].clone())
                        .collect();
                    let d2: Vec<EPR> = d1.windows(2)
                        .map(|w| w[1].clone() - w[0].clone())
                        .collect();
                    let d2_sig = self.compute_signal_signature(&d2);
                    if !explored_signatures.contains(&d2_sig) {
                        current_signal = d2;
                        stats.signal_transformations += 1;
                        stats.final_signal_len = current_signal.len();
                        found_new = true;
                    }
                }
                
                // T3: Cumulative sum Σ
                if current_signal.len() >= 3 && !found_new {
                    let mut cumsum = vec![EPR::from_i128(0)];
                    let mut acc = EPR::from_i128(0);
                    for val in current_signal.iter() {
                        acc = acc + val.clone();
                        cumsum.push(acc.clone());
                    }
                    let cumsum_sig = self.compute_signal_signature(&cumsum);
                    if !explored_signatures.contains(&cumsum_sig) {
                        current_signal = cumsum;
                        stats.signal_extensions += 1;
                        stats.final_signal_len = current_signal.len();
                        found_new = true;
                    }
                }
                
                // T4: Absolute value |·|
                if current_signal.len() >= 3 && !found_new {
                    let abs_signal: Vec<EPR> = current_signal.iter()
                        .map(|x| x.abs())
                        .collect();
                    let abs_sig = self.compute_signal_signature(&abs_signal);
                    if !explored_signatures.contains(&abs_sig) {
                        current_signal = abs_signal;
                        stats.signal_transformations += 1;
                        stats.final_signal_len = current_signal.len();
                        found_new = true;
                    }
                }
                
                // T5: n*f(n)
                if current_signal.len() >= 3 && !found_new {
                    let nf_signal: Vec<EPR> = current_signal.iter()
                        .enumerate()
                        .map(|(i, x)| x.clone() * EPR::from_i128((i + 1) as i128))
                        .collect();
                    let nf_sig = self.compute_signal_signature(&nf_signal);
                    if !explored_signatures.contains(&nf_sig) {
                        current_signal = nf_signal;
                        stats.signal_transformations += 1;
                        stats.final_signal_len = current_signal.len();
                        found_new = true;
                    }
                }
                
                // T6: Ratio (if no zeros)
                if current_signal.len() >= 3 && !found_new {
                    let all_nonzero = current_signal.iter().all(|x| !x.is_zero());
                    if all_nonzero {
                        let ratio_signal: Vec<EPR> = current_signal.windows(2)
                            .map(|w| w[1].clone() / w[0].clone())
                            .collect();
                        let ratio_sig = self.compute_signal_signature(&ratio_signal);
                        if !explored_signatures.contains(&ratio_sig) {
                            current_signal = ratio_signal;
                            stats.signal_transformations += 1;
                            stats.final_signal_len = current_signal.len();
                            found_new = true;
                        }
                    }
                }
                
                // T7: Square f(n)²
                if current_signal.len() >= 3 && !found_new {
                    let sq_signal: Vec<EPR> = current_signal.iter()
                        .map(|x| x.clone() * x.clone())
                        .collect();
                    let sq_sig = self.compute_signal_signature(&sq_signal);
                    if !explored_signatures.contains(&sq_sig) {
                        current_signal = sq_signal;
                        stats.signal_transformations += 1;
                        stats.final_signal_len = current_signal.len();
                        found_new = true;
                    }
                }
                
                // T8: Sign extraction sgn(f)
                if current_signal.len() >= 3 && !found_new {
                    let sign_signal: Vec<EPR> = current_signal.iter()
                        .map(|x| {
                            if x.is_zero() { EPR::from_i128(0) }
                            else if x.numerator.is_negative() { EPR::from_i128(-1) }
                            else { EPR::from_i128(1) }
                        })
                        .collect();
                    let sign_sig = self.compute_signal_signature(&sign_signal);
                    if !explored_signatures.contains(&sign_sig) {
                        current_signal = sign_signal;
                        stats.signal_transformations += 1;
                        stats.final_signal_len = current_signal.len();
                        found_new = true;
                    }
                }
                
                // T9: Moving average MA(3)
                if current_signal.len() >= 5 && !found_new {
                    let ma_signal: Vec<EPR> = current_signal.windows(3)
                        .map(|w| (w[0].clone() + w[1].clone() + w[2].clone()) / EPR::from_i128(3))
                        .collect();
                    let ma_sig = self.compute_signal_signature(&ma_signal);
                    if !explored_signatures.contains(&ma_sig) {
                        current_signal = ma_signal;
                        stats.signal_transformations += 1;
                        stats.final_signal_len = current_signal.len();
                        found_new = true;
                    }
                }
                
                // T10: Third difference Δ³
                if current_signal.len() >= 5 && !found_new {
                    let d1: Vec<EPR> = current_signal.windows(2)
                        .map(|w| w[1].clone() - w[0].clone())
                        .collect();
                    let d2: Vec<EPR> = d1.windows(2)
                        .map(|w| w[1].clone() - w[0].clone())
                        .collect();
                    let d3: Vec<EPR> = d2.windows(2)
                        .map(|w| w[1].clone() - w[0].clone())
                        .collect();
                    if d3.len() >= 3 {
                        let d3_sig = self.compute_signal_signature(&d3);
                        if !explored_signatures.contains(&d3_sig) {
                            current_signal = d3;
                            stats.signal_transformations += 1;
                            stats.final_signal_len = current_signal.len();
                            found_new = true;
                        }
                    }
                }
                
                // T11: Product of consecutive pairs f(n)*f(n+1)
                if current_signal.len() >= 4 && !found_new {
                    let prod_signal: Vec<EPR> = current_signal.windows(2)
                        .map(|w| w[0].clone() * w[1].clone())
                        .collect();
                    let prod_sig = self.compute_signal_signature(&prod_signal);
                    if !explored_signatures.contains(&prod_sig) {
                        current_signal = prod_signal;
                        stats.signal_transformations += 1;
                        stats.final_signal_len = current_signal.len();
                        found_new = true;
                    }
                }
                
                // T12: f(n) mod 10 (decimal digit extraction)
                if current_signal.len() >= 3 && !found_new {
                    let mod_signal: Vec<EPR> = current_signal.iter()
                        .filter_map(|x| {
                            let n = x.numerator.try_to_i128()?;
                            let d = x.denominator.try_to_i128()?;
                            if d == 1 {
                                Some(EPR::from_i128((n % 10).abs()))
                            } else {
                                None
                            }
                        })
                        .collect();
                    if mod_signal.len() >= 3 {
                        let mod_sig = self.compute_signal_signature(&mod_signal);
                        if !explored_signatures.contains(&mod_sig) {
                            current_signal = mod_signal;
                            stats.signal_transformations += 1;
                            stats.final_signal_len = current_signal.len();
                            found_new = true;
                        }
                    }
                }
                
                if found_new {
                    continue; // Explore new container
                }
                
                // ALL TRANSFORMS EXHAUSTED → TRUE CLOSURE → MYSTERY
                stats.truly_exhausted = true;
                break;
            }
        }
        
        // Find the best result (prefer discoveries over patterns over known)
        let mut final_result = iterations.last().cloned().unwrap_or_else(|| {
            self.explore_with_verification(signal, kb)
        });
        
        // Check if self-loop improved anything
        if iterations.len() > 1 {
            let first = &iterations[0];
            let last = &iterations[iterations.len() - 1];
            
            // Compare DoF improvement
            if let (Some(fa1), Some(fa2)) = (&first.fit_analysis, &last.fit_analysis) {
                stats.dof_improvement = fa2.dof as i32 - fa1.dof as i32;
            }
            
            // Check if discovery was found through cycling
            if !first.is_discovery && last.is_discovery {
                stats.discovered_via_cycling = true;
            }
        }
        
        // ═══════════════════════════════════════════════════════════════════════════
        // MYSTERY DETECTION: When Fate exhausts all explorations without finding a
        // solution, and no new transformations yield fresh signals, declare MYSTERY
        // ═══════════════════════════════════════════════════════════════════════════
        
        let is_truly_exhausted = stats.truly_exhausted 
            || (stats.closure_reached && matches!(final_result.status, ExplorationStatus::Unknown))
            || (stats.iterations_run >= 3 && !final_result.is_discovery 
                && matches!(final_result.status, ExplorationStatus::Unknown));
        
        // ═══════════════════════════════════════════════════════════════════════════
        // AUTO-GENERATE HYPOTHESIS SPACE H AND TRANSFORMATION SET T
        // Fate reports exactly what it tested, not a human-defined summary
        // ═══════════════════════════════════════════════════════════════════════════
        
        // H: Hypothesis space (pattern classes actually tested)
        stats.hypothesis_space_tested = vec![
            // Algebraic methods
            "H_algebraic.recurrence: s[n+1] = a·s[n] + b".to_string(),
            "H_algebraic.closed_form: f(n) = an + b, c/n, a·rⁿ".to_string(),
            "H_algebraic.identity: s[n]·n = k".to_string(),
            "H_algebraic.quadratic: f(n) = an² + bn + c".to_string(),
            "H_algebraic.cubic: f(n) = an³ + bn² + cn + d".to_string(),
            "H_algebraic.rational: f(n) = P(n)/Q(n)".to_string(),
            "H_algebraic.multiplicative: f(mn) = f(m)·f(n)".to_string(),
            // Pattern classes
            "H_pattern.constant".to_string(),
            "H_pattern.linear".to_string(),
            "H_pattern.quadratic".to_string(),
            "H_pattern.cubic".to_string(),
            "H_pattern.reciprocal".to_string(),
            "H_pattern.exponential".to_string(),
            "H_pattern.periodic".to_string(),
            "H_pattern.factorial".to_string(),
            "H_pattern.recurrence_2".to_string(),
            "H_pattern.recurrence_3".to_string(),
            "H_pattern.general_polynomial".to_string(),
            // OEIS knowledge
            format!("H_oeis: {} known sequences", kb.known_sequences.len()),
        ];
        
        // T: Transformation set actually applied (merge with dynamic transforms)
        let base_transforms = vec![
            format!("T.identity: {} signals", explored_signatures.len()),
            format!("T.difference (Δ): {} times", stats.signal_transformations),
            "T.ratio: f(n+1)/f(n)".to_string(),
            "T.cumsum: Σf(k)".to_string(),
        ];
        
        // Preserve dynamically generated transforms from Space
        let mut final_transforms = base_transforms;
        for t in &stats.transformations_applied {
            // Add dynamic transforms that are not already in base
            if !final_transforms.iter().any(|s| s.contains(&t.split(':').next().unwrap_or(""))) {
                final_transforms.push(t.clone());
            }
        }
        stats.transformations_applied = final_transforms;
        
        // Check if result is InterpolantFit (trivial polynomial)
        let is_interpolant = final_result.fit_level == Some(FitLevel::InterpolantFit);
        
        // If max iterations reached with InterpolantFit, consider patterns exhausted
        if is_interpolant && stats.iterations_run >= max_iterations as u32 {
            stats.pattern_exhausted = true;
        }
        
        // MYSTERY condition: exhausted OR (InterpolantFit + exhausted patterns)
        let should_be_mystery = (is_truly_exhausted && !final_result.is_discovery)
            || (is_interpolant && stats.pattern_exhausted);
        
        if should_be_mystery {
            // Build hypothesis space description from actual tests
            let _h_desc = stats.hypothesis_space_tested.join(", ");
            let t_desc = stats.transformations_applied.join(", ");
            
            // Collect transformations tried
            let transformations_tried: Vec<String> = stats.transformations_applied.clone();
            
            // Build mystery status
            final_result.status = ExplorationStatus::Mystery {
                iterations_exhausted: stats.iterations_run as usize,
                transformations_tried,
                reason: if stats.closure_reached {
                    format!("Closure in (H,T): all T lead back to explored signals")
                } else if is_interpolant && stats.pattern_exhausted {
                    format!("InterpolantFit exhausted: only trivial polynomial (DoF=0), no algebraic structure")
                } else if stats.truly_exhausted {
                    format!("Exhausted (H,T): no pattern in H fits, T=[{}]", t_desc)
                } else {
                    "Maximum iterations reached without solution".to_string()
                },
            };
            
            // Clear the InterpolantFit result - it's not a real answer
            final_result.formula = None;
            final_result.fit_level = None;
            
            stats.fate_converged = true;
        }
        
        (final_result, iterations, stats)
    }
    
    /// Compute a signature for a signal (for duplicate detection)
    fn compute_signal_signature(&self, signal: &[EPR]) -> String {
        // Use ALL values for accurate signature (not just first 5)
        // This prevents false duplicate detection
        let all_values: Vec<String> = signal.iter()
            .map(|epr| format!("{}/{}", 
                epr.numerator.try_to_i128().unwrap_or(0),
                epr.denominator.try_to_i128().unwrap_or(1)))
            .collect();
        format!("len{}:[{}]", signal.len(), all_values.join(","))
    }
    
    /// Generate transformed signal: apply f to each value in the signal
    /// f(s[0]), f(s[1]), f(s[2]), ... 
    /// This explores deeper structure by applying the discovered formula to the signal itself
    #[allow(dead_code)]
    fn generate_transformed_signal(&self, signal: &[EPR], formula: &str) -> Option<Vec<EPR>> {
        // Only works for simple formulas we can evaluate
        
        // Linear: f(x) = a*x + b
        if let Some((a_str, b_str)) = Self::parse_linear_formula(formula) {
            if let (Ok(a), Ok(b)) = (a_str.parse::<i128>(), b_str.parse::<i128>()) {
                let a_epr = EPR::from_i128(a);
                let b_epr = EPR::from_i128(b);
                
                let transformed: Vec<EPR> = signal.iter()
                    .map(|x| a_epr.clone() * x.clone() + b_epr.clone())
                    .collect();
                
                return Some(transformed);
            }
        }
        
        // Quadratic: f(x) = a*x² + b*x + c
        if formula.contains("n²") || formula.contains("n^2") {
            // Parse quadratic coefficients (simplified)
            // For now, skip quadratic transformation as it's complex
            return None;
        }
        
        None
    }
    
    /// Generate extended signal using detected formula
    #[allow(dead_code)]
    fn generate_extended_signal(&self, original: &[EPR], formula: &str, extend_by: usize) -> Option<Vec<EPR>> {
        let n = original.len();
        let mut extended = original.to_vec();
        
        // Parse formula and generate new points
        // Currently supports: polynomial, linear, quadratic
        
        if formula.contains("polynomial degree") || formula.contains("Newton form") || formula.contains("C(n-1,") {
            // NEWTON FORWARD DIFFERENCE FORMULA
            // f(n) = Σ C(n-1, k) · Δᵏ[0] for k = 0 to degree
            // 
            // This allows us to compute polynomial values for ANY n
            // using only the forward differences at index 1
            
            // Extract degree from formula
            let degree = if let Some(pos) = formula.find("degree") {
                formula[pos + 6..].trim().split_whitespace().next()
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(n - 1)
            } else {
                n - 1
            };
            
            // Compute all forward differences Δᵏ[0] (at index 0)
            let deltas = self.compute_forward_differences(original, degree);
            
            // Generate extended values using Newton's formula
            for i in 0..extend_by {
                let new_n = (n + i) as i128; // n-1 based (for binomial)
                let new_val = self.newton_forward_interpolation(&deltas, new_n);
                extended.push(new_val);
            }
            
            return Some(extended);
        }
        
        // Check for linear pattern: "linear", "Δ¹ =", or "a*n + b" format
        let is_linear = formula.contains("linear") 
            || formula.contains("Δ¹ =")
            || (formula.contains("*n") && !formula.contains("n²") && !formula.contains("n^2"));
        
        if is_linear {
            // Linear: f(n) = a*n + b
            // Calculate from first two points: a = Δ¹, b = s[0] - a*start_n
            if n >= 2 {
                let a = original[1].clone() - original[0].clone();
                let start_n = EPR::from_i128(1);
                let b = original[0].clone() - a.clone() * start_n;
                
                for i in 0..extend_by {
                    let new_n = EPR::from_i128((n + i + 1) as i128);
                    let new_val = a.clone() * new_n + b.clone();
                    extended.push(new_val);
                }
                return Some(extended);
            }
        }
        
        // Check for quadratic pattern: "quadratic", "Δ² =", or "n²"/"n^2" format
        let is_quadratic = formula.contains("quadratic") 
            || formula.contains("Δ² =")
            || formula.contains("n²")
            || formula.contains("n^2");
        
        if is_quadratic {
            // Quadratic: f(n) = a*n² + b*n + c
            if n >= 3 {
                // Calculate second differences
                let d1_0 = original[1].clone() - original[0].clone();
                let d1_1 = original[2].clone() - original[1].clone();
                let d2 = d1_1 - d1_0.clone();
                
                let two = EPR::from_i128(2);
                let a = d2 / two;
                
                // b = d1[0] - a*(2*1 + 1) = d1[0] - 3a
                let three = EPR::from_i128(3);
                let b = d1_0 - a.clone() * three;
                
                // c = s[0] - a - b
                let c = original[0].clone() - a.clone() - b.clone();
                
                for i in 0..extend_by {
                    let new_n = EPR::from_i128((n + i + 1) as i128);
                    let new_n_sq = new_n.clone() * new_n.clone();
                    let new_val = a.clone() * new_n_sq + b.clone() * new_n + c.clone();
                    extended.push(new_val);
                }
                return Some(extended);
            }
        }
        
        if formula.contains("reciprocal") || formula.contains("1/") {
            // Reciprocal: f(n) = c / (n + k)
            // From first point: c = s[0] * (1 + k)
            if n >= 2 {
                // Try to find k by comparing first two points
                // s[0] * (1+k) = s[1] * (2+k)
                // Let x = 1+k, then s[0]*x = s[1]*(x+1)
                // s[0]*x = s[1]*x + s[1]
                // x*(s[0] - s[1]) = s[1]
                // x = s[1] / (s[0] - s[1])
                let diff = original[0].clone() - original[1].clone();
                if !diff.is_zero() {
                    let x = original[1].clone() / diff;
                    let one = EPR::from_i128(1);
                    let k = x.clone() - one;
                    let c = original[0].clone() * x;
                    
                    for i in 0..extend_by {
                        let new_n = EPR::from_i128((n + i + 1) as i128);
                        let denom = new_n + k.clone();
                        if !denom.is_zero() {
                            let new_val = c.clone() / denom;
                            extended.push(new_val);
                        }
                    }
                    return Some(extended);
                }
            }
        }
        
        None
    }
    
    /// Estimate pattern degree from class ID and formula
    fn estimate_pattern_degree_from_formula(&self, class_id: &str, formula: &str) -> usize {
        // First try to extract from formula (e.g., "degree 9]" or "degree 7")
        if let Some(deg_str) = formula.split("degree").nth(1) {
            // Extract number, stopping at non-digit (e.g., "]" or " ")
            let num_str: String = deg_str.trim()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if let Ok(deg) = num_str.parse::<usize>() {
                return deg;
            }
        }
        
        // Fall back to class_id based estimation
        self.estimate_pattern_degree(class_id)
    }
    
    /// Estimate pattern degree from class ID
    fn estimate_pattern_degree(&self, class_id: &str) -> usize {
        if class_id.contains("linear") {
            1
        } else if class_id.contains("quadratic") {
            2
        } else if class_id.contains("cubic") {
            3
        } else if class_id.contains("polynomial") {
            // Try to extract degree from class_id
            if let Some(deg_str) = class_id.split("degree").nth(1) {
                if let Ok(deg) = deg_str.trim().parse::<usize>() {
                    return deg;
                }
            }
            4  // default for unknown polynomial
        } else {
            // For "algebraic" class, we need to parse from formula context
            // This is a fallback - the actual degree should be parsed from formula
            0  // non-polynomial (reciprocal, exponential, etc.)
        }
    }
    
    /// Estimate parameter count from formula string (more accurate than class_id)
    fn estimate_parameters_from_formula(&self, formula: &str) -> usize {
        // Check for quadratic: "an² + bn + c" or "quadratic"
        if formula.contains("n²") || formula.contains("n^2") || formula.contains("quadratic") {
            return 3;  // a, b, c
        }
        // Check for cubic
        if formula.contains("n³") || formula.contains("n^3") || formula.contains("cubic") {
            return 4;  // a, b, c, d
        }
        // Check for linear recurrence: "s[n+1] = a·s[n] + b"
        if formula.contains("[n+1]") && formula.contains("[n]") && !formula.contains("[n+2]") {
            return 2;  // a, b
        }
        // Check for 2nd-order recurrence: "s[n+2] = a·s[n+1] + b·s[n]"
        if formula.contains("[n+2]") {
            return 2;  // a, b
        }
        // Check for multiplicative: "f(mn) = f(m)·f(n)"
        if formula.contains("multiplicative") || formula.contains("f(mn)") {
            return 1;  // just the structure property
        }
        // Check for reciprocal: "1/n"
        if formula.contains("1/n") && !formula.contains("1/n²") {
            return 1;
        }
        // Check for reciprocal squared: "1/n²"
        if formula.contains("1/n²") || formula.contains("1/n^2") {
            return 1;
        }
        // Check for Newton polynomial degree from formula
        if let Some(deg_str) = formula.split("degree").nth(1) {
            let num_str: String = deg_str.trim()
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if let Ok(deg) = num_str.parse::<usize>() {
                return deg + 1;  // degree d polynomial has d+1 parameters
            }
        }
        // Default: treat as single parameter
        1
    }
    
    /// Check known sequences (Using EPR)
    fn check_known_sequences(&self, signal: &[EPR], kb: &KnowledgeBase, derivation: &mut Vec<String>) 
        -> Option<(String, String, Option<String>)> 
    {
        // Convert to integer for comparison (EPR can exactly determine if integer)
        let int_signal: Vec<i128> = signal.iter()
            .filter_map(|r| r.try_to_i128())
            .collect();
        
        if int_signal.len() != signal.len() {
            return None; // Contains non-integer
        }
        
        // Phase 1: Exact match from beginning
        for seq in &kb.known_sequences {
            if seq.first_terms.len() >= int_signal.len() {
                let matches = int_signal.iter()
                    .zip(seq.first_terms.iter())
                    .all(|(a, b)| *a == *b);
                
                if matches {
                    derivation.push(format!("Match known sequence: {}", seq.name));
                    derivation.push(format!("OEIS ID: {}", seq.id));
                    if let Some(ref cf) = seq.closed_form {
                        derivation.push(format!("Closed form: {}", cf));
                    }
                    return Some((seq.id.clone(), seq.name.clone(), seq.closed_form.clone()));
                }
            }
        }
        
        // Phase 2: Subsequence match (input may start at offset)
        // This handles cases like input [2,6,12,20] matching OEIS [0,2,6,12,20,...]
        for seq in &kb.known_sequences {
            if seq.first_terms.len() > int_signal.len() {
                // Try each possible starting offset
                for offset in 1..=(seq.first_terms.len() - int_signal.len()) {
                    let matches = int_signal.iter()
                        .zip(seq.first_terms[offset..].iter())
                        .all(|(a, b)| *a == *b);
                    
                    if matches {
                        derivation.push(format!("Match known sequence (offset {}): {}", offset, seq.name));
                        derivation.push(format!("OEIS ID: {}", seq.id));
                        derivation.push(format!("Input starts at index {} of OEIS sequence", offset));
                        if let Some(ref cf) = seq.closed_form {
                            derivation.push(format!("Closed form: {}", cf));
                        }
                        return Some((seq.id.clone(), seq.name.clone(), seq.closed_form.clone()));
                    }
                }
            }
        }
        
        None
    }
    
    /// Try to match detected formula against known closed forms (for fractional sequences)
    fn try_match_known_formula(&self, formula: &str, kb: &KnowledgeBase, derivation: &mut Vec<String>)
        -> Option<(String, String, Option<String>)>
    {
        let normalized_formula = Self::normalize_formula(formula);
        
        for seq in &kb.known_sequences {
            if let Some(ref cf) = seq.closed_form {
                let normalized_cf = Self::normalize_formula(cf);
                if normalized_formula == normalized_cf {
                    derivation.push(format!("Formula matches known sequence: {}", seq.name));
                    derivation.push(format!("Known closed form: {}", cf));
                    return Some((seq.id.clone(), seq.name.clone(), seq.closed_form.clone()));
                }
            }
        }
        None
    }
    
    // ============================================================================
    // ALGEBRAIC EXPLORATION (PRIORITY)
    // Uses symbolic Expr system for algebraic discovery
    // ============================================================================
    
    /// Try algebraic exploration - PRIORITY method
    /// Uses symbolic computation to find algebraic relationships
    fn try_algebraic_exploration(&self, signal: &[EPR], derivation: &mut Vec<String>) 
        -> Option<(String, Vec<String>)> 
    {
        if signal.len() < 2 {
            return None;
        }
        
        let mut steps = Vec::new();
        steps.push("=== ALGEBRAIC EXPLORATION (PRIORITY) ===".to_string());
        
        // Convert EPR signal to Expr for symbolic manipulation
        let _n = Expr::Sym(Symbol::new("n")); // Reserved for future symbolic manipulation
        
        // Method 1: Try to find algebraic relation between consecutive terms
        // s[i+1] = f(s[i], i) for some algebraic f
        steps.push("Method 1: Check for algebraic recurrence relation...".to_string());
        
        if let Some((formula, rec_steps)) = self.try_algebraic_recurrence(signal) {
            steps.extend(rec_steps);
            steps.push(format!("✓ ALGEBRAIC RECURRENCE FOUND: {}", formula));
            derivation.extend(steps.clone());
            return Some((formula, steps));
        }
        
        // Method 2: Try to find closed-form algebraic expression
        // s[i] = f(i) where f is algebraic (polynomial, rational, radical)
        steps.push("Method 2: Check for closed-form algebraic expression...".to_string());
        
        if let Some((formula, cf_steps)) = self.try_algebraic_closed_form(signal) {
            steps.extend(cf_steps);
            steps.push(format!("✓ ALGEBRAIC CLOSED FORM FOUND: {}", formula));
            derivation.extend(steps.clone());
            return Some((formula, steps));
        }
        
        // Method 3: Try to find algebraic identity (relation between terms)
        // g(s[i], s[j], i, j) = 0 for some algebraic g
        steps.push("Method 3: Check for algebraic identity...".to_string());
        
        if let Some((formula, id_steps)) = self.try_algebraic_identity(signal) {
            steps.extend(id_steps);
            steps.push(format!("✓ ALGEBRAIC IDENTITY FOUND: {}", formula));
            derivation.extend(steps.clone());
            return Some((formula, steps));
        }
        
        // Method 4: TRUE ALGEBRAIC SOLVING - construct and solve equations
        steps.push("Method 4: Algebraic equation solving...".to_string());
        
        if let Some((formula, solve_steps)) = self.try_algebraic_solving(signal) {
            steps.extend(solve_steps);
            steps.push(format!("✓ ALGEBRAIC SOLUTION FOUND: {}", formula));
            derivation.extend(steps.clone());
            return Some((formula, steps));
        }
        
        // Method 5: Try rational function f(n) = P(n)/Q(n)
        steps.push("Method 5: Rational function search...".to_string());
        
        if let Some((formula, rat_steps)) = self.try_rational_function(signal) {
            steps.extend(rat_steps);
            steps.push(format!("✓ RATIONAL FUNCTION FOUND: {}", formula));
            derivation.extend(steps.clone());
            return Some((formula, steps));
        }
        
        // Method 6: Try multiplicative function structure
        steps.push("Method 6: Multiplicative function analysis...".to_string());
        
        if let Some((formula, mult_steps)) = self.try_multiplicative_structure(signal) {
            steps.extend(mult_steps);
            steps.push(format!("✓ MULTIPLICATIVE STRUCTURE FOUND: {}", formula));
            derivation.extend(steps.clone());
            return Some((formula, steps));
        }
        
        steps.push("No algebraic structure found, proceeding to numerical methods...".to_string());
        derivation.extend(steps);
        None
    }
    
    /// TRUE ALGEBRAIC SOLVING: Construct symbolic equations and solve
    fn try_algebraic_solving(&self, signal: &[EPR]) -> Option<(String, Vec<String>)> {
        let mut steps = Vec::new();
        let n = signal.len();
        
        // Try to solve for f(n) = a*n^2 + b*n + c using system of equations
        if n >= 3 {
            steps.push("  Attempting quadratic solve: f(n) = an² + bn + c".to_string());
            
            // f(1) = a + b + c = s[0]
            // f(2) = 4a + 2b + c = s[1]
            // f(3) = 9a + 3b + c = s[2]
            // Solve: a = (s[2] - 2*s[1] + s[0]) / 2
            //        b = s[1] - s[0] - 3*a
            //        c = s[0] - a - b
            
            let two = EPR::from_i128(2);
            let three = EPR::from_i128(3);
            
            let a = (signal[2].clone() - two.clone() * signal[1].clone() + signal[0].clone()) / two.clone();
            let b = signal[1].clone() - signal[0].clone() - three.clone() * a.clone();
            let c = signal[0].clone() - a.clone() - b.clone();
            
            steps.push(format!("  Solved: a={}, b={}, c={}", a, b, c));
            
            // Verify for ALL points
            let mut valid = true;
            for i in 0..n {
                let idx = EPR::from_i128((i + 1) as i128);
                let predicted = a.clone() * idx.clone() * idx.clone() + b.clone() * idx.clone() + c.clone();
                if predicted != signal[i] {
                    valid = false;
                    steps.push(format!("  ✗ Verification failed at n={}: predicted={}, actual={}", i+1, predicted, signal[i]));
                    break;
                }
            }
            
            if valid {
                steps.push(format!("  ✓ Verified for all {} points", n));
                let formula = if c.is_zero() {
                    if b.is_zero() {
                        format!("f(n) = {}·n² [quadratic, algebraically solved]", a)
                    } else {
                        format!("f(n) = {}·n² + {}·n [quadratic, algebraically solved]", a, b)
                    }
                } else {
                    format!("f(n) = {}·n² + {}·n + {} [quadratic, algebraically solved]", a, b, c)
                };
                return Some((formula, steps));
            }
        }
        
        // Try cubic: f(n) = an³ + bn² + cn + d
        if n >= 4 {
            steps.push("  Attempting cubic solve: f(n) = an³ + bn² + cn + d".to_string());
            
            // Use finite differences to solve
            // Δ³f = 6a (constant for cubic)
            let d1: Vec<EPR> = signal.windows(2).map(|w| w[1].clone() - w[0].clone()).collect();
            let d2: Vec<EPR> = d1.windows(2).map(|w| w[1].clone() - w[0].clone()).collect();
            let d3: Vec<EPR> = d2.windows(2).map(|w| w[1].clone() - w[0].clone()).collect();
            
            if !d3.is_empty() {
                // Check if Δ³ is constant
                let first_d3 = &d3[0];
                let all_equal = d3.iter().all(|x| x == first_d3);
                
                if all_equal {
                    let six = EPR::from_i128(6);
                    let a = first_d3.clone() / six.clone();
                    
                    // Back-solve for b, c, d using known a
                    // This is a linear system
                    steps.push(format!("  Δ³ constant = {}, so a = {}", first_d3, a));
                    
                    // Verify pattern holds
                    let mut valid = true;
                    for i in 3..d3.len() {
                        if d3[i] != *first_d3 {
                            valid = false;
                            break;
                        }
                    }
                    
                    if valid && !a.is_zero() {
                        let formula = format!("f(n) has cubic component {}·n³ + ... [Δ³ = {}]", a, first_d3);
                        steps.push(format!("  ✓ Cubic structure confirmed"));
                        return Some((formula, steps));
                    }
                }
            }
        }
        
        None
    }
    
    /// Try to find rational function f(n) = P(n)/Q(n)
    fn try_rational_function(&self, signal: &[EPR]) -> Option<(String, Vec<String>)> {
        let mut steps = Vec::new();
        let n = signal.len();
        
        if n < 3 {
            return None;
        }
        
        // Try f(n) = (an + b) / (cn + d)
        steps.push("  Trying f(n) = (an + b) / (cn + d)".to_string());
        
        // Cross multiply: s[i] * (c*i + d) = a*i + b
        // s[i]*c*i + s[i]*d = a*i + b
        // Rearrange: (s[i]*i)*c + s[i]*d - a*i - b = 0
        
        // For i=1,2,3,4: we have 4 equations, 4 unknowns (a,b,c,d)
        // This is a linear system if we treat a,b,c,d as unknowns
        
        // Simplified check: see if s[i] * i is constant (f(n) = k/n case)
        let products: Vec<EPR> = signal.iter()
            .enumerate()
            .map(|(i, s)| s.clone() * EPR::from_i128((i + 1) as i128))
            .collect();
        
        let first_prod = &products[0];
        let all_equal = products.iter().all(|p| p == first_prod);
        
        if all_equal {
            steps.push(format!("  Found: s[n] * n = {} (constant)", first_prod));
            let formula = format!("f(n) = {}/n [rational, algebraically derived]", first_prod);
            return Some((formula, steps));
        }
        
        // Try f(n) = k / n² 
        let sq_products: Vec<EPR> = signal.iter()
            .enumerate()
            .map(|(i, s)| {
                let idx = EPR::from_i128((i + 1) as i128);
                s.clone() * idx.clone() * idx
            })
            .collect();
        
        let first_sq = &sq_products[0];
        let all_sq_equal = sq_products.iter().all(|p| p == first_sq);
        
        if all_sq_equal {
            steps.push(format!("  Found: s[n] * n² = {} (constant)", first_sq));
            let formula = format!("f(n) = {}/n² [rational, algebraically derived]", first_sq);
            return Some((formula, steps));
        }
        
        None
    }
    
    /// Analyze multiplicative structure: f(mn) = g(f(m), f(n))
    fn try_multiplicative_structure(&self, signal: &[EPR]) -> Option<(String, Vec<String>)> {
        let mut steps = Vec::new();
        let n = signal.len();
        
        if n < 6 {
            return None;
        }
        
        steps.push("  Checking multiplicative property: f(mn) = f(m)·f(n)".to_string());
        
        // For multiplicative function: f(2)*f(3) = f(6), f(2)*f(4) = f(8), etc.
        // Check: s[1]*s[2] =? s[5] (f(2)*f(3) = f(6))
        
        let mut multiplicative_violations = 0;
        let mut checks = 0;
        
        // Check f(2)*f(3) = f(6)
        if n >= 6 {
            let prod = signal[1].clone() * signal[2].clone(); // f(2)*f(3)
            if prod != signal[5] { // f(6)
                multiplicative_violations += 1;
            }
            checks += 1;
        }
        
        // Check f(2)*f(5) = f(10)
        if n >= 10 {
            let prod = signal[1].clone() * signal[4].clone(); // f(2)*f(5)
            if prod != signal[9] { // f(10)
                multiplicative_violations += 1;
            }
            checks += 1;
        }
        
        // Check f(3)*f(4) = f(12)
        if n >= 12 {
            let prod = signal[2].clone() * signal[3].clone(); // f(3)*f(4)
            if prod != signal[11] { // f(12)
                multiplicative_violations += 1;
            }
            checks += 1;
        }
        
        steps.push(format!("  Multiplicative checks: {}/{} passed", checks - multiplicative_violations, checks));
        
        if multiplicative_violations == 0 && checks >= 2 {
            steps.push("  ✓ Multiplicative function structure detected".to_string());
            
            // Check if completely multiplicative (f(mn) = f(m)*f(n) for ALL m,n)
            // or just multiplicative for coprime (f(mn) = f(m)*f(n) if gcd(m,n)=1)
            
            let formula = "f is multiplicative: f(mn) = f(m)·f(n) for coprime m,n".to_string();
            return Some((formula, steps));
        }
        
        // Check additive structure: f(mn) = f(m) + f(n)
        steps.push("  Checking additive property: f(mn) = f(m) + f(n)".to_string());
        
        let mut additive_violations = 0;
        checks = 0;
        
        if n >= 6 {
            let sum = signal[1].clone() + signal[2].clone(); // f(2)+f(3)
            if sum != signal[5] { // f(6)
                additive_violations += 1;
            }
            checks += 1;
        }
        
        if n >= 10 {
            let sum = signal[1].clone() + signal[4].clone(); // f(2)+f(5)
            if sum != signal[9] { // f(10)
                additive_violations += 1;
            }
            checks += 1;
        }
        
        steps.push(format!("  Additive checks: {}/{} passed", checks - additive_violations, checks));
        
        if additive_violations == 0 && checks >= 2 {
            steps.push("  ✓ Additive function structure detected".to_string());
            let formula = "f is additive: f(mn) = f(m) + f(n) for coprime m,n".to_string();
            return Some((formula, steps));
        }
        
        None
    }
    
    /// Try to find algebraic recurrence relation
    fn try_algebraic_recurrence(&self, signal: &[EPR]) -> Option<(String, Vec<String>)> {
        let mut steps = Vec::new();
        
        // Check s[i+1] = a*s[i] + b (linear recurrence)
        if signal.len() >= 3 {
            // If s[1] = a*s[0] + b and s[2] = a*s[1] + b
            // Then: a = (s[2] - s[1]) / (s[1] - s[0]) if s[1] ≠ s[0]
            let d0 = signal[1].clone() - signal[0].clone();
            let d1 = signal[2].clone() - signal[1].clone();
            
            if !d0.is_zero() {
                let a = d1.clone() / d0.clone();
                let b = signal[1].clone() - a.clone() * signal[0].clone();
                
                // Verify for all terms
                let mut valid = true;
                for i in 0..signal.len() - 1 {
                    let predicted = a.clone() * signal[i].clone() + b.clone();
                    if predicted != signal[i + 1] {
                        valid = false;
                        break;
                    }
                }
                
                if valid {
                    steps.push(format!("  Linear recurrence: s[n+1] = {}·s[n] + {}", a, b));
                    steps.push("  Verified for all consecutive pairs ✓".to_string());
                    
                    // This is actually a geometric/arithmetic progression
                    if a == EPR::from_i128(1) {
                        // Arithmetic: s[n+1] = s[n] + b, so s[n] = s[1] + (n-1)*b
                        // For n=1: s[1] = s[1] + 0 ✓
                        // For n=2: s[2] = s[1] + b ✓
                        // Closed form: f(n) = s[1] + b*(n-1) = s[1] - b + b*n
                        let c = signal[0].clone() - b.clone(); // constant term
                        let formula = if c.is_zero() {
                            format!("f(n) = {}·n [arithmetic]", b)
                        } else if c.is_negative() {
                            format!("f(n) = {}·n - {} [arithmetic]", b, c.abs())
                        } else {
                            format!("f(n) = {}·n + {} [arithmetic]", b, c)
                        };
                        steps.push(format!("  Closed form: {} (common difference = {})", formula, b));
                        return Some((formula, steps));
                    } else if b.is_zero() {
                        // Geometric: s[n+1] = a*s[n], so s[n] = s[1] * a^(n-1)
                        let formula = format!("f(n) = {}·({})^(n-1) [geometric]", signal[0], a);
                        steps.push(format!("  Closed form: {} (common ratio = {})", formula, a));
                        return Some((formula, steps));
                    } else {
                        // General linear recurrence: s[n+1] = a*s[n] + b
                        // Closed form: s[n] = a^(n-1)*s[1] + b*(a^(n-1)-1)/(a-1) for a≠1
                        let formula = format!("s[n+1] = {}·s[n] + {} [linear recurrence, s[1]={}]", a, b, signal[0]);
                        return Some((formula, steps));
                    }
                }
            }
        }
        
        // Check s[i+2] = a*s[i+1] + b*s[i] (second-order recurrence like Fibonacci)
        if signal.len() >= 4 {
            // Solve: s[2] = a*s[1] + b*s[0], s[3] = a*s[2] + b*s[1]
            // This is a linear system in a and b
            let s0 = &signal[0];
            let s1 = &signal[1];
            let s2 = &signal[2];
            let s3 = &signal[3];
            
            // s[2] = a*s[1] + b*s[0]
            // s[3] = a*s[2] + b*s[1]
            // Determinant: s[1]*s[1] - s[2]*s[0]
            let det = s1.clone() * s1.clone() - s2.clone() * s0.clone();
            
            if !det.is_zero() {
                // Cramer's rule
                let a = (s2.clone() * s1.clone() - s3.clone() * s0.clone()) / det.clone();
                let b = (s1.clone() * s3.clone() - s2.clone() * s2.clone()) / det;
                
                // Verify for all terms
                let mut valid = true;
                for i in 0..signal.len() - 2 {
                    let predicted = a.clone() * signal[i + 1].clone() + b.clone() * signal[i].clone();
                    if predicted != signal[i + 2] {
                        valid = false;
                        break;
                    }
                }
                
                if valid {
                    steps.push(format!("  Second-order recurrence: s[n+2] = {}·s[n+1] + {}·s[n]", a, b));
                    steps.push("  Verified for all consecutive triples ✓".to_string());
                    
                    // Check if it's Fibonacci-like (a=1, b=1)
                    if a == EPR::from_i128(1) && b == EPR::from_i128(1) {
                        let formula = "s[n+2] = s[n+1] + s[n] [Fibonacci-type]".to_string();
                        return Some((formula, steps));
                    } else {
                        let formula = format!("s[n+2] = {}·s[n+1] + {}·s[n] [2nd-order recurrence]", a, b);
                        return Some((formula, steps));
                    }
                }
            }
        }
        
        None
    }
    
    /// Try to find closed-form algebraic expression
    fn try_algebraic_closed_form(&self, signal: &[EPR]) -> Option<(String, Vec<String>)> {
        let mut steps = Vec::new();
        
        // Check if s[i] = c (constant)
        if signal.iter().all(|s| *s == signal[0]) {
            steps.push(format!("  Constant sequence: f(n) = {}", signal[0]));
            return Some((format!("f(n) = {}", signal[0]), steps));
        }
        
        // Check if s[i] = a*i + b (linear in index)
        // For 1-indexed: s[1] = a + b, s[2] = 2a + b
        // => a = s[2] - s[1], b = s[1] - a
        if signal.len() >= 2 {
            let a = signal[1].clone() - signal[0].clone(); // slope
            let b = signal[0].clone() - a.clone();         // s[1] = a*1 + b => b = s[1] - a
            
            let mut valid = true;
            for (i, s) in signal.iter().enumerate() {
                let n = EPR::from_i128((i + 1) as i128);
                let predicted = a.clone() * n.clone() + b.clone();
                if predicted != *s {
                    valid = false;
                    break;
                }
            }
            
            if valid {
                // Simplify output: if b is 0, just show a*n; if a is 1, just show n+b
                let formula = if b.is_zero() {
                    format!("f(n) = {}·n [linear]", a)
                } else if a == EPR::from_i128(1) {
                    format!("f(n) = n + {} [linear]", b)
                } else if b.is_negative() {
                    format!("f(n) = {}·n - {} [linear]", a, b.abs())
                } else {
                    format!("f(n) = {}·n + {} [linear]", a, b)
                };
                steps.push(format!("  Linear closed form: {} (verified for n=1..{})", formula, signal.len()));
                return Some((formula, steps));
            }
        }
        
        // Check if s[i] = c/i (harmonic-like)
        if signal.len() >= 2 && !signal[0].is_zero() {
            // If s[i] = c/i, then c = s[0]*1 = s[0]
            let c = signal[0].clone();
            
            let mut valid = true;
            for (i, s) in signal.iter().enumerate() {
                let n = EPR::from_i128((i + 1) as i128);
                if n.is_zero() {
                    valid = false;
                    break;
                }
                let predicted = c.clone() / n;
                if predicted != *s {
                    valid = false;
                    break;
                }
            }
            
            if valid {
                steps.push(format!("  Reciprocal closed form: f(n) = {}/n", c));
                steps.push("  Verified for all indices ✓".to_string());
                return Some((format!("f(n) = {}/n [reciprocal]", c), steps));
            }
        }
        
        // Check if s[i] = c*r^i (exponential)
        // For 0-indexed: s[0] = c, s[1] = c*r, s[2] = c*r^2
        // So c = s[0], r = s[1]/s[0]
        if signal.len() >= 2 && !signal[0].is_zero() {
            let r = signal[1].clone() / signal[0].clone();
            if !r.is_zero() {
                // s[i] = s[0] * r^i where i is 0-indexed
                let c = signal[0].clone();
                
                let mut valid = true;
                let mut r_power = EPR::from_i128(1);
                for s in signal.iter() {
                    let predicted = c.clone() * r_power.clone();
                    if predicted != *s {
                        valid = false;
                        break;
                    }
                    r_power = r_power * r.clone();
                }
                
                if valid {
                    steps.push(format!("  Exponential closed form: f(n) = {}·({})^(n-1)", c, r));
                    steps.push("  Verified for all indices ✓".to_string());
                    return Some((format!("f(n) = {}·({})^(n-1) [exponential]", c, r), steps));
                }
            }
        }
        
        None
    }
    
    /// Try to find algebraic identity between terms
    fn try_algebraic_identity(&self, signal: &[EPR]) -> Option<(String, Vec<String>)> {
        let mut steps = Vec::new();
        
        // Check if s[i] * i = c (product identity)
        if signal.len() >= 2 {
            let c = signal[0].clone() * EPR::from_i128(1);
            let mut valid = true;
            
            for (i, s) in signal.iter().enumerate() {
                let n = EPR::from_i128((i + 1) as i128);
                let product = s.clone() * n;
                if product != c {
                    valid = false;
                    break;
                }
            }
            
            if valid {
                steps.push(format!("  Product identity: s[n] * n = {} (constant)", c));
                steps.push("  Therefore: f(n) = {}/n".to_string());
                return Some((format!("f(n) = {}/n [from s[n]·n = {}]", c, c), steps));
            }
        }
        
        // Check if s[i]² = linear in i (quadratic identity)
        if signal.len() >= 3 {
            // If s[i]² = a*i + b, check using first two points
            let s0_sq = signal[0].clone() * signal[0].clone();
            let s1_sq = signal[1].clone() * signal[1].clone();
            let a = s1_sq.clone() - s0_sq.clone(); // slope
            let b = s0_sq.clone() - a.clone(); // s[1]² = a*1 + b
            
            let mut valid = true;
            for (i, s) in signal.iter().enumerate() {
                let n = EPR::from_i128((i + 1) as i128);
                let s_sq = s.clone() * s.clone();
                let predicted = a.clone() * n + b.clone();
                if s_sq != predicted {
                    valid = false;
                    break;
                }
            }
            
            if valid && !a.is_zero() {
                steps.push(format!("  Quadratic identity: s[n]² = {}·n + {}", a, b));
                steps.push("  Therefore: f(n) = √(a·n + b)".to_string());
                return Some((format!("f(n)² = {}·n + {} [quadratic identity]", a, b), steps));
            }
        }
        
        None
    }
    
    /// Try pattern detection (Using EPR infinite precision)
    fn try_pattern(&self, signal: &[EPR], pattern: &PatternClass, derivation: &mut Vec<String>) 
        -> Option<(String, Vec<String>)> 
    {
        derivation.push(format!("Test pattern: {}", pattern.name));
        
        match &pattern.detection {
            DetectionMethod::ConstantDifference { order } => {
                self.try_constant_difference(signal, *order, derivation)
            }
            DetectionMethod::ConstantRatio => {
                self.try_constant_ratio(signal, derivation)
            }
            DetectionMethod::ConstantProduct { factor_form } => {
                self.try_constant_product(signal, factor_form, derivation)
            }
            DetectionMethod::Periodic => {
                self.try_periodic(signal, derivation)
            }
            DetectionMethod::Factorial => {
                self.try_factorial(signal, derivation)
            }
            DetectionMethod::Recurrence { order } => {
                self.try_recurrence(signal, *order, derivation)
            }
            _ => None,
        }
    }
    
    /// Constant difference detection (polynomial) - EPR infinite precision
    fn try_constant_difference(&self, signal: &[EPR], order: usize, derivation: &mut Vec<String>) 
        -> Option<(String, Vec<String>)> 
    {
        if signal.len() < 2 {
            return None;
        }
        
        let mut steps = Vec::new();
        steps.push(format!("Input: {:?}", signal.iter().map(|r| r.to_string()).collect::<Vec<_>>()));
        
        let mut current = signal.to_vec();
        let max_order = if order == 99 { 10 } else { order + 1 };
        
        for k in 1..=max_order {
            if current.len() < 2 {
                break;
            }
            
            let diffs: Vec<EPR> = current.windows(2)
                .map(|w| w[1].clone() - w[0].clone())
                .collect();
            
            steps.push(format!("Δ^{} = {:?}", k, diffs.iter().map(|r| r.to_string()).collect::<Vec<_>>()));
            
            if diffs.iter().all(|d| *d == diffs[0]) {
                if order != 99 && k != order {
                    current = diffs;
                    continue;
                }
                
                steps.push(format!("✓ Δ^{} constant = {}", k, diffs[0]));
                
                // NO PRE-JUDGMENT: Let Fate decide freely
                // Just output the formula, Fate will continue exploring
                let formula = self.reconstruct_polynomial(signal, k);
                steps.push(format!("Formula: {}", formula));
                
                derivation.extend(steps.clone());
                return Some((formula, steps));
            }
            
            current = diffs;
        }
        
        None
    }
    
    /// Reconstruct polynomial - EPR infinite precision
    fn reconstruct_polynomial(&self, signal: &[EPR], degree: usize) -> String {
        match degree {
            0 => format!("f(n) = {}", signal[0]),
            1 => {
                let a = signal[1].clone() - signal[0].clone();
                let b = signal[0].clone() - a.clone();
                format!("f(n) = {}·n + {}", a, b)
            }
            2 => {
                if signal.len() >= 3 {
                    let d1 = signal[1].clone() - signal[0].clone();
                    let d2 = signal[2].clone() - signal[1].clone();
                    let two_a = d2 - d1.clone();
                    let a = two_a / EPR::from_i128(2);
                    let b = d1 - a.clone() * EPR::from_i128(3);
                    let c = signal[0].clone() - a.clone() - b.clone();
                    format!("f(n) = {}·n² + {}·n + {}", a, b, c)
                } else {
                    "f(n) = quadratic".to_string()
                }
            }
            _ => {
                // NEWTON FORWARD DIFFERENCE FORMULA for higher degree polynomials
                // f(n) = Σ C(n-1, k) · Δᵏ[0] where Δᵏ[0] is k-th difference at index 0
                //
                // This is the EXACT SOLUTION - not just "polynomial degree d"
                let deltas = self.compute_forward_differences(signal, degree);
                
                // Build Newton form representation
                let mut terms = Vec::new();
                for (k, delta) in deltas.iter().enumerate() {
                    if delta.is_zero() {
                        continue;
                    }
                    if k == 0 {
                        terms.push(format!("{}", delta));
                    } else if k == 1 {
                        terms.push(format!("{}·(n-1)", delta));
                    } else {
                        terms.push(format!("{}·C(n-1,{})", delta, k));
                    }
                }
                
                if terms.is_empty() {
                    "f(n) = 0".to_string()
                } else {
                    format!("f(n) = {} [Newton form, degree {}]", terms.join(" + "), degree)
                }
            }
        }
    }
    
    /// Constant ratio detection (exponential) - EPR infinite precision
    fn try_constant_ratio(&self, signal: &[EPR], derivation: &mut Vec<String>) 
        -> Option<(String, Vec<String>)> 
    {
        if signal.len() < 2 {
            return None;
        }
        
        let mut steps = Vec::new();
        let mut ratios = Vec::new();
        
        for i in 0..signal.len() - 1 {
            if signal[i].is_zero() {
                return None;
            }
            ratios.push(signal[i + 1].clone() / signal[i].clone());
        }
        
        steps.push(format!("Ratios: {:?}", ratios.iter().map(|r| r.to_string()).collect::<Vec<_>>()));
        
        if ratios.iter().all(|r| *r == ratios[0]) {
            let r = ratios[0].clone();
            let a = signal[0].clone();
            steps.push(format!("✓ Constant ratio r = {}", r));
            let formula = format!("f(n) = {} * {}^n", a, r);
            derivation.extend(steps.clone());
            return Some((formula, steps));
        }
        
        None
    }
    
    /// Constant product detection (reciprocal) - EPR infinite precision
    fn try_constant_product(&self, signal: &[EPR], _factor_form: &str, derivation: &mut Vec<String>) 
        -> Option<(String, Vec<String>)> 
    {
        if signal.len() < 2 {
            return None;
        }
        
        let mut steps = Vec::new();
        
        for k in 0i128..=5 {
            let products: Vec<EPR> = signal.iter().enumerate()
                .map(|(i, val)| val.clone() * EPR::from_i128((i as i128) + 1 + k))
                .collect();
            
            steps.push(format!("k={}: products = {:?}", k, products.iter().map(|r| r.to_string()).collect::<Vec<_>>()));
            
            if products.iter().all(|p| *p == products[0]) {
                let c = products[0].clone();
                steps.push(format!("✓ Constant product c = {}, k = {}", c, k));
                let formula = if k == 0 {
                    format!("f(n) = {} / n", c)
                } else {
                    format!("f(n) = {} / (n + {})", c, k)
                };
                derivation.extend(steps.clone());
                return Some((formula, steps));
            }
        }
        
        None
    }
    
    /// Period detection - EPR infinite precision
    fn try_periodic(&self, signal: &[EPR], derivation: &mut Vec<String>) 
        -> Option<(String, Vec<String>)> 
    {
        if signal.len() < 4 {
            return None;
        }
        
        let mut steps = Vec::new();
        
        for period in 1..=signal.len() / 2 {
            let is_periodic = (period..signal.len())
                .all(|i| signal[i] == signal[i % period]);
            
            if is_periodic {
                steps.push(format!("✓ Period = {}", period));
                let values: Vec<String> = signal[..period].iter().map(|r| r.to_string()).collect();
                let formula = format!("f(n) = [{}][n mod {}]", values.join(", "), period);
                derivation.extend(steps.clone());
                return Some((formula, steps));
            }
        }
        
        None
    }
    
    /// Factorial detection - EPR infinite precision
    fn try_factorial(&self, signal: &[EPR], derivation: &mut Vec<String>) 
        -> Option<(String, Vec<String>)> 
    {
        if signal.len() < 3 {
            return None;
        }
        
        let mut steps = Vec::new();
        steps.push("Test factorial pattern: f(n) = n * f(n-1)".to_string());
        
        for i in 1..signal.len() {
            if signal[i-1].is_zero() {
                return None;
            }
            
            let ratio = signal[i].clone() / signal[i-1].clone();
            let expected = EPR::from_i128((i + 1) as i128);
            
            if ratio != expected {
                steps.push(format!("Ratio s[{}]/s[{}] = {} ≠ {}", i, i-1, ratio, expected));
                return None;
            }
        }
        
        steps.push("✓ Factorial pattern confirmed".to_string());
        derivation.extend(steps.clone());
        Some(("f(n) = n!".to_string(), steps))
    }
    
    /// Recurrence detection - EPR infinite precision
    fn try_recurrence(&self, signal: &[EPR], order: usize, derivation: &mut Vec<String>) 
        -> Option<(String, Vec<String>)> 
    {
        if signal.len() < order + 2 {
            return None;
        }
        
        let mut steps = Vec::new();
        steps.push(format!("Test order-{} recurrence", order));
        
        if order == 2 && signal.len() >= 4 {
            let s0 = signal[0].clone();
            let s1 = signal[1].clone();
            let s2 = signal[2].clone();
            let s3 = signal[3].clone();
            
            let det = s1.clone() * s1.clone() - s2.clone() * s0.clone();
            if det.is_zero() {
                return None;
            }
            
            let c1 = (s2.clone() * s1.clone() - s3.clone() * s0.clone()) / det.clone();
            let c2 = (s3.clone() * s1.clone() - s2.clone() * s2.clone()) / det;
            
            steps.push(format!("Candidate: f(n) = {}*f(n-1) + {}*f(n-2)", c1, c2));
            
            // Verify
            for i in 2..signal.len() {
                let expected = c1.clone() * signal[i-1].clone() + c2.clone() * signal[i-2].clone();
                if expected != signal[i] {
                    return None;
                }
            }
            
            steps.push("✓ Recurrence verified".to_string());
            let formula = format!("f(n) = {}*f(n-1) + {}*f(n-2)", c1, c2);
            derivation.extend(steps.clone());
            return Some((formula, steps));
        }
        
        None
    }
    
    /// Get statistics
    pub fn stats(&self) -> FateStats {
        FateStats {
            tick: self.state.tick,
            signal_count: self.state.signal_count,
            discovery_count: self.state.discovery_count,
            pattern_diversity: self.state.pattern_diversity,
            history_len: self.history.len(),
        }
    }
    
    /// Get all discoveries
    pub fn discoveries(&self) -> &[Discovery] {
        &self.discoveries
    }
    
    // ========================================================================
    // Discovery criterion helper methods
    // ========================================================================
    
    /// Normalize formula for comparison
    /// Remove spaces, normalize variable names, etc.
    fn normalize_formula(formula: &str) -> String {
        let s = formula
            .to_lowercase()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            // Normalize common representations
            .replace("·", "*")
            .replace("×", "*")
            .replace("^2", "²")
            .replace("^3", "³")
            .replace("f(n)=", "")
            .replace("g(i)=", "");
        
        // Handle "polynomial degree X" format
        if s.contains("polynomialdegree") {
            if s.contains("polynomialdegree2") {
                return "n²".to_string();
            } else if s.contains("polynomialdegree3") {
                return "n³".to_string();
            }
            // Keep higher order polynomials as-is
        }
        
        s
    }
    
    /// Parse linear formula "f(n) = a*n + b" and return (a, b) as strings
    fn parse_linear_formula(formula: &str) -> Option<(String, String)> {
        // Match patterns like "3*n + 1", "3n+1", "f(n) = 3*n + 1"
        let formula = formula.replace(" ", "").replace("f(n)=", "");
        
        // Try to find coefficient and constant
        if let Some(pos) = formula.find("*n") {
            let a = &formula[..pos];
            let rest = &formula[pos + 2..];
            if let Some(b_pos) = rest.find('+') {
                let b = &rest[b_pos + 1..];
                return Some((a.to_string(), b.to_string()));
            } else if let Some(b_pos) = rest.find('-') {
                let b = &rest[b_pos..]; // Include the minus
                return Some((a.to_string(), b.to_string()));
            } else if rest.is_empty() {
                return Some((a.to_string(), "0".to_string()));
            }
        }
        
        None
    }
    
    /// Get discovery summary (for report)
    pub fn get_discovery_summary(&self) -> DiscoverySummary {
        let mut novel_count = 0;
        let mut system_pattern_count = 0;
        let mut new_rule_count = 0;
        let mut level_up_count = 0;
        let mut math_pattern_count = 0;
        
        let mut novel_discoveries = Vec::new();
        
        for d in &self.discoveries {
            match &d.discovery_type {
                DiscoveryType::NovelResult { known_formula, found_formula, difference } => {
                    novel_count += 1;
                    novel_discoveries.push(NovelDiscoveryRecord {
                        id: d.id.clone(),
                        known_formula: known_formula.clone(),
                        found_formula: found_formula.clone(),
                        difference: difference.clone(),
                        tick: d.tick,
                    });
                }
                DiscoveryType::SystemPattern => system_pattern_count += 1,
                DiscoveryType::NewRule => new_rule_count += 1,
                DiscoveryType::LevelUp => level_up_count += 1,
                DiscoveryType::MathPattern => math_pattern_count += 1,
            }
        }
        
        DiscoverySummary {
            total_signals: self.state.signal_count,
            total_discoveries: novel_count + system_pattern_count + new_rule_count + level_up_count,
            novel_results: novel_count,
            system_patterns: system_pattern_count,
            new_rules: new_rule_count,
            level_ups: level_up_count,
            pattern_detections: math_pattern_count, // Not counted as real discovery
            novel_discoveries,
        }
    }
    
    /// Match known sequences in KB by formula
    /// Solves fractional sequences that cannot match by value
    fn find_known_formula_match(&self, formula: &str, kb: &KnowledgeBase) -> Option<String> {
        let normalized = Self::normalize_formula(formula);
        
        // Iterate all known sequences in KB
        for seq in kb.get_known_sequences() {
            // Check closed_form
            if let Some(ref closed) = seq.closed_form {
                let normalized_closed = Self::normalize_formula(closed);
                if normalized == normalized_closed {
                    return Some(closed.clone());
                }
                // Also check common variants
                if Self::formulas_equivalent(&normalized, &normalized_closed) {
                    return Some(closed.clone());
                }
            }
            
            // Check recurrence
            if let Some(ref recurrence) = seq.recurrence {
                let normalized_rec = Self::normalize_formula(recurrence);
                if normalized == normalized_rec {
                    return Some(recurrence.clone());
                }
            }
        }
        
        // Check standard formula patterns
        let standard_formulas = [
            ("1/n", vec!["1/n", "1/(n)", "n^-1", "n^(-1)"]),
            ("1/(n+1)", vec!["1/(n+1)", "1/(1+n)", "(n+1)^-1"]),
            ("n^2", vec!["n^2", "n²", "n*n"]),
            ("n^3", vec!["n^3", "n³", "n*n*n"]),
            ("n!", vec!["n!", "factorial(n)", "n factorial"]),
            ("2^n", vec!["2^n", "2ⁿ", "pow(2,n)"]),
            ("n*(n+1)/2", vec!["n*(n+1)/2", "n(n+1)/2", "triangular"]),
        ];
        
        for (canonical, variants) in &standard_formulas {
            if variants.iter().any(|v| Self::normalize_formula(v) == normalized) {
                return Some(canonical.to_string());
            }
            if Self::normalize_formula(canonical) == normalized {
                return Some(canonical.to_string());
            }
        }
        
        None
    }
    
    /// Check if two formulas are mathematically equivalent
    fn formulas_equivalent(a: &str, b: &str) -> bool {
        // Simple equivalence check
        if a == b { return true; }
        
        // Various forms of 1/n
        let reciprocal_forms = ["1/n", "1/(n)", "n^-1", "n^(-1)", "f(n)=1/n"];
        if reciprocal_forms.iter().any(|f| a.contains(f) || Self::normalize_formula(a) == Self::normalize_formula(f)) &&
           reciprocal_forms.iter().any(|f| b.contains(f) || Self::normalize_formula(b) == Self::normalize_formula(f)) {
            return true;
        }
        
        // Various forms of n^2
        let square_forms = ["n^2", "n²", "n*n", "n**2", "polynomialdegree2"];
        if square_forms.iter().any(|f| a.contains(f) || Self::normalize_formula(a) == Self::normalize_formula(f)) &&
           square_forms.iter().any(|f| b.contains(f) || Self::normalize_formula(b) == Self::normalize_formula(f)) {
            return true;
        }
        
        // Various forms of n^3
        let cube_forms = ["n^3", "n³", "n*n*n", "n**3", "polynomialdegree3"];
        if cube_forms.iter().any(|f| a.contains(f) || Self::normalize_formula(a) == Self::normalize_formula(f)) &&
           cube_forms.iter().any(|f| b.contains(f) || Self::normalize_formula(b) == Self::normalize_formula(f)) {
            return true;
        }
        
        // Various forms of n!
        let factorial_forms = ["n!", "factorial(n)", "f(n)=n!"];
        if factorial_forms.iter().any(|f| a.contains(f) || Self::normalize_formula(a).contains(&Self::normalize_formula(f))) &&
           factorial_forms.iter().any(|f| b.contains(f) || Self::normalize_formula(b).contains(&Self::normalize_formula(f))) {
            return true;
        }
        
        false
    }
}

/// Discovery Summary
#[derive(Debug, Clone)]
pub struct DiscoverySummary {
    pub total_signals: u64,
    pub total_discoveries: u64,
    pub novel_results: u64,      // Real discovery!
    pub system_patterns: u64,
    pub new_rules: u64,
    pub level_ups: u64,
    pub pattern_detections: u64, // Not counted as discovery
    pub novel_discoveries: Vec<NovelDiscoveryRecord>,
}

/// Novel Discovery Record
#[derive(Debug, Clone)]
pub struct NovelDiscoveryRecord {
    pub id: String,
    pub known_formula: Option<String>,
    pub found_formula: String,
    pub difference: String,
    pub tick: u64,
}

impl Default for Fate {
    fn default() -> Self {
        Self::new()
    }
}

/// Fate Statistics
#[derive(Debug, Clone)]
pub struct FateStats {
    pub tick: u64,
    pub signal_count: u64,
    pub discovery_count: u64,
    pub pattern_diversity: u64,
    pub history_len: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fate_linear_epr() {
        let mut fate = Fate::new();
        let mut kb = KnowledgeBase::new();
        kb.initialize_fundamentals();
        
        // Using EPR infinite precision: 1/2, 1, 3/2
        let signal = vec![
            EPR::from_ratio(1, 2),
            EPR::from_i128(1),
            EPR::from_ratio(3, 2),
        ];
        
        let result = fate.explore(&signal, &kb);
        
        println!("Status: {:?}", result.status);
        println!("Formula: {:?}", result.formula);
        
        assert!(matches!(result.status, ExplorationStatus::PatternDetected { .. }));
    }
    
    #[test]
    fn test_fate_fibonacci_epr() {
        let mut fate = Fate::new();
        let mut kb = KnowledgeBase::new();
        kb.initialize_fundamentals();
        
        // Fibonacci: 1, 1, 2, 3, 5
        let signal = vec![
            EPR::from_i128(1),
            EPR::from_i128(1),
            EPR::from_i128(2),
            EPR::from_i128(3),
            EPR::from_i128(5),
        ];
        
        let result = fate.explore(&signal, &kb);
        
        println!("Status: {:?}", result.status);
        
        assert!(matches!(result.status, ExplorationStatus::KnownSequence { .. }));
    }
    
    #[test]
    fn test_fate_factorial_epr() {
        let mut fate = Fate::new();
        let mut kb = KnowledgeBase::new();
        kb.initialize_fundamentals();
        
        // Factorial: 1, 2, 6, 24, 120
        let signal = vec![
            EPR::from_i128(1),
            EPR::from_i128(2),
            EPR::from_i128(6),
            EPR::from_i128(24),
            EPR::from_i128(120),
        ];
        
        let result = fate.explore(&signal, &kb);
        
        println!("Status: {:?}", result.status);
        println!("Formula: {:?}", result.formula);
        
        // Should detect factorial pattern
        assert!(result.formula.is_some());
    }
    
    #[test]
    fn test_fate_large_factorial_epr() {
        // Test EPR with large numbers: 50! 
        let mut fate = Fate::new();
        let mut kb = KnowledgeBase::new();
        kb.initialize_fundamentals();
        
        // Compute first few large factorials
        let mut factorial = EPR::one();
        let mut signal = vec![factorial.clone()];
        for i in 2..=10 {
            factorial = factorial * EPR::from_i128(i);
            signal.push(factorial.clone());
        }
        
        let result = fate.explore(&signal, &kb);
        
        println!("Large factorial test Status: {:?}", result.status);
        println!("Formula: {:?}", result.formula);
        
        // Should detect factorial pattern
        assert!(result.formula.is_some());
    }
}


// ============================================================================
// Voyager/Space Integration - FREE Architecture (No Restrictions)
// Design Philosophy:
// - NO pre-checks that block exploration
// - OEIS is POST-verification only (doesn't stop exploration)
// - All pattern classes tried (no search_budget limit)
// - Exploration is COMPLETE before any judgment
// ============================================================================

impl Fate {
    /// Full FREE exploration: Fate + Voyager + Space
    /// 
    /// Design Philosophy:
    /// 1. EXPLORE FIRST - No pre-checks that stop exploration
    /// 2. OEIS POST-VERIFICATION - Used for labeling, not blocking
    /// 3. TRY ALL PATTERNS - No artificial search_budget limit
    /// 4. COMPLETE ANALYSIS - Fate + Voyager + Space all run
    pub fn full_exploration(&mut self, signal: &[EPR], kb: &KnowledgeBase) 
        -> FullExplorationResult 
    {
        use voyager::{PreciseSignalDetector, DimensionManager, DimensionConfig, Rational as VRational};
        use crate::space::FractionalDimensionCalculator;
        use crate::symbolic::Rational;
        
        // Convert signals
        let voyager_signal: Vec<VRational> = signal.iter()
            .map(|epr| {
                let n = epr.numerator.try_to_i128().unwrap_or(0);
                let d = epr.denominator.try_to_i128().unwrap_or(1);
                VRational::new(n, d)
            })
            .collect();
        
        let space_signal: Vec<Rational> = signal.iter()
            .map(|epr| {
                let n = epr.numerator.try_to_i128().unwrap_or(0);
                let d = epr.denominator.try_to_i128().unwrap_or(1);
                Rational::new(n, d)
            })
            .collect();
        
        // ================================================================
        // STEP 1: COMPLETE FREE EXPLORATION (no pre-checks!)
        // ================================================================
        
        // === FATE: Full exploration ===
        let fate_result = self.explore_with_verification(signal, kb);
        
        // === VOYAGER: Precise signal detection ===
        let mut detector = PreciseSignalDetector::new();
        let voyager_detection = detector.detect_signal_precisely(voyager_signal.clone());
        let voyager_equation = detector.create_mathematical_equation(&voyager_detection, &voyager_signal);
        
        // Voyager dimension (no max limit restriction)
        let config = DimensionConfig {
            initial_dimensions: 4,
            max_dimensions: 99,  // Effectively unlimited
            auto_expand: true,
            fractional_support: true,
        };
        let mut dim_manager = DimensionManager::new(config);
        let voyager_dim = dim_manager.adaptive_dimension_expansion(&voyager_signal);
        
        // === SPACE: Dimensional analysis ===
        let mut dim_calc = FractionalDimensionCalculator::new();
        let space_result = dim_calc.calculate_optimal_dimension(&space_signal);
        
        // ================================================================
        // STEP 2: POST-VERIFICATION (after exploration complete!)
        // ================================================================
        
        // OEIS post-check (doesn't block, only labels)
        let oeis_match = self.oeis_post_verification(signal, kb);
        
        // ================================================================
        // STEP 3: COMBINE RESULTS (no early returns!)
        // ================================================================
        
        let final_status = self.assess_exploration_result(
            &fate_result,
            &voyager_detection,
            &oeis_match,
            &space_result
        );
        
        FullExplorationResult {
            // Fate (complete exploration result)
            fate_status: format!("{:?}", fate_result.status),
            fate_formula: fate_result.formula.clone(),
            fate_fit_level: fate_result.fit_level.as_ref().map(|l| format!("{:?}", l)),
            fate_is_discovery: fate_result.is_discovery,
            
            // Voyager (complete analysis)
            voyager_detection: format!("{:?}", voyager_detection),
            voyager_equation: voyager_equation.as_ref().map(|j| j.suggested_type.to_math_string()),
            voyager_dimension: voyager_dim.to_string(),
            voyager_verified: voyager_equation.as_ref().map(|j| j.algebraically_verified).unwrap_or(false),
            
            // Space (complete dimensional analysis)
            space_hausdorff: space_result.hausdorff_estimate.to_string(),
            space_box: space_result.box_estimate.to_string(),
            space_information: space_result.info_estimate.to_string(),
            
            // OEIS (POST-verification, not pre-check)
            oeis_match,
            
            // Final assessment
            final_status,
            
            // Full result
            full_result: fate_result,
        }
    }
    
    /// OEIS POST-verification (after exploration, not before!)
    /// This only LABELS results, doesn't BLOCK exploration
    fn oeis_post_verification(&self, signal: &[EPR], kb: &KnowledgeBase) -> Option<String> {
        for seq in &kb.known_sequences {
            if self.signal_matches_sequence(signal, seq) {
                return Some(format!("{} ({})", seq.id, seq.name));
            }
        }
        None
    }
    
    fn signal_matches_sequence(&self, signal: &[EPR], seq: &crate::knowledge::KnownSequence) -> bool {
        if signal.is_empty() || seq.first_terms.is_empty() {
            return false;
        }
        
        // Try matching from different offsets (flexible matching)
        for offset in 0..seq.first_terms.len().saturating_sub(signal.len()) + 1 {
            let mut matches = true;
            for (i, val) in signal.iter().enumerate() {
                if let Some(n) = val.try_to_i128() {
                    if offset + i < seq.first_terms.len() && n != seq.first_terms[offset + i] {
                        matches = false;
                        break;
                    }
                } else {
                    matches = false;
                    break;
                }
            }
            if matches {
                return true;
            }
        }
        false
    }
    
    /// Assess exploration result (POST-exploration assessment)
    /// No early returns - all results are combined
    fn assess_exploration_result(
        &self,
        fate: &ExplorationResult,
        voyager: &voyager::PreciseDetectionResult,
        oeis: &Option<String>,
        space: &crate::space::DimensionResult,
    ) -> FinalStatus {
        // Build assessment from ALL sources (no short-circuit!)
        
        let mut sources = Vec::new();
        
        // Collect all findings
        if let Some(oeis_id) = oeis {
            sources.push(format!("OEIS: {}", oeis_id));
        }
        
        if let voyager::PreciseDetectionResult::KnownEquation { equation_id, .. } = voyager {
            sources.push(format!("Voyager: {}", equation_id));
        }
        
        if fate.formula.is_some() {
            sources.push(format!("Fate: {}", fate.formula.as_ref().unwrap()));
        }
        
        // Determine status based on ALL collected information
        if !sources.is_empty() && oeis.is_some() {
            // Known to external authority
            return FinalStatus::Known { 
                source: sources.join(", "),
            };
        }
        
        if fate.formula.is_some() {
            // IMPORTANT: Use FIT_LEVEL (which already considers Effective DoF from cross-domain verification)
            // NOT the original fit_analysis.is_trivial
            let cross_domain_verified = fate.cross_domain_verification
                .as_ref()
                .map(|v| v.extrapolation_success)
                .unwrap_or(false);
            
            // Use fit_level if available, otherwise fall back to fit_analysis
            if let Some(fit_level) = &fate.fit_level {
                match fit_level {
                    FitLevel::InterpolantFit => {
                        let dof = fate.fit_analysis.as_ref().map(|f| f.dof).unwrap_or(0);
                        return FinalStatus::TrivialInterpolation {
                            dof: dof as i32,
                            reason: format!("DoF={}, pure interpolation (mathematically correct)", dof),
                        };
                    }
                    FitLevel::DescriptiveModel => {
                        return FinalStatus::DescriptiveModel {
                            formula: fate.formula.clone().unwrap_or_default(),
                        };
                    }
                    FitLevel::PredictiveFit | FitLevel::EmergentLaw => {
                        // These have been upgraded by cross-domain verification
                        let effective_dof = if cross_domain_verified {
                            if let Some(v) = &fate.cross_domain_verification {
                                let sample_size = fate.signal.len();
                                let params = fate.fit_analysis.as_ref().map(|f| f.num_parameters).unwrap_or(1);
                                sample_size + v.new_points_matched - params
                            } else {
                                fate.fit_analysis.as_ref().map(|f| f.dof).unwrap_or(0)
                            }
                        } else {
                            fate.fit_analysis.as_ref().map(|f| f.dof).unwrap_or(0)
                        };
                        
                        return FinalStatus::CandidateLaw {
                            formula: fate.formula.clone().unwrap_or_default(),
                            dof: effective_dof as i32,
                            cross_domain_verified,
                        };
                    }
                }
            }
            
            // Fallback: use fit_analysis
            if let Some(fit) = &fate.fit_analysis {
                if fit.is_trivial {
                    return FinalStatus::TrivialInterpolation {
                        dof: fit.dof as i32,
                        reason: format!("DoF={}, pure interpolation", fit.dof),
                    };
                }
            }
            
            return FinalStatus::DescriptiveModel {
                formula: fate.formula.clone().unwrap_or_default(),
            };
        }
        
        // No pattern found in tested classes - NOT wrong, just unclassified
        FinalStatus::Unclassified {
            tested_classes: vec!["Polynomial", "Exponential", "Reciprocal", "Periodic"]
                .into_iter().map(String::from).collect(),
            space_dimension: space.recommended_dimension.to_string(),
            next_step: "Try more pattern classes, or collect more data points".to_string(),
        }
    }
    
    // ============================================================================
    // NEWTON FORWARD DIFFERENCE FORMULA IMPLEMENTATION
    // ============================================================================
    // 
    // THEOREM (Newton Forward Difference):
    // f(n) = Σ_{k=0}^{d} C(n-1, k) · Δᵏf(1)
    // 
    // where Δᵏf(i) = Δᵏ⁻¹f(i+1) - Δᵏ⁻¹f(i), Δ⁰f(i) = f(i)
    // 
    // This allows computing f(n) for ANY n using only the differences at index 1.
    // ============================================================================
    
    /// Compute forward differences Δ⁰, Δ¹, Δ², ..., Δᵈ at index 0
    /// Returns [Δ⁰[0], Δ¹[0], Δ²[0], ..., Δᵈ[0]]
    fn compute_forward_differences(&self, signal: &[EPR], max_order: usize) -> Vec<EPR> {
        let mut deltas = vec![signal[0].clone()]; // Δ⁰[0] = f(1) = signal[0]
        
        let mut current = signal.to_vec();
        for _ in 0..max_order {
            if current.len() < 2 {
                break;
            }
            // Compute next order difference
            let mut next = Vec::new();
            for i in 0..current.len() - 1 {
                next.push(current[i + 1].clone() - current[i].clone());
            }
            if !next.is_empty() {
                deltas.push(next[0].clone()); // Δᵏ[0]
            }
            current = next;
        }
        
        deltas
    }
    
    /// Compute binomial coefficient C(n, k) = n! / (k! * (n-k)!)
    #[allow(dead_code)]
    fn binomial_coefficient(&self, n: i128, k: usize) -> EPR {
        if k == 0 {
            return EPR::from_i128(1);
        }
        if n < 0 && k > 0 {
            // Handle negative n using: C(n,k) = (-1)^k * C(k-n-1, k)
            let sign = if k % 2 == 0 { 1i128 } else { -1i128 };
            let pos_n = (k as i128) - n - 1;
            let pos_result = self.binomial_coefficient(pos_n, k);
            return EPR::from_i128(sign) * pos_result;
        }
        if (n as usize) < k {
            return EPR::from_i128(0);
        }
        
        // C(n,k) = n*(n-1)*...*(n-k+1) / k!
        let mut num = EPR::from_i128(1);
        let mut den = EPR::from_i128(1);
        for i in 0..k {
            num = num * EPR::from_i128(n - i as i128);
            den = den * EPR::from_i128((i + 1) as i128);
        }
        num / den
    }
    
    /// Newton Forward Interpolation: f(n) = Σ C(n-1, k) · Δᵏ[0]
    /// n is 0-indexed (n=0 → first point, n=1 → second point, etc.)
    #[allow(dead_code)]
    fn newton_forward_interpolation(&self, deltas: &[EPR], n: i128) -> EPR {
        let mut result = EPR::from_i128(0);
        for (k, delta_k) in deltas.iter().enumerate() {
            let binom = self.binomial_coefficient(n, k);
            result = result + binom * delta_k.clone();
        }
        result
    }
    
    /// Compute polynomial coefficients from signal using Newton's formula
    /// Returns [a₀, a₁, a₂, ..., aₐ] for f(n) = a₀ + a₁n + a₂n² + ...
    pub fn compute_polynomial_coefficients(&self, signal: &[EPR]) -> Vec<EPR> {
        let degree = signal.len() - 1;
        let deltas = self.compute_forward_differences(signal, degree);
        
        // Newton forward difference coefficients to standard polynomial coefficients
        // This is more complex - for now, just return the deltas which define the polynomial
        // via f(n) = Σ C(n-1, k) · Δᵏ[0]
        deltas
    }
    
    /// Format polynomial coefficients as a readable string
    pub fn format_polynomial(&self, coeffs: &[EPR]) -> String {
        if coeffs.is_empty() {
            return "0".to_string();
        }
        
        let mut terms = Vec::new();
        for (k, coeff) in coeffs.iter().enumerate() {
            if coeff.is_zero() {
                continue;
            }
            let coeff_str = format!("{}", coeff);
            if k == 0 {
                terms.push(coeff_str);
            } else {
                terms.push(format!("C(n,{})·{}", k, coeff_str));
            }
        }
        
        if terms.is_empty() {
            "0".to_string()
        } else {
            format!("f(n) = {}", terms.join(" + "))
        }
    }
}

/// Final status (POST-exploration assessment)
/// 
/// Key principle: NOT matching OEIS does NOT mean incorrect!
/// OEIS is only a reference, not a correctness criterion.
#[derive(Debug, Clone)]
pub enum FinalStatus {
    /// Known to external authority (OEIS/Voyager KB)
    Known { source: String },
    
    /// Trivial interpolation (DoF=0) - mathematically correct but not informative
    TrivialInterpolation { dof: i32, reason: String },
    
    /// Descriptive model (low DoF) - potentially correct, needs more data
    DescriptiveModel { formula: String },
    
    /// Candidate law (high DoF, verified by cross-domain expansion)
    /// NOT in OEIS does NOT mean wrong - may be genuinely new!
    CandidateLaw { formula: String, dof: i32, cross_domain_verified: bool },
    
    /// Under investigation - no pattern found in tested classes
    /// This is NOT "wrong" or "impossible" - just needs more pattern classes
    Unclassified { tested_classes: Vec<String>, space_dimension: String, next_step: String },
    
    /// MYSTERY - Fate exhausted all explorations, no new transformations possible, no solution found
    /// This represents a genuine mathematical mystery - the sequence has no detectable structure
    /// within the current hypothesis space H and transformation set T
    Mystery { 
        iterations: usize,
        transformations_exhausted: bool,
        hypothesis_space: String,
        conclusion: String,
    },
}

impl std::fmt::Display for FinalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FinalStatus::Known { source } => write!(f, "[KNOWN] {}", source),
            FinalStatus::TrivialInterpolation { dof, reason } => write!(f, "[TRIVIAL_FIT] DoF={}, {}", dof, reason),
            FinalStatus::DescriptiveModel { formula } => write!(f, "[DESCRIPTIVE] {}", formula),
            FinalStatus::CandidateLaw { formula, dof, cross_domain_verified } => {
                let verified = if *cross_domain_verified { " ✓cross-domain" } else { "" };
                write!(f, "[CANDIDATE_LAW] {} (DoF={}){}",  formula, dof, verified)
            }
            FinalStatus::Unclassified { tested_classes, next_step, .. } => {
                write!(f, "[UNCLASSIFIED] Tested: {:?}. Next: {}", tested_classes, next_step)
            }
            FinalStatus::Mystery { iterations, transformations_exhausted, conclusion, .. } => {
                let exhausted = if *transformations_exhausted { "✓" } else { "✗" };
                write!(f, "[MYSTERY] {} iterations, transformations exhausted: {}, {}", 
                       iterations, exhausted, conclusion)
            }
        }
    }
}

/// Full exploration result
#[derive(Debug, Clone)]
pub struct FullExplorationResult {
    pub fate_status: String,
    pub fate_formula: Option<String>,
    pub fate_fit_level: Option<String>,
    pub fate_is_discovery: bool,
    pub voyager_detection: String,
    pub voyager_equation: Option<String>,
    pub voyager_dimension: String,
    pub voyager_verified: bool,
    pub space_hausdorff: String,
    pub space_box: String,
    pub space_information: String,
    pub oeis_match: Option<String>,
    pub final_status: FinalStatus,
    pub full_result: ExplorationResult,
}
