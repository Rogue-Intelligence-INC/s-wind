//! Pattern Class Library
//!
//! All mathematical pattern classes and their detection methods.
//! Fate loads these from KnowledgeBase, never hardcodes them.

use serde::{Deserialize, Serialize};

/// Pattern class definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternClass {
    pub id: String,
    pub name: String,
    pub description: String,
    /// Symbolic form: e.g., "a*n + b" for linear
    pub symbolic_form: String,
    /// Detection method
    pub detection: DetectionMethod,
    /// Required minimum samples
    pub min_samples: usize,
    /// Computational complexity estimate
    pub complexity: Complexity,
    /// Priority (lower = try first). Fate uses this to order exploration.
    pub priority: u32,
}

/// Detection method - how to detect this pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectionMethod {
    /// Constant k-th difference
    ConstantDifference { order: usize },
    /// Constant ratio s[n+1]/s[n]
    ConstantRatio,
    /// Constant product s[n] * f(n)
    ConstantProduct { factor_form: String },
    /// Recurrence relation
    Recurrence { order: usize },
    /// Periodic with period p
    Periodic,
    /// Factorial: s[n]/s[n-1] = n
    Factorial,
    /// Logarithmic pattern
    Logarithmic,
    /// Custom detection function (stored as expression)
    CustomExpr { test_expr: String },
    /// Custom detection based on discovered structure
    Custom { method: crate::fate::StructureType },
}

/// Complexity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Complexity {
    Constant,   // O(1)
    Linear,     // O(n)
    Quadratic,  // O(n²)
    Polynomial, // O(n^k)
    Exponential,// O(2^n)
}

/// Known sequence from OEIS or other sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownSequence {
    pub id: String,           // e.g., "A000045" for Fibonacci
    pub name: String,
    pub first_terms: Vec<i128>, // First terms as integers (for matching)
    pub pattern_class: Option<String>, // Which pattern class it belongs to
    pub recurrence: Option<String>,    // Recurrence formula (symbolic)
    pub closed_form: Option<String>,   // Closed form (symbolic)
    pub generating_function: Option<String>,
    pub description: String,
}

/// Initialize all pattern classes
/// Priority order: specific patterns first (reciprocal, exponential, periodic, recurrence)
/// then polynomial (from low to high degree)
pub fn get_default_pattern_classes() -> Vec<PatternClass> {
    vec![
        // === SPECIFIC PATTERNS (try first) ===
        
        // Reciprocal (try early - harmonic sequences)
        PatternClass {
            id: "reciprocal".into(),
            name: "Reciprocal".into(),
            description: "f(n) = c / (n + k)".into(),
            symbolic_form: "c / (n + k)".into(),
            detection: DetectionMethod::ConstantProduct { factor_form: "n + k".into() },
            min_samples: 2,
            complexity: Complexity::Linear,
            priority: 10,  // High priority
        },
        
        // Exponential (try early - geometric sequences)
        PatternClass {
            id: "exponential".into(),
            name: "Exponential".into(),
            description: "f(n) = a * r^n".into(),
            symbolic_form: "a * r^n".into(),
            detection: DetectionMethod::ConstantRatio,
            min_samples: 3,
            complexity: Complexity::Linear,
            priority: 20,
        },
        
        // Periodic (try early - repeating patterns)
        PatternClass {
            id: "periodic".into(),
            name: "Periodic".into(),
            description: "f(n) = f(n mod p) for some period p".into(),
            symbolic_form: "f(n mod p)".into(),
            detection: DetectionMethod::Periodic,
            min_samples: 4,
            complexity: Complexity::Quadratic,
            priority: 30,
        },
        
        // Recurrence (order 2) - Fibonacci, Lucas, etc.
        // Priority after polynomial because polynomial is more interpretable
        PatternClass {
            id: "recurrence_2".into(),
            name: "Linear Recurrence (order 2)".into(),
            description: "f(n) = c₁*f(n-1) + c₂*f(n-2)".into(),
            symbolic_form: "c1*f(n-1) + c2*f(n-2)".into(),
            detection: DetectionMethod::Recurrence { order: 2 },
            min_samples: 4,
            complexity: Complexity::Linear,
            priority: 150,  // After polynomial patterns
        },
        
        // Recurrence (order 3)
        PatternClass {
            id: "recurrence_3".into(),
            name: "Linear Recurrence (order 3)".into(),
            description: "f(n) = c₁*f(n-1) + c₂*f(n-2) + c₃*f(n-3)".into(),
            symbolic_form: "c1*f(n-1) + c2*f(n-2) + c3*f(n-3)".into(),
            detection: DetectionMethod::Recurrence { order: 3 },
            min_samples: 5,
            complexity: Complexity::Linear,
            priority: 160,  // After polynomial patterns
        },
        
        // === POLYNOMIAL PATTERNS (try after specific patterns) ===
        
        // Constant (simplest polynomial)
        PatternClass {
            id: "constant".into(),
            name: "Constant".into(),
            description: "f(n) = c for all n".into(),
            symbolic_form: "c".into(),
            detection: DetectionMethod::ConstantDifference { order: 0 },  // Order 0 = constant
            min_samples: 2,
            complexity: Complexity::Constant,
            priority: 100,
        },
        
        // Linear (degree 1)
        PatternClass {
            id: "linear".into(),
            name: "Linear Polynomial".into(),
            description: "f(n) = a*n + b (degree 1)".into(),
            symbolic_form: "a*n + b".into(),
            detection: DetectionMethod::ConstantDifference { order: 1 },
            min_samples: 2,
            complexity: Complexity::Linear,
            priority: 110,
        },
        
        // Quadratic (degree 2)
        PatternClass {
            id: "quadratic".into(),
            name: "Quadratic Polynomial".into(),
            description: "f(n) = a*n² + b*n + c (degree 2)".into(),
            symbolic_form: "a*n^2 + b*n + c".into(),
            detection: DetectionMethod::ConstantDifference { order: 2 },
            min_samples: 3,
            complexity: Complexity::Linear,
            priority: 120,
        },
        
        // Cubic (degree 3)
        PatternClass {
            id: "cubic".into(),
            name: "Cubic Polynomial".into(),
            description: "f(n) = a*n³ + b*n² + c*n + d (degree 3)".into(),
            symbolic_form: "a*n^3 + b*n^2 + c*n + d".into(),
            detection: DetectionMethod::ConstantDifference { order: 3 },
            min_samples: 4,
            complexity: Complexity::Linear,
            priority: 130,
        },
        
        // Factorial
        PatternClass {
            id: "factorial".into(),
            name: "Factorial".into(),
            description: "f(n) = n!".into(),
            symbolic_form: "n!".into(),
            detection: DetectionMethod::Factorial,
            min_samples: 3,
            complexity: Complexity::Linear,
            priority: 35,  // Try early, before polynomial
        },
        
        // General Polynomial (fallback, auto-detect order)
        PatternClass {
            id: "polynomial".into(),
            name: "General Polynomial".into(),
            description: "f(n) = Σ aᵢnⁱ (degree k, auto-detect)".into(),
            symbolic_form: "sum(a_i * n^i, i=0..k)".into(),
            detection: DetectionMethod::ConstantDifference { order: 99 }, // 99 = auto-detect
            min_samples: 2,
            complexity: Complexity::Linear,
            priority: 200,  // Last resort
        },
    ]
}

/// Initialize known sequences (subset of OEIS)
pub fn get_known_sequences() -> Vec<KnownSequence> {
    vec![
        // === FIBONACCI FAMILY ===
        KnownSequence {
            id: "A000045".into(),
            name: "Fibonacci numbers".into(),
            first_terms: vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144],
            pattern_class: Some("recurrence_2".into()),
            recurrence: Some("f(n) = f(n-1) + f(n-2)".into()),
            closed_form: Some("(phi^n - psi^n) / sqrt(5)".into()),
            generating_function: Some("x / (1 - x - x^2)".into()),
            description: "Fibonacci sequence".into(),
        },
        KnownSequence {
            id: "A000045_alt".into(),
            name: "Fibonacci numbers (starts 1,1)".into(),
            first_terms: vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144],
            pattern_class: Some("recurrence_2".into()),
            recurrence: Some("f(n) = f(n-1) + f(n-2)".into()),
            closed_form: Some("(phi^n - psi^n) / sqrt(5)".into()),
            generating_function: Some("x / (1 - x - x^2)".into()),
            description: "Fibonacci sequence (alternative index)".into(),
        },
        KnownSequence {
            id: "A000032".into(),
            name: "Lucas numbers".into(),
            first_terms: vec![2, 1, 3, 4, 7, 11, 18, 29, 47, 76, 123],
            pattern_class: Some("recurrence_2".into()),
            recurrence: Some("f(n) = f(n-1) + f(n-2)".into()),
            closed_form: Some("phi^n + psi^n".into()),
            generating_function: Some("(2-x) / (1-x-x^2)".into()),
            description: "Lucas numbers (Fibonacci cousin)".into(),
        },
        KnownSequence {
            id: "A000073".into(),
            name: "Tribonacci numbers".into(),
            first_terms: vec![0, 0, 1, 1, 2, 4, 7, 13, 24, 44, 81, 149],
            pattern_class: Some("recurrence_3".into()),
            recurrence: Some("f(n) = f(n-1) + f(n-2) + f(n-3)".into()),
            closed_form: None,
            generating_function: Some("x^2 / (1-x-x^2-x^3)".into()),
            description: "Tribonacci numbers".into(),
        },
        
        // === PRIME NUMBERS ===
        KnownSequence {
            id: "A000040".into(),
            name: "Prime numbers".into(),
            first_terms: vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47],
            pattern_class: None,
            recurrence: None,
            closed_form: None,
            generating_function: None,
            description: "Prime numbers (no polynomial formula exists)".into(),
        },
        KnownSequence {
            id: "A001359".into(),
            name: "Twin prime pairs (lesser)".into(),
            first_terms: vec![3, 5, 11, 17, 29, 41, 59, 71, 101, 107],
            pattern_class: None,
            recurrence: None,
            closed_form: None,
            generating_function: None,
            description: "Lesser of twin prime pairs".into(),
        },
        
        // === POWERS AND EXPONENTIALS ===
        KnownSequence {
            id: "A000079".into(),
            name: "Powers of 2".into(),
            first_terms: vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024],
            pattern_class: Some("exponential".into()),
            recurrence: Some("f(n) = 2 * f(n-1)".into()),
            closed_form: Some("2^n".into()),
            generating_function: Some("1 / (1 - 2x)".into()),
            description: "Powers of 2".into(),
        },
        KnownSequence {
            id: "A000244".into(),
            name: "Powers of 3".into(),
            first_terms: vec![1, 3, 9, 27, 81, 243, 729, 2187, 6561],
            pattern_class: Some("exponential".into()),
            recurrence: Some("f(n) = 3 * f(n-1)".into()),
            closed_form: Some("3^n".into()),
            generating_function: Some("1 / (1 - 3x)".into()),
            description: "Powers of 3".into(),
        },
        
        // === POLYNOMIAL SEQUENCES ===
        KnownSequence {
            id: "A000290".into(),
            name: "Square numbers".into(),
            first_terms: vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100],
            pattern_class: Some("quadratic".into()),
            recurrence: None,
            closed_form: Some("n^2".into()),
            generating_function: Some("x*(1+x)/(1-x)^3".into()),
            description: "Perfect squares".into(),
        },
        KnownSequence {
            id: "A000290_alt".into(),
            name: "Square numbers (1-indexed)".into(),
            first_terms: vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100],
            pattern_class: Some("quadratic".into()),
            recurrence: None,
            closed_form: Some("n^2".into()),
            generating_function: None,
            description: "Perfect squares starting from 1".into(),
        },
        KnownSequence {
            id: "A000578".into(),
            name: "Cubes".into(),
            first_terms: vec![0, 1, 8, 27, 64, 125, 216, 343, 512, 729],
            pattern_class: Some("cubic".into()),
            recurrence: None,
            closed_form: Some("n^3".into()),
            generating_function: Some("x*(1+4x+x^2)/(1-x)^4".into()),
            description: "Perfect cubes".into(),
        },
        KnownSequence {
            id: "A000578_alt".into(),
            name: "Cubes (1-indexed)".into(),
            first_terms: vec![1, 8, 27, 64, 125, 216, 343, 512, 729, 1000],
            pattern_class: Some("cubic".into()),
            recurrence: None,
            closed_form: Some("n^3".into()),
            generating_function: None,
            description: "Perfect cubes starting from 1".into(),
        },
        KnownSequence {
            id: "A000217".into(),
            name: "Triangular numbers".into(),
            first_terms: vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55],
            pattern_class: Some("quadratic".into()),
            recurrence: Some("f(n) = f(n-1) + n".into()),
            closed_form: Some("n*(n+1)/2".into()),
            generating_function: Some("x / (1-x)^3".into()),
            description: "Triangular numbers".into(),
        },
        KnownSequence {
            id: "A000217_alt".into(),
            name: "Triangular numbers (1-indexed)".into(),
            first_terms: vec![1, 3, 6, 10, 15, 21, 28, 36, 45, 55],
            pattern_class: Some("quadratic".into()),
            recurrence: Some("f(n) = f(n-1) + n".into()),
            closed_form: Some("n*(n+1)/2".into()),
            generating_function: None,
            description: "Triangular numbers starting from 1".into(),
        },
        KnownSequence {
            id: "A000326".into(),
            name: "Pentagonal numbers".into(),
            first_terms: vec![0, 1, 5, 12, 22, 35, 51, 70, 92, 117, 145],
            pattern_class: Some("quadratic".into()),
            recurrence: None,
            closed_form: Some("n*(3n-1)/2".into()),
            generating_function: Some("x*(1+2x)/(1-x)^3".into()),
            description: "Pentagonal numbers".into(),
        },
        
        // === FACTORIAL AND COMBINATORIAL ===
        KnownSequence {
            id: "A000142".into(),
            name: "Factorial".into(),
            first_terms: vec![1, 1, 2, 6, 24, 120, 720, 5040, 40320],
            pattern_class: Some("factorial".into()),
            recurrence: Some("f(n) = n * f(n-1)".into()),
            closed_form: Some("n!".into()),
            generating_function: None,
            description: "Factorial numbers".into(),
        },
        KnownSequence {
            id: "A000110".into(),
            name: "Bell numbers".into(),
            first_terms: vec![1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147],
            pattern_class: None,
            recurrence: Some("B(n+1) = sum(C(n,k)*B(k), k=0..n)".into()),
            closed_form: None,
            generating_function: None,
            description: "Bell numbers (partitions of set)".into(),
        },
        KnownSequence {
            id: "A000108".into(),
            name: "Catalan numbers".into(),
            first_terms: vec![1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862],
            pattern_class: None,
            recurrence: Some("C(n+1) = sum(C(i)*C(n-i), i=0..n)".into()),
            closed_form: Some("C(2n,n)/(n+1)".into()),
            generating_function: Some("(1-sqrt(1-4x))/(2x)".into()),
            description: "Catalan numbers".into(),
        },
        
        // === HARMONIC AND RECIPROCAL ===
        KnownSequence {
            id: "HARMONIC".into(),
            name: "Harmonic sequence (unit fractions)".into(),
            first_terms: vec![], // Can't store fractions here, use closed_form for matching
            pattern_class: Some("reciprocal".into()),
            recurrence: None,
            closed_form: Some("1 / n".into()),
            generating_function: None,
            description: "Harmonic sequence 1, 1/2, 1/3, 1/4, ...".into(),
        },
        KnownSequence {
            id: "HARMONIC_OFFSET".into(),
            name: "Harmonic offset sequence".into(),
            first_terms: vec![], // Can't store fractions here
            pattern_class: Some("reciprocal".into()),
            recurrence: None,
            closed_form: Some("1 / (n + 1)".into()),
            generating_function: None,
            description: "Harmonic sequence with offset: 1/2, 1/3, 1/4, ...".into(),
        },
        
        // === SPECIAL CONSTANTS ===
        KnownSequence {
            id: "PI_DIGITS".into(),
            name: "Digits of pi".into(),
            first_terms: vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9],
            pattern_class: None,
            recurrence: None,
            closed_form: None,
            generating_function: None,
            description: "Decimal digits of pi (believed normal, no pattern)".into(),
        },
        KnownSequence {
            id: "E_DIGITS".into(),
            name: "Digits of e".into(),
            first_terms: vec![2, 7, 1, 8, 2, 8, 1, 8, 2, 8, 4, 5, 9, 0, 4],
            pattern_class: None,
            recurrence: None,
            closed_form: None,
            generating_function: None,
            description: "Decimal digits of e (believed normal, no pattern)".into(),
        },
        
        // === MISC ===
        KnownSequence {
            id: "A001477".into(),
            name: "Natural numbers".into(),
            first_terms: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12],
            pattern_class: Some("linear".into()),
            recurrence: Some("f(n) = f(n-1) + 1".into()),
            closed_form: Some("n".into()),
            generating_function: Some("x/(1-x)^2".into()),
            description: "Natural numbers (non-negative integers)".into(),
        },
        KnownSequence {
            id: "A005408".into(),
            name: "Odd numbers".into(),
            first_terms: vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21],
            pattern_class: Some("linear".into()),
            recurrence: Some("f(n) = f(n-1) + 2".into()),
            closed_form: Some("2n+1".into()),
            generating_function: Some("(1+x)/(1-x)^2".into()),
            description: "Odd numbers".into(),
        },
        KnownSequence {
            id: "A005843".into(),
            name: "Even numbers".into(),
            first_terms: vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20],
            pattern_class: Some("linear".into()),
            recurrence: Some("f(n) = f(n-1) + 2".into()),
            closed_form: Some("2n".into()),
            generating_function: Some("2x/(1-x)^2".into()),
            description: "Even numbers".into(),
        },
        
        // === ADDITIONAL IMPORTANT SEQUENCES ===
        
        // Partition numbers
        KnownSequence {
            id: "A000041".into(),
            name: "Partition numbers".into(),
            first_terms: vec![1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135],
            pattern_class: None,
            recurrence: Some("Hardy-Ramanujan-Rademacher formula".into()),
            closed_form: None,
            generating_function: Some("prod(1/(1-x^k), k=1..inf)".into()),
            description: "Number of partitions of n".into(),
        },
        
        // Mersenne primes exponents
        KnownSequence {
            id: "A000043".into(),
            name: "Mersenne prime exponents".into(),
            first_terms: vec![2, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127, 521],
            pattern_class: None,
            recurrence: None,
            closed_form: None,
            generating_function: None,
            description: "Exponents p such that 2^p - 1 is prime".into(),
        },
        
        // Perfect numbers
        KnownSequence {
            id: "A000396".into(),
            name: "Perfect numbers".into(),
            first_terms: vec![6, 28, 496, 8128],
            pattern_class: None,
            recurrence: None,
            closed_form: Some("2^(p-1) * (2^p - 1) when 2^p - 1 is prime".into()),
            generating_function: None,
            description: "Numbers equal to sum of proper divisors".into(),
        },
        
        // Fermat numbers
        KnownSequence {
            id: "A000215".into(),
            name: "Fermat numbers".into(),
            first_terms: vec![3, 5, 17, 257, 65537],
            pattern_class: Some("exponential".into()),
            recurrence: None,
            closed_form: Some("2^(2^n) + 1".into()),
            generating_function: None,
            description: "Fermat numbers F(n) = 2^(2^n) + 1".into(),
        },
        
        // Sophie Germain primes
        KnownSequence {
            id: "A005384".into(),
            name: "Sophie Germain primes".into(),
            first_terms: vec![2, 3, 5, 11, 23, 29, 41, 53, 83, 89, 113, 131],
            pattern_class: None,
            recurrence: None,
            closed_form: None,
            generating_function: None,
            description: "Primes p where 2p+1 is also prime".into(),
        },
        
        // Pell numbers
        KnownSequence {
            id: "A000129".into(),
            name: "Pell numbers".into(),
            first_terms: vec![0, 1, 2, 5, 12, 29, 70, 169, 408, 985, 2378],
            pattern_class: Some("recurrence_2".into()),
            recurrence: Some("f(n) = 2*f(n-1) + f(n-2)".into()),
            closed_form: Some("((1+sqrt(2))^n - (1-sqrt(2))^n) / (2*sqrt(2))".into()),
            generating_function: Some("x / (1 - 2x - x^2)".into()),
            description: "Pell numbers".into(),
        },
        
        // Jacobsthal numbers
        KnownSequence {
            id: "A001045".into(),
            name: "Jacobsthal numbers".into(),
            first_terms: vec![0, 1, 1, 3, 5, 11, 21, 43, 85, 171, 341],
            pattern_class: Some("recurrence_2".into()),
            recurrence: Some("f(n) = f(n-1) + 2*f(n-2)".into()),
            closed_form: Some("(2^n - (-1)^n) / 3".into()),
            generating_function: Some("x / ((1-2x)*(1+x))".into()),
            description: "Jacobsthal numbers".into(),
        },
        
        // Padovan sequence
        KnownSequence {
            id: "A000931".into(),
            name: "Padovan sequence".into(),
            first_terms: vec![1, 0, 0, 1, 0, 1, 1, 1, 2, 2, 3, 4, 5, 7, 9, 12],
            pattern_class: Some("recurrence_3".into()),
            recurrence: Some("f(n) = f(n-2) + f(n-3)".into()),
            closed_form: None,
            generating_function: Some("(1-x^2) / (1-x^2-x^3)".into()),
            description: "Padovan sequence (plastic constant analog of Fibonacci)".into(),
        },
        
        // Perrin sequence
        KnownSequence {
            id: "A001608".into(),
            name: "Perrin sequence".into(),
            first_terms: vec![3, 0, 2, 3, 2, 5, 5, 7, 10, 12, 17, 22, 29, 39],
            pattern_class: Some("recurrence_3".into()),
            recurrence: Some("f(n) = f(n-2) + f(n-3)".into()),
            closed_form: None,
            generating_function: Some("(3-x^2) / (1-x^2-x^3)".into()),
            description: "Perrin sequence (divisibility by n implies n is prime or pseudo)".into(),
        },
        
        // Central binomial coefficients
        KnownSequence {
            id: "A000984".into(),
            name: "Central binomial coefficients".into(),
            first_terms: vec![1, 2, 6, 20, 70, 252, 924, 3432, 12870, 48620],
            pattern_class: None,
            recurrence: None,
            closed_form: Some("C(2n, n) = (2n)! / (n!)^2".into()),
            generating_function: Some("1 / sqrt(1 - 4x)".into()),
            description: "C(2n,n) - central binomial coefficients".into(),
        },
        
        // Motzkin numbers
        KnownSequence {
            id: "A001006".into(),
            name: "Motzkin numbers".into(),
            first_terms: vec![1, 1, 2, 4, 9, 21, 51, 127, 323, 835, 2188],
            pattern_class: None,
            recurrence: Some("M(n) = M(n-1) + sum(M(i)*M(n-2-i), i=0..n-2)".into()),
            closed_form: None,
            generating_function: Some("(1 - x - sqrt(1 - 2x - 3x^2)) / (2x^2)".into()),
            description: "Motzkin numbers (paths that don't go below x-axis)".into(),
        },
        
        // Narayana numbers (first row)
        KnownSequence {
            id: "A001263".into(),
            name: "Narayana numbers (diagonal)".into(),
            first_terms: vec![1, 1, 1, 1, 3, 1, 1, 6, 6, 1, 1, 10, 20, 10, 1],
            pattern_class: None,
            recurrence: None,
            closed_form: Some("N(n,k) = C(n,k)*C(n,k-1)/n".into()),
            generating_function: None,
            description: "Narayana numbers - refinement of Catalan".into(),
        },
        
        // Bernoulli numerators
        KnownSequence {
            id: "A027641".into(),
            name: "Bernoulli numerators".into(),
            first_terms: vec![1, -1, 1, 0, -1, 0, 1, 0, -1, 0, 5, 0, -691],
            pattern_class: None,
            recurrence: None,
            closed_form: None,
            generating_function: Some("x / (e^x - 1)".into()),
            description: "Numerators of Bernoulli numbers".into(),
        },
        
        // Stirling numbers of 2nd kind (rows)
        KnownSequence {
            id: "A008277".into(),
            name: "Triangle of Stirling numbers of the second kind (by rows)".into(),
            first_terms: vec![1, 1, 1, 1, 3, 1, 1, 7, 6, 1, 1, 15, 25, 10, 1, 1],
            pattern_class: None,
            recurrence: Some("S(n,k) = k*S(n-1,k) + S(n-1,k-1)".into()),
            closed_form: None,
            generating_function: None,
            description: "Stirling numbers of the second kind".into(),
        },
        
        // Euler numbers
        KnownSequence {
            id: "A000111".into(),
            name: "Euler zigzag numbers".into(),
            first_terms: vec![1, 1, 1, 2, 5, 16, 61, 272, 1385, 7936, 50521],
            pattern_class: None,
            recurrence: None,
            closed_form: None,
            generating_function: Some("sec(x) + tan(x)".into()),
            description: "Euler zigzag numbers (alternating permutations)".into(),
        },
        
        // Lazy caterer's sequence
        KnownSequence {
            id: "A000124".into(),
            name: "Lazy caterer's sequence".into(),
            first_terms: vec![1, 2, 4, 7, 11, 16, 22, 29, 37, 46, 56],
            pattern_class: Some("quadratic".into()),
            recurrence: Some("f(n) = f(n-1) + n".into()),
            closed_form: Some("n*(n+1)/2 + 1".into()),
            generating_function: Some("(1-x+x^2)/(1-x)^3".into()),
            description: "Maximum pieces of circle cut by n lines".into(),
        },
        
        // Cake numbers (3D)
        KnownSequence {
            id: "A000125".into(),
            name: "Cake numbers".into(),
            first_terms: vec![1, 2, 4, 8, 15, 26, 42, 64, 93, 130, 176],
            pattern_class: Some("cubic".into()),
            recurrence: Some("f(n) = f(n-1) + n*(n+1)/2".into()),
            closed_form: Some("(n^3 + 5n + 6)/6".into()),
            generating_function: Some("(1+x)/(1-x)^4".into()),
            description: "Maximum pieces of 3D cake cut by n planes".into(),
        },
        
        // Derangements
        KnownSequence {
            id: "A000166".into(),
            name: "Derangements".into(),
            first_terms: vec![1, 0, 1, 2, 9, 44, 265, 1854, 14833, 133496],
            pattern_class: None,
            recurrence: Some("D(n) = (n-1)*(D(n-1) + D(n-2))".into()),
            closed_form: Some("n! * sum((-1)^k/k!, k=0..n)".into()),
            generating_function: Some("exp(-x)/(1-x)".into()),
            description: "Permutations with no fixed points".into(),
        },
        
        // Collatz-related (number of steps)
        KnownSequence {
            id: "A006577".into(),
            name: "Collatz stopping time".into(),
            first_terms: vec![0, 1, 7, 2, 5, 8, 16, 3, 19, 6, 14, 9, 9, 17, 17],
            pattern_class: None,
            recurrence: None,
            closed_form: None,
            generating_function: None,
            description: "Number of steps for n to reach 1 in Collatz sequence".into(),
        },
        
        // Recaman's sequence
        KnownSequence {
            id: "A005132".into(),
            name: "Recaman's sequence".into(),
            first_terms: vec![0, 1, 3, 6, 2, 7, 13, 20, 12, 21, 11, 22, 10, 23, 9],
            pattern_class: None,
            recurrence: Some("a(n) = a(n-1)-n if positive and new, else a(n-1)+n".into()),
            closed_form: None,
            generating_function: None,
            description: "Recaman's sequence - peculiar growth pattern".into(),
        },
        
        // Lucky numbers
        KnownSequence {
            id: "A000959".into(),
            name: "Lucky numbers".into(),
            first_terms: vec![1, 3, 7, 9, 13, 15, 21, 25, 31, 33, 37, 43, 49],
            pattern_class: None,
            recurrence: None,
            closed_form: None,
            generating_function: None,
            description: "Lucky numbers (sieve similar to Eratosthenes)".into(),
        },
        
        // Highly composite numbers
        KnownSequence {
            id: "A002182".into(),
            name: "Highly composite numbers".into(),
            first_terms: vec![1, 2, 4, 6, 12, 24, 36, 48, 60, 120, 180, 240, 360],
            pattern_class: None,
            recurrence: None,
            closed_form: None,
            generating_function: None,
            description: "Numbers with more divisors than any smaller positive integer".into(),
        },
        
        // Superior highly composite numbers
        KnownSequence {
            id: "A002201".into(),
            name: "Superior highly composite".into(),
            first_terms: vec![2, 6, 12, 60, 120, 360, 2520, 5040, 55440, 720720],
            pattern_class: None,
            recurrence: None,
            closed_form: None,
            generating_function: None,
            description: "Superior highly composite numbers (Ramanujan)".into(),
        },
    ]
}
/// Match a signal against known sequences
pub fn match_known_sequence(signal: &[i128]) -> Option<&'static str> {
    let sequences = get_known_sequences();
    
    for seq in &sequences {
        if seq.first_terms.len() >= signal.len() {
            let matches = signal.iter()
                .zip(seq.first_terms.iter())
                .all(|(a, b)| *a == *b);
            if matches {
                return Some(Box::leak(seq.id.clone().into_boxed_str()));
            }
        }
    }
    
    None
}
