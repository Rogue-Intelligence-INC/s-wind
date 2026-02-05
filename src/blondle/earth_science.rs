//! Earth Science Module for Blondle
//!
//! Covers:
//! - Geology (plate tectonics, seismology)
//! - Geophysics (Earth structure, magnetic field)
//! - Meteorology (atmospheric physics)
//! - Oceanography basics

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all earth science knowledge
pub fn load(kb: &mut KnowledgeBase) {
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::EarthScience] Loaded {} constants, {} laws, {} theorems",
             CONSTANT_COUNT, LAW_COUNT, THEOREM_COUNT);
}

pub fn count() -> (usize, usize, usize) {
    (CONSTANT_COUNT, LAW_COUNT, THEOREM_COUNT)
}

const CONSTANT_COUNT: usize = 15;
const LAW_COUNT: usize = 10;
const THEOREM_COUNT: usize = 8;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Earth parameters
        PhysicalConstant {
            id: "R_Earth".into(),
            name: "Earth mean radius".into(),
            symbol: "R_E".into(),
            value: "6.371e6".into(),
            unit: "m".into(),
            uncertainty: Some("1e3".into()),
            source: "WGS84".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"R_E = 6.371 \times 10^6 \text{ m}".into(),
        },
        PhysicalConstant {
            id: "M_Earth".into(),
            name: "Earth mass".into(),
            symbol: "M_E".into(),
            value: "5.972e24".into(),
            unit: "kg".into(),
            uncertainty: Some("6e20".into()),
            source: "Geodesy".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"M_E = 5.972 \times 10^{24} \text{ kg}".into(),
        },
        PhysicalConstant {
            id: "g_surface".into(),
            name: "Earth surface gravity".into(),
            symbol: "g".into(),
            value: "9.81".into(),
            unit: "m/s²".into(),
            uncertainty: Some("0.01".into()),
            source: "Geodesy".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"g \approx 9.81 \text{ m/s}^2 \text{ (varies with location)}".into(),
        },
        PhysicalConstant {
            id: "rho_Earth".into(),
            name: "Earth mean density".into(),
            symbol: r"\bar{\rho}_E".into(),
            value: "5515".into(),
            unit: "kg/m³".into(),
            uncertainty: Some("5".into()),
            source: "Geodesy".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\bar{\rho}_E = 5515 \text{ kg/m}^3".into(),
        },
        PhysicalConstant {
            id: "Earth_age".into(),
            name: "Age of Earth".into(),
            symbol: "t_E".into(),
            value: "4.54e9".into(),
            unit: "years".into(),
            uncertainty: Some("0.05e9".into()),
            source: "Radiometric dating".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"t_E = 4.54 \pm 0.05 \times 10^9 \text{ years}".into(),
        },
        
        // Earth structure
        PhysicalConstant {
            id: "crust_thickness".into(),
            name: "Continental crust thickness".into(),
            symbol: "h_{crust}".into(),
            value: "35".into(),
            unit: "km".into(),
            uncertainty: Some("10".into()),
            source: "Seismology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"h_{crust} \approx 30-40 \text{ km (continental)}".into(),
        },
        PhysicalConstant {
            id: "oceanic_crust".into(),
            name: "Oceanic crust thickness".into(),
            symbol: "h_{oceanic}".into(),
            value: "7".into(),
            unit: "km".into(),
            uncertainty: Some("2".into()),
            source: "Seismology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"h_{oceanic} \approx 5-10 \text{ km}".into(),
        },
        PhysicalConstant {
            id: "core_radius".into(),
            name: "Earth outer core radius".into(),
            symbol: "R_{core}".into(),
            value: "3480".into(),
            unit: "km".into(),
            uncertainty: Some("10".into()),
            source: "Seismology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"R_{outer\_core} = 3480 \text{ km}".into(),
        },
        PhysicalConstant {
            id: "inner_core_radius".into(),
            name: "Earth inner core radius".into(),
            symbol: "R_{inner}".into(),
            value: "1220".into(),
            unit: "km".into(),
            uncertainty: Some("10".into()),
            source: "Seismology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"R_{inner\_core} = 1220 \text{ km}".into(),
        },
        
        // Seismology
        PhysicalConstant {
            id: "v_P_crust".into(),
            name: "P-wave velocity in crust".into(),
            symbol: "v_P".into(),
            value: "6".into(),
            unit: "km/s".into(),
            uncertainty: Some("0.5".into()),
            source: "Seismology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"v_P \approx 6 \text{ km/s (crust)}".into(),
        },
        PhysicalConstant {
            id: "v_S_crust".into(),
            name: "S-wave velocity in crust".into(),
            symbol: "v_S".into(),
            value: "3.5".into(),
            unit: "km/s".into(),
            uncertainty: Some("0.3".into()),
            source: "Seismology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"v_S \approx 3.5 \text{ km/s (crust)}".into(),
        },
        
        // Atmosphere
        PhysicalConstant {
            id: "scale_height".into(),
            name: "Atmospheric scale height".into(),
            symbol: "H".into(),
            value: "8.5".into(),
            unit: "km".into(),
            uncertainty: Some("0.5".into()),
            source: "Atmospheric physics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"H = \frac{k_B T}{m g} \approx 8.5 \text{ km}".into(),
        },
        PhysicalConstant {
            id: "tropopause_height".into(),
            name: "Tropopause height".into(),
            symbol: "h_{trop}".into(),
            value: "11".into(),
            unit: "km".into(),
            uncertainty: Some("3".into()),
            source: "Meteorology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"h_{tropopause} \approx 8-16 \text{ km (latitude dependent)}".into(),
        },
        PhysicalConstant {
            id: "lapse_rate".into(),
            name: "Tropospheric lapse rate".into(),
            symbol: r"\Gamma".into(),
            value: "6.5".into(),
            unit: "K/km".into(),
            uncertainty: Some("0.5".into()),
            source: "Meteorology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\Gamma \approx 6.5 \text{ K/km (average)}".into(),
        },
        
        // Geomagnetism
        PhysicalConstant {
            id: "B_Earth".into(),
            name: "Earth's magnetic field strength".into(),
            symbol: "B_E".into(),
            value: "25-65".into(),
            unit: "μT".into(),
            uncertainty: None,
            source: "Geomagnetism".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"B_E \approx 25-65 \text{ μT (location dependent)}".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // Seismology
        PhysicalLaw {
            id: "richter_scale".into(),
            name: "Richter Magnitude Scale".into(),
            domain: PhysicsDomain::Other("Seismology".into()),
            equation_tex: r"M_L = \log_{10}(A) - \log_{10}(A_0)".into(),
            description: "Local earthquake magnitude from amplitude".into(),
            variables: vec![
                Variable { symbol: "M_L".into(), name: "Local magnitude".into(), unit: None },
                Variable { symbol: "A".into(), name: "Maximum amplitude".into(), unit: Some("mm".into()) },
            ],
            source: "Richter 1935".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "moment_magnitude".into(),
            name: "Moment Magnitude".into(),
            domain: PhysicsDomain::Other("Seismology".into()),
            equation_tex: r"M_W = \frac{2}{3}\log_{10}(M_0) - 10.7".into(),
            description: "Magnitude from seismic moment".into(),
            variables: vec![
                Variable { symbol: "M_W".into(), name: "Moment magnitude".into(), unit: None },
                Variable { symbol: "M_0".into(), name: "Seismic moment".into(), unit: Some("N·m".into()) },
            ],
            source: "Hanks, Kanamori 1979".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "gutenberg_richter".into(),
            name: "Gutenberg-Richter Law".into(),
            domain: PhysicsDomain::Other("Seismology".into()),
            equation_tex: r"\log_{10} N = a - bM".into(),
            description: "Earthquake frequency-magnitude relation".into(),
            variables: vec![
                Variable { symbol: "N".into(), name: "Number of earthquakes ≥ M".into(), unit: None },
                Variable { symbol: "b".into(), name: "b-value (~1)".into(), unit: None },
            ],
            source: "Gutenberg, Richter 1944".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Atmosphere
        PhysicalLaw {
            id: "barometric_formula".into(),
            name: "Barometric Formula".into(),
            domain: PhysicsDomain::Other("Atmospheric Physics".into()),
            equation_tex: r"P(h) = P_0 e^{-h/H}".into(),
            description: "Pressure decrease with altitude".into(),
            variables: vec![
                Variable { symbol: "P".into(), name: "Pressure".into(), unit: Some("Pa".into()) },
                Variable { symbol: "H".into(), name: "Scale height".into(), unit: Some("km".into()) },
            ],
            source: "Atmospheric physics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "coriolis_force".into(),
            name: "Coriolis Force".into(),
            domain: PhysicsDomain::Other("Geophysics".into()),
            equation_tex: r"\mathbf{F}_C = -2m\boldsymbol{\Omega} \times \mathbf{v}".into(),
            description: "Apparent force in rotating reference frame".into(),
            variables: vec![
                Variable { symbol: "Ω".into(), name: "Earth's rotation rate".into(), unit: Some("rad/s".into()) },
                Variable { symbol: "v".into(), name: "Velocity".into(), unit: Some("m/s".into()) },
            ],
            source: "Coriolis 1835".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "rossby_number".into(),
            name: "Rossby Number".into(),
            domain: PhysicsDomain::Other("Geophysics".into()),
            equation_tex: r"Ro = \frac{U}{fL}".into(),
            description: "Ratio of inertial to Coriolis forces".into(),
            variables: vec![
                Variable { symbol: "U".into(), name: "Characteristic velocity".into(), unit: Some("m/s".into()) },
                Variable { symbol: "f".into(), name: "Coriolis parameter".into(), unit: Some("s⁻¹".into()) },
                Variable { symbol: "L".into(), name: "Length scale".into(), unit: Some("m".into()) },
            ],
            source: "Atmospheric dynamics".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Heat flow
        PhysicalLaw {
            id: "geothermal_gradient".into(),
            name: "Geothermal Gradient".into(),
            domain: PhysicsDomain::Thermodynamics,
            equation_tex: r"\frac{dT}{dz} \approx 25-30 \text{ °C/km}".into(),
            description: "Temperature increase with depth".into(),
            variables: vec![
                Variable { symbol: "dT/dz".into(), name: "Geothermal gradient".into(), unit: Some("°C/km".into()) },
            ],
            source: "Geophysics".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Isostasy
        PhysicalLaw {
            id: "airy_isostasy".into(),
            name: "Airy Isostasy".into(),
            domain: PhysicsDomain::Other("Geophysics".into()),
            equation_tex: r"\rho_c h = \rho_m r".into(),
            description: "Crustal thickness variation for isostatic equilibrium".into(),
            variables: vec![
                Variable { symbol: "ρ_c".into(), name: "Crustal density".into(), unit: Some("kg/m³".into()) },
                Variable { symbol: "ρ_m".into(), name: "Mantle density".into(), unit: Some("kg/m³".into()) },
            ],
            source: "Airy 1855".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Radioactive dating
        PhysicalLaw {
            id: "radioactive_decay_geo".into(),
            name: "Radioactive Decay (Geochronology)".into(),
            domain: PhysicsDomain::Other("Geochronology".into()),
            equation_tex: r"t = \frac{1}{\lambda}\ln\left(1 + \frac{D}{N}\right)".into(),
            description: "Age from parent-daughter ratio".into(),
            variables: vec![
                Variable { symbol: "t".into(), name: "Age".into(), unit: Some("years".into()) },
                Variable { symbol: "λ".into(), name: "Decay constant".into(), unit: Some("year⁻¹".into()) },
                Variable { symbol: "D".into(), name: "Daughter atoms".into(), unit: None },
                Variable { symbol: "N".into(), name: "Parent atoms".into(), unit: None },
            ],
            source: "Geochronology".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Ocean
        PhysicalLaw {
            id: "ekman_depth".into(),
            name: "Ekman Depth".into(),
            domain: PhysicsDomain::Other("Oceanography".into()),
            equation_tex: r"D_E = \sqrt{\frac{2A_z}{f}}".into(),
            description: "Depth of Ekman layer".into(),
            variables: vec![
                Variable { symbol: "D_E".into(), name: "Ekman depth".into(), unit: Some("m".into()) },
                Variable { symbol: "A_z".into(), name: "Vertical eddy viscosity".into(), unit: Some("m²/s".into()) },
            ],
            source: "Ekman 1905".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        MathTheorem {
            id: "plate_tectonics".into(),
            name: "Plate Tectonics".into(),
            category: TheoremCategory::Other("Geology".into()),
            statement_tex: r"\text{Earth's lithosphere divided into ~15 major plates}".into(),
            proof_tex: Some("Evidence: seafloor spreading, magnetic stripes, earthquakes".into()),
            references: vec![],
            source: "Wegener, Hess".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "mohorovicic".into(),
            name: "Mohorovičić Discontinuity".into(),
            category: TheoremCategory::Other("Seismology".into()),
            statement_tex: r"\text{Crust-mantle boundary marked by seismic velocity jump}".into(),
            proof_tex: Some("P-wave velocity increases from ~6.5 to ~8 km/s".into()),
            references: vec![],
            source: "Mohorovičić 1909".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "shadow_zone".into(),
            name: "Seismic Shadow Zone".into(),
            category: TheoremCategory::Other("Seismology".into()),
            statement_tex: r"\text{P-wave shadow: } 104°-140°, \text{ S-wave: } > 104°".into(),
            proof_tex: Some("Due to liquid outer core (blocks S-waves, refracts P-waves)".into()),
            references: vec![],
            source: "Seismology".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "seafloor_spreading".into(),
            name: "Seafloor Spreading".into(),
            category: TheoremCategory::Other("Geology".into()),
            statement_tex: r"\text{New oceanic crust forms at mid-ocean ridges, spreads ~2-10 cm/yr}".into(),
            proof_tex: Some("Magnetic stripe symmetry, age increase from ridge".into()),
            references: vec!["plate_tectonics".into()],
            source: "Hess, Dietz 1960s".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "rock_cycle".into(),
            name: "Rock Cycle".into(),
            category: TheoremCategory::Other("Geology".into()),
            statement_tex: r"\text{Igneous} \leftrightarrow \text{Sedimentary} \leftrightarrow \text{Metamorphic}".into(),
            proof_tex: Some("Continuous transformation through geological processes".into()),
            references: vec![],
            source: "Hutton 1788".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "geologic_time".into(),
            name: "Geologic Time Scale".into(),
            category: TheoremCategory::Other("Geology".into()),
            statement_tex: r"\text{Eon} > \text{Era} > \text{Period} > \text{Epoch}".into(),
            proof_tex: Some("Phanerozoic (541 Ma-present), Proterozoic, Archean, Hadean".into()),
            references: vec![],
            source: "Stratigraphy".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "snells_law_seismic".into(),
            name: "Snell's Law (Seismic)".into(),
            category: TheoremCategory::Other("Seismology".into()),
            statement_tex: r"\frac{\sin i_1}{v_1} = \frac{\sin i_2}{v_2} = p \text{ (ray parameter)}".into(),
            proof_tex: Some("Seismic ray bending at velocity discontinuities".into()),
            references: vec![],
            source: "Seismology".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "buys_ballot".into(),
            name: "Buys Ballot's Law".into(),
            category: TheoremCategory::Other("Meteorology".into()),
            statement_tex: r"\text{Northern Hemisphere: low pressure to left when wind at back}".into(),
            proof_tex: Some("Consequence of Coriolis effect on geostrophic wind".into()),
            references: vec![],
            source: "Buys Ballot 1857".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
