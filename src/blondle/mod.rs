//! Blondle: Universal Knowledge Bundle for S-wind
//!
//! A comprehensive external knowledge library covering all STEM fields.
//! Designed to extend S-wind's capabilities across engineering and sciences.
//!
//! # Structure
//! ## Core Sciences
//! - `chemistry`: Chemical constants, reactions, thermodynamics
//! - `biology`: Molecular biology, biochemistry, genetics
//! - `computer_science`: Algorithms, complexity theory, data structures
//! - `statistics`: Distributions, statistical tests, theorems
//!
//! ## General Engineering  
//! - `engineering`: Electrical, mechanical, civil engineering formulas
//! - `materials`: Material properties, crystallography
//!
//! ## Natural Sciences
//! - `earth_science`: Geology, geophysics, meteorology
//! - `astronomy`: Celestial mechanics, cosmology, stellar physics
//!
//! ## Industry-Specific (Mountable)
//! - `aerospace`: Wind tunnel testing, aerodynamics, aircraft design (AIAA, SAE)
//! - `automotive`: Vehicle dynamics, SAE standards, crashworthiness
//! - `finance`: Options pricing, portfolio theory, risk management
//! - `chemical_process`: Reactor design, separation, permeation
//! - `manufacturing`: GD&T, SPC, machining, quality control
//! - `electronics`: PCB design, signal integrity, EMC/EMI (IPC standards)
//!
//! # Usage
//! ```rust
//! use s_wind::blondle::BlondleBundle;
//!
//! let mut kb = KnowledgeBase::new();
//! BlondleBundle::load_all(&mut kb);
//! ```
//!
//! # Etymology
//! "Blondle" = Bundle of Learned Ontological Nodes for Domain-Level Expertise

// Core sciences
pub mod chemistry;
pub mod biology;
pub mod computer_science;
pub mod statistics;

// General engineering
pub mod engineering;
pub mod materials;

// Natural sciences
pub mod earth_science;
pub mod astronomy;

// Industry-specific (mountable modules)
pub mod aerospace;
pub mod automotive;
pub mod finance;
pub mod chemical_process;
pub mod manufacturing;
pub mod electronics;

use crate::knowledge::KnowledgeBase;

/// Blondle: Complete STEM knowledge bundle
pub struct BlondleBundle;

impl BlondleBundle {
    /// Load all Blondle knowledge into the knowledge base (core modules only)
    pub fn load_all(kb: &mut KnowledgeBase) {
        println!("[Blondle] Loading universal knowledge bundle...");
        
        // Core sciences
        chemistry::load(kb);
        biology::load(kb);
        
        // Computational
        computer_science::load(kb);
        statistics::load(kb);
        
        // Engineering
        engineering::load(kb);
        materials::load(kb);
        
        // Natural sciences
        earth_science::load(kb);
        astronomy::load(kb);
        
        println!("[Blondle] Core knowledge loaded successfully!");
    }
    
    /// Load all including industry-specific modules
    pub fn load_complete(kb: &mut KnowledgeBase) {
        Self::load_all(kb);
        
        println!("[Blondle] Loading industry-specific modules...");
        
        // Industry-specific
        aerospace::load(kb);
        automotive::load(kb);
        finance::load(kb);
        chemical_process::load(kb);
        manufacturing::load(kb);
        electronics::load(kb);
        
        println!("[Blondle] Complete knowledge bundle loaded!");
    }
    
    /// Load selective modules by name
    pub fn load_modules(kb: &mut KnowledgeBase, modules: &[&str]) {
        for module in modules {
            match *module {
                // Core sciences
                "chemistry" => chemistry::load(kb),
                "biology" => biology::load(kb),
                "computer_science" | "cs" => computer_science::load(kb),
                "statistics" | "stats" => statistics::load(kb),
                
                // General engineering
                "engineering" => engineering::load(kb),
                "materials" => materials::load(kb),
                
                // Natural sciences
                "earth_science" | "earth" => earth_science::load(kb),
                "astronomy" | "astro" => astronomy::load(kb),
                
                // Industry-specific
                "aerospace" | "aero" => aerospace::load(kb),
                "automotive" | "auto" => automotive::load(kb),
                "finance" | "fin" => finance::load(kb),
                "chemical_process" | "chem_proc" => chemical_process::load(kb),
                "manufacturing" | "mfg" => manufacturing::load(kb),
                "electronics" | "elec" => electronics::load(kb),
                
                _ => eprintln!("[Blondle] Unknown module: {}", module),
            }
        }
    }
    
    /// Get list of available modules
    pub fn available_modules() -> Vec<(&'static str, &'static str)> {
        vec![
            // Core
            ("chemistry", "Chemical constants, reactions, thermodynamics"),
            ("biology", "Molecular biology, biochemistry, genetics"),
            ("computer_science", "Algorithms, complexity, data structures"),
            ("statistics", "Distributions, tests, estimation theory"),
            // Engineering
            ("engineering", "Electrical, mechanical, civil formulas"),
            ("materials", "Material properties, crystallography"),
            // Natural sciences
            ("earth_science", "Geology, geophysics, meteorology"),
            ("astronomy", "Celestial mechanics, cosmology"),
            // Industry-specific
            ("aerospace", "Wind tunnel, aerodynamics, aircraft (AIAA/SAE)"),
            ("automotive", "Vehicle dynamics, crashworthiness (SAE)"),
            ("finance", "Options pricing, portfolio theory, risk"),
            ("chemical_process", "Reactor design, separation, permeation"),
            ("manufacturing", "GD&T, SPC, machining (ASME/ISO)"),
            ("electronics", "PCB design, signal integrity (IPC)"),
        ]
    }
    
    /// Get module statistics (core modules)
    pub fn stats() -> BlondleStats {
        BlondleStats {
            chemistry: chemistry::count(),
            biology: biology::count(),
            computer_science: computer_science::count(),
            engineering: engineering::count(),
            materials: materials::count(),
            earth_science: earth_science::count(),
            astronomy: astronomy::count(),
            statistics: statistics::count(),
            // Industry-specific
            aerospace: aerospace::count(),
            automotive: automotive::count(),
            finance: finance::count(),
            chemical_process: chemical_process::count(),
            manufacturing: manufacturing::count(),
            electronics: electronics::count(),
        }
    }
}

/// Statistics for Blondle knowledge bundle
#[derive(Debug, Clone)]
pub struct BlondleStats {
    // Core sciences (constants, laws, theorems)
    pub chemistry: (usize, usize, usize),
    pub biology: (usize, usize, usize),
    pub computer_science: (usize, usize, usize),
    pub statistics: (usize, usize, usize),
    // General engineering
    pub engineering: (usize, usize, usize),
    pub materials: (usize, usize, usize),
    // Natural sciences
    pub earth_science: (usize, usize, usize),
    pub astronomy: (usize, usize, usize),
    // Industry-specific
    pub aerospace: (usize, usize, usize),
    pub automotive: (usize, usize, usize),
    pub finance: (usize, usize, usize),
    pub chemical_process: (usize, usize, usize),
    pub manufacturing: (usize, usize, usize),
    pub electronics: (usize, usize, usize),
}

impl BlondleStats {
    pub fn total(&self) -> (usize, usize, usize) {
        let fields = [
            // Core
            self.chemistry,
            self.biology,
            self.computer_science,
            self.statistics,
            // Engineering
            self.engineering,
            self.materials,
            // Natural
            self.earth_science,
            self.astronomy,
            // Industry
            self.aerospace,
            self.automotive,
            self.finance,
            self.chemical_process,
            self.manufacturing,
            self.electronics,
        ];
        
        fields.iter().fold((0, 0, 0), |acc, x| {
            (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2)
        })
    }
    
    pub fn core_total(&self) -> (usize, usize, usize) {
        let fields = [
            self.chemistry, self.biology, self.computer_science, self.statistics,
            self.engineering, self.materials, self.earth_science, self.astronomy,
        ];
        fields.iter().fold((0, 0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2))
    }
    
    pub fn industry_total(&self) -> (usize, usize, usize) {
        let fields = [
            self.aerospace, self.automotive, self.finance,
            self.chemical_process, self.manufacturing, self.electronics,
        ];
        fields.iter().fold((0, 0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2))
    }
}

impl std::fmt::Display for BlondleStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "╔═══════════════════════════════════════════════════════════╗")?;
        writeln!(f, "║           Blondle Knowledge Bundle Statistics             ║")?;
        writeln!(f, "╠═══════════════════════════════════════════════════════════╣")?;
        writeln!(f, "║  Module             │ Constants │ Laws │ Theorems        ║")?;
        writeln!(f, "╟─────────────────────┼───────────┼──────┼─────────────────╢")?;
        
        // Core sciences
        writeln!(f, "║  ── Core Sciences ──────────────────────────────────────  ║")?;
        writeln!(f, "║  Chemistry          │ {:>9} │ {:>4} │ {:>15} ║", 
                 self.chemistry.0, self.chemistry.1, self.chemistry.2)?;
        writeln!(f, "║  Biology            │ {:>9} │ {:>4} │ {:>15} ║",
                 self.biology.0, self.biology.1, self.biology.2)?;
        writeln!(f, "║  Computer Science   │ {:>9} │ {:>4} │ {:>15} ║",
                 self.computer_science.0, self.computer_science.1, self.computer_science.2)?;
        writeln!(f, "║  Statistics         │ {:>9} │ {:>4} │ {:>15} ║",
                 self.statistics.0, self.statistics.1, self.statistics.2)?;
        
        // General engineering
        writeln!(f, "║  ── General Engineering ────────────────────────────────  ║")?;
        writeln!(f, "║  Engineering        │ {:>9} │ {:>4} │ {:>15} ║",
                 self.engineering.0, self.engineering.1, self.engineering.2)?;
        writeln!(f, "║  Materials          │ {:>9} │ {:>4} │ {:>15} ║",
                 self.materials.0, self.materials.1, self.materials.2)?;
        
        // Natural sciences
        writeln!(f, "║  ── Natural Sciences ───────────────────────────────────  ║")?;
        writeln!(f, "║  Earth Science      │ {:>9} │ {:>4} │ {:>15} ║",
                 self.earth_science.0, self.earth_science.1, self.earth_science.2)?;
        writeln!(f, "║  Astronomy          │ {:>9} │ {:>4} │ {:>15} ║",
                 self.astronomy.0, self.astronomy.1, self.astronomy.2)?;
        
        // Industry-specific
        writeln!(f, "║  ── Industry-Specific ──────────────────────────────────  ║")?;
        writeln!(f, "║  Aerospace          │ {:>9} │ {:>4} │ {:>15} ║",
                 self.aerospace.0, self.aerospace.1, self.aerospace.2)?;
        writeln!(f, "║  Automotive         │ {:>9} │ {:>4} │ {:>15} ║",
                 self.automotive.0, self.automotive.1, self.automotive.2)?;
        writeln!(f, "║  Finance            │ {:>9} │ {:>4} │ {:>15} ║",
                 self.finance.0, self.finance.1, self.finance.2)?;
        writeln!(f, "║  Chemical Process   │ {:>9} │ {:>4} │ {:>15} ║",
                 self.chemical_process.0, self.chemical_process.1, self.chemical_process.2)?;
        writeln!(f, "║  Manufacturing      │ {:>9} │ {:>4} │ {:>15} ║",
                 self.manufacturing.0, self.manufacturing.1, self.manufacturing.2)?;
        writeln!(f, "║  Electronics        │ {:>9} │ {:>4} │ {:>15} ║",
                 self.electronics.0, self.electronics.1, self.electronics.2)?;
        
        writeln!(f, "╟─────────────────────┼───────────┼──────┼─────────────────╢")?;
        let core = self.core_total();
        writeln!(f, "║  Core Subtotal      │ {:>9} │ {:>4} │ {:>15} ║",
                 core.0, core.1, core.2)?;
        let industry = self.industry_total();
        writeln!(f, "║  Industry Subtotal  │ {:>9} │ {:>4} │ {:>15} ║",
                 industry.0, industry.1, industry.2)?;
        writeln!(f, "╟─────────────────────┼───────────┼──────┼─────────────────╢")?;
        let total = self.total();
        writeln!(f, "║  TOTAL              │ {:>9} │ {:>4} │ {:>15} ║",
                 total.0, total.1, total.2)?;
        write!(f, "╚═══════════════════════════════════════════════════════════╝")
    }
}
