//! S-wind Quad System - Rigorous Physical Simulation
//!
//! Four fundamental physical systems with exact algebraic computation:
//!
//! 1. **Gravitational System** - Newtonian gravity with Schwarzschild corrections
//! 2. **Quantum Field System** - Field modes and vacuum fluctuations
//! 3. **Thermal Radiation System** - Stefan-Boltzmann and Planck radiation
//! 4. **Fundamental Coupling System** - Force unification and coupling constants
//!
//! All calculations use exact Rational arithmetic (i128/i128).
//! Time is measured by signal_tick (deformation signal count).

use crate::symbolic::Rational;
use serde::{Deserialize, Serialize};

// ============================================================================
// I. GRAVITATIONAL SYSTEM
// ============================================================================
// Physics:
//   Newton's Law: F = G × M × m / r²
//   Schwarzschild Radius: r_s = 2GM/c²
//   Escape Velocity: v_e² = 2GM/r
//   Gravitational Potential: φ = -GM/r
// ============================================================================

/// Gravitational constants (in natural units where G = 1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravitationalConstants {
    /// Gravitational constant G (set to 1 in natural units)
    pub g_constant: Rational,
    /// Speed of light c (set to 1 in natural units)
    pub c_light: Rational,
}

impl Default for GravitationalConstants {
    fn default() -> Self {
        Self {
            g_constant: Rational::one(),  // G = 1 (natural units)
            c_light: Rational::one(),     // c = 1 (natural units)
        }
    }
}

/// Mass distribution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassDistribution {
    /// Total mass M
    pub mass: Rational,
    /// Schwarzschild radius r_s = 2GM/c² (with G=c=1: r_s = 2M)
    pub schwarzschild_radius: Rational,
}

impl MassDistribution {
    pub fn new(mass: Rational) -> Self {
        // r_s = 2M (in natural units where G = c = 1)
        let schwarzschild_radius = Rational::new(2, 1) * mass;
        Self { mass, schwarzschild_radius }
    }
}

impl Default for MassDistribution {
    fn default() -> Self {
        Self::new(Rational::new(10, 1))  // M = 10
    }
}

/// Gravitational field configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct GravitationalFieldConfig {
    pub constants: GravitationalConstants,
    pub mass_distribution: MassDistribution,
}


/// Gravitational field system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GravitationalField {
    pub config: GravitationalFieldConfig,
}

impl GravitationalField {
    pub fn new(config: GravitationalFieldConfig) -> Self {
        Self { config }
    }

    /// Newton's gravitational force: F = G × M × m / r²
    /// For unit test mass (m = 1): F = GM/r²
    pub fn gravitational_force(&self, distance: Rational) -> Option<Rational> {
        if distance.is_zero() {
            return None; // Singularity at r = 0
        }
        // F = G × M / r²
        let gm = self.config.constants.g_constant * self.config.mass_distribution.mass;
        let r_squared = distance * distance;
        Some(gm / r_squared)
    }
    
    /// Backward compatibility alias for gravitational_force
    pub fn calculate_gravity(&self, distance: Rational) -> Option<Rational> {
        self.gravitational_force(distance)
    }

    /// Gravitational potential: φ = -GM/r
    pub fn gravitational_potential(&self, distance: Rational) -> Option<Rational> {
        if distance.is_zero() {
            return None;
        }
        let gm = self.config.constants.g_constant * self.config.mass_distribution.mass;
        Some(-gm / distance)
    }

    /// Escape velocity squared: v_e² = 2GM/r
    /// Returns v_e² to avoid irrational sqrt
    pub fn escape_velocity_squared(&self, distance: Rational) -> Option<Rational> {
        if distance.is_zero() {
            return None;
        }
        let two_gm = Rational::new(2, 1) * self.config.constants.g_constant 
                   * self.config.mass_distribution.mass;
        Some(two_gm / distance)
    }

    /// Check if within event horizon: r < r_s
    pub fn is_within_event_horizon(&self, distance: Rational) -> bool {
        distance.cmp_rational(&self.config.mass_distribution.schwarzschild_radius) < 0
    }

    /// Check if particle can escape: kinetic_energy² > escape_velocity²
    /// KE = (1/2)mv², for unit mass: KE = v²/2
    /// Escape condition: v² > v_e²
    pub fn can_escape(&self, velocity_squared: Rational, distance: Rational) -> bool {
        if let Some(v_e_squared) = self.escape_velocity_squared(distance) {
            velocity_squared.cmp_rational(&v_e_squared) > 0
        } else {
            false
        }
    }
}

// Aliases for backward compatibility
pub type GravityField = GravitationalField;
pub type GravityFieldConfig = GravitationalFieldConfig;
pub type EventHorizonConfig = GravitationalFieldConfig;
pub type EventHorizon = GravitationalField;
pub type BlackHoleConfig = GravitationalFieldConfig;

// ============================================================================
// II. QUANTUM FIELD SYSTEM
// ============================================================================
// Physics:
//   Harmonic Oscillator: E_n = ℏω(n + 1/2)
//   Zero-point Energy: E_0 = ℏω/2
//   Vacuum Fluctuation: ΔE × Δt ≥ ℏ/2
//   Field Mode Amplitude: A = √(ℏ/2ωV)
// ============================================================================

/// Quantum field configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumFieldConfig {
    /// Number of field modes
    pub mode_count: usize,
    /// Base frequency ω (in natural units where ℏ = 1)
    pub base_frequency: Rational,
    /// Volume factor V
    pub volume: Rational,
    /// Fluctuation intensity parameter
    pub fluctuation_intensity: Rational,
}

impl Default for QuantumFieldConfig {
    fn default() -> Self {
        Self {
            mode_count: 1000,
            base_frequency: Rational::new(1, 1),   // ω = 1
            volume: Rational::new(10, 1),          // V = 10
            fluctuation_intensity: Rational::new(3, 10), // 0.3
        }
    }
}

/// Quantum field excitation type
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FieldExcitationType {
    /// Ground state oscillation
    VacuumFluctuation,
    /// Excited state transition
    QuantumExcitation,
}

/// Quantum field signal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumFieldSignal {
    pub excitation_type: FieldExcitationType,
    /// Excitation amplitude
    pub amplitude: Rational,
    /// Position in configuration space
    pub position: [Rational; 3],
    /// Signal tick (logical time)
    pub signal_tick: u64,
    /// Energy of the mode: E = ℏω(n + 1/2)
    pub mode_energy: Rational,
    /// Occupation number n
    pub occupation_number: Rational,
}

/// Quantum field system
#[derive(Debug, Clone)]
pub struct QuantumFieldSystem {
    pub config: QuantumFieldConfig,
    pub signal_buffer: Vec<QuantumFieldSignal>,
    pub signal_tick: u64,
}

impl QuantumFieldSystem {
    pub fn new(config: QuantumFieldConfig) -> Self {
        Self {
            config,
            signal_buffer: Vec::new(),
            signal_tick: 0,
        }
    }

    /// Zero-point energy: E_0 = ℏω/2 (with ℏ = 1: E_0 = ω/2)
    pub fn zero_point_energy(&self) -> Rational {
        self.config.base_frequency / Rational::new(2, 1)
    }

    /// Mode energy: E_n = ω(n + 1/2)
    pub fn mode_energy(&self, occupation_number: Rational) -> Rational {
        let n_plus_half = occupation_number + Rational::new(1, 2);
        self.config.base_frequency * n_plus_half
    }

    /// Generate quantum field signal
    pub fn generate_signal(&mut self, amplitude: Rational, position: [Rational; 3]) -> QuantumFieldSignal {
        self.signal_tick += 1;
        
        // Excitation type: amplitude > 0.5 → Quantum excitation
        let excitation_type = if amplitude.cmp_rational(&Rational::new(1, 2)) > 0 {
            FieldExcitationType::QuantumExcitation
        } else {
            FieldExcitationType::VacuumFluctuation
        };
        
        // Occupation number from amplitude: n ∝ amplitude²
        let occupation_number = amplitude * amplitude;
        
        // Energy of the mode
        let mode_energy = self.mode_energy(occupation_number);
        
        let signal = QuantumFieldSignal {
            excitation_type,
            amplitude,
            position,
            signal_tick: self.signal_tick,
            mode_energy,
            occupation_number,
        };
        self.signal_buffer.push(signal.clone());
        signal
    }
}

// Aliases for backward compatibility
pub type MeshBrainConfig = QuantumFieldConfig;
pub type MeshBrainSignalType = FieldExcitationType;
pub type MeshBrainDeformationSignal = QuantumFieldSignal;
pub type MeshBrainSystem = QuantumFieldSystem;

// ============================================================================
// III. THERMAL RADIATION SYSTEM
// ============================================================================
// Physics:
//   Stefan-Boltzmann Law: P = σT⁴
//   Wien's Displacement: λ_max × T = b
//   Planck's Law: B(ν,T) = (2hν³/c²) × 1/(e^(hν/kT) - 1)
//   Total Energy: E = σT⁴ × A × t
// ============================================================================

/// Thermal radiation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalRadiationConfig {
    /// Stefan-Boltzmann constant σ (in natural units)
    pub stefan_boltzmann: Rational,
    /// Critical temperature threshold
    pub critical_temperature: Rational,
    /// Radiation efficiency factor
    pub efficiency: Rational,
}

impl Default for ThermalRadiationConfig {
    fn default() -> Self {
        Self {
            stefan_boltzmann: Rational::new(1, 1),     // σ = 1 (natural units)
            critical_temperature: Rational::new(8, 10), // T_c = 0.8
            efficiency: Rational::new(1, 1),           // η = 1
        }
    }
}

/// Thermal radiation event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalRadiationEvent {
    pub event_id: u64,
    /// Temperature T
    pub temperature: Rational,
    /// Is above critical temperature
    pub is_supercritical: bool,
    /// Radiated power: P = σT⁴
    pub radiated_power: Rational,
}

/// Thermal radiation processor
#[derive(Debug, Clone)]
pub struct ThermalRadiationProcessor {
    pub config: ThermalRadiationConfig,
    pub total_events: u64,
}

impl ThermalRadiationProcessor {
    pub fn new(config: ThermalRadiationConfig) -> Self {
        Self {
            config,
            total_events: 0,
        }
    }

    /// Stefan-Boltzmann power: P = σT⁴
    pub fn radiated_power(&self, temperature: Rational) -> Rational {
        // P = σ × T⁴
        let t2 = temperature * temperature;
        let t4 = t2 * t2;
        self.config.stefan_boltzmann * t4
    }

    /// Process thermal radiation event
    pub fn process_radiation(&mut self, temperature: Rational) -> ThermalRadiationEvent {
        self.total_events += 1;
        
        // Check if supercritical
        let is_supercritical = temperature.cmp_rational(&self.config.critical_temperature) > 0;
        
        // Calculate radiated power
        let radiated_power = if is_supercritical {
            self.radiated_power(temperature) * self.config.efficiency
        } else {
            Rational::zero()
        };
        
        ThermalRadiationEvent {
            event_id: self.total_events,
            temperature,
            is_supercritical,
            radiated_power,
        }
    }
}

// Aliases for backward compatibility
pub type MultinovaConfig = ThermalRadiationConfig;
pub type NovaEvent = ThermalRadiationEvent;
pub type MultinovaProcessor = ThermalRadiationProcessor;

// ============================================================================
// IV. FUNDAMENTAL COUPLING SYSTEM
// ============================================================================
// Physics:
//   Coupling Constants: α_em ≈ 1/137, α_s ≈ 1, α_w ≈ 1/30
//   Running Coupling: α(μ) = α(μ₀)/(1 + β×ln(μ/μ₀))
//   Force Unification: Weighted average of active couplings
// ============================================================================

/// Coupling constant configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouplingConfig {
    pub buffer_size: usize,
    /// Coupling threshold for activation
    pub activation_threshold: Rational,
}

impl Default for CouplingConfig {
    fn default() -> Self {
        Self {
            buffer_size: 100,
            activation_threshold: Rational::new(1, 2), // 0.5
        }
    }
}

/// Gravitational coupling channel
#[derive(Debug, Clone)]
pub struct GravitationalCoupling {
    pub config: CouplingConfig,
    pub interaction_count: u64,
}

impl GravitationalCoupling {
    pub fn new() -> Self {
        Self {
            config: CouplingConfig::default(),
            interaction_count: 0,
        }
    }

    /// Check if coupling is active
    pub fn is_active(&mut self, coupling_strength: Rational) -> bool {
        self.interaction_count += 1;
        coupling_strength.cmp_rational(&self.config.activation_threshold) > 0
    }
}

impl Default for GravitationalCoupling {
    fn default() -> Self {
        Self::new()
    }
}

/// Electromagnetic coupling channel
#[derive(Debug, Clone)]
pub struct ElectromagneticCoupling {
    pub config: CouplingConfig,
    pub interaction_count: u64,
}

impl ElectromagneticCoupling {
    pub fn new() -> Self {
        Self {
            config: CouplingConfig::default(),
            interaction_count: 0,
        }
    }

    pub fn is_active(&mut self, coupling_strength: Rational) -> bool {
        self.interaction_count += 1;
        coupling_strength.cmp_rational(&self.config.activation_threshold) > 0
    }
}

impl Default for ElectromagneticCoupling {
    fn default() -> Self {
        Self::new()
    }
}

/// Strong force coupling channel
#[derive(Debug, Clone)]
pub struct StrongCoupling {
    pub config: CouplingConfig,
    pub interaction_count: u64,
}

impl StrongCoupling {
    pub fn new() -> Self {
        Self {
            config: CouplingConfig::default(),
            interaction_count: 0,
        }
    }

    pub fn is_active(&mut self, coupling_strength: Rational) -> bool {
        self.interaction_count += 1;
        coupling_strength.cmp_rational(&self.config.activation_threshold) > 0
    }
}

impl Default for StrongCoupling {
    fn default() -> Self {
        Self::new()
    }
}

/// Coupling coordination result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouplingResult {
    pub gravitational_active: bool,
    pub electromagnetic_active: bool,
    pub strong_active: bool,
    /// Unified effective coupling: average of active couplings
    pub effective_coupling: Rational,
}

/// Fundamental force coupling coordinator
#[derive(Debug, Clone)]
pub struct FundamentalCouplingCoordinator {
    pub gravitational: GravitationalCoupling,
    pub electromagnetic: ElectromagneticCoupling,
    pub strong: StrongCoupling,
}

impl FundamentalCouplingCoordinator {
    pub fn new() -> Self {
        Self {
            gravitational: GravitationalCoupling::new(),
            electromagnetic: ElectromagneticCoupling::new(),
            strong: StrongCoupling::new(),
        }
    }

    /// Coordinate force couplings
    /// effective_coupling = (α_g + α_em + α_s) / 3
    pub fn coordinate(
        &mut self,
        gravitational_strength: Rational,
        electromagnetic_strength: Rational,
        strong_strength: Rational,
    ) -> CouplingResult {
        let gravitational_active = self.gravitational.is_active(gravitational_strength);
        let electromagnetic_active = self.electromagnetic.is_active(electromagnetic_strength);
        let strong_active = self.strong.is_active(strong_strength);
        
        // Effective coupling: average
        let sum = gravitational_strength + electromagnetic_strength + strong_strength;
        let effective_coupling = sum / Rational::new(3, 1);
        
        CouplingResult {
            gravitational_active,
            electromagnetic_active,
            strong_active,
            effective_coupling,
        }
    }
}

impl Default for FundamentalCouplingCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

// Aliases for backward compatibility
pub type SignalConnectionConfig = CouplingConfig;
pub type BlackHoleSignalConnector = GravitationalCoupling;
pub type MeshBrainSignalConnector = ElectromagneticCoupling;
pub type MultiNovaSignalConnector = StrongCoupling;
pub type CoordinationResult = CouplingResult;
pub type BaseForceCoordinator = FundamentalCouplingCoordinator;

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gravitational_force() {
        // Newton's law: F = GM/r² (with G = 1, M = 10)
        let config = GravitationalFieldConfig::default();
        let gf = GravitationalField::new(config);
        
        let distance = Rational::new(2, 1);  // r = 2
        let force = gf.gravitational_force(distance).unwrap();
        
        // F = 10 / 4 = 5/2
        assert_eq!(force, Rational::new(5, 2));
    }
    
    #[test]
    fn test_schwarzschild_radius() {
        // r_s = 2M (in natural units)
        let mass = Rational::new(5, 1);  // M = 5
        let md = MassDistribution::new(mass);
        
        assert_eq!(md.schwarzschild_radius, Rational::new(10, 1)); // r_s = 10
    }
    
    #[test]
    fn test_event_horizon() {
        let config = GravitationalFieldConfig::default(); // M = 10, r_s = 20
        let gf = GravitationalField::new(config);
        
        assert!(gf.is_within_event_horizon(Rational::new(15, 1)));  // 15 < 20
        assert!(!gf.is_within_event_horizon(Rational::new(25, 1))); // 25 > 20
    }
    
    #[test]
    fn test_escape_velocity() {
        let config = GravitationalFieldConfig::default(); // G = 1, M = 10
        let gf = GravitationalField::new(config);
        
        let distance = Rational::new(4, 1);  // r = 4
        let v_e_squared = gf.escape_velocity_squared(distance).unwrap();
        
        // v_e² = 2GM/r = 2×1×10/4 = 5
        assert_eq!(v_e_squared, Rational::new(5, 1));
    }
    
    #[test]
    fn test_quantum_field_energy() {
        let config = QuantumFieldConfig::default(); // ω = 1
        let qf = QuantumFieldSystem::new(config);
        
        // Zero-point energy: E_0 = ω/2 = 1/2
        assert_eq!(qf.zero_point_energy(), Rational::new(1, 2));
        
        // Mode energy with n = 2: E = ω(n + 1/2) = 1 × 2.5 = 5/2
        let n = Rational::new(2, 1);
        assert_eq!(qf.mode_energy(n), Rational::new(5, 2));
    }
    
    #[test]
    fn test_quantum_field_signal() {
        let config = QuantumFieldConfig::default();
        let mut qf = QuantumFieldSystem::new(config);
        
        let amplitude = Rational::new(6, 10); // 0.6 > 0.5
        let position = [Rational::zero(); 3];
        let signal = qf.generate_signal(amplitude, position);
        
        assert!(matches!(signal.excitation_type, FieldExcitationType::QuantumExcitation));
        assert_eq!(signal.signal_tick, 1);
    }
    
    #[test]
    fn test_thermal_radiation() {
        let config = ThermalRadiationConfig::default(); // σ = 1, T_c = 0.8
        let mut tr = ThermalRadiationProcessor::new(config);
        
        // T = 1.0 > 0.8 (supercritical)
        let event = tr.process_radiation(Rational::new(1, 1));
        assert!(event.is_supercritical);
        // P = σT⁴ = 1 × 1⁴ = 1
        assert_eq!(event.radiated_power, Rational::one());
        
        // T = 0.5 < 0.8 (subcritical)
        let event = tr.process_radiation(Rational::new(1, 2));
        assert!(!event.is_supercritical);
        assert_eq!(event.radiated_power, Rational::zero());
    }
    
    #[test]
    fn test_stefan_boltzmann() {
        let config = ThermalRadiationConfig::default();
        let tr = ThermalRadiationProcessor::new(config);
        
        // P = σT⁴ with T = 2
        let temperature = Rational::new(2, 1);
        let power = tr.radiated_power(temperature);
        
        // P = 1 × 2⁴ = 16
        assert_eq!(power, Rational::new(16, 1));
    }
    
    #[test]
    fn test_fundamental_coupling() {
        let mut fc = FundamentalCouplingCoordinator::new(); // threshold = 0.5
        
        let result = fc.coordinate(
            Rational::new(6, 10),  // 0.6 > 0.5 → active
            Rational::new(4, 10),  // 0.4 < 0.5 → inactive
            Rational::new(9, 10),  // 0.9 > 0.5 → active
        );
        
        assert!(result.gravitational_active);
        assert!(!result.electromagnetic_active);
        assert!(result.strong_active);
        
        // effective = (0.6 + 0.4 + 0.9) / 3 = 1.9/3 = 19/30
        assert_eq!(result.effective_coupling, Rational::new(19, 30));
    }
    
    // Backward compatibility tests
    #[test]
    fn test_gravity_calculation() {
        let config = GravityFieldConfig::default();
        let gf = GravityField::new(config);
        let gravity = gf.gravitational_force(Rational::new(2, 1)).unwrap();
        assert_eq!(gravity, Rational::new(5, 2));
    }
    
    #[test]
    fn test_meshbrain_signal() {
        let config = MeshBrainConfig::default();
        let mut mb = MeshBrainSystem::new(config);
        let signal = mb.generate_signal(Rational::new(6, 10), [Rational::zero(); 3]);
        assert!(matches!(signal.excitation_type, MeshBrainSignalType::QuantumExcitation));
    }
    
    #[test]
    fn test_nova_event() {
        let config = MultinovaConfig::default();
        let mut mn = MultinovaProcessor::new(config);
        let event = mn.process_radiation(Rational::new(9, 10));
        assert!(event.is_supercritical);
    }
    
    #[test]
    fn test_baseforce_coordination() {
        let mut bf = BaseForceCoordinator::new();
        let result = bf.coordinate(
            Rational::new(6, 10),
            Rational::new(6, 10),
            Rational::new(6, 10),
        );
        assert!(result.gravitational_active);
    }
}
