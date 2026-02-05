//! 🛩️ Complete Optimal Aircraft Design System
//!
//! 完整最优飞行器设计系统
//! 基于 Blondle 知识库进行穷尽搜索
//! 支持多种飞行器类型：亚音速、跨音速、超音速
//! 输出：详细报告 + 高精度 3D 模型

use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════════════════════╗");
    println!("║  🛩️  S-WIND COMPLETE OPTIMAL AIRCRAFT DESIGN SYSTEM                            ║");
    println!("║                                                                                ║");
    println!("║  完整最优飞行器设计系统 - 基于 Blondle 航空航天知识库                             ║");
    println!("║  穷尽搜索 • 物理约束 • 多类型设计 • 高精度 3D 模型                               ║");
    println!("╚════════════════════════════════════════════════════════════════════════════════╝\n");

    // ==========================================================================
    // PHASE 0: 加载知识库常数
    // ==========================================================================
    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 0: 加载 Blondle 航空航天知识库                                           │");
    println!("└─────────────────────────────────────────────────────────────────────────────────┘\n");
    
    let kb = load_aerospace_knowledge();
    println!("  ✓ 加载 {} 个物理常数", kb.constants.len());
    println!("  ✓ 加载 {} 个物理定律", kb.laws.len());
    println!("  ✓ 加载 {} 个数学定理", kb.theorems.len());
    println!();

    // ==========================================================================
    // PHASE 1: 设计多种飞行器类型
    // ==========================================================================
    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 1: 多类型飞行器优化设计                                                  │");
    println!("└─────────────────────────────────────────────────────────────────────────────────┘\n");

    let design_types = vec![
        DesignObjective::SubsonicEfficiency,    // 亚音速高效
        DesignObjective::HighAltitudeLongEndurance,  // 高空长航时
        DesignObjective::HighSubsonic,          // 高亚音速运输
        DesignObjective::Supersonic,            // 超音速
    ];

    let mut all_designs = Vec::new();

    for objective in &design_types {
        println!("  ━━━ 设计目标: {} ━━━", objective.name());
        let design = optimize_for_objective(objective, &kb);
        print_design_summary(&design);
        all_designs.push(design);
        println!();
    }

    // ==========================================================================
    // PHASE 2: 选择最佳亚音速设计进行详细分析
    // ==========================================================================
    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 2: 最佳亚音速设计详细分析                                                │");
    println!("└─────────────────────────────────────────────────────────────────────────────────┘\n");

    let best_subsonic = &all_designs[0];
    print_detailed_derivation(best_subsonic, &kb);

    // ==========================================================================
    // PHASE 3: 生成所有设计的 3D 模型
    // ==========================================================================
    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 3: 生成高精度 3D 模型                                                    │");
    println!("└─────────────────────────────────────────────────────────────────────────────────┘\n");

    for (i, design) in all_designs.iter().enumerate() {
        let filename = format!("optimal_aircraft_{}.obj", design.objective.short_name());
        generate_high_fidelity_3d(design, &filename);
        println!("  ✓ 生成: {} (顶点: {}, 面: {})", 
                 filename, design.model_vertices, design.model_faces);
    }
    println!();

    // ==========================================================================
    // PHASE 4: 生成综合报告
    // ==========================================================================
    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 4: 生成综合设计报告                                                      │");
    println!("└─────────────────────────────────────────────────────────────────────────────────┘\n");

    generate_comprehensive_report(&all_designs, &kb);

    // ==========================================================================
    // PHASE 5: STL 格式输出 (3D 打印兼容)
    // ==========================================================================
    println!("┌─────────────────────────────────────────────────────────────────────────────────┐");
    println!("│ PHASE 5: 生成 STL 格式 (3D 打印兼容)                                           │");
    println!("└─────────────────────────────────────────────────────────────────────────────────┘\n");

    for design in &all_designs {
        let filename = format!("optimal_aircraft_{}.stl", design.objective.short_name());
        generate_stl(design, &filename);
        println!("  ✓ 生成: {}", filename);
    }
    println!();

    // ==========================================================================
    // 完成
    // ==========================================================================
    println!("╔════════════════════════════════════════════════════════════════════════════════╗");
    println!("║  ✅ 完成! 基于 Blondle 知识库的完整最优飞行器设计已生成                          ║");
    println!("║                                                                                ║");
    println!("║  输出文件:                                                                     ║");
    println!("║    • optimal_aircraft_*.obj  - Wavefront OBJ 格式 (Blender/FreeCAD)            ║");
    println!("║    • optimal_aircraft_*.stl  - STL 格式 (3D 打印)                              ║");
    println!("║    • AIRCRAFT_COMPREHENSIVE_REPORT.md - 综合设计报告                           ║");
    println!("╚════════════════════════════════════════════════════════════════════════════════╝");
}

// ============================================================================
// 知识库结构
// ============================================================================

struct AerospaceKnowledge {
    constants: Vec<PhysicalConstant>,
    laws: Vec<PhysicalLaw>,
    theorems: Vec<MathTheorem>,
}

#[derive(Clone)]
struct PhysicalConstant {
    id: String,
    name: String,
    value: f64,
    unit: String,
    tex: String,
}

#[derive(Clone)]
struct PhysicalLaw {
    id: String,
    name: String,
    equation_tex: String,
    description: String,
}

#[derive(Clone)]
struct MathTheorem {
    id: String,
    name: String,
    statement: String,
}

fn load_aerospace_knowledge() -> AerospaceKnowledge {
    let constants = vec![
        PhysicalConstant {
            id: "ISA_T0".into(),
            name: "ISA 海平面温度".into(),
            value: 288.15,
            unit: "K".into(),
            tex: r"T_0 = 288.15 \text{ K}".into(),
        },
        PhysicalConstant {
            id: "ISA_P0".into(),
            name: "ISA 海平面压力".into(),
            value: 101325.0,
            unit: "Pa".into(),
            tex: r"P_0 = 101325 \text{ Pa}".into(),
        },
        PhysicalConstant {
            id: "ISA_rho0".into(),
            name: "ISA 海平面密度".into(),
            value: 1.225,
            unit: "kg/m³".into(),
            tex: r"\rho_0 = 1.225 \text{ kg/m}^3".into(),
        },
        PhysicalConstant {
            id: "ISA_a0".into(),
            name: "ISA 海平面音速".into(),
            value: 340.29,
            unit: "m/s".into(),
            tex: r"a_0 = 340.29 \text{ m/s}".into(),
        },
        PhysicalConstant {
            id: "g0".into(),
            name: "标准重力加速度".into(),
            value: 9.80665,
            unit: "m/s²".into(),
            tex: r"g_0 = 9.80665 \text{ m/s}^2".into(),
        },
        PhysicalConstant {
            id: "gamma_air".into(),
            name: "空气比热比".into(),
            value: 1.4,
            unit: "-".into(),
            tex: r"\gamma = 1.4".into(),
        },
        PhysicalConstant {
            id: "R_air".into(),
            name: "空气气体常数".into(),
            value: 287.058,
            unit: "J/(kg·K)".into(),
            tex: r"R = 287.058 \text{ J/(kg·K)}".into(),
        },
    ];

    let laws = vec![
        PhysicalLaw {
            id: "lift_equation".into(),
            name: "升力方程".into(),
            equation_tex: r"L = \frac{1}{2}\rho V^2 S C_L".into(),
            description: "气动升力计算".into(),
        },
        PhysicalLaw {
            id: "drag_equation".into(),
            name: "阻力方程".into(),
            equation_tex: r"D = \frac{1}{2}\rho V^2 S C_D".into(),
            description: "气动阻力计算".into(),
        },
        PhysicalLaw {
            id: "drag_polar".into(),
            name: "抛物线阻力极曲线".into(),
            equation_tex: r"C_D = C_{D0} + \frac{C_L^2}{\pi e AR}".into(),
            description: "阻力系数与升力系数关系".into(),
        },
        PhysicalLaw {
            id: "oswald_efficiency".into(),
            name: "Oswald 效率因子".into(),
            equation_tex: r"e \approx 1.78(1-0.045AR^{0.68})-0.64".into(),
            description: "翼展效率估算".into(),
        },
        PhysicalLaw {
            id: "max_ld".into(),
            name: "最大升阻比".into(),
            equation_tex: r"(L/D)_{max} = \frac{1}{2}\sqrt{\frac{\pi e AR}{C_{D0}}}".into(),
            description: "理论最大升阻比".into(),
        },
        PhysicalLaw {
            id: "breguet_range".into(),
            name: "Breguet 航程公式".into(),
            equation_tex: r"R = \frac{V}{c}\frac{L}{D}\ln\frac{W_1}{W_2}".into(),
            description: "巡航航程计算".into(),
        },
        PhysicalLaw {
            id: "wave_drag".into(),
            name: "激波阻力".into(),
            equation_tex: r"C_{D,wave} \propto (M-M_{crit})^2".into(),
            description: "跨音速/超音速激波阻力".into(),
        },
    ];

    let theorems = vec![
        MathTheorem {
            id: "kutta_joukowski".into(),
            name: "Kutta-Joukowski 定理".into(),
            statement: r"L' = \rho V \Gamma".into(),
        },
        MathTheorem {
            id: "elliptic_loading".into(),
            name: "椭圆升力分布".into(),
            statement: r"\Gamma(y) = \Gamma_0\sqrt{1-(2y/b)^2}".into(),
        },
        MathTheorem {
            id: "prandtl_lifting_line".into(),
            name: "Prandtl 升力线理论".into(),
            statement: "三维机翼升力从二维翼型数据计算".into(),
        },
        MathTheorem {
            id: "area_rule".into(),
            name: "Whitcomb 面积律".into(),
            statement: "跨音速阻力最小化需截面积平滑变化".into(),
        },
    ];

    AerospaceKnowledge { constants, laws, theorems }
}

// ============================================================================
// 设计目标
// ============================================================================

#[derive(Clone, Copy, PartialEq)]
enum DesignObjective {
    SubsonicEfficiency,         // 亚音速高效 (M < 0.6)
    HighAltitudeLongEndurance,  // 高空长航时 (M ≈ 0.5, 高度 > 15km)
    HighSubsonic,               // 高亚音速 (M ≈ 0.85)
    Supersonic,                 // 超音速 (M ≈ 1.5-2.0)
}

impl DesignObjective {
    fn name(&self) -> &str {
        match self {
            Self::SubsonicEfficiency => "亚音速高效 (M < 0.6)",
            Self::HighAltitudeLongEndurance => "高空长航时 UAV (M ≈ 0.5)",
            Self::HighSubsonic => "高亚音速运输 (M ≈ 0.85)",
            Self::Supersonic => "超音速巡航 (M ≈ 1.8)",
        }
    }

    fn short_name(&self) -> &str {
        match self {
            Self::SubsonicEfficiency => "subsonic_efficient",
            Self::HighAltitudeLongEndurance => "hale_uav",
            Self::HighSubsonic => "high_subsonic",
            Self::Supersonic => "supersonic",
        }
    }

    fn design_mach(&self) -> f64 {
        match self {
            Self::SubsonicEfficiency => 0.4,
            Self::HighAltitudeLongEndurance => 0.5,
            Self::HighSubsonic => 0.85,
            Self::Supersonic => 1.8,
        }
    }
}

// ============================================================================
// 飞行器设计
// ============================================================================

#[derive(Clone)]
struct AircraftDesign {
    objective: DesignObjective,
    
    // 几何参数
    aspect_ratio: f64,
    sweep_angle: f64,       // 度
    taper_ratio: f64,
    thickness: f64,         // t/c
    camber: f64,            // f/c
    
    // 气动特性
    oswald_e: f64,
    cd0: f64,
    induced_factor: f64,
    cl_optimal: f64,
    l_d_ratio: f64,
    
    // 速度相关
    design_mach: f64,
    critical_mach: f64,
    
    // 3D 模型
    model_vertices: usize,
    model_faces: usize,
    
    // 尺寸 (30m 翼展基准)
    wing_span: f64,
    wing_area: f64,
    root_chord: f64,
    tip_chord: f64,
    
    is_valid: bool,
}

// ============================================================================
// 优化算法
// ============================================================================

fn optimize_for_objective(objective: &DesignObjective, _kb: &AerospaceKnowledge) -> AircraftDesign {
    let design_mach = objective.design_mach();
    
    // 根据设计目标调整搜索范围
    let (ar_range, sweep_range, thickness_range) = match objective {
        DesignObjective::SubsonicEfficiency => {
            (vec_range(5.0, 40.0, 0.5), vec_range(0.0, 15.0, 1.0), vec_range(0.06, 0.15, 0.01))
        }
        DesignObjective::HighAltitudeLongEndurance => {
            (vec_range(15.0, 50.0, 0.5), vec_range(0.0, 10.0, 1.0), vec_range(0.06, 0.12, 0.01))
        }
        DesignObjective::HighSubsonic => {
            (vec_range(7.0, 12.0, 0.2), vec_range(20.0, 40.0, 1.0), vec_range(0.08, 0.14, 0.01))
        }
        DesignObjective::Supersonic => {
            (vec_range(1.5, 5.0, 0.1), vec_range(40.0, 75.0, 1.0), vec_range(0.03, 0.08, 0.005))
        }
    };
    
    let taper_range = vec_range(0.1, 1.0, 0.05);
    let camber_range = vec_range(0.0, 0.08, 0.01);
    
    let mut best_design: Option<AircraftDesign> = None;
    let mut best_score = f64::NEG_INFINITY;
    
    for &ar in &ar_range {
        for &sweep in &sweep_range {
            for &taper in &taper_range {
                for &thickness in &thickness_range {
                    for &camber in &camber_range {
                        let design = evaluate_design(*objective, ar, sweep, taper, thickness, camber, design_mach);
                        
                        if design.is_valid {
                            let score = calculate_score(&design, objective);
                            if score > best_score {
                                best_score = score;
                                best_design = Some(design);
                            }
                        }
                    }
                }
            }
        }
    }
    
    best_design.unwrap_or_else(|| {
        // 返回默认设计
        evaluate_design(*objective, 10.0, 25.0, 0.4, 0.10, 0.02, design_mach)
    })
}

fn vec_range(start: f64, end: f64, step: f64) -> Vec<f64> {
    let n = ((end - start) / step).ceil() as usize + 1;
    (0..n).map(|i| start + i as f64 * step).collect()
}

fn evaluate_design(
    objective: DesignObjective,
    ar: f64,
    sweep_deg: f64,
    taper: f64,
    thickness: f64,
    camber: f64,
    design_mach: f64,
) -> AircraftDesign {
    let sweep_rad = sweep_deg * PI / 180.0;
    
    // Oswald 效率因子
    let e_base = 1.78 * (1.0 - 0.045 * ar.powf(0.68)) - 0.64;
    let e_sweep = sweep_rad.cos().powf(0.5);
    let e_taper = 1.0 - 0.25 * (taper - 0.4).abs();
    let oswald_e = (e_base * e_sweep * e_taper).max(0.1).min(0.98);
    
    // 零升阻力
    let cf = 0.0025;
    let form_factor = 1.0 + 1.2 * thickness + 100.0 * thickness.powi(4);
    let wetted_ratio = 2.1;
    let cd0_wing = cf * form_factor * wetted_ratio;
    let cd0_body = 0.002 * (1.0 + 0.5 * sweep_rad.cos());
    
    // 激波阻力 (高亚音速和超音速)
    let m_crit = 0.7 - 0.1 * thickness * 10.0 + 0.1 * sweep_rad.cos(); // 临界马赫数
    let cd_wave = if design_mach > m_crit {
        0.1 * (design_mach - m_crit).powi(2)
    } else {
        0.0
    };
    
    let cd0 = cd0_wing + cd0_body + cd_wave;
    
    // 诱导阻力因子
    let induced_factor = 1.0 / (PI * oswald_e * ar);
    
    // 最优升力系数和升阻比
    let cl_optimal = (cd0 / induced_factor).sqrt();
    let cd_at_optimal = cd0 + induced_factor * cl_optimal.powi(2);
    let l_d_ratio = cl_optimal / cd_at_optimal;
    
    // 尺寸计算 (30m 翼展基准)
    let wing_span = 30.0;
    let wing_area = wing_span * wing_span / ar;
    let mean_chord = wing_area / wing_span;
    let root_chord = 2.0 * mean_chord / (1.0 + taper);
    let tip_chord = root_chord * taper;
    
    // 有效性检查
    let is_valid = 
        ar >= 1.5 && ar <= 55.0 &&
        oswald_e > 0.3 &&
        cl_optimal > 0.05 && cl_optimal < 2.5 &&
        l_d_ratio > 0.0 && l_d_ratio < 100.0;
    
    AircraftDesign {
        objective,
        aspect_ratio: ar,
        sweep_angle: sweep_deg,
        taper_ratio: taper,
        thickness,
        camber,
        oswald_e,
        cd0,
        induced_factor,
        cl_optimal,
        l_d_ratio,
        design_mach,
        critical_mach: m_crit,
        model_vertices: 0,
        model_faces: 0,
        wing_span,
        wing_area,
        root_chord,
        tip_chord,
        is_valid,
    }
}

fn calculate_score(design: &AircraftDesign, objective: &DesignObjective) -> f64 {
    match objective {
        DesignObjective::SubsonicEfficiency => {
            // 最大化 L/D
            design.l_d_ratio
        }
        DesignObjective::HighAltitudeLongEndurance => {
            // 最大化 L/D，同时考虑高展弦比结构可行性
            design.l_d_ratio - 0.1 * (design.aspect_ratio - 25.0).max(0.0)
        }
        DesignObjective::HighSubsonic => {
            // 平衡 L/D 和跨音速性能
            design.l_d_ratio * (1.0 - 0.5 * design.cd0.min(0.02))
        }
        DesignObjective::Supersonic => {
            // 超音速性能，考虑激波阻力
            design.l_d_ratio + 2.0 * design.critical_mach
        }
    }
}

// ============================================================================
// 输出函数
// ============================================================================

fn print_design_summary(design: &AircraftDesign) {
    println!("    展弦比 AR = {:.1}, 后掠角 = {:.0}°, 梢根比 = {:.2}", 
             design.aspect_ratio, design.sweep_angle, design.taper_ratio);
    println!("    相对厚度 = {:.1}%, 升阻比 L/D = {:.2}", 
             design.thickness * 100.0, design.l_d_ratio);
}

fn print_detailed_derivation(design: &AircraftDesign, kb: &AerospaceKnowledge) {
    println!("  ╔═══════════════════════════════════════════════════════════════════════╗");
    println!("  ║                    完整推导过程                                        ║");
    println!("  ╚═══════════════════════════════════════════════════════════════════════╝\n");
    
    println!("  Step 1: 应用升力方程 [{}]", kb.laws[0].id);
    println!("          {}", kb.laws[0].equation_tex);
    println!();
    
    println!("  Step 2: 应用阻力极曲线 [{}]", kb.laws[2].id);
    println!("          {}", kb.laws[2].equation_tex);
    println!();
    
    println!("  Step 3: 计算 Oswald 效率 [{}]", kb.laws[3].id);
    println!("          {}", kb.laws[3].equation_tex);
    println!("          e_base = 1.78 × (1 - 0.045 × {:.1}^0.68) - 0.64 = {:.4}", 
             design.aspect_ratio, 1.78 * (1.0 - 0.045 * design.aspect_ratio.powf(0.68)) - 0.64);
    println!("          e_sweep = cos({:.0}°)^0.5 = {:.4}", 
             design.sweep_angle, (design.sweep_angle * PI / 180.0).cos().sqrt());
    println!("          e = {:.4}", design.oswald_e);
    println!();
    
    println!("  Step 4: 计算零升阻力 C_D0");
    println!("          C_f = 0.0025 (混合边界层)");
    println!("          FF = 1 + 1.2×{:.2} + 100×{:.2}^4 = {:.4}", 
             design.thickness, design.thickness,
             1.0 + 1.2 * design.thickness + 100.0 * design.thickness.powi(4));
    println!("          C_D0 = {:.5}", design.cd0);
    println!();
    
    println!("  Step 5: 求解最优升力系数 [{}]", kb.laws[4].id);
    println!("          {}", kb.laws[4].equation_tex);
    println!("          K = 1/(π × {:.3} × {:.1}) = {:.5}", 
             design.oswald_e, design.aspect_ratio, design.induced_factor);
    println!("          C_L,opt = √(C_D0/K) = √({:.5}/{:.5}) = {:.4}",
             design.cd0, design.induced_factor, design.cl_optimal);
    println!();
    
    println!("  Step 6: 计算最大升阻比");
    println!("          (L/D)_max = C_L,opt / (C_D0 + K×C_L,opt²)");
    println!("                    = {:.4} / ({:.5} + {:.5}×{:.4}²)",
             design.cl_optimal, design.cd0, design.induced_factor, design.cl_optimal);
    println!("                    = {:.2} ✓", design.l_d_ratio);
    println!();
}

// ============================================================================
// 3D 模型生成
// ============================================================================

fn generate_high_fidelity_3d(design: &AircraftDesign, filename: &str) {
    let mut verts: Vec<[f64; 3]> = Vec::new();
    let mut faces: Vec<Vec<usize>> = Vec::new();
    
    let wing_span = design.wing_span;
    let root_chord = design.root_chord;
    let tip_chord = design.tip_chord;
    let sweep_rad = design.sweep_angle * PI / 180.0;
    let dihedral = 3.0_f64.to_radians();
    
    // 高精度展向分段
    let n_span = 60;
    let n_chord = 20;  // 翼型弦向分段
    
    // ========================================
    // 机翼生成 (NACA 4412 近似)
    // ========================================
    for side in [-1.0_f64, 1.0] {
        let wing_start = verts.len();
        
        for i in 0..=n_span {
            let eta = i as f64 / n_span as f64;
            let y = side * eta * wing_span / 2.0;
            
            // 弦长分布 (混合椭圆-线性)
            let chord_elliptic = root_chord * (1.0 - eta * eta).sqrt().max(0.1);
            let chord_linear = root_chord + eta * (tip_chord - root_chord);
            let chord = 0.6 * chord_elliptic + 0.4 * chord_linear;
            
            // 前缘位置
            let x_le = eta * wing_span / 2.0 * sweep_rad.tan();
            let z_dihedral = eta * wing_span / 2.0 * dihedral.sin();
            
            // 翼型厚度和弯度
            let t = design.thickness;
            let m = design.camber;  // 最大弯度位置
            
            // 生成翼型点 (NACA 近似)
            for j in 0..=n_chord {
                let x_frac = j as f64 / n_chord as f64;
                let x = x_le + x_frac * chord;
                
                // NACA 厚度分布
                let yt = t / 0.2 * chord * (
                    0.2969 * x_frac.sqrt() 
                    - 0.1260 * x_frac 
                    - 0.3516 * x_frac.powi(2)
                    + 0.2843 * x_frac.powi(3)
                    - 0.1015 * x_frac.powi(4)
                );
                
                // 弯度线
                let yc = if x_frac < 0.4 {
                    m / 0.16 * (2.0 * 0.4 * x_frac - x_frac.powi(2))
                } else {
                    m / 0.36 * (1.0 - 2.0 * 0.4 + 2.0 * 0.4 * x_frac - x_frac.powi(2))
                } * chord;
                
                // 上下表面
                if j == 0 || j == n_chord {
                    verts.push([x, y, z_dihedral + yc]);
                } else {
                    verts.push([x, y, z_dihedral + yc + yt]); // 上表面
                    verts.push([x, y, z_dihedral + yc - yt]); // 下表面
                }
            }
        }
        
        // 生成翼面
        let pts_per_section = 2 * n_chord - 1;
        for i in 0..n_span {
            let base = wing_start + i * pts_per_section;
            let next = wing_start + (i + 1) * pts_per_section;
            
            for j in 0..pts_per_section - 1 {
                faces.push(vec![base + j + 1, base + j + 2, next + j + 2, next + j + 1]);
            }
        }
    }
    
    // ========================================
    // 机身 (翼身融合或传统，取决于设计)
    // ========================================
    let body_start = verts.len();
    let body_length = root_chord * 2.5;
    let body_width = root_chord * 0.4;
    let body_height = root_chord * 0.3;
    
    let n_body = 30;
    let n_circ = 16;
    
    for i in 0..=n_body {
        let t = i as f64 / n_body as f64;
        let x = -body_length * 0.2 + t * body_length;
        
        // 超椭圆截面
        let scale = if t < 0.15 {
            (t / 0.15).powf(0.5)
        } else if t > 0.85 {
            ((1.0 - t) / 0.15).powf(0.5).max(0.15)
        } else {
            1.0
        };
        
        let w = body_width * scale;
        let h = body_height * scale;
        
        for j in 0..n_circ {
            let angle = j as f64 / n_circ as f64 * 2.0 * PI;
            let y = w * angle.cos();
            let z = h * angle.sin() + body_height * 0.3;
            verts.push([x, y, z]);
        }
    }
    
    for i in 0..n_body {
        for j in 0..n_circ {
            let j1 = (j + 1) % n_circ;
            let v0 = body_start + i * n_circ + j + 1;
            let v1 = body_start + i * n_circ + j1 + 1;
            let v2 = body_start + (i + 1) * n_circ + j1 + 1;
            let v3 = body_start + (i + 1) * n_circ + j + 1;
            faces.push(vec![v0, v1, v2, v3]);
        }
    }
    
    // 机头封闭
    let nose_center = verts.len();
    verts.push([-body_length * 0.2, 0.0, body_height * 0.3]);
    for j in 0..n_circ {
        let j1 = (j + 1) % n_circ;
        faces.push(vec![nose_center + 1, body_start + j1 + 1, body_start + j + 1]);
    }
    
    // ========================================
    // 尾翼
    // ========================================
    let use_v_tail = design.aspect_ratio > 15.0;  // 高展弦比使用V尾
    
    if use_v_tail {
        // V 型尾翼
        let v_tail_span = wing_span * 0.12;
        let v_tail_chord = root_chord * 0.5;
        let v_tail_angle = 35.0_f64.to_radians();
        
        for side in [-1.0_f64, 1.0] {
            let tail_start = verts.len();
            
            for i in 0..=10 {
                let t = i as f64 / 10.0;
                let span_pos = t * v_tail_span;
                let y = side * span_pos * v_tail_angle.cos();
                let z = span_pos * v_tail_angle.sin() + body_height * 0.5;
                let x = body_length * 0.65 + span_pos * 0.25;
                
                let c = v_tail_chord * (1.0 - t * 0.4);
                let thick = c * 0.10;
                
                verts.push([x, y, z]);
                verts.push([x + c * 0.3, y, z + thick]);
                verts.push([x + c, y, z]);
                verts.push([x + c * 0.3, y, z - thick]);
            }
            
            for i in 0..10 {
                let base = tail_start + i * 4;
                let next = tail_start + (i + 1) * 4;
                faces.push(vec![base + 1, base + 2, next + 2, next + 1]);
                faces.push(vec![base + 2, base + 3, next + 3, next + 2]);
                faces.push(vec![base + 3, base + 4, next + 4, next + 3]);
                faces.push(vec![base + 4, base + 1, next + 1, next + 4]);
            }
        }
    } else {
        // 传统水平尾翼 + 垂直尾翼
        // 水平尾翼
        for side in [-1.0_f64, 1.0] {
            let h_tail_start = verts.len();
            let h_tail_span = wing_span * 0.25;
            let h_tail_chord = root_chord * 0.45;
            
            for i in 0..=8 {
                let t = i as f64 / 8.0;
                let y = side * t * h_tail_span / 2.0;
                let x = body_length * 0.7 + t * h_tail_span / 2.0 * 0.4;
                let z = body_height * 0.6;
                
                let c = h_tail_chord * (1.0 - t * 0.4);
                
                verts.push([x, y, z]);
                verts.push([x + c, y, z]);
            }
            
            for i in 0..8 {
                let base = h_tail_start + i * 2;
                let next = h_tail_start + (i + 1) * 2;
                faces.push(vec![base + 1, base + 2, next + 2, next + 1]);
            }
        }
        
        // 垂直尾翼
        let v_tail_start = verts.len();
        let v_tail_height = wing_span * 0.12;
        let v_tail_chord = root_chord * 0.5;
        
        for i in 0..=8 {
            let t = i as f64 / 8.0;
            let z = body_height * 0.5 + t * v_tail_height;
            let x = body_length * 0.65 + t * v_tail_height * 0.5;
            
            let c = v_tail_chord * (1.0 - t * 0.35);
            
            verts.push([x, 0.0, z]);
            verts.push([x + c, 0.0, z]);
        }
        
        for i in 0..8 {
            let base = v_tail_start + i * 2;
            let next = v_tail_start + (i + 1) * 2;
            faces.push(vec![base + 1, base + 2, next + 2, next + 1]);
        }
    }
    
    // ========================================
    // 小翼 (Winglet)
    // ========================================
    if design.aspect_ratio > 10.0 {
        let winglet_height = wing_span * 0.035;
        let winglet_chord = tip_chord * 0.7;
        
        for side in [-1.0_f64, 1.0] {
            let winglet_start = verts.len();
            let tip_y = side * wing_span / 2.0;
            let tip_x = wing_span / 2.0 * sweep_rad.tan() + tip_chord;
            let tip_z = wing_span / 2.0 * dihedral.sin();
            
            for i in 0..=5 {
                let t = i as f64 / 5.0;
                let z = tip_z + t * winglet_height;
                let x = tip_x - tip_chord + t * winglet_height * 0.15;
                let y = tip_y + side * t * winglet_height * 0.25;
                let c = winglet_chord * (1.0 - t * 0.3);
                
                verts.push([x, y, z]);
                verts.push([x + c, y, z]);
            }
            
            for i in 0..5 {
                let base = winglet_start + i * 2;
                let next = winglet_start + (i + 1) * 2;
                faces.push(vec![base + 1, base + 2, next + 2, next + 1]);
            }
        }
    }
    
    // 写入 OBJ 文件
    let mut file = File::create(filename).unwrap();
    
    writeln!(file, "# S-wind Optimal Aircraft Design").unwrap();
    writeln!(file, "# Design: {}", design.objective.name()).unwrap();
    writeln!(file, "# L/D = {:.2}, AR = {:.1}, Sweep = {:.1}°", 
             design.l_d_ratio, design.aspect_ratio, design.sweep_angle).unwrap();
    writeln!(file, "# Vertices: {}, Faces: {}", verts.len(), faces.len()).unwrap();
    writeln!(file, "mtllib {}.mtl", filename.replace(".obj", "")).unwrap();
    writeln!(file).unwrap();
    
    for v in &verts {
        writeln!(file, "v {:.6} {:.6} {:.6}", v[0], v[1], v[2]).unwrap();
    }
    
    writeln!(file).unwrap();
    writeln!(file, "usemtl aircraft").unwrap();
    
    for face in &faces {
        let s: Vec<String> = face.iter().map(|i| i.to_string()).collect();
        writeln!(file, "f {}", s.join(" ")).unwrap();
    }
    
    // MTL 文件
    let mtl_filename = filename.replace(".obj", ".mtl");
    let mut mtl = File::create(&mtl_filename).unwrap();
    writeln!(mtl, "# Aircraft Material").unwrap();
    writeln!(mtl, "newmtl aircraft").unwrap();
    writeln!(mtl, "Ns 250.0").unwrap();
    writeln!(mtl, "Ka 0.1 0.1 0.1").unwrap();
    writeln!(mtl, "Kd 0.92 0.92 0.95").unwrap();
    writeln!(mtl, "Ks 0.6 0.6 0.6").unwrap();
    writeln!(mtl, "d 1.0").unwrap();
    writeln!(mtl, "illum 2").unwrap();
}

fn generate_stl(design: &AircraftDesign, filename: &str) {
    // 简化版 STL 生成
    let mut file = File::create(filename).unwrap();
    
    writeln!(file, "solid optimal_aircraft_{}", design.objective.short_name()).unwrap();
    
    // 生成简化三角形网格
    let wing_span = design.wing_span;
    let root_chord = design.root_chord;
    let sweep_rad = design.sweep_angle * PI / 180.0;
    
    // 简化机翼表示
    for side in [-1.0_f64, 1.0] {
        let y_root = 0.0;
        let y_tip = side * wing_span / 2.0;
        let x_root = 0.0;
        let x_tip = wing_span / 2.0 * sweep_rad.tan();
        
        // 上表面三角形
        write_stl_triangle(&mut file, 
            [x_root, y_root, 0.0],
            [x_root + root_chord, y_root, 0.0],
            [x_tip + design.tip_chord, y_tip, 0.0],
            [0.0, 0.0, 1.0]);
        
        write_stl_triangle(&mut file,
            [x_root, y_root, 0.0],
            [x_tip + design.tip_chord, y_tip, 0.0],
            [x_tip, y_tip, 0.0],
            [0.0, 0.0, 1.0]);
    }
    
    writeln!(file, "endsolid optimal_aircraft_{}", design.objective.short_name()).unwrap();
}

fn write_stl_triangle(file: &mut File, v1: [f64; 3], v2: [f64; 3], v3: [f64; 3], normal: [f64; 3]) {
    writeln!(file, "  facet normal {} {} {}", normal[0], normal[1], normal[2]).unwrap();
    writeln!(file, "    outer loop").unwrap();
    writeln!(file, "      vertex {} {} {}", v1[0], v1[1], v1[2]).unwrap();
    writeln!(file, "      vertex {} {} {}", v2[0], v2[1], v2[2]).unwrap();
    writeln!(file, "      vertex {} {} {}", v3[0], v3[1], v3[2]).unwrap();
    writeln!(file, "    endloop").unwrap();
    writeln!(file, "  endfacet").unwrap();
}

fn generate_comprehensive_report(designs: &[AircraftDesign], kb: &AerospaceKnowledge) {
    let mut file = File::create("AIRCRAFT_COMPREHENSIVE_REPORT.md").unwrap();
    
    writeln!(file, "# 🛩️ S-wind 完整飞行器设计报告").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "**生成时间**: 2026-01-29").unwrap();
    writeln!(file, "**知识库**: Blondle Aerospace Module").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "---").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "## 设计对比总览").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "| 设计类型 | L/D | AR | 后掠角 | 厚度 | 马赫数 |").unwrap();
    writeln!(file, "|----------|-----|-----|--------|------|--------|").unwrap();
    
    for design in designs {
        writeln!(file, "| {} | {:.1} | {:.1} | {:.0}° | {:.1}% | {:.2} |",
                 design.objective.name(),
                 design.l_d_ratio,
                 design.aspect_ratio,
                 design.sweep_angle,
                 design.thickness * 100.0,
                 design.design_mach).unwrap();
    }
    
    writeln!(file, "").unwrap();
    writeln!(file, "---").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "## 使用的物理定律").unwrap();
    writeln!(file, "").unwrap();
    
    for law in &kb.laws {
        writeln!(file, "### {}", law.name).unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "$${}$$", law.equation_tex).unwrap();
        writeln!(file, "").unwrap();
        writeln!(file, "{}", law.description).unwrap();
        writeln!(file, "").unwrap();
    }
    
    writeln!(file, "---").unwrap();
    writeln!(file, "").unwrap();
    writeln!(file, "## 3D 模型文件").unwrap();
    writeln!(file, "").unwrap();
    
    for design in designs {
        writeln!(file, "- `optimal_aircraft_{}.obj` - {}", 
                 design.objective.short_name(), design.objective.name()).unwrap();
        writeln!(file, "- `optimal_aircraft_{}.stl` - 3D打印格式",
                 design.objective.short_name()).unwrap();
    }
    
    println!("  ✓ 生成: AIRCRAFT_COMPREHENSIVE_REPORT.md");
}
