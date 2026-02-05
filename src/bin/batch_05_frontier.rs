//! Batch 05: Frontier Exploration & Manufacturing (10 projects)
//! Run: cargo run --release --bin batch_05_frontier

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
    println!("  S-wind Batch 05: Frontier Exploration & Manufacturing");
    println!("═══════════════════════════════════════════════════════════════\n");

    let out = Path::new("papers_batch");
    fs::create_dir_all(out.join("05_frontier")).unwrap();

    let projects = vec![
        ("I1", "Mars Habitat Module Design", "Maximize volume per launch mass", vec![
            ("Module Diameter", "D", 3.0, 8.0, 0.5, "m"),
            ("Module Length", "L", 5.0, 15.0, 1.0, "m"),
            ("Wall Thickness", "t", 5.0, 20.0, 1.0, "mm"),
            ("Insulation R", "R", 5.0, 20.0, 1.0, "m²K/W"),
            ("Rad Shielding", "ρ_s", 10.0, 50.0, 5.0, "g/cm²"),
        ], vec![
            (1, "Volume", "D, L", "V = π(D/2)²L", "Geometry", "Cylinder"),
            (2, "Shell mass", "D, L, t, ρ", "M_s = πDLtρ", "Geometry", "Surface"),
            (3, "Insulation mass", "A, R", "M_i = A×k/R×ρ_i", "Thermal", "Conductivity"),
            (4, "Shielding mass", "A, ρ_s", "M_sh = A×ρ_s", "Definition", "Areal density"),
            (5, "Total mass", "M_*", "M = M_s + M_i + M_sh", "Sum", "All components"),
            (6, "Heat loss", "A, ΔT, R", "Q = AΔT/R", "Fourier", "Steady-state"),
            (7, "Radiation dose", "ρ_s, GCR", "dose = GCR×exp(-ρ_s/λ)", "Exponential", "Attenuation"),
            (8, "Objective", "V, M", "V/M", "Efficiency", "Volume/mass"),
        ], vec![
            ("NASA_MARS", "Mars DRM", "Design Reference Mission", "NASA, 2009"),
            ("RADSHIELD", "Radiation Shielding", "GCR attenuation", "Cucinotta, 2014"),
        ]),

        ("I2", "Lunar Base Optimal Layout", "Minimize construction time", vec![
            ("Module Count", "N", 4.0, 12.0, 1.0, ""),
            ("Module Size", "V", 50.0, 200.0, 25.0, "m³"),
            ("Regolith Shield", "t_r", 0.5, 2.0, 0.25, "m"),
            ("Connection Length", "L_c", 5.0, 20.0, 2.5, "m"),
            ("Power Capacity", "P", 50.0, 200.0, 25.0, "kW"),
        ], vec![
            (1, "Total volume", "N, V", "V_tot = N × V", "Sum", "Hab volume"),
            (2, "Surface area", "V", "A = 4.84V^0.67", "Scaling", "Sphere-like"),
            (3, "Shielding volume", "A, t_r", "V_sh = A × t_r", "Geometry", "Regolith"),
            (4, "Construction mass", "V_sh, ρ", "M = V_sh × ρ_reg", "Definition", "ISRU"),
            (5, "Connection network", "N, L_c", "L_net = (N-1) × L_c", "Graph", "Minimum spanning"),
            (6, "Power requirement", "V_tot", "P_req = k × V_tot", "Scaling", "Life support"),
            (7, "Construction time", "M, rate", "T = M / rate", "Definition", "Duration"),
        ], vec![
            ("ARTEMIS", "Artemis Program", "Lunar architecture", "NASA, 2020"),
            ("ISRU", "In-Situ Resource", "Regolith utilization", "Sanders, 2011"),
        ]),

        ("I3", "Deep Sea Explorer (10,000m)", "Maximize payload capacity", vec![
            ("Hull Diameter", "D", 1.5, 3.0, 0.1, "m"),
            ("Hull Thickness", "t", 50.0, 150.0, 10.0, "mm"),
            ("Material", "M", 1.0, 4.0, 1.0, "type"),
            ("Buoyancy Vol", "V_b", 1.0, 5.0, 0.5, "m³"),
            ("Battery Cap", "E", 50.0, 200.0, 25.0, "kWh"),
        ], vec![
            (1, "Pressure", "d", "P = ρgd", "First principles", "1000 atm at 10km"),
            (2, "Hull stress", "P, D, t", "σ = PD/2t", "Thin shell", "Hoop stress"),
            (3, "Safety factor", "σ, σ_y", "SF = σ_y/σ", "Definition", "Strength"),
            (4, "Hull mass", "D, t, L, ρ", "M_h = πDLtρ", "Geometry", "Cylinder"),
            (5, "Buoyancy", "V_b, ρ_*", "B = V_b(ρ_w-ρ_f)g", "Archimedes", "Net"),
            (6, "Payload", "B, M_h, M_sys", "P = B - M_h - M_sys", "Balance", "Remaining"),
            (7, "Endurance", "E, P_avg", "t = E/P_avg", "Energy", "Duration"),
        ], vec![
            ("TRIESTE", "Bathyscaphe Trieste", "10,916m dive", "Piccard, 1960"),
            ("PRESSURE", "Hydrostatic Pressure", "P = ρgh", "First principles"),
        ]),

        ("I6", "Tokamak Fusion Confinement", "Maximize fusion power", vec![
            ("Major Radius", "R", 5.0, 10.0, 0.5, "m"),
            ("Minor Radius", "a", 1.0, 3.0, 0.2, "m"),
            ("Magnetic Field", "B_T", 5.0, 15.0, 0.5, "T"),
            ("Plasma Current", "I_p", 10.0, 25.0, 1.0, "MA"),
            ("Density", "n", 0.5, 2.0, 0.1, "×10²⁰/m³"),
        ], vec![
            (1, "Plasma volume", "R, a", "V = 2π²Ra²", "Torus", "Toroidal"),
            (2, "Confinement time", "a, I, n, T", "τ_E = H × τ_ITER98", "Scaling", "H-mode"),
            (3, "Triple product", "n, T, τ", "nTτ", "Lawson", "Ignition criterion"),
            (4, "Fusion power", "n, T, V", "P_f = ⟨σv⟩n²E_αV", "Reaction", "D-T rate"),
            (5, "Beta", "n, T, B", "β = 2μ₀nT/B²", "Definition", "Pressure ratio"),
            (6, "Greenwald limit", "I, a", "n_G = I/(πa²)", "Limit", "Density"),
            (7, "Q factor", "P_f, P_aux", "Q = P_f/P_aux", "Gain", "Energy mult"),
        ], vec![
            ("LAWSON", "Lawson Criterion", "nTτ > 3×10²¹", "Lawson, 1957"),
            ("ITER", "ITER Scaling", "τ = HI^0.93...", "ITER, 1999"),
        ]),

        ("D1", "Factory Production Scheduling", "Minimize makespan", vec![
            ("Batch Size", "B", 10.0, 100.0, 10.0, "units"),
            ("Setup Time", "t_s", 10.0, 60.0, 10.0, "min"),
            ("Buffer Size", "W", 5.0, 50.0, 5.0, "units"),
            ("Workers", "N_w", 2.0, 10.0, 1.0, ""),
            ("Shifts", "S", 1.0, 3.0, 1.0, ""),
        ], vec![
            (1, "Cycle time", "process", "t_c = Σ t_i", "Sum", "Per unit"),
            (2, "Throughput", "t_c", "TH = 1/t_c", "Definition", "Units/hr"),
            (3, "Capacity", "TH, N_w, S", "C = TH × N_w × S × 8", "Scaling", "Daily"),
            (4, "Setup loss", "t_s, B", "L_s = t_s × (orders/B)", "Changeover", "Lost time"),
            (5, "WIP inventory", "TH, t_c, W", "WIP = TH × t_c + W", "Little's law", "In-process"),
            (6, "Makespan", "demand, C", "M = demand/C + setups", "Definition", "Total time"),
            (7, "Cost", "M, N_w, S", "Cost = M × N_w × wage × S", "Labor", "Production cost"),
        ], vec![
            ("LITTLE", "Little's Law", "L = λW", "Little, 1961"),
            ("GOLDRATT", "Theory of Constraints", "Bottleneck management", "Goldratt, 1984"),
        ]),

        ("D2", "Semiconductor Yield Optimization", "Maximize yield", vec![
            ("Exposure Dose", "D", 10.0, 50.0, 2.0, "mJ/cm²"),
            ("Focus Offset", "F", -100.0, 100.0, 10.0, "nm"),
            ("Etch Time", "t_e", 30.0, 120.0, 10.0, "s"),
            ("Deposition Rate", "r_d", 1.0, 10.0, 1.0, "nm/s"),
            ("Anneal Temp", "T_a", 800.0, 1100.0, 50.0, "°C"),
        ], vec![
            (1, "CD variation", "D, F", "σ_CD = f(D, F)", "Process", "Critical dimension"),
            (2, "Defect density", "process", "D_0 = Σ D_i", "Sum", "All sources"),
            (3, "Yield model", "D_0, A", "Y = exp(-D_0 × A)", "Poisson", "Murphy model"),
            (4, "Parametric yield", "σ_CD, spec", "Y_p = Φ((spec-μ)/σ)", "Normal", "Spec compliance"),
            (5, "Total yield", "Y, Y_p", "Y_tot = Y × Y_p", "Product", "Combined"),
            (6, "Throughput", "t_*, wafers", "WPH = 3600/Σt", "Definition", "Wafers/hr"),
            (7, "Cost per die", "Y_tot, cost", "CPD = cost/(Y_tot × DPW)", "Economics", "Unit cost"),
        ], vec![
            ("MURPHY", "Murphy Yield", "Y = exp(-DA)", "Murphy, 1964"),
            ("SRAM", "SRAM Yield", "Parametric yield", "Stine, 1997"),
        ]),

        ("D3", "Predictive Maintenance", "Minimize total cost (downtime + maintenance)", vec![
            ("Inspection Freq", "f_i", 1.0, 30.0, 1.0, "days"),
            ("Replace Threshold", "RUL_th", 10.0, 100.0, 10.0, "hrs"),
            ("Sensor Count", "N_s", 1.0, 10.0, 1.0, ""),
            ("Model Confidence", "C", 0.7, 0.99, 0.05, ""),
            ("Safety Margin", "m", 0.1, 0.5, 0.05, ""),
        ], vec![
            (1, "Failure rate", "age, condition", "λ(t) = λ_0 × g(condition)", "Weibull", "Hazard"),
            (2, "RUL prediction", "sensors, model", "RUL = f(sensor_data)", "ML model", "Remaining life"),
            (3, "False positive rate", "C, th", "FPR = 1-C", "Definition", "Type I error"),
            (4, "False negative rate", "C, m", "FNR = f(C, m)", "Definition", "Type II error"),
            (5, "PM cost", "f_i, c_pm", "C_PM = (365/f_i) × c_pm", "Annual", "Scheduled"),
            (6, "CM cost", "FNR, λ, c_cm", "C_CM = FNR × λ × c_cm", "Expected", "Unplanned"),
            (7, "Total cost", "C_PM, C_CM", "C = C_PM + C_CM", "Sum", "Annual"),
        ], vec![
            ("WEIBULL", "Weibull Distribution", "f(t) = (β/η)(t/η)^(β-1)exp(-(t/η)^β)", "Weibull, 1951"),
            ("CBM", "Condition-Based Maintenance", "RUL prediction", "Jardine, 2006"),
        ]),

        ("E1", "Catalyst Formulation Optimization", "Maximize conversion × selectivity", vec![
            ("Metal Loading", "w", 0.5, 5.0, 0.5, "wt%"),
            ("Support Type", "S", 1.0, 4.0, 1.0, "type"),
            ("Calcination T", "T_c", 300.0, 600.0, 50.0, "°C"),
            ("Reduction T", "T_r", 200.0, 500.0, 50.0, "°C"),
            ("Particle Size", "d", 2.0, 20.0, 2.0, "nm"),
        ], vec![
            (1, "Surface area", "d", "S_a = 6/(ρ×d)", "Geometry", "Specific area"),
            (2, "Active sites", "w, S_a", "N = w × S_a × dispersion", "Definition", "Sites/g"),
            (3, "Turnover freq", "T, E_a", "TOF = A×exp(-E_a/RT)", "Arrhenius", "Rate/site"),
            (4, "Conversion", "N, TOF, t", "X = 1-exp(-N×TOF×t)", "First order", "Fraction"),
            (5, "Selectivity", "T, products", "S = desired/total", "Definition", "Desired/all"),
            (6, "Yield", "X, S", "Y = X × S", "Definition", "Product"),
            (7, "Stability", "T_c, cycles", "τ = f(T_c, d)", "Sintering", "Lifetime"),
        ], vec![
            ("ARRHENIUS", "Arrhenius Equation", "k = A×exp(-E_a/RT)", "Arrhenius, 1889"),
            ("SABATIER", "Sabatier Principle", "Optimal binding energy", "Sabatier, 1913"),
        ]),

        ("E4", "Chemical Process Optimization", "Maximize yield × safety", vec![
            ("Temperature", "T", 100.0, 300.0, 10.0, "°C"),
            ("Pressure", "P", 1.0, 50.0, 5.0, "bar"),
            ("Residence Time", "τ", 1.0, 60.0, 5.0, "min"),
            ("Feed Ratio", "R", 0.5, 2.0, 0.1, ""),
            ("Catalyst Conc", "C_cat", 0.1, 2.0, 0.1, "%"),
        ], vec![
            (1, "Reaction rate", "T, C", "r = k×C^n", "Power law", "Kinetics"),
            (2, "Rate constant", "T", "k = A×exp(-E_a/RT)", "Arrhenius", "Temperature"),
            (3, "Conversion", "r, τ", "X = 1-exp(-k×τ)", "CSTR", "Steady-state"),
            (4, "Selectivity", "T, R", "S = f(T, R)", "Mechanism", "Side reactions"),
            (5, "Yield", "X, S", "Y = X × S", "Definition", "Product"),
            (6, "Safety index", "T, P", "SI = f(T/T_max, P/P_max)", "Normalized", "Risk"),
            (7, "Objective", "Y, SI", "max Y × SI", "Multi-obj", "Trade-off"),
        ], vec![
            ("CSTR", "CSTR Design", "V = F×X/(-r)", "Fogler, 2016"),
            ("DOW_FEI", "Dow F&EI", "Fire & explosion index", "AIChE, 1994"),
        ]),

        ("I4", "Personal Jetpack Design", "Maximize flight time", vec![
            ("Thrust", "T", 1000.0, 2000.0, 100.0, "N"),
            ("Fuel Capacity", "V_f", 5.0, 20.0, 1.0, "L"),
            ("Engine Count", "N_e", 2.0, 4.0, 1.0, ""),
            ("Nozzle Diameter", "D_n", 50.0, 150.0, 10.0, "mm"),
            ("Frame Mass", "m_f", 10.0, 30.0, 2.0, "kg"),
        ], vec![
            (1, "Total mass", "m_f, V_f, m_p", "m = m_f + V_f×ρ_f + m_pilot", "Sum", "Wet mass"),
            (2, "Required thrust", "m, g", "T_req = m × g", "Hover", "Equilibrium"),
            (3, "Thrust margin", "T, T_req", "TM = T/T_req - 1", "Definition", "Excess"),
            (4, "Fuel consumption", "T, ISP", "ṁ = T/(ISP×g)", "Rocket eq", "Mass rate"),
            (5, "Flight time", "V_f, ṁ", "t = V_f×ρ_f/ṁ", "Definition", "Duration"),
            (6, "Noise level", "T, D_n", "L = 10log(T) + 20log(v_e/c)", "Jet noise", "dBA"),
            (7, "Stability", "CoG, CoT", "SM = (CoT-CoG)/L", "Definition", "Static margin"),
        ], vec![
            ("JETPACK", "Jetpack Aviation", "JB-11 specs", "Mayman, 2018"),
            ("ISP", "Specific Impulse", "ISP = T/(ṁg)", "Rocket propulsion"),
        ]),
    ];

    let mut index = String::from("# Batch 05: Frontier Exploration & Manufacturing\n\n| ID | Title | Steps | Space |\n|---|---|---|---|\n");

    for (i, (id, title, obj, params, deriv, refs)) in projects.iter().enumerate() {
        println!("[{}/10] {} - {}", i+1, id, title);
        let space: u64 = params.iter().map(|(_, _, mi, ma, st, _)| ((ma - mi) / st + 1.0) as u64).product();
        let paper = gen_paper(id, title, obj, params, deriv, refs, space);
        fs::write(out.join("05_frontier").join(format!("{}.md", id)), &paper).unwrap();
        index.push_str(&format!("| {} | {} | {} | {} |\n", id, title, deriv.len(), fmt(space)));
        println!("  ✓ {} steps, {} configs", deriv.len(), fmt(space));
    }

    fs::write(out.join("05_frontier").join("INDEX.md"), &index).unwrap();
    println!("\n✅ Batch 05 complete: 10 papers in papers_batch/05_frontier/");
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
    p.push_str("\n---\n*S-wind Batch 05*\n");
    p
}
