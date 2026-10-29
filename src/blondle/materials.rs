//! Materials Science Module for Blondle
//!
//! Covers:
//! - Crystallography (lattice structures, Miller indices)
//! - Material properties (mechanical, thermal, electrical)
//! - Phase diagrams
//! - Defects and dislocations

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all materials science knowledge
pub fn load(kb: &mut KnowledgeBase) {
    let before = (kb.physical_constants.len(), kb.physical_laws.len(), kb.theorems.len());
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::Materials] Loaded {} constants, {} laws, {} theorems",
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

const CONSTANT_COUNT: usize = 12;
const LAW_COUNT: usize = 10;
const THEOREM_COUNT: usize = 8;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Lattice constants
        PhysicalConstant {
            id: "a_Si".into(),
            name: "Silicon lattice constant".into(),
            symbol: "a_{Si}".into(),
            value: "5.431".into(),
            unit: "Å".into(),
            uncertainty: Some("0.001".into()),
            source: "Crystallography".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"a_{Si} = 5.431 \text{ Å (diamond cubic)}".into(),
        },
        PhysicalConstant {
            id: "a_Fe".into(),
            name: "Iron (BCC) lattice constant".into(),
            symbol: "a_{Fe}".into(),
            value: "2.87".into(),
            unit: "Å".into(),
            uncertainty: Some("0.01".into()),
            source: "Crystallography".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"a_{Fe} = 2.87 \text{ Å (BCC)}".into(),
        },
        PhysicalConstant {
            id: "a_Cu".into(),
            name: "Copper (FCC) lattice constant".into(),
            symbol: "a_{Cu}".into(),
            value: "3.61".into(),
            unit: "Å".into(),
            uncertainty: Some("0.01".into()),
            source: "Crystallography".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"a_{Cu} = 3.61 \text{ Å (FCC)}".into(),
        },
        PhysicalConstant {
            id: "a_Al".into(),
            name: "Aluminum (FCC) lattice constant".into(),
            symbol: "a_{Al}".into(),
            value: "4.05".into(),
            unit: "Å".into(),
            uncertainty: Some("0.01".into()),
            source: "Crystallography".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"a_{Al} = 4.05 \text{ Å (FCC)}".into(),
        },
        
        // Band gaps
        PhysicalConstant {
            id: "Eg_Si".into(),
            name: "Silicon band gap".into(),
            symbol: "E_g(Si)".into(),
            value: "1.12".into(),
            unit: "eV".into(),
            uncertainty: Some("0.01".into()),
            source: "Semiconductor physics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"E_g(Si) = 1.12 \text{ eV at 300K}".into(),
        },
        PhysicalConstant {
            id: "Eg_GaAs".into(),
            name: "GaAs band gap".into(),
            symbol: "E_g(GaAs)".into(),
            value: "1.42".into(),
            unit: "eV".into(),
            uncertainty: Some("0.01".into()),
            source: "Semiconductor physics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"E_g(GaAs) = 1.42 \text{ eV (direct)}".into(),
        },
        PhysicalConstant {
            id: "Eg_Ge".into(),
            name: "Germanium band gap".into(),
            symbol: "E_g(Ge)".into(),
            value: "0.67".into(),
            unit: "eV".into(),
            uncertainty: Some("0.01".into()),
            source: "Semiconductor physics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"E_g(Ge) = 0.67 \text{ eV}".into(),
        },
        
        // Dislocation properties
        PhysicalConstant {
            id: "burgers_fcc".into(),
            name: "FCC Burgers vector magnitude".into(),
            symbol: "|b|_{FCC}".into(),
            value: "a/√2".into(),
            unit: "lattice units".into(),
            uncertainty: None,
            source: "Dislocation theory".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"|\mathbf{b}| = \frac{a}{\sqrt{2}} \text{ for FCC } \langle 110 \rangle".into(),
        },
        
        // Thermal expansion
        PhysicalConstant {
            id: "alpha_steel_thermal".into(),
            name: "Thermal expansion of steel".into(),
            symbol: r"\alpha_{steel}".into(),
            value: "12".into(),
            unit: "10⁻⁶/K".into(),
            uncertainty: Some("1".into()),
            source: "Materials handbook".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\alpha_{steel} \approx 12 \times 10^{-6} \text{ K}^{-1}".into(),
        },
        PhysicalConstant {
            id: "alpha_al_thermal".into(),
            name: "Thermal expansion of aluminum".into(),
            symbol: r"\alpha_{Al}".into(),
            value: "23".into(),
            unit: "10⁻⁶/K".into(),
            uncertainty: Some("1".into()),
            source: "Materials handbook".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\alpha_{Al} \approx 23 \times 10^{-6} \text{ K}^{-1}".into(),
        },
        
        // Specific heat
        PhysicalConstant {
            id: "Cp_steel".into(),
            name: "Specific heat of steel".into(),
            symbol: "C_p(steel)".into(),
            value: "500".into(),
            unit: "J/(kg·K)".into(),
            uncertainty: Some("20".into()),
            source: "Materials handbook".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"C_p(steel) \approx 500 \text{ J/(kg·K)}".into(),
        },
        PhysicalConstant {
            id: "Cp_water".into(),
            name: "Specific heat of water".into(),
            symbol: "C_p(H_2O)".into(),
            value: "4186".into(),
            unit: "J/(kg·K)".into(),
            uncertainty: Some("1".into()),
            source: "Standard".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"C_p(H_2O) = 4186 \text{ J/(kg·K)}".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // Crystallography
        PhysicalLaw {
            id: "bragg_law".into(),
            name: "Bragg's Law".into(),
            domain: PhysicsDomain::Other("Crystallography".into()),
            equation_tex: r"n\lambda = 2d\sin\theta".into(),
            description: "X-ray diffraction condition".into(),
            variables: vec![
                Variable { symbol: "λ".into(), name: "Wavelength".into(), unit: Some("Å".into()) },
                Variable { symbol: "d".into(), name: "Interplanar spacing".into(), unit: Some("Å".into()) },
                Variable { symbol: "θ".into(), name: "Bragg angle".into(), unit: Some("rad".into()) },
            ],
            source: "Bragg 1913".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "interplanar_spacing".into(),
            name: "Interplanar Spacing (Cubic)".into(),
            domain: PhysicsDomain::Other("Crystallography".into()),
            equation_tex: r"d_{hkl} = \frac{a}{\sqrt{h^2+k^2+l^2}}".into(),
            description: "Spacing between Miller planes in cubic crystal".into(),
            variables: vec![
                Variable { symbol: "a".into(), name: "Lattice constant".into(), unit: Some("Å".into()) },
                Variable { symbol: "h,k,l".into(), name: "Miller indices".into(), unit: None },
            ],
            source: "Crystallography".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Diffusion
        PhysicalLaw {
            id: "fick_first".into(),
            name: "Fick's First Law".into(),
            domain: PhysicsDomain::Other("Materials Science".into()),
            equation_tex: r"J = -D\frac{dC}{dx}".into(),
            description: "Diffusion flux proportional to concentration gradient".into(),
            variables: vec![
                Variable { symbol: "J".into(), name: "Diffusion flux".into(), unit: Some("mol/(m²·s)".into()) },
                Variable { symbol: "D".into(), name: "Diffusivity".into(), unit: Some("m²/s".into()) },
            ],
            source: "Fick 1855".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "fick_second".into(),
            name: "Fick's Second Law".into(),
            domain: PhysicsDomain::Other("Materials Science".into()),
            equation_tex: r"\frac{\partial C}{\partial t} = D\frac{\partial^2 C}{\partial x^2}".into(),
            description: "Time evolution of concentration".into(),
            variables: vec![
                Variable { symbol: "C".into(), name: "Concentration".into(), unit: Some("mol/m³".into()) },
            ],
            source: "Fick 1855".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "arrhenius_diffusion".into(),
            name: "Arrhenius Diffusion".into(),
            domain: PhysicsDomain::Other("Materials Science".into()),
            equation_tex: r"D = D_0 e^{-Q/(RT)}".into(),
            description: "Temperature dependence of diffusivity".into(),
            variables: vec![
                Variable { symbol: "D_0".into(), name: "Pre-exponential factor".into(), unit: Some("m²/s".into()) },
                Variable { symbol: "Q".into(), name: "Activation energy".into(), unit: Some("J/mol".into()) },
            ],
            source: "Arrhenius".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Mechanical
        PhysicalLaw {
            id: "hall_petch".into(),
            name: "Hall-Petch Equation".into(),
            domain: PhysicsDomain::Other("Materials Science".into()),
            equation_tex: r"\sigma_y = \sigma_0 + \frac{k}{\sqrt{d}}".into(),
            description: "Yield strength vs grain size".into(),
            variables: vec![
                Variable { symbol: "σ_y".into(), name: "Yield strength".into(), unit: Some("MPa".into()) },
                Variable { symbol: "d".into(), name: "Grain size".into(), unit: Some("m".into()) },
            ],
            source: "Hall 1951, Petch 1953".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "orowan_equation".into(),
            name: "Orowan Equation".into(),
            domain: PhysicsDomain::Other("Materials Science".into()),
            equation_tex: r"\dot{\gamma} = \rho b v".into(),
            description: "Plastic strain rate from dislocation motion".into(),
            variables: vec![
                Variable { symbol: "γ̇".into(), name: "Shear strain rate".into(), unit: Some("s⁻¹".into()) },
                Variable { symbol: "ρ".into(), name: "Dislocation density".into(), unit: Some("m⁻²".into()) },
                Variable { symbol: "b".into(), name: "Burgers vector".into(), unit: Some("m".into()) },
            ],
            source: "Orowan 1940".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Thermal
        PhysicalLaw {
            id: "thermal_expansion".into(),
            name: "Linear Thermal Expansion".into(),
            domain: PhysicsDomain::Thermodynamics,
            equation_tex: r"\Delta L = \alpha L_0 \Delta T".into(),
            description: "Length change with temperature".into(),
            variables: vec![
                Variable { symbol: "α".into(), name: "Thermal expansion coefficient".into(), unit: Some("K⁻¹".into()) },
            ],
            source: "Thermodynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Semiconductors
        PhysicalLaw {
            id: "carrier_concentration".into(),
            name: "Intrinsic Carrier Concentration".into(),
            domain: PhysicsDomain::Other("Semiconductors".into()),
            equation_tex: r"n_i = \sqrt{N_c N_v} e^{-E_g/(2k_BT)}".into(),
            description: "Intrinsic carrier density in semiconductor".into(),
            variables: vec![
                Variable { symbol: "n_i".into(), name: "Intrinsic carrier concentration".into(), unit: Some("cm⁻³".into()) },
                Variable { symbol: "E_g".into(), name: "Band gap".into(), unit: Some("eV".into()) },
            ],
            source: "Semiconductor physics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "drift_velocity".into(),
            name: "Carrier Drift Velocity".into(),
            domain: PhysicsDomain::Other("Semiconductors".into()),
            equation_tex: r"v_d = \mu E".into(),
            description: "Drift velocity in electric field".into(),
            variables: vec![
                Variable { symbol: "v_d".into(), name: "Drift velocity".into(), unit: Some("m/s".into()) },
                Variable { symbol: "μ".into(), name: "Mobility".into(), unit: Some("m²/(V·s)".into()) },
            ],
            source: "Semiconductor physics".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        MathTheorem {
            id: "bravais_lattices".into(),
            name: "14 Bravais Lattices".into(),
            category: TheoremCategory::Other("Crystallography".into()),
            statement_tex: r"\text{Only 14 distinct 3D lattice types exist}".into(),
            proof_tex: Some("7 crystal systems × centering possibilities".into()),
            references: vec![],
            source: "Bravais 1850".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "crystal_systems".into(),
            name: "7 Crystal Systems".into(),
            category: TheoremCategory::Other("Crystallography".into()),
            statement_tex: r"\text{Cubic, Tetragonal, Orthorhombic, Hexagonal, Trigonal, Monoclinic, Triclinic}".into(),
            proof_tex: Some("Classified by lattice parameter relationships".into()),
            references: vec![],
            source: "Crystallography".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "coordination_number".into(),
            name: "Coordination Numbers".into(),
            category: TheoremCategory::Other("Crystallography".into()),
            statement_tex: r"\text{SC: 6, BCC: 8, FCC/HCP: 12}".into(),
            proof_tex: Some("Number of nearest neighbors in each structure".into()),
            references: vec![],
            source: "Crystallography".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "atomic_packing_factor".into(),
            name: "Atomic Packing Factors".into(),
            category: TheoremCategory::Other("Crystallography".into()),
            statement_tex: r"\text{SC: } 0.52, \text{ BCC: } 0.68, \text{ FCC/HCP: } 0.74".into(),
            proof_tex: Some("Volume fraction occupied by atoms".into()),
            references: vec![],
            source: "Crystallography".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "slip_systems".into(),
            name: "Slip Systems".into(),
            category: TheoremCategory::Other("Materials Science".into()),
            statement_tex: r"\text{FCC: } 12, \text{ BCC: } 48, \text{ HCP: } 3 \text{ (basal)}".into(),
            proof_tex: Some("Number of independent slip systems affects ductility".into()),
            references: vec![],
            source: "Materials science".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "lever_rule".into(),
            name: "Lever Rule".into(),
            category: TheoremCategory::Other("Phase Diagrams".into()),
            statement_tex: r"W_\alpha = \frac{C_\beta - C_0}{C_\beta - C_\alpha}".into(),
            proof_tex: Some("Weight fraction of phases from phase diagram".into()),
            references: vec![],
            source: "Phase diagrams".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "gibbs_phase_rule".into(),
            name: "Gibbs Phase Rule".into(),
            category: TheoremCategory::Other("Phase Diagrams".into()),
            statement_tex: r"F = C - P + 2".into(),
            proof_tex: Some("Degrees of freedom = Components - Phases + 2".into()),
            references: vec![],
            source: "Gibbs".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "schmid_law".into(),
            name: "Schmid's Law".into(),
            category: TheoremCategory::Other("Materials Science".into()),
            statement_tex: r"\tau_R = \sigma \cos\phi \cos\lambda".into(),
            proof_tex: Some("Resolved shear stress on slip plane".into()),
            references: vec![],
            source: "Schmid 1924".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
