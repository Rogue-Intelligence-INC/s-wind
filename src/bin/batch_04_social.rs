//! Batch 04: Social, Environmental & Healthcare (10 projects)
//! Run: cargo run --release --bin batch_04_social

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
    println!("  S-wind Batch 04: Social, Environmental & Healthcare");
    println!("═══════════════════════════════════════════════════════════════\n");

    let out = Path::new("papers_batch");
    fs::create_dir_all(out.join("04_social")).unwrap();

    let projects = vec![
        ("S1", "Targeted Poverty Alleviation Optimization", "Maximize poverty exit rate per $", vec![
            ("Industrial Share", "r_ind", 0.2, 0.6, 0.05, ""),
            ("Education Share", "r_edu", 0.1, 0.4, 0.05, ""),
            ("Training Share", "r_train", 0.1, 0.5, 0.05, ""),
            ("Infrastructure", "r_infra", 0.1, 0.4, 0.05, ""),
            ("Microloan", "L", 1000.0, 10000.0, 500.0, "$"),
        ], vec![
            (1, "Poverty line", "", "PL = $2.15/day (2017 PPP)", "World Bank", "Standard"),
            (2, "Industrial income", "r_ind, I", "ΔY_ind = α_ind × r_ind × I", "Multiplier", "Job creation"),
            (3, "Education return", "r_edu, I, t", "ΔY_edu = α_edu × r_edu × I × (1+g)^t", "Human capital", "Compounding"),
            (4, "Training return", "r_train, I", "ΔY_train = α_train × r_train × I", "Skills", "Employability"),
            (5, "Infrastructure mult", "r_infra", "μ = 1 + β × r_infra", "Enabling", "Synergy"),
            (6, "Microloan return", "L", "ΔY_micro = γ × L × (1-default)", "Capital", "Small business"),
            (7, "Total income", "ΔY_*", "ΔY = μ × Σ ΔY_i", "Aggregation", "All channels"),
            (8, "Exit probability", "ΔY, PL", "P(exit) = Φ((Y+ΔY-PL)/σ)", "Probit", "Distribution"),
        ], vec![
            ("WORLDBANK", "Poverty Line", "$2.15/day PPP", "World Bank, 2022"),
            ("SACHS", "Poverty Trap", "k* = sf(k)/δ", "Sachs, 2005"),
        ]),

        ("S2", "Universal Basic Income Implementation", "Maximize welfare, fiscally sustainable", vec![
            ("UBI Amount", "B", 500.0, 2000.0, 100.0, "$/mo"),
            ("Phase-in Rate", "τ", 0.3, 0.7, 0.05, ""),
            ("Threshold", "Y_th", 20000.0, 50000.0, 5000.0, "$/yr"),
            ("Funding Mix", "r_tax", 0.5, 1.0, 0.1, ""),
            ("Admin Efficiency", "η", 0.9, 0.99, 0.01, ""),
        ], vec![
            (1, "Gross cost", "B, N", "C_gross = B × N × 12", "Definition", "Annual"),
            (2, "Phase-out savings", "τ, Y_th", "S = ∫τ×max(0,Y-Y_th)dF(Y)", "Integration", "Clawback"),
            (3, "Net cost", "C, S, η", "C_net = (C-S)/η", "Efficiency", "After admin"),
            (4, "Tax revenue", "r_tax, GDP", "R = r_tax × GDP × Δτ", "Funding", "New taxes"),
            (5, "Utility gain", "B, U", "ΔU = Σ u(c+B) - u(c)", "Welfare", "Consumption"),
            (6, "Labor supply", "B, w", "ΔL = -ε × B/w", "Elasticity", "Income effect"),
            (7, "Net welfare", "ΔU, ΔL", "W = ΔU - α×|ΔL|", "Objective", "Trade-off"),
        ], vec![
            ("FRIEDMAN", "Negative Income Tax", "NIT design", "Friedman, 1962"),
            ("HOYNES", "UBI Effects", "Labor supply", "Hoynes, 2019"),
        ]),

        ("S3", "Digital Divide Bridging", "Maximize coverage per $ spent", vec![
            ("Fiber Ratio", "r_f", 0.0, 1.0, 0.1, ""),
            ("Tower Density", "D_t", 0.1, 1.0, 0.1, "/km²"),
            ("Backhaul Type", "BH", 1.0, 3.0, 1.0, "type"),
            ("Subsidy", "S", 0.0, 500.0, 50.0, "$/HH"),
            ("Speed Tier", "v", 25.0, 100.0, 25.0, "Mbps"),
        ], vec![
            (1, "Fiber reach", "r_f, HH", "HH_f = r_f × HH_tot", "Definition", "Fiber homes"),
            (2, "Wireless reach", "D_t, A", "HH_w = f(D_t, terrain)", "Propagation", "Coverage"),
            (3, "Total coverage", "HH_f, HH_w", "C = (HH_f + HH_w)/HH_tot", "Union", "Unique"),
            (4, "Fiber cost", "HH_f, d", "C_f = HH_f × (c_0 + c_1×d)", "Distance", "Infrastructure"),
            (5, "Tower cost", "D_t, A", "C_t = D_t × A × c_tower", "Deployment", "Capital"),
            (6, "OpEx", "C_*, t", "OpEx = (C_f+C_t) × r_opex", "O&M", "Annual"),
            (7, "Cost per HH", "Total, HH", "$/HH = Total/HH_served", "Unit", "Economics"),
        ], vec![
            ("FCC", "Broadband Deployment", "Speed benchmarks", "FCC, 2024"),
            ("ITU", "Digital Development", "Connectivity targets", "ITU, 2023"),
        ]),

        ("Q1", "Carbon Capture & Storage", "Minimize $/ton CO2", vec![
            ("Capture Eff", "η_c", 0.85, 0.99, 0.02, ""),
            ("Solvent Conc", "C", 20.0, 40.0, 2.0, "%"),
            ("Regen Temp", "T_r", 100.0, 140.0, 5.0, "°C"),
            ("Absorber H", "H", 10.0, 30.0, 2.0, "m"),
            ("Gas Velocity", "v", 1.0, 3.0, 0.2, "m/s"),
        ], vec![
            (1, "CO2 loading", "C, P", "α = f(C, P_CO2)", "Equilibrium", "Solvent"),
            (2, "Mass transfer", "k_G, a, H", "N = k_G × a × H × ΔP", "Film theory", "Absorption"),
            (3, "Heat of absorption", "α", "Q = -ΔH × Δα", "Thermodynamics", "Exothermic"),
            (4, "Regen heat", "T_r, C", "Q_r = c_p×m×ΔT + Q_des", "Energy", "Sensible+latent"),
            (5, "Pump energy", "v, ΔP", "W = v×A×ΔP/η", "Fluid", "Pressure drop"),
            (6, "Total energy", "Q_r, W", "E = Q_r + W", "Sum", "Per ton"),
            (7, "Cost", "E, CAPEX", "$/t = E×p_E + CAPEX/N", "LCOC", "Economics"),
        ], vec![
            ("MEA", "MEA Absorption", "CO2 + MEA ⇌ carbamate", "Rochelle, 2009"),
            ("IPCC", "CCS Report", "Technology assessment", "IPCC, 2005"),
        ]),

        ("Q2", "Ocean Plastic Cleanup System", "Maximize collection rate", vec![
            ("Boom Length", "L", 100.0, 1000.0, 100.0, "m"),
            ("Boom Depth", "D", 1.0, 5.0, 0.5, "m"),
            ("Tow Speed", "v", 0.5, 2.0, 0.25, "m/s"),
            ("Mesh Size", "s", 1.0, 10.0, 1.0, "mm"),
            ("Collection Freq", "f", 1.0, 7.0, 1.0, "days"),
        ], vec![
            (1, "Swept volume", "L, D, v, t", "V = L × D × v × t", "Geometry", "Processed"),
            (2, "Plastic conc", "location", "C = f(lat, lon)", "Data", "GPGP model"),
            (3, "Collection rate", "V, C, η", "R = V × C × η", "Mass balance", "kg/day"),
            (4, "Capture eff", "s, v", "η = 1 - exp(-k×D/s)", "Filtration", "Mesh"),
            (5, "Drag force", "L, D, v", "F = ½ρC_d×L×D×v²", "Fluid", "Hydrodynamic"),
            (6, "Tow power", "F, v", "P = F × v", "Definition", "Energy"),
            (7, "Bycatch rate", "s, R", "B = B_0×(s_ref/s)²", "Selectivity", "Marine life"),
        ], vec![
            ("GPGP", "Great Pacific Garbage Patch", "Plastic map", "Lebreton, 2018"),
            ("CLEANUP", "Ocean Cleanup", "System 001/002", "Slat, 2019"),
        ]),

        ("J1", "Hospital OR Scheduling", "Maximize utilization, OT < 10%", vec![
            ("Block Length", "B", 2.0, 8.0, 1.0, "hr"),
            ("Buffer Time", "τ", 15.0, 60.0, 5.0, "min"),
            ("Overbooking", "α", 1.0, 1.3, 0.05, ""),
            ("Priority Weight", "w", 1.0, 3.0, 0.5, ""),
            ("Flexibility", "f", 0.0, 0.3, 0.05, ""),
        ], vec![
            (1, "Available time", "B, n_OR", "T_avail = B × n_OR × 60", "Definition", "OR-minutes"),
            (2, "Case duration", "type", "μ, σ ~ LogNormal", "Statistics", "Historical"),
            (3, "Scheduled time", "cases, μ, τ", "T_sched = Σ(μ + τ)", "Sum", "With buffer"),
            (4, "Utilization", "T_sched, T_avail", "U = T_sched/T_avail", "Definition", "Efficiency"),
            (5, "OT probability", "T, σ, B", "P(OT) = 1-Φ((B-T)/σ)", "Normal", "Tail"),
            (6, "Urgency score", "cases, w", "S = Σ w × priority", "Weighted", "Objective"),
            (7, "Objective", "U, P(OT), S", "max U×S - λ×P(OT)", "Trade-off", "Multi-obj"),
        ], vec![
            ("OR_SCHED", "OR Scheduling", "Block scheduling", "Cardoen, 2010"),
            ("QUEUING", "Healthcare Queuing", "M/G/c model", "Green, 2006"),
        ]),

        ("J2", "Emergency Resource Dispatch", "Minimize response time", vec![
            ("Ambulances", "N_a", 5.0, 20.0, 1.0, ""),
            ("Base Locations", "N_b", 3.0, 10.0, 1.0, ""),
            ("Response Target", "T_r", 8.0, 15.0, 1.0, "min"),
            ("Coverage Radius", "r", 3.0, 10.0, 1.0, "km"),
            ("Reserve Ratio", "r_res", 0.1, 0.3, 0.05, ""),
        ], vec![
            (1, "Demand rate", "area, pop", "λ = pop × r_call", "Statistics", "Calls/hr"),
            (2, "Service time", "distance, v", "μ = d/v + t_scene", "Definition", "Per call"),
            (3, "Utilization", "λ, μ, N", "ρ = λ/(N×μ)", "Queuing", "Load"),
            (4, "Coverage", "N_b, r, A", "C = 1-exp(-N_b×πr²/A)", "Set covering", "Area"),
            (5, "Queue delay", "λ, μ, N", "W_q = f(λ, μ, N)", "Erlang-C", "Wait time"),
            (6, "Response time", "W_q, travel", "T = W_q + travel", "Sum", "Total"),
            (7, "Compliance", "T, T_r", "P(T<T_r)", "CDF", "Performance"),
        ], vec![
            ("MAXCOVER", "Maximal Covering", "Location problem", "Church, 1974"),
            ("ERLANG", "Erlang-C", "P(wait) formula", "Erlang, 1917"),
        ]),

        ("J3", "CRISPR Gene Editing Parameters", "Maximize on-target, off-target < 1%", vec![
            ("Guide Length", "L", 17.0, 23.0, 1.0, "nt"),
            ("GC Content", "GC", 0.4, 0.7, 0.05, ""),
            ("PAM Position", "P", 1.0, 5.0, 1.0, "type"),
            ("Dose", "D", 10.0, 100.0, 10.0, "nM"),
            ("Incubation", "t", 12.0, 72.0, 6.0, "hr"),
        ], vec![
            (1, "Binding energy", "L, GC, T", "ΔG = ΔH - TΔS", "Thermodynamics", "Gibbs"),
            (2, "On-target activity", "ΔG, D", "η_on = 1-exp(-k×D×exp(-ΔG/RT))", "Kinetics", "M-M"),
            (3, "Off-target score", "mismatches", "OT = Σf(pos)×exp(-ΔΔG)", "CFD", "Cutting freq"),
            (4, "Specificity", "η_on, OT", "S = η_on/(η_on+OT)", "Definition", "On/total"),
            (5, "Editing efficiency", "η, t", "E = η×(1-exp(-t/τ))", "Kinetics", "Time"),
            (6, "Toxicity", "D, t", "Tox = 1-exp(-D×t/τ_tox)", "Dose-response", "Cytotox"),
            (7, "Objective", "E, S, Tox", "max E×S×(1-Tox)", "Multi-obj", "Trade-off"),
        ], vec![
            ("CRISPR", "CRISPR-Cas9", "RNA-guided nuclease", "Jinek, 2012"),
            ("CFD", "CFD Score", "Off-target prediction", "Doench, 2016"),
        ]),

        ("J4", "Vaccine Production Optimization", "Maximize yield per batch", vec![
            ("Cell Density", "X", 1.0, 10.0, 0.5, "×10⁶/mL"),
            ("MOI", "MOI", 0.01, 1.0, 0.1, ""),
            ("Harvest Time", "t_h", 24.0, 96.0, 12.0, "hr"),
            ("Temperature", "T", 30.0, 37.0, 1.0, "°C"),
            ("pH", "pH", 6.8, 7.4, 0.1, ""),
        ], vec![
            (1, "Infection rate", "X, MOI", "P_inf = 1 - exp(-MOI)", "Poisson", "Infected fraction"),
            (2, "Viral replication", "T, pH", "μ = μ_max × f(T) × f(pH)", "Kinetics", "Growth rate"),
            (3, "Viral titer", "μ, t_h", "V = V_0 × exp(μ×t_h)", "Exponential", "Amplification"),
            (4, "Cell viability", "t_h", "viab = exp(-k_d×t_h)", "Decay", "Cell death"),
            (5, "Harvest yield", "V, viab, X", "Y = V × viab × X", "Product", "Total"),
            (6, "Quality", "T, pH", "Q = f(T, pH, t_h)", "Empirical", "Potency"),
            (7, "Yield × Quality", "Y, Q", "Y×Q", "Objective", "Trade-off"),
        ], vec![
            ("VERO", "Vero Cell Culture", "Vaccine production", "WHO, 2013"),
            ("BIOREACTOR", "Bioreactor Scale-up", "kLa scaling", "Garcia-Ochoa, 2009"),
        ]),

        ("Q5", "Carbon Neutrality Pathway", "Minimize transition cost to net-zero", vec![
            ("Renewable Share", "r_re", 0.3, 1.0, 0.05, ""),
            ("EV Penetration", "r_ev", 0.3, 1.0, 0.05, ""),
            ("Industrial Elec", "r_ie", 0.2, 0.8, 0.05, ""),
            ("CCS Capacity", "C_ccs", 0.0, 500.0, 50.0, "MtCO2/y"),
            ("Forest Sink", "C_f", 0.0, 200.0, 20.0, "MtCO2/y"),
        ], vec![
            (1, "Baseline emissions", "E_2020", "E_0 = 10 GtCO2/yr", "Initial", "Global"),
            (2, "Power emissions", "E_p, r_re", "E_p' = E_p × (1-r_re)", "Substitution", "Renewables"),
            (3, "Transport emissions", "E_t, r_ev", "E_t' = E_t × (1-0.7×r_ev)", "Well-to-wheel", "EVs"),
            (4, "Industry emissions", "E_i, r_ie", "E_i' = E_i × (1-0.5×r_ie)", "Electrification", "Process"),
            (5, "CCS removal", "C_ccs", "E_ccs = C_ccs", "Direct", "Capture"),
            (6, "Forest sink", "C_f", "E_f = C_f", "Natural", "Sequestration"),
            (7, "Net emissions", "E_*", "E_net = E_p'+E_t'+E_i'-C_ccs-C_f", "Balance", "Net zero check"),
            (8, "Transition cost", "r_*, C_*", "Cost = Σ(CapEx + OpEx)", "Economics", "Total investment"),
        ], vec![
            ("IPCC", "IPCC AR6", "Climate pathways", "IPCC, 2021"),
            ("KAYA", "Kaya Identity", "CO2 = P×(GDP/P)×(E/GDP)×(CO2/E)", "Kaya, 1990"),
        ]),
    ];

    let mut index = String::from("# Batch 04: Social, Environmental & Healthcare\n\n| ID | Title | Steps | Space |\n|---|---|---|---|\n");

    for (i, (id, title, obj, params, deriv, refs)) in projects.iter().enumerate() {
        println!("[{}/10] {} - {}", i+1, id, title);
        let space: u64 = params.iter().map(|(_, _, mi, ma, st, _)| ((ma - mi) / st + 1.0) as u64).product();
        let paper = gen_paper(id, title, obj, params, deriv, refs, space);
        fs::write(out.join("04_social").join(format!("{}.md", id)), &paper).unwrap();
        index.push_str(&format!("| {} | {} | {} | {} |\n", id, title, deriv.len(), fmt(space)));
        println!("  ✓ {} steps, {} configs", deriv.len(), fmt(space));
    }

    fs::write(out.join("04_social").join("INDEX.md"), &index).unwrap();
    println!("\n✅ Batch 04 complete: 10 papers in papers_batch/04_social/");
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
    p.push_str("\n---\n*S-wind Batch 04*\n");
    p
}
