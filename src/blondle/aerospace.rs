//! Aerospace Engineering Module for Blondle
//!
//! Covers:
//! - Wind tunnel testing standards (SAE, AIAA)
//! - Aerodynamic coefficients and parameters
//! - Aircraft design formulas
//! - Propulsion fundamentals
//! - Flight mechanics

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all aerospace engineering knowledge
pub fn load(kb: &mut KnowledgeBase) {
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::Aerospace] Loaded {} constants, {} laws, {} theorems",
             CONSTANT_COUNT, LAW_COUNT, THEOREM_COUNT);
}

pub fn count() -> (usize, usize, usize) {
    (CONSTANT_COUNT, LAW_COUNT, THEOREM_COUNT)
}

const CONSTANT_COUNT: usize = 20;
const LAW_COUNT: usize = 18;
const THEOREM_COUNT: usize = 12;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Standard atmosphere (ISA)
        PhysicalConstant {
            id: "ISA_T0".into(),
            name: "ISA sea level temperature".into(),
            symbol: "T_0".into(),
            value: "288.15".into(),
            unit: "K".into(),
            uncertainty: Some("exact".into()),
            source: "ICAO Standard Atmosphere".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"T_0 = 288.15 \text{ K} = 15°\text{C}".into(),
        },
        PhysicalConstant {
            id: "ISA_P0".into(),
            name: "ISA sea level pressure".into(),
            symbol: "P_0".into(),
            value: "101325".into(),
            unit: "Pa".into(),
            uncertainty: Some("exact".into()),
            source: "ICAO Standard Atmosphere".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"P_0 = 101325 \text{ Pa}".into(),
        },
        PhysicalConstant {
            id: "ISA_rho0".into(),
            name: "ISA sea level density".into(),
            symbol: r"\rho_0".into(),
            value: "1.225".into(),
            unit: "kg/m³".into(),
            uncertainty: Some("0.001".into()),
            source: "ICAO Standard Atmosphere".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\rho_0 = 1.225 \text{ kg/m}^3".into(),
        },
        PhysicalConstant {
            id: "ISA_a0".into(),
            name: "ISA sea level speed of sound".into(),
            symbol: "a_0".into(),
            value: "340.29".into(),
            unit: "m/s".into(),
            uncertainty: Some("0.01".into()),
            source: "ICAO Standard Atmosphere".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"a_0 = 340.29 \text{ m/s}".into(),
        },
        PhysicalConstant {
            id: "ISA_lapse_rate".into(),
            name: "ISA temperature lapse rate".into(),
            symbol: "L".into(),
            value: "0.0065".into(),
            unit: "K/m".into(),
            uncertainty: Some("exact".into()),
            source: "ICAO Standard Atmosphere".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"L = 0.0065 \text{ K/m} = 6.5 \text{ K/km}".into(),
        },
        PhysicalConstant {
            id: "tropopause_alt".into(),
            name: "Tropopause altitude (ISA)".into(),
            symbol: "h_{trop}".into(),
            value: "11000".into(),
            unit: "m".into(),
            uncertainty: Some("exact".into()),
            source: "ICAO Standard Atmosphere".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"h_{tropopause} = 11000 \text{ m}".into(),
        },
        
        // Typical aerodynamic coefficients
        PhysicalConstant {
            id: "Cd_sphere".into(),
            name: "Drag coefficient of sphere".into(),
            symbol: "C_{D,sphere}".into(),
            value: "0.47".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.02".into()),
            source: "Aerodynamics handbooks".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"C_D \approx 0.47 \text{ (sphere, Re } > 10^3\text{)}".into(),
        },
        PhysicalConstant {
            id: "Cd_flat_plate".into(),
            name: "Drag coefficient flat plate (normal)".into(),
            symbol: "C_{D,plate}".into(),
            value: "1.28".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.05".into()),
            source: "Aerodynamics handbooks".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"C_D \approx 1.28 \text{ (flat plate, normal to flow)}".into(),
        },
        PhysicalConstant {
            id: "Cl_max_clean".into(),
            name: "Typical max lift coefficient (clean wing)".into(),
            symbol: "C_{L,max}".into(),
            value: "1.5".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.3".into()),
            source: "Aircraft design".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"C_{L,max} \approx 1.2-1.8 \text{ (clean wing)}".into(),
        },
        PhysicalConstant {
            id: "Cl_max_flaps".into(),
            name: "Typical max lift coefficient (with flaps)".into(),
            symbol: "C_{L,max,flaps}".into(),
            value: "2.5".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.5".into()),
            source: "Aircraft design".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"C_{L,max} \approx 2.0-3.0 \text{ (with flaps)}".into(),
        },
        
        // Wind tunnel parameters
        PhysicalConstant {
            id: "turbulence_intensity_good".into(),
            name: "Good wind tunnel turbulence intensity".into(),
            symbol: "Tu".into(),
            value: "0.001".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Wind tunnel standards".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"Tu < 0.1\% \text{ (research quality)}".into(),
        },
        PhysicalConstant {
            id: "blockage_limit".into(),
            name: "Wind tunnel blockage limit".into(),
            symbol: r"\beta".into(),
            value: "0.05".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "AIAA standards".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\beta = A_{model}/A_{tunnel} < 5\%".into(),
        },
        
        // Aircraft performance
        PhysicalConstant {
            id: "L_D_glider".into(),
            name: "Glider L/D ratio".into(),
            symbol: "(L/D)_{glider}".into(),
            value: "40".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("10".into()),
            source: "Aircraft performance".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"(L/D)_{glider} \approx 30-60".into(),
        },
        PhysicalConstant {
            id: "L_D_transport".into(),
            name: "Transport aircraft L/D ratio".into(),
            symbol: "(L/D)_{transport}".into(),
            value: "18".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("2".into()),
            source: "Aircraft performance".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"(L/D)_{transport} \approx 15-20".into(),
        },
        PhysicalConstant {
            id: "L_D_fighter".into(),
            name: "Fighter aircraft L/D ratio".into(),
            symbol: "(L/D)_{fighter}".into(),
            value: "8".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("2".into()),
            source: "Aircraft performance".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"(L/D)_{fighter} \approx 6-10".into(),
        },
        
        // Propulsion
        PhysicalConstant {
            id: "TSFC_turbofan".into(),
            name: "Turbofan TSFC (high bypass)".into(),
            symbol: "TSFC".into(),
            value: "0.5".into(),
            unit: "lb/(lbf·hr)".into(),
            uncertainty: Some("0.1".into()),
            source: "Propulsion systems".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"TSFC \approx 0.4-0.6 \text{ lb/(lbf·hr)}".into(),
        },
        PhysicalConstant {
            id: "eta_prop".into(),
            name: "Propeller efficiency (cruise)".into(),
            symbol: r"\eta_{prop}".into(),
            value: "0.85".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.05".into()),
            source: "Propulsion systems".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\eta_{prop} \approx 0.80-0.90 \text{ (cruise)}".into(),
        },
        
        // Structural
        PhysicalConstant {
            id: "load_factor_transport".into(),
            name: "Transport aircraft limit load factor".into(),
            symbol: "n_{limit}".into(),
            value: "2.5".into(),
            unit: "g".into(),
            uncertainty: None,
            source: "FAR Part 25".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"n_{limit} = +2.5/-1.0 \text{ (FAR 25)}".into(),
        },
        PhysicalConstant {
            id: "safety_factor_aircraft".into(),
            name: "Aircraft ultimate safety factor".into(),
            symbol: "SF".into(),
            value: "1.5".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "FAR Part 25".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"n_{ultimate} = 1.5 \times n_{limit}".into(),
        },
        PhysicalConstant {
            id: "gust_velocity".into(),
            name: "Design gust velocity (cruise)".into(),
            symbol: "U_{de}".into(),
            value: "15.24".into(),
            unit: "m/s".into(),
            uncertainty: None,
            source: "FAR Part 25".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"U_{de} = 50 \text{ ft/s} = 15.24 \text{ m/s (cruise)}".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // Basic aerodynamics
        PhysicalLaw {
            id: "lift_equation".into(),
            name: "Lift Equation".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"L = \frac{1}{2}\rho V^2 S C_L".into(),
            description: "Aerodynamic lift force".into(),
            variables: vec![
                Variable { symbol: "L".into(), name: "Lift force".into(), unit: Some("N".into()) },
                Variable { symbol: "C_L".into(), name: "Lift coefficient".into(), unit: None },
                Variable { symbol: "S".into(), name: "Wing area".into(), unit: Some("m²".into()) },
            ],
            source: "Aerodynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "drag_equation_aero".into(),
            name: "Drag Equation".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"D = \frac{1}{2}\rho V^2 S C_D".into(),
            description: "Aerodynamic drag force".into(),
            variables: vec![
                Variable { symbol: "D".into(), name: "Drag force".into(), unit: Some("N".into()) },
                Variable { symbol: "C_D".into(), name: "Drag coefficient".into(), unit: None },
            ],
            source: "Aerodynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "drag_polar".into(),
            name: "Parabolic Drag Polar".into(),
            domain: PhysicsDomain::Other("Aerodynamics".into()),
            equation_tex: r"C_D = C_{D0} + \frac{C_L^2}{\pi e AR}".into(),
            description: "Drag coefficient vs lift coefficient".into(),
            variables: vec![
                Variable { symbol: "C_{D0}".into(), name: "Zero-lift drag".into(), unit: None },
                Variable { symbol: "e".into(), name: "Oswald efficiency".into(), unit: None },
                Variable { symbol: "AR".into(), name: "Aspect ratio".into(), unit: None },
            ],
            source: "Aerodynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "oswald_efficiency".into(),
            name: "Oswald Efficiency Approximation".into(),
            domain: PhysicsDomain::Other("Aerodynamics".into()),
            equation_tex: r"e \approx 1.78(1 - 0.045 AR^{0.68}) - 0.64".into(),
            description: "Oswald span efficiency estimation".into(),
            variables: vec![
                Variable { symbol: "e".into(), name: "Oswald efficiency".into(), unit: None },
                Variable { symbol: "AR".into(), name: "Aspect ratio".into(), unit: None },
            ],
            source: "Raymer Aircraft Design".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "induced_drag".into(),
            name: "Induced Drag Coefficient".into(),
            domain: PhysicsDomain::Other("Aerodynamics".into()),
            equation_tex: r"C_{Di} = \frac{C_L^2}{\pi e AR}".into(),
            description: "Drag induced by lift".into(),
            variables: vec![
                Variable { symbol: "C_{Di}".into(), name: "Induced drag coefficient".into(), unit: None },
            ],
            source: "Lifting line theory".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Thin airfoil theory
        PhysicalLaw {
            id: "thin_airfoil_cl".into(),
            name: "Thin Airfoil Lift Slope".into(),
            domain: PhysicsDomain::Other("Aerodynamics".into()),
            equation_tex: r"C_{l\alpha} = 2\pi \text{ rad}^{-1}".into(),
            description: "2D lift curve slope (incompressible)".into(),
            variables: vec![
                Variable { symbol: "C_{lα}".into(), name: "Lift slope".into(), unit: Some("rad⁻¹".into()) },
            ],
            source: "Thin airfoil theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "prandtl_glauert".into(),
            name: "Prandtl-Glauert Compressibility".into(),
            domain: PhysicsDomain::Other("Aerodynamics".into()),
            equation_tex: r"C_{l\alpha} = \frac{2\pi}{\sqrt{1-M^2}}".into(),
            description: "Compressibility correction (subsonic)".into(),
            variables: vec![
                Variable { symbol: "M".into(), name: "Mach number".into(), unit: None },
            ],
            source: "Compressible aerodynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Propulsion
        PhysicalLaw {
            id: "thrust_equation".into(),
            name: "Jet Thrust Equation".into(),
            domain: PhysicsDomain::Other("Propulsion".into()),
            equation_tex: r"T = \dot{m}(V_e - V_0) + (P_e - P_0)A_e".into(),
            description: "Jet engine thrust".into(),
            variables: vec![
                Variable { symbol: "T".into(), name: "Thrust".into(), unit: Some("N".into()) },
                Variable { symbol: "ṁ".into(), name: "Mass flow rate".into(), unit: Some("kg/s".into()) },
                Variable { symbol: "V_e".into(), name: "Exit velocity".into(), unit: Some("m/s".into()) },
            ],
            source: "Propulsion".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "propeller_thrust".into(),
            name: "Propeller Thrust".into(),
            domain: PhysicsDomain::Other("Propulsion".into()),
            equation_tex: r"T = \frac{\eta_{prop} P}{V}".into(),
            description: "Propeller thrust from power".into(),
            variables: vec![
                Variable { symbol: "P".into(), name: "Shaft power".into(), unit: Some("W".into()) },
                Variable { symbol: "η_prop".into(), name: "Propeller efficiency".into(), unit: None },
            ],
            source: "Propulsion".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "specific_impulse".into(),
            name: "Specific Impulse".into(),
            domain: PhysicsDomain::Other("Propulsion".into()),
            equation_tex: r"I_{sp} = \frac{T}{\dot{m} g_0}".into(),
            description: "Thrust per unit weight flow".into(),
            variables: vec![
                Variable { symbol: "I_sp".into(), name: "Specific impulse".into(), unit: Some("s".into()) },
            ],
            source: "Rocket propulsion".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "tsiolkovsky".into(),
            name: "Tsiolkovsky Rocket Equation".into(),
            domain: PhysicsDomain::Other("Propulsion".into()),
            equation_tex: r"\Delta V = I_{sp} g_0 \ln\frac{m_0}{m_f}".into(),
            description: "Ideal rocket velocity change".into(),
            variables: vec![
                Variable { symbol: "ΔV".into(), name: "Velocity change".into(), unit: Some("m/s".into()) },
                Variable { symbol: "m_0/m_f".into(), name: "Mass ratio".into(), unit: None },
            ],
            source: "Tsiolkovsky 1903".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Flight performance
        PhysicalLaw {
            id: "stall_speed".into(),
            name: "Stall Speed".into(),
            domain: PhysicsDomain::Other("Flight Mechanics".into()),
            equation_tex: r"V_s = \sqrt{\frac{2W}{\rho S C_{L,max}}}".into(),
            description: "Minimum flight speed".into(),
            variables: vec![
                Variable { symbol: "V_s".into(), name: "Stall speed".into(), unit: Some("m/s".into()) },
                Variable { symbol: "W".into(), name: "Weight".into(), unit: Some("N".into()) },
            ],
            source: "Flight mechanics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "rate_of_climb".into(),
            name: "Rate of Climb".into(),
            domain: PhysicsDomain::Other("Flight Mechanics".into()),
            equation_tex: r"RC = \frac{(T-D)V}{W} = \frac{P_{excess}}{W}".into(),
            description: "Climb rate from excess power".into(),
            variables: vec![
                Variable { symbol: "RC".into(), name: "Rate of climb".into(), unit: Some("m/s".into()) },
                Variable { symbol: "P_excess".into(), name: "Excess power".into(), unit: Some("W".into()) },
            ],
            source: "Flight mechanics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "range_breguet".into(),
            name: "Breguet Range Equation".into(),
            domain: PhysicsDomain::Other("Flight Mechanics".into()),
            equation_tex: r"R = \frac{V}{c}\frac{L}{D}\ln\frac{W_1}{W_2}".into(),
            description: "Cruise range for jet aircraft".into(),
            variables: vec![
                Variable { symbol: "R".into(), name: "Range".into(), unit: Some("m".into()) },
                Variable { symbol: "c".into(), name: "Specific fuel consumption".into(), unit: Some("kg/(N·s)".into()) },
            ],
            source: "Breguet".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "endurance".into(),
            name: "Endurance Equation".into(),
            domain: PhysicsDomain::Other("Flight Mechanics".into()),
            equation_tex: r"E = \frac{1}{c}\frac{L}{D}\ln\frac{W_1}{W_2}".into(),
            description: "Maximum flight time".into(),
            variables: vec![
                Variable { symbol: "E".into(), name: "Endurance".into(), unit: Some("s".into()) },
            ],
            source: "Flight mechanics".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Wind tunnel corrections
        PhysicalLaw {
            id: "blockage_correction".into(),
            name: "Solid Blockage Correction".into(),
            domain: PhysicsDomain::Other("Wind Tunnel".into()),
            equation_tex: r"\frac{\Delta V}{V} = \epsilon_s = K_1 \frac{V_{model}}{C^{3/2}}".into(),
            description: "Velocity correction for model volume".into(),
            variables: vec![
                Variable { symbol: "ε_s".into(), name: "Solid blockage".into(), unit: None },
                Variable { symbol: "V_model".into(), name: "Model volume".into(), unit: Some("m³".into()) },
                Variable { symbol: "C".into(), name: "Tunnel cross-section".into(), unit: Some("m²".into()) },
            ],
            source: "AIAA wind tunnel standards".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "wall_interference".into(),
            name: "Wall Interference Correction".into(),
            domain: PhysicsDomain::Other("Wind Tunnel".into()),
            equation_tex: r"\Delta\alpha = \delta \frac{S}{C} C_L".into(),
            description: "Angle of attack correction".into(),
            variables: vec![
                Variable { symbol: "Δα".into(), name: "Angle correction".into(), unit: Some("rad".into()) },
                Variable { symbol: "δ".into(), name: "Boundary correction factor".into(), unit: None },
            ],
            source: "Wind tunnel testing".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        MathTheorem {
            id: "kutta_joukowski".into(),
            name: "Kutta-Joukowski Theorem".into(),
            category: TheoremCategory::Other("Aerodynamics".into()),
            statement_tex: r"L' = \rho V \Gamma".into(),
            proof_tex: Some("Lift per unit span equals density × velocity × circulation".into()),
            references: vec![],
            source: "Kutta 1902, Joukowski 1906".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "kutta_condition".into(),
            name: "Kutta Condition".into(),
            category: TheoremCategory::Other("Aerodynamics".into()),
            statement_tex: r"\text{Flow leaves trailing edge smoothly (finite velocity)}".into(),
            proof_tex: Some("Determines unique circulation for given angle of attack".into()),
            references: vec!["kutta_joukowski".into()],
            source: "Potential flow theory".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "helmholtz_vortex".into(),
            name: "Helmholtz Vortex Theorems".into(),
            category: TheoremCategory::Other("Fluid Mechanics".into()),
            statement_tex: r"\text{Vortex lines move with fluid; circulation constant along vortex tube}".into(),
            proof_tex: Some("Foundation for trailing vortex theory".into()),
            references: vec![],
            source: "Helmholtz 1858".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "lifting_line".into(),
            name: "Prandtl Lifting Line Theory".into(),
            category: TheoremCategory::Other("Aerodynamics".into()),
            statement_tex: r"C_L = \frac{C_{l\alpha} AR}{2 + \sqrt{4 + AR^2(1 + \tan^2\Lambda)}}".into(),
            proof_tex: Some("3D wing lift from 2D airfoil data".into()),
            references: vec![],
            source: "Prandtl 1918".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "elliptic_loading".into(),
            name: "Elliptic Lift Distribution".into(),
            category: TheoremCategory::Other("Aerodynamics".into()),
            statement_tex: r"\Gamma(y) = \Gamma_0 \sqrt{1 - (2y/b)^2}".into(),
            proof_tex: Some("Minimizes induced drag for given lift".into()),
            references: vec![],
            source: "Lifting line theory".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "area_rule".into(),
            name: "Whitcomb Area Rule".into(),
            category: TheoremCategory::Other("Aerodynamics".into()),
            statement_tex: r"\text{Transonic drag minimized when cross-section varies smoothly}".into(),
            proof_tex: Some("Total area distribution should resemble Sears-Haack body".into()),
            references: vec![],
            source: "Whitcomb 1952".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "critical_mach".into(),
            name: "Critical Mach Number".into(),
            category: TheoremCategory::Other("Aerodynamics".into()),
            statement_tex: r"M_{cr}: \text{ first point reaches } M_{local} = 1".into(),
            proof_tex: Some("Onset of compressibility drag rise".into()),
            references: vec![],
            source: "Compressible aerodynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "prandtl_meyer".into(),
            name: "Prandtl-Meyer Expansion".into(),
            category: TheoremCategory::Other("Aerodynamics".into()),
            statement_tex: r"\nu(M) = \sqrt{\frac{\gamma+1}{\gamma-1}}\arctan\sqrt{\frac{\gamma-1}{\gamma+1}(M^2-1)} - \arctan\sqrt{M^2-1}".into(),
            proof_tex: Some("Isentropic expansion around corner".into()),
            references: vec![],
            source: "Prandtl, Meyer 1908".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "shock_relations".into(),
            name: "Normal Shock Relations".into(),
            category: TheoremCategory::Other("Aerodynamics".into()),
            statement_tex: r"\frac{P_2}{P_1} = 1 + \frac{2\gamma}{\gamma+1}(M_1^2 - 1)".into(),
            proof_tex: Some("Pressure ratio across normal shock".into()),
            references: vec![],
            source: "Rankine-Hugoniot".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "phugoid".into(),
            name: "Phugoid Mode".into(),
            category: TheoremCategory::Other("Flight Dynamics".into()),
            statement_tex: r"\omega_p \approx \frac{g\sqrt{2}}{V}, \quad \zeta_p \approx \frac{1}{\sqrt{2}}\frac{C_D}{C_L}".into(),
            proof_tex: Some("Long-period oscillation in pitch, exchange of kinetic/potential energy".into()),
            references: vec![],
            source: "Flight dynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "short_period".into(),
            name: "Short Period Mode".into(),
            category: TheoremCategory::Other("Flight Dynamics".into()),
            statement_tex: r"\omega_{sp} \approx \sqrt{\frac{Z_\alpha M_q}{U_0}}, \quad \zeta_{sp} \approx -\frac{M_q + M_{\dot\alpha} + Z_\alpha/U_0}{2\omega_{sp}}".into(),
            proof_tex: Some("Rapid pitch oscillation, damped by horizontal tail".into()),
            references: vec![],
            source: "Flight dynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "v_n_diagram".into(),
            name: "V-n Diagram Limits".into(),
            category: TheoremCategory::Other("Aircraft Structures".into()),
            statement_tex: r"n_{max} = \frac{\rho V^2 S C_{L,max}}{2W}, \quad n_{gust} = 1 + \frac{\rho V U_{de} C_{L\alpha} S}{2W}".into(),
            proof_tex: Some("Structural limits vs airspeed".into()),
            references: vec![],
            source: "FAR Part 25".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
