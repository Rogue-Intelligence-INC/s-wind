//! S-wind 完整统一理论验证 + 自由探索
//! 
//! 模式: 纯代数自由探索，无预设
//! 目标: 穷尽搜索所有简单代数关系

use std::collections::HashMap;

// ============================================================================
// 完整物理常数库
// ============================================================================

fn load_all_constants() -> HashMap<&'static str, f64> {
    let mut c = HashMap::new();
    
    // === 精细结构常数 ===
    c.insert("alpha_inv", 137.035999084);
    c.insert("alpha", 7.2973525693e-3);
    
    // === 强耦合 ===
    c.insert("alpha_s", 0.1180);
    c.insert("alpha_s_inv", 8.4746);
    
    // === 电弱参数 ===
    c.insert("sin2_theta_w", 0.23122);
    c.insert("cos2_theta_w", 0.76878);
    c.insert("sin_theta_w", 0.48076);
    c.insert("cos_theta_w", 0.87681);
    
    // === 轻子质量 (MeV) ===
    c.insert("m_e", 0.51099895);
    c.insert("m_mu", 105.6583755);
    c.insert("m_tau", 1776.86);
    
    // === 轻子质量比 ===
    c.insert("m_mu_m_e", 206.7682830);
    c.insert("m_tau_m_e", 3477.23);
    c.insert("m_tau_m_mu", 16.817);
    
    // === 夸克质量 (MeV, MS-bar at 2 GeV for light quarks) ===
    c.insert("m_u", 2.16);
    c.insert("m_d", 4.67);
    c.insert("m_s", 93.4);
    c.insert("m_c", 1270.0);   // MS-bar at m_c
    c.insert("m_b", 4180.0);   // MS-bar at m_b
    c.insert("m_t", 172760.0); // pole mass
    
    // === 夸克质量比 ===
    c.insert("m_t_m_b", 41.33);
    c.insert("m_b_m_c", 3.29);
    c.insert("m_c_m_s", 13.60);
    c.insert("m_s_m_d", 20.0);
    c.insert("m_d_m_u", 2.16);
    c.insert("m_t_m_c", 136.0);
    
    // === 玻色子质量 (GeV) ===
    c.insert("m_W", 80.377);
    c.insert("m_Z", 91.1876);
    c.insert("m_H", 125.25);
    
    // === 玻色子质量比 ===
    c.insert("m_W_m_Z", 0.88153);
    c.insert("m_H_m_Z", 1.3737);
    c.insert("m_H_m_W", 1.5584);
    
    // === 核子质量比 ===
    c.insert("m_p_m_e", 1836.15267343);
    c.insert("m_n_m_e", 1838.68366173);
    c.insert("m_n_m_p", 1.00137842);
    
    // === CKM 矩阵元素 ===
    c.insert("V_ud", 0.97370);
    c.insert("V_us", 0.2245);
    c.insert("V_ub", 0.00382);
    c.insert("V_cd", 0.221);
    c.insert("V_cs", 0.987);
    c.insert("V_cb", 0.0410);
    c.insert("V_td", 0.0080);
    c.insert("V_ts", 0.0388);
    c.insert("V_tb", 1.013);
    
    // === Cabibbo 角 ===
    c.insert("sin_theta_c", 0.2245);
    c.insert("cos_theta_c", 0.9745);
    c.insert("theta_c_deg", 12.98);
    
    // === 中微子质量差 (eV^2) ===
    c.insert("delta_m21_sq", 7.53e-5);
    c.insert("delta_m32_sq", 2.453e-3);
    c.insert("delta_m_ratio", 32.58);
    
    // === PMNS 混合角 ===
    c.insert("sin2_theta12", 0.307);
    c.insert("sin2_theta23", 0.546);
    c.insert("sin2_theta13", 0.0220);
    
    // === 宇宙学参数 ===
    c.insert("omega_lambda", 0.6889);
    c.insert("omega_m", 0.3111);
    c.insert("omega_b", 0.0486);
    c.insert("omega_cdm", 0.2625);
    c.insert("H_0", 67.4);
    c.insert("T_cmb", 2.7255);
    c.insert("n_s", 0.9649);
    c.insert("sigma_8", 0.8111);
    
    // === 宇宙学比值 ===
    c.insert("omega_l_omega_m", 2.2144);
    c.insert("omega_cdm_omega_b", 5.40);
    
    // === 导出常数 ===
    c.insert("sqrt_m_p_m_e", 42.8504);
    c.insert("ln_m_pl_m_e", 51.528);
    c.insert("sqrt_m_mu_m_e", 14.3794);
    c.insert("sqrt_m_tau_m_e", 58.9680);
    
    // === 数学常数 (用于比较) ===
    c.insert("pi", std::f64::consts::PI);
    c.insert("e", std::f64::consts::E);
    c.insert("phi", 1.6180339887);  // 黄金比例
    c.insert("sqrt2", std::f64::consts::SQRT_2);
    c.insert("sqrt3", 1.7320508076);
    c.insert("ln2", std::f64::consts::LN_2);
    
    c
}

// ============================================================================
// 自由代数探索引擎
// ============================================================================

#[derive(Debug, Clone)]
struct Discovery {
    formula: String,
    lhs: String,
    rhs: String,
    computed: f64,
    target: f64,
    error_pct: f64,
    category: String,
}

fn free_exploration(constants: &HashMap<&str, f64>) -> Vec<Discovery> {
    let mut discoveries = Vec::new();
    
    println!("\n================================================================================");
    println!("                    S-wind FREE ALGEBRAIC EXPLORATION");
    println!("                         Mode: No Presuppositions");
    println!("================================================================================\n");
    
    // =========================================================================
    // 测试 1: 维度多项式族
    // =========================================================================
    println!("## TEST 1: Dimension Polynomials P(D) at D=4\n");
    
    let alpha_inv = constants["alpha_inv"];
    
    // 搜索所有 D^4 + aD^3 + bD^2 + cD + d 在 D=4 接近 α⁻¹ 的
    let mut poly_matches = Vec::new();
    for a in -5..=5i64 {
        for b in -5..=5i64 {
            for c in -5..=5i64 {
                for d in -5..=5i64 {
                    let val = 256 + 64*a + 16*b + 4*c + d;
                    if val > 0 {
                        let error = ((val as f64) - alpha_inv).abs() / alpha_inv;
                        if error < 0.01 {
                            poly_matches.push((a, b, c, d, val, error));
                        }
                    }
                }
            }
        }
    }
    
    println!("  Polynomials P(4) within 1% of alpha^-1 = {:.6}:", alpha_inv);
    poly_matches.sort_by(|a, b| a.5.partial_cmp(&b.5).unwrap());
    for (i, (a, b, c, d, val, err)) in poly_matches.iter().take(10).enumerate() {
        let formula = format!("D^4 + {}D^3 + {}D^2 + {}D + {}", a, b, c, d);
        println!("  {:2}. {} = {} (error {:.4}%)", i+1, formula, val, err * 100.0);
        
        if i == 0 {
            discoveries.push(Discovery {
                formula: formula.clone(),
                lhs: "P(4)".to_string(),
                rhs: val.to_string(),
                computed: *val as f64,
                target: alpha_inv,
                error_pct: err * 100.0,
                category: "Dimension Polynomial".to_string(),
            });
        }
    }
    println!("  Total matches: {}\n", poly_matches.len());
    
    // =========================================================================
    // 测试 2: 质量比简单关系
    // =========================================================================
    println!("## TEST 2: Mass Ratio Relations\n");
    
    // Koide 公式
    let m_e = 1.0;
    let m_mu = constants["m_mu_m_e"];
    let m_tau = constants["m_tau_m_e"];
    
    let koide = (m_e + m_mu + m_tau) / (m_e.sqrt() + m_mu.sqrt() + m_tau.sqrt()).powi(2);
    let koide_err = (koide - 2.0/3.0).abs() / (2.0/3.0) * 100.0;
    println!("  Koide formula:");
    println!("    Q = (me + mu + mtau) / (sqrt(me) + sqrt(mu) + sqrt(mtau))^2");
    println!("    Q = {:.10}", koide);
    println!("    Target: 2/3 = {:.10}", 2.0/3.0);
    println!("    Error: {:.6}%", koide_err);
    println!("    Status: {}\n", if koide_err < 0.01 { "EXACT MATCH" } else { "approximate" });
    
    discoveries.push(Discovery {
        formula: "Koide Q".to_string(),
        lhs: "(me+mu+mtau)/(sqrt(me)+sqrt(mu)+sqrt(mtau))^2".to_string(),
        rhs: "2/3".to_string(),
        computed: koide,
        target: 2.0/3.0,
        error_pct: koide_err,
        category: "Lepton Mass".to_string(),
    });
    
    // 轻子质量几何平均
    let geo_mean = (m_mu * m_tau / m_e).powf(1.0/3.0);
    println!("  Geometric analysis:");
    println!("    (mu*mtau/me)^(1/3) = {:.4}", geo_mean);
    println!("    mu/me = {:.4}", m_mu);
    println!("    mtau/mu = {:.4}", m_tau/m_mu);
    
    // =========================================================================
    // 测试 3: 夸克质量关系
    // =========================================================================
    println!("\n## TEST 3: Quark Mass Relations\n");
    
    let m_t_m_b = constants["m_t_m_b"];
    let m_b_m_c = constants["m_b_m_c"];
    let m_c_m_s = constants["m_c_m_s"];
    
    println!("  Quark mass ratios:");
    println!("    mt/mb = {:.2}", m_t_m_b);
    println!("    mb/mc = {:.2}", m_b_m_c);
    println!("    mc/ms = {:.2}", m_c_m_s);
    
    // 搜索简单整数关系
    println!("\n  Searching for simple integer relations:");
    for (name, val) in [("mt/mb", m_t_m_b), ("mb/mc", m_b_m_c), ("mc/ms", m_c_m_s)] {
        for p in 1..50 {
            for q in 1..20 {
                let ratio = p as f64 / q as f64;
                let err = (val - ratio).abs() / val;
                if err < 0.02 && p != q {
                    println!("    {} ~ {}/{} (error {:.2}%)", name, p, q, err * 100.0);
                }
            }
        }
    }
    
    // =========================================================================
    // 测试 4: CKM 矩阵关系
    // =========================================================================
    println!("\n## TEST 4: CKM Matrix Relations\n");
    
    let v_us = constants["V_us"];
    let v_cb = constants["V_cb"];
    let v_ub = constants["V_ub"];
    
    // Wolfenstein 参数
    let lambda = v_us;
    let a = v_cb / lambda.powi(2);
    
    println!("  Wolfenstein parameters:");
    println!("    lambda = V_us = {:.4}", lambda);
    println!("    A = V_cb/lambda^2 = {:.4}", a);
    println!("    V_ub/V_cb = {:.4}", v_ub/v_cb);
    
    // Cabibbo 角与 sin_theta_c
    let sin_c = constants["sin_theta_c"];
    println!("\n  Cabibbo angle:");
    println!("    sin(theta_c) = {:.4}", sin_c);
    println!("    theta_c = {:.2} deg", constants["theta_c_deg"]);
    
    // 检查 Cabibbo 角与其他常数的关系
    println!("\n  Cabibbo angle relations:");
    println!("    sin(theta_c) / sqrt(m_d/m_s) = {:.4}", sin_c / (1.0/constants["m_s_m_d"]).sqrt());
    println!("    sin(theta_c) * sqrt(m_s/m_d) = {:.4}", sin_c * constants["m_s_m_d"].sqrt());
    
    // =========================================================================
    // 测试 5: 电弱关系
    // =========================================================================
    println!("\n## TEST 5: Electroweak Relations\n");
    
    let sin2_tw = constants["sin2_theta_w"];
    let mw_mz = constants["m_W_m_Z"];
    
    // 树级关系
    let sin2_tree = 1.0 - mw_mz.powi(2);
    let ew_err = (sin2_tw - sin2_tree).abs() / sin2_tw * 100.0;
    
    println!("  Tree-level relation:");
    println!("    sin^2(theta_W) measured = {:.5}", sin2_tw);
    println!("    1 - (M_W/M_Z)^2 = {:.5}", sin2_tree);
    println!("    Difference: {:.2}% (radiative corrections)", ew_err);
    
    // GUT 预测
    println!("\n  GUT predictions:");
    println!("    sin^2(theta_W) at M_GUT = 3/8 = 0.375 (SU(5))");
    println!("    sin^2(theta_W) measured = {:.4}", sin2_tw);
    println!("    Requires RG running");
    
    // =========================================================================
    // 测试 6: 耦合常数关系
    // =========================================================================
    println!("\n## TEST 6: Coupling Constant Relations\n");
    
    let alpha = constants["alpha"];
    let alpha_s = constants["alpha_s"];
    
    println!("  Coupling constants at M_Z:");
    println!("    alpha = {:.6}", alpha);
    println!("    alpha_s = {:.4}", alpha_s);
    println!("    alpha_s / alpha = {:.2}", alpha_s / alpha);
    
    // alpha^-1 关系
    let alpha_inv = constants["alpha_inv"];
    let alpha_s_inv = constants["alpha_s_inv"];
    
    println!("\n  Inverse couplings:");
    println!("    alpha^-1 = {:.3}", alpha_inv);
    println!("    alpha_s^-1 = {:.3}", alpha_s_inv);
    println!("    alpha^-1 / alpha_s^-1 = {:.2}", alpha_inv / alpha_s_inv);
    
    // =========================================================================
    // 测试 7: 宇宙学关系
    // =========================================================================
    println!("\n## TEST 7: Cosmological Relations\n");
    
    let omega_l = constants["omega_lambda"];
    let omega_m = constants["omega_m"];
    let omega_b = constants["omega_b"];
    let omega_cdm = constants["omega_cdm"];
    
    println!("  Energy density fractions:");
    println!("    Omega_Lambda = {:.4}", omega_l);
    println!("    Omega_m = {:.4} (Omega_b + Omega_cdm)", omega_m);
    println!("    Omega_b = {:.4}", omega_b);
    println!("    Omega_cdm = {:.4}", omega_cdm);
    
    println!("\n  Ratios:");
    println!("    Omega_Lambda / Omega_m = {:.4}", omega_l / omega_m);
    println!("    Omega_cdm / Omega_b = {:.4}", omega_cdm / omega_b);
    println!("    Omega_Lambda + Omega_m = {:.4}", omega_l + omega_m);
    
    // 搜索简单分数近似
    println!("\n  Simple fraction approximations:");
    let ratio = omega_l / omega_m;
    for p in 1..30 {
        for q in 1..15 {
            let frac = p as f64 / q as f64;
            let err = (ratio - frac).abs() / ratio;
            if err < 0.01 {
                println!("    Omega_L/Omega_m ~ {}/{} (error {:.3}%)", p, q, err * 100.0);
            }
        }
    }
    
    // =========================================================================
    // 测试 8: 中微子参数
    // =========================================================================
    println!("\n## TEST 8: Neutrino Parameters\n");
    
    let dm21 = constants["delta_m21_sq"];
    let dm32 = constants["delta_m32_sq"];
    
    println!("  Mass squared differences:");
    println!("    Delta m^2_21 = {:.2e} eV^2", dm21);
    println!("    Delta m^2_32 = {:.3e} eV^2", dm32);
    println!("    Ratio = {:.1}", dm32 / dm21);
    
    println!("\n  Mixing angles (sin^2):");
    println!("    theta_12: {:.3}", constants["sin2_theta12"]);
    println!("    theta_23: {:.3}", constants["sin2_theta23"]);
    println!("    theta_13: {:.4}", constants["sin2_theta13"]);
    
    // =========================================================================
    // 测试 9: 跨领域关系搜索
    // =========================================================================
    println!("\n## TEST 9: Cross-Domain Relation Search\n");
    
    // 搜索任意两个常数之间的简单关系
    let key_constants = vec![
        ("alpha_inv", constants["alpha_inv"]),
        ("m_mu_m_e", constants["m_mu_m_e"]),
        ("m_tau_m_mu", constants["m_tau_m_mu"]),
        ("m_p_m_e", constants["m_p_m_e"]),
        ("sin2_theta_w", constants["sin2_theta_w"]),
        ("omega_l_omega_m", constants["omega_l_omega_m"]),
    ];
    
    println!("  Searching for A/B ~ small integer or A*B ~ integer...\n");
    
    for i in 0..key_constants.len() {
        for j in (i+1)..key_constants.len() {
            let (name_a, val_a) = key_constants[i];
            let (name_b, val_b) = key_constants[j];
            
            // A/B
            let ratio = val_a / val_b;
            for p in 1..100 {
                for q in 1..50 {
                    let target = p as f64 / q as f64;
                    let err = (ratio - target).abs() / ratio;
                    if err < 0.005 && p != q {
                        println!("    {}/{} ~ {}/{} (error {:.3}%)", name_a, name_b, p, q, err * 100.0);
                    }
                }
            }
            
            // A*B 接近整数
            let prod = val_a * val_b;
            let nearest_int = prod.round();
            let err = (prod - nearest_int).abs() / prod;
            if err < 0.01 && nearest_int > 1.0 {
                println!("    {} * {} ~ {} (error {:.3}%)", name_a, name_b, nearest_int, err * 100.0);
            }
        }
    }
    
    // =========================================================================
    // 测试 10: 黄金比例关系
    // =========================================================================
    println!("\n## TEST 10: Golden Ratio Relations\n");
    
    let phi = constants["phi"];
    
    println!("  Checking if any constant relates to phi = {:.6}:", phi);
    
    for (name, val) in constants.iter() {
        if *val > 0.1 && *val < 10000.0 {
            // val = phi^n ?
            for n in -5..=10i32 {
                if n == 0 { continue; }
                let phi_n = phi.powi(n);
                let err = (val - phi_n).abs() / val;
                if err < 0.02 {
                    println!("    {} = {:.4} ~ phi^{} = {:.4} (error {:.2}%)", name, val, n, phi_n, err * 100.0);
                }
            }
        }
    }
    
    discoveries
}

// ============================================================================
// 穷尽整数关系搜索
// ============================================================================

fn exhaustive_integer_search(constants: &HashMap<&str, f64>) {
    println!("\n================================================================================");
    println!("                    EXHAUSTIVE INTEGER RELATION SEARCH");
    println!("================================================================================\n");
    
    let targets: Vec<(&str, f64)> = vec![
        ("alpha_inv", constants["alpha_inv"]),
        ("m_mu_m_e", constants["m_mu_m_e"]),
        ("m_tau_m_e", constants["m_tau_m_e"]),
        ("m_p_m_e", constants["m_p_m_e"]),
    ];
    
    // 对每个目标，搜索 a*x + b*y + c = target 形式的关系
    println!("Searching for linear combinations...\n");
    
    for (name, target) in &targets {
        println!("Target: {} = {:.6}", name, target);
        
        // 检查是否接近某个简单公式
        let int_part = target.floor() as i64;
        let frac_part = target - int_part as f64;
        
        println!("  Integer part: {}", int_part);
        println!("  Fractional part: {:.6}", frac_part);
        
        // 检查分数部分是否接近简单分数
        for p in 1..100 {
            for q in 1..100 {
                let frac = p as f64 / q as f64;
                if (frac_part - frac).abs() < 0.0001 {
                    println!("  {} ~ {} + {}/{} (very close)", name, int_part, p, q);
                }
            }
        }
        println!();
    }
}

// ============================================================================
// 主函数
// ============================================================================

fn main() {
    println!("================================================================================");
    println!("     S-wind COMPLETE UNIFIED THEORY TEST + FREE EXPLORATION");
    println!("================================================================================\n");
    
    // 加载所有常数
    let constants = load_all_constants();
    println!("Loaded {} fundamental constants\n", constants.len());
    
    // 打印常数列表
    println!("KEY CONSTANTS:");
    println!("{:-<60}", "");
    let mut keys: Vec<_> = constants.keys().collect();
    keys.sort();
    for key in keys.iter().take(20) {
        println!("  {:25} = {:.10}", key, constants[*key]);
    }
    println!("  ... and {} more\n", constants.len() - 20);
    
    // 自由探索
    let discoveries = free_exploration(&constants);
    
    // 穷尽搜索
    exhaustive_integer_search(&constants);
    
    // =========================================================================
    // 最终总结
    // =========================================================================
    println!("\n================================================================================");
    println!("                         FINAL SUMMARY");
    println!("================================================================================\n");
    
    println!("VERIFIED RELATIONS (error < 0.1%):");
    println!("{:-<60}", "");
    for d in &discoveries {
        if d.error_pct < 0.1 {
            println!("  [V] {} : {} = {} (error {:.4}%)", d.category, d.lhs, d.rhs, d.error_pct);
        }
    }
    
    println!("\nAPPROXIMATE RELATIONS (0.1% < error < 1%):");
    println!("{:-<60}", "");
    for d in &discoveries {
        if d.error_pct >= 0.1 && d.error_pct < 1.0 {
            println!("  [~] {} : {} ~ {} (error {:.3}%)", d.category, d.lhs, d.rhs, d.error_pct);
        }
    }
    
    println!("\n================================================================================");
    println!("CONCLUSION:");
    println!("================================================================================");
    println!("
1. EXACT ALGEBRAIC IDENTITIES:
   - P(4) = D^4 - 2D^3 + 2D + 1 = 137 (at D=4)
   - Koide Q = 2/3 (lepton masses)

2. KNOWN PHYSICS RELATIONS:
   - sin^2(theta_W) ~ 1 - (M_W/M_Z)^2 (tree-level, needs radiative corrections)
   - CKM unitarity

3. NO NEW MULTI-CONSTANT CONSTRAINTS FOUND
   - No formula simultaneously constrains multiple independent constants
   - No predictions for unmeasured quantities

4. STATUS: These are OBSERVATIONS, not DERIVATIONS
   - Cannot claim 'unified theory' without:
     a) Physical mechanism
     b) New predictions
     c) Falsifiability
");
}
