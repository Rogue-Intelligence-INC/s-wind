//! Manufacturing Engineering Module for Blondle
//!
//! Covers:
//! - GD&T (Geometric Dimensioning and Tolerancing)
//! - Statistical process control (SPC)
//! - Machining parameters
//! - Quality control (Six Sigma)
//! - Metrology standards

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all manufacturing engineering knowledge
pub fn load(kb: &mut KnowledgeBase) {
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::Manufacturing] Loaded {} constants, {} laws, {} theorems",
             CONSTANT_COUNT, LAW_COUNT, THEOREM_COUNT);
}

pub fn count() -> (usize, usize, usize) {
    (CONSTANT_COUNT, LAW_COUNT, THEOREM_COUNT)
}

const CONSTANT_COUNT: usize = 15;
const LAW_COUNT: usize = 12;
const THEOREM_COUNT: usize = 10;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Tolerance grades (ISO 286)
        PhysicalConstant {
            id: "iso_it6".into(),
            name: "IT6 tolerance grade".into(),
            symbol: "IT6".into(),
            value: "10i".into(),
            unit: "μm".into(),
            uncertainty: None,
            source: "ISO 286".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"IT6 = 10i \text{ where } i = 0.45\sqrt[3]{D} + 0.001D".into(),
        },
        PhysicalConstant {
            id: "iso_it7".into(),
            name: "IT7 tolerance grade".into(),
            symbol: "IT7".into(),
            value: "16i".into(),
            unit: "μm".into(),
            uncertainty: None,
            source: "ISO 286".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"IT7 = 16i \text{ (precision machining)}".into(),
        },
        PhysicalConstant {
            id: "iso_it8".into(),
            name: "IT8 tolerance grade".into(),
            symbol: "IT8".into(),
            value: "25i".into(),
            unit: "μm".into(),
            uncertainty: None,
            source: "ISO 286".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"IT8 = 25i \text{ (general machining)}".into(),
        },
        
        // Surface roughness
        PhysicalConstant {
            id: "Ra_grinding".into(),
            name: "Ra for grinding".into(),
            symbol: "Ra_{grind}".into(),
            value: "0.4".into(),
            unit: "μm".into(),
            uncertainty: Some("0.3".into()),
            source: "Machining handbooks".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"Ra \approx 0.1-0.8 \text{ μm (grinding)}".into(),
        },
        PhysicalConstant {
            id: "Ra_turning".into(),
            name: "Ra for turning".into(),
            symbol: "Ra_{turn}".into(),
            value: "1.6".into(),
            unit: "μm".into(),
            uncertainty: Some("1.0".into()),
            source: "Machining handbooks".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"Ra \approx 0.8-3.2 \text{ μm (turning)}".into(),
        },
        PhysicalConstant {
            id: "Ra_milling".into(),
            name: "Ra for milling".into(),
            symbol: "Ra_{mill}".into(),
            value: "3.2".into(),
            unit: "μm".into(),
            uncertainty: Some("2.0".into()),
            source: "Machining handbooks".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"Ra \approx 1.6-6.3 \text{ μm (milling)}".into(),
        },
        
        // Six Sigma
        PhysicalConstant {
            id: "six_sigma_dpmo".into(),
            name: "Six Sigma DPMO".into(),
            symbol: "DPMO_{6σ}".into(),
            value: "3.4".into(),
            unit: "ppm".into(),
            uncertainty: None,
            source: "Six Sigma".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"DPMO = 3.4 \text{ ppm (6σ with 1.5σ shift)}".into(),
        },
        PhysicalConstant {
            id: "cpk_target".into(),
            name: "Target Cpk".into(),
            symbol: "C_{pk}".into(),
            value: "1.33".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Quality standards".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"C_{pk} \geq 1.33 \text{ (4σ capability)}".into(),
        },
        PhysicalConstant {
            id: "cp_six_sigma".into(),
            name: "Cp for Six Sigma".into(),
            symbol: "C_p".into(),
            value: "2.0".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Six Sigma".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"C_p = 2.0 \text{ for 6σ process}".into(),
        },
        
        // Machining parameters
        PhysicalConstant {
            id: "cutting_speed_steel_hss".into(),
            name: "Cutting speed (steel, HSS)".into(),
            symbol: "V_c".into(),
            value: "30".into(),
            unit: "m/min".into(),
            uncertainty: Some("10".into()),
            source: "Machining handbooks".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"V_c \approx 20-40 \text{ m/min (steel, HSS tool)}".into(),
        },
        PhysicalConstant {
            id: "cutting_speed_steel_carbide".into(),
            name: "Cutting speed (steel, carbide)".into(),
            symbol: "V_{c,carb}".into(),
            value: "150".into(),
            unit: "m/min".into(),
            uncertainty: Some("50".into()),
            source: "Machining handbooks".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"V_c \approx 100-300 \text{ m/min (steel, carbide)}".into(),
        },
        PhysicalConstant {
            id: "specific_cutting_force_steel".into(),
            name: "Specific cutting force (steel)".into(),
            symbol: "k_c".into(),
            value: "2500".into(),
            unit: "MPa".into(),
            uncertainty: Some("500".into()),
            source: "Machining handbooks".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"k_c \approx 2000-3000 \text{ MPa (carbon steel)}".into(),
        },
        
        // Measurement uncertainty
        PhysicalConstant {
            id: "cmm_uncertainty".into(),
            name: "CMM measurement uncertainty".into(),
            symbol: "U_{CMM}".into(),
            value: "2".into(),
            unit: "μm + L/300".into(),
            uncertainty: None,
            source: "CMM specifications".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"U = (2 + L/300) \text{ μm (typical CMM)}".into(),
        },
        PhysicalConstant {
            id: "micrometer_resolution".into(),
            name: "Micrometer resolution".into(),
            symbol: r"\delta_{mic}".into(),
            value: "1".into(),
            unit: "μm".into(),
            uncertainty: None,
            source: "Metrology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\delta = 1 \text{ μm (digital micrometer)}".into(),
        },
        PhysicalConstant {
            id: "gauge_rr_limit".into(),
            name: "Gauge R&R acceptance limit".into(),
            symbol: r"\%GRR".into(),
            value: "10".into(),
            unit: "%".into(),
            uncertainty: None,
            source: "MSA".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\%GRR < 10\% \text{ (acceptable)}, < 30\% \text{ (marginal)}".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // GD&T
        PhysicalLaw {
            id: "position_tolerance".into(),
            name: "Position Tolerance Zone".into(),
            domain: PhysicsDomain::Other("GD&T".into()),
            equation_tex: r"t = 2\sqrt{(x-x_0)^2 + (y-y_0)^2}".into(),
            description: "Cylindrical tolerance zone diameter".into(),
            variables: vec![
                Variable { symbol: "t".into(), name: "Tolerance diameter".into(), unit: Some("mm".into()) },
                Variable { symbol: "x_0, y_0".into(), name: "True position".into(), unit: Some("mm".into()) },
            ],
            source: "ASME Y14.5".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "mmc_bonus".into(),
            name: "MMC Bonus Tolerance".into(),
            domain: PhysicsDomain::Other("GD&T".into()),
            equation_tex: r"t_{total} = t_{geo} + |D_{actual} - D_{MMC}|".into(),
            description: "Total tolerance with MMC modifier".into(),
            variables: vec![
                Variable { symbol: "t_geo".into(), name: "Geometric tolerance".into(), unit: Some("mm".into()) },
                Variable { symbol: "D_MMC".into(), name: "MMC size".into(), unit: Some("mm".into()) },
            ],
            source: "ASME Y14.5".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Surface roughness
        PhysicalLaw {
            id: "theoretical_roughness".into(),
            name: "Theoretical Surface Roughness".into(),
            domain: PhysicsDomain::Other("Machining".into()),
            equation_tex: r"R_t = \frac{f^2}{8r}".into(),
            description: "Ideal roughness from feed and tool radius".into(),
            variables: vec![
                Variable { symbol: "R_t".into(), name: "Theoretical roughness".into(), unit: Some("μm".into()) },
                Variable { symbol: "f".into(), name: "Feed per revolution".into(), unit: Some("mm/rev".into()) },
                Variable { symbol: "r".into(), name: "Tool nose radius".into(), unit: Some("mm".into()) },
            ],
            source: "Machining theory".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Machining
        PhysicalLaw {
            id: "taylor_tool_life".into(),
            name: "Taylor Tool Life Equation".into(),
            domain: PhysicsDomain::Other("Machining".into()),
            equation_tex: r"VT^n = C".into(),
            description: "Cutting speed vs tool life relationship".into(),
            variables: vec![
                Variable { symbol: "V".into(), name: "Cutting speed".into(), unit: Some("m/min".into()) },
                Variable { symbol: "T".into(), name: "Tool life".into(), unit: Some("min".into()) },
                Variable { symbol: "n".into(), name: "Taylor exponent".into(), unit: None },
                Variable { symbol: "C".into(), name: "Taylor constant".into(), unit: None },
            ],
            source: "Taylor 1907".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "cutting_power".into(),
            name: "Cutting Power".into(),
            domain: PhysicsDomain::Other("Machining".into()),
            equation_tex: r"P = F_c \cdot V_c = k_c \cdot A \cdot V_c".into(),
            description: "Power required for metal removal".into(),
            variables: vec![
                Variable { symbol: "P".into(), name: "Power".into(), unit: Some("W".into()) },
                Variable { symbol: "F_c".into(), name: "Cutting force".into(), unit: Some("N".into()) },
                Variable { symbol: "A".into(), name: "Chip cross-section".into(), unit: Some("mm²".into()) },
            ],
            source: "Machining theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "mrr".into(),
            name: "Material Removal Rate".into(),
            domain: PhysicsDomain::Other("Machining".into()),
            equation_tex: r"MRR = V_c \cdot f \cdot d".into(),
            description: "Volume of material removed per unit time".into(),
            variables: vec![
                Variable { symbol: "MRR".into(), name: "Material removal rate".into(), unit: Some("mm³/s".into()) },
                Variable { symbol: "d".into(), name: "Depth of cut".into(), unit: Some("mm".into()) },
            ],
            source: "Machining theory".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Statistical Process Control
        PhysicalLaw {
            id: "control_limits".into(),
            name: "Control Chart Limits".into(),
            domain: PhysicsDomain::Other("SPC".into()),
            equation_tex: r"UCL = \bar{X} + 3\frac{\bar{R}}{d_2}, \quad LCL = \bar{X} - 3\frac{\bar{R}}{d_2}".into(),
            description: "X-bar chart control limits".into(),
            variables: vec![
                Variable { symbol: "UCL, LCL".into(), name: "Upper/lower control limits".into(), unit: None },
                Variable { symbol: "d_2".into(), name: "Control chart constant".into(), unit: None },
            ],
            source: "SPC".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "process_capability".into(),
            name: "Process Capability Index".into(),
            domain: PhysicsDomain::Other("SPC".into()),
            equation_tex: r"C_p = \frac{USL - LSL}{6\sigma}".into(),
            description: "Ratio of tolerance to process spread".into(),
            variables: vec![
                Variable { symbol: "C_p".into(), name: "Process capability".into(), unit: None },
                Variable { symbol: "USL, LSL".into(), name: "Specification limits".into(), unit: None },
            ],
            source: "Quality control".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "cpk".into(),
            name: "Process Capability Index Cpk".into(),
            domain: PhysicsDomain::Other("SPC".into()),
            equation_tex: r"C_{pk} = \min\left(\frac{USL-\mu}{3\sigma}, \frac{\mu-LSL}{3\sigma}\right)".into(),
            description: "Capability considering process centering".into(),
            variables: vec![
                Variable { symbol: "C_pk".into(), name: "Cpk".into(), unit: None },
                Variable { symbol: "μ".into(), name: "Process mean".into(), unit: None },
            ],
            source: "Quality control".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Measurement
        PhysicalLaw {
            id: "gauge_rr".into(),
            name: "Gauge R&R".into(),
            domain: PhysicsDomain::Other("Metrology".into()),
            equation_tex: r"\sigma^2_{total} = \sigma^2_{parts} + \sigma^2_{gauge}".into(),
            description: "Total variance decomposition".into(),
            variables: vec![
                Variable { symbol: "σ_gauge".into(), name: "Gauge variation".into(), unit: None },
                Variable { symbol: "σ_parts".into(), name: "Part-to-part variation".into(), unit: None },
            ],
            source: "MSA".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "measurement_uncertainty".into(),
            name: "Combined Standard Uncertainty".into(),
            domain: PhysicsDomain::Other("Metrology".into()),
            equation_tex: r"u_c = \sqrt{\sum_i u_i^2}".into(),
            description: "Root sum square of uncertainty components".into(),
            variables: vec![
                Variable { symbol: "u_c".into(), name: "Combined uncertainty".into(), unit: None },
                Variable { symbol: "u_i".into(), name: "Individual uncertainties".into(), unit: None },
            ],
            source: "GUM".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "expanded_uncertainty".into(),
            name: "Expanded Uncertainty".into(),
            domain: PhysicsDomain::Other("Metrology".into()),
            equation_tex: r"U = k \cdot u_c".into(),
            description: "Uncertainty at given confidence level".into(),
            variables: vec![
                Variable { symbol: "U".into(), name: "Expanded uncertainty".into(), unit: None },
                Variable { symbol: "k".into(), name: "Coverage factor".into(), unit: None },
            ],
            source: "GUM".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        MathTheorem {
            id: "tolerance_stack".into(),
            name: "Tolerance Stack-Up (RSS)".into(),
            category: TheoremCategory::Other("Tolerancing".into()),
            statement_tex: r"T_{total} = \sqrt{\sum_i T_i^2}".into(),
            proof_tex: Some("Statistical tolerance accumulation (assumes independence)".into()),
            references: vec![],
            source: "Tolerance analysis".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "worst_case_stack".into(),
            name: "Worst-Case Stack-Up".into(),
            category: TheoremCategory::Other("Tolerancing".into()),
            statement_tex: r"T_{total} = \sum_i |T_i|".into(),
            proof_tex: Some("Arithmetic sum guarantees 100% interchangeability".into()),
            references: vec![],
            source: "Tolerance analysis".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "six_sigma_yield".into(),
            name: "Six Sigma Process Yield".into(),
            category: TheoremCategory::Other("Quality".into()),
            statement_tex: r"Y = 2\Phi(k\sigma) - 1 \text{ where } k = 6 \text{ gives } 99.99966\%".into(),
            proof_tex: Some("With 1.5σ shift: 99.99966% yield = 3.4 DPMO".into()),
            references: vec![],
            source: "Six Sigma".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "western_electric_rules".into(),
            name: "Western Electric Rules".into(),
            category: TheoremCategory::Other("SPC".into()),
            statement_tex: r"\text{Out of control if: 1 point > 3σ, 2/3 > 2σ, 4/5 > 1σ, 8 same side}".into(),
            proof_tex: Some("Detection rules for special cause variation".into()),
            references: vec![],
            source: "Western Electric 1956".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "ten_to_one_rule".into(),
            name: "10:1 Measurement Rule".into(),
            category: TheoremCategory::Other("Metrology".into()),
            statement_tex: r"U_{gauge} \leq \frac{Tolerance}{10}".into(),
            proof_tex: Some("Measurement uncertainty should be 10% or less of tolerance".into()),
            references: vec![],
            source: "Metrology practice".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "gdt_hierarchy".into(),
            name: "GD&T Datum Feature Hierarchy".into(),
            category: TheoremCategory::Other("GD&T".into()),
            statement_tex: r"\text{Primary (3 DOF)} \to \text{Secondary (2 DOF)} \to \text{Tertiary (1 DOF)}".into(),
            proof_tex: Some("Datum reference frame from 3-2-1 principle".into()),
            references: vec![],
            source: "ASME Y14.5".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "merchant_model".into(),
            name: "Merchant's Cutting Model".into(),
            category: TheoremCategory::Other("Machining".into()),
            statement_tex: r"\phi = \frac{\pi}{4} - \frac{\beta - \alpha}{2}".into(),
            proof_tex: Some("Shear angle from rake angle and friction angle".into()),
            references: vec![],
            source: "Merchant 1945".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "orthogonal_cutting".into(),
            name: "Orthogonal Cutting Forces".into(),
            category: TheoremCategory::Other("Machining".into()),
            statement_tex: r"F_c = \frac{\tau_s A_s}{\sin\phi\cos(\phi+\beta-\alpha)}".into(),
            proof_tex: Some("Cutting force from shear plane model".into()),
            references: vec![],
            source: "Metal cutting theory".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "abbe_principle".into(),
            name: "Abbe Principle".into(),
            category: TheoremCategory::Other("Metrology".into()),
            statement_tex: r"\text{Measurement axis should coincide with scale axis}".into(),
            proof_tex: Some("Eliminates cosine error and Abbe offset error".into()),
            references: vec![],
            source: "Abbe 1890".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "dmaic".into(),
            name: "DMAIC Process".into(),
            category: TheoremCategory::Other("Quality".into()),
            statement_tex: r"\text{Define} \to \text{Measure} \to \text{Analyze} \to \text{Improve} \to \text{Control}".into(),
            proof_tex: Some("Six Sigma improvement methodology".into()),
            references: vec![],
            source: "Six Sigma".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
