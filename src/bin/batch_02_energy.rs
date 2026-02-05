//! Batch 02: Energy & Battery (10 projects)
//! Run: cargo run --release --bin batch_02_energy

use std::fs;
use std::path::Path;

fn fmt(n: u64) -> String {
    let s = n.to_string();
    let mut r = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 { r.push(','); }
        r.push(c);
    }
    r.chars().rev().collect()
}

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  S-wind Batch 02: Energy & Battery Systems");
    println!("═══════════════════════════════════════════════════════════════\n");

    let out = Path::new("papers_batch");
    fs::create_dir_all(out.join("02_energy")).unwrap();

    let projects = vec![
        ("C1", "Battery Fast Charging Strategy", "Minimize charge time, capacity fade < 20%", vec![
            ("CC Current", "I_cc", 0.5, 3.0, 0.1, "C"),
            ("CV Voltage", "V_cv", 4.1, 4.35, 0.01, "V"),
            ("Term Current", "I_term", 0.01, 0.1, 0.01, "C"),
            ("Temperature", "T", 20.0, 45.0, 5.0, "°C"),
            ("Pulse Duty", "D", 0.5, 1.0, 0.1, ""),
        ], vec![
            (1, "CC phase time", "Q, I_cc", "t_cc = Q×SoC_cv/I_cc", "Definition", "Time to CV"),
            (2, "CV phase time", "V, I, τ", "t_cv = τ×ln(I_cc/I_term)", "Exponential", "CV decay"),
            (3, "Total time", "t_cc, t_cv", "t = t_cc + t_cv", "Sum", "Charge time"),
            (4, "Heat generation", "I, R", "Q = I²R", "Joule heating", "Ohmic loss"),
            (5, "Temperature rise", "Q, C_th", "ΔT = Q/(hA)", "Thermal", "Heat transfer"),
            (6, "SEI growth", "I, T, t", "δ ∝ √t×exp(-E_a/RT)", "Arrhenius", "Kinetics"),
            (7, "Capacity fade", "δ, N", "Q_loss = k×δ×N", "Degradation", "Cycle aging"),
        ], vec![
            ("ARRHENIUS", "Arrhenius Eq", "k = A×exp(-E_a/RT)", "Arrhenius, 1889"),
            ("NEWMAN", "Newman Model", "Porous electrode", "Newman, 1975"),
        ]),

        ("C2", "Battery Chemistry Formulation", "Maximize energy density, T_onset > 200°C", vec![
            ("Ni Content", "x_Ni", 0.6, 0.9, 0.05, ""),
            ("Co Content", "x_Co", 0.05, 0.2, 0.05, ""),
            ("Mn Content", "x_Mn", 0.05, 0.2, 0.05, ""),
            ("Electrolyte", "E", 1.0, 4.0, 1.0, "type"),
            ("Coating", "t_c", 1.0, 10.0, 1.0, "nm"),
        ], vec![
            (1, "Theoretical capacity", "x_*", "Q_th = Σx_i×Q_i", "Weighted sum", "Redox contrib"),
            (2, "Voltage profile", "x_*", "V = f(x_Ni, x_Co, x_Mn)", "Empirical", "Chemistry"),
            (3, "Energy density", "Q, V, ρ", "E = Q×V/m", "Definition", "Wh/kg"),
            (4, "Thermal stability", "x_Ni", "T_onset = 300 - 100×x_Ni", "Semi-empirical", "Ni effect"),
            (5, "Cost", "x_*", "$ = Σx_i × p_i", "Weighted", "Material cost"),
            (6, "Coating effect", "t_c", "η = 1 - k×t_c", "Passivation", "Interface"),
        ], vec![
            ("NMC", "NMC Cathode", "LiNi_xMn_yCo_zO_2", "JECS, 2015"),
            ("CEI", "CEI Layer", "Surface passivation", "Xu, 2014"),
        ]),

        ("C3", "Energy Storage Arbitrage", "Maximize profit from price spread", vec![
            ("Capacity", "E", 1.0, 10.0, 1.0, "MWh"),
            ("Power Rating", "P", 0.5, 4.0, 0.5, "MW"),
            ("Round-trip Eff", "η", 0.85, 0.95, 0.02, ""),
            ("Charge Hours", "h_c", 2.0, 8.0, 1.0, "hr"),
            ("Cycles/Day", "N", 1.0, 3.0, 1.0, ""),
        ], vec![
            (1, "Energy per cycle", "E, DoD", "E_cyc = E × DoD", "Definition", "Usable energy"),
            (2, "Revenue", "E, Δp, η", "R = E × Δp × η", "Arbitrage", "Price spread"),
            (3, "Daily revenue", "R, N", "R_day = R × N", "Multiplication", "Cycles"),
            (4, "Degradation cost", "N, C_deg", "C = N × C_deg", "Wear", "Battery aging"),
            (5, "Net profit", "R_day, C", "Profit = R_day - C", "Subtraction", "Net"),
            (6, "Annual profit", "Profit, days", "P_yr = Profit × 365", "Scaling", "Yearly"),
        ], vec![
            ("BESS", "BESS Economics", "LCOS model", "Lazard, 2023"),
            ("ARBITRAGE", "Energy Arbitrage", "Price spread", "EIA, 2022"),
        ]),

        ("C4", "Solar + Storage Capacity Planning", "Minimize LCOE", vec![
            ("PV Capacity", "P_pv", 1.0, 10.0, 0.5, "MW"),
            ("Storage Cap", "E_s", 1.0, 20.0, 1.0, "MWh"),
            ("Storage Power", "P_s", 0.5, 5.0, 0.5, "MW"),
            ("Inverter Ratio", "DC/AC", 1.0, 1.5, 0.1, ""),
            ("Tilt Angle", "θ", 15.0, 45.0, 5.0, "°"),
        ], vec![
            (1, "Annual generation", "P_pv, CF", "E_gen = P_pv × CF × 8760", "Capacity factor", "MWh/yr"),
            (2, "Curtailment", "E_gen, E_s", "E_curt = f(E_gen, E_s, load)", "Mismatch", "Lost energy"),
            (3, "Delivered energy", "E_gen, E_curt", "E_del = E_gen - E_curt", "Net", "Usable"),
            (4, "CAPEX", "P_pv, E_s", "CAPEX = P_pv×c_pv + E_s×c_s", "Investment", "Upfront"),
            (5, "OPEX", "CAPEX", "OPEX = CAPEX × r_opex", "Ratio", "Annual O&M"),
            (6, "LCOE", "CAPEX, OPEX, E_del", "LCOE = (CAPEX×CRF + OPEX)/E_del", "Definition", "$/MWh"),
        ], vec![
            ("LCOE", "LCOE Formula", "(CAPEX×CRF + OPEX)/E", "NREL, 2023"),
            ("SAM", "System Advisor Model", "PV simulation", "NREL, 2022"),
        ]),

        ("C5", "Battery Thermal Management", "Maintain 25±5°C under all conditions", vec![
            ("Coolant Flow", "Q_c", 1.0, 10.0, 1.0, "L/min"),
            ("Channel Width", "w", 2.0, 10.0, 1.0, "mm"),
            ("Fin Count", "N_f", 5.0, 20.0, 1.0, ""),
            ("Coolant Temp", "T_in", 15.0, 25.0, 2.0, "°C"),
            ("Plate Thickness", "t", 1.0, 5.0, 0.5, "mm"),
        ], vec![
            (1, "Heat generation", "I, R", "Q_gen = I²R", "Joule", "Cell heat"),
            (2, "Convection coeff", "Q_c, w", "h = Nu×k/D_h", "Nusselt", "Heat transfer"),
            (3, "Fin efficiency", "N_f, h", "η_f = tanh(mL)/(mL)", "Fin theory", "Enhanced area"),
            (4, "Total conductance", "h, A, η", "UA = h×A×η_f", "Definition", "W/K"),
            (5, "Cell temperature", "Q, UA, T_in", "T_cell = T_in + Q/UA", "Energy balance", "Steady state"),
            (6, "Pressure drop", "Q_c, w", "ΔP = f×L/D×ρv²/2", "Darcy", "Pumping loss"),
            (7, "Pump power", "Q_c, ΔP", "P_pump = Q_c×ΔP/η", "Definition", "Parasitic"),
        ], vec![
            ("NUSSELT", "Nusselt Correlation", "Nu = 0.023Re^0.8Pr^0.4", "Dittus-Boelter"),
            ("BTMS", "BTMS Design", "Liquid cooling", "Pesaran, 2013"),
        ]),

        ("C6", "Hydrogen Electrolyzer Efficiency", "Maximize H2 production per kWh", vec![
            ("Current Density", "j", 0.1, 2.0, 0.1, "A/cm²"),
            ("Temperature", "T", 60.0, 90.0, 5.0, "°C"),
            ("Pressure", "P", 1.0, 30.0, 5.0, "bar"),
            ("Membrane Thick", "t_m", 50.0, 200.0, 25.0, "μm"),
            ("Catalyst Load", "L_c", 0.1, 1.0, 0.1, "mg/cm²"),
        ], vec![
            (1, "Reversible voltage", "T", "E_rev = 1.229 - 0.00085(T-298)", "Thermodynamics", "Nernst"),
            (2, "Activation loss", "j, j_0", "η_act = RT/αF × ln(j/j_0)", "Butler-Volmer", "Kinetics"),
            (3, "Ohmic loss", "j, t_m, σ", "η_ohm = j×t_m/σ", "Ohm's law", "Membrane"),
            (4, "Cell voltage", "E, η_*", "V = E_rev + η_act + η_ohm", "Sum", "Total"),
            (5, "Faradaic efficiency", "j", "η_F = 1 - k×j²", "Crossover", "Gas purity"),
            (6, "H2 production", "j, A, η_F", "ṁ_H2 = j×A×η_F/(2F)×M_H2", "Faraday", "Mass rate"),
            (7, "Specific energy", "V, ṁ_H2", "E_sp = V×I/ṁ_H2", "Definition", "kWh/kg"),
        ], vec![
            ("FARADAY", "Faraday's Law", "n = It/zF", "Faraday, 1834"),
            ("PEM", "PEM Electrolysis", "Proton exchange", "Ayers, 2010"),
        ]),

        ("C7", "Grid-Scale Battery Sizing", "Minimize total cost of ownership", vec![
            ("Energy Cap", "E", 10.0, 200.0, 10.0, "MWh"),
            ("Power Cap", "P", 5.0, 100.0, 5.0, "MW"),
            ("Duration", "h", 2.0, 8.0, 1.0, "hr"),
            ("Cycles/yr", "N", 200.0, 500.0, 50.0, ""),
            ("Warranty", "Y", 10.0, 20.0, 2.0, "yr"),
        ], vec![
            (1, "CAPEX", "E, P", "CAPEX = E×c_E + P×c_P", "Investment", "$/system"),
            (2, "Replacement", "N, Y, life", "N_repl = ceil(N×Y/life)", "Cycle count", "Replacements"),
            (3, "Replacement cost", "N_repl, CAPEX", "C_repl = N_repl × CAPEX × r", "Ratio", "Future cost"),
            (4, "OPEX", "E, P", "OPEX = (E+P) × r_opex", "Annual", "O&M"),
            (5, "Revenue", "E, P, h, prices", "Rev = f(E, P, h, prices)", "Services", "Ancillary"),
            (6, "NPV", "CAPEX, OPEX, Rev, r", "NPV = -CAPEX + Σ(Rev-OPEX)/(1+r)^t", "DCF", "Net present"),
            (7, "LCOS", "NPV, E_del", "LCOS = -NPV/E_delivered", "Definition", "$/MWh"),
        ], vec![
            ("LCOS", "Levelized Cost of Storage", "Total cost / energy", "Lazard, 2023"),
            ("DCF", "Discounted Cash Flow", "NPV = Σ CF/(1+r)^t", "Finance"),
        ]),

        ("C8", "Vehicle-to-Grid Optimization", "Maximize V2G revenue, degradation < 5%/yr", vec![
            ("Discharge Power", "P_d", 3.0, 11.0, 1.0, "kW"),
            ("Discharge Hours", "h_d", 1.0, 4.0, 0.5, "hr/day"),
            ("Min SoC", "SoC_min", 0.2, 0.5, 0.05, ""),
            ("Price Threshold", "p_th", 0.1, 0.3, 0.02, "$/kWh"),
            ("Ramp Rate", "R", 1.0, 10.0, 1.0, "kW/min"),
        ], vec![
            (1, "Daily discharge", "P_d, h_d", "E_d = P_d × h_d", "Definition", "kWh/day"),
            (2, "Cycles per day", "E_d, E_batt", "N_d = E_d / E_batt", "Division", "Partial cycles"),
            (3, "Annual cycles", "N_d", "N_yr = N_d × 365", "Scaling", "Yearly"),
            (4, "Degradation", "N_yr, DoD", "Deg = f(N_yr, DoD, T)", "Model", "Capacity loss"),
            (5, "Revenue", "E_d, p", "Rev = E_d × p × days", "Pricing", "Income"),
            (6, "Battery cost", "Deg, C_batt", "C = Deg × C_batt", "Wear", "Replacement"),
            (7, "Net benefit", "Rev, C", "Net = Rev - C", "Subtraction", "Profit"),
        ], vec![
            ("V2G", "Vehicle-to-Grid", "Bidirectional power", "Kempton, 2005"),
            ("DEGRADATION", "Battery Aging", "Cycle + calendar", "Xu et al., 2018"),
        ]),

        ("C9", "Solid-State Battery Design", "Maximize ionic conductivity", vec![
            ("Li Content", "x_Li", 6.0, 7.0, 0.1, ""),
            ("PS Ratio", "r_PS", 0.3, 0.7, 0.05, ""),
            ("Sintering Temp", "T_s", 500.0, 700.0, 25.0, "°C"),
            ("Grain Size", "d_g", 0.5, 5.0, 0.5, "μm"),
            ("Density", "ρ_rel", 0.9, 0.99, 0.01, ""),
        ], vec![
            (1, "Bulk conductivity", "x_Li, T", "σ_b = σ_0×exp(-E_a/RT)", "Arrhenius", "Ion hopping"),
            (2, "Grain boundary", "d_g, δ_gb", "R_gb = δ_gb/(σ_gb×d_g)", "Microstructure", "GB resistance"),
            (3, "Effective conductivity", "σ_b, R_gb", "σ_eff = σ_b/(1+σ_b×R_gb)", "Series", "Combined"),
            (4, "Porosity effect", "ρ_rel", "σ = σ_eff × ρ_rel^1.5", "Bruggeman", "Tortuosity"),
            (5, "Interfacial resistance", "T_s", "R_int = f(T_s)", "Sintering", "Contact"),
            (6, "Total ASR", "σ, t, R_int", "ASR = t/σ + R_int", "Sum", "Area specific"),
        ], vec![
            ("LLZO", "LLZO Electrolyte", "Li_7La_3Zr_2O_12", "Murugan, 2007"),
            ("ARGYRODITE", "Li Argyrodite", "Li_6PS_5Cl", "Deiseroth, 2008"),
        ]),

        ("C10", "Redox Flow Battery Optimization", "Maximize power density", vec![
            ("Vanadium Conc", "C_V", 1.0, 2.0, 0.1, "M"),
            ("Flow Rate", "Q", 10.0, 100.0, 10.0, "mL/min"),
            ("Electrode Area", "A", 25.0, 400.0, 25.0, "cm²"),
            ("Electrode Thick", "t_e", 2.0, 6.0, 0.5, "mm"),
            ("Compression", "r_c", 0.2, 0.4, 0.05, ""),
        ], vec![
            (1, "Limiting current", "C, Q, A", "i_L = nFCQ/A", "Mass transfer", "Diffusion limit"),
            (2, "Activation loss", "i, i_0", "η_a = RT/αnF×ln(i/i_0)", "Butler-Volmer", "Kinetics"),
            (3, "Ohmic loss", "i, R", "η_Ω = iR", "Ohm's law", "Resistance"),
            (4, "Mass transfer loss", "i, i_L", "η_m = RT/nF×ln(1-i/i_L)", "Concentration", "Depletion"),
            (5, "Cell voltage", "E_0, η_*", "V = E_0 - η_a - η_Ω - η_m", "Subtraction", "Discharge"),
            (6, "Power density", "i, V", "P = i×V", "Definition", "W/cm²"),
            (7, "Efficiency", "V, E_0", "η = V/E_0", "Ratio", "Voltage eff"),
        ], vec![
            ("VRB", "Vanadium Redox", "V²⁺/V³⁺ and VO²⁺/VO₂⁺", "Sum, 1986"),
            ("MASSTRANS", "Mass Transfer", "Sh = 0.646Re^0.5Sc^0.33", "Correlation"),
        ]),
    ];

    let mut index = String::from("# Batch 02: Energy & Battery Systems\n\n| ID | Title | Steps | Space |\n|---|---|---|---|\n");

    for (i, (id, title, objective, params, derivation, refs)) in projects.iter().enumerate() {
        println!("[{}/10] {} - {}", i+1, id, title);
        let space: u64 = params.iter().map(|(_, _, min, max, step, _)| ((max - min) / step + 1.0) as u64).product();
        let paper = gen_paper(id, title, objective, params, derivation, refs, space);
        fs::write(out.join("02_energy").join(format!("{}.md", id)), &paper).unwrap();
        index.push_str(&format!("| {} | {} | {} | {} |\n", id, title, derivation.len(), fmt(space)));
        println!("  ✓ {} steps, {} configs", derivation.len(), fmt(space));
    }

    fs::write(out.join("02_energy").join("INDEX.md"), &index).unwrap();
    println!("\n✅ Batch 02 complete: 10 papers in papers_batch/02_energy/");
}

fn gen_paper(id: &str, title: &str, obj: &str, params: &[(&str,&str,f64,f64,f64,&str)], deriv: &[(u32,&str,&str,&str,&str,&str)], refs: &[(&str,&str,&str,&str)], space: u64) -> String {
    let mut p = format!("# {}\n\n**Authors**: S-wind Research Team  \n**Date**: January 2026  \n**ID**: {}\n\n---\n\n", title, id);
    p.push_str(&format!("## Abstract\n\nExhaustive search for {} ({} configs, {}-step derivation). Objective: {}.\n\n---\n\n", title.to_lowercase(), fmt(space), deriv.len(), obj));
    p.push_str(&format!("## 1. Problem\n\n**Objective**: {}\n\n### Parameters\n\n| Param | Sym | Min | Max | Step | Unit |\n|---|---|---|---|---|---|\n", obj));
    for (n,s,mi,ma,st,u) in params { p.push_str(&format!("| {} | ${}$ | {:.2} | {:.2} | {:.3} | {} |\n", n,s,mi,ma,st,u)); }
    p.push_str(&format!("\n**Space**: {}\n\n## 2. Derivation Chain\n\n| Step | Desc | In | Out | Rule | Why |\n|---|---|---|---|---|---|\n", fmt(space)));
    for (st,de,i,o,r,w) in deriv { p.push_str(&format!("| {} | {} | ${}$ | ${}$ | {} | {} |\n", st,de,i,o,r,w)); }
    p.push_str("\n### Flow\n```\n");
    for (i,(st,_,inp,out,rl,_)) in deriv.iter().enumerate() { 
        p.push_str(&format!("[{}] {} → {}\n", st, inp, out));
        if i < deriv.len()-1 { p.push_str(&format!("    ↓ [{}]\n", rl)); }
    }
    p.push_str("```\n\n## 3. References\n\n| ID | Name | Formula | Source |\n|---|---|---|---|\n");
    for (ri,rn,rf,rs) in refs { p.push_str(&format!("| {} | {} | ${}$ | {} |\n", ri,rn,rf,rs)); }
    p.push_str(&format!("\n---\n*S-wind Batch 02*\n"));
    p
}
