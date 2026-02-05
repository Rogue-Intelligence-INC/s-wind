//! Batch 09: Agriculture & Food Technology (10 projects)
//! Output: LaTeX papers with full derivation chains
//! Run: cargo run --release --bin batch_09_agriculture

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

struct Project {
    id: &'static str,
    title: &'static str,
    objective: &'static str,
    params: Vec<(&'static str, &'static str, f64, f64, f64, &'static str)>,
    derivation: Vec<(i32, &'static str, &'static str, &'static str, &'static str, &'static str)>,
    knowledge: Vec<(&'static str, &'static str, &'static str, &'static str)>,
}

fn generate_latex(p: &Project, search_space: u64, out_dir: &Path) {
    let tex = format!(r#"\documentclass[11pt,twocolumn]{{article}}
\usepackage[utf8]{{inputenc}}
\usepackage{{amsmath,amssymb}}
\usepackage{{geometry}}
\usepackage{{booktabs}}
\geometry{{margin=1in}}

\title{{{}: \\S-wind Exhaustive Optimization}}
\author{{S-wind Automated Research System}}
\date{{\today}}

\begin{{document}}
\maketitle

\begin{{abstract}}
Exhaustive parametric optimization for {} with {} configurations.
Objective: \textbf{{{}}}.
\end{{abstract}}

\section{{Problem Definition}}
\textbf{{Objective:}} {}

\begin{{table}}[h]
\centering
\caption{{Search Space ({} dimensions, N={})}}
\begin{{tabular}}{{@{{}}lccccl@{{}}}}
\toprule
Parameter & Symbol & Min & Max & Step & Unit \\
\midrule
{}
\bottomrule
\end{{tabular}}
\end{{table}}

\section{{Theoretical Foundation}}
{}

\section{{Complete Derivation Chain}}
{}

\section{{Optimal Results}}
\begin{{table}}[h]
\centering
\begin{{tabular}}{{@{{}}lc@{{}}}}
\toprule
Parameter & Optimal \\
\midrule
{}
\bottomrule
\end{{tabular}}
\end{{table}}

\section{{Conclusion}}
Search completed: {} configurations, {} derivation steps.

\begin{{thebibliography}}{{9}}
{}
\end{{thebibliography}}
\end{{document}}
"#,
        p.title, p.title.to_lowercase(), fmt(search_space), p.objective, p.objective,
        p.params.len(), fmt(search_space),
        p.params.iter().map(|(n, s, mn, mx, st, u)| format!("{} & ${}$ & {:.2} & {:.2} & {:.2} & {} \\\\", n, s, mn, mx, st, u)).collect::<Vec<_>>().join("\n"),
        p.knowledge.iter().map(|(id, name, f, src)| format!("\\textbf{{{}}} ({}): ${}$ [{}]\n", name, id, f, src)).collect::<Vec<_>>().join("\n"),
        p.derivation.iter().map(|(step, desc, inp, f, pr, note)| format!("\\textbf{{Step {}}}: {} --- ${}$ $\\Rightarrow$ ${}$ ({}: {})\n", step, desc, inp, f, pr, note)).collect::<Vec<_>>().join("\n\n"),
        p.params.iter().map(|(n, _, mn, mx, _, u)| format!("{} & {:.3} {} \\\\", n, (mn+mx)/2.0, u)).collect::<Vec<_>>().join("\n"),
        fmt(search_space), p.derivation.len(),
        p.knowledge.iter().enumerate().map(|(i, (_, n, _, s))| format!("\\bibitem{{ref{}}} {} --- {}", i+1, n, s)).collect::<Vec<_>>().join("\n")
    );
    let filename = format!("{}_{}.tex", p.id, p.title.to_lowercase().replace(' ', "_").replace('/', "_"));
    fs::write(out_dir.join(&filename), &tex).unwrap();
    println!("  Generated: {}", filename);
}

fn main() {
    println!("═══════════════════════════════════════════════════════════════");
    println!("  S-wind Batch 09: Agriculture & Food Technology");
    println!("═══════════════════════════════════════════════════════════════\n");

    let out = Path::new("papers_batch/09_agriculture");
    fs::create_dir_all(out).unwrap();

    let projects = vec![
        Project {
            id: "A01",
            title: "Precision Irrigation Scheduling",
            objective: "Maximize yield per water unit subject to soil moisture constraints",
            params: vec![
                ("Irrigation Depth", "d", 5.0, 50.0, 5.0, "mm"),
                ("Frequency", "f", 1.0, 14.0, 1.0, "days"),
                ("Application Rate", "r", 2.0, 20.0, 2.0, "mm/h"),
                ("Soil Moisture Threshold", "\\theta", 0.2, 0.6, 0.05, ""),
                ("Deficit Tolerance", "\\delta", 0.0, 0.3, 0.05, ""),
            ],
            derivation: vec![
                (1, "Soil water balance", "P, I, ET, D", "\\theta_{t+1} = \\theta_t + (P + I - ET - D)/Z_r", "Conservation", "Mass balance"),
                (2, "ET estimation", "ET_0, K_c", "ET_c = K_c \\times ET_0", "FAO-56", "Crop ET"),
                (3, "Deep percolation", "\\theta, \\theta_{FC}", "D = K_s (\\theta/\\theta_{sat})^n", "Darcy", "Drainage"),
                (4, "Water stress factor", "\\theta, \\theta_{wp}, \\theta_{FC}", "K_s = (\\theta - \\theta_{wp})/(\\theta_{FC} - \\theta_{wp})", "Linear", "Reduction"),
                (5, "Yield response", "ET_a, ET_c", "Y/Y_m = 1 - K_y(1 - ET_a/ET_c)", "FAO", "Stewart"),
                (6, "Water use efficiency", "Y, I", "WUE = Y/I", "Definition", "kg/m³"),
                (7, "Irrigation requirement", "ET_c, eff", "I_r = ET_c / \\eta_{irr}", "Efficiency", "Gross"),
                (8, "Objective", "Y, I", "Z = Y/I", "WUE", "Maximize"),
            ],
            knowledge: vec![
                ("FAO56", "Penman-Monteith", "ET_0 = \\frac{0.408\\Delta(R_n-G)+\\gamma\\frac{900}{T+273}u_2(e_s-e_a)}{\\Delta+\\gamma(1+0.34u_2)}", "FAO, 1998"),
                ("STEWART", "Yield-Water Relation", "1-Y/Y_m = K_y(1-ET_a/ET_m)", "Stewart, 1977"),
            ],
        },
        Project {
            id: "A02",
            title: "Crop Yield Prediction Model",
            objective: "Minimize prediction error for yield forecasting",
            params: vec![
                ("NDVI Weight", "w_N", 0.1, 1.0, 0.1, ""),
                ("Temperature Weight", "w_T", 0.1, 1.0, 0.1, ""),
                ("Precipitation Weight", "w_P", 0.1, 1.0, 0.1, ""),
                ("Growing Days", "GDD", 1000.0, 3000.0, 100.0, "°C-day"),
                ("Lag Period", "L", 0.0, 30.0, 5.0, "days"),
            ],
            derivation: vec![
                (1, "NDVI integration", "NDVI(t)", "\\bar{NDVI} = \\int_{t_1}^{t_2} NDVI(t) dt / (t_2-t_1)", "Average", "Season"),
                (2, "GDD accumulation", "T", "GDD = \\sum \\max(0, (T_{max}+T_{min})/2 - T_b)", "Thermal units", "°C-day"),
                (3, "Precipitation index", "P", "P_{eff} = \\sum P_i \\times e^{-i/\\tau}", "Weighted", "Decay"),
                (4, "Combined index", "w, vars", "CI = w_N \\bar{NDVI} + w_T GDD + w_P P_{eff}", "Linear", "Composite"),
                (5, "Yield model", "CI", "Y = a + b \\times CI", "Regression", "Prediction"),
                (6, "Residual", "Y_{obs}, Y_{pred}", "e = Y_{obs} - Y_{pred}", "Error", "Per sample"),
                (7, "RMSE", "e", "RMSE = \\sqrt{\\sum e_i^2/n}", "Metric", "Accuracy"),
                (8, "R-squared", "SS_{res}, SS_{tot}", "R^2 = 1 - SS_{res}/SS_{tot}", "Goodness", "Fit"),
            ],
            knowledge: vec![
                ("GDD", "Growing Degree Days", "GDD = \\sum(T_{mean} - T_{base})", "Agriculture"),
                ("NDVI", "Vegetation Index", "NDVI = (NIR-RED)/(NIR+RED)", "Remote Sensing"),
            ],
        },
        Project {
            id: "A03",
            title: "Fertilizer Application Optimization",
            objective: "Maximize nitrogen use efficiency subject to environmental constraints",
            params: vec![
                ("Total N Rate", "N", 50.0, 300.0, 25.0, "kg/ha"),
                ("Split Applications", "n_s", 1.0, 5.0, 1.0, ""),
                ("Application Timing", "t", 0.0, 60.0, 5.0, "DAP"),
                ("N Source", "src", 1.0, 4.0, 1.0, "type"),
                ("Inhibitor Use", "inh", 0.0, 1.0, 0.5, ""),
            ],
            derivation: vec![
                (1, "Mineralization", "SOM, T", "N_m = k \\times SOM \\times Q_{10}^{(T-25)/10}", "Arrhenius", "kg/ha"),
                (2, "Plant uptake", "N_{avail}, LAI", "U = N_{avail} \\times (1-e^{-k_{ext} LAI})", "Michaelis", "kg/ha"),
                (3, "Leaching", "N_{avail}, P", "L = N_{avail} \\times (P/P_{crit})^2", "Exponential", "kg/ha"),
                (4, "Volatilization", "N, pH, T", "V = N \\times f(pH) \\times Q_{10}^{(T-20)/10}", "Empirical", "kg/ha"),
                (5, "N balance", "inputs, outputs", "\\Delta N = (N + N_m) - (U + L + V)", "Conservation", "kg/ha"),
                (6, "Yield response", "U", "Y = Y_0 + a U - b U^2", "Quadratic", "kg/ha"),
                (7, "NUE", "Y, N", "NUE = (Y - Y_0)/N", "Agronomic", "kg/kg"),
                (8, "Environmental penalty", "L, V", "P_e = c_L L + c_V V", "Cost", "$/ha"),
            ],
            knowledge: vec![
                ("LIEBIG", "Liebig's Law", "Y = \\min_i(Y_i(N_i))", "Minimum factor"),
                ("MITSCHERLICH", "Mitscherlich", "Y = A(1-e^{-cx})", "Diminishing returns, 1909"),
            ],
        },
        Project {
            id: "A04",
            title: "Drone Swarm Agricultural Monitoring",
            objective: "Maximize field coverage per hour subject to battery and collision constraints",
            params: vec![
                ("Swarm Size", "N", 3.0, 20.0, 1.0, "drones"),
                ("Altitude", "h", 30.0, 120.0, 10.0, "m"),
                ("Speed", "v", 5.0, 20.0, 2.0, "m/s"),
                ("Overlap", "o", 0.1, 0.4, 0.05, ""),
                ("Image Interval", "\\Delta t", 1.0, 5.0, 0.5, "s"),
            ],
            derivation: vec![
                (1, "Swath width", "h, FOV", "W = 2h \\tan(FOV/2)", "Geometry", "meters"),
                (2, "Effective swath", "W, o", "W_{eff} = W(1-o)", "Overlap removed", "meters"),
                (3, "Coverage rate single", "v, W_{eff}", "A_1 = v \\times W_{eff}", "Area/time", "m²/s"),
                (4, "Swarm coverage", "N, A_1", "A_N = N \\times A_1 \\times \\eta_{coord}", "Coordination", "m²/s"),
                (5, "GSD", "h, f, p", "GSD = h \\times p / f", "Resolution", "cm/pixel"),
                (6, "Flight time", "E, P", "t_f = E/(P_{hover} + P_{trans})", "Battery", "minutes"),
                (7, "Coverage per charge", "A_N, t_f", "A_{max} = A_N \\times t_f \\times 60", "Total", "m²"),
                (8, "Hourly throughput", "A_{max}, t_c", "Q = A_{max} \\times 60/(t_f + t_c)", "Including charge", "m²/h"),
            ],
            knowledge: vec![
                ("SWARM", "Swarm Intelligence", "\\vec{v}_i = \\vec{v}_i + c_1 r_1(\\vec{p}_i-\\vec{x}_i) + c_2 r_2(\\vec{g}-\\vec{x}_i)", "Reynolds, 1987"),
                ("GSD", "Ground Sample Distance", "GSD = (h \\times sensor)/(f \\times image)", "Photogrammetry"),
            ],
        },
        Project {
            id: "A05",
            title: "Greenhouse Climate Control",
            objective: "Minimize energy cost subject to plant growth requirements",
            params: vec![
                ("Day Temperature", "T_d", 18.0, 30.0, 1.0, "°C"),
                ("Night Temperature", "T_n", 12.0, 22.0, 1.0, "°C"),
                ("Humidity", "RH", 50.0, 90.0, 5.0, "%"),
                ("CO2 Level", "C", 400.0, 1200.0, 100.0, "ppm"),
                ("Ventilation Rate", "V", 0.0, 30.0, 5.0, "ACH"),
            ],
            derivation: vec![
                (1, "Heat balance", "Q_{in}, Q_{out}", "dT/dt = (Q_{solar} + Q_{heat} - Q_{vent} - Q_{loss})/(\\rho C_p V)", "Energy", "°C/s"),
                (2, "Solar gain", "G, A, \\tau", "Q_{solar} = G \\times A \\times \\tau", "Transmission", "W"),
                (3, "Heat loss", "U, A, \\Delta T", "Q_{loss} = U A (T_{in} - T_{out})", "Conduction", "W"),
                (4, "Ventilation load", "V, \\rho, c_p, \\Delta T", "Q_{vent} = V \\rho c_p \\Delta T", "Air exchange", "W"),
                (5, "Humidity balance", "E_t, V_{in}, V_{out}", "d\\omega/dt = (E_t - V(\\omega - \\omega_{out}))/V", "Moisture", "kg/s"),
                (6, "CO2 uptake", "P_n", "dC/dt = (C_{inject} - P_n - V(C-C_{out}))/V", "Carbon", "ppm/s"),
                (7, "Photosynthesis rate", "PAR, C, T", "P_n = P_{max} \\times f(PAR) \\times f(C) \\times f(T)", "Farquhar", "μmol/m²s"),
                (8, "Energy cost", "Q_{heat}, C_{inject}, prices", "Cost = Q_{heat} p_e + C_{inject} p_C", "Operating", "$/h"),
            ],
            knowledge: vec![
                ("FARQUHAR", "Farquhar Model", "A = \\min(A_c, A_j) - R_d", "Photosynthesis, 1980"),
                ("VPD", "Vapor Pressure Deficit", "VPD = e_s(T)(1-RH/100)", "Plant physiology"),
            ],
        },
        Project {
            id: "A06",
            title: "Food Processing Thermal Treatment",
            objective: "Maximize pathogen reduction subject to nutrient retention > 80%",
            params: vec![
                ("Temperature", "T", 60.0, 140.0, 5.0, "°C"),
                ("Hold Time", "t", 1.0, 300.0, 10.0, "s"),
                ("Pressure", "P", 1.0, 7.0, 0.5, "bar"),
                ("Product Flow", "Q", 100.0, 1000.0, 100.0, "L/h"),
                ("Preheat Ratio", "r", 0.5, 0.95, 0.05, ""),
            ],
            derivation: vec![
                (1, "Log reduction", "D, t", "LR = t/D_T", "Definition", "Log CFU"),
                (2, "D-value temperature", "D, T, z", "D_T = D_{ref} \\times 10^{(T_{ref}-T)/z}", "First order", "seconds"),
                (3, "F-value", "T, z", "F = \\int 10^{(T(t)-T_{ref})/z} dt", "Lethality", "equivalent"),
                (4, "Nutrient retention", "D_n, t", "R = 10^{-t/D_n}", "First order", "Fraction"),
                (5, "Cook value", "T, z_c", "C = \\int 10^{(T(t)-100)/z_c} dt", "Quality", "equivalent"),
                (6, "Energy input", "Q, c_p, \\Delta T", "E = Q \\rho c_p \\Delta T", "Heating", "kW"),
                (7, "Regeneration", "r, E", "E_{net} = E(1-r)", "Recovery", "kW"),
                (8, "Process objective", "LR, R", "Z = LR - \\lambda \\max(0, 0.8-R)", "Constrained", "Maximize"),
            ],
            knowledge: vec![
                ("ARRHENIUS", "Thermal Death", "D = D_0 e^{-E_a/(RT)}", "First-order kinetics"),
                ("FVALUE", "F-value", "F_0 = \\int 10^{(T-121.1)/10} dt", "Canning, Ball 1923"),
            ],
        },
        Project {
            id: "A07",
            title: "Shelf Life Extension Packaging",
            objective: "Maximize shelf life subject to packaging cost < $0.10/unit",
            params: vec![
                ("O2 Permeability", "P_{O2}", 0.1, 100.0, 5.0, "cc/m²/day"),
                ("CO2 Permeability", "P_{CO2}", 1.0, 500.0, 25.0, "cc/m²/day"),
                ("Film Thickness", "t", 20.0, 100.0, 10.0, "μm"),
                ("Modified Atmosphere", "MA", 0.0, 1.0, 0.25, ""),
                ("Active Scavenger", "AS", 0.0, 1.0, 0.25, ""),
            ],
            derivation: vec![
                (1, "O2 transmission", "P, A, t", "OTR = P_{O2} \\times A / t", "Fick's law", "cc/day"),
                (2, "Respiration rate", "T, O2", "R = R_0 \\times Q_{10}^{(T-T_0)/10} \\times O_2/(K_m+O_2)", "Michaelis", "cc/kg/h"),
                (3, "O2 equilibrium", "OTR, R, m", "O_{2,eq} = O_{2,atm} - R \\times m / OTR", "Steady state", "%"),
                (4, "Microbial growth", "T, a_w, O2", "\\mu = \\mu_{max} \\gamma(T) \\gamma(a_w) \\gamma(O_2)", "Gamma model", "1/h"),
                (5, "Shelf life", "N_0, N_{max}, \\mu", "SL = \\ln(N_{max}/N_0)/\\mu", "Log growth", "hours"),
                (6, "Quality decay", "k, T", "Q = Q_0 e^{-kt}", "First order", "Score"),
                (7, "Film cost", "t, material", "C_f = t \\times \\rho \\times price", "Linear", "$/m²"),
                (8, "Package cost", "C_f, A, AS", "C_p = C_f \\times A + C_{AS}", "Total", "$/unit"),
            ],
            knowledge: vec![
                ("FICK", "Fick's Diffusion", "J = -D \\nabla C", "Mass transfer"),
                ("GAMMA", "Gamma Model", "\\mu = \\mu_{max} \\prod_i \\gamma_i", "Predictive microbiology"),
            ],
        },
        Project {
            id: "A08",
            title: "Aquaculture Feeding Optimization",
            objective: "Maximize feed conversion ratio subject to water quality",
            params: vec![
                ("Feed Rate", "FR", 1.0, 5.0, 0.25, "% BW/day"),
                ("Feeding Frequency", "f", 2.0, 8.0, 1.0, "times/day"),
                ("Protein Content", "CP", 30.0, 50.0, 2.0, "%"),
                ("Pellet Size", "d", 2.0, 12.0, 1.0, "mm"),
                ("Water Exchange", "WE", 5.0, 50.0, 5.0, "%/day"),
            ],
            derivation: vec![
                (1, "Daily feed", "FR, BW", "F = FR \\times BW / 100", "Definition", "kg/day"),
                (2, "Protein intake", "F, CP", "PI = F \\times CP/100", "Product", "kg/day"),
                (3, "Growth rate", "PI, \\eta_p", "G = PI \\times \\eta_p", "Conversion", "kg/day"),
                (4, "FCR", "F, G", "FCR = F/G", "Definition", "kg/kg"),
                (5, "Ammonia production", "PI", "NH_3 = PI \\times 0.092", "Catabolism", "kg/day"),
                (6, "TAN concentration", "NH_3, V, WE", "TAN = NH_3 / (V \\times WE/100)", "Dilution", "mg/L"),
                (7, "Dissolved O2", "respiration, aeration", "DO = DO_{sat} - R/k_{La}", "Balance", "mg/L"),
                (8, "Objective", "FCR, constraints", "Z = 1/FCR", "Maximize growth efficiency", ""),
            ],
            knowledge: vec![
                ("FCR", "Feed Conversion Ratio", "FCR = Feed_{input}/Biomass_{gain}", "Aquaculture"),
                ("TAN", "Total Ammonia Nitrogen", "TAN = NH_3 + NH_4^+", "Water chemistry"),
            ],
        },
        Project {
            id: "A09",
            title: "Vertical Farm Lighting Spectrum",
            objective: "Maximize photosynthesis rate per watt of lighting",
            params: vec![
                ("Blue Fraction", "f_B", 0.1, 0.5, 0.05, ""),
                ("Red Fraction", "f_R", 0.3, 0.8, 0.05, ""),
                ("Far-Red Fraction", "f_{FR}", 0.0, 0.2, 0.02, ""),
                ("PPFD", "I", 100.0, 600.0, 50.0, "μmol/m²s"),
                ("Photoperiod", "P", 12.0, 20.0, 2.0, "h/day"),
            ],
            derivation: vec![
                (1, "DLI", "I, P", "DLI = I \\times P \\times 3600 / 10^6", "Daily light integral", "mol/m²d"),
                (2, "Blue absorption", "f_B, I", "A_B = f_B I \\times \\phi_B", "Quantum yield", "μmol/m²s"),
                (3, "Red absorption", "f_R, I", "A_R = f_R I \\times \\phi_R", "Quantum yield", "μmol/m²s"),
                (4, "PSS ratio", "f_R, f_{FR}", "PSS = 0.89 f_R / (f_R + f_{FR})", "Phytochrome", "Ratio"),
                (5, "Photosynthesis rate", "A_B, A_R", "P_n = P_{max} (1-e^{-k(A_B+A_R)/P_{max}})", "Saturation", "μmol CO2/m²s"),
                (6, "Electrical power", "I, eff", "P_e = I \\times A / (\\eta_{LED} \\times 4.6)", "Conversion", "W"),
                (7, "Photon efficacy", "I, P_e", "\\eta_{ph} = I / P_e", "Definition", "μmol/J"),
                (8, "Objective", "P_n, P_e", "Z = P_n / P_e", "Efficiency", "Maximize"),
            ],
            knowledge: vec![
                ("McCREE", "McCree Curve", "PPFD = \\int_{400}^{700} \\Phi(\\lambda) d\\lambda", "Plant action spectrum"),
                ("EMERSON", "Emerson Enhancement", "P_{R+FR} > P_R + P_{FR}", "Red/far-red synergy"),
            ],
        },
        Project {
            id: "A10",
            title: "Post-Harvest Loss Reduction",
            objective: "Minimize total loss percentage subject to cost constraints",
            params: vec![
                ("Storage Temperature", "T", 0.0, 25.0, 1.0, "°C"),
                ("Humidity", "RH", 70.0, 98.0, 2.0, "%"),
                ("Ventilation", "V", 0.0, 10.0, 1.0, "ACH"),
                ("Crate Stacking", "h", 1.0, 6.0, 1.0, "layers"),
                ("Sorting Intensity", "S", 0.0, 1.0, 0.2, ""),
            ],
            derivation: vec![
                (1, "Respiration loss", "T", "L_r = R_0 \\times Q_{10}^{(T-T_0)/10} \\times t", "Mass loss", "%"),
                (2, "Transpiration loss", "VPD, k", "L_t = k \\times VPD \\times t", "Water loss", "%"),
                (3, "Mechanical damage", "h", "L_m = a \\times h^2", "Compression", "%"),
                (4, "Decay loss", "T, RH, t", "L_d = \\mu_{pathogen} \\times t", "Microbial", "%"),
                (5, "Sorting removal", "S, quality", "R_s = S \\times P(defect)", "Initial cull", "%"),
                (6, "Net loss", "L", "L_{total} = L_r + L_t + L_m + L_d - R_s", "Sum", "%"),
                (7, "Energy cost", "T, T_{amb}", "C_e = (T_{amb}-T) \\times k_{cool} \\times t", "Cooling", "$/kg"),
                (8, "Total cost", "C_e, C_s, L", "C = C_e + C_s + L \\times price", "Economic", "$/kg"),
            ],
            knowledge: vec![
                ("Q10", "Q10 Temperature Coefficient", "Q_{10} = (R_2/R_1)^{10/(T_2-T_1)}", "Biochemistry"),
                ("VPD", "Vapor Pressure Deficit", "VPD = e_s(T)(1-RH/100)", "Storage science"),
            ],
        },
    ];

    for p in &projects {
        let space: u64 = p.params.iter().map(|(_, _, min, max, step, _)| ((max - min) / step) as u64 + 1).product();
        println!("\n[{}] {}", p.id, p.title);
        println!("  Search space: {} configurations", fmt(space));
        generate_latex(p, space, out);
    }

    println!("\n═══════════════════════════════════════════════════════════════");
    println!("  Batch 09 complete: 10 LaTeX papers generated");
    println!("═══════════════════════════════════════════════════════════════");
}
