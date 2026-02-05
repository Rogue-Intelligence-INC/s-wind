//! S-wind Mount: Command-line utility for mounting Blondle knowledge modules
//!
//! This tool allows users to extend S-wind's capabilities by "mounting"
//! specialized knowledge modules from the Blondle bundle.
//!
//! Usage:
//!   cargo run --bin swind_mount -- <command> [args...]
//!
//! Commands:
//!   list              - List all available modules
//!   info <module>     - Get detailed information about a module
//!   load <modules...> - Load specified modules into knowledge base
//!   stats             - Show statistics for all modules
//!   demo              - Run a demo with all modules loaded

use std::env;

use s_wind::blondle::BlondleBundle;
use s_wind::knowledge::KnowledgeBase;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "list" => cmd_list(),
        "info" => {
            if args.len() < 3 {
                println!("Error: Missing module name for 'info' command.");
                println!("Usage: swind_mount info <module>");
                return;
            }
            cmd_info(&args[2]);
        }
        "load" => {
            if args.len() < 3 {
                println!("Error: Missing module names for 'load' command.");
                println!("Usage: swind_mount load <module1> [module2] ...");
                return;
            }
            let modules: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();
            cmd_load(&modules);
        }
        "stats" => cmd_stats(),
        "demo" => cmd_demo(),
        "help" | "-h" | "--help" => print_help(),
        _ => {
            println!("Error: Unknown command '{}'.", command);
            print_help();
        }
    }
}

fn print_help() {
    println!(r#"
╔═══════════════════════════════════════════════════════════════════════╗
║                S-wind Mount: Blondle Module Loader                    ║
╚═══════════════════════════════════════════════════════════════════════╝

Usage: cargo run --bin swind_mount -- <command> [args...]

Commands:
  list              List all available Blondle modules
  info <module>     Get detailed information about a specific module
  load <modules...> Load specified modules into knowledge base
  stats             Show statistics for all modules
  demo              Run a demo with complete knowledge loaded
  help              Show this help message

Examples:
  cargo run --bin swind_mount -- list
  cargo run --bin swind_mount -- info aerospace
  cargo run --bin swind_mount -- load aerospace automotive finance
  cargo run --bin swind_mount -- stats
  cargo run --bin swind_mount -- demo

Module Categories:
  Core Sciences:      chemistry, biology, computer_science, statistics
  General Engineering: engineering, materials
  Natural Sciences:    earth_science, astronomy
  Industry-Specific:   aerospace, automotive, finance, chemical_process,
                       manufacturing, electronics
"#);
}

fn cmd_list() {
    println!("\n╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║                  Available Blondle Modules                            ║");
    println!("╠═══════════════════════════════════════════════════════════════════════╣");
    
    let modules = BlondleBundle::available_modules();
    let mut current_category = "";
    
    for (name, desc) in modules {
        // Detect category changes
        let category = match name {
            "chemistry" | "biology" | "computer_science" | "statistics" => "Core Sciences",
            "engineering" | "materials" => "General Engineering",
            "earth_science" | "astronomy" => "Natural Sciences",
            _ => "Industry-Specific",
        };
        
        if category != current_category {
            println!("║ ── {} ──", category);
            println!("║    {:^19} │ {}", "-".repeat(19), "-".repeat(40));
            current_category = category;
        }
        
        println!("║    {:19} │ {:<40} ║", name, desc);
    }
    
    println!("╚═══════════════════════════════════════════════════════════════════════╝\n");
}

fn cmd_info(module: &str) {
    println!("\n╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║                    Module Information: {:28} ║", module);
    println!("╠═══════════════════════════════════════════════════════════════════════╣");
    
    let stats = BlondleBundle::stats();
    
    let (count, description, standards) = match module {
        "chemistry" => (
            stats.chemistry,
            "Chemical constants, thermodynamics, kinetics, electrochemistry",
            "IUPAC, NIST Chemistry WebBook",
        ),
        "biology" => (
            stats.biology,
            "Molecular biology, biochemistry, genetics, cell biology",
            "NCBI, UniProt, PDB standards",
        ),
        "computer_science" | "cs" => (
            stats.computer_science,
            "Algorithms, complexity theory, data structures, cryptography",
            "IEEE, ACM standards",
        ),
        "statistics" | "stats" => (
            stats.statistics,
            "Probability distributions, statistical tests, estimation theory",
            "ISO 3534, NIST Statistical Methods",
        ),
        "engineering" => (
            stats.engineering,
            "Electrical, mechanical, civil, control engineering formulas",
            "IEEE, ASME, ASCE standards",
        ),
        "materials" => (
            stats.materials,
            "Material properties, crystallography, phase diagrams",
            "ASM, ASTM standards",
        ),
        "earth_science" | "earth" => (
            stats.earth_science,
            "Geology, geophysics, meteorology, hydrology",
            "USGS, WMO standards",
        ),
        "astronomy" | "astro" => (
            stats.astronomy,
            "Celestial mechanics, stellar physics, cosmology",
            "IAU, NASA standards",
        ),
        "aerospace" | "aero" => (
            stats.aerospace,
            "Aerodynamics, wind tunnel testing, aircraft design, propulsion",
            "AIAA, SAE ARP, ICAO ISA, FAR Part 25",
        ),
        "automotive" | "auto" => (
            stats.automotive,
            "Vehicle dynamics, tire mechanics, powertrain, crashworthiness",
            "SAE J670, FMVSS, Euro NCAP, Pacejka Magic Formula",
        ),
        "finance" | "fin" => (
            stats.finance,
            "Options pricing, portfolio theory, risk management, fixed income",
            "Black-Scholes, Basel III, ISDA, Markowitz",
        ),
        "chemical_process" | "chem_proc" => (
            stats.chemical_process,
            "Reactor design, separation processes, mass/heat transfer, permeation",
            "TEMA, Arrhenius, Fick's Laws, McCabe-Thiele",
        ),
        "manufacturing" | "mfg" => (
            stats.manufacturing,
            "GD&T, SPC, machining, quality control, metrology",
            "ASME Y14.5, ISO 286, Six Sigma, GUM",
        ),
        "electronics" | "elec" => (
            stats.electronics,
            "PCB design, signal integrity, EMC/EMI, thermal management",
            "IPC-2221, IPC-2141, FCC Part 15, CISPR 32",
        ),
        _ => {
            println!("║  Module '{}' not found.", module);
            println!("║  Use 'swind_mount list' to see available modules.");
            println!("╚═══════════════════════════════════════════════════════════════════════╝\n");
            return;
        }
    };
    
    println!("║  Description: {}", description);
    println!("║  Standards:   {}", standards);
    println!("║");
    println!("║  Content:");
    println!("║    Constants:  {:>4}", count.0);
    println!("║    Laws:       {:>4}", count.1);
    println!("║    Theorems:   {:>4}", count.2);
    println!("║    ─────────────────");
    println!("║    Total:      {:>4}", count.0 + count.1 + count.2);
    println!("╚═══════════════════════════════════════════════════════════════════════╝\n");
}

fn cmd_load(modules: &[&str]) {
    println!("\n[S-wind Mount] Loading modules...\n");
    
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    
    let base_stats = kb.stats();
    let initial_count = base_stats.physical_constants + base_stats.math_constants 
                       + base_stats.theorems + base_stats.physical_laws;
    println!("  Base S-wind knowledge: {} entries\n", initial_count);
    
    BlondleBundle::load_modules(&mut kb, modules);
    
    let final_stats = kb.stats();
    let final_count = final_stats.physical_constants + final_stats.math_constants 
                     + final_stats.theorems + final_stats.physical_laws;
    let added = final_count - initial_count;
    
    println!("\n[S-wind Mount] Loading complete!");
    println!("  Added: {} entries", added);
    println!("  Total: {} entries\n", final_count);
}

fn cmd_stats() {
    println!("\n[S-wind Mount] Blondle Module Statistics\n");
    
    let stats = BlondleBundle::stats();
    println!("{}", stats);
    
    let total = stats.total();
    println!("\n  Total entries: {} (constants: {}, laws: {}, theorems: {})\n",
             total.0 + total.1 + total.2, total.0, total.1, total.2);
}

fn cmd_demo() {
    println!("\n╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║              S-wind + Blondle Complete Demo                           ║");
    println!("╚═══════════════════════════════════════════════════════════════════════╝\n");
    
    println!("[1] Initializing S-wind base knowledge...");
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    
    let base_stats = kb.stats();
    println!("    Physical constants: {}", base_stats.physical_constants);
    println!("    Math constants:     {}", base_stats.math_constants);
    println!("    Theorems:           {}", base_stats.theorems);
    println!("    Physical laws:      {}\n", base_stats.physical_laws);
    
    println!("[2] Loading complete Blondle bundle (all modules)...\n");
    BlondleBundle::load_complete(&mut kb);
    
    println!("\n[3] Final statistics:\n");
    let stats = BlondleBundle::stats();
    println!("{}", stats);
    
    println!("\n[4] Combined S-wind + Blondle knowledge:\n");
    let final_stats = kb.stats();
    println!("    Physical constants: {}", final_stats.physical_constants);
    println!("    Math constants:     {}", final_stats.math_constants);
    println!("    Theorems:           {}", final_stats.theorems);
    println!("    Physical laws:      {}", final_stats.physical_laws);
    println!("    ────────────────────────");
    let total = final_stats.physical_constants + final_stats.math_constants 
               + final_stats.theorems + final_stats.physical_laws;
    println!("    TOTAL ENTRIES:      {}\n", total);
    
    println!("╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║  S-wind is now equipped with comprehensive STEM knowledge!           ║");
    println!("║                                                                       ║");
    println!("║  Coverage includes:                                                   ║");
    println!("║    ✓ Physics (Classical, Quantum, Relativity, Thermodynamics)        ║");
    println!("║    ✓ Mathematics (Algebra, Analysis, Number Theory)                  ║");
    println!("║    ✓ Chemistry (Thermochem, Kinetics, Electrochemistry)              ║");
    println!("║    ✓ Biology (Molecular, Biochemistry, Genetics)                     ║");
    println!("║    ✓ Computer Science (Algorithms, Complexity)                       ║");
    println!("║    ✓ Engineering (Electrical, Mechanical, Civil)                     ║");
    println!("║    ✓ Materials Science (Properties, Crystallography)                 ║");
    println!("║    ✓ Earth Science (Geology, Meteorology)                            ║");
    println!("║    ✓ Astronomy (Celestial Mechanics, Cosmology)                      ║");
    println!("║    ✓ Statistics (Distributions, Tests, Estimation)                   ║");
    println!("║    ✓ Aerospace (Aerodynamics, Wind Tunnel, Aircraft Design)          ║");
    println!("║    ✓ Automotive (Vehicle Dynamics, SAE Standards)                    ║");
    println!("║    ✓ Finance (Options, Portfolio Theory, Risk)                       ║");
    println!("║    ✓ Chemical Process (Reactors, Separation, Permeation)             ║");
    println!("║    ✓ Manufacturing (GD&T, SPC, Quality Control)                      ║");
    println!("║    ✓ Electronics (PCB Design, Signal Integrity, EMC)                 ║");
    println!("╚═══════════════════════════════════════════════════════════════════════╝\n");
}
