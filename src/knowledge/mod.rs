//! Knowledge Base Module
//! 
//! Persistent storage for physics and mathematics knowledge.
//! - Physical constants (NIST CODATA)
//! - Mathematical constants  
//! - Mathematical theorems and identities
//! - Physical laws
//! - Discovered equations (from exploration)
//! - **Pattern classes (for Fate to load)**
//! - **Known sequences (OEIS etc.)**
//! 
//! Storage format: JSON (efficient with serde)
//! Import/Export: TeX (human-readable)
//!
//! # Architecture
//! 
//! Fate loads pattern classes from here - it has NO hardcoded patterns.
//! All mathematical knowledge lives here.

pub mod constants;
pub mod theorems;
pub mod storage;
pub mod patterns;
pub mod oeis;

pub use patterns::{
    PatternClass, DetectionMethod, Complexity,
    KnownSequence,
    get_default_pattern_classes, get_known_sequences,
    match_known_sequence,
};

use serde::{Deserialize, Serialize};

/// Knowledge entry status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeStatus {
    /// Verified from authoritative source (NIST, textbooks, etc.)
    Verified,
    /// Discovered by S-wind, pending human review
    PendingReview,
    /// Rejected after review
    Rejected,
}

/// Physical constant entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalConstant {
    pub id: String,
    pub name: String,
    pub symbol: String,           // LaTeX symbol, e.g., "c", "\\hbar"
    pub value: String,            // Exact or best known value
    pub unit: String,             // SI unit
    pub uncertainty: Option<String>,
    pub source: String,           // e.g., "NIST CODATA 2022"
    pub status: KnowledgeStatus,
    pub tex_definition: String,   // Full LaTeX definition
}

/// Mathematical constant entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathConstant {
    pub id: String,
    pub name: String,
    pub symbol: String,           // LaTeX symbol
    pub value_approx: f64,        // Approximate value for reference
    pub exact_form: Option<String>, // Exact algebraic form if known
    pub series_form: Option<String>, // Series representation
    pub source: String,
    pub status: KnowledgeStatus,
    pub tex_definition: String,
}

/// Mathematical identity/theorem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathTheorem {
    pub id: String,
    pub name: String,
    pub category: TheoremCategory,
    pub statement_tex: String,    // LaTeX statement
    pub proof_tex: Option<String>, // LaTeX proof if available
    pub references: Vec<String>,  // Related theorems
    pub source: String,
    pub status: KnowledgeStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TheoremCategory {
    Algebra,
    Analysis,
    NumberTheory,
    Geometry,
    Topology,
    Combinatorics,
    Probability,
    Logic,
    SetTheory,
    Other(String),
}

/// Physical law entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalLaw {
    pub id: String,
    pub name: String,
    pub domain: PhysicsDomain,
    pub equation_tex: String,     // LaTeX equation
    pub description: String,
    pub variables: Vec<Variable>,
    pub source: String,
    pub status: KnowledgeStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhysicsDomain {
    ClassicalMechanics,
    Electromagnetism,
    Thermodynamics,
    QuantumMechanics,
    Relativity,
    StatisticalMechanics,
    QuantumFieldTheory,
    GeneralRelativity,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub symbol: String,
    pub name: String,
    pub unit: Option<String>,
}

/// Discovered equation (from S-wind exploration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredEquation {
    pub id: String,
    pub equation_tex: String,
    pub description: String,
    pub discovered_at: u64,       // Unix timestamp
    pub derivation_tex: String,   // Full derivation in LaTeX
    pub verification_count: u64,
    pub status: KnowledgeStatus,  // Always starts as PendingReview
}

/// The complete knowledge base
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KnowledgeBase {
    pub physical_constants: Vec<PhysicalConstant>,
    pub math_constants: Vec<MathConstant>,
    pub theorems: Vec<MathTheorem>,
    pub physical_laws: Vec<PhysicalLaw>,
    pub discovered: Vec<DiscoveredEquation>,
    
    /// Pattern classes - Fate loads these, no hardcoding!
    #[serde(default)]
    pub pattern_classes: Vec<PatternClass>,
    
    /// Known sequences (from OEIS, etc.)
    #[serde(default)]
    pub known_sequences: Vec<KnownSequence>,
    
    /// Metadata
    pub version: String,
    pub last_updated: u64,
}

impl KnowledgeBase {
    pub fn new() -> Self {
        let mut kb = KnowledgeBase::default();
        kb.version = "1.0.0".to_string();
        kb.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        kb
    }
    
    /// Initialize with fundamental constants and laws
    pub fn initialize_fundamentals(&mut self) {
        constants::add_physical_constants(self);
        constants::add_math_constants(self);
        theorems::add_fundamental_theorems(self);
        theorems::add_physical_laws(self);
        
        // Load pattern classes (for Fate to use)
        self.pattern_classes = get_default_pattern_classes();
        self.known_sequences = get_known_sequences();
        
        // Load OEIS built-in sequences (95+ sequences)
        let mut oeis_loader = oeis::OeisLoader::new();
        oeis_loader.load_builtin();
        oeis_loader.add_to_knowledge_base(self);
    }
    
    /// Get available pattern classes (for Fate to load)
    pub fn get_pattern_classes(&self) -> &[PatternClass] {
        &self.pattern_classes
    }
    
    /// Get available patterns (alias for Fate compatibility)
    pub fn get_available_patterns(&self) -> Vec<PatternClass> {
        let mut patterns = self.pattern_classes.clone();
        patterns.sort_by_key(|p| p.priority);
        patterns
    }
    
    /// Get all known sequences (for formula matching in Fate)
    pub fn get_known_sequences(&self) -> &[KnownSequence] {
        &self.known_sequences
    }
    
    /// Check if a sequence matches a known one
    pub fn match_sequence(&self, signal: &[i128]) -> Option<&KnownSequence> {
        for seq in &self.known_sequences {
            if seq.first_terms.len() >= signal.len() && signal.len() >= 3 {
                let matches = signal.iter()
                    .zip(seq.first_terms.iter())
                    .all(|(a, b)| *a == *b);
                if matches {
                    return Some(seq);
                }
            }
        }
        None
    }
    
    /// Add a new pattern class (discovered by Fate)
    pub fn add_pattern_class(&mut self, pattern: PatternClass) {
        // Check if already exists
        if !self.pattern_classes.iter().any(|p| p.id == pattern.id) {
            self.pattern_classes.push(pattern);
            self.last_updated = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }
    
    /// Add a new known sequence (discovered by Fate)
    pub fn add_known_sequence(&mut self, sequence: KnownSequence) {
        if !self.known_sequences.iter().any(|s| s.id == sequence.id) {
            self.known_sequences.push(sequence);
        }
    }
    
    /// Get pattern class by ID
    pub fn get_pattern_class(&self, id: &str) -> Option<&PatternClass> {
        self.pattern_classes.iter().find(|p| p.id == id)
    }
    
    /// Get known formula for a signal (if it matches a known sequence)
    /// For discovery criterion: compare S-wind found formula with known authoritative formula
    pub fn get_formula_for_sequence(&self, signal: &[crate::symbolic::EPR]) -> Option<String> {
        use crate::symbolic::EPR;
        
        // Try to match known sequences
        for seq in &self.known_sequences {
            if seq.first_terms.len() >= signal.len() && signal.len() >= 3 {
                // Convert i128 to EPR for comparison
                let matches = signal.iter()
                    .zip(seq.first_terms.iter())
                    .all(|(a, b)| {
                        let b_epr = EPR::from_i128(*b);
                        *a == b_epr
                    });
                    
                if matches {
                    // Return known formula
                    return seq.closed_form.clone()
                        .or_else(|| seq.recurrence.clone())
                        .or_else(|| Some(format!("{} sequence", seq.name)));
                }
            }
        }
        
        None
    }
    
    /// Add a discovered equation (automatically marked as PendingReview)
    pub fn add_discovery(&mut self, equation_tex: String, description: String, derivation_tex: String) -> String {
        let id = format!("discovered_{}", self.discovered.len());
        let discovery = DiscoveredEquation {
            id: id.clone(),
            equation_tex,
            description,
            discovered_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            derivation_tex,
            verification_count: 0,
            status: KnowledgeStatus::PendingReview,
        };
        self.discovered.push(discovery);
        self.last_updated = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        id
    }
    
    /// Search for a constant by symbol or name
    pub fn find_constant(&self, query: &str) -> Option<&PhysicalConstant> {
        self.physical_constants.iter()
            .find(|c| c.symbol == query || c.name.to_lowercase().contains(&query.to_lowercase()))
    }
    
    /// Search for a theorem by name
    pub fn find_theorem(&self, query: &str) -> Option<&MathTheorem> {
        self.theorems.iter()
            .find(|t| t.name.to_lowercase().contains(&query.to_lowercase()))
    }
    
    /// Get all pending review items
    pub fn get_pending_review(&self) -> Vec<&DiscoveredEquation> {
        self.discovered.iter()
            .filter(|d| d.status == KnowledgeStatus::PendingReview)
            .collect()
    }
    
    /// Mark a discovery as verified
    pub fn verify_discovery(&mut self, id: &str) -> bool {
        if let Some(d) = self.discovered.iter_mut().find(|d| d.id == id) {
            d.status = KnowledgeStatus::Verified;
            true
        } else {
            false
        }
    }
    
    /// Statistics
    pub fn stats(&self) -> KnowledgeStats {
        KnowledgeStats {
            physical_constants: self.physical_constants.len(),
            math_constants: self.math_constants.len(),
            theorems: self.theorems.len(),
            physical_laws: self.physical_laws.len(),
            discovered: self.discovered.len(),
            pending_review: self.discovered.iter().filter(|d| d.status == KnowledgeStatus::PendingReview).count(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KnowledgeStats {
    pub physical_constants: usize,
    pub math_constants: usize,
    pub theorems: usize,
    pub physical_laws: usize,
    pub discovered: usize,
    pub pending_review: usize,
}

impl std::fmt::Display for KnowledgeStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Knowledge Base Statistics:")?;
        writeln!(f, "  Physical constants: {}", self.physical_constants)?;
        writeln!(f, "  Math constants: {}", self.math_constants)?;
        writeln!(f, "  Theorems: {}", self.theorems)?;
        writeln!(f, "  Physical laws: {}", self.physical_laws)?;
        writeln!(f, "  Discovered: {} ({} pending review)", self.discovered, self.pending_review)?;
        write!(f, "  Total: {}", self.physical_constants + self.math_constants + self.theorems + self.physical_laws + self.discovered)
    }
}
