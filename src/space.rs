//! S-wind Space System - Global Adaptive Dimension Management
//!
//! Features:
//! - FractionalDimensionCalculator (Hausdorff, Box, Information dimensions)
//! - adaptive_dimension_expansion()
//! - SpaceContainer with fractional dimensions
//!
//! Based on valhalla-rational/src/space.rs

use crate::symbolic::Rational;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Fractional Dimension Calculator
// ============================================================================

/// Dimension type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DimensionType {
    Integer,
    Fractional,
    Complex,
    Auto,
}

/// Dimension calculation result
#[derive(Debug, Clone)]
pub struct DimensionResult {
    pub recommended_dimension: Rational,
    pub dimension_type: DimensionType,
    pub mathematical_basis: String,
    pub hausdorff_estimate: Rational,
    pub box_estimate: Rational,
    pub info_estimate: Rational,
}

/// Fractional Dimension Calculator - GLOBAL adaptive component
#[derive(Debug, Clone)]
pub struct FractionalDimensionCalculator {
    pub calculation_count: u64,
    pub current_dimension: Rational,
    pub max_dimension: Rational,
}

impl FractionalDimensionCalculator {
    pub fn new() -> Self {
        Self {
            calculation_count: 0,
            current_dimension: Rational::new(4, 1),
            max_dimension: Rational::new(11, 1),
        }
    }

    /// Calculate optimal dimension
    pub fn calculate_optimal_dimension(&mut self, data: &[Rational]) -> DimensionResult {
        self.calculation_count += 1;
        
        if data.is_empty() {
            return DimensionResult {
                recommended_dimension: Rational::one(),
                dimension_type: DimensionType::Fractional,
                mathematical_basis: "Empty data".into(),
                hausdorff_estimate: Rational::one(),
                box_estimate: Rational::one(),
                info_estimate: Rational::one(),
            };
        }
        
        let hausdorff = self.estimate_hausdorff(data);
        let box_dim = self.estimate_box_dimension(data);
        let info_dim = self.estimate_information_dimension(data);
        
        // Combine: (hausdorff + 2*box + info) / 4
        let sum = hausdorff + box_dim + box_dim + info_dim;
        let fractional_dim = sum / Rational::new(4, 1);
        
        DimensionResult {
            recommended_dimension: self.clamp_dimension(fractional_dim),
            dimension_type: DimensionType::Fractional,
            mathematical_basis: format!(
                "Hausdorff={}, Box={}, Info={}",
                hausdorff, box_dim, info_dim
            ),
            hausdorff_estimate: hausdorff,
            box_estimate: box_dim,
            info_estimate: info_dim,
        }
    }

    fn estimate_hausdorff(&self, data: &[Rational]) -> Rational {
        if data.len() < 2 {
            return Rational::one();
        }
        
        let n = data.len() as i128;
        let sum: Rational = data.iter().fold(Rational::zero(), |acc, x| acc + *x);
        let mean = sum / Rational::new(n, 1);
        
        let mut variance_sum = Rational::zero();
        for x in data {
            let diff = *x - mean;
            variance_sum = variance_sum + diff * diff;
        }
        let variance = variance_sum / Rational::new(n, 1);
        
        self.clamp_dimension(Rational::one() + variance.abs())
    }

    fn estimate_box_dimension(&self, data: &[Rational]) -> Rational {
        if data.len() < 2 {
            return Rational::one();
        }
        
        let mut min = data[0];
        let mut max = data[0];
        for x in data.iter().skip(1) {
            if x.cmp_rational(&min) < 0 {
                min = *x;
            }
            if x.cmp_rational(&max) > 0 {
                max = *x;
            }
        }
        
        let range = (max - min).abs();
        if range.is_zero() {
            return Rational::one();
        }
        
        let n = Rational::new(data.len() as i128, 1);
        let normalized = n / (range + Rational::one());
        self.clamp_dimension(Rational::one() + normalized / Rational::new(10, 1))
    }

    fn estimate_information_dimension(&self, data: &[Rational]) -> Rational {
        if data.is_empty() {
            return Rational::one();
        }
        
        let mut distinct = 1i128;
        for i in 1..data.len() {
            if data[i] != data[i-1] {
                distinct += 1;
            }
        }
        
        let n = data.len() as i128;
        let ratio = Rational::new(distinct, n.max(1));
        Rational::one() + ratio
    }

    fn clamp_dimension(&self, dim: Rational) -> Rational {
        let one = Rational::one();
        if dim.cmp_rational(&one) < 0 {
            return one;
        }
        if dim.cmp_rational(&self.max_dimension) > 0 {
            return self.max_dimension;
        }
        dim
    }

    /// Adaptive dimension expansion (only increase - conservative)
    pub fn adaptive_expand(&mut self, data: &[Rational]) -> Rational {
        let result = self.calculate_optimal_dimension(data);
        
        if result.recommended_dimension.cmp_rational(&self.current_dimension) > 0 {
            self.current_dimension = result.recommended_dimension;
        }
        
        self.current_dimension
    }

    /// Fate-driven dimension adjustment (can increase OR decrease)
    /// Called by Fate based on internal state analysis
    pub fn fate_driven_adjust(&mut self, data: &[Rational]) -> DimensionAdjustment {
        let result = self.calculate_optimal_dimension(data);
        let old_dimension = self.current_dimension;
        
        // Fate can adjust in BOTH directions
        self.current_dimension = result.recommended_dimension;
        
        let change = if result.recommended_dimension.cmp_rational(&old_dimension) > 0 {
            DimensionChange::Increased
        } else if result.recommended_dimension.cmp_rational(&old_dimension) < 0 {
            DimensionChange::Decreased
        } else {
            DimensionChange::Unchanged
        };
        
        DimensionAdjustment {
            old_dimension,
            new_dimension: self.current_dimension,
            change,
            mathematical_basis: result.mathematical_basis,
        }
    }

    /// Force set dimension (used by Fate for explicit control)
    pub fn set_dimension(&mut self, dimension: Rational) {
        self.current_dimension = self.clamp_dimension(dimension);
    }

    pub fn get_current_dimension(&self) -> Rational {
        self.current_dimension
    }
}

/// Dimension change direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DimensionChange {
    Increased,
    Decreased,
    Unchanged,
}

/// Dimension adjustment result
#[derive(Debug, Clone)]
pub struct DimensionAdjustment {
    pub old_dimension: Rational,
    pub new_dimension: Rational,
    pub change: DimensionChange,
    pub mathematical_basis: String,
}

impl Default for FractionalDimensionCalculator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Space Container
// ============================================================================

/// Container type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerType {
    Basic,
    Adaptive,
    Fractional { dimension: Rational },
}

/// Container status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerStatus {
    Initializing,
    Running,
    Paused,
}

/// Space container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceContainer {
    pub container_id: String,
    pub dimensions: usize,
    pub fractional_dimension: Option<Rational>,
    pub container_type: ContainerType,
    pub status: ContainerStatus,
    pub created_at_tick: u64,
}

// ============================================================================
// S-wind Space (Global Space Manager)
// ============================================================================

/// S-wind Space - Global space management
#[derive(Debug)]
pub struct SWindSpace {
    pub space_id: String,
    pub fractional_calculator: FractionalDimensionCalculator,
    pub containers: HashMap<String, SpaceContainer>,
    pub signal_tick: u64,
}

impl SWindSpace {
    pub fn new() -> Self {
        Self {
            space_id: format!("swind_space_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() % 1_000_000),
            fractional_calculator: FractionalDimensionCalculator::new(),
            containers: HashMap::new(),
            signal_tick: 0,
        }
    }

    /// Create basic container
    pub fn create_container(&mut self, container_type: ContainerType) -> String {
        self.signal_tick += 1;
        
        let container_id = format!("container_{}_{}", self.signal_tick, self.containers.len());
        let container = SpaceContainer {
            container_id: container_id.clone(),
            dimensions: (self.fractional_calculator.current_dimension.numerator 
                        / self.fractional_calculator.current_dimension.denominator) as usize,
            fractional_dimension: Some(self.fractional_calculator.current_dimension),
            container_type,
            status: ContainerStatus::Running,
            created_at_tick: self.signal_tick,
        };
        self.containers.insert(container_id.clone(), container);
        container_id
    }

    /// Create fractional dimension container
    pub fn create_fractional_container(&mut self, data: &[Rational]) -> String {
        self.signal_tick += 1;
        
        let dimension = self.fractional_calculator.adaptive_expand(data);
        
        let container_id = format!("frac_{}_{}", self.signal_tick, self.containers.len());
        let container = SpaceContainer {
            container_id: container_id.clone(),
            dimensions: (dimension.numerator / dimension.denominator) as usize,
            fractional_dimension: Some(dimension),
            container_type: ContainerType::Fractional { dimension },
            status: ContainerStatus::Running,
            created_at_tick: self.signal_tick,
        };
        self.containers.insert(container_id.clone(), container);
        container_id
    }

    /// Adaptive dimension expansion (only increase - conservative)
    pub fn adaptive_dimension_expansion(&mut self, data: &[Rational]) -> Rational {
        self.fractional_calculator.adaptive_expand(data)
    }

    /// Fate-driven dimension adjustment (can increase OR decrease)
    pub fn fate_driven_adjustment(&mut self, data: &[Rational]) -> DimensionAdjustment {
        self.fractional_calculator.fate_driven_adjust(data)
    }

    /// On discovery: create container for unknown phenomenon
    /// Called by Fate when unknown signal pattern is detected
    pub fn on_discovery(&mut self, phenomenon_data: &[Rational], phenomenon_id: &str) -> DiscoveryResult {
        self.signal_tick += 1;
        
        // Fate-driven adjustment (can increase OR decrease)
        let adjustment = self.fractional_calculator.fate_driven_adjust(phenomenon_data);
        
        // Create new container for this discovery
        let container_id = format!("discovery_{}_{}", phenomenon_id, self.signal_tick);
        let container = SpaceContainer {
            container_id: container_id.clone(),
            dimensions: (adjustment.new_dimension.numerator 
                        / adjustment.new_dimension.denominator) as usize,
            fractional_dimension: Some(adjustment.new_dimension),
            container_type: ContainerType::Fractional { dimension: adjustment.new_dimension },
            status: ContainerStatus::Running,
            created_at_tick: self.signal_tick,
        };
        self.containers.insert(container_id.clone(), container);
        
        DiscoveryResult {
            container_id,
            dimension_adjustment: adjustment,
            signal_tick: self.signal_tick,
        }
    }

    /// Get current global dimension
    pub fn get_global_dimension(&self) -> Rational {
        self.fractional_calculator.get_current_dimension()
    }

    /// Set global dimension explicitly (Fate control)
    pub fn set_global_dimension(&mut self, dimension: Rational) {
        self.fractional_calculator.set_dimension(dimension);
    }

    /// Get logical time
    pub fn get_logical_time(&self) -> u64 {
        self.signal_tick
    }
}

/// Discovery result
#[derive(Debug, Clone)]
pub struct DiscoveryResult {
    pub container_id: String,
    pub dimension_adjustment: DimensionAdjustment,
    pub signal_tick: u64,
}

impl Default for SWindSpace {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Dynamic Transform Generator - Space analyzes signal and generates transforms
// ============================================================================

/// Signal characteristics detected by Space
#[derive(Debug, Clone)]
pub struct SignalCharacteristics {
    /// Is the signal monotonic (always increasing or decreasing)?
    pub is_monotonic: bool,
    /// Does the signal oscillate (sign changes)?
    pub oscillation_count: usize,
    /// Are there zeros in the signal?
    pub has_zeros: bool,
    /// Is the signal bounded (all values within a reasonable range)?
    pub is_bounded: bool,
    /// Growth rate estimate (ratio of consecutive terms)
    pub growth_type: GrowthType,
    /// Periodicity estimate (0 if not periodic)
    pub estimated_period: usize,
    /// Variance measure
    pub variance_level: VarianceLevel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GrowthType {
    Constant,       // No growth
    Linear,         // Arithmetic growth
    Polynomial,     // Polynomial growth
    Exponential,    // Exponential growth
    Oscillating,    // Alternating signs
    Irregular,      // No clear pattern
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VarianceLevel {
    Zero,       // All same values
    Low,        // Small variance
    Medium,     // Moderate variance
    High,       // Large variance
}

/// A dynamically generated transform
#[derive(Debug, Clone)]
pub struct DynamicTransform {
    pub name: String,
    pub description: String,
    pub transform_type: TransformType,
    pub priority: u32, // Higher = try first
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransformType {
    Difference,         // Δ: f(n+1) - f(n)
    SecondDifference,   // Δ²
    CumulativeSum,      // Σ
    AbsoluteValue,      // |·|
    ScaleByIndex,       // n·f(n)
    Ratio,              // f(n+1)/f(n)
    Modulo(i128),       // f(n) mod k
    SignExtract,        // sign(f(n))
    SquareRoot,         // √|f(n)|
    LogScale,           // ln(|f(n)|+1)
    PowerTransform(i32),// f(n)^k
    MovingAverage(usize), // (f(n-k)+...+f(n))/k
    Convolution,        // f*g type operations
}

impl SWindSpace {
    /// Analyze signal characteristics
    pub fn analyze_signal(&self, signal: &[Rational]) -> SignalCharacteristics {
        if signal.is_empty() {
            return SignalCharacteristics {
                is_monotonic: true,
                oscillation_count: 0,
                has_zeros: false,
                is_bounded: true,
                growth_type: GrowthType::Constant,
                estimated_period: 0,
                variance_level: VarianceLevel::Zero,
            };
        }
        
        // Check monotonicity
        let mut increasing = true;
        let mut decreasing = true;
        for i in 1..signal.len() {
            if signal[i].cmp_rational(&signal[i-1]) < 0 {
                increasing = false;
            }
            if signal[i].cmp_rational(&signal[i-1]) > 0 {
                decreasing = false;
            }
        }
        let is_monotonic = increasing || decreasing;
        
        // Count sign changes (oscillations)
        let mut oscillation_count = 0;
        for i in 1..signal.len() {
            let prev_sign = signal[i-1].cmp_rational(&Rational::zero());
            let curr_sign = signal[i].cmp_rational(&Rational::zero());
            if prev_sign != curr_sign && prev_sign != 0 && curr_sign != 0 {
                oscillation_count += 1;
            }
        }
        
        // Check for zeros
        let has_zeros = signal.iter().any(|x| x.is_zero());
        
        // Check boundedness (range)
        let mut min = signal[0];
        let mut max = signal[0];
        for x in signal.iter() {
            if x.cmp_rational(&min) < 0 { min = *x; }
            if x.cmp_rational(&max) > 0 { max = *x; }
        }
        let range = (max - min).abs();
        let is_bounded = range.cmp_rational(&Rational::new(1000, 1)) < 0;
        
        // Estimate growth type
        let growth_type = if signal.len() < 3 {
            GrowthType::Constant
        } else if oscillation_count > signal.len() / 3 {
            GrowthType::Oscillating
        } else {
            // Check differences
            let diffs: Vec<Rational> = signal.windows(2)
                .map(|w| w[1] - w[0])
                .collect();
            
            // Check if differences are constant (linear)
            let all_same_diff = diffs.windows(2)
                .all(|w| (w[1] - w[0]).abs().cmp_rational(&Rational::new(1, 100)) < 0);
            
            if all_same_diff {
                GrowthType::Linear
            } else {
                // Check second differences (quadratic)
                let second_diffs: Vec<Rational> = diffs.windows(2)
                    .map(|w| w[1] - w[0])
                    .collect();
                let all_same_second = second_diffs.windows(2)
                    .all(|w| (w[1] - w[0]).abs().cmp_rational(&Rational::new(1, 10)) < 0);
                
                if all_same_second {
                    GrowthType::Polynomial
                } else if !has_zeros {
                    // Check for exponential (constant ratio)
                    let ratios: Vec<Rational> = signal.windows(2)
                        .filter(|w| !w[0].is_zero())
                        .map(|w| w[1] / w[0])
                        .collect();
                    let all_same_ratio = ratios.len() >= 2 && ratios.windows(2)
                        .all(|w| (w[1] - w[0]).abs().cmp_rational(&Rational::new(1, 10)) < 0);
                    
                    if all_same_ratio {
                        GrowthType::Exponential
                    } else {
                        GrowthType::Irregular
                    }
                } else {
                    GrowthType::Irregular
                }
            }
        };
        
        // Estimate periodicity
        let estimated_period = self.detect_period(signal);
        
        // Variance level
        let n = signal.len() as i128;
        let mut sum = Rational::zero();
        for x in signal.iter() {
            sum = sum + *x;
        }
        let mean = sum / Rational::new(n, 1);
        let mut variance = Rational::zero();
        for x in signal.iter() {
            let diff = *x - mean;
            variance = variance + diff * diff;
        }
        let variance_level = if variance.is_zero() {
            VarianceLevel::Zero
        } else if variance.cmp_rational(&Rational::new(10, 1)) < 0 {
            VarianceLevel::Low
        } else if variance.cmp_rational(&Rational::new(100, 1)) < 0 {
            VarianceLevel::Medium
        } else {
            VarianceLevel::High
        };
        
        SignalCharacteristics {
            is_monotonic,
            oscillation_count,
            has_zeros,
            is_bounded,
            growth_type,
            estimated_period,
            variance_level,
        }
    }
    
    /// Detect periodicity in signal
    fn detect_period(&self, signal: &[Rational]) -> usize {
        if signal.len() < 4 {
            return 0;
        }
        
        // Try periods from 2 to len/2
        for period in 2..=signal.len()/2 {
            let mut is_periodic = true;
            for i in period..signal.len() {
                if signal[i] != signal[i - period] {
                    is_periodic = false;
                    break;
                }
            }
            if is_periodic {
                return period;
            }
        }
        0
    }
    
    /// Generate transforms dynamically based on signal characteristics
    pub fn generate_dynamic_transforms(&self, characteristics: &SignalCharacteristics) -> Vec<DynamicTransform> {
        let mut transforms = Vec::new();
        
        // Always include basic transforms
        transforms.push(DynamicTransform {
            name: "Δ".to_string(),
            description: "First difference".to_string(),
            transform_type: TransformType::Difference,
            priority: 100,
        });
        
        // Oscillating signal → try absolute value and sign extraction
        if characteristics.oscillation_count > 0 {
            transforms.push(DynamicTransform {
                name: "|·|".to_string(),
                description: "Absolute value".to_string(),
                transform_type: TransformType::AbsoluteValue,
                priority: 90,
            });
            transforms.push(DynamicTransform {
                name: "sign".to_string(),
                description: "Sign extraction".to_string(),
                transform_type: TransformType::SignExtract,
                priority: 85,
            });
        }
        
        // Non-monotonic → try cumulative sum
        if !characteristics.is_monotonic {
            transforms.push(DynamicTransform {
                name: "Σ".to_string(),
                description: "Cumulative sum".to_string(),
                transform_type: TransformType::CumulativeSum,
                priority: 80,
            });
        }
        
        // No zeros → ratio and division-based transforms
        if !characteristics.has_zeros {
            transforms.push(DynamicTransform {
                name: "ratio".to_string(),
                description: "Consecutive ratio".to_string(),
                transform_type: TransformType::Ratio,
                priority: 75,
            });
        }
        
        // Polynomial growth → second difference
        if characteristics.growth_type == GrowthType::Polynomial {
            transforms.push(DynamicTransform {
                name: "Δ²".to_string(),
                description: "Second difference".to_string(),
                transform_type: TransformType::SecondDifference,
                priority: 95,
            });
        }
        
        // High variance → try scaling transforms
        if characteristics.variance_level == VarianceLevel::High {
            transforms.push(DynamicTransform {
                name: "n·f".to_string(),
                description: "Scale by index".to_string(),
                transform_type: TransformType::ScaleByIndex,
                priority: 70,
            });
            transforms.push(DynamicTransform {
                name: "√|·|".to_string(),
                description: "Square root of absolute".to_string(),
                transform_type: TransformType::SquareRoot,
                priority: 65,
            });
        }
        
        // Periodic signal → try modular arithmetic
        if characteristics.estimated_period > 0 {
            transforms.push(DynamicTransform {
                name: format!("mod{}", characteristics.estimated_period),
                description: format!("Modulo {}", characteristics.estimated_period),
                transform_type: TransformType::Modulo(characteristics.estimated_period as i128),
                priority: 88,
            });
        }
        
        // Exponential growth → try log scale
        if characteristics.growth_type == GrowthType::Exponential {
            transforms.push(DynamicTransform {
                name: "ln".to_string(),
                description: "Log scale".to_string(),
                transform_type: TransformType::LogScale,
                priority: 85,
            });
        }
        
        // Moving average for irregular signals
        if characteristics.growth_type == GrowthType::Irregular {
            transforms.push(DynamicTransform {
                name: "MA3".to_string(),
                description: "Moving average (window 3)".to_string(),
                transform_type: TransformType::MovingAverage(3),
                priority: 60,
            });
            transforms.push(DynamicTransform {
                name: "f²".to_string(),
                description: "Square transform".to_string(),
                transform_type: TransformType::PowerTransform(2),
                priority: 55,
            });
        }
        
        // Sort by priority (descending)
        transforms.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        transforms
    }
    
    /// Apply a dynamic transform to a signal
    pub fn apply_transform(&self, signal: &[Rational], transform: &DynamicTransform) -> Option<Vec<Rational>> {
        if signal.len() < 2 {
            return None;
        }
        
        match transform.transform_type {
            TransformType::Difference => {
                Some(signal.windows(2).map(|w| w[1] - w[0]).collect())
            },
            TransformType::SecondDifference => {
                if signal.len() < 3 {
                    return None;
                }
                let d1: Vec<Rational> = signal.windows(2).map(|w| w[1] - w[0]).collect();
                Some(d1.windows(2).map(|w| w[1] - w[0]).collect())
            },
            TransformType::CumulativeSum => {
                let mut result = vec![Rational::zero()];
                let mut acc = Rational::zero();
                for val in signal.iter() {
                    acc = acc + *val;
                    result.push(acc);
                }
                Some(result)
            },
            TransformType::AbsoluteValue => {
                Some(signal.iter().map(|x| x.abs()).collect())
            },
            TransformType::ScaleByIndex => {
                Some(signal.iter().enumerate()
                    .map(|(i, x)| *x * Rational::new((i + 1) as i128, 1))
                    .collect())
            },
            TransformType::Ratio => {
                if signal.iter().any(|x| x.is_zero()) {
                    return None;
                }
                Some(signal.windows(2).map(|w| w[1] / w[0]).collect())
            },
            TransformType::SignExtract => {
                Some(signal.iter().map(|x| {
                    let cmp = x.cmp_rational(&Rational::zero());
                    if cmp > 0 { Rational::one() }
                    else if cmp < 0 { Rational::new(-1, 1) }
                    else { Rational::zero() }
                }).collect())
            },
            TransformType::SquareRoot => {
                // Approximate √|x| as |x|^(1/2) ≈ |x| for rational
                // For exact, we'd need irrational support
                Some(signal.iter().map(|x| x.abs()).collect())
            },
            TransformType::LogScale => {
                // Can't do exact log with rationals
                None
            },
            TransformType::Modulo(k) => {
                Some(signal.iter().map(|x| {
                    let n = x.numerator % (k * x.denominator);
                    Rational::new(n, x.denominator)
                }).collect())
            },
            TransformType::PowerTransform(p) => {
                Some(signal.iter().map(|x| {
                    let mut result = Rational::one();
                    for _ in 0..p.abs() {
                        result = result * *x;
                    }
                    if p < 0 && !result.is_zero() {
                        Rational::new(result.denominator, result.numerator)
                    } else {
                        result
                    }
                }).collect())
            },
            TransformType::MovingAverage(window) => {
                if signal.len() < window {
                    return None;
                }
                let w = Rational::new(window as i128, 1);
                Some(signal.windows(window)
                    .map(|win| {
                        let mut sum = Rational::zero();
                        for x in win.iter() {
                            sum = sum + *x;
                        }
                        sum / w
                    })
                    .collect())
            },
            TransformType::Convolution => {
                // Too complex for simple implementation
                None
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fractional_calculator() {
        let mut calc = FractionalDimensionCalculator::new();
        let data = vec![Rational::new(1, 2), Rational::one(), Rational::new(3, 2)];
        let result = calc.calculate_optimal_dimension(&data);
        assert!(result.recommended_dimension.numerator >= result.recommended_dimension.denominator);
    }

    #[test]
    fn test_create_fractional_container() {
        let mut space = SWindSpace::new();
        let data = vec![Rational::one(), Rational::new(10, 1)];
        let id = space.create_fractional_container(&data);
        assert!(space.containers.contains_key(&id));
    }

    #[test]
    fn test_adaptive_expand() {
        let mut space = SWindSpace::new();
        let data1 = vec![Rational::one()];
        let dim1 = space.adaptive_dimension_expansion(&data1);
        
        let data2 = vec![Rational::one(), Rational::new(100, 1), Rational::new(-50, 1)];
        let dim2 = space.adaptive_dimension_expansion(&data2);
        
        // Dimension should only expand, never shrink
        assert!(dim2.cmp_rational(&dim1) >= 0);
    }
}
