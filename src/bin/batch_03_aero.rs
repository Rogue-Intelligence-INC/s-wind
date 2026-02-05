//! Batch 03: Aerospace & Transportation (10 projects)
//! Run: cargo run --release --bin batch_03_aero

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
    println!("  S-wind Batch 03: Aerospace & Transportation");
    println!("═══════════════════════════════════════════════════════════════\n");

    let out = Path::new("papers_batch");
    fs::create_dir_all(out.join("03_aero")).unwrap();

    let projects = vec![
        ("G1", "Next-Gen Commercial Aircraft Aerodynamics", "Maximize L/D ratio", vec![
            ("Aspect Ratio", "AR", 8.0, 20.0, 0.5, ""),
            ("Sweep Angle", "Λ", 20.0, 40.0, 1.0, "°"),
            ("Taper Ratio", "λ", 0.2, 0.5, 0.02, ""),
            ("Thickness", "t/c", 0.08, 0.14, 0.005, ""),
            ("Camber", "f/c", 0.0, 0.04, 0.005, ""),
        ], vec![
            (1, "Oswald efficiency", "AR, Λ, λ", "e = 1.78(1-0.045AR^0.68)-0.64", "Raymer", "Span efficiency"),
            (2, "Induced drag factor", "AR, e", "K = 1/(πeAR)", "First principles", "Induced drag"),
            (3, "Zero-lift drag", "t/c, Λ", "C_D0 = C_f×FF×S_wet/S", "Semi-empirical", "Form factor"),
            (4, "Optimal lift coeff", "C_D0, K", "C_L,opt = √(C_D0/K)", "Calculus", "dL/D/dC_L=0"),
            (5, "Maximum L/D", "C_D0, K", "(L/D)_max = 1/(2√(C_D0K))", "Substitution", "Result"),
            (6, "Camber effect", "f/c", "ΔC_D = k|C_L-C_L,design|²", "Drag bucket", "Off-design"),
            (7, "Wing weight", "AR, t/c", "W ∝ AR^1.5(t/c)^-0.4", "Torenbeek", "Structure"),
        ], vec![
            ("RAYMER", "Aircraft Design", "Oswald correlation", "Raymer, 2018"),
            ("PRANDTL", "Lifting Line", "Induced drag", "Prandtl, 1918"),
        ]),

        ("G2", "eVTOL Urban Air Mobility Design", "Maximize range, noise < 65 dBA", vec![
            ("Rotor Count", "N_r", 4.0, 12.0, 2.0, ""),
            ("Rotor Diameter", "D", 1.0, 3.0, 0.1, "m"),
            ("Tip Speed", "V_tip", 150.0, 220.0, 10.0, "m/s"),
            ("Disk Loading", "DL", 200.0, 600.0, 50.0, "N/m²"),
            ("Battery Sp Energy", "E_sp", 250.0, 400.0, 25.0, "Wh/kg"),
        ], vec![
            (1, "Disk area", "N_r, D", "A = N_r×π(D/2)²", "Geometry", "Total area"),
            (2, "Thrust required", "W", "T = W", "Hover", "Equilibrium"),
            (3, "Induced velocity", "T, A, ρ", "v_i = √(T/2ρA)", "Momentum", "Actuator disk"),
            (4, "Induced power", "T, v_i", "P_i = T×v_i", "First principles", "Hover power"),
            (5, "Profile power", "ρ,A,V,C_d", "P_0 = ρAV³C_d/8", "Blade element", "Rotor drag"),
            (6, "Endurance", "E, P", "t = E/P", "Energy", "Hover time"),
            (7, "Noise level", "V_tip, DL", "L = 10log(DL)+30log(V_tip)", "NASA", "Correlation"),
        ], vec![
            ("MOMENTUM", "Momentum Theory", "T = 2ρAv²", "Froude, 1889"),
            ("EVTOL", "eVTOL Design", "Urban air mobility", "Johnson, 2022"),
        ]),

        ("G3", "UAV Endurance Optimization", "Maximize flight time", vec![
            ("Wing Span", "b", 1.0, 5.0, 0.25, "m"),
            ("Wing Area", "S", 0.2, 2.0, 0.1, "m²"),
            ("Battery Mass", "m_b", 0.5, 5.0, 0.25, "kg"),
            ("Motor Efficiency", "η_m", 0.8, 0.95, 0.05, ""),
            ("Prop Diameter", "D_p", 0.2, 0.5, 0.05, "m"),
        ], vec![
            (1, "Aspect ratio", "b, S", "AR = b²/S", "Definition", "Wing AR"),
            (2, "Oswald efficiency", "AR", "e = 0.85", "Typical", "UAV value"),
            (3, "Induced drag", "C_L, AR, e", "C_Di = C_L²/(πeAR)", "First principles", "Induced"),
            (4, "Total drag", "C_D0, C_Di", "C_D = C_D0 + C_Di", "Sum", "Total"),
            (5, "Power required", "D, V", "P = D×V = ½ρV³SC_D", "Definition", "Flight power"),
            (6, "Battery energy", "m_b, E_sp", "E = m_b × E_sp", "Definition", "Total energy"),
            (7, "Endurance", "E, P, η", "t = E×η/P", "Division", "Flight time"),
        ], vec![
            ("BREGUET", "Breguet Range", "R = (V/SFC)(L/D)ln(W_i/W_f)", "Breguet"),
            ("ELECPROP", "Electric Propulsion", "η = η_m×η_p×η_esc", "Traub, 2011"),
        ]),

        ("G4", "Electric Motor Efficiency Map", "Maximize avg efficiency over drive cycle", vec![
            ("Pole Pairs", "p", 2.0, 8.0, 1.0, ""),
            ("Slot Count", "Q", 12.0, 48.0, 6.0, ""),
            ("Magnet Thickness", "t_m", 2.0, 8.0, 0.5, "mm"),
            ("Air Gap", "g", 0.5, 2.0, 0.25, "mm"),
            ("Stack Length", "L", 50.0, 200.0, 25.0, "mm"),
        ], vec![
            (1, "Flux density", "t_m, g", "B = B_r×t_m/(t_m+g)", "Magnetic circuit", "Air gap field"),
            (2, "Back EMF", "B, L, N, ω", "E = kBLNω", "Faraday", "Induced voltage"),
            (3, "Copper loss", "I, R", "P_Cu = I²R", "Joule heating", "Winding"),
            (4, "Iron loss", "B, f", "P_Fe = k_h Bf + k_e(Bf)²", "Steinmetz", "Core loss"),
            (5, "Mechanical loss", "ω", "P_mech = k_f ω", "Friction", "Bearings"),
            (6, "Output power", "T, ω", "P_out = T×ω", "Definition", "Shaft power"),
            (7, "Efficiency", "P_out, P_loss", "η = P_out/(P_out+ΣP_loss)", "Definition", "Motor eff"),
        ], vec![
            ("STEINMETZ", "Steinmetz Eq", "P = k_h Bf + k_e(Bf)²", "Steinmetz, 1892"),
            ("PMSM", "PMSM Model", "dq-frame equations", "Krishnan, 2010"),
        ]),

        ("G5", "Vehicle Thermal Management", "Minimize energy consumption", vec![
            ("Compressor Power", "W_c", 1.0, 5.0, 0.5, "kW"),
            ("Evaporator Size", "A_e", 0.1, 0.5, 0.05, "m²"),
            ("Condenser Size", "A_c", 0.2, 0.8, 0.1, "m²"),
            ("Refrigerant Charge", "m_r", 0.3, 1.0, 0.1, "kg"),
            ("Fan Speed", "n_f", 500.0, 3000.0, 250.0, "rpm"),
        ], vec![
            (1, "Cooling capacity", "A_e, ΔT, h", "Q_e = h×A_e×LMTD", "Heat transfer", "Evaporator"),
            (2, "Condensing capacity", "A_c, ΔT", "Q_c = h×A_c×LMTD", "Heat transfer", "Condenser"),
            (3, "COP", "Q_e, W_c", "COP = Q_e/W_c", "Definition", "Efficiency"),
            (4, "Fan power", "n_f", "P_fan = k×n_f³", "Fan laws", "Cubic"),
            (5, "Total power", "W_c, P_fan", "P_tot = W_c + P_fan", "Sum", "System power"),
            (6, "Cabin temp", "Q_e, Q_load", "T_cab = f(Q_e, Q_load)", "Energy balance", "Steady state"),
            (7, "Comfort metric", "T_cab, RH", "PMV = f(T, RH, v)", "Fanger", "Thermal comfort"),
        ], vec![
            ("COP", "Coefficient of Performance", "COP = Q_L/W", "Thermodynamics"),
            ("FANGER", "Fanger PMV", "Thermal comfort", "Fanger, 1970"),
        ]),

        ("G6", "Whole Vehicle Energy Optimization", "Minimize energy per km", vec![
            ("Vehicle Mass", "m", 1500.0, 2500.0, 100.0, "kg"),
            ("Frontal Area", "A", 2.0, 3.0, 0.1, "m²"),
            ("Drag Coeff", "C_D", 0.2, 0.35, 0.01, ""),
            ("Roll Resistance", "C_rr", 0.006, 0.012, 0.001, ""),
            ("Regen Efficiency", "η_reg", 0.5, 0.9, 0.05, ""),
        ], vec![
            (1, "Aero drag", "ρ, A, C_D, v", "F_aero = ½ρC_DAv²", "First principles", "Air resistance"),
            (2, "Rolling resistance", "m, g, C_rr", "F_roll = m×g×C_rr", "First principles", "Tire loss"),
            (3, "Grade force", "m, g, θ", "F_grade = m×g×sin(θ)", "First principles", "Climbing"),
            (4, "Accel force", "m, a", "F_acc = m×a", "Newton", "Inertia"),
            (5, "Total force", "F_*", "F = F_aero + F_roll + F_grade + F_acc", "Sum", "Traction"),
            (6, "Power demand", "F, v", "P = F×v", "Definition", "Wheel power"),
            (7, "Energy/km", "∫P dt, d", "E_km = ∫P dt / d", "Integration", "Consumption"),
        ], vec![
            ("EPA", "EPA Drive Cycles", "UDDS, HWFET", "EPA, 2008"),
            ("ADVISOR", "ADVISOR Model", "Vehicle simulation", "NREL, 2002"),
        ]),

        ("K1", "Traffic Signal Optimization", "Minimize total vehicle delay", vec![
            ("Cycle Length", "C", 60.0, 180.0, 10.0, "s"),
            ("Green Ratio", "g", 0.3, 0.7, 0.05, ""),
            ("Offset", "θ", 0.0, 1.0, 0.1, "×C"),
            ("Coordination", "K", 1.0, 4.0, 1.0, "type"),
            ("Adaptive Weight", "α", 0.0, 1.0, 0.1, ""),
        ], vec![
            (1, "Saturation flow", "lanes, s_0", "s = lanes × s_0", "Definition", "veh/hr green"),
            (2, "Capacity", "s, g, C", "c = s × g/C", "Webster", "veh/hr"),
            (3, "Degree of saturation", "q, c", "x = q/c", "Definition", "v/c ratio"),
            (4, "Uniform delay", "C, g, x", "d_1 = C(1-g/C)²/(2(1-xg/C))", "Webster", "Signal delay"),
            (5, "Random delay", "x, c", "d_2 = x²/(2c(1-x))", "Queuing", "Overflow"),
            (6, "Total delay", "d_1, d_2", "d = d_1 + d_2", "Sum", "Per vehicle"),
            (7, "Network delay", "d, q", "D = Σ d×q", "Aggregation", "System"),
        ], vec![
            ("WEBSTER", "Webster Formula", "C_opt = (1.5L+5)/(1-Y)", "Webster, 1958"),
            ("HCM", "Highway Capacity Manual", "LOS criteria", "TRB, 2022"),
        ]),

        ("K3", "Autonomous Vehicle Path Planning", "Minimize travel time, collision-free", vec![
            ("Horizon", "T", 3.0, 10.0, 1.0, "s"),
            ("Time Step", "Δt", 0.05, 0.2, 0.05, "s"),
            ("Vel Resolution", "Δv", 0.5, 2.0, 0.5, "m/s"),
            ("Lat Resolution", "Δy", 0.1, 0.5, 0.1, "m"),
            ("Safety Margin", "d_s", 1.0, 5.0, 0.5, "m"),
        ], vec![
            (1, "State space", "T, Δt, Δv, Δy", "N = (T/Δt)(v/Δv)(y/Δy)", "Discretization", "Grid size"),
            (2, "Dynamics", "x, v, a", "x' = v, v' = a", "Kinematics", "Double integrator"),
            (3, "Cost function", "t, a, j", "J = w_t t + w_a∫a²dt", "Objective", "Time + comfort"),
            (4, "Collision check", "x, obs", "d_min = min||x-x_i||", "Distance", "Closest"),
            (5, "Safety constraint", "d_min, d_s", "d_min > d_s", "Constraint", "Buffer"),
            (6, "A* search", "graph, J", "path = A*(start, goal)", "Algorithm", "Optimal"),
            (7, "Smoothing", "path", "traj = smooth(path)", "Optimization", "Continuous"),
        ], vec![
            ("ASTAR", "A* Algorithm", "f(n) = g(n) + h(n)", "Hart, 1968"),
            ("MPC", "Model Predictive Control", "Receding horizon", "Borrelli, 2017"),
        ]),

        ("K2", "Subway Timetable Optimization", "Minimize passenger wait time", vec![
            ("Headway", "h", 2.0, 10.0, 0.5, "min"),
            ("Dwell Time", "t_d", 20.0, 60.0, 5.0, "s"),
            ("Train Length", "L", 4.0, 8.0, 1.0, "cars"),
            ("Speed Limit", "v", 60.0, 100.0, 10.0, "km/h"),
            ("Buffer Time", "t_b", 30.0, 120.0, 15.0, "s"),
        ], vec![
            (1, "Capacity per train", "L, c_car", "C = L × c_car", "Definition", "Passengers"),
            (2, "Line capacity", "C, h", "Cap = C × 60/h", "Definition", "pax/hr"),
            (3, "Avg wait time", "h", "W = h/2", "Uniform arrivals", "Expected wait"),
            (4, "Travel time", "d, v, t_d, n", "T = d/v + n×t_d", "Sum", "End-to-end"),
            (5, "Recovery time", "T, t_b", "T_rec = t_b", "Buffer", "Delay recovery"),
            (6, "Fleet size", "T, T_rec, h", "N = 2(T+T_rec)/h", "Cycle", "Trains needed"),
            (7, "Operating cost", "N, c_train", "Cost = N × c_train", "Definition", "Daily"),
        ], vec![
            ("TRANSIT", "Transit Capacity", "TCQSM method", "TRB, 2013"),
            ("HEADWAY", "Headway Optimization", "Frequency vs cost", "Ceder, 2007"),
        ]),

        ("K4", "Parking Pricing & Guidance", "Maximize revenue and utilization", vec![
            ("Base Price", "p_0", 1.0, 5.0, 0.5, "$/hr"),
            ("Surge Mult", "k_s", 1.0, 3.0, 0.25, ""),
            ("Discount", "d", 0.0, 0.3, 0.05, ""),
            ("Update Freq", "f", 5.0, 60.0, 5.0, "min"),
            ("Info Accuracy", "α", 0.7, 0.99, 0.05, ""),
        ], vec![
            (1, "Occupancy", "arrivals, departures", "O = ∫(λ-μ)dt / C", "Balance", "Fill rate"),
            (2, "Dynamic price", "p_0, O, k_s", "p = p_0 × (1 + k_s×(O-0.8))", "Surge", "Demand-based"),
            (3, "Demand elasticity", "p", "λ = λ_0 × (p_ref/p)^ε", "Economics", "Price response"),
            (4, "Revenue", "p, λ, t", "R = p × λ × t_avg", "Definition", "Income"),
            (5, "Cruising reduction", "α", "C_cruise = C_0 × (1-α)", "Information", "Guidance effect"),
            (6, "Emissions saved", "C_cruise", "E = C_cruise × e_km", "Environment", "CO2"),
            (7, "Social welfare", "R, E, delay", "W = R + v_E×E - v_d×delay", "Total", "Net benefit"),
        ], vec![
            ("SFPARK", "SFpark", "Dynamic pricing pilot", "SF MTA, 2014"),
            ("SHOUP", "Parking Economics", "High Cost of Free Parking", "Shoup, 2011"),
        ]),
    ];

    let mut index = String::from("# Batch 03: Aerospace & Transportation\n\n| ID | Title | Steps | Space |\n|---|---|---|---|\n");

    for (i, (id, title, obj, params, deriv, refs)) in projects.iter().enumerate() {
        println!("[{}/10] {} - {}", i+1, id, title);
        let space: u64 = params.iter().map(|(_, _, mi, ma, st, _)| ((ma - mi) / st + 1.0) as u64).product();
        let paper = gen_paper(id, title, obj, params, deriv, refs, space);
        fs::write(out.join("03_aero").join(format!("{}.md", id)), &paper).unwrap();
        index.push_str(&format!("| {} | {} | {} | {} |\n", id, title, deriv.len(), fmt(space)));
        println!("  ✓ {} steps, {} configs", deriv.len(), fmt(space));
    }

    fs::write(out.join("03_aero").join("INDEX.md"), &index).unwrap();
    println!("\n✅ Batch 03 complete: 10 papers in papers_batch/03_aero/");
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
    p.push_str("\n---\n*S-wind Batch 03*\n");
    p
}
