//! S-wind: 完整精确代数模拟框架
//!
//! # 核心原则
//!
//! 1. **代数优先**: 所有计算尽可能保持符号形式
//! 2. **精度可控**: EPR 用于精确计算，f64 仅用于最终验证
//! 3. **符号计算**: 代数表达式包含有理数、代数数、超越数
//! 4. **完整知识库**: 物理/数学常数、定理、定律、已知序列
//!
//! # 代数系统层次
//!
//! - Rational (i128/i128): 精确有理数
//! - EPR: 无限精度有理数
//! - Algebraic: 代数数 (√n, φ)
//! - Transcendental: 超越数 (π, e, ln)
//! - AlgebraicExpr: 统一符号表达式
//!
//! # 架构
//!
//! ```text
//!                    ┌─────────────────┐
//!                    │      Fate       │  ← 统一探索引擎 (EPR)
//!                    └────────┬────────┘
//!                             │
//!          ┌──────────────────┼──────────────────┐
//!          │                  │                  │
//!    ┌─────▼─────┐     ┌─────▼─────┐     ┌─────▼─────┐
//!    │   Quad    │     │   Space   │     │  Voyager  │
//!    │ (物理系统) │     │  (空间)   │     │ (模式检测) │
//!    └───────────┘     └───────────┘     └───────────┘
//!          │                  │                  │
//!          └──────────────────┼──────────────────┘
//!                             │
//!                    ┌────────▼────────┐
//!                    │  KnowledgeBase  │  ← 完整知识库
//!                    └─────────────────┘
//! ```

pub mod symbolic;
pub mod knowledge;
pub mod fate;
pub mod quad;
pub mod space;
pub mod experiments;
pub mod interface;
pub mod blondle;
pub mod conjecture;
pub mod conjecture_algebraic;
pub mod search;

// Re-export Voyager
pub use voyager::{
    PreciseSignalDetector, DimensionManager, DimensionConfig,
    SignalEquation, EquationType, MathematicalProcessingResult,
    Rational as VoyagerRational,
};

// === 核心导出 ===

// 精确算术 (EPR 为主)
pub use symbolic::{Rational, EPR, ExtendedInt, Expr, Symbol, PhysicsConst};

// 代数系统 (支持无理数和超越数)
pub use symbolic::{
    AlgebraicBase, AlgebraicTerm, AlgebraicExpr, 
    AlgebraicBuilder, AlgebraicDiscovery
};

// 代数搜索引擎
pub use search::{
    AlgebraicSearchEngine, PhysicalConstant,
    constants as physical_constants,
};

// Unified Fate (EPR infinite precision)
pub use fate::{
    Fate, FateConfig, FateStats,
    Signal, SignalSource,
    ExplorationResult as FateResult,
    ExplorationStatus as FateStatus,
    Discovery, DiscoveryType,
    DiscoverySummary, NovelDiscoveryRecord,
    // Fit level classification (academic rigor)
    FitLevel, FitAnalysis, CrossDomainVerification,
    // Three-layer discovery assessment
    DiscoveryAssessment, OeisStatus, DiscoveryStatus,
    // Self-loop statistics
    SelfLoopStats, FinalStatus, FullExplorationResult,
};

// 知识库
pub use knowledge::{
    KnowledgeBase, KnowledgeStatus,
    PatternClass, DetectionMethod, Complexity,
    KnownSequence,
    oeis::{OeisLoader, OeisSequence, get_builtin_oeis},
};

// 物理系统
pub use quad::{
    GravitationalField, GravitationalFieldConfig, GravitationalConstants, MassDistribution,
    QuantumFieldSystem, QuantumFieldConfig, QuantumFieldSignal, FieldExcitationType,
    ThermalRadiationProcessor, ThermalRadiationConfig, ThermalRadiationEvent,
    FundamentalCouplingCoordinator, CouplingConfig, CouplingResult,
    GravitationalCoupling, ElectromagneticCoupling, StrongCoupling,
    GravityField, GravityFieldConfig, EventHorizon, EventHorizonConfig, BlackHoleConfig,
    MeshBrainSystem, MeshBrainConfig, MeshBrainSignalType, MeshBrainDeformationSignal,
    MultinovaProcessor, MultinovaConfig, NovaEvent,
    BaseForceCoordinator, CoordinationResult, SignalConnectionConfig,
    BlackHoleSignalConnector, MeshBrainSignalConnector, MultiNovaSignalConnector,
};

// 空间系统
pub use space::{
    SWindSpace, SpaceContainer, ContainerType, ContainerStatus,
    FractionalDimensionCalculator, DimensionType, DimensionResult,
    DimensionChange, DimensionAdjustment, DiscoveryResult,
};

// 接口
pub use interface::{
    SignalBuilder, ExperimentConfig, ExperimentRunner, ExperimentResult,
    quick_experiment, experiment_from_integers, experiment_from_string,
    capabilities, print_capabilities,
};

// Blondle: Universal STEM Knowledge Bundle
pub use blondle::{BlondleBundle, BlondleStats};

// Conjecture verification system
pub use conjecture::{
    Conjecture, ConjectureVerifier, Constraint, Domain,
    ExperimentContainer, PhysicsModel, VerificationResult,
};

// Algebraic Conjecture System (EPR-only, no f64)
pub use conjecture_algebraic::{
    AlgebraicConjecture, AlgebraicConstraint, AlgebraicContainer,
    AlgebraicDomain, AlgebraicVerificationResult, AlgebraicVerifier,
    ContainerType as AlgebraicContainerType, VerifierStats,
};

/// Compute exact partial sum of Basel series: S_N = Σ(n=1 to N) 1/n²
pub fn basel_partial_sum(n: u64) -> Rational {
    let mut sum = Rational::zero();
    for k in 1..=n as i128 {
        sum = sum + Rational::new(1, k * k);
    }
    sum
}

/// Compute exact harmonic number: H_N = Σ(n=1 to N) 1/n
pub fn harmonic_number(n: u64) -> Rational {
    let mut sum = Rational::zero();
    for k in 1..=n as i128 {
        sum = sum + Rational::new(1, k);
    }
    sum
}

/// Compute exact twin prime constant partial product: C₂(primes) = Π(1 - 1/(p-1)²)
pub fn twin_prime_partial(primes: &[i128]) -> Rational {
    let mut product = Rational::one();
    for &p in primes {
        if p > 2 {
            let pm1 = p - 1;
            product = product * Rational::new(pm1 * pm1 - 1, pm1 * pm1);
        }
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basel_s20() {
        let s20 = basel_partial_sum(20);
        assert_eq!(s20.numerator, 17299975731542641);
        assert_eq!(s20.denominator, 10838475198270720);
    }

    #[test]
    fn test_harmonic_h4() {
        let h4 = harmonic_number(4);
        assert_eq!(h4, Rational::new(25, 12));
    }

    
    #[test]
    fn test_quad_system_integration() {
        // Test Gravitational System (F = GM/r²)
        let gf = GravitationalField::new(GravitationalFieldConfig::default());
        let gravity = gf.gravitational_force(Rational::new(2, 1)).unwrap();
        assert_eq!(gravity, Rational::new(5, 2)); // 10/4 = 5/2
        
        // Test Quantum Field System (E = ℏω(n + 1/2))
        let mut qf = QuantumFieldSystem::new(QuantumFieldConfig::default());
        let signal = qf.generate_signal(Rational::new(6, 10), [Rational::zero(); 3]);
        assert!(matches!(signal.excitation_type, FieldExcitationType::QuantumExcitation));
        
        // Test Thermal Radiation System (P = σT⁴)
        let mut tr = ThermalRadiationProcessor::new(ThermalRadiationConfig::default());
        let event = tr.process_radiation(Rational::new(9, 10));
        assert!(event.is_supercritical);
        
        // Test Fundamental Coupling System
        let mut fc = FundamentalCouplingCoordinator::new();
        let result = fc.coordinate(
            Rational::new(6, 10),
            Rational::new(6, 10),
            Rational::new(6, 10),
        );
        assert!(result.gravitational_active);
    }
}
