//! S-wind Algebraic Conjecture Container System
//!
//! PURE ALGEBRAIC VERSION - ALL computation uses Rational/EPR
//! - NO floating point (f64)
//! - ALL comparisons are exact equality or rational bounds
//! - Auto-generates new conjecture containers for exhaustive exploration
//!
//! This is the CORRECT implementation aligned with S-wind's core philosophy.

use crate::symbolic::Rational;
use std::collections::HashMap;

// ============================================================================
// ALGEBRAIC VERIFICATION RESULT
// ============================================================================

/// Conjecture verification result (ALGEBRAIC)
#[derive(Debug, Clone)]
pub enum AlgebraicVerificationResult {
    /// Conjecture is EXACTLY verified (algebraic equality)
    ExactlyVerified { 
        evidence: Vec<String>, 
        /// Computed value
        computed: Rational,
        /// Expected value
        expected: Rational,
    },
    /// Conjecture is verified within rational tolerance
    ApproximatelyVerified { 
        evidence: Vec<String>,
        computed: Rational,
        expected: Rational,
        /// Relative error as rational
        error: Rational,
        /// Tolerance bound as rational
        tolerance: Rational,
    },
    /// Conjecture is EXACTLY falsified (algebraic inequality)
    Falsified { 
        counter_evidence: Vec<String>,
        computed: Rational,
        expected: Rational,
        difference: Rational,
    },
    /// Cannot decide (requires irrational/transcendental operations)
    Undecidable { reason: String },
}

// ============================================================================
// ALGEBRAIC CONJECTURE
// ============================================================================

/// Algebraic conjecture - all parameters are exact Rational
#[derive(Debug, Clone)]
pub struct AlgebraicConjecture {
    /// Unique identifier
    pub id: String,
    /// Human-readable statement
    pub statement: String,
    /// Mathematical formula as string
    pub formula: String,
    /// Parameters (all Rational)
    pub parameters: HashMap<String, Rational>,
    /// Constraints (all algebraic)
    pub constraints: Vec<AlgebraicConstraint>,
    /// Domain
    pub domain: AlgebraicDomain,
}

#[derive(Debug, Clone)]
pub enum AlgebraicDomain {
    PureMathematics,
    NumberTheory,
    Polynomial,
    GroupTheory,
    PhysicsRatioApproximation,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct AlgebraicConstraint {
    pub name: String,
    /// Expected value (exact Rational)
    pub expected: Rational,
    /// Tolerance as rational fraction (e.g., 1/100 = 1%)
    pub tolerance: Option<Rational>,
    /// Whether exact equality is required
    pub require_exact: bool,
}

// ============================================================================
// ALGEBRAIC EXPERIMENT CONTAINER (Auto-Generated)
// ============================================================================

/// Algebraic experiment container
pub struct AlgebraicContainer {
    /// Container name
    pub name: String,
    /// Container type
    pub container_type: ContainerType,
    /// Polynomial coefficients (for polynomial containers)
    pub polynomial_coeffs: Option<Vec<Rational>>,
    /// Computed outputs
    pub outputs: HashMap<String, Rational>,
    /// Computation log
    pub log: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ContainerType {
    /// Polynomial evaluation: P(D) = a_n*D^n + ... + a_0
    Polynomial { degree: usize },
    /// Rational function: P(D)/Q(D)
    RationalFunction { num_degree: usize, den_degree: usize },
    /// Linear combination of constants
    LinearCombination,
    /// Group dimension calculation
    GroupDimension { group: String },
}

impl AlgebraicContainer {
    /// Create polynomial container
    pub fn new_polynomial(name: &str, coefficients: Vec<Rational>) -> Self {
        let degree = coefficients.len().saturating_sub(1);
        AlgebraicContainer {
            name: name.to_string(),
            container_type: ContainerType::Polynomial { degree },
            polynomial_coeffs: Some(coefficients),
            outputs: HashMap::new(),
            log: vec![format!("Created polynomial container: {}", name)],
        }
    }
    
    /// Evaluate polynomial at rational point D
    pub fn evaluate_polynomial(&mut self, d: &Rational) -> Rational {
        if let Some(coeffs) = &self.polynomial_coeffs {
            let mut result = Rational::zero();
            let mut d_power = Rational::one();
            
            // P(D) = a_0 + a_1*D + a_2*D^2 + ... + a_n*D^n
            for (i, coeff) in coeffs.iter().enumerate() {
                result = result + *coeff * d_power;
                d_power = d_power * *d;
                self.log.push(format!("  Term {}: coeff={}, D^{}={}", i, coeff, i, d_power));
            }
            
            self.outputs.insert("P(D)".to_string(), result);
            self.log.push(format!("P({}) = {}", d, result));
            result
        } else {
            Rational::zero()
        }
    }
    
    /// Evaluate polynomial at integer D
    pub fn evaluate_at_int(&mut self, d: i128) -> Rational {
        self.evaluate_polynomial(&Rational::from_i128(d))
    }
}

// ============================================================================
// ALGEBRAIC CONJECTURE VERIFIER
// ============================================================================

/// Algebraic conjecture verifier - uses ONLY exact computation
pub struct AlgebraicVerifier {
    /// Active containers (auto-generated)
    containers: HashMap<String, AlgebraicContainer>,
    /// Tested conjectures
    tested: Vec<(AlgebraicConjecture, AlgebraicVerificationResult)>,
    /// Verification log
    pub log: Vec<String>,
    /// Statistics
    pub stats: VerifierStats,
}

#[derive(Debug, Clone, Default)]
pub struct VerifierStats {
    pub total_tested: usize,
    pub exactly_verified: usize,
    pub approximately_verified: usize,
    pub falsified: usize,
    pub undecidable: usize,
    pub containers_created: usize,
}

impl AlgebraicVerifier {
    pub fn new() -> Self {
        AlgebraicVerifier {
            containers: HashMap::new(),
            tested: Vec::new(),
            log: vec!["AlgebraicVerifier initialized (EPR mode)".to_string()],
            stats: VerifierStats::default(),
        }
    }
    
    /// AUTO-CREATE container from conjecture (S-wind core capability)
    pub fn auto_create_container(&mut self, conjecture: &AlgebraicConjecture) -> String {
        let container_name = format!("auto_{}", conjecture.id);
        
        self.log.push(format!("AUTO-CREATING container: {}", container_name));
        
        // Determine container type from conjecture domain
        let container = match &conjecture.domain {
            AlgebraicDomain::Polynomial => {
                // Extract polynomial coefficients from parameters
                let mut coeffs = Vec::new();
                for i in 0..10 {
                    let key = format!("a{}", i);
                    if let Some(&coeff) = conjecture.parameters.get(&key) {
                        coeffs.push(coeff);
                    } else {
                        break;
                    }
                }
                if coeffs.is_empty() {
                    coeffs = vec![Rational::one()]; // Default: P(D) = 1
                }
                AlgebraicContainer::new_polynomial(&container_name, coeffs)
            }
            AlgebraicDomain::GroupTheory => {
                // Create group dimension container
                AlgebraicContainer {
                    name: container_name.clone(),
                    container_type: ContainerType::GroupDimension { group: "E7".to_string() },
                    polynomial_coeffs: None,
                    outputs: HashMap::new(),
                    log: vec!["Group theory container".to_string()],
                }
            }
            _ => {
                // Default: polynomial container
                AlgebraicContainer::new_polynomial(&container_name, vec![Rational::one()])
            }
        };
        
        self.containers.insert(container_name.clone(), container);
        self.stats.containers_created += 1;
        self.log.push(format!("Container created: {} (type: {:?})", 
            container_name, 
            self.containers.get(&container_name).map(|c| &c.container_type)
        ));
        
        container_name
    }
    
    /// Verify conjecture (ALGEBRAIC)
    pub fn verify(&mut self, conjecture: &AlgebraicConjecture) -> AlgebraicVerificationResult {
        self.stats.total_tested += 1;
        self.log.push(format!("=== Testing: {} ===", conjecture.id));
        self.log.push(format!("Statement: {}", conjecture.statement));
        
        // Auto-create container if needed
        let container_name = self.auto_create_container(conjecture);
        
        // Get D value from parameters
        let d = conjecture.parameters.get("D")
            .copied()
            .unwrap_or_else(|| Rational::from_i128(4));
        
        // Compute result
        let computed = if let Some(container) = self.containers.get_mut(&container_name) {
            container.evaluate_polynomial(&d)
        } else {
            return AlgebraicVerificationResult::Undecidable { 
                reason: "Container not found".to_string() 
            };
        };
        
        // Check constraints
        for constraint in &conjecture.constraints {
            let expected = constraint.expected;
            
            if constraint.require_exact {
                // EXACT equality check
                if computed == expected {
                    self.stats.exactly_verified += 1;
                    let result = AlgebraicVerificationResult::ExactlyVerified {
                        evidence: vec![format!("P({}) = {} = {} (EXACT)", d, computed, expected)],
                        computed,
                        expected,
                    };
                    self.tested.push((conjecture.clone(), result.clone()));
                    return result;
                } else {
                    self.stats.falsified += 1;
                    let diff = if computed > expected {
                        computed - expected
                    } else {
                        expected - computed
                    };
                    let result = AlgebraicVerificationResult::Falsified {
                        counter_evidence: vec![format!("P({}) = {} ≠ {} (diff: {})", d, computed, expected, diff)],
                        computed,
                        expected,
                        difference: diff,
                    };
                    self.tested.push((conjecture.clone(), result.clone()));
                    return result;
                }
            } else if let Some(tol) = constraint.tolerance {
                // Check within tolerance
                let diff = if computed > expected {
                    computed - expected
                } else {
                    expected - computed
                };
                let rel_error = diff / expected.abs();
                
                if rel_error < tol {
                    self.stats.approximately_verified += 1;
                    let result = AlgebraicVerificationResult::ApproximatelyVerified {
                        evidence: vec![format!("P({}) = {} ≈ {} (error: {} < {})", d, computed, expected, rel_error, tol)],
                        computed,
                        expected,
                        error: rel_error,
                        tolerance: tol,
                    };
                    self.tested.push((conjecture.clone(), result.clone()));
                    return result;
                } else {
                    self.stats.falsified += 1;
                    let result = AlgebraicVerificationResult::Falsified {
                        counter_evidence: vec![format!("P({}) = {} ≠ {} (error: {} > {})", d, computed, expected, rel_error, tol)],
                        computed,
                        expected,
                        difference: diff,
                    };
                    self.tested.push((conjecture.clone(), result.clone()));
                    return result;
                }
            }
        }
        
        self.stats.undecidable += 1;
        AlgebraicVerificationResult::Undecidable { 
            reason: "No constraints to check".to_string() 
        }
    }
    
    /// AUTO-GENERATE conjectures from template (EXHAUSTIVE EXPLORATION)
    pub fn auto_generate_polynomial_conjectures(
        &self,
        template_id: &str,
        coeff_range: std::ops::RangeInclusive<i128>,
        d_range: std::ops::RangeInclusive<i128>,
        target: Rational,
        require_exact: bool,
    ) -> Vec<AlgebraicConjecture> {
        let mut generated = Vec::new();
        
        // Generate all monic quartic polynomials within range
        for a3 in coeff_range.clone() {
            for a2 in coeff_range.clone() {
                for a1 in coeff_range.clone() {
                    for a0 in coeff_range.clone() {
                        for d in d_range.clone() {
                            let mut params = HashMap::new();
                            params.insert("a0".to_string(), Rational::from_i128(a0));
                            params.insert("a1".to_string(), Rational::from_i128(a1));
                            params.insert("a2".to_string(), Rational::from_i128(a2));
                            params.insert("a3".to_string(), Rational::from_i128(a3));
                            params.insert("a4".to_string(), Rational::one()); // monic
                            params.insert("D".to_string(), Rational::from_i128(d));
                            
                            let conj = AlgebraicConjecture {
                                id: format!("{}_{}_{}_{}_{}_D{}", template_id, a3, a2, a1, a0, d),
                                statement: format!("D⁴+{}D³+{}D²+{}D+{} = {} at D={}", a3, a2, a1, a0, target, d),
                                formula: format!("D^4 + {}*D^3 + {}*D^2 + {}*D + {}", a3, a2, a1, a0),
                                parameters: params,
                                constraints: vec![
                                    AlgebraicConstraint {
                                        name: "P(D)".to_string(),
                                        expected: target,
                                        tolerance: if require_exact { None } else { Some(Rational::new(1, 100)) },
                                        require_exact,
                                    }
                                ],
                                domain: AlgebraicDomain::Polynomial,
                            };
                            generated.push(conj);
                        }
                    }
                }
            }
        }
        
        generated
    }
    
    /// Batch test conjectures (EXHAUSTIVE)
    pub fn batch_test(&mut self, conjectures: &[AlgebraicConjecture]) -> Vec<(String, AlgebraicVerificationResult)> {
        self.log.push(format!("=== BATCH TEST: {} conjectures ===", conjectures.len()));
        
        let mut results = Vec::new();
        for conj in conjectures {
            let result = self.verify(conj);
            results.push((conj.id.clone(), result));
        }
        
        self.log.push(format!("Batch complete: {} tested", results.len()));
        results
    }
    
    /// Get summary
    pub fn summary(&self) -> String {
        format!(
            "=== Algebraic Verifier Summary ===\n\
             Total tested: {}\n\
             Exactly verified: {}\n\
             Approximately verified: {}\n\
             Falsified: {}\n\
             Undecidable: {}\n\
             Containers created: {}",
            self.stats.total_tested,
            self.stats.exactly_verified,
            self.stats.approximately_verified,
            self.stats.falsified,
            self.stats.undecidable,
            self.stats.containers_created
        )
    }
    
    /// Get exactly verified conjectures
    pub fn get_verified(&self) -> Vec<&AlgebraicConjecture> {
        self.tested.iter()
            .filter(|(_, r)| matches!(r, 
                AlgebraicVerificationResult::ExactlyVerified { .. } |
                AlgebraicVerificationResult::ApproximatelyVerified { .. }
            ))
            .map(|(c, _)| c)
            .collect()
    }
}

impl Default for AlgebraicVerifier {
    fn default() -> Self {
        Self::new()
    }
}
