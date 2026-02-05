//! Blondle Problems: Extract challenging problems from all STEM domains
//!
//! Usage: cargo run --bin blondle_problems > problems.json

use s_wind::knowledge::KnowledgeBase;
use s_wind::blondle::BlondleBundle;

fn main() {
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    BlondleBundle::load_complete(&mut kb);
    
    println!("{{");
    println!("  \"timestamp\": \"{}\",", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"));
    println!("  \"domains\": [");
    
    // Extract key formulas/problems from each domain
    let domains = vec![
        ("physics", extract_physics_problems()),
        ("chemistry", extract_chemistry_problems()),
        ("aerospace", extract_aerospace_problems()),
        ("automotive", extract_automotive_problems()),
        ("finance", extract_finance_problems()),
        ("manufacturing", extract_manufacturing_problems()),
        ("electronics", extract_electronics_problems()),
    ];
    
    for (i, (name, problems)) in domains.iter().enumerate() {
        println!("    {{");
        println!("      \"name\": \"{}\",", name);
        println!("      \"problems\": [");
        for (j, p) in problems.iter().enumerate() {
            let comma = if j < problems.len() - 1 { "," } else { "" };
            println!("        {{ \"id\": \"{}\", \"formula\": \"{}\", \"desc\": \"{}\" }}{}", 
                     p.0, p.1.replace("\"", "\\\""), p.2, comma);
        }
        println!("      ]");
        let comma = if i < domains.len() - 1 { "," } else { "" };
        println!("    }}{}", comma);
    }
    
    println!("  ]");
    println!("}}");
}

fn extract_physics_problems() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("P1", "α⁻¹ = D⁴ - 2D³ + 2D + 1", "Fine structure constant from dimension"),
        ("P2", "s[n+1] = D·s[n] - (3D² - 2)", "Scale hierarchy recurrence"),
        ("P3", "S_BH = k_B·A / (4·l_P²)", "Bekenstein-Hawking entropy"),
        ("P4", "T_H = ℏc³ / (8πGMk_B)", "Hawking temperature"),
        ("P5", "ρ_QFT / ρ_obs = 10^122", "Vacuum energy discrepancy"),
    ]
}

fn extract_chemistry_problems() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("C1", "k = A·exp(-Ea/RT)", "Arrhenius activation"),
        ("C2", "ΔG = ΔH - TΔS", "Gibbs free energy"),
        ("C3", "E = E° - (RT/nF)·ln(Q)", "Nernst equation"),
        ("C4", "pH = -log[H⁺]", "pH calculation"),
        ("C5", "PV = nRT", "Ideal gas law"),
    ]
}

fn extract_aerospace_problems() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("A1", "L = ½ρV²SC_L", "Lift equation"),
        ("A2", "C_D = C_D0 + C_L²/(πeAR)", "Drag polar"),
        ("A3", "V_s = √(2W/(ρSC_Lmax))", "Stall speed"),
        ("A4", "R = (V/c)(L/D)·ln(W1/W2)", "Breguet range"),
        ("A5", "ΔV = I_sp·g₀·ln(m0/mf)", "Tsiolkovsky rocket"),
    ]
}

fn extract_automotive_problems() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("V1", "F = D·sin(C·arctan(Bα - E(Bα - arctan(Bα))))", "Pacejka magic formula"),
        ("V2", "K = Wf/Cαf - Wr/Cαr", "Understeer gradient"),
        ("V3", "d = V²/(2μg)", "Braking distance"),
        ("V4", "F_road = mgf_r + ½ρC_DAV²", "Road load"),
        ("V5", "HIC = max[(t2-t1)(∫a dt)^2.5]", "Head injury criterion"),
    ]
}

fn extract_finance_problems() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("F1", "C = SN(d1) - Ke^(-rT)N(d2)", "Black-Scholes call"),
        ("F2", "E[Ri] = rf + βi(E[Rm] - rf)", "CAPM"),
        ("F3", "SR = (E[Rp] - rf) / σp", "Sharpe ratio"),
        ("F4", "D = Σ(t·PV(CFt)) / P", "Macaulay duration"),
        ("F5", "VaR_α = μ + z_α·σ", "Parametric VaR"),
    ]
}

fn extract_manufacturing_problems() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("M1", "Cp = (USL - LSL) / 6σ", "Process capability"),
        ("M2", "VT^n = C", "Taylor tool life"),
        ("M3", "Rt = f² / 8r", "Theoretical roughness"),
        ("M4", "T_total = √(ΣTi²)", "RSS tolerance stack"),
        ("M5", "UCL = X̄ + 3R̄/d2", "Control chart limits"),
    ]
}

fn extract_electronics_problems() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("E1", "Z0 ≈ 87/√(εr+1.41)·ln(5.98h/(0.8w+t))", "Microstrip impedance"),
        ("E2", "Γ = (ZL - Z0) / (ZL + Z0)", "Reflection coefficient"),
        ("E3", "f0 = 1 / (2π√LC)", "LC resonance"),
        ("E4", "δ = √(2ρ/ωμ)", "Skin depth"),
        ("E5", "TJ = TA + PD·θJA", "Junction temperature"),
    ]
}
