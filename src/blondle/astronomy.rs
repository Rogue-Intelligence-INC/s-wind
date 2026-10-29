//! Astronomy Module for Blondle
//!
//! Covers:
//! - Celestial mechanics
//! - Stellar physics
//! - Cosmology extensions
//! - Observational astronomy

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all astronomy knowledge
pub fn load(kb: &mut KnowledgeBase) {
    let before = (kb.physical_constants.len(), kb.physical_laws.len(), kb.theorems.len());
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::Astronomy] Loaded {} constants, {} laws, {} theorems",
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
const LAW_COUNT: usize = 12;
const THEOREM_COUNT: usize = 10;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Distance units
        PhysicalConstant {
            id: "AU".into(),
            name: "Astronomical Unit".into(),
            symbol: "AU".into(),
            value: "1.495978707e11".into(),
            unit: "m".into(),
            uncertainty: Some("exact".into()),
            source: "IAU 2012".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"1 \text{ AU} = 1.495978707 \times 10^{11} \text{ m (exact)}".into(),
        },
        PhysicalConstant {
            id: "ly".into(),
            name: "Light-year".into(),
            symbol: "ly".into(),
            value: "9.4607e15".into(),
            unit: "m".into(),
            uncertainty: Some("exact".into()),
            source: "IAU".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"1 \text{ ly} = 9.4607 \times 10^{15} \text{ m}".into(),
        },
        PhysicalConstant {
            id: "pc".into(),
            name: "Parsec".into(),
            symbol: "pc".into(),
            value: "3.0857e16".into(),
            unit: "m".into(),
            uncertainty: Some("exact".into()),
            source: "IAU".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"1 \text{ pc} = 3.0857 \times 10^{16} \text{ m} = 3.262 \text{ ly}".into(),
        },
        
        // Solar properties
        PhysicalConstant {
            id: "M_Sun".into(),
            name: "Solar mass".into(),
            symbol: "M_\\odot".into(),
            value: "1.98892e30".into(),
            unit: "kg".into(),
            uncertainty: Some("2.5e25".into()),
            source: "IAU 2015".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"M_\odot = 1.98892 \times 10^{30} \text{ kg}".into(),
        },
        PhysicalConstant {
            id: "R_Sun".into(),
            name: "Solar radius".into(),
            symbol: "R_\\odot".into(),
            value: "6.96e8".into(),
            unit: "m".into(),
            uncertainty: Some("0.01e8".into()),
            source: "IAU 2015".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"R_\odot = 6.96 \times 10^8 \text{ m}".into(),
        },
        PhysicalConstant {
            id: "L_Sun".into(),
            name: "Solar luminosity".into(),
            symbol: "L_\\odot".into(),
            value: "3.828e26".into(),
            unit: "W".into(),
            uncertainty: Some("exact".into()),
            source: "IAU 2015".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"L_\odot = 3.828 \times 10^{26} \text{ W}".into(),
        },
        PhysicalConstant {
            id: "T_Sun".into(),
            name: "Solar effective temperature".into(),
            symbol: "T_\\odot".into(),
            value: "5772".into(),
            unit: "K".into(),
            uncertainty: Some("1".into()),
            source: "IAU 2015".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"T_{eff,\odot} = 5772 \text{ K}".into(),
        },
        
        // Cosmological
        PhysicalConstant {
            id: "H_0".into(),
            name: "Hubble constant".into(),
            symbol: "H_0".into(),
            value: "67.4".into(),
            unit: "km/s/Mpc".into(),
            uncertainty: Some("0.5".into()),
            source: "Planck 2018".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"H_0 = 67.4 \pm 0.5 \text{ km/s/Mpc}".into(),
        },
        PhysicalConstant {
            id: "t_universe".into(),
            name: "Age of universe".into(),
            symbol: "t_0".into(),
            value: "13.8e9".into(),
            unit: "years".into(),
            uncertainty: Some("0.02e9".into()),
            source: "Planck 2018".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"t_0 = 13.80 \pm 0.02 \times 10^9 \text{ years}".into(),
        },
        PhysicalConstant {
            id: "Omega_m".into(),
            name: "Matter density parameter".into(),
            symbol: r"\Omega_m".into(),
            value: "0.315".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.007".into()),
            source: "Planck 2018".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\Omega_m = 0.315 \pm 0.007".into(),
        },
        PhysicalConstant {
            id: "Omega_Lambda".into(),
            name: "Dark energy density parameter".into(),
            symbol: r"\Omega_\Lambda".into(),
            value: "0.685".into(),
            unit: "dimensionless".into(),
            uncertainty: Some("0.007".into()),
            source: "Planck 2018".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\Omega_\Lambda = 0.685 \pm 0.007".into(),
        },
        PhysicalConstant {
            id: "T_CMB".into(),
            name: "CMB temperature".into(),
            symbol: "T_{CMB}".into(),
            value: "2.7255".into(),
            unit: "K".into(),
            uncertainty: Some("0.0006".into()),
            source: "COBE/WMAP/Planck".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"T_{CMB} = 2.7255 \pm 0.0006 \text{ K}".into(),
        },
        
        // Magnitudes
        PhysicalConstant {
            id: "M_Sun_V".into(),
            name: "Solar absolute magnitude (V)".into(),
            symbol: "M_{V,\\odot}".into(),
            value: "4.83".into(),
            unit: "mag".into(),
            uncertainty: Some("0.01".into()),
            source: "IAU".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"M_{V,\odot} = +4.83 \text{ mag}".into(),
        },
        PhysicalConstant {
            id: "m_Sun_V".into(),
            name: "Solar apparent magnitude (V)".into(),
            symbol: "m_{V,\\odot}".into(),
            value: "-26.74".into(),
            unit: "mag".into(),
            uncertainty: Some("0.01".into()),
            source: "IAU".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"m_{V,\odot} = -26.74 \text{ mag}".into(),
        },
        
        // Other
        PhysicalConstant {
            id: "d_galactic_center".into(),
            name: "Distance to Galactic center".into(),
            symbol: "R_0".into(),
            value: "8.2".into(),
            unit: "kpc".into(),
            uncertainty: Some("0.1".into()),
            source: "GRAVITY 2019".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"R_0 = 8.2 \pm 0.1 \text{ kpc}".into(),
        },
        PhysicalConstant {
            id: "M_MW".into(),
            name: "Milky Way mass".into(),
            symbol: "M_{MW}".into(),
            value: "1.5e12".into(),
            unit: "M_☉".into(),
            uncertainty: Some("0.5e12".into()),
            source: "Dynamics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"M_{MW} \approx 1.5 \times 10^{12} M_\odot".into(),
        },
        PhysicalConstant {
            id: "rho_crit".into(),
            name: "Critical density of universe".into(),
            symbol: r"\rho_c".into(),
            value: "9.47e-27".into(),
            unit: "kg/m³".into(),
            uncertainty: Some("0.1e-27".into()),
            source: "Cosmology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\rho_c = \frac{3H_0^2}{8\pi G} \approx 9.5 \times 10^{-27} \text{ kg/m}^3".into(),
        },
        PhysicalConstant {
            id: "Chandrasekhar_limit".into(),
            name: "Chandrasekhar mass limit".into(),
            symbol: "M_{Ch}".into(),
            value: "1.4".into(),
            unit: "M_☉".into(),
            uncertainty: Some("0.02".into()),
            source: "Chandrasekhar 1931".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"M_{Ch} \approx 1.4 M_\odot".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // Stellar physics
        PhysicalLaw {
            id: "luminosity_radius_temp".into(),
            name: "Stefan-Boltzmann (Stellar)".into(),
            domain: PhysicsDomain::Other("Stellar Physics".into()),
            equation_tex: r"L = 4\pi R^2 \sigma T_{eff}^4".into(),
            description: "Stellar luminosity from radius and temperature".into(),
            variables: vec![
                Variable { symbol: "L".into(), name: "Luminosity".into(), unit: Some("W".into()) },
                Variable { symbol: "R".into(), name: "Stellar radius".into(), unit: Some("m".into()) },
                Variable { symbol: "T_eff".into(), name: "Effective temperature".into(), unit: Some("K".into()) },
            ],
            source: "Stellar physics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "mass_luminosity".into(),
            name: "Mass-Luminosity Relation".into(),
            domain: PhysicsDomain::Other("Stellar Physics".into()),
            equation_tex: r"L \propto M^{3.5}".into(),
            description: "Main sequence star luminosity vs mass".into(),
            variables: vec![
                Variable { symbol: "L".into(), name: "Luminosity".into(), unit: Some("L_☉".into()) },
                Variable { symbol: "M".into(), name: "Mass".into(), unit: Some("M_☉".into()) },
            ],
            source: "Eddington 1924".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "main_sequence_lifetime".into(),
            name: "Main Sequence Lifetime".into(),
            domain: PhysicsDomain::Other("Stellar Physics".into()),
            equation_tex: r"\tau_{MS} \approx 10^{10} \left(\frac{M}{M_\odot}\right)^{-2.5} \text{ years}".into(),
            description: "Time on main sequence".into(),
            variables: vec![
                Variable { symbol: "τ_MS".into(), name: "Main sequence lifetime".into(), unit: Some("years".into()) },
            ],
            source: "Stellar evolution".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Distance measurement
        PhysicalLaw {
            id: "parallax".into(),
            name: "Trigonometric Parallax".into(),
            domain: PhysicsDomain::Other("Astrometry".into()),
            equation_tex: r"d(\text{pc}) = \frac{1}{p(\text{arcsec})}".into(),
            description: "Distance from parallax angle".into(),
            variables: vec![
                Variable { symbol: "d".into(), name: "Distance".into(), unit: Some("pc".into()) },
                Variable { symbol: "p".into(), name: "Parallax angle".into(), unit: Some("arcsec".into()) },
            ],
            source: "Astrometry".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "distance_modulus".into(),
            name: "Distance Modulus".into(),
            domain: PhysicsDomain::Other("Astrometry".into()),
            equation_tex: r"m - M = 5\log_{10}\left(\frac{d}{10\text{ pc}}\right)".into(),
            description: "Relates apparent/absolute magnitude to distance".into(),
            variables: vec![
                Variable { symbol: "m".into(), name: "Apparent magnitude".into(), unit: Some("mag".into()) },
                Variable { symbol: "M".into(), name: "Absolute magnitude".into(), unit: Some("mag".into()) },
            ],
            source: "Photometry".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "pogson".into(),
            name: "Pogson's Equation".into(),
            domain: PhysicsDomain::Other("Photometry".into()),
            equation_tex: r"m_1 - m_2 = -2.5\log_{10}\left(\frac{F_1}{F_2}\right)".into(),
            description: "Magnitude difference from flux ratio".into(),
            variables: vec![
                Variable { symbol: "m".into(), name: "Magnitude".into(), unit: Some("mag".into()) },
                Variable { symbol: "F".into(), name: "Flux".into(), unit: Some("W/m²".into()) },
            ],
            source: "Pogson 1856".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Cosmology
        PhysicalLaw {
            id: "redshift_velocity".into(),
            name: "Cosmological Redshift".into(),
            domain: PhysicsDomain::GeneralRelativity,
            equation_tex: r"z = \frac{\lambda_{obs} - \lambda_{emit}}{\lambda_{emit}} = \frac{v}{c} \text{ (low z)}".into(),
            description: "Redshift definition and Doppler approximation".into(),
            variables: vec![
                Variable { symbol: "z".into(), name: "Redshift".into(), unit: None },
                Variable { symbol: "λ".into(), name: "Wavelength".into(), unit: Some("nm".into()) },
            ],
            source: "Cosmology".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "scale_factor_redshift".into(),
            name: "Scale Factor-Redshift Relation".into(),
            domain: PhysicsDomain::GeneralRelativity,
            equation_tex: r"1 + z = \frac{a_0}{a(t)}".into(),
            description: "Redshift from expansion".into(),
            variables: vec![
                Variable { symbol: "a".into(), name: "Scale factor".into(), unit: None },
                Variable { symbol: "z".into(), name: "Redshift".into(), unit: None },
            ],
            source: "Cosmology".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "lookback_time".into(),
            name: "Lookback Time".into(),
            domain: PhysicsDomain::GeneralRelativity,
            equation_tex: r"t_{lookback} = \int_0^z \frac{dz'}{(1+z')H(z')}".into(),
            description: "Time since light was emitted".into(),
            variables: vec![
                Variable { symbol: "t_lookback".into(), name: "Lookback time".into(), unit: Some("Gyr".into()) },
                Variable { symbol: "H(z)".into(), name: "Hubble parameter at z".into(), unit: Some("km/s/Mpc".into()) },
            ],
            source: "Cosmology".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Celestial mechanics
        PhysicalLaw {
            id: "vis_viva".into(),
            name: "Vis-Viva Equation".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"v^2 = GM\left(\frac{2}{r} - \frac{1}{a}\right)".into(),
            description: "Orbital velocity at any point".into(),
            variables: vec![
                Variable { symbol: "v".into(), name: "Orbital velocity".into(), unit: Some("m/s".into()) },
                Variable { symbol: "r".into(), name: "Current distance".into(), unit: Some("m".into()) },
                Variable { symbol: "a".into(), name: "Semi-major axis".into(), unit: Some("m".into()) },
            ],
            source: "Orbital mechanics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "escape_velocity".into(),
            name: "Escape Velocity".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"v_{esc} = \sqrt{\frac{2GM}{r}}".into(),
            description: "Minimum velocity to escape gravitational field".into(),
            variables: vec![
                Variable { symbol: "v_esc".into(), name: "Escape velocity".into(), unit: Some("m/s".into()) },
            ],
            source: "Mechanics".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "roche_limit".into(),
            name: "Roche Limit".into(),
            domain: PhysicsDomain::ClassicalMechanics,
            equation_tex: r"d = R_M \left(\frac{2\rho_M}{\rho_m}\right)^{1/3}".into(),
            description: "Minimum distance before tidal disruption".into(),
            variables: vec![
                Variable { symbol: "d".into(), name: "Roche limit".into(), unit: Some("m".into()) },
                Variable { symbol: "R_M".into(), name: "Primary radius".into(), unit: Some("m".into()) },
                Variable { symbol: "ρ".into(), name: "Density".into(), unit: Some("kg/m³".into()) },
            ],
            source: "Roche 1848".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        MathTheorem {
            id: "hr_diagram".into(),
            name: "Hertzsprung-Russell Diagram".into(),
            category: TheoremCategory::Other("Stellar Physics".into()),
            statement_tex: r"\text{Stars cluster on Main Sequence, Giants, Supergiants, White Dwarfs}".into(),
            proof_tex: Some("Luminosity vs temperature classification".into()),
            references: vec![],
            source: "Hertzsprung 1911, Russell 1913".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "stellar_classification".into(),
            name: "Stellar Spectral Classification".into(),
            category: TheoremCategory::Other("Stellar Physics".into()),
            statement_tex: r"\text{O B A F G K M (hot to cool)}".into(),
            proof_tex: Some("Based on absorption lines and temperature".into()),
            references: vec![],
            source: "Harvard classification".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "eddington_limit".into(),
            name: "Eddington Luminosity Limit".into(),
            category: TheoremCategory::Other("Stellar Physics".into()),
            statement_tex: r"L_{Edd} = \frac{4\pi GMm_p c}{\sigma_T}".into(),
            proof_tex: Some("Maximum luminosity where radiation pressure balances gravity".into()),
            references: vec![],
            source: "Eddington".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "chandrasekhar_theorem".into(),
            name: "Chandrasekhar Limit".into(),
            category: TheoremCategory::Other("Stellar Physics".into()),
            statement_tex: r"M_{WD} < 1.4 M_\odot \text{ for electron degeneracy support}".into(),
            proof_tex: Some("Above this, white dwarf collapses".into()),
            references: vec![],
            source: "Chandrasekhar 1931".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "tolman_oppenheimer".into(),
            name: "Tolman-Oppenheimer-Volkoff Limit".into(),
            category: TheoremCategory::Other("Stellar Physics".into()),
            statement_tex: r"M_{NS} \lesssim 2-3 M_\odot".into(),
            proof_tex: Some("Maximum neutron star mass before black hole collapse".into()),
            references: vec![],
            source: "TOV 1939".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "hubble_sequence".into(),
            name: "Hubble Sequence".into(),
            category: TheoremCategory::Other("Galaxies".into()),
            statement_tex: r"\text{Ellipticals (E0-E7)} \to \text{S0} \to \text{Spirals (Sa-Sd), Barred (SBa-SBd)}".into(),
            proof_tex: Some("Morphological classification of galaxies".into()),
            references: vec![],
            source: "Hubble 1926".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "olbers_paradox".into(),
            name: "Olbers' Paradox".into(),
            category: TheoremCategory::Other("Cosmology".into()),
            statement_tex: r"\text{Why is the night sky dark in infinite universe?}".into(),
            proof_tex: Some("Resolved by: finite age, expansion, finite speed of light".into()),
            references: vec![],
            source: "Olbers 1823".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "big_bang_evidence".into(),
            name: "Big Bang Evidence".into(),
            category: TheoremCategory::Other("Cosmology".into()),
            statement_tex: r"\text{CMB, Hubble expansion, light element abundances}".into(),
            proof_tex: Some("Three pillars of Big Bang cosmology".into()),
            references: vec![],
            source: "Modern cosmology".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "cosmic_distance_ladder".into(),
            name: "Cosmic Distance Ladder".into(),
            category: TheoremCategory::Other("Astrometry".into()),
            statement_tex: r"\text{Parallax} \to \text{Cepheids} \to \text{Type Ia SNe} \to \text{Hubble Law}".into(),
            proof_tex: Some("Overlapping methods for increasing distances".into()),
            references: vec![],
            source: "Observational astronomy".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "drake_equation".into(),
            name: "Drake Equation".into(),
            category: TheoremCategory::Other("Astrobiology".into()),
            statement_tex: r"N = R_* \cdot f_p \cdot n_e \cdot f_l \cdot f_i \cdot f_c \cdot L".into(),
            proof_tex: Some("Estimates communicating civilizations in galaxy".into()),
            references: vec![],
            source: "Drake 1961".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
