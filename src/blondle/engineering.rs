//! Engineering Module for Blondle
//!
//! Covers:
//! - Electrical engineering (circuits, signals)
//! - Mechanical engineering (stress, strain, materials)
//! - Civil/Structural engineering
//! - Control systems
//! - Fluid mechanics

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all engineering knowledge
pub fn load(kb: &mut KnowledgeBase) {
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::Engineering] Loaded {} constants, {} laws, {} theorems",
             CONSTANT_COUNT, LAW_COUNT, THEOREM_COUNT);
}

pub fn count() -> (usize, usize, usize) {
    (CONSTANT_COUNT, LAW_COUNT, THEOREM_COUNT)
}

const CONSTANT_COUNT: usize = 15;
const LAW_COUNT: usize = 25;
const THEOREM_COUNT: usize = 15;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Material properties
        PhysicalConstant {
            id: "E_steel".into(),
            name: "Young's modulus of steel".into(),
            symbol: "E_{steel}".into(),
            value: "200".into(),
            unit: "GPa".into(),
            uncertainty: Some("10".into()),
            source: "Engineering handbook".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"E_{steel} \approx 200 \text{ GPa}".into(),
        },
        PhysicalConstant {
            id: "E_aluminum".into(),
            name: "Young's modulus of aluminum".into(),
            symbol: "E_{Al}".into(),
            value: "70".into(),
            unit: "GPa".into(),
            uncertainty: Some("5".into()),
            source: "Engineering handbook".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"E_{Al} \approx 70 \text{ GPa}".into(),
        },
        PhysicalConstant {
            id: "E_concrete".into(),
            name: "Young's modulus of concrete".into(),
            symbol: "E_{concrete}".into(),
            value: "30".into(),
            unit: "GPa".into(),
            uncertainty: Some("5".into()),
            source: "Engineering handbook".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"E_{concrete} \approx 25-35 \text{ GPa}".into(),
        },
        PhysicalConstant {
            id: "nu_steel".into(),
            name: "Poisson's ratio of steel".into(),
            symbol: r"\nu_{steel}".into(),
            value: "0.30".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.01".into()),
            source: "Engineering handbook".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\nu_{steel} \approx 0.30".into(),
        },
        PhysicalConstant {
            id: "yield_steel".into(),
            name: "Yield strength of mild steel".into(),
            symbol: r"\sigma_y".into(),
            value: "250".into(),
            unit: "MPa".into(),
            uncertainty: Some("20".into()),
            source: "Engineering handbook".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\sigma_y \approx 250 \text{ MPa (mild steel)}".into(),
        },
        
        // Fluid properties
        PhysicalConstant {
            id: "rho_water".into(),
            name: "Density of water at 20°C".into(),
            symbol: r"\rho_{H_2O}".into(),
            value: "998".into(),
            unit: "kg/m³".into(),
            uncertainty: Some("1".into()),
            source: "Standard".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\rho_{H_2O} = 998 \text{ kg/m}^3 \text{ at 20°C}".into(),
        },
        PhysicalConstant {
            id: "rho_air".into(),
            name: "Density of air at STP".into(),
            symbol: r"\rho_{air}".into(),
            value: "1.225".into(),
            unit: "kg/m³".into(),
            uncertainty: Some("0.005".into()),
            source: "ISA".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\rho_{air} = 1.225 \text{ kg/m}^3 \text{ at STP}".into(),
        },
        PhysicalConstant {
            id: "mu_water".into(),
            name: "Dynamic viscosity of water at 20°C".into(),
            symbol: r"\mu_{H_2O}".into(),
            value: "1.002e-3".into(),
            unit: "Pa·s".into(),
            uncertainty: Some("0.001e-3".into()),
            source: "Standard".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\mu_{H_2O} = 1.002 \times 10^{-3} \text{ Pa·s}".into(),
        },
        PhysicalConstant {
            id: "mu_air".into(),
            name: "Dynamic viscosity of air at 20°C".into(),
            symbol: r"\mu_{air}".into(),
            value: "1.81e-5".into(),
            unit: "Pa·s".into(),
            uncertainty: Some("0.01e-5".into()),
            source: "Standard".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\mu_{air} = 1.81 \times 10^{-5} \text{ Pa·s}".into(),
        },
        
        // Electrical
        PhysicalConstant {
            id: "rho_copper".into(),
            name: "Electrical resistivity of copper".into(),
            symbol: r"\rho_{Cu}".into(),
            value: "1.68e-8".into(),
            unit: "Ω·m".into(),
            uncertainty: Some("0.01e-8".into()),
            source: "Engineering handbook".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\rho_{Cu} = 1.68 \times 10^{-8} \text{ Ω·m}".into(),
        },
        PhysicalConstant {
            id: "rho_aluminum_elec".into(),
            name: "Electrical resistivity of aluminum".into(),
            symbol: r"\rho_{Al,elec}".into(),
            value: "2.65e-8".into(),
            unit: "Ω·m".into(),
            uncertainty: Some("0.01e-8".into()),
            source: "Engineering handbook".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\rho_{Al} = 2.65 \times 10^{-8} \text{ Ω·m}".into(),
        },
        
        // Thermal
        PhysicalConstant {
            id: "k_copper".into(),
            name: "Thermal conductivity of copper".into(),
            symbol: "k_{Cu}".into(),
            value: "401".into(),
            unit: "W/(m·K)".into(),
            uncertainty: Some("5".into()),
            source: "Engineering handbook".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"k_{Cu} = 401 \text{ W/(m·K)}".into(),
        },
        PhysicalConstant {
            id: "k_steel".into(),
            name: "Thermal conductivity of steel".into(),
            symbol: "k_{steel}".into(),
            value: "50".into(),
            unit: "W/(m·K)".into(),
            uncertainty: Some("5".into()),
            source: "Engineering handbook".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"k_{steel} \approx 50 \text{ W/(m·K)}".into(),
        },
        
        // Standard conditions
        PhysicalConstant {
            id: "p_atm".into(),
            name: "Standard atmospheric pressure".into(),
            symbol: "p_{atm}".into(),
            value: "101325".into(),
            unit: "Pa".into(),
            uncertainty: Some("exact".into()),
            source: "ISA".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"p_{atm} = 101325 \text{ Pa} = 1 \text{ atm}".into(),
        },
        PhysicalConstant {
            id: "g_standard".into(),
            name: "Standard gravity".into(),
            symbol: "g_0".into(),
            value: "9.80665".into(),
            unit: "m/s²".into(),
            uncertainty: Some("exact".into()),
            source: "SI definition".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"g_0 = 9.80665 \text{ m/s}^2".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // Solid mechanics
        PhysicalLaw {
            id: "hooke_law".into(),
            name: "Hooke's Law".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"\sigma = E \epsilon".into(),
            description: "Stress proportional to strain in elastic region".into(),
            variables: vec![
                Variable { symbol: "σ".into(), name: "Stress".into(), unit: Some("Pa".into()) },
                Variable { symbol: "E".into(), name: "Young's modulus".into(), unit: Some("Pa".into()) },
                Variable { symbol: "ε".into(), name: "Strain".into(), unit: None },
            ],
            source: "Hooke 1660".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "poisson_effect".into(),
            name: "Poisson's Effect".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"\epsilon_{lateral} = -\nu \epsilon_{axial}".into(),
            description: "Lateral strain from axial loading".into(),
            variables: vec![
                Variable { symbol: "ν".into(), name: "Poisson's ratio".into(), unit: None },
            ],
            source: "Poisson 1827".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "shear_modulus".into(),
            name: "Shear Modulus Relation".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"G = \frac{E}{2(1+\nu)}".into(),
            description: "Relates shear modulus to Young's modulus".into(),
            variables: vec![
                Variable { symbol: "G".into(), name: "Shear modulus".into(), unit: Some("Pa".into()) },
            ],
            source: "Elasticity theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "bending_moment".into(),
            name: "Euler-Bernoulli Beam Equation".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"M = EI\frac{d^2y}{dx^2}".into(),
            description: "Bending moment in beam".into(),
            variables: vec![
                Variable { symbol: "M".into(), name: "Bending moment".into(), unit: Some("N·m".into()) },
                Variable { symbol: "I".into(), name: "Second moment of area".into(), unit: Some("m⁴".into()) },
            ],
            source: "Euler 1744".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "bending_stress".into(),
            name: "Bending Stress".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"\sigma = \frac{My}{I}".into(),
            description: "Stress distribution in beam bending".into(),
            variables: vec![
                Variable { symbol: "σ".into(), name: "Bending stress".into(), unit: Some("Pa".into()) },
                Variable { symbol: "y".into(), name: "Distance from neutral axis".into(), unit: Some("m".into()) },
            ],
            source: "Beam theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "torsion_shaft".into(),
            name: "Torsion in Circular Shaft".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"\tau = \frac{Tr}{J}".into(),
            description: "Shear stress in twisted shaft".into(),
            variables: vec![
                Variable { symbol: "τ".into(), name: "Shear stress".into(), unit: Some("Pa".into()) },
                Variable { symbol: "T".into(), name: "Torque".into(), unit: Some("N·m".into()) },
                Variable { symbol: "J".into(), name: "Polar moment of inertia".into(), unit: Some("m⁴".into()) },
            ],
            source: "Mechanics of materials".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Fluid mechanics
        PhysicalLaw {
            id: "bernoulli".into(),
            name: "Bernoulli's Equation".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"P + \frac{1}{2}\rho v^2 + \rho gh = \text{const}".into(),
            description: "Energy conservation in ideal fluid".into(),
            variables: vec![
                Variable { symbol: "P".into(), name: "Pressure".into(), unit: Some("Pa".into()) },
                Variable { symbol: "v".into(), name: "Velocity".into(), unit: Some("m/s".into()) },
                Variable { symbol: "h".into(), name: "Height".into(), unit: Some("m".into()) },
            ],
            source: "Bernoulli 1738".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "continuity_fluid".into(),
            name: "Continuity Equation (Fluids)".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"A_1 v_1 = A_2 v_2".into(),
            description: "Mass conservation in incompressible flow".into(),
            variables: vec![
                Variable { symbol: "A".into(), name: "Cross-sectional area".into(), unit: Some("m²".into()) },
                Variable { symbol: "v".into(), name: "Flow velocity".into(), unit: Some("m/s".into()) },
            ],
            source: "Fluid mechanics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "reynolds_number".into(),
            name: "Reynolds Number".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"Re = \frac{\rho v L}{\mu} = \frac{v L}{\nu}".into(),
            description: "Ratio of inertial to viscous forces".into(),
            variables: vec![
                Variable { symbol: "Re".into(), name: "Reynolds number".into(), unit: None },
                Variable { symbol: "L".into(), name: "Characteristic length".into(), unit: Some("m".into()) },
                Variable { symbol: "ν".into(), name: "Kinematic viscosity".into(), unit: Some("m²/s".into()) },
            ],
            source: "Reynolds 1883".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "hagen_poiseuille".into(),
            name: "Hagen-Poiseuille Equation".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"Q = \frac{\pi r^4 \Delta P}{8 \mu L}".into(),
            description: "Laminar flow in pipe".into(),
            variables: vec![
                Variable { symbol: "Q".into(), name: "Volume flow rate".into(), unit: Some("m³/s".into()) },
                Variable { symbol: "r".into(), name: "Pipe radius".into(), unit: Some("m".into()) },
                Variable { symbol: "ΔP".into(), name: "Pressure drop".into(), unit: Some("Pa".into()) },
            ],
            source: "Hagen 1839, Poiseuille 1840".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "darcy_weisbach".into(),
            name: "Darcy-Weisbach Equation".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"h_f = f \frac{L}{D} \frac{v^2}{2g}".into(),
            description: "Head loss in pipe flow".into(),
            variables: vec![
                Variable { symbol: "h_f".into(), name: "Head loss".into(), unit: Some("m".into()) },
                Variable { symbol: "f".into(), name: "Friction factor".into(), unit: None },
            ],
            source: "Darcy 1857".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "navier_stokes".into(),
            name: "Navier-Stokes Equations".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"\rho\left(\frac{\partial \mathbf{v}}{\partial t} + \mathbf{v}\cdot\nabla\mathbf{v}\right) = -\nabla P + \mu\nabla^2\mathbf{v} + \mathbf{f}".into(),
            description: "Momentum equation for viscous fluid".into(),
            variables: vec![
                Variable { symbol: "v".into(), name: "Velocity field".into(), unit: Some("m/s".into()) },
                Variable { symbol: "P".into(), name: "Pressure".into(), unit: Some("Pa".into()) },
            ],
            source: "Navier 1822, Stokes 1845".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "drag_force".into(),
            name: "Drag Force".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"F_D = \frac{1}{2}C_D \rho A v^2".into(),
            description: "Aerodynamic/hydrodynamic drag".into(),
            variables: vec![
                Variable { symbol: "F_D".into(), name: "Drag force".into(), unit: Some("N".into()) },
                Variable { symbol: "C_D".into(), name: "Drag coefficient".into(), unit: None },
                Variable { symbol: "A".into(), name: "Reference area".into(), unit: Some("m²".into()) },
            ],
            source: "Fluid dynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "lift_force".into(),
            name: "Lift Force".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"F_L = \frac{1}{2}C_L \rho A v^2".into(),
            description: "Aerodynamic lift".into(),
            variables: vec![
                Variable { symbol: "F_L".into(), name: "Lift force".into(), unit: Some("N".into()) },
                Variable { symbol: "C_L".into(), name: "Lift coefficient".into(), unit: None },
            ],
            source: "Aerodynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Electrical
        PhysicalLaw {
            id: "kirchhoff_voltage".into(),
            name: "Kirchhoff's Voltage Law".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"\sum_{loop} V_i = 0".into(),
            description: "Sum of voltages around loop is zero".into(),
            variables: vec![
                Variable { symbol: "V".into(), name: "Voltage".into(), unit: Some("V".into()) },
            ],
            source: "Kirchhoff 1845".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "kirchhoff_current".into(),
            name: "Kirchhoff's Current Law".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"\sum_{node} I_i = 0".into(),
            description: "Sum of currents at node is zero".into(),
            variables: vec![
                Variable { symbol: "I".into(), name: "Current".into(), unit: Some("A".into()) },
            ],
            source: "Kirchhoff 1845".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "capacitor_current".into(),
            name: "Capacitor Current-Voltage".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"i = C\frac{dv}{dt}".into(),
            description: "Current through capacitor".into(),
            variables: vec![
                Variable { symbol: "C".into(), name: "Capacitance".into(), unit: Some("F".into()) },
            ],
            source: "Circuit theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "inductor_voltage".into(),
            name: "Inductor Voltage-Current".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"v = L\frac{di}{dt}".into(),
            description: "Voltage across inductor".into(),
            variables: vec![
                Variable { symbol: "L".into(), name: "Inductance".into(), unit: Some("H".into()) },
            ],
            source: "Circuit theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "rc_time_constant".into(),
            name: "RC Time Constant".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"\tau = RC".into(),
            description: "Characteristic time of RC circuit".into(),
            variables: vec![
                Variable { symbol: "τ".into(), name: "Time constant".into(), unit: Some("s".into()) },
            ],
            source: "Circuit theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "resonant_frequency".into(),
            name: "LC Resonant Frequency".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"f_0 = \frac{1}{2\pi\sqrt{LC}}".into(),
            description: "Resonance frequency of LC circuit".into(),
            variables: vec![
                Variable { symbol: "f_0".into(), name: "Resonant frequency".into(), unit: Some("Hz".into()) },
            ],
            source: "Circuit theory".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Heat transfer
        PhysicalLaw {
            id: "fourier_heat".into(),
            name: "Fourier's Law of Heat Conduction".into(),
            domain: PhysicsDomain::Thermodynamics,
            equation_tex: r"q = -k\frac{dT}{dx}".into(),
            description: "Heat flux proportional to temperature gradient".into(),
            variables: vec![
                Variable { symbol: "q".into(), name: "Heat flux".into(), unit: Some("W/m²".into()) },
                Variable { symbol: "k".into(), name: "Thermal conductivity".into(), unit: Some("W/(m·K)".into()) },
            ],
            source: "Fourier 1822".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "newton_cooling".into(),
            name: "Newton's Law of Cooling".into(),
            domain: PhysicsDomain::Thermodynamics,
            equation_tex: r"q = h(T_s - T_\infty)".into(),
            description: "Convective heat transfer".into(),
            variables: vec![
                Variable { symbol: "h".into(), name: "Heat transfer coefficient".into(), unit: Some("W/(m²·K)".into()) },
            ],
            source: "Newton".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Control systems
        PhysicalLaw {
            id: "transfer_function".into(),
            name: "Transfer Function".into(),
            domain: PhysicsDomain::Other("Control Systems".into()),
            equation_tex: r"G(s) = \frac{Y(s)}{U(s)}".into(),
            description: "Ratio of output to input in Laplace domain".into(),
            variables: vec![
                Variable { symbol: "G(s)".into(), name: "Transfer function".into(), unit: None },
                Variable { symbol: "s".into(), name: "Complex frequency".into(), unit: Some("rad/s".into()) },
            ],
            source: "Control theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "pid_controller".into(),
            name: "PID Controller".into(),
            domain: PhysicsDomain::Other("Control Systems".into()),
            equation_tex: r"u(t) = K_p e(t) + K_i \int e(\tau)d\tau + K_d \frac{de}{dt}".into(),
            description: "Proportional-Integral-Derivative control".into(),
            variables: vec![
                Variable { symbol: "K_p".into(), name: "Proportional gain".into(), unit: None },
                Variable { symbol: "K_i".into(), name: "Integral gain".into(), unit: None },
                Variable { symbol: "K_d".into(), name: "Derivative gain".into(), unit: None },
            ],
            source: "Control theory".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        // Structural
        MathTheorem {
            id: "von_mises".into(),
            name: "Von Mises Yield Criterion".into(),
            category: TheoremCategory::Other("Mechanics".into()),
            statement_tex: r"\sigma_{vm} = \sqrt{\frac{1}{2}[(\sigma_1-\sigma_2)^2+(\sigma_2-\sigma_3)^2+(\sigma_3-\sigma_1)^2]} \leq \sigma_y".into(),
            proof_tex: Some("Yield occurs when distortion energy exceeds limit".into()),
            references: vec![],
            source: "von Mises 1913".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "euler_buckling".into(),
            name: "Euler Buckling Formula".into(),
            category: TheoremCategory::Other("Mechanics".into()),
            statement_tex: r"P_{cr} = \frac{\pi^2 EI}{(KL)^2}".into(),
            proof_tex: Some("Critical load for column buckling; K depends on end conditions".into()),
            references: vec![],
            source: "Euler 1744".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "mohr_circle".into(),
            name: "Mohr's Circle".into(),
            category: TheoremCategory::Other("Mechanics".into()),
            statement_tex: r"\left(\sigma - \frac{\sigma_x+\sigma_y}{2}\right)^2 + \tau^2 = \left(\frac{\sigma_x-\sigma_y}{2}\right)^2 + \tau_{xy}^2".into(),
            proof_tex: Some("Graphical method for stress transformation".into()),
            references: vec![],
            source: "Mohr 1882".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "castigliano".into(),
            name: "Castigliano's Theorem".into(),
            category: TheoremCategory::Other("Mechanics".into()),
            statement_tex: r"\delta_i = \frac{\partial U}{\partial F_i}".into(),
            proof_tex: Some("Displacement from strain energy derivative".into()),
            references: vec![],
            source: "Castigliano 1879".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Fluid
        MathTheorem {
            id: "reynolds_transition".into(),
            name: "Reynolds Number Transition".into(),
            category: TheoremCategory::Other("Fluid Mechanics".into()),
            statement_tex: r"Re < 2300: \text{laminar}, \quad Re > 4000: \text{turbulent}".into(),
            proof_tex: Some("For pipe flow; transition region between".into()),
            references: vec![],
            source: "Reynolds 1883".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "stokes_drag".into(),
            name: "Stokes Drag Law".into(),
            category: TheoremCategory::Other("Fluid Mechanics".into()),
            statement_tex: r"F_D = 6\pi\mu r v \quad (Re \ll 1)".into(),
            proof_tex: Some("Drag on sphere in creeping flow".into()),
            references: vec![],
            source: "Stokes 1851".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "mach_number".into(),
            name: "Mach Number Regimes".into(),
            category: TheoremCategory::Other("Aerodynamics".into()),
            statement_tex: r"Ma < 0.3: \text{incomp.}, 0.8-1.2: \text{transonic}, Ma > 1: \text{supersonic}".into(),
            proof_tex: Some("Compressibility effects at high Mach".into()),
            references: vec![],
            source: "Aerodynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Electrical
        MathTheorem {
            id: "thevenin".into(),
            name: "Thévenin's Theorem".into(),
            category: TheoremCategory::Other("Circuit Analysis".into()),
            statement_tex: r"\text{Any linear circuit} \equiv V_{th} \text{ in series with } R_{th}".into(),
            proof_tex: Some("Equivalent voltage source with series resistance".into()),
            references: vec![],
            source: "Thévenin 1883".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "norton".into(),
            name: "Norton's Theorem".into(),
            category: TheoremCategory::Other("Circuit Analysis".into()),
            statement_tex: r"\text{Any linear circuit} \equiv I_N \text{ in parallel with } R_N".into(),
            proof_tex: Some("Equivalent current source with parallel resistance".into()),
            references: vec!["thevenin".into()],
            source: "Norton 1926".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "superposition".into(),
            name: "Superposition Principle (Circuits)".into(),
            category: TheoremCategory::Other("Circuit Analysis".into()),
            statement_tex: r"V = \sum_i V_i \text{ (each source acting alone)}".into(),
            proof_tex: Some("For linear circuits only".into()),
            references: vec![],
            source: "Circuit theory".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "maximum_power".into(),
            name: "Maximum Power Transfer".into(),
            category: TheoremCategory::Other("Circuit Analysis".into()),
            statement_tex: r"R_L = R_{th} \text{ for max power to load}".into(),
            proof_tex: Some("Load resistance equals source resistance".into()),
            references: vec!["thevenin".into()],
            source: "Circuit theory".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Control
        MathTheorem {
            id: "nyquist_stability".into(),
            name: "Nyquist Stability Criterion".into(),
            category: TheoremCategory::Other("Control Systems".into()),
            statement_tex: r"Z = N + P \text{ (encirclements of -1)}".into(),
            proof_tex: Some("Closed-loop stability from open-loop plot".into()),
            references: vec![],
            source: "Nyquist 1932".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "bode_gain_margin".into(),
            name: "Gain and Phase Margins".into(),
            category: TheoremCategory::Other("Control Systems".into()),
            statement_tex: r"GM = -20\log|G(j\omega_\pi)|, \quad PM = 180° + \angle G(j\omega_c)".into(),
            proof_tex: Some("Stability margins from Bode plot".into()),
            references: vec![],
            source: "Bode 1940".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "routh_hurwitz".into(),
            name: "Routh-Hurwitz Stability".into(),
            category: TheoremCategory::Other("Control Systems".into()),
            statement_tex: r"\text{All first-column elements positive} \Rightarrow \text{stable}".into(),
            proof_tex: Some("Algebraic stability test for polynomials".into()),
            references: vec![],
            source: "Routh 1877, Hurwitz 1895".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "shannon_sampling".into(),
            name: "Nyquist-Shannon Sampling Theorem".into(),
            category: TheoremCategory::Other("Signal Processing".into()),
            statement_tex: r"f_s > 2 f_{max}".into(),
            proof_tex: Some("Sampling rate must exceed twice max frequency".into()),
            references: vec![],
            source: "Nyquist 1928, Shannon 1949".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
