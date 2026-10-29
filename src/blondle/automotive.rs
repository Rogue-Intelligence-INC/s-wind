//! Automotive Engineering Module for Blondle
//!
//! Covers:
//! - Vehicle dynamics (SAE J670)
//! - Tire mechanics (Pacejka Magic Formula)
//! - Powertrain fundamentals
//! - Crashworthiness standards
//! - Emissions regulations

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all automotive engineering knowledge
pub fn load(kb: &mut KnowledgeBase) {
    let before = (kb.physical_constants.len(), kb.physical_laws.len(), kb.theorems.len());
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::Automotive] Loaded {} constants, {} laws, {} theorems",
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

const CONSTANT_COUNT: usize = 18;
const LAW_COUNT: usize = 15;
const THEOREM_COUNT: usize = 10;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Tire properties (typical passenger car)
        PhysicalConstant {
            id: "tire_friction_dry".into(),
            name: "Tire-road friction (dry asphalt)".into(),
            symbol: r"\mu_{dry}".into(),
            value: "0.9".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.1".into()),
            source: "SAE tire data".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\mu_{dry} \approx 0.8-1.0 \text{ (dry asphalt)}".into(),
        },
        PhysicalConstant {
            id: "tire_friction_wet".into(),
            name: "Tire-road friction (wet asphalt)".into(),
            symbol: r"\mu_{wet}".into(),
            value: "0.6".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.1".into()),
            source: "SAE tire data".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\mu_{wet} \approx 0.5-0.7 \text{ (wet asphalt)}".into(),
        },
        PhysicalConstant {
            id: "tire_friction_ice".into(),
            name: "Tire-road friction (ice)".into(),
            symbol: r"\mu_{ice}".into(),
            value: "0.15".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.05".into()),
            source: "SAE tire data".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\mu_{ice} \approx 0.1-0.2 \text{ (ice)}".into(),
        },
        PhysicalConstant {
            id: "tire_cornering_stiffness".into(),
            name: "Typical cornering stiffness".into(),
            symbol: "C_\\alpha".into(),
            value: "1000".into(),
            unit: "N/deg".into(),
            uncertainty: Some("200".into()),
            source: "Tire mechanics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"C_\alpha \approx 800-1500 \text{ N/deg per tire}".into(),
        },
        PhysicalConstant {
            id: "rolling_resistance".into(),
            name: "Rolling resistance coefficient".into(),
            symbol: "f_r".into(),
            value: "0.012".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.002".into()),
            source: "Tire mechanics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"f_r \approx 0.008-0.015 \text{ (radial tire)}".into(),
        },
        
        // Vehicle parameters
        PhysicalConstant {
            id: "Cd_sedan".into(),
            name: "Drag coefficient (sedan)".into(),
            symbol: "C_{D,sedan}".into(),
            value: "0.30".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.05".into()),
            source: "Automotive aerodynamics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"C_D \approx 0.25-0.35 \text{ (modern sedan)}".into(),
        },
        PhysicalConstant {
            id: "Cd_suv".into(),
            name: "Drag coefficient (SUV)".into(),
            symbol: "C_{D,SUV}".into(),
            value: "0.40".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.05".into()),
            source: "Automotive aerodynamics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"C_D \approx 0.35-0.45 \text{ (SUV)}".into(),
        },
        PhysicalConstant {
            id: "frontal_area_car".into(),
            name: "Typical frontal area (car)".into(),
            symbol: "A_f".into(),
            value: "2.2".into(),
            unit: "m²".into(),
            uncertainty: Some("0.3".into()),
            source: "Automotive design".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"A_f \approx 1.8-2.5 \text{ m}^2 \text{ (passenger car)}".into(),
        },
        
        // Engine parameters
        PhysicalConstant {
            id: "otto_efficiency".into(),
            name: "Otto cycle theoretical efficiency (r=10)".into(),
            symbol: r"\eta_{Otto}".into(),
            value: "0.60".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "Thermodynamics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\eta_{Otto} = 1 - r^{1-\gamma} \approx 0.60 \text{ (r=10)}".into(),
        },
        PhysicalConstant {
            id: "diesel_efficiency".into(),
            name: "Diesel cycle theoretical efficiency".into(),
            symbol: r"\eta_{Diesel}".into(),
            value: "0.55".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.05".into()),
            source: "Thermodynamics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\eta_{Diesel} \approx 0.50-0.60 \text{ (theoretical)}".into(),
        },
        PhysicalConstant {
            id: "BSFC_gasoline".into(),
            name: "BSFC gasoline engine (best point)".into(),
            symbol: "BSFC".into(),
            value: "230".into(),
            unit: "g/kWh".into(),
            uncertainty: Some("20".into()),
            source: "Engine performance".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"BSFC \approx 220-250 \text{ g/kWh (gasoline, best)}".into(),
        },
        PhysicalConstant {
            id: "BSFC_diesel".into(),
            name: "BSFC diesel engine (best point)".into(),
            symbol: "BSFC_{diesel}".into(),
            value: "200".into(),
            unit: "g/kWh".into(),
            uncertainty: Some("15".into()),
            source: "Engine performance".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"BSFC \approx 190-220 \text{ g/kWh (diesel, best)}".into(),
        },
        
        // Fuel properties
        PhysicalConstant {
            id: "LHV_gasoline".into(),
            name: "Lower heating value of gasoline".into(),
            symbol: "LHV_{gas}".into(),
            value: "44".into(),
            unit: "MJ/kg".into(),
            uncertainty: Some("1".into()),
            source: "Fuel properties".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"LHV_{gasoline} \approx 44 \text{ MJ/kg}".into(),
        },
        PhysicalConstant {
            id: "LHV_diesel".into(),
            name: "Lower heating value of diesel".into(),
            symbol: "LHV_{diesel}".into(),
            value: "43".into(),
            unit: "MJ/kg".into(),
            uncertainty: Some("1".into()),
            source: "Fuel properties".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"LHV_{diesel} \approx 43 \text{ MJ/kg}".into(),
        },
        PhysicalConstant {
            id: "rho_gasoline".into(),
            name: "Gasoline density".into(),
            symbol: r"\rho_{gas}".into(),
            value: "750".into(),
            unit: "kg/m³".into(),
            uncertainty: Some("20".into()),
            source: "Fuel properties".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\rho_{gasoline} \approx 720-780 \text{ kg/m}^3".into(),
        },
        
        // Safety standards
        PhysicalConstant {
            id: "crash_velocity_FMVSS".into(),
            name: "FMVSS 208 frontal crash velocity".into(),
            symbol: "V_{crash}".into(),
            value: "56".into(),
            unit: "km/h".into(),
            uncertainty: None,
            source: "FMVSS 208".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"V_{crash} = 56 \text{ km/h} = 35 \text{ mph (FMVSS 208)}".into(),
        },
        PhysicalConstant {
            id: "HIC_limit".into(),
            name: "Head Injury Criterion limit".into(),
            symbol: "HIC_{36}".into(),
            value: "1000".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "FMVSS 208".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"HIC_{36} < 1000 \text{ (FMVSS 208)}".into(),
        },
        PhysicalConstant {
            id: "chest_decel_limit".into(),
            name: "Chest deceleration limit".into(),
            symbol: "a_{chest}".into(),
            value: "60".into(),
            unit: "g".into(),
            uncertainty: None,
            source: "FMVSS 208".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"a_{chest} < 60g \text{ for 3 ms (FMVSS 208)}".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // Vehicle dynamics
        PhysicalLaw {
            id: "bicycle_model".into(),
            name: "Bicycle Model Dynamics".into(),
            domain: PhysicsDomain::Other("Vehicle Dynamics".into()),
            equation_tex: r"m(\dot{v} + u\dot{\psi}) = F_{yf} + F_{yr}".into(),
            description: "Lateral dynamics of 2-DOF vehicle model".into(),
            variables: vec![
                Variable { symbol: "v".into(), name: "Lateral velocity".into(), unit: Some("m/s".into()) },
                Variable { symbol: "ψ".into(), name: "Yaw angle".into(), unit: Some("rad".into()) },
                Variable { symbol: "F_y".into(), name: "Lateral force".into(), unit: Some("N".into()) },
            ],
            source: "SAE J670".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "yaw_rate".into(),
            name: "Steady-State Yaw Rate".into(),
            domain: PhysicsDomain::Other("Vehicle Dynamics".into()),
            equation_tex: r"\dot{\psi}_{ss} = \frac{V/L}{1 + KV^2}\delta".into(),
            description: "Yaw rate in steady-state cornering".into(),
            variables: vec![
                Variable { symbol: "K".into(), name: "Understeer gradient".into(), unit: Some("rad/m/s²".into()) },
                Variable { symbol: "L".into(), name: "Wheelbase".into(), unit: Some("m".into()) },
                Variable { symbol: "δ".into(), name: "Steering angle".into(), unit: Some("rad".into()) },
            ],
            source: "Vehicle dynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "understeer_gradient".into(),
            name: "Understeer Gradient".into(),
            domain: PhysicsDomain::Other("Vehicle Dynamics".into()),
            equation_tex: r"K = \frac{W_f}{C_{\alpha f}} - \frac{W_r}{C_{\alpha r}}".into(),
            description: "Determines vehicle handling characteristic".into(),
            variables: vec![
                Variable { symbol: "W_f, W_r".into(), name: "Front/rear weight".into(), unit: Some("N".into()) },
                Variable { symbol: "C_α".into(), name: "Cornering stiffness".into(), unit: Some("N/rad".into()) },
            ],
            source: "Vehicle dynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "lateral_acceleration".into(),
            name: "Lateral Acceleration (Steady Turn)".into(),
            domain: PhysicsDomain::Other("Vehicle Dynamics".into()),
            equation_tex: r"a_y = \frac{V^2}{R} = V\dot{\psi}".into(),
            description: "Centripetal acceleration in turn".into(),
            variables: vec![
                Variable { symbol: "a_y".into(), name: "Lateral acceleration".into(), unit: Some("m/s²".into()) },
                Variable { symbol: "R".into(), name: "Turn radius".into(), unit: Some("m".into()) },
            ],
            source: "Vehicle dynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Tire mechanics
        PhysicalLaw {
            id: "pacejka_magic".into(),
            name: "Pacejka Magic Formula".into(),
            domain: PhysicsDomain::Other("Tire Mechanics".into()),
            equation_tex: r"F = D\sin(C\arctan(B\alpha - E(B\alpha - \arctan(B\alpha))))".into(),
            description: "Tire force vs slip angle/ratio".into(),
            variables: vec![
                Variable { symbol: "B".into(), name: "Stiffness factor".into(), unit: None },
                Variable { symbol: "C".into(), name: "Shape factor".into(), unit: None },
                Variable { symbol: "D".into(), name: "Peak factor".into(), unit: Some("N".into()) },
                Variable { symbol: "E".into(), name: "Curvature factor".into(), unit: None },
            ],
            source: "Pacejka 1987".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "slip_angle".into(),
            name: "Slip Angle Definition".into(),
            domain: PhysicsDomain::Other("Tire Mechanics".into()),
            equation_tex: r"\alpha = \arctan\frac{v_y}{|v_x|}".into(),
            description: "Angle between tire direction and velocity".into(),
            variables: vec![
                Variable { symbol: "α".into(), name: "Slip angle".into(), unit: Some("rad".into()) },
                Variable { symbol: "v_y".into(), name: "Lateral velocity".into(), unit: Some("m/s".into()) },
            ],
            source: "SAE J670".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "slip_ratio".into(),
            name: "Longitudinal Slip Ratio".into(),
            domain: PhysicsDomain::Other("Tire Mechanics".into()),
            equation_tex: r"\kappa = \frac{\omega r - v_x}{\max(\omega r, v_x)}".into(),
            description: "Slip between tire and road (braking/driving)".into(),
            variables: vec![
                Variable { symbol: "κ".into(), name: "Slip ratio".into(), unit: None },
                Variable { symbol: "ω".into(), name: "Wheel angular velocity".into(), unit: Some("rad/s".into()) },
                Variable { symbol: "r".into(), name: "Wheel radius".into(), unit: Some("m".into()) },
            ],
            source: "SAE J670".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "friction_circle".into(),
            name: "Friction Circle (Ellipse)".into(),
            domain: PhysicsDomain::Other("Tire Mechanics".into()),
            equation_tex: r"\left(\frac{F_x}{\mu_x F_z}\right)^2 + \left(\frac{F_y}{\mu_y F_z}\right)^2 \leq 1".into(),
            description: "Combined slip tire force limit".into(),
            variables: vec![
                Variable { symbol: "F_x, F_y".into(), name: "Longitudinal/lateral force".into(), unit: Some("N".into()) },
                Variable { symbol: "F_z".into(), name: "Normal load".into(), unit: Some("N".into()) },
            ],
            source: "Tire mechanics".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Powertrain
        PhysicalLaw {
            id: "tractive_force".into(),
            name: "Tractive Force at Wheels".into(),
            domain: PhysicsDomain::Other("Powertrain".into()),
            equation_tex: r"F_t = \frac{T_e \cdot i_g \cdot i_0 \cdot \eta_t}{r}".into(),
            description: "Wheel force from engine torque".into(),
            variables: vec![
                Variable { symbol: "T_e".into(), name: "Engine torque".into(), unit: Some("N·m".into()) },
                Variable { symbol: "i_g".into(), name: "Gear ratio".into(), unit: None },
                Variable { symbol: "i_0".into(), name: "Final drive ratio".into(), unit: None },
                Variable { symbol: "η_t".into(), name: "Transmission efficiency".into(), unit: None },
            ],
            source: "Powertrain engineering".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "road_load".into(),
            name: "Road Load Equation".into(),
            domain: PhysicsDomain::Other("Vehicle Performance".into()),
            equation_tex: r"F_{road} = mgf_r\cos\theta + mg\sin\theta + \frac{1}{2}\rho C_D A V^2".into(),
            description: "Total resistance force".into(),
            variables: vec![
                Variable { symbol: "f_r".into(), name: "Rolling resistance coeff.".into(), unit: None },
                Variable { symbol: "θ".into(), name: "Road grade angle".into(), unit: Some("rad".into()) },
            ],
            source: "Vehicle dynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "acceleration_performance".into(),
            name: "Acceleration Performance".into(),
            domain: PhysicsDomain::Other("Vehicle Performance".into()),
            equation_tex: r"a = \frac{F_t - F_{road}}{m_{eff}}".into(),
            description: "Vehicle acceleration".into(),
            variables: vec![
                Variable { symbol: "m_eff".into(), name: "Effective mass (incl. rotational)".into(), unit: Some("kg".into()) },
            ],
            source: "Vehicle performance".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Braking
        PhysicalLaw {
            id: "braking_distance".into(),
            name: "Braking Distance".into(),
            domain: PhysicsDomain::Other("Vehicle Dynamics".into()),
            equation_tex: r"d = \frac{V^2}{2\mu g}".into(),
            description: "Minimum stopping distance".into(),
            variables: vec![
                Variable { symbol: "d".into(), name: "Braking distance".into(), unit: Some("m".into()) },
                Variable { symbol: "μ".into(), name: "Friction coefficient".into(), unit: None },
            ],
            source: "Vehicle dynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "brake_balance".into(),
            name: "Ideal Brake Balance".into(),
            domain: PhysicsDomain::Other("Vehicle Dynamics".into()),
            equation_tex: r"\frac{F_{bf}}{F_{br}} = \frac{l_r + ha/g}{l_f - ha/g}".into(),
            description: "Front/rear brake force ratio for ideal deceleration".into(),
            variables: vec![
                Variable { symbol: "l_f, l_r".into(), name: "CG to front/rear axle".into(), unit: Some("m".into()) },
                Variable { symbol: "h".into(), name: "CG height".into(), unit: Some("m".into()) },
            ],
            source: "Brake system design".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Suspension
        PhysicalLaw {
            id: "ride_frequency".into(),
            name: "Ride Frequency".into(),
            domain: PhysicsDomain::Other("Suspension".into()),
            equation_tex: r"f_n = \frac{1}{2\pi}\sqrt{\frac{K}{m}}".into(),
            description: "Natural frequency of suspension".into(),
            variables: vec![
                Variable { symbol: "f_n".into(), name: "Natural frequency".into(), unit: Some("Hz".into()) },
                Variable { symbol: "K".into(), name: "Spring rate".into(), unit: Some("N/m".into()) },
            ],
            source: "Suspension design".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "damping_ratio".into(),
            name: "Critical Damping Ratio".into(),
            domain: PhysicsDomain::Other("Suspension".into()),
            equation_tex: r"\zeta = \frac{C}{2\sqrt{Km}}".into(),
            description: "Damping relative to critical".into(),
            variables: vec![
                Variable { symbol: "ζ".into(), name: "Damping ratio".into(), unit: None },
                Variable { symbol: "C".into(), name: "Damping coefficient".into(), unit: Some("N·s/m".into()) },
            ],
            source: "Suspension design".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        MathTheorem {
            id: "neutral_steer".into(),
            name: "Neutral Steer Condition".into(),
            category: TheoremCategory::Other("Vehicle Dynamics".into()),
            statement_tex: r"K = 0 \Rightarrow \text{neutral steer}, \quad K > 0 \Rightarrow \text{understeer}".into(),
            proof_tex: Some("K < 0 means oversteer (unstable at high speed)".into()),
            references: vec!["understeer_gradient".into()],
            source: "Vehicle dynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "critical_speed".into(),
            name: "Critical Speed (Oversteer)".into(),
            category: TheoremCategory::Other("Vehicle Dynamics".into()),
            statement_tex: r"V_{crit} = \sqrt{-\frac{L^2}{K}} \text{ for } K < 0".into(),
            proof_tex: Some("Speed at which oversteer vehicle becomes unstable".into()),
            references: vec![],
            source: "Vehicle dynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "weight_transfer".into(),
            name: "Lateral Weight Transfer".into(),
            category: TheoremCategory::Other("Vehicle Dynamics".into()),
            statement_tex: r"\Delta F_z = \frac{ma_y h}{t}".into(),
            proof_tex: Some("Load transfer from inner to outer wheels in turn".into()),
            references: vec![],
            source: "Vehicle dynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "tire_load_sensitivity".into(),
            name: "Tire Load Sensitivity".into(),
            category: TheoremCategory::Other("Tire Mechanics".into()),
            statement_tex: r"\frac{dF_y/F_z}{dF_z} < 0".into(),
            proof_tex: Some("Normalized lateral force decreases with increasing load".into()),
            references: vec![],
            source: "Tire mechanics".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "gear_ratio_selection".into(),
            name: "Gear Ratio Selection".into(),
            category: TheoremCategory::Other("Powertrain".into()),
            statement_tex: r"i_{n+1}/i_n = \text{const} \text{ (geometric progression)}".into(),
            proof_tex: Some("Equal rpm drop between shifts".into()),
            references: vec![],
            source: "Transmission design".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "ride_comfort".into(),
            name: "Ride Comfort Criteria".into(),
            category: TheoremCategory::Other("Suspension".into()),
            statement_tex: r"f_n \approx 1-1.5 \text{ Hz (passenger car)}".into(),
            proof_tex: Some("Human comfort optimum; sports car 1.5-2.5 Hz".into()),
            references: vec![],
            source: "Suspension design".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "damping_optimal".into(),
            name: "Optimal Damping Ratio".into(),
            category: TheoremCategory::Other("Suspension".into()),
            statement_tex: r"\zeta \approx 0.2-0.4 \text{ (comfort)}, 0.5-0.7 \text{ (handling)}".into(),
            proof_tex: Some("Trade-off between comfort and tire contact".into()),
            references: vec![],
            source: "Suspension design".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "abs_slip_target".into(),
            name: "ABS Target Slip".into(),
            category: TheoremCategory::Other("Brake Systems".into()),
            statement_tex: r"\kappa_{target} \approx 0.10-0.20".into(),
            proof_tex: Some("Slip ratio for maximum braking force".into()),
            references: vec![],
            source: "ABS control".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "hic_calculation".into(),
            name: "HIC Calculation".into(),
            category: TheoremCategory::Other("Crashworthiness".into()),
            statement_tex: r"HIC = \max\left[(t_2-t_1)\left(\frac{1}{t_2-t_1}\int_{t_1}^{t_2}a(t)dt\right)^{2.5}\right]".into(),
            proof_tex: Some("Head Injury Criterion over 36 ms window".into()),
            references: vec![],
            source: "FMVSS 208".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "emissions_stoich".into(),
            name: "Stoichiometric Air-Fuel Ratio".into(),
            category: TheoremCategory::Other("Emissions".into()),
            statement_tex: r"AFR_{stoich} = 14.7 \text{ (gasoline)}, 14.5 \text{ (diesel)}".into(),
            proof_tex: Some("Mass ratio for complete combustion".into()),
            references: vec![],
            source: "Engine combustion".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
