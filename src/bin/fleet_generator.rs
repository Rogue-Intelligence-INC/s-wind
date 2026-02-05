//! Fleet Generator - 生成多种飞行器 3D 模型
//! 
//! 战斗机 / 直升机 / 大型客机 / 无人机

use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║                    🛩️  FLEET 3D GENERATOR  🚁                         ║");
    println!("║                 战斗机 / 直升机 / 大客机 / 无人机                        ║");
    println!("╚═══════════════════════════════════════════════════════════════════════╝\n");

    // 1. 战斗机 (F-22 风格)
    println!("  [1/4] 🔥 生成战斗机 (F-22 Stealth Fighter)...");
    generate_fighter();
    
    // 2. 直升机
    println!("  [2/4] 🚁 生成直升机 (Attack Helicopter)...");
    generate_helicopter();
    
    // 3. 大型客机 (A380 风格)
    println!("  [3/4] ✈️  生成大型客机 (A380 Superjumbo)...");
    generate_jumbo();
    
    // 4. 无人机
    println!("  [4/4] 🛸 生成无人机 (Quadcopter Drone)...");
    generate_drone();
    
    println!();
    println!("╔═══════════════════════════════════════════════════════════════════════╗");
    println!("║  ✅ 生成完成！                                                        ║");
    println!("║                                                                       ║");
    println!("║  📁 文件列表:                                                         ║");
    println!("║     • fighter.obj     - F-22 隐身战斗机                               ║");
    println!("║     • helicopter.obj  - 武装直升机                                    ║");
    println!("║     • jumbo.obj       - A380 大型客机                                 ║");
    println!("║     • drone.obj       - 四旋翼无人机                                  ║");
    println!("║                                                                       ║");
    println!("║  🎮 在 Blender 中打开: File → Import → Wavefront (.obj)              ║");
    println!("╚═══════════════════════════════════════════════════════════════════════╝");
}

// ============================================================================
// 战斗机 (F-22 风格)
// ============================================================================
fn generate_fighter() {
    let mut verts: Vec<[f64; 3]> = Vec::new();
    let mut faces: Vec<Vec<usize>> = Vec::new();
    
    // 机身参数
    let length = 19.0;
    let width = 4.5;
    let height = 2.5;
    
    // 机身 - 菱形截面
    let n_len = 30;
    for i in 0..=n_len {
        let t = i as f64 / n_len as f64;
        let x = t * length;
        
        // 截面尺寸 (隐身外形)
        let (w, h) = if t < 0.15 {
            // 机头锥形
            (width * t / 0.15 * 0.3, height * t / 0.15 * 0.4)
        } else if t < 0.7 {
            // 中段
            (width * 0.5, height * 0.5)
        } else {
            // 尾部收缩
            let tt = (t - 0.7) / 0.3;
            (width * 0.5 * (1.0 - tt * 0.6), height * 0.5 * (1.0 - tt * 0.4))
        };
        
        // 菱形截面 4点
        verts.push([x, 0.0, h]);           // 上
        verts.push([x, w, 0.0]);           // 右
        verts.push([x, 0.0, -h * 0.3]);    // 下
        verts.push([x, -w, 0.0]);          // 左
    }
    
    // 机身面
    for i in 0..n_len {
        for j in 0..4 {
            let j1 = (j + 1) % 4;
            let v0 = i * 4 + j + 1;
            let v1 = i * 4 + j1 + 1;
            let v2 = (i + 1) * 4 + j1 + 1;
            let v3 = (i + 1) * 4 + j + 1;
            faces.push(vec![v0, v1, v2, v3]);
        }
    }
    
    // 三角翼
    let wing_start = verts.len();
    let wing_x = 8.0;
    let wing_span = 13.5;
    let wing_chord = 7.0;
    
    // 右翼
    verts.push([wing_x, 0.0, 0.0]);
    verts.push([wing_x + wing_chord, 0.0, 0.0]);
    verts.push([wing_x + wing_chord * 0.8, wing_span / 2.0, 0.0]);
    faces.push(vec![wing_start + 1, wing_start + 2, wing_start + 3]);
    
    // 左翼
    verts.push([wing_x, 0.0, 0.0]);
    verts.push([wing_x + wing_chord, 0.0, 0.0]);
    verts.push([wing_x + wing_chord * 0.8, -wing_span / 2.0, 0.0]);
    faces.push(vec![wing_start + 4, wing_start + 6, wing_start + 5]);
    
    // 垂尾 (双垂尾外倾)
    let vtail_start = verts.len();
    let vtail_x = 14.0;
    let vtail_height = 2.8;
    let cant_angle = 25.0_f64.to_radians();
    
    for side in [-1.0_f64, 1.0] {
        let y_base = side * 1.5;
        let y_top = y_base + side * vtail_height * cant_angle.sin();
        let z_top = vtail_height * cant_angle.cos();
        
        let idx = verts.len();
        verts.push([vtail_x, y_base, 0.5]);
        verts.push([vtail_x + 4.0, y_base, 0.5]);
        verts.push([vtail_x + 2.5, y_top, z_top]);
        faces.push(vec![idx + 1, idx + 2, idx + 3]);
    }
    
    // 座舱盖
    let canopy_start = verts.len();
    verts.push([3.0, 0.0, 1.0]);
    verts.push([5.0, 0.0, 1.5]);
    verts.push([7.0, 0.0, 1.2]);
    verts.push([5.0, 0.6, 1.2]);
    verts.push([5.0, -0.6, 1.2]);
    faces.push(vec![canopy_start + 1, canopy_start + 2, canopy_start + 4]);
    faces.push(vec![canopy_start + 1, canopy_start + 5, canopy_start + 2]);
    faces.push(vec![canopy_start + 2, canopy_start + 3, canopy_start + 4]);
    faces.push(vec![canopy_start + 2, canopy_start + 5, canopy_start + 3]);
    
    write_obj(&verts, &faces, "fighter.obj", "fighter.mtl", [0.3, 0.3, 0.35]);
    println!("       → fighter.obj ({} 顶点, {} 面)", verts.len(), faces.len());
}

// ============================================================================
// 直升机
// ============================================================================
fn generate_helicopter() {
    let mut verts: Vec<[f64; 3]> = Vec::new();
    let mut faces: Vec<Vec<usize>> = Vec::new();
    
    // 机身 (椭球形)
    let n_circ = 16;
    let n_len = 20;
    let body_length = 12.0;
    let body_width = 2.0;
    let body_height = 2.5;
    
    for i in 0..=n_len {
        let t = i as f64 / n_len as f64;
        let x = t * body_length - body_length * 0.3;
        
        // 椭球形状
        let r_scale = (1.0 - (2.0 * t - 1.0).powi(2)).sqrt().max(0.1);
        
        for j in 0..n_circ {
            let angle = j as f64 / n_circ as f64 * 2.0 * PI;
            let y = body_width * r_scale * angle.cos();
            let z = body_height * r_scale * angle.sin();
            verts.push([x, y, z]);
        }
    }
    
    for i in 0..n_len {
        for j in 0..n_circ {
            let j1 = (j + 1) % n_circ;
            let v0 = i * n_circ + j + 1;
            let v1 = i * n_circ + j1 + 1;
            let v2 = (i + 1) * n_circ + j1 + 1;
            let v3 = (i + 1) * n_circ + j + 1;
            faces.push(vec![v0, v1, v2, v3]);
        }
    }
    
    // 尾梁
    let tail_start = verts.len();
    let tail_length = 8.0;
    let tail_segments = 10;
    
    for i in 0..=tail_segments {
        let t = i as f64 / tail_segments as f64;
        let x = body_length * 0.6 + t * tail_length;
        let r = 0.4 * (1.0 - t * 0.5);
        
        for j in 0..8 {
            let angle = j as f64 / 8.0 * 2.0 * PI;
            verts.push([x, r * angle.cos(), r * angle.sin() + 1.0]);
        }
    }
    
    for i in 0..tail_segments {
        for j in 0..8 {
            let j1 = (j + 1) % 8;
            let v0 = tail_start + i * 8 + j + 1;
            let v1 = tail_start + i * 8 + j1 + 1;
            let v2 = tail_start + (i + 1) * 8 + j1 + 1;
            let v3 = tail_start + (i + 1) * 8 + j + 1;
            faces.push(vec![v0, v1, v2, v3]);
        }
    }
    
    // 主旋翼 (4片桨叶)
    let rotor_start = verts.len();
    let rotor_radius = 7.0;
    let rotor_z = body_height + 0.5;
    
    for blade in 0..4 {
        let angle = blade as f64 * PI / 2.0;
        let idx = verts.len();
        
        // 桨叶
        verts.push([0.0, 0.0, rotor_z]);
        verts.push([rotor_radius * angle.cos(), rotor_radius * angle.sin(), rotor_z]);
        verts.push([rotor_radius * angle.cos() - 0.3 * angle.sin(), 
                    rotor_radius * angle.sin() + 0.3 * angle.cos(), rotor_z]);
        faces.push(vec![idx + 1, idx + 2, idx + 3]);
    }
    
    // 主旋翼轴
    let shaft_idx = verts.len();
    verts.push([0.0, 0.0, body_height]);
    verts.push([0.0, 0.0, rotor_z + 0.3]);
    verts.push([0.2, 0.0, body_height]);
    verts.push([0.2, 0.0, rotor_z + 0.3]);
    faces.push(vec![shaft_idx + 1, shaft_idx + 2, shaft_idx + 4, shaft_idx + 3]);
    
    // 尾旋翼
    let tail_rotor_x = body_length * 0.6 + tail_length;
    let tail_rotor_start = verts.len();
    
    for blade in 0..4 {
        let angle = blade as f64 * PI / 2.0;
        let idx = verts.len();
        let r = 1.2;
        
        verts.push([tail_rotor_x, 0.5, 1.0]);
        verts.push([tail_rotor_x, 0.5 + r * angle.cos(), 1.0 + r * angle.sin()]);
        verts.push([tail_rotor_x, 0.5 + r * angle.cos() - 0.1, 1.0 + r * angle.sin() + 0.1]);
        faces.push(vec![idx + 1, idx + 2, idx + 3]);
    }
    
    // 垂尾
    let fin_idx = verts.len();
    verts.push([tail_rotor_x - 1.0, 0.0, 1.0]);
    verts.push([tail_rotor_x, 0.0, 1.0]);
    verts.push([tail_rotor_x - 0.5, 0.0, 3.0]);
    faces.push(vec![fin_idx + 1, fin_idx + 2, fin_idx + 3]);
    
    // 起落架 (滑橇式)
    let skid_idx = verts.len();
    for side in [-1.0_f64, 1.0] {
        let y = side * 1.2;
        let idx = verts.len();
        verts.push([-2.0, y, -body_height]);
        verts.push([4.0, y, -body_height]);
        verts.push([4.0, y, -body_height + 0.1]);
        verts.push([-2.0, y, -body_height + 0.1]);
        faces.push(vec![idx + 1, idx + 2, idx + 3, idx + 4]);
        
        // 支柱
        let strut_idx = verts.len();
        verts.push([0.0, y, -body_height]);
        verts.push([0.0, y * 0.8, 0.0]);
        verts.push([0.1, y, -body_height]);
        verts.push([0.1, y * 0.8, 0.0]);
        faces.push(vec![strut_idx + 1, strut_idx + 2, strut_idx + 4, strut_idx + 3]);
    }
    
    write_obj(&verts, &faces, "helicopter.obj", "helicopter.mtl", [0.2, 0.25, 0.2]);
    println!("       → helicopter.obj ({} 顶点, {} 面)", verts.len(), faces.len());
}

// ============================================================================
// 大型客机 (A380 风格)
// ============================================================================
fn generate_jumbo() {
    let mut verts: Vec<[f64; 3]> = Vec::new();
    let mut faces: Vec<Vec<usize>> = Vec::new();
    
    // 巨大的双层机身
    let length = 73.0;
    let diameter = 7.1;  // 双层宽体
    let n_circ = 32;
    let n_len = 60;
    
    for i in 0..=n_len {
        let t = i as f64 / n_len as f64;
        let x = t * length;
        
        let r = if t < 0.08 {
            // 机头
            diameter / 2.0 * (t / 0.08).sqrt()
        } else if t > 0.88 {
            // 尾部
            diameter / 2.0 * (1.0 - (t - 0.88) / 0.12 * 0.6)
        } else {
            diameter / 2.0
        };
        
        let z_offset = if t > 0.88 {
            (t - 0.88) / 0.12 * 2.0
        } else { 0.0 };
        
        for j in 0..n_circ {
            let angle = j as f64 / n_circ as f64 * 2.0 * PI;
            verts.push([x, r * angle.cos(), r * angle.sin() + z_offset]);
        }
    }
    
    for i in 0..n_len {
        for j in 0..n_circ {
            let j1 = (j + 1) % n_circ;
            let v0 = i * n_circ + j + 1;
            let v1 = i * n_circ + j1 + 1;
            let v2 = (i + 1) * n_circ + j1 + 1;
            let v3 = (i + 1) * n_circ + j + 1;
            faces.push(vec![v0, v1, v2, v3]);
        }
    }
    
    // 巨大的机翼
    let wing_start = verts.len();
    let wing_x = 28.0;
    let wing_span = 79.8;
    let wing_root = 17.0;
    let wing_tip = 4.0;
    let sweep = 33.5_f64.to_radians();
    
    let n_span = 25;
    for i in 0..=n_span {
        let t = i as f64 / n_span as f64;
        let y = t * wing_span / 2.0;
        let chord = wing_root + t * (wing_tip - wing_root);
        let x_le = wing_x + y * sweep.tan();
        let thick = chord * 0.12;
        let z_dihedral = y * 0.05;  // 5% 上反
        
        verts.push([x_le, y, z_dihedral]);
        verts.push([x_le + chord * 0.35, y, z_dihedral + thick]);
        verts.push([x_le + chord, y, z_dihedral]);
        verts.push([x_le + chord * 0.35, y, z_dihedral - thick * 0.5]);
    }
    
    for i in 0..n_span {
        let base = wing_start + i * 4;
        let next = wing_start + (i + 1) * 4;
        faces.push(vec![base + 1, base + 2, next + 2, next + 1]);
        faces.push(vec![base + 2, base + 3, next + 3, next + 2]);
        faces.push(vec![base + 3, base + 4, next + 4, next + 3]);
        faces.push(vec![base + 4, base + 1, next + 1, next + 4]);
    }
    
    // 左翼 (镜像)
    let left_wing_start = verts.len();
    for i in 0..=n_span {
        let t = i as f64 / n_span as f64;
        let y = -t * wing_span / 2.0;
        let chord = wing_root + t * (wing_tip - wing_root);
        let x_le = wing_x + t * wing_span / 2.0 * sweep.tan();
        let thick = chord * 0.12;
        let z_dihedral = -y * 0.05;
        
        verts.push([x_le, y, z_dihedral]);
        verts.push([x_le + chord * 0.35, y, z_dihedral + thick]);
        verts.push([x_le + chord, y, z_dihedral]);
        verts.push([x_le + chord * 0.35, y, z_dihedral - thick * 0.5]);
    }
    
    for i in 0..n_span {
        let base = left_wing_start + i * 4;
        let next = left_wing_start + (i + 1) * 4;
        faces.push(vec![base + 1, next + 1, next + 2, base + 2]);
        faces.push(vec![base + 2, next + 2, next + 3, base + 3]);
        faces.push(vec![base + 3, next + 3, next + 4, base + 4]);
        faces.push(vec![base + 4, next + 4, next + 1, base + 1]);
    }
    
    // 4台发动机
    let engine_positions = [
        (34.0, 12.0), (34.0, -12.0),
        (32.0, 22.0), (32.0, -22.0),
    ];
    
    for (ex, ey) in engine_positions {
        let eng_start = verts.len();
        let eng_r = 1.5;
        let eng_len = 5.0;
        let n_eng = 12;
        
        for i in 0..=4 {
            let t = i as f64 / 4.0;
            let x = ex + t * eng_len;
            let r = if t < 0.2 { eng_r * (0.9 + t * 0.5) }
                    else if t > 0.8 { eng_r * (1.0 - (t - 0.8)) }
                    else { eng_r };
            
            for j in 0..n_eng {
                let angle = j as f64 / n_eng as f64 * 2.0 * PI;
                verts.push([x, ey + r * angle.cos(), -diameter / 2.0 - 1.0 + r * angle.sin()]);
            }
        }
        
        for i in 0..4 {
            for j in 0..n_eng {
                let j1 = (j + 1) % n_eng;
                let v0 = eng_start + i * n_eng + j + 1;
                let v1 = eng_start + i * n_eng + j1 + 1;
                let v2 = eng_start + (i + 1) * n_eng + j1 + 1;
                let v3 = eng_start + (i + 1) * n_eng + j + 1;
                faces.push(vec![v0, v1, v2, v3]);
            }
        }
    }
    
    // 尾翼
    let h_stab_start = verts.len();
    let h_stab_x = 62.0;
    let h_stab_span = 25.0;
    
    for side in [-1.0_f64, 1.0] {
        for i in 0..=8 {
            let t = i as f64 / 8.0;
            let y = side * t * h_stab_span / 2.0;
            let chord = 6.0 - t * 4.0;
            let x_le = h_stab_x + t * h_stab_span / 2.0 * 0.5;
            
            let idx = verts.len();
            verts.push([x_le, y, 2.0]);
            verts.push([x_le + chord, y, 2.0]);
        }
    }
    
    // 垂尾
    let v_stab_start = verts.len();
    for i in 0..=8 {
        let t = i as f64 / 8.0;
        let z = 2.0 + t * 8.0;
        let chord = 10.0 - t * 6.0;
        let x_le = 58.0 + t * 8.0 * 0.6;
        
        verts.push([x_le, 0.0, z]);
        verts.push([x_le + chord, 0.0, z]);
    }
    
    for i in 0..8 {
        let v0 = v_stab_start + i * 2 + 1;
        let v1 = v_stab_start + i * 2 + 2;
        let v2 = v_stab_start + (i + 1) * 2 + 2;
        let v3 = v_stab_start + (i + 1) * 2 + 1;
        faces.push(vec![v0, v1, v2, v3]);
    }
    
    write_obj(&verts, &faces, "jumbo.obj", "jumbo.mtl", [0.95, 0.95, 0.98]);
    println!("       → jumbo.obj ({} 顶点, {} 面)", verts.len(), faces.len());
}

// ============================================================================
// 四旋翼无人机
// ============================================================================
fn generate_drone() {
    let mut verts: Vec<[f64; 3]> = Vec::new();
    let mut faces: Vec<Vec<usize>> = Vec::new();
    
    // 中心机身 (方形)
    let body_size = 0.15;
    let body_height = 0.08;
    
    // 8个顶点的立方体
    verts.push([-body_size, -body_size, 0.0]);
    verts.push([body_size, -body_size, 0.0]);
    verts.push([body_size, body_size, 0.0]);
    verts.push([-body_size, body_size, 0.0]);
    verts.push([-body_size, -body_size, body_height]);
    verts.push([body_size, -body_size, body_height]);
    verts.push([body_size, body_size, body_height]);
    verts.push([-body_size, body_size, body_height]);
    
    // 6个面
    faces.push(vec![1, 2, 3, 4]);  // 底
    faces.push(vec![5, 8, 7, 6]);  // 顶
    faces.push(vec![1, 5, 6, 2]);  // 前
    faces.push(vec![3, 7, 8, 4]);  // 后
    faces.push(vec![2, 6, 7, 3]);  // 右
    faces.push(vec![4, 8, 5, 1]);  // 左
    
    // 4个机臂
    let arm_length = 0.25;
    let arm_positions = [
        (1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)
    ];
    
    for (dx, dy) in arm_positions {
        let arm_start = verts.len();
        let arm_end_x = dx * arm_length;
        let arm_end_y = dy * arm_length;
        let arm_thick = 0.02;
        
        // 机臂 (简化为细长方块)
        verts.push([0.0, 0.0, body_height / 2.0 - arm_thick]);
        verts.push([arm_end_x, arm_end_y, body_height / 2.0 - arm_thick]);
        verts.push([arm_end_x, arm_end_y, body_height / 2.0 + arm_thick]);
        verts.push([0.0, 0.0, body_height / 2.0 + arm_thick]);
        faces.push(vec![arm_start + 1, arm_start + 2, arm_start + 3, arm_start + 4]);
        
        // 电机 (圆柱)
        let motor_start = verts.len();
        let motor_r = 0.03;
        let motor_h = 0.04;
        let n_motor = 8;
        
        for i in 0..=1 {
            let z = body_height / 2.0 + i as f64 * motor_h;
            for j in 0..n_motor {
                let angle = j as f64 / n_motor as f64 * 2.0 * PI;
                verts.push([
                    arm_end_x + motor_r * angle.cos(),
                    arm_end_y + motor_r * angle.sin(),
                    z
                ]);
            }
        }
        
        for j in 0..n_motor {
            let j1 = (j + 1) % n_motor;
            let v0 = motor_start + j + 1;
            let v1 = motor_start + j1 + 1;
            let v2 = motor_start + n_motor + j1 + 1;
            let v3 = motor_start + n_motor + j + 1;
            faces.push(vec![v0, v1, v2, v3]);
        }
        
        // 螺旋桨 (2片)
        let prop_start = verts.len();
        let prop_r = 0.12;
        let prop_z = body_height / 2.0 + motor_h + 0.01;
        
        for blade in 0..2 {
            let angle = blade as f64 * PI;
            let idx = verts.len();
            
            verts.push([arm_end_x, arm_end_y, prop_z]);
            verts.push([
                arm_end_x + prop_r * angle.cos(),
                arm_end_y + prop_r * angle.sin(),
                prop_z
            ]);
            verts.push([
                arm_end_x + prop_r * angle.cos() - 0.01 * angle.sin(),
                arm_end_y + prop_r * angle.sin() + 0.01 * angle.cos(),
                prop_z
            ]);
            faces.push(vec![idx + 1, idx + 2, idx + 3]);
        }
    }
    
    // 起落架 (4个小脚)
    for (dx, dy) in [(1.0, 1.0), (1.0, -1.0), (-1.0, 1.0), (-1.0, -1.0)].iter() {
        let leg_start = verts.len();
        let leg_x = dx * body_size * 0.8;
        let leg_y = dy * body_size * 0.8;
        let leg_h = 0.06;
        
        verts.push([leg_x - 0.01, leg_y - 0.01, 0.0]);
        verts.push([leg_x + 0.01, leg_y - 0.01, 0.0]);
        verts.push([leg_x + 0.01, leg_y + 0.01, 0.0]);
        verts.push([leg_x - 0.01, leg_y + 0.01, 0.0]);
        verts.push([leg_x - 0.01, leg_y - 0.01, -leg_h]);
        verts.push([leg_x + 0.01, leg_y - 0.01, -leg_h]);
        verts.push([leg_x + 0.01, leg_y + 0.01, -leg_h]);
        verts.push([leg_x - 0.01, leg_y + 0.01, -leg_h]);
        
        faces.push(vec![leg_start + 1, leg_start + 2, leg_start + 6, leg_start + 5]);
        faces.push(vec![leg_start + 2, leg_start + 3, leg_start + 7, leg_start + 6]);
        faces.push(vec![leg_start + 3, leg_start + 4, leg_start + 8, leg_start + 7]);
        faces.push(vec![leg_start + 4, leg_start + 1, leg_start + 5, leg_start + 8]);
    }
    
    // 摄像头 (底部小球)
    let cam_start = verts.len();
    let cam_r = 0.025;
    let cam_z = -0.02;
    let n_cam = 8;
    
    for i in 0..=4 {
        let phi = i as f64 / 4.0 * PI / 2.0;
        let r = cam_r * phi.sin();
        let z = cam_z - cam_r * phi.cos();
        
        for j in 0..n_cam {
            let theta = j as f64 / n_cam as f64 * 2.0 * PI;
            verts.push([r * theta.cos(), r * theta.sin(), z]);
        }
    }
    
    for i in 0..4 {
        for j in 0..n_cam {
            let j1 = (j + 1) % n_cam;
            let v0 = cam_start + i * n_cam + j + 1;
            let v1 = cam_start + i * n_cam + j1 + 1;
            let v2 = cam_start + (i + 1) * n_cam + j1 + 1;
            let v3 = cam_start + (i + 1) * n_cam + j + 1;
            faces.push(vec![v0, v1, v2, v3]);
        }
    }
    
    write_obj(&verts, &faces, "drone.obj", "drone.mtl", [0.1, 0.1, 0.1]);
    println!("       → drone.obj ({} 顶点, {} 面)", verts.len(), faces.len());
}

// ============================================================================
// 辅助函数
// ============================================================================
fn write_obj(verts: &[[f64; 3]], faces: &[Vec<usize>], obj_path: &str, mtl_path: &str, color: [f64; 3]) {
    let mut file = File::create(obj_path).unwrap();
    
    writeln!(file, "# Generated by S-wind Fleet Generator").unwrap();
    writeln!(file, "# Vertices: {}, Faces: {}", verts.len(), faces.len()).unwrap();
    writeln!(file, "mtllib {}", mtl_path).unwrap();
    writeln!(file).unwrap();
    
    for v in verts {
        writeln!(file, "v {:.6} {:.6} {:.6}", v[0], v[1], v[2]).unwrap();
    }
    
    writeln!(file).unwrap();
    writeln!(file, "usemtl material").unwrap();
    
    for face in faces {
        let s: Vec<String> = face.iter().map(|i| i.to_string()).collect();
        writeln!(file, "f {}", s.join(" ")).unwrap();
    }
    
    // MTL file
    let mut mtl = File::create(mtl_path).unwrap();
    writeln!(mtl, "newmtl material").unwrap();
    writeln!(mtl, "Ns 200.0").unwrap();
    writeln!(mtl, "Ka 0.1 0.1 0.1").unwrap();
    writeln!(mtl, "Kd {:.2} {:.2} {:.2}", color[0], color[1], color[2]).unwrap();
    writeln!(mtl, "Ks 0.5 0.5 0.5").unwrap();
    writeln!(mtl, "d 1.0").unwrap();
    writeln!(mtl, "illum 2").unwrap();
}
