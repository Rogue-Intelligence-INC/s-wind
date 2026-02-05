//! Electronics Engineering Module for Blondle
//!
//! Covers:
//! - Circuit analysis fundamentals
//! - PCB design rules (IPC standards)
//! - Component specifications
//! - Signal integrity
//! - EMC/EMI standards

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all electronics engineering knowledge
pub fn load(kb: &mut KnowledgeBase) {
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::Electronics] Loaded {} constants, {} laws, {} theorems",
             CONSTANT_COUNT, LAW_COUNT, THEOREM_COUNT);
}

pub fn count() -> (usize, usize, usize) {
    (CONSTANT_COUNT, LAW_COUNT, THEOREM_COUNT)
}

const CONSTANT_COUNT: usize = 18;
const LAW_COUNT: usize = 15;
const THEOREM_COUNT: usize = 10;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Fundamental electrical
        PhysicalConstant {
            id: "vacuum_permittivity".into(),
            name: "Vacuum permittivity".into(),
            symbol: r"\varepsilon_0".into(),
            value: "8.854e-12".into(),
            unit: "F/m".into(),
            uncertainty: Some("exact".into()),
            source: "NIST CODATA".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\varepsilon_0 = 8.854 \times 10^{-12} \text{ F/m}".into(),
        },
        PhysicalConstant {
            id: "vacuum_permeability".into(),
            name: "Vacuum permeability".into(),
            symbol: r"\mu_0".into(),
            value: "1.257e-6".into(),
            unit: "H/m".into(),
            uncertainty: Some("exact".into()),
            source: "NIST CODATA".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\mu_0 = 4\pi \times 10^{-7} \text{ H/m}".into(),
        },
        PhysicalConstant {
            id: "cu_resistivity".into(),
            name: "Copper resistivity (20°C)".into(),
            symbol: r"\rho_{Cu}".into(),
            value: "1.68e-8".into(),
            unit: "Ω·m".into(),
            uncertainty: Some("0.01e-8".into()),
            source: "Materials data".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\rho_{Cu} = 1.68 \times 10^{-8} \text{ Ω·m}".into(),
        },
        PhysicalConstant {
            id: "cu_temp_coeff".into(),
            name: "Copper temperature coefficient".into(),
            symbol: r"\alpha_{Cu}".into(),
            value: "0.00393".into(),
            unit: "1/°C".into(),
            uncertainty: Some("0.0001".into()),
            source: "Materials data".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\alpha_{Cu} = 0.00393 \text{ /°C}".into(),
        },
        
        // PCB design (IPC standards)
        PhysicalConstant {
            id: "fr4_dielectric".into(),
            name: "FR-4 dielectric constant".into(),
            symbol: r"\varepsilon_r".into(),
            value: "4.5".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.3".into()),
            source: "IPC".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\varepsilon_r \approx 4.2-4.8 \text{ (FR-4, 1 MHz)}".into(),
        },
        PhysicalConstant {
            id: "fr4_loss_tangent".into(),
            name: "FR-4 loss tangent".into(),
            symbol: r"\tan\delta".into(),
            value: "0.02".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.005".into()),
            source: "IPC".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\tan\delta \approx 0.015-0.025 \text{ (FR-4)}".into(),
        },
        PhysicalConstant {
            id: "pcb_trace_1oz".into(),
            name: "1 oz copper thickness".into(),
            symbol: "t_{1oz}".into(),
            value: "35".into(),
            unit: "μm".into(),
            uncertainty: None,
            source: "IPC".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"t_{1oz} = 35 \text{ μm} = 1.4 \text{ mil}".into(),
        },
        PhysicalConstant {
            id: "ipc_class2_clearance".into(),
            name: "IPC Class 2 min clearance".into(),
            symbol: "d_{min}".into(),
            value: "0.13".into(),
            unit: "mm".into(),
            uncertainty: None,
            source: "IPC-2221".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"d_{min} = 0.13 \text{ mm (5 mil) Class 2}".into(),
        },
        PhysicalConstant {
            id: "ipc_class3_clearance".into(),
            name: "IPC Class 3 min clearance".into(),
            symbol: "d_{min,3}".into(),
            value: "0.10".into(),
            unit: "mm".into(),
            uncertainty: None,
            source: "IPC-2221".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"d_{min} = 0.10 \text{ mm (4 mil) Class 3}".into(),
        },
        
        // Component parameters
        PhysicalConstant {
            id: "resistor_e96".into(),
            name: "E96 resistor series (1%)".into(),
            symbol: "E96".into(),
            value: "96 values/decade".into(),
            unit: "dimensionless".into(),
            uncertainty: None,
            source: "IEC 60063".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"E96: 96 \text{ values per decade, 1\% tolerance}".into(),
        },
        PhysicalConstant {
            id: "mlcc_x7r".into(),
            name: "X7R capacitor temp range".into(),
            symbol: "X7R".into(),
            value: "-55 to +125".into(),
            unit: "°C".into(),
            uncertainty: None,
            source: "EIA-198".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"X7R: -55°\text{C to }+125°\text{C, }±15\%".into(),
        },
        PhysicalConstant {
            id: "mlcc_c0g".into(),
            name: "C0G/NP0 capacitor tolerance".into(),
            symbol: "C0G".into(),
            value: "±30 ppm/°C".into(),
            unit: "ppm/°C".into(),
            uncertainty: None,
            source: "EIA-198".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"C0G/NP0: ±30 \text{ ppm/°C (stable)}".into(),
        },
        
        // Signal integrity
        PhysicalConstant {
            id: "rise_time_tr".into(),
            name: "Typical rise time (fast digital)".into(),
            symbol: "t_r".into(),
            value: "1".into(),
            unit: "ns".into(),
            uncertainty: Some("0.5".into()),
            source: "Signal integrity".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"t_r \approx 0.3-2 \text{ ns (modern digital)}".into(),
        },
        PhysicalConstant {
            id: "pcb_propagation".into(),
            name: "PCB signal propagation delay".into(),
            symbol: "t_{pd}".into(),
            value: "6.7".into(),
            unit: "ns/m".into(),
            uncertainty: Some("0.5".into()),
            source: "Signal integrity".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"t_{pd} \approx 6-7 \text{ ns/m (microstrip, FR-4)}".into(),
        },
        
        // EMC
        PhysicalConstant {
            id: "fcc_class_b_limit".into(),
            name: "FCC Class B limit (30-88 MHz)".into(),
            symbol: "E_{limit}".into(),
            value: "100".into(),
            unit: "μV/m".into(),
            uncertainty: None,
            source: "FCC Part 15".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"E < 100 \text{ μV/m at 3m (30-88 MHz)}".into(),
        },
        PhysicalConstant {
            id: "cispr_class_b".into(),
            name: "CISPR 32 Class B limit (30 MHz)".into(),
            symbol: "E_{CISPR}".into(),
            value: "30".into(),
            unit: "dBμV/m".into(),
            uncertainty: None,
            source: "CISPR 32".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"E < 30 \text{ dBμV/m at 10m (30-230 MHz)}".into(),
        },
        
        // Thermal
        PhysicalConstant {
            id: "theta_ja_smd".into(),
            name: "Typical θJA for SMD package".into(),
            symbol: r"\theta_{JA}".into(),
            value: "50".into(),
            unit: "°C/W".into(),
            uncertainty: Some("30".into()),
            source: "Package thermal".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\theta_{JA} \approx 30-100 \text{ °C/W (SMD)}".into(),
        },
        PhysicalConstant {
            id: "junction_temp_max".into(),
            name: "Typical max junction temperature".into(),
            symbol: "T_{J,max}".into(),
            value: "125".into(),
            unit: "°C".into(),
            uncertainty: None,
            source: "Component specs".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"T_{J,max} = 125°\text{C (commercial)}".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // Circuit fundamentals
        PhysicalLaw {
            id: "ohm_law".into(),
            name: "Ohm's Law".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"V = IR".into(),
            description: "Voltage equals current times resistance".into(),
            variables: vec![
                Variable { symbol: "V".into(), name: "Voltage".into(), unit: Some("V".into()) },
                Variable { symbol: "I".into(), name: "Current".into(), unit: Some("A".into()) },
                Variable { symbol: "R".into(), name: "Resistance".into(), unit: Some("Ω".into()) },
            ],
            source: "Ohm 1827".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "kvl".into(),
            name: "Kirchhoff's Voltage Law".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"\sum_{k=1}^n V_k = 0".into(),
            description: "Sum of voltages around a loop is zero".into(),
            variables: vec![
                Variable { symbol: "V_k".into(), name: "Voltage across element k".into(), unit: Some("V".into()) },
            ],
            source: "Kirchhoff 1845".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "kcl".into(),
            name: "Kirchhoff's Current Law".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"\sum_{k=1}^n I_k = 0".into(),
            description: "Sum of currents at a node is zero".into(),
            variables: vec![
                Variable { symbol: "I_k".into(), name: "Current in branch k".into(), unit: Some("A".into()) },
            ],
            source: "Kirchhoff 1845".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Transmission lines
        PhysicalLaw {
            id: "microstrip_z0".into(),
            name: "Microstrip Impedance".into(),
            domain: PhysicsDomain::Other("Signal Integrity".into()),
            equation_tex: r"Z_0 \approx \frac{87}{\sqrt{\varepsilon_r + 1.41}}\ln\frac{5.98h}{0.8w + t}".into(),
            description: "Characteristic impedance of microstrip".into(),
            variables: vec![
                Variable { symbol: "Z_0".into(), name: "Characteristic impedance".into(), unit: Some("Ω".into()) },
                Variable { symbol: "h".into(), name: "Dielectric height".into(), unit: Some("mm".into()) },
                Variable { symbol: "w".into(), name: "Trace width".into(), unit: Some("mm".into()) },
                Variable { symbol: "t".into(), name: "Trace thickness".into(), unit: Some("mm".into()) },
            ],
            source: "IPC-2141".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "stripline_z0".into(),
            name: "Stripline Impedance".into(),
            domain: PhysicsDomain::Other("Signal Integrity".into()),
            equation_tex: r"Z_0 \approx \frac{60}{\sqrt{\varepsilon_r}}\ln\frac{4h}{0.67\pi(0.8w + t)}".into(),
            description: "Characteristic impedance of stripline".into(),
            variables: vec![
                Variable { symbol: "h".into(), name: "Distance to ground planes".into(), unit: Some("mm".into()) },
            ],
            source: "IPC-2141".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "reflection_coeff".into(),
            name: "Reflection Coefficient".into(),
            domain: PhysicsDomain::Other("Signal Integrity".into()),
            equation_tex: r"\Gamma = \frac{Z_L - Z_0}{Z_L + Z_0}".into(),
            description: "Signal reflection at impedance discontinuity".into(),
            variables: vec![
                Variable { symbol: "Γ".into(), name: "Reflection coefficient".into(), unit: None },
                Variable { symbol: "Z_L".into(), name: "Load impedance".into(), unit: Some("Ω".into()) },
            ],
            source: "Transmission line theory".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // PCB design
        PhysicalLaw {
            id: "trace_current_capacity".into(),
            name: "PCB Trace Current Capacity (IPC)".into(),
            domain: PhysicsDomain::Other("PCB Design".into()),
            equation_tex: r"I = k \cdot \Delta T^b \cdot A^c".into(),
            description: "Current capacity from temperature rise".into(),
            variables: vec![
                Variable { symbol: "I".into(), name: "Current".into(), unit: Some("A".into()) },
                Variable { symbol: "ΔT".into(), name: "Temperature rise".into(), unit: Some("°C".into()) },
                Variable { symbol: "A".into(), name: "Cross-section area".into(), unit: Some("mil²".into()) },
            ],
            source: "IPC-2152".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "via_current".into(),
            name: "Via Current Capacity".into(),
            domain: PhysicsDomain::Other("PCB Design".into()),
            equation_tex: r"I_{via} \approx 1.5 \cdot d_{via} \cdot t_{plating}".into(),
            description: "Approximate via current (A, mm)".into(),
            variables: vec![
                Variable { symbol: "d_via".into(), name: "Via diameter".into(), unit: Some("mm".into()) },
                Variable { symbol: "t_plating".into(), name: "Plating thickness".into(), unit: Some("μm".into()) },
            ],
            source: "PCB design practice".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Power
        PhysicalLaw {
            id: "power_dissipation".into(),
            name: "Power Dissipation".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"P = I^2 R = \frac{V^2}{R} = IV".into(),
            description: "Electrical power".into(),
            variables: vec![
                Variable { symbol: "P".into(), name: "Power".into(), unit: Some("W".into()) },
            ],
            source: "Circuit theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "thermal_resistance".into(),
            name: "Thermal Resistance Equation".into(),
            domain: PhysicsDomain::Thermodynamics,
            equation_tex: r"T_J = T_A + P_D \cdot \theta_{JA}".into(),
            description: "Junction temperature from power".into(),
            variables: vec![
                Variable { symbol: "T_J".into(), name: "Junction temperature".into(), unit: Some("°C".into()) },
                Variable { symbol: "T_A".into(), name: "Ambient temperature".into(), unit: Some("°C".into()) },
                Variable { symbol: "θ_JA".into(), name: "Junction-ambient thermal resistance".into(), unit: Some("°C/W".into()) },
            ],
            source: "Thermal design".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // RC time constants
        PhysicalLaw {
            id: "rc_time_constant".into(),
            name: "RC Time Constant".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"\tau = RC".into(),
            description: "Time constant of RC circuit".into(),
            variables: vec![
                Variable { symbol: "τ".into(), name: "Time constant".into(), unit: Some("s".into()) },
                Variable { symbol: "C".into(), name: "Capacitance".into(), unit: Some("F".into()) },
            ],
            source: "Circuit theory".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "lc_resonance".into(),
            name: "LC Resonant Frequency".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"f_0 = \frac{1}{2\pi\sqrt{LC}}".into(),
            description: "Resonant frequency of LC circuit".into(),
            variables: vec![
                Variable { symbol: "f_0".into(), name: "Resonant frequency".into(), unit: Some("Hz".into()) },
                Variable { symbol: "L".into(), name: "Inductance".into(), unit: Some("H".into()) },
            ],
            source: "Circuit theory".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Signal integrity
        PhysicalLaw {
            id: "critical_length".into(),
            name: "Critical Trace Length".into(),
            domain: PhysicsDomain::Other("Signal Integrity".into()),
            equation_tex: r"L_{crit} = \frac{t_r \cdot v_p}{6}".into(),
            description: "Length above which transmission line effects matter".into(),
            variables: vec![
                Variable { symbol: "L_crit".into(), name: "Critical length".into(), unit: Some("m".into()) },
                Variable { symbol: "t_r".into(), name: "Rise time".into(), unit: Some("s".into()) },
                Variable { symbol: "v_p".into(), name: "Propagation velocity".into(), unit: Some("m/s".into()) },
            ],
            source: "Signal integrity".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "bandwidth_rise_time".into(),
            name: "Bandwidth-Rise Time".into(),
            domain: PhysicsDomain::Other("Signal Integrity".into()),
            equation_tex: r"BW \approx \frac{0.35}{t_r}".into(),
            description: "3dB bandwidth from 10-90% rise time".into(),
            variables: vec![
                Variable { symbol: "BW".into(), name: "Bandwidth".into(), unit: Some("Hz".into()) },
            ],
            source: "Signal integrity".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "skin_depth".into(),
            name: "Skin Depth".into(),
            domain: PhysicsDomain::Electromagnetism,
            equation_tex: r"\delta = \sqrt{\frac{2\rho}{\omega\mu}}".into(),
            description: "Depth at which current density drops to 1/e".into(),
            variables: vec![
                Variable { symbol: "δ".into(), name: "Skin depth".into(), unit: Some("m".into()) },
                Variable { symbol: "ω".into(), name: "Angular frequency".into(), unit: Some("rad/s".into()) },
            ],
            source: "Electromagnetics".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        MathTheorem {
            id: "thevenin".into(),
            name: "Thévenin's Theorem".into(),
            category: TheoremCategory::Other("Circuit Theory".into()),
            statement_tex: r"\text{Any linear network = voltage source } V_{th} \text{ in series with } R_{th}".into(),
            proof_tex: Some("Equivalent circuit at any two terminals".into()),
            references: vec![],
            source: "Thévenin 1883".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "norton".into(),
            name: "Norton's Theorem".into(),
            category: TheoremCategory::Other("Circuit Theory".into()),
            statement_tex: r"\text{Any linear network = current source } I_N \text{ in parallel with } R_N".into(),
            proof_tex: Some("Dual of Thévenin; IN = Vth/Rth".into()),
            references: vec!["thevenin".into()],
            source: "Norton 1926".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "superposition".into(),
            name: "Superposition Principle".into(),
            category: TheoremCategory::Other("Circuit Theory".into()),
            statement_tex: r"\text{Response to multiple sources = sum of individual responses}".into(),
            proof_tex: Some("Valid for linear circuits only".into()),
            references: vec![],
            source: "Circuit theory".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "maximum_power_transfer".into(),
            name: "Maximum Power Transfer".into(),
            category: TheoremCategory::Other("Circuit Theory".into()),
            statement_tex: r"P_{max} \text{ when } Z_L = Z_S^* \text{ (conjugate match)}".into(),
            proof_tex: Some("For resistive: RL = RS gives Pmax = Vth²/4Rs".into()),
            references: vec![],
            source: "Circuit theory".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "nyquist_sampling".into(),
            name: "Nyquist-Shannon Sampling Theorem".into(),
            category: TheoremCategory::Other("Signal Processing".into()),
            statement_tex: r"f_s > 2f_{max}".into(),
            proof_tex: Some("Sampling rate must exceed twice the highest frequency".into()),
            references: vec![],
            source: "Nyquist 1928, Shannon 1949".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "3db_rule".into(),
            name: "3dB Rule for Parallel Resistors".into(),
            category: TheoremCategory::Other("Circuit Theory".into()),
            statement_tex: r"R_{parallel} = R/2 \text{ for two equal resistors}".into(),
            proof_tex: Some("RC cutoff frequency doubles with parallel C".into()),
            references: vec![],
            source: "Circuit theory".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "decoupling_rule".into(),
            name: "Decoupling Capacitor Placement".into(),
            category: TheoremCategory::Other("PCB Design".into()),
            statement_tex: r"d_{cap} < \frac{\lambda}{10} = \frac{v_p}{10 f_{knee}}".into(),
            proof_tex: Some("Capacitor must be within 1/10 wavelength of IC power pin".into()),
            references: vec![],
            source: "EMC design".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "loop_area".into(),
            name: "EMI Loop Area Rule".into(),
            category: TheoremCategory::Other("EMC".into()),
            statement_tex: r"E \propto f^2 \cdot I \cdot A".into(),
            proof_tex: Some("Radiated emissions proportional to loop area".into()),
            references: vec![],
            source: "EMC design".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "return_path".into(),
            name: "Return Path Principle".into(),
            category: TheoremCategory::Other("Signal Integrity".into()),
            statement_tex: r"\text{High-frequency current follows path of minimum inductance}".into(),
            proof_tex: Some("Return current flows directly under signal trace".into()),
            references: vec![],
            source: "Signal integrity".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "bypass_impedance".into(),
            name: "Bypass Capacitor Impedance".into(),
            category: TheoremCategory::Other("Power Integrity".into()),
            statement_tex: r"Z_{cap} = ESR + j(\omega L_{ESL} - \frac{1}{\omega C})".into(),
            proof_tex: Some("Real capacitor has parasitic ESR and ESL".into()),
            references: vec![],
            source: "Power integrity".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
