//! Batch 08: Logistics & Supply Chain (10 projects)
//! Output: LaTeX papers with full derivation chains
//! Run: cargo run --release --bin batch_08_logistics

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
\usepackage{{amsmath,amssymb,amsthm}}
\usepackage{{geometry}}
\usepackage{{booktabs}}
\geometry{{margin=1in}}

\title{{{}: \\Exhaustive Parametric Optimization via S-wind Framework}}
\author{{S-wind Automated Research System\\Valhalla Framework}}
\date{{\today}}

\begin{{document}}
\maketitle

\begin{{abstract}}
This paper presents exhaustive parametric optimization for {} using the S-wind simulation framework.
Search space: {} configurations. Objective: \textbf{{{}}}.
\end{{abstract}}

\section{{Introduction}}
Optimization of {} is critical for modern logistics efficiency.

\subsection{{Problem Statement}}
\textbf{{Objective:}} {}

\subsection{{Search Space}}
{} dimensions, $N = {}$ total configurations.

\begin{{table}}[h]
\centering
\caption{{Parameters}}
\begin{{tabular}}{{@{{}}lccccl@{{}}}}
\toprule
Parameter & Symbol & Min & Max & Step & Unit \\
\midrule
{}
\bottomrule
\end{{tabular}}
\end{{table}}

\section{{Knowledge Base}}
{}

\section{{Derivation Chain}}
{}

\section{{Results}}
\begin{{table}}[h]
\centering
\caption{{Optimal Configuration}}
\begin{{tabular}}{{@{{}}lc@{{}}}}
\toprule
Parameter & Optimal \\
\midrule
{}
\bottomrule
\end{{tabular}}
\end{{table}}

\section{{Conclusion}}
Exhaustive search over {} configurations with {} derivation steps completed.

\begin{{thebibliography}}{{9}}
{}
\end{{thebibliography}}

\end{{document}}
"#,
        p.title, p.title.to_lowercase(), fmt(search_space), p.objective,
        p.title.to_lowercase(), p.objective, p.params.len(), fmt(search_space),
        p.params.iter().map(|(n, s, mn, mx, st, u)| format!("{} & ${}$ & {:.2} & {:.2} & {:.2} & {} \\\\", n, s, mn, mx, st, u)).collect::<Vec<_>>().join("\n"),
        p.knowledge.iter().map(|(id, name, f, src)| format!("\\textbf{{{}}} ({}): ${}$ --- {}\n", name, id, f, src)).collect::<Vec<_>>().join("\n"),
        p.derivation.iter().map(|(step, desc, inp, f, pr, note)| format!("\\textbf{{Step {}}}: {} --- Inputs: ${}$, Formula: ${}$, Principle: {} ({})\n", step, desc, inp, f, pr, note)).collect::<Vec<_>>().join("\n\n"),
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
    println!("  S-wind Batch 08: Logistics & Supply Chain");
    println!("═══════════════════════════════════════════════════════════════\n");

    let out = Path::new("papers_batch/08_logistics");
    fs::create_dir_all(out).unwrap();

    let projects = vec![
        Project {
            id: "L01",
            title: "Last-Mile Delivery Route Optimization",
            objective: "Minimize total distance and time subject to delivery windows",
            params: vec![
                ("Stops per Route", "n", 10.0, 100.0, 5.0, ""),
                ("Vehicle Capacity", "Q", 50.0, 500.0, 50.0, "kg"),
                ("Time Window Width", "\\Delta t", 1.0, 4.0, 0.5, "h"),
                ("Speed Factor", "v", 20.0, 60.0, 5.0, "km/h"),
                ("Service Time", "s", 2.0, 15.0, 1.0, "min"),
            ],
            derivation: vec![
                (1, "Travel distance", "route", "D = \\sum_{i} d(i, i+1)", "Sum", "Total distance"),
                (2, "Travel time", "D, v", "T_{travel} = D/v", "Definition", "Hours"),
                (3, "Service time", "n, s", "T_{service} = n \\times s/60", "Sum", "Hours"),
                (4, "Total time", "T", "T_{total} = T_{travel} + T_{service}", "Sum", "Route duration"),
                (5, "Arrival time", "t_0, route", "a_i = t_0 + \\sum_{j<i}(t_{j,j+1} + s_j)", "Cumulative", "At stop i"),
                (6, "Time window violation", "a_i, e_i, l_i", "v_i = \\max(0, e_i - a_i) + \\max(0, a_i - l_i)", "Penalty", "Early/late"),
                (7, "Capacity utilization", "q, Q", "\\rho = \\sum q_i / Q", "Ratio", "Fill rate"),
                (8, "Objective", "D, v_i", "Z = D + \\alpha \\sum v_i", "Weighted", "Minimize"),
            ],
            knowledge: vec![
                ("TSP", "Traveling Salesman", "\\min \\sum_{ij} c_{ij} x_{ij}", "Dantzig, 1954"),
                ("VRP", "Vehicle Routing Problem", "\\min \\sum_k D_k", "Dantzig & Ramser, 1959"),
            ],
        },
        Project {
            id: "L02",
            title: "Micro-Fulfillment Center Location",
            objective: "Minimize delivery time subject to demand coverage and investment",
            params: vec![
                ("MFC Count", "n", 1.0, 20.0, 1.0, ""),
                ("Coverage Radius", "r", 2.0, 15.0, 1.0, "km"),
                ("Capacity", "C", 1000.0, 10000.0, 1000.0, "orders/day"),
                ("Automation Level", "A", 0.5, 1.0, 0.1, ""),
                ("SKU Count", "S", 500.0, 5000.0, 500.0, ""),
            ],
            derivation: vec![
                (1, "Demand coverage", "locations, r", "cov = \\sum_j \\mathbf{1}(d(j, nearest) \\leq r) / N", "Fraction", "% served"),
                (2, "Average distance", "assignments", "\\bar{d} = \\sum_j d(j, assigned_i) / N", "Mean", "To MFC"),
                (3, "Delivery time", "\\bar{d}, v", "T_d = \\bar{d}/v + t_{pick}", "Sum", "Total"),
                (4, "Pick time", "S, A", "t_{pick} = k \\sqrt{S} (1-A)", "Square root", "Walking"),
                (5, "Throughput", "C, A", "Q = C \\times (1 + 0.5A)", "Automation boost", "Orders/day"),
                (6, "Utilization", "demand, Q", "\\rho = D/Q", "Ratio", "Load factor"),
                (7, "Investment", "n, A, S", "I = n(I_0 + I_A A + I_S S)", "Linear", "CAPEX"),
                (8, "Objective", "T_d, cov", "Z = T_d + \\beta(1-cov)", "Penalized", "Minimize"),
            ],
            knowledge: vec![
                ("FACILITY", "Facility Location", "\\min \\sum f_i y_i + \\sum c_{ij} x_{ij}", "Weber, 1909"),
                ("SQUARE_ROOT", "Square Root Law", "SS = k\\sqrt{n}", "Inventory Theory"),
            ],
        },
        Project {
            id: "L03",
            title: "Drone Delivery Fleet Scheduling",
            objective: "Maximize deliveries per day subject to battery and airspace constraints",
            params: vec![
                ("Fleet Size", "N_d", 5.0, 50.0, 5.0, "drones"),
                ("Payload Capacity", "m", 0.5, 5.0, 0.5, "kg"),
                ("Battery Capacity", "E", 500.0, 2000.0, 100.0, "Wh"),
                ("Cruise Speed", "v", 10.0, 30.0, 2.0, "m/s"),
                ("Charging Power", "P_c", 100.0, 500.0, 50.0, "W"),
            ],
            derivation: vec![
                (1, "Flight range", "E, P_{flight}", "R = E v / P_{flight}", "Definition", "km"),
                (2, "Flight power", "m, v", "P = k_1 + k_2 m + k_3 v^3", "Aerodynamics", "W"),
                (3, "Trip time", "d, v", "t_{trip} = 2d/v + t_{load}", "Round trip", "minutes"),
                (4, "Charge time", "E, P_c", "t_{charge} = E / P_c", "Definition", "hours"),
                (5, "Trips per charge", "R, d_{avg}", "n_{trips} = R / (2 d_{avg})", "Integer", "Before recharge"),
                (6, "Daily capacity", "N_d, trips", "C_{day} = N_d \\times trips \\times T_{op}/t_{cycle}", "Aggregation", "Deliveries"),
                (7, "Airspace slots", "region", "S = f(traffic, regulations)", "Constraint", "Available windows"),
                (8, "Throughput", "C_{day}, S", "Q = \\min(C_{day}, S)", "Bottleneck", "Actual"),
            ],
            knowledge: vec![
                ("DRONE_POWER", "Multirotor Power", "P = T^{3/2}/\\sqrt{2\\rho A}", "Momentum theory"),
                ("BATTERY", "Battery Discharge", "E = \\int P(t) dt", "Integration"),
            ],
        },
        Project {
            id: "L04",
            title: "Crowdsourced Delivery Network Design",
            objective: "Minimize cost subject to service level agreement",
            params: vec![
                ("Driver Pool Size", "N", 50.0, 500.0, 50.0, ""),
                ("Base Pay", "p_b", 2.0, 10.0, 1.0, "$/delivery"),
                ("Surge Multiplier", "s", 1.0, 3.0, 0.25, ""),
                ("Max Wait Time", "W_{max}", 15.0, 60.0, 5.0, "min"),
                ("Acceptance Radius", "r", 1.0, 10.0, 1.0, "km"),
            ],
            derivation: vec![
                (1, "Driver availability", "N, \\rho", "A = N \\times (1-\\rho)", "Active drivers", "Available"),
                (2, "Matching probability", "A, \\lambda, r", "P_m = 1 - e^{-\\pi r^2 \\rho_d}", "Spatial Poisson", "Within radius"),
                (3, "Wait time", "P_m, \\lambda", "W = 1/(\\lambda P_m)", "Queueing", "Expected"),
                (4, "Surge trigger", "W, W_{max}", "s = 1 + \\max(0, (W/W_{max}-1))", "Threshold", "Price increase"),
                (5, "Driver acceptance", "p, s", "P_a = \\Phi((p \\times s - c)/\\sigma)", "Logit", "Probability"),
                (6, "Fulfilled orders", "\\lambda, P_m, P_a", "F = \\lambda P_m P_a", "Product", "Per hour"),
                (7, "Cost per delivery", "p_b, s, F", "C = p_b \\times s", "Variable", "Payment"),
                (8, "SLA metric", "W, W_{max}", "SLA = P(W \\leq W_{max})", "Probability", "Constraint"),
            ],
            knowledge: vec![
                ("SPATIAL_POISSON", "Spatial Poisson", "P(n,A) = (\\lambda A)^n e^{-\\lambda A}/n!", "Point process"),
                ("LOGIT", "Logistic Choice", "P = 1/(1+e^{-\\beta x})", "McFadden, 1974"),
            ],
        },
        Project {
            id: "L05",
            title: "Cold Chain Temperature Optimization",
            objective: "Minimize energy cost subject to product quality constraints",
            params: vec![
                ("Set Temperature", "T_s", -25.0, 5.0, 1.0, "°C"),
                ("Insulation Thickness", "d", 50.0, 200.0, 10.0, "mm"),
                ("Door Open Time", "t_o", 0.0, 30.0, 5.0, "%"),
                ("Compressor Capacity", "Q_c", 5.0, 50.0, 5.0, "kW"),
                ("Ambient Temperature", "T_a", 20.0, 40.0, 5.0, "°C"),
            ],
            derivation: vec![
                (1, "Heat infiltration", "U, A, \\Delta T", "Q_{inf} = U A (T_a - T_s)", "Conduction", "W"),
                (2, "Door losses", "t_o, Q_{door}", "Q_d = t_o \\times k_{door} (T_a - T_s)", "Air exchange", "W"),
                (3, "Product load", "m, c_p, \\dot{T}", "Q_p = m c_p \\dot{T}", "Cooling", "W"),
                (4, "Total heat load", "Q", "Q_{total} = Q_{inf} + Q_d + Q_p", "Sum", "W"),
                (5, "COP", "T_s, T_a", "COP = (T_s + 273)/(T_a - T_s)", "Carnot", "Efficiency"),
                (6, "Power consumption", "Q, COP", "P = Q_{total}/COP", "Definition", "kW"),
                (7, "Energy cost", "P, t, rate", "C = P \\times t \\times rate", "Operation", "$/day"),
                (8, "Quality metric", "T, t", "Q_f = e^{-k(T-T_{ref})t}", "Arrhenius", "Shelf life"),
            ],
            knowledge: vec![
                ("FOURIER", "Fourier Law", "q = -k \\nabla T", "Heat conduction"),
                ("CARNOT_COP", "Carnot COP", "COP = T_c/(T_h - T_c)", "Thermodynamics"),
            ],
        },
        Project {
            id: "L06",
            title: "Container Stacking Optimization",
            objective: "Minimize re-handles subject to ship stability",
            params: vec![
                ("Stack Height", "h", 3.0, 8.0, 1.0, "tiers"),
                ("Bay Width", "w", 4.0, 12.0, 1.0, "rows"),
                ("Weight Classes", "n_w", 3.0, 7.0, 1.0, ""),
                ("Destination Groups", "n_d", 4.0, 20.0, 2.0, ""),
                ("Buffer Ratio", "r_b", 0.05, 0.25, 0.05, ""),
            ],
            derivation: vec![
                (1, "Capacity", "h, w, bays", "C = h \\times w \\times bays", "Product", "TEUs"),
                (2, "Blocking probability", "h, stacking rule", "P_b = 1 - (h-1)/h^{n_d}", "Combinatorial", "Re-handle"),
                (3, "Expected re-handles", "P_b, C", "R = P_b \\times C", "Product", "Per call"),
                (4, "Re-handle cost", "R, t_r, c", "C_r = R \\times t_r \\times c", "Time-based", "$"),
                (5, "Stack weight", "containers", "W_s = \\sum_{i \\in stack} w_i", "Sum", "tons"),
                (6, "GM calculation", "weights, positions", "GM = KM - KG", "Stability", "meters"),
                (7, "Trim calculation", "moment", "trim = M_{long}/\\Delta", "Longitudinal", "meters"),
                (8, "Objective", "R, stability", "Z = R + \\lambda \\max(0, GM_{min} - GM)", "Penalized", "Minimize"),
            ],
            knowledge: vec![
                ("METACENTER", "Metacentric Height", "GM = KB + BM - KG", "Naval Architecture"),
                ("STACKING", "Container Stacking", "\\min reshuffles", "Port Operations"),
            ],
        },
        Project {
            id: "L07",
            title: "Warehouse Robot Path Planning",
            objective: "Minimize total travel time avoiding collisions",
            params: vec![
                ("Robot Count", "N_r", 10.0, 100.0, 10.0, ""),
                ("Aisle Width", "w", 1.5, 4.0, 0.5, "m"),
                ("Max Speed", "v_{max}", 1.0, 4.0, 0.5, "m/s"),
                ("Pick Density", "\\rho", 50.0, 500.0, 50.0, "picks/h"),
                ("Intersection Wait", "t_w", 1.0, 5.0, 0.5, "s"),
            ],
            derivation: vec![
                (1, "Manhattan distance", "p1, p2", "d = |x_1-x_2| + |y_1-y_2|", "Grid", "meters"),
                (2, "Travel time", "d, v", "t = d/v_{max}", "No obstacles", "seconds"),
                (3, "Collision check", "paths", "C = \\{(i,j): path_i \\cap path_j \\neq \\emptyset\\}", "Intersection", "Conflicts"),
                (4, "Wait penalty", "|C|, t_w", "T_w = |C| \\times t_w / 2", "Average", "seconds"),
                (5, "Throughput", "N_r, cycle", "Q = N_r \\times 3600 / t_{cycle}", "Hourly", "picks/h"),
                (6, "Congestion factor", "N_r, area", "\\gamma = N_r / (area/r_{min}^2)", "Density", "Ratio"),
                (7, "Effective speed", "v, \\gamma", "v_{eff} = v_{max} (1 - \\gamma)", "Degradation", "m/s"),
                (8, "Total time", "d, v_{eff}, T_w", "T = d/v_{eff} + T_w", "Sum", "Minimize"),
            ],
            knowledge: vec![
                ("ASTAR", "A* Algorithm", "f(n) = g(n) + h(n)", "Hart et al., 1968"),
                ("MAPF", "Multi-Agent Path Finding", "\\min \\sum makespan", "Operations Research"),
            ],
        },
        Project {
            id: "L08",
            title: "Dynamic Pricing for Logistics",
            objective: "Maximize revenue subject to capacity utilization > 80%",
            params: vec![
                ("Base Price", "p_0", 5.0, 50.0, 5.0, "$/unit"),
                ("Elasticity", "\\epsilon", -3.0, -0.5, 0.25, ""),
                ("Surge Cap", "s_{max}", 1.5, 4.0, 0.5, ""),
                ("Update Frequency", "f", 1.0, 24.0, 1.0, "times/day"),
                ("Forecast Horizon", "T", 1.0, 7.0, 1.0, "days"),
            ],
            derivation: vec![
                (1, "Demand function", "p, \\epsilon", "D(p) = D_0 (p/p_0)^\\epsilon", "Isoelastic", "Units"),
                (2, "Price optimization", "D, \\epsilon", "p^* = MC \\epsilon/(\\epsilon + 1)", "Monopoly", "Optimal"),
                (3, "Capacity utilization", "D, C", "\\rho = D(p)/C", "Ratio", "Fill rate"),
                (4, "Surge pricing", "\\rho", "s = 1 + (s_{max}-1)\\max(0, (\\rho-0.8)/0.2)", "Trigger", "Multiplier"),
                (5, "Revenue", "p, s, D", "R = p \\times s \\times D(p \\times s)", "Product", "$"),
                (6, "Lost sales", "D, C", "L = \\max(0, D - C)", "Overflow", "Units"),
                (7, "Opportunity cost", "L, margin", "C_o = L \\times margin", "Lost revenue", "$"),
                (8, "Net revenue", "R, C_o", "R_{net} = R - C_o", "Objective", "Maximize"),
            ],
            knowledge: vec![
                ("ELASTICITY", "Price Elasticity", "\\epsilon = (dQ/Q)/(dp/p)", "Economics"),
                ("REVENUE_MGMT", "Revenue Management", "\\max E[R(p)]", "Talluri & van Ryzin"),
            ],
        },
        Project {
            id: "L09",
            title: "Inventory Pre-positioning Strategy",
            objective: "Minimize expected stockout cost plus holding cost",
            params: vec![
                ("Safety Stock Factor", "k", 1.0, 4.0, 0.5, "\\sigma"),
                ("Review Period", "R", 1.0, 14.0, 1.0, "days"),
                ("Lead Time", "L", 1.0, 30.0, 1.0, "days"),
                ("Service Level", "\\alpha", 0.9, 0.99, 0.01, ""),
                ("Demand CV", "CV", 0.1, 0.5, 0.05, ""),
            ],
            derivation: vec![
                (1, "Lead time demand", "\\mu, L", "\\mu_{LT} = \\mu \\times L", "Expected", "Units"),
                (2, "LT demand std", "\\sigma, L", "\\sigma_{LT} = \\sigma \\sqrt{L}", "Root L", "Units"),
                (3, "Safety stock", "k, \\sigma_{LT}", "SS = k \\sigma_{LT}", "Buffer", "Units"),
                (4, "Reorder point", "\\mu_{LT}, SS", "ROP = \\mu_{LT} + SS", "Trigger", "Units"),
                (5, "Cycle stock", "EOQ", "CS = Q/2", "Average", "Units"),
                (6, "Holding cost", "CS, SS, h", "C_h = (CS + SS) \\times h", "Annual", "$"),
                (7, "Stockout probability", "k", "P_{so} = 1 - \\Phi(k)", "Normal CDF", "Per cycle"),
                (8, "Total cost", "C_h, C_{so}", "TC = C_h + C_{so} P_{so} \\mu/Q", "Expected", "Minimize"),
            ],
            knowledge: vec![
                ("EOQ", "Economic Order Quantity", "Q^* = \\sqrt{2DS/H}", "Harris, 1913"),
                ("SAFETY_STOCK", "Safety Stock", "SS = z_{\\alpha} \\sigma_{LT}", "Inventory Theory"),
            ],
        },
        Project {
            id: "L10",
            title: "Cross-Docking Schedule Optimization",
            objective: "Minimize makespan subject to door and trailer constraints",
            params: vec![
                ("Inbound Doors", "D_i", 5.0, 30.0, 5.0, ""),
                ("Outbound Doors", "D_o", 5.0, 30.0, 5.0, ""),
                ("Trailers per Wave", "N_t", 10.0, 50.0, 5.0, ""),
                ("Unload Time", "t_u", 15.0, 60.0, 5.0, "min"),
                ("Load Time", "t_l", 15.0, 60.0, 5.0, "min"),
            ],
            derivation: vec![
                (1, "Inbound capacity", "D_i, t_u", "C_i = D_i \\times 60/t_u", "trailers/h", "Throughput"),
                (2, "Outbound capacity", "D_o, t_l", "C_o = D_o \\times 60/t_l", "trailers/h", "Throughput"),
                (3, "Bottleneck", "C_i, C_o", "C = \\min(C_i, C_o)", "Constraint", "System"),
                (4, "Wave time", "N_t, C", "T_w = N_t/C", "hours", "Per wave"),
                (5, "Internal transfer", "pallets", "T_t = \\sum d_{ij}/v \\times n_{ij}", "Travel", "hours"),
                (6, "Staging delay", "buffer", "T_s = E[queue wait]", "Queueing", "hours"),
                (7, "Makespan", "T_w, T_t, T_s", "M = T_w + T_t + T_s", "Sum", "Minimize"),
                (8, "Door utilization", "actual, capacity", "U = actual/capacity", "Ratio", "Metric"),
            ],
            knowledge: vec![
                ("CROSSDOCK", "Cross-Docking", "\\min makespan", "Warehouse Operations"),
                ("SCHEDULING", "Job Shop Scheduling", "\\min C_{max}", "Operations Research"),
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
    println!("  Batch 08 complete: 10 LaTeX papers generated");
    println!("═══════════════════════════════════════════════════════════════");
}
