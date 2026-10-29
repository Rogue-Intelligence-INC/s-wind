//! Biology Module for Blondle
//!
//! Covers:
//! - Molecular biology constants
//! - Biochemistry (enzyme kinetics, metabolism)
//! - Genetics (replication, transcription, translation)
//! - Cell biology fundamentals
//! - Biophysics

use crate::knowledge::{
    KnowledgeBase, PhysicalConstant, PhysicalLaw, MathTheorem,
    KnowledgeStatus, PhysicsDomain, TheoremCategory, Variable,
};

/// Load all biology knowledge
pub fn load(kb: &mut KnowledgeBase) {
    let before = (kb.physical_constants.len(), kb.physical_laws.len(), kb.theorems.len());
    add_constants(kb);
    add_laws(kb);
    add_theorems(kb);
    println!("[Blondle::Biology] Loaded {} constants, {} laws, {} theorems",
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
const LAW_COUNT: usize = 12;
const THEOREM_COUNT: usize = 10;

fn add_constants(kb: &mut KnowledgeBase) {
    let constants = vec![
        // Molecular biology
        PhysicalConstant {
            id: "bp_length".into(),
            name: "DNA base pair length".into(),
            symbol: "l_{bp}".into(),
            value: "0.34".into(),
            unit: "nm".into(),
            uncertainty: Some("0.01".into()),
            source: "Watson-Crick model".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"l_{bp} = 0.34 \text{ nm (rise per base pair)}".into(),
        },
        PhysicalConstant {
            id: "dna_helix_diameter".into(),
            name: "DNA double helix diameter".into(),
            symbol: "d_{DNA}".into(),
            value: "2.0".into(),
            unit: "nm".into(),
            uncertainty: Some("0.1".into()),
            source: "X-ray crystallography".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"d_{DNA} = 2.0 \text{ nm}".into(),
        },
        PhysicalConstant {
            id: "dna_pitch".into(),
            name: "DNA helix pitch (B-form)".into(),
            symbol: "p_{DNA}".into(),
            value: "3.4".into(),
            unit: "nm".into(),
            uncertainty: Some("0.1".into()),
            source: "B-DNA structure".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"p_{DNA} = 3.4 \text{ nm (10.5 bp per turn)}".into(),
        },
        PhysicalConstant {
            id: "ribosome_size".into(),
            name: "Ribosome diameter (eukaryotic)".into(),
            symbol: "d_{ribosome}".into(),
            value: "25".into(),
            unit: "nm".into(),
            uncertainty: Some("5".into()),
            source: "Cryo-EM studies".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"d_{ribosome} \approx 25 \text{ nm (80S)}".into(),
        },
        
        // Protein properties
        PhysicalConstant {
            id: "amino_acid_mass_avg".into(),
            name: "Average amino acid mass".into(),
            symbol: "m_{aa}".into(),
            value: "110".into(),
            unit: "Da".into(),
            uncertainty: Some("5".into()),
            source: "Statistical average".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"m_{aa} \approx 110 \text{ Da}".into(),
        },
        PhysicalConstant {
            id: "peptide_bond_length".into(),
            name: "Peptide bond length".into(),
            symbol: "l_{peptide}".into(),
            value: "0.133".into(),
            unit: "nm".into(),
            uncertainty: Some("0.002".into()),
            source: "Protein crystallography".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"l_{C-N} = 0.133 \text{ nm}".into(),
        },
        PhysicalConstant {
            id: "alpha_helix_pitch".into(),
            name: "Alpha helix pitch".into(),
            symbol: "p_{\\alpha}".into(),
            value: "0.54".into(),
            unit: "nm".into(),
            uncertainty: Some("0.01".into()),
            source: "Pauling, Corey".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"p_\alpha = 0.54 \text{ nm (3.6 residues/turn)}".into(),
        },
        
        // Bioenergetics
        PhysicalConstant {
            id: "atp_hydrolysis".into(),
            name: "ATP hydrolysis free energy".into(),
            symbol: r"\Delta G'^\circ_{ATP}".into(),
            value: "-30.5".into(),
            unit: "kJ/mol".into(),
            uncertainty: Some("0.5".into()),
            source: "Standard biochemistry".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\Delta G'^\circ(ATP \to ADP + P_i) = -30.5 \text{ kJ/mol}".into(),
        },
        PhysicalConstant {
            id: "nadh_oxidation".into(),
            name: "NADH oxidation free energy".into(),
            symbol: r"\Delta G'^\circ_{NADH}".into(),
            value: "-220".into(),
            unit: "kJ/mol".into(),
            uncertainty: Some("5".into()),
            source: "Biochemistry textbooks".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"\Delta G'^\circ(NADH \to NAD^+) = -220 \text{ kJ/mol}".into(),
        },
        
        // Membrane properties
        PhysicalConstant {
            id: "membrane_thickness".into(),
            name: "Lipid bilayer thickness".into(),
            symbol: "t_{membrane}".into(),
            value: "5".into(),
            unit: "nm".into(),
            uncertainty: Some("1".into()),
            source: "Membrane biophysics".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"t_{membrane} \approx 5 \text{ nm}".into(),
        },
        PhysicalConstant {
            id: "membrane_potential".into(),
            name: "Resting membrane potential (neuron)".into(),
            symbol: "V_m".into(),
            value: "-70".into(),
            unit: "mV".into(),
            uncertainty: Some("10".into()),
            source: "Neurophysiology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"V_m \approx -70 \text{ mV (resting)}".into(),
        },
        
        // Cell dimensions
        PhysicalConstant {
            id: "ecoli_length".into(),
            name: "E. coli cell length".into(),
            symbol: "l_{E.coli}".into(),
            value: "2".into(),
            unit: "μm".into(),
            uncertainty: Some("0.5".into()),
            source: "Microbiology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"l_{E.coli} \approx 2 \text{ μm}".into(),
        },
        PhysicalConstant {
            id: "human_cell_diameter".into(),
            name: "Typical human cell diameter".into(),
            symbol: "d_{cell}".into(),
            value: "10".into(),
            unit: "μm".into(),
            uncertainty: Some("5".into()),
            source: "Cell biology".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"d_{cell} \approx 10-20 \text{ μm}".into(),
        },
        
        // Genetics
        PhysicalConstant {
            id: "human_genome_size".into(),
            name: "Human genome size".into(),
            symbol: "N_{genome}".into(),
            value: "3.2e9".into(),
            unit: "bp".into(),
            uncertainty: Some("0.1e9".into()),
            source: "Human Genome Project".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"N_{genome} \approx 3.2 \times 10^9 \text{ bp}".into(),
        },
        PhysicalConstant {
            id: "human_gene_count".into(),
            name: "Human protein-coding genes".into(),
            symbol: "N_{genes}".into(),
            value: "20000".into(),
            unit: "genes".into(),
            uncertainty: Some("2000".into()),
            source: "ENCODE project".into(),
            status: KnowledgeStatus::Verified,
            tex_definition: r"N_{genes} \approx 20,000 \text{ protein-coding genes}".into(),
        },
    ];
    
    kb.physical_constants.extend(constants);
}

fn add_laws(kb: &mut KnowledgeBase) {
    let laws = vec![
        // Enzyme kinetics
        PhysicalLaw {
            id: "michaelis_menten".into(),
            name: "Michaelis-Menten Equation".into(),
            domain: PhysicsDomain::Other("Biochemistry".into()),
            equation_tex: r"v = \frac{V_{max}[S]}{K_M + [S]}".into(),
            description: "Rate equation for enzyme-catalyzed reactions".into(),
            variables: vec![
                Variable { symbol: "v".into(), name: "Reaction rate".into(), unit: Some("mol/(L·s)".into()) },
                Variable { symbol: "V_max".into(), name: "Maximum rate".into(), unit: Some("mol/(L·s)".into()) },
                Variable { symbol: "K_M".into(), name: "Michaelis constant".into(), unit: Some("mol/L".into()) },
                Variable { symbol: "[S]".into(), name: "Substrate concentration".into(), unit: Some("mol/L".into()) },
            ],
            source: "Michaelis, Menten 1913".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "lineweaver_burk".into(),
            name: "Lineweaver-Burk Equation".into(),
            domain: PhysicsDomain::Other("Biochemistry".into()),
            equation_tex: r"\frac{1}{v} = \frac{K_M}{V_{max}}\cdot\frac{1}{[S]} + \frac{1}{V_{max}}".into(),
            description: "Double-reciprocal plot for enzyme kinetics".into(),
            variables: vec![
                Variable { symbol: "v".into(), name: "Reaction rate".into(), unit: Some("mol/(L·s)".into()) },
            ],
            source: "Lineweaver, Burk 1934".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "hill_equation".into(),
            name: "Hill Equation".into(),
            domain: PhysicsDomain::Other("Biochemistry".into()),
            equation_tex: r"\theta = \frac{[L]^n}{K_d + [L]^n}".into(),
            description: "Cooperative binding with Hill coefficient n".into(),
            variables: vec![
                Variable { symbol: "θ".into(), name: "Fractional saturation".into(), unit: None },
                Variable { symbol: "n".into(), name: "Hill coefficient".into(), unit: None },
                Variable { symbol: "K_d".into(), name: "Dissociation constant".into(), unit: Some("mol/L".into()) },
            ],
            source: "Hill 1910".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Membrane transport
        PhysicalLaw {
            id: "nernst_potential".into(),
            name: "Nernst Equation (Biology)".into(),
            domain: PhysicsDomain::Other("Biophysics".into()),
            equation_tex: r"E_{ion} = \frac{RT}{zF}\ln\frac{[ion]_{out}}{[ion]_{in}}".into(),
            description: "Equilibrium potential for a single ion".into(),
            variables: vec![
                Variable { symbol: "E_ion".into(), name: "Equilibrium potential".into(), unit: Some("V".into()) },
                Variable { symbol: "z".into(), name: "Ion valence".into(), unit: None },
                Variable { symbol: "[ion]".into(), name: "Ion concentration".into(), unit: Some("mol/L".into()) },
            ],
            source: "Nernst (biological application)".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "goldman_hodgkin_katz".into(),
            name: "Goldman-Hodgkin-Katz Equation".into(),
            domain: PhysicsDomain::Other("Biophysics".into()),
            equation_tex: r"V_m = \frac{RT}{F}\ln\frac{P_K[K^+]_o + P_{Na}[Na^+]_o + P_{Cl}[Cl^-]_i}{P_K[K^+]_i + P_{Na}[Na^+]_i + P_{Cl}[Cl^-]_o}".into(),
            description: "Membrane potential with multiple ions".into(),
            variables: vec![
                Variable { symbol: "V_m".into(), name: "Membrane potential".into(), unit: Some("V".into()) },
                Variable { symbol: "P".into(), name: "Permeability".into(), unit: Some("m/s".into()) },
            ],
            source: "Goldman 1943".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "fick_diffusion_bio".into(),
            name: "Fick's Law (Membrane Diffusion)".into(),
            domain: PhysicsDomain::Other("Biophysics".into()),
            equation_tex: r"J = -D\frac{dC}{dx}".into(),
            description: "Diffusion flux across membrane".into(),
            variables: vec![
                Variable { symbol: "J".into(), name: "Flux".into(), unit: Some("mol/(m²·s)".into()) },
                Variable { symbol: "D".into(), name: "Diffusion coefficient".into(), unit: Some("m²/s".into()) },
            ],
            source: "Fick 1855".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Population genetics
        PhysicalLaw {
            id: "hardy_weinberg".into(),
            name: "Hardy-Weinberg Equilibrium".into(),
            domain: PhysicsDomain::Other("Genetics".into()),
            equation_tex: r"p^2 + 2pq + q^2 = 1".into(),
            description: "Allele frequency equilibrium in ideal population".into(),
            variables: vec![
                Variable { symbol: "p".into(), name: "Frequency of allele A".into(), unit: None },
                Variable { symbol: "q".into(), name: "Frequency of allele a".into(), unit: None },
            ],
            source: "Hardy 1908, Weinberg 1908".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "wright_fisher".into(),
            name: "Wright-Fisher Drift".into(),
            domain: PhysicsDomain::Other("Genetics".into()),
            equation_tex: r"\Delta p = p(1-p) \cdot \frac{s}{1 + sp}".into(),
            description: "Change in allele frequency under selection".into(),
            variables: vec![
                Variable { symbol: "p".into(), name: "Allele frequency".into(), unit: None },
                Variable { symbol: "s".into(), name: "Selection coefficient".into(), unit: None },
            ],
            source: "Wright, Fisher".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Molecular biology
        PhysicalLaw {
            id: "central_dogma".into(),
            name: "Central Dogma of Molecular Biology".into(),
            domain: PhysicsDomain::Other("Molecular Biology".into()),
            equation_tex: r"\text{DNA} \xrightarrow{\text{transcription}} \text{RNA} \xrightarrow{\text{translation}} \text{Protein}".into(),
            description: "Information flow in biological systems".into(),
            variables: vec![],
            source: "Crick 1958".into(),
            status: KnowledgeStatus::Verified,
        },
        
        // Growth dynamics
        PhysicalLaw {
            id: "exponential_growth_bio".into(),
            name: "Exponential Growth (Population)".into(),
            domain: PhysicsDomain::Other("Ecology".into()),
            equation_tex: r"\frac{dN}{dt} = rN".into(),
            description: "Unlimited population growth".into(),
            variables: vec![
                Variable { symbol: "N".into(), name: "Population size".into(), unit: None },
                Variable { symbol: "r".into(), name: "Intrinsic growth rate".into(), unit: Some("s⁻¹".into()) },
            ],
            source: "Malthus".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "logistic_growth".into(),
            name: "Logistic Growth".into(),
            domain: PhysicsDomain::Other("Ecology".into()),
            equation_tex: r"\frac{dN}{dt} = rN\left(1 - \frac{N}{K}\right)".into(),
            description: "Population growth with carrying capacity".into(),
            variables: vec![
                Variable { symbol: "N".into(), name: "Population size".into(), unit: None },
                Variable { symbol: "K".into(), name: "Carrying capacity".into(), unit: None },
            ],
            source: "Verhulst 1838".into(),
            status: KnowledgeStatus::Verified,
        },
        PhysicalLaw {
            id: "monod_equation".into(),
            name: "Monod Equation".into(),
            domain: PhysicsDomain::Other("Microbiology".into()),
            equation_tex: r"\mu = \mu_{max}\frac{S}{K_S + S}".into(),
            description: "Microbial growth rate vs substrate".into(),
            variables: vec![
                Variable { symbol: "μ".into(), name: "Growth rate".into(), unit: Some("h⁻¹".into()) },
                Variable { symbol: "S".into(), name: "Substrate concentration".into(), unit: Some("g/L".into()) },
                Variable { symbol: "K_S".into(), name: "Half-saturation constant".into(), unit: Some("g/L".into()) },
            ],
            source: "Monod 1949".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.physical_laws.extend(laws);
}

fn add_theorems(kb: &mut KnowledgeBase) {
    let theorems = vec![
        MathTheorem {
            id: "chargaff_rules".into(),
            name: "Chargaff's Rules".into(),
            category: TheoremCategory::Other("Molecular Biology".into()),
            statement_tex: r"[A] = [T], \quad [G] = [C]".into(),
            proof_tex: Some("Base pairing: A-T (2 H-bonds), G-C (3 H-bonds)".into()),
            references: vec![],
            source: "Chargaff 1950".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "codon_degeneracy".into(),
            name: "Genetic Code Degeneracy".into(),
            category: TheoremCategory::Other("Molecular Biology".into()),
            statement_tex: r"64 \text{ codons} \to 20 \text{ amino acids} + 3 \text{ stops}".into(),
            proof_tex: Some("Third position wobble allows synonymous codons".into()),
            references: vec![],
            source: "Genetic code".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "reading_frame".into(),
            name: "Reading Frame Theorem".into(),
            category: TheoremCategory::Other("Molecular Biology".into()),
            statement_tex: r"\text{3 possible reading frames per strand (6 total)}".into(),
            proof_tex: Some("Frameshift mutations change entire downstream sequence".into()),
            references: vec![],
            source: "Molecular biology".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "enzyme_inhibition_types".into(),
            name: "Enzyme Inhibition Types".into(),
            category: TheoremCategory::Other("Biochemistry".into()),
            statement_tex: r"\text{Competitive: } \uparrow K_M, \text{same } V_{max}; \text{ Non-competitive: same } K_M, \downarrow V_{max}".into(),
            proof_tex: Some("Based on inhibitor binding site (active vs allosteric)".into()),
            references: vec!["michaelis_menten".into()],
            source: "Enzyme kinetics".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "anfinsen_dogma".into(),
            name: "Anfinsen's Dogma".into(),
            category: TheoremCategory::Other("Biochemistry".into()),
            statement_tex: r"\text{Amino acid sequence determines 3D structure}".into(),
            proof_tex: Some("Denatured RNase can refold to active form".into()),
            references: vec![],
            source: "Anfinsen 1961".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "levinthal_paradox".into(),
            name: "Levinthal's Paradox".into(),
            category: TheoremCategory::Other("Biochemistry".into()),
            statement_tex: r"\text{Protein can't fold by random search (} 10^{300} \text{ conformations)}".into(),
            proof_tex: Some("Solution: folding follows a pathway/funnel".into()),
            references: vec!["anfinsen_dogma".into()],
            source: "Levinthal 1969".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "haldane_relationship".into(),
            name: "Haldane Relationship".into(),
            category: TheoremCategory::Other("Biochemistry".into()),
            statement_tex: r"\frac{V_{max,f}/K_{M,S}}{V_{max,r}/K_{M,P}} = K_{eq}".into(),
            proof_tex: Some("Links kinetic parameters to thermodynamic equilibrium".into()),
            references: vec!["michaelis_menten".into()],
            source: "Haldane 1930".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "stoichiometry_atp".into(),
            name: "ATP Synthesis Stoichiometry".into(),
            category: TheoremCategory::Other("Biochemistry".into()),
            statement_tex: r"\text{~2.5 ATP per NADH, ~1.5 ATP per FADH}_2".into(),
            proof_tex: Some("Based on proton gradient and ATP synthase rotation".into()),
            references: vec![],
            source: "Bioenergetics".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "mendelian_ratios".into(),
            name: "Mendelian Inheritance Ratios".into(),
            category: TheoremCategory::Other("Genetics".into()),
            statement_tex: r"\text{Monohybrid: } 3:1, \text{ Dihybrid: } 9:3:3:1".into(),
            proof_tex: Some("Independent assortment of alleles".into()),
            references: vec![],
            source: "Mendel 1865".into(),
            status: KnowledgeStatus::Verified,
        },
        MathTheorem {
            id: "allosteric_model".into(),
            name: "MWC Allosteric Model".into(),
            category: TheoremCategory::Other("Biochemistry".into()),
            statement_tex: r"L = \frac{[T_0]}{[R_0]}, \text{ all subunits switch together}".into(),
            proof_tex: Some("Concerted model: T (tense) ⇌ R (relaxed) equilibrium".into()),
            references: vec!["hill_equation".into()],
            source: "Monod, Wyman, Changeux 1965".into(),
            status: KnowledgeStatus::Verified,
        },
    ];
    
    kb.theorems.extend(theorems);
}
