//! Chemical Process Engineering Module for Blondle
//!
//! Covers:
//! - Reactor design (CSTR, PFR, batch)
//! - Separation processes (distillation, extraction)
//! - Mass transfer and permeation
//! - Heat exchanger design
//! - Process control fundamentals

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all chemical process engineering knowledge
pub fn load(kb: &mut KnowledgeBase) {
    let before = (kb.physical_constants.len(), kb.physical_laws.len(), kb.theorems.len());
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::ChemProcess] Loaded {} constants, {} laws, {} theorems",
             kb.physical_constants.len() - before.0,
             kb.physical_laws.len() - before.1,
             kb.theorems.len() - before.2);
}

pub fn count() -> (usize, usize, usize) {
    // Measured, not declared: count what this module actually inserts
    // into a standard (fundamentals-initialized) knowledge base.
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    let before = (kb.physical_constants.len(), kb.physical_laws.len(), kb.theorems.len());
    add_constants(&mut kb);
    add_laws(&mut kb);
    add_theorems(&mut kb);
    (
        kb.physical_constants.len() - before.0,
        kb.physical_laws.len() - before.1,
        kb.theorems.len() - before.2,
    )
}

const CONSTANT_COUNT: usize = 15;
const LAW_COUNT: usize = 18;
const THEOREM_COUNT: usize = 10;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Mass transfer parameters
        PhysicalConstant {
            id: "diff_gas_air".into(),
            name: "Typical gas diffusivity in air".into(),
            symbol: "D_{gas}".into(),
            value: "1e-5".into(),
            unit: "m²/s".into(),
            uncertainty: Some("0.5e-5".into()),
            source: "Transport phenomena".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"D_{gas} \sim 10^{-5} \text{ m}^2\text{/s}".into(),
        },
        PhysicalConstant {
            id: "diff_liquid_water".into(),
            name: "Typical liquid diffusivity in water".into(),
            symbol: "D_{liq}".into(),
            value: "1e-9".into(),
            unit: "m²/s".into(),
            uncertainty: Some("0.5e-9".into()),
            source: "Transport phenomena".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"D_{liq} \sim 10^{-9} \text{ m}^2\text{/s}".into(),
        },
        PhysicalConstant {
            id: "diff_solid".into(),
            name: "Typical solid diffusivity".into(),
            symbol: "D_{solid}".into(),
            value: "1e-14".into(),
            unit: "m²/s".into(),
            uncertainty: Some("varies".into()),
            source: "Materials science".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"D_{solid} \sim 10^{-14} - 10^{-10} \text{ m}^2\text{/s}".into(),
        },
        
        // Membrane permeation
        PhysicalConstant {
            id: "permeability_O2_PDMS".into(),
            name: "O2 permeability in PDMS".into(),
            symbol: "P_{O2}".into(),
            value: "600".into(),
            unit: "Barrer".into(),
            uncertainty: Some("50".into()),
            source: "Membrane data".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"P_{O_2} \approx 600 \text{ Barrer (PDMS at 25°C)}".into(),
        },
        PhysicalConstant {
            id: "barrer_definition".into(),
            name: "Barrer unit definition".into(),
            symbol: "Barrer".into(),
            value: "3.35e-16".into(),
            unit: "mol·m/(m²·s·Pa)".into(),
            uncertainty: None,
            source: "Membrane science".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"1 \text{ Barrer} = 3.35 \times 10^{-16} \text{ mol·m/(m}^2\text{·s·Pa)}".into(),
        },
        
        // Heat transfer
        PhysicalConstant {
            id: "htc_air_free".into(),
            name: "Heat transfer coeff (air, free convection)".into(),
            symbol: "h_{air,free}".into(),
            value: "10".into(),
            unit: "W/(m²·K)".into(),
            uncertainty: Some("5".into()),
            source: "Heat transfer".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"h \approx 5-25 \text{ W/(m}^2\text{·K) (air, free convection)}".into(),
        },
        PhysicalConstant {
            id: "htc_water_forced".into(),
            name: "Heat transfer coeff (water, forced)".into(),
            symbol: "h_{water}".into(),
            value: "3000".into(),
            unit: "W/(m²·K)".into(),
            uncertainty: Some("2000".into()),
            source: "Heat transfer".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"h \approx 500-10000 \text{ W/(m}^2\text{·K) (water, forced)}".into(),
        },
        PhysicalConstant {
            id: "htc_boiling".into(),
            name: "Heat transfer coeff (boiling water)".into(),
            symbol: "h_{boil}".into(),
            value: "10000".into(),
            unit: "W/(m²·K)".into(),
            uncertainty: Some("5000".into()),
            source: "Heat transfer".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"h \approx 2500-25000 \text{ W/(m}^2\text{·K) (boiling)}".into(),
        },
        
        // Reactor parameters
        PhysicalConstant {
            id: "activation_energy_typical".into(),
            name: "Typical activation energy".into(),
            symbol: "E_a".into(),
            value: "50".into(),
            unit: "kJ/mol".into(),
            uncertainty: Some("30".into()),
            source: "Chemical kinetics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"E_a \sim 30-200 \text{ kJ/mol (typical chemical reactions)}".into(),
        },
        PhysicalConstant {
            id: "pre_exp_factor".into(),
            name: "Typical pre-exponential factor".into(),
            symbol: "A".into(),
            value: "1e10".into(),
            unit: "s⁻¹".into(),
            uncertainty: Some("varies widely".into()),
            source: "Chemical kinetics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"A \sim 10^{8} - 10^{14} \text{ s}^{-1}".into(),
        },
        
        // Distillation
        PhysicalConstant {
            id: "murphree_efficiency".into(),
            name: "Murphree tray efficiency (typical)".into(),
            symbol: r"\eta_M".into(),
            value: "0.7".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.2".into()),
            source: "Distillation".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\eta_M \approx 0.5-0.9 \text{ (typical)}".into(),
        },
        PhysicalConstant {
            id: "hetp_packed".into(),
            name: "HETP for structured packing".into(),
            symbol: "HETP".into(),
            value: "0.3".into(),
            unit: "m".into(),
            uncertainty: Some("0.1".into()),
            source: "Distillation".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"HETP \approx 0.1-0.5 \text{ m (structured packing)}".into(),
        },
        
        // Process control
        PhysicalConstant {
            id: "ziegler_nichols_Kc".into(),
            name: "Ziegler-Nichols Kc (PID)".into(),
            symbol: "K_c".into(),
            value: "0.6 K_u".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Process control".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"K_c = 0.6 K_u \text{ (Ziegler-Nichols PID)}".into(),
        },
        PhysicalConstant {
            id: "ziegler_nichols_Ti".into(),
            name: "Ziegler-Nichols Ti (PID)".into(),
            symbol: "T_i".into(),
            value: "P_u / 2".into(),
            unit: "time".into(),
            uncertainty: None,
            source: "Process control".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"T_i = P_u/2 \text{ (Ziegler-Nichols PID)}".into(),
        },
        PhysicalConstant {
            id: "ziegler_nichols_Td".into(),
            name: "Ziegler-Nichols Td (PID)".into(),
            symbol: "T_d".into(),
            value: "P_u / 8".into(),
            unit: "time".into(),
            uncertainty: None,
            source: "Process control".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"T_d = P_u/8 \text{ (Ziegler-Nichols PID)}".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // Mass transfer
        PhysicalLaw {
            id: "fick_first".into(),
            name: "Fick's First Law".into(),
            domain: PhysicsDomain::Other("Mass Transfer".into()),
            equation_tex: r"J = -D\frac{dC}{dx}".into(),
            description: "Diffusive flux proportional to concentration gradient".into(),
            variables: vec![
                Variable { symbol: "J".into(), name: "Molar flux".into(), unit: Some("mol/(m²·s)".into()) },
                Variable { symbol: "D".into(), name: "Diffusivity".into(), unit: Some("m²/s".into()) },
                Variable { symbol: "C".into(), name: "Concentration".into(), unit: Some("mol/m³".into()) },
            ],
            source: "Fick 1855".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "fick_second".into(),
            name: "Fick's Second Law".into(),
            domain: PhysicsDomain::Other("Mass Transfer".into()),
            equation_tex: r"\frac{\partial C}{\partial t} = D\frac{\partial^2 C}{\partial x^2}".into(),
            description: "Transient diffusion equation".into(),
            variables: vec![
                Variable { symbol: "t".into(), name: "Time".into(), unit: Some("s".into()) },
            ],
            source: "Fick 1855".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "membrane_permeation".into(),
            name: "Membrane Permeation".into(),
            domain: PhysicsDomain::Other("Membrane Science".into()),
            equation_tex: r"J = \frac{P}{L}(p_1 - p_2)".into(),
            description: "Flux through membrane".into(),
            variables: vec![
                Variable { symbol: "P".into(), name: "Permeability".into(), unit: Some("mol·m/(m²·s·Pa)".into()) },
                Variable { symbol: "L".into(), name: "Membrane thickness".into(), unit: Some("m".into()) },
                Variable { symbol: "p_1, p_2".into(), name: "Partial pressures".into(), unit: Some("Pa".into()) },
            ],
            source: "Membrane science".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "solution_diffusion".into(),
            name: "Solution-Diffusion Model".into(),
            domain: PhysicsDomain::Other("Membrane Science".into()),
            equation_tex: r"P = S \cdot D".into(),
            description: "Permeability = solubility × diffusivity".into(),
            variables: vec![
                Variable { symbol: "S".into(), name: "Solubility".into(), unit: Some("mol/(m³·Pa)".into()) },
            ],
            source: "Membrane science".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Reactor design
        PhysicalLaw {
            id: "cstr_design".into(),
            name: "CSTR Design Equation".into(),
            domain: PhysicsDomain::Other("Reactor Engineering".into()),
            equation_tex: r"V = \frac{F_{A0}X}{-r_A}".into(),
            description: "Volume of continuous stirred tank reactor".into(),
            variables: vec![
                Variable { symbol: "V".into(), name: "Reactor volume".into(), unit: Some("m³".into()) },
                Variable { symbol: "F_A0".into(), name: "Inlet molar flow".into(), unit: Some("mol/s".into()) },
                Variable { symbol: "X".into(), name: "Conversion".into(), unit: None },
                Variable { symbol: "-r_A".into(), name: "Reaction rate".into(), unit: Some("mol/(m³·s)".into()) },
            ],
            source: "Reaction engineering".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "pfr_design".into(),
            name: "PFR Design Equation".into(),
            domain: PhysicsDomain::Other("Reactor Engineering".into()),
            equation_tex: r"V = F_{A0}\int_0^X \frac{dX}{-r_A}".into(),
            description: "Volume of plug flow reactor".into(),
            variables: vec![
                Variable { symbol: "V".into(), name: "Reactor volume".into(), unit: Some("m³".into()) },
            ],
            source: "Reaction engineering".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "arrhenius".into(),
            name: "Arrhenius Equation".into(),
            domain: PhysicsDomain::Other("Chemical Kinetics".into()),
            equation_tex: r"k = A\exp\left(-\frac{E_a}{RT}\right)".into(),
            description: "Temperature dependence of rate constant".into(),
            variables: vec![
                Variable { symbol: "k".into(), name: "Rate constant".into(), unit: Some("varies".into()) },
                Variable { symbol: "A".into(), name: "Pre-exponential factor".into(), unit: Some("varies".into()) },
                Variable { symbol: "E_a".into(), name: "Activation energy".into(), unit: Some("J/mol".into()) },
            ],
            source: "Arrhenius 1889".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "residence_time".into(),
            name: "Residence Time".into(),
            domain: PhysicsDomain::Other("Reactor Engineering".into()),
            equation_tex: r"\tau = \frac{V}{Q}".into(),
            description: "Average time fluid spends in reactor".into(),
            variables: vec![
                Variable { symbol: "τ".into(), name: "Residence time".into(), unit: Some("s".into()) },
                Variable { symbol: "Q".into(), name: "Volumetric flow".into(), unit: Some("m³/s".into()) },
            ],
            source: "Reaction engineering".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Heat transfer
        PhysicalLaw {
            id: "newton_cooling".into(),
            name: "Newton's Law of Cooling".into(),
            domain: PhysicsDomain::Thermodynamics,
            equation_tex: r"q = hA(T_s - T_\infty)".into(),
            description: "Convective heat transfer".into(),
            variables: vec![
                Variable { symbol: "q".into(), name: "Heat transfer rate".into(), unit: Some("W".into()) },
                Variable { symbol: "h".into(), name: "Heat transfer coefficient".into(), unit: Some("W/(m²·K)".into()) },
            ],
            source: "Heat transfer".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "lmtd".into(),
            name: "Log Mean Temperature Difference".into(),
            domain: PhysicsDomain::Thermodynamics,
            equation_tex: r"\Delta T_{lm} = \frac{\Delta T_1 - \Delta T_2}{\ln(\Delta T_1/\Delta T_2)}".into(),
            description: "Effective temperature difference for heat exchangers".into(),
            variables: vec![
                Variable { symbol: "ΔT_lm".into(), name: "LMTD".into(), unit: Some("K".into()) },
            ],
            source: "Heat exchanger design".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "heat_exchanger_design".into(),
            name: "Heat Exchanger Design Equation".into(),
            domain: PhysicsDomain::Thermodynamics,
            equation_tex: r"Q = UA\Delta T_{lm}".into(),
            description: "Heat duty from overall coefficient and area".into(),
            variables: vec![
                Variable { symbol: "Q".into(), name: "Heat duty".into(), unit: Some("W".into()) },
                Variable { symbol: "U".into(), name: "Overall heat transfer coeff.".into(), unit: Some("W/(m²·K)".into()) },
                Variable { symbol: "A".into(), name: "Heat transfer area".into(), unit: Some("m²".into()) },
            ],
            source: "Heat exchanger design".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "overall_htc".into(),
            name: "Overall Heat Transfer Coefficient".into(),
            domain: PhysicsDomain::Thermodynamics,
            equation_tex: r"\frac{1}{U} = \frac{1}{h_i} + \frac{t}{k} + \frac{1}{h_o}".into(),
            description: "Series resistance analogy".into(),
            variables: vec![
                Variable { symbol: "h_i, h_o".into(), name: "Inside/outside HTC".into(), unit: Some("W/(m²·K)".into()) },
                Variable { symbol: "t".into(), name: "Wall thickness".into(), unit: Some("m".into()) },
                Variable { symbol: "k".into(), name: "Thermal conductivity".into(), unit: Some("W/(m·K)".into()) },
            ],
            source: "Heat transfer".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Distillation
        PhysicalLaw {
            id: "raoults_law".into(),
            name: "Raoult's Law".into(),
            domain: PhysicsDomain::Other("Thermodynamics".into()),
            equation_tex: r"p_i = x_i P_i^{sat}".into(),
            description: "Ideal solution vapor pressure".into(),
            variables: vec![
                Variable { symbol: "p_i".into(), name: "Partial pressure".into(), unit: Some("Pa".into()) },
                Variable { symbol: "x_i".into(), name: "Liquid mole fraction".into(), unit: None },
                Variable { symbol: "P_i^sat".into(), name: "Saturation pressure".into(), unit: Some("Pa".into()) },
            ],
            source: "Raoult 1887".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "relative_volatility".into(),
            name: "Relative Volatility".into(),
            domain: PhysicsDomain::Other("Distillation".into()),
            equation_tex: r"\alpha = \frac{y_i/x_i}{y_j/x_j} = \frac{K_i}{K_j}".into(),
            description: "Ratio of K-values (ease of separation)".into(),
            variables: vec![
                Variable { symbol: "α".into(), name: "Relative volatility".into(), unit: None },
                Variable { symbol: "K".into(), name: "K-value (y/x)".into(), unit: None },
            ],
            source: "Distillation".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "fenske".into(),
            name: "Fenske Equation".into(),
            domain: PhysicsDomain::Other("Distillation".into()),
            equation_tex: r"N_{min} = \frac{\ln[(x_D/x_B)(x_B'/x_D')]}{\ln\alpha}".into(),
            description: "Minimum stages at total reflux".into(),
            variables: vec![
                Variable { symbol: "N_min".into(), name: "Minimum stages".into(), unit: None },
                Variable { symbol: "x_D, x_B".into(), name: "Distillate/bottoms composition".into(), unit: None },
            ],
            source: "Fenske 1932".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "underwood".into(),
            name: "Underwood Equation".into(),
            domain: PhysicsDomain::Other("Distillation".into()),
            equation_tex: r"\sum\frac{\alpha_i x_{F,i}}{\alpha_i - \theta} = 1 - q".into(),
            description: "Minimum reflux ratio calculation".into(),
            variables: vec![
                Variable { symbol: "θ".into(), name: "Underwood root".into(), unit: None },
                Variable { symbol: "q".into(), name: "Feed quality".into(), unit: None },
            ],
            source: "Underwood 1948".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Process control
        PhysicalLaw {
            id: "pid_controller".into(),
            name: "PID Controller".into(),
            domain: PhysicsDomain::Other("Process Control".into()),
            equation_tex: r"u(t) = K_c\left(e + \frac{1}{T_i}\int e\,dt + T_d\frac{de}{dt}\right)".into(),
            description: "Proportional-Integral-Derivative control".into(),
            variables: vec![
                Variable { symbol: "u".into(), name: "Controller output".into(), unit: None },
                Variable { symbol: "e".into(), name: "Error (setpoint - measurement)".into(), unit: None },
                Variable { symbol: "K_c".into(), name: "Controller gain".into(), unit: None },
            ],
            source: "Process control".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        MathTheorem {
            id: "levenspiel_cstr_series".into(),
            name: "CSTRs in Series".into(),
            category: TheoremCategory::Other("Reactor Engineering".into()),
            statement_tex: r"\frac{V_{CSTR}}{V_{PFR}} \to 1 \text{ as } N_{CSTR} \to \infty".into(),
            proof_tex: Some("Series of CSTRs approaches PFR behavior".into()),
            references: vec![],
            source: "Levenspiel".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "damkohler_number".into(),
            name: "Damköhler Number".into(),
            category: TheoremCategory::Other("Reactor Engineering".into()),
            statement_tex: r"Da = \frac{k\tau C^{n-1}}{1} = \frac{\text{reaction rate}}{\text{transport rate}}".into(),
            proof_tex: Some("Da >> 1: reaction-limited; Da << 1: transport-limited".into()),
            references: vec![],
            source: "Reaction engineering".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "thiele_modulus".into(),
            name: "Thiele Modulus".into(),
            category: TheoremCategory::Other("Catalysis".into()),
            statement_tex: r"\phi = L\sqrt{\frac{k}{D_{eff}}}".into(),
            proof_tex: Some("Ratio of reaction to diffusion in catalyst pore".into()),
            references: vec![],
            source: "Catalysis".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "effectiveness_factor".into(),
            name: "Effectiveness Factor".into(),
            category: TheoremCategory::Other("Catalysis".into()),
            statement_tex: r"\eta = \frac{\tanh\phi}{\phi} \text{ (slab geometry)}".into(),
            proof_tex: Some("Actual rate / rate without diffusion limitation".into()),
            references: vec![],
            source: "Catalysis".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "mccabe_thiele".into(),
            name: "McCabe-Thiele Method".into(),
            category: TheoremCategory::Other("Distillation".into()),
            statement_tex: r"\text{Graphical stage-by-stage calculation between equilibrium and operating lines}".into(),
            proof_tex: Some("Determines number of theoretical stages".into()),
            references: vec![],
            source: "McCabe-Thiele 1925".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "gilliland_correlation".into(),
            name: "Gilliland Correlation".into(),
            category: TheoremCategory::Other("Distillation".into()),
            statement_tex: r"\frac{N - N_{min}}{N + 1} = f\left(\frac{R - R_{min}}{R + 1}\right)".into(),
            proof_tex: Some("Empirical correlation for stages vs reflux".into()),
            references: vec![],
            source: "Gilliland 1940".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "ntu_method".into(),
            name: "NTU-Effectiveness Method".into(),
            category: TheoremCategory::Other("Heat Exchangers".into()),
            statement_tex: r"\varepsilon = \frac{Q}{Q_{max}} = f(NTU, C_r)".into(),
            proof_tex: Some("Number of transfer units determines effectiveness".into()),
            references: vec![],
            source: "Heat exchanger design".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "fouling_factor".into(),
            name: "Fouling Resistance".into(),
            category: TheoremCategory::Other("Heat Exchangers".into()),
            statement_tex: r"\frac{1}{U_d} = \frac{1}{U_c} + R_{f,i} + R_{f,o}".into(),
            proof_tex: Some("Dirty coefficient includes fouling resistances".into()),
            references: vec![],
            source: "TEMA".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "control_stability".into(),
            name: "Control Loop Stability".into(),
            category: TheoremCategory::Other("Process Control".into()),
            statement_tex: r"K_c K_p < K_{critical} \text{ for stability}".into(),
            proof_tex: Some("Open-loop gain must not cause instability".into()),
            references: vec![],
            source: "Control theory".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "ideal_selectivity".into(),
            name: "Ideal Membrane Selectivity".into(),
            category: TheoremCategory::Other("Membrane Science".into()),
            statement_tex: r"\alpha_{A/B} = \frac{P_A}{P_B} = \frac{D_A}{D_B} \cdot \frac{S_A}{S_B}".into(),
            proof_tex: Some("Selectivity from diffusivity and solubility ratios".into()),
            references: vec![],
            source: "Membrane science".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
