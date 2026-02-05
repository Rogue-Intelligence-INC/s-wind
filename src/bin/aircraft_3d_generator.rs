//! Aircraft 3D Model Generator
//! 
//! Generates a parametric aircraft model as OBJ file
//! Commercial jet style (Boeing 737 / A320 inspired)

use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║           AIRCRAFT 3D MODEL GENERATOR                         ║");
    println!("║           Output: Wavefront OBJ + MTL                         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    let params = AircraftParams {
        // Fuselage
        fuselage_length: 38.0,
        fuselage_diameter: 3.8,
        nose_length: 6.0,
        tail_length: 8.0,
        cockpit_height: 2.8,
        
        // Wings
        wing_span: 35.0,
        wing_root_chord: 6.5,
        wing_tip_chord: 1.8,
        wing_sweep: 25.0,           // degrees
        wing_dihedral: 5.0,         // degrees
        wing_position_x: 15.0,      // from nose
        wing_thickness: 0.45,       // as ratio of chord
        
        // Horizontal stabilizer
        h_stab_span: 12.0,
        h_stab_root_chord: 3.5,
        h_stab_tip_chord: 1.2,
        h_stab_sweep: 30.0,
        h_stab_position_x: 34.0,
        
        // Vertical stabilizer
        v_stab_height: 6.0,
        v_stab_root_chord: 5.5,
        v_stab_tip_chord: 2.0,
        v_stab_sweep: 35.0,
        v_stab_position_x: 32.0,
        
        // Engines
        engine_diameter: 1.8,
        engine_length: 3.5,
        engine_y_position: 6.0,     // from centerline
        engine_x_position: 17.0,    // from nose
        
        // Landing gear (simplified)
        main_gear_x: 18.0,
        nose_gear_x: 4.0,
        gear_height: 2.5,
    };

    println!("  Aircraft Parameters:");
    println!("    Fuselage: {:.1}m length, {:.1}m diameter", 
             params.fuselage_length, params.fuselage_diameter);
    println!("    Wing: {:.1}m span, {:.0}° sweep", 
             params.wing_span, params.wing_sweep);
    println!("    Engines: 2x {:.1}m diameter turbofan", params.engine_diameter);
    println!();

    let mesh = generate_aircraft_mesh(&params);
    
    println!("  Generated {} vertices, {} faces", mesh.vertices.len(), mesh.faces.len());
    
    let obj_path = "aircraft_model.obj";
    let mtl_path = "aircraft_model.mtl";
    
    write_obj_file(&mesh, obj_path, mtl_path).expect("Failed to write OBJ");
    write_mtl_file(mtl_path).expect("Failed to write MTL");
    
    println!();
    println!("  ✓ Generated: {}", obj_path);
    println!("  ✓ Generated: {}", mtl_path);
    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Open in: Blender, FreeCAD, MeshLab                           ║");
    println!("║  Blender: File -> Import -> Wavefront (.obj)                  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

struct AircraftParams {
    fuselage_length: f64,
    fuselage_diameter: f64,
    nose_length: f64,
    tail_length: f64,
    cockpit_height: f64,
    
    wing_span: f64,
    wing_root_chord: f64,
    wing_tip_chord: f64,
    wing_sweep: f64,
    wing_dihedral: f64,
    wing_position_x: f64,
    wing_thickness: f64,
    
    h_stab_span: f64,
    h_stab_root_chord: f64,
    h_stab_tip_chord: f64,
    h_stab_sweep: f64,
    h_stab_position_x: f64,
    
    v_stab_height: f64,
    v_stab_root_chord: f64,
    v_stab_tip_chord: f64,
    v_stab_sweep: f64,
    v_stab_position_x: f64,
    
    engine_diameter: f64,
    engine_length: f64,
    engine_y_position: f64,
    engine_x_position: f64,
    
    main_gear_x: f64,
    nose_gear_x: f64,
    gear_height: f64,
}

struct Mesh {
    vertices: Vec<[f64; 3]>,
    faces: Vec<Vec<usize>>,
}

fn generate_aircraft_mesh(p: &AircraftParams) -> Mesh {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    
    // Generate fuselage
    generate_fuselage(p, &mut vertices, &mut faces);
    
    // Generate wings (both sides)
    generate_wing(p, &mut vertices, &mut faces, 1.0);  // Right
    generate_wing(p, &mut vertices, &mut faces, -1.0); // Left
    
    // Generate horizontal stabilizer
    generate_h_stab(p, &mut vertices, &mut faces, 1.0);
    generate_h_stab(p, &mut vertices, &mut faces, -1.0);
    
    // Generate vertical stabilizer
    generate_v_stab(p, &mut vertices, &mut faces);
    
    // Generate engines
    generate_engine(p, &mut vertices, &mut faces, 1.0);
    generate_engine(p, &mut vertices, &mut faces, -1.0);
    
    // Generate landing gear (simplified)
    generate_landing_gear(p, &mut vertices, &mut faces);
    
    Mesh { vertices, faces }
}

fn generate_fuselage(p: &AircraftParams, vertices: &mut Vec<[f64; 3]>, faces: &mut Vec<Vec<usize>>) {
    let n_circ = 24;  // Points around circumference
    let n_len = 50;   // Points along length
    let r = p.fuselage_diameter / 2.0;
    
    let v_start = vertices.len();
    
    // Generate fuselage vertices
    for i in 0..=n_len {
        let t = i as f64 / n_len as f64;
        let x = t * p.fuselage_length;
        
        // Radius varies along length
        let local_r = get_fuselage_radius(x, p);
        let z_offset = get_fuselage_z_offset(x, p);
        
        for j in 0..n_circ {
            let angle = j as f64 / n_circ as f64 * 2.0 * PI;
            let y = local_r * angle.cos();
            let z = local_r * angle.sin() + z_offset;
            vertices.push([x, y, z]);
        }
    }
    
    // Generate fuselage faces
    for i in 0..n_len {
        for j in 0..n_circ {
            let j1 = (j + 1) % n_circ;
            let v0 = v_start + i * n_circ + j + 1;
            let v1 = v_start + i * n_circ + j1 + 1;
            let v2 = v_start + (i + 1) * n_circ + j1 + 1;
            let v3 = v_start + (i + 1) * n_circ + j + 1;
            faces.push(vec![v0, v1, v2, v3]);
        }
    }
    
    // Nose cap
    let nose_center = vertices.len();
    vertices.push([0.0, 0.0, get_fuselage_z_offset(0.0, p)]);
    for j in 0..n_circ {
        let j1 = (j + 1) % n_circ;
        faces.push(vec![nose_center + 1, v_start + j + 1, v_start + j1 + 1]);
    }
    
    // Tail cap
    let tail_center = vertices.len();
    let tail_z = get_fuselage_z_offset(p.fuselage_length, p);
    vertices.push([p.fuselage_length, 0.0, tail_z]);
    let tail_start = v_start + n_len * n_circ;
    for j in 0..n_circ {
        let j1 = (j + 1) % n_circ;
        faces.push(vec![tail_center + 1, tail_start + j1 + 1, tail_start + j + 1]);
    }
}

fn get_fuselage_radius(x: f64, p: &AircraftParams) -> f64 {
    let r = p.fuselage_diameter / 2.0;
    
    if x < p.nose_length {
        // Nose taper (elliptical)
        let t = x / p.nose_length;
        r * (1.0 - (1.0 - t).powi(2)).sqrt()
    } else if x > p.fuselage_length - p.tail_length {
        // Tail taper
        let t = (x - (p.fuselage_length - p.tail_length)) / p.tail_length;
        r * (1.0 - t * 0.7)
    } else {
        // Constant section
        r
    }
}

fn get_fuselage_z_offset(x: f64, p: &AircraftParams) -> f64 {
    // Fuselage rises toward tail (typical transport)
    if x > p.fuselage_length - p.tail_length {
        let t = (x - (p.fuselage_length - p.tail_length)) / p.tail_length;
        t * 1.5  // Rise 1.5m at tail
    } else {
        0.0
    }
}

fn generate_wing(p: &AircraftParams, vertices: &mut Vec<[f64; 3]>, faces: &mut Vec<Vec<usize>>, side: f64) {
    let n_span = 20;
    let v_start = vertices.len();
    
    let sweep_rad = p.wing_sweep * PI / 180.0;
    let dihedral_rad = p.wing_dihedral * PI / 180.0;
    
    for i in 0..=n_span {
        let t = i as f64 / n_span as f64;
        let y = side * t * p.wing_span / 2.0;
        let z_dihedral = t * p.wing_span / 2.0 * dihedral_rad.sin();
        
        // Chord at this span position (linear taper)
        let chord = p.wing_root_chord + t * (p.wing_tip_chord - p.wing_root_chord);
        
        // Leading edge position (swept back)
        let x_le = p.wing_position_x + t * p.wing_span / 2.0 * sweep_rad.tan();
        
        // Airfoil points (simplified NACA-like)
        let thickness = chord * p.wing_thickness;
        
        // Leading edge
        vertices.push([x_le, y, z_dihedral]);
        // Upper surface
        vertices.push([x_le + chord * 0.3, y, z_dihedral + thickness * 0.5]);
        // Trailing edge
        vertices.push([x_le + chord, y, z_dihedral]);
        // Lower surface
        vertices.push([x_le + chord * 0.3, y, z_dihedral - thickness * 0.3]);
    }
    
    // Wing faces
    for i in 0..n_span {
        let base = v_start + i * 4;
        let next = v_start + (i + 1) * 4;
        
        // Upper surface
        faces.push(vec![base + 1, base + 2, next + 2, next + 1]);
        faces.push(vec![base + 2, base + 3, next + 3, next + 2]);
        
        // Lower surface
        faces.push(vec![base + 1, next + 1, next + 4, base + 4]);
        faces.push(vec![base + 4, next + 4, next + 3, base + 3]);
    }
    
    // Wing tip cap
    let tip_base = v_start + n_span * 4;
    faces.push(vec![tip_base + 1, tip_base + 2, tip_base + 3, tip_base + 4]);
}

fn generate_h_stab(p: &AircraftParams, vertices: &mut Vec<[f64; 3]>, faces: &mut Vec<Vec<usize>>, side: f64) {
    let n_span = 10;
    let v_start = vertices.len();
    
    let sweep_rad = p.h_stab_sweep * PI / 180.0;
    let z_base = get_fuselage_z_offset(p.h_stab_position_x, p) + p.fuselage_diameter * 0.3;
    
    for i in 0..=n_span {
        let t = i as f64 / n_span as f64;
        let y = side * t * p.h_stab_span / 2.0;
        
        let chord = p.h_stab_root_chord + t * (p.h_stab_tip_chord - p.h_stab_root_chord);
        let x_le = p.h_stab_position_x + t * p.h_stab_span / 2.0 * sweep_rad.tan();
        let thickness = chord * 0.12;
        
        vertices.push([x_le, y, z_base]);
        vertices.push([x_le + chord * 0.3, y, z_base + thickness]);
        vertices.push([x_le + chord, y, z_base]);
        vertices.push([x_le + chord * 0.3, y, z_base - thickness * 0.5]);
    }
    
    for i in 0..n_span {
        let base = v_start + i * 4;
        let next = v_start + (i + 1) * 4;
        faces.push(vec![base + 1, base + 2, next + 2, next + 1]);
        faces.push(vec![base + 2, base + 3, next + 3, next + 2]);
        faces.push(vec![base + 1, next + 1, next + 4, base + 4]);
        faces.push(vec![base + 4, next + 4, next + 3, base + 3]);
    }
}

fn generate_v_stab(p: &AircraftParams, vertices: &mut Vec<[f64; 3]>, faces: &mut Vec<Vec<usize>>) {
    let n_height = 10;
    let v_start = vertices.len();
    
    let sweep_rad = p.v_stab_sweep * PI / 180.0;
    let z_base = get_fuselage_z_offset(p.v_stab_position_x, p) + p.fuselage_diameter * 0.4;
    
    for i in 0..=n_height {
        let t = i as f64 / n_height as f64;
        let z = z_base + t * p.v_stab_height;
        
        let chord = p.v_stab_root_chord + t * (p.v_stab_tip_chord - p.v_stab_root_chord);
        let x_le = p.v_stab_position_x + t * p.v_stab_height * sweep_rad.tan();
        let thickness = chord * 0.10;
        
        vertices.push([x_le, 0.0, z]);
        vertices.push([x_le + chord * 0.3, thickness, z]);
        vertices.push([x_le + chord, 0.0, z]);
        vertices.push([x_le + chord * 0.3, -thickness, z]);
    }
    
    for i in 0..n_height {
        let base = v_start + i * 4;
        let next = v_start + (i + 1) * 4;
        faces.push(vec![base + 1, base + 2, next + 2, next + 1]);
        faces.push(vec![base + 2, base + 3, next + 3, next + 2]);
        faces.push(vec![base + 1, next + 1, next + 4, base + 4]);
        faces.push(vec![base + 4, next + 4, next + 3, base + 3]);
    }
}

fn generate_engine(p: &AircraftParams, vertices: &mut Vec<[f64; 3]>, faces: &mut Vec<Vec<usize>>, side: f64) {
    let n_circ = 16;
    let n_len = 8;
    let v_start = vertices.len();
    
    let r = p.engine_diameter / 2.0;
    let y_pos = side * p.engine_y_position;
    let z_pos = -p.fuselage_diameter * 0.3;  // Below wing
    
    // Nacelle
    for i in 0..=n_len {
        let t = i as f64 / n_len as f64;
        let x = p.engine_x_position + t * p.engine_length;
        
        // Radius varies (intake to exhaust)
        let local_r = if t < 0.2 {
            r * (0.8 + t)  // Intake
        } else if t > 0.8 {
            r * (1.0 - (t - 0.8) * 0.5)  // Exhaust
        } else {
            r
        };
        
        for j in 0..n_circ {
            let angle = j as f64 / n_circ as f64 * 2.0 * PI;
            let y = y_pos + local_r * angle.cos();
            let z = z_pos + local_r * angle.sin();
            vertices.push([x, y, z]);
        }
    }
    
    // Nacelle faces
    for i in 0..n_len {
        for j in 0..n_circ {
            let j1 = (j + 1) % n_circ;
            let v0 = v_start + i * n_circ + j + 1;
            let v1 = v_start + i * n_circ + j1 + 1;
            let v2 = v_start + (i + 1) * n_circ + j1 + 1;
            let v3 = v_start + (i + 1) * n_circ + j + 1;
            faces.push(vec![v0, v1, v2, v3]);
        }
    }
    
    // Intake face
    let intake_center = vertices.len();
    vertices.push([p.engine_x_position, y_pos, z_pos]);
    for j in 0..n_circ {
        let j1 = (j + 1) % n_circ;
        faces.push(vec![intake_center + 1, v_start + j1 + 1, v_start + j + 1]);
    }
    
    // Pylon (simplified)
    let pylon_start = vertices.len();
    let pylon_height = p.fuselage_diameter * 0.3 + r;
    vertices.push([p.engine_x_position + 0.5, y_pos, z_pos + r]);
    vertices.push([p.engine_x_position + 2.5, y_pos, z_pos + r]);
    vertices.push([p.engine_x_position + 2.5, y_pos, z_pos + r + pylon_height]);
    vertices.push([p.engine_x_position + 0.5, y_pos, z_pos + r + pylon_height]);
    faces.push(vec![pylon_start + 1, pylon_start + 2, pylon_start + 3, pylon_start + 4]);
}

fn generate_landing_gear(p: &AircraftParams, vertices: &mut Vec<[f64; 3]>, faces: &mut Vec<Vec<usize>>) {
    // Main gear (simplified as cylinders)
    for side in [-1.0_f64, 1.0_f64] {
        let v_start = vertices.len();
        let y = side * p.fuselage_diameter * 0.8;
        let n = 8;
        
        // Strut
        for i in 0..=1 {
            let z = -p.fuselage_diameter / 2.0 - i as f64 * p.gear_height;
            for j in 0..n {
                let angle = j as f64 / n as f64 * 2.0 * PI;
                let r = 0.1;
                vertices.push([
                    p.main_gear_x + r * angle.cos(),
                    y + r * angle.sin(),
                    z,
                ]);
            }
        }
        
        for j in 0..n {
            let j1 = (j + 1) % n;
            faces.push(vec![v_start + j + 1, v_start + j1 + 1, v_start + n + j1 + 1, v_start + n + j + 1]);
        }
        
        // Wheel (simplified)
        let wheel_start = vertices.len();
        let wheel_z = -p.fuselage_diameter / 2.0 - p.gear_height;
        let wheel_r = 0.5;
        for j in 0..n {
            let angle = j as f64 / n as f64 * 2.0 * PI;
            vertices.push([
                p.main_gear_x + wheel_r * angle.cos(),
                y - 0.2,
                wheel_z + wheel_r * angle.sin(),
            ]);
            vertices.push([
                p.main_gear_x + wheel_r * angle.cos(),
                y + 0.2,
                wheel_z + wheel_r * angle.sin(),
            ]);
        }
        
        for j in 0..n {
            let j1 = (j + 1) % n;
            faces.push(vec![
                wheel_start + j * 2 + 1,
                wheel_start + j1 * 2 + 1,
                wheel_start + j1 * 2 + 2,
                wheel_start + j * 2 + 2,
            ]);
        }
    }
    
    // Nose gear (simplified)
    let v_start = vertices.len();
    let n = 8;
    for i in 0..=1 {
        let z = -p.fuselage_diameter / 2.0 - i as f64 * (p.gear_height * 0.8);
        for j in 0..n {
            let angle = j as f64 / n as f64 * 2.0 * PI;
            let r = 0.08;
            vertices.push([p.nose_gear_x + r * angle.cos(), r * angle.sin(), z]);
        }
    }
    for j in 0..n {
        let j1 = (j + 1) % n;
        faces.push(vec![v_start + j + 1, v_start + j1 + 1, v_start + n + j1 + 1, v_start + n + j + 1]);
    }
}

fn write_obj_file(mesh: &Mesh, obj_path: &str, mtl_path: &str) -> std::io::Result<()> {
    let mut file = File::create(obj_path)?;
    
    writeln!(file, "# Aircraft 3D Model")?;
    writeln!(file, "# Generated by S-wind Aircraft Designer")?;
    writeln!(file, "# Vertices: {}, Faces: {}", mesh.vertices.len(), mesh.faces.len())?;
    writeln!(file)?;
    writeln!(file, "mtllib {}", mtl_path)?;
    writeln!(file)?;
    
    for v in &mesh.vertices {
        writeln!(file, "v {:.6} {:.6} {:.6}", v[0], v[1], v[2])?;
    }
    writeln!(file)?;
    
    writeln!(file, "usemtl aircraft")?;
    for face in &mesh.faces {
        let face_str: Vec<String> = face.iter().map(|&v| format!("{}", v)).collect();
        writeln!(file, "f {}", face_str.join(" "))?;
    }
    
    Ok(())
}

fn write_mtl_file(mtl_path: &str) -> std::io::Result<()> {
    let mut file = File::create(mtl_path)?;
    
    writeln!(file, "# Aircraft Materials")?;
    writeln!(file)?;
    
    writeln!(file, "newmtl aircraft")?;
    writeln!(file, "Ns 200.0")?;
    writeln!(file, "Ka 0.1 0.1 0.1")?;
    writeln!(file, "Kd 0.9 0.9 0.95")?;  // White/silver
    writeln!(file, "Ks 0.5 0.5 0.5")?;
    writeln!(file, "Ni 1.0")?;
    writeln!(file, "d 1.0")?;
    writeln!(file, "illum 2")?;
    
    Ok(())
}
