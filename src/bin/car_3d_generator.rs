//! Car 3D Model Generator
//! 
//! Generates a parametric car model as OBJ file
//! Can be opened in Blender, FreeCAD, MeshLab, etc.

use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║           CAR 3D MODEL GENERATOR                              ║");
    println!("║           Output: Wavefront OBJ + MTL                         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");

    // Car parameters (meters)
    let params = CarParams {
        // Overall dimensions
        length: 4.75,
        width: 1.85,
        height: 1.42,
        
        // Body proportions
        hood_length: 0.95,       // Front hood
        cabin_start: 1.15,       // Where windshield starts
        cabin_end: 3.60,         // Where rear window ends
        trunk_start: 3.80,       // Trunk lid start
        
        // Heights
        hood_height: 0.85,
        beltline_height: 1.05,
        roof_height: 1.40,
        
        // Wheel parameters
        wheelbase: 2.85,
        track: 1.58,
        wheel_radius: 0.33,
        wheel_width: 0.245,
        front_axle_x: 0.95,
        
        // Shape parameters
        body_radius: 0.15,       // Edge rounding
        windshield_angle: 32.0,  // degrees
        rear_angle: 25.0,        // degrees
    };

    println!("  Generating car model with parameters:");
    println!("    Length: {:.2}m, Width: {:.2}m, Height: {:.2}m", 
             params.length, params.width, params.height);
    println!("    Wheelbase: {:.2}m, Track: {:.2}m", params.wheelbase, params.track);
    println!();

    // Generate mesh
    let mesh = generate_car_mesh(&params);
    
    println!("  Generated {} vertices, {} faces", mesh.vertices.len(), mesh.faces.len());
    
    // Write OBJ file
    let obj_path = "car_model.obj";
    let mtl_path = "car_model.mtl";
    
    write_obj_file(&mesh, obj_path, mtl_path).expect("Failed to write OBJ file");
    write_mtl_file(mtl_path).expect("Failed to write MTL file");
    
    println!();
    println!("  ✓ Generated: {}", obj_path);
    println!("  ✓ Generated: {}", mtl_path);
    println!();
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Open in: Blender, FreeCAD, MeshLab, 3ds Max, Maya           ║");
    println!("║                                                               ║");
    println!("║  Blender: File -> Import -> Wavefront (.obj)                  ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

struct CarParams {
    length: f64,
    width: f64,
    height: f64,
    hood_length: f64,
    cabin_start: f64,
    cabin_end: f64,
    trunk_start: f64,
    hood_height: f64,
    beltline_height: f64,
    roof_height: f64,
    wheelbase: f64,
    track: f64,
    wheel_radius: f64,
    wheel_width: f64,
    front_axle_x: f64,
    body_radius: f64,
    windshield_angle: f64,
    rear_angle: f64,
}

struct Mesh {
    vertices: Vec<[f64; 3]>,
    normals: Vec<[f64; 3]>,
    faces: Vec<Vec<usize>>,  // vertex indices (1-based for OBJ)
    materials: Vec<(String, Vec<usize>)>, // material name -> face indices
}

fn generate_car_mesh(p: &CarParams) -> Mesh {
    let mut vertices = Vec::new();
    let mut faces = Vec::new();
    let mut body_faces = Vec::new();
    let mut wheel_faces = Vec::new();
    let mut glass_faces = Vec::new();

    // Resolution
    let n_profile = 24;  // Points around profile
    let n_length = 40;   // Points along length
    
    // Generate body vertices
    for i in 0..=n_length {
        let t = i as f64 / n_length as f64;
        let x = t * p.length;
        
        // Get cross-section at this x position
        let profile = get_body_profile(x, p);
        
        for j in 0..n_profile {
            let angle = j as f64 / n_profile as f64 * PI; // Half circle (top half)
            let (y, z) = apply_profile(angle, &profile);
            vertices.push([x, y, z]);
        }
    }
    
    // Generate body faces
    for i in 0..n_length {
        for j in 0..n_profile - 1 {
            let v0 = i * n_profile + j + 1;
            let v1 = i * n_profile + j + 2;
            let v2 = (i + 1) * n_profile + j + 2;
            let v3 = (i + 1) * n_profile + j + 1;
            
            faces.push(vec![v0, v1, v2, v3]);
            body_faces.push(faces.len() - 1);
        }
    }
    
    // Generate bottom (flat)
    let bottom_start = vertices.len();
    let bottom_pts = vec![
        [0.0, -p.width / 2.0 + 0.1, 0.15],
        [0.0, p.width / 2.0 - 0.1, 0.15],
        [p.length, p.width / 2.0 - 0.1, 0.15],
        [p.length, -p.width / 2.0 + 0.1, 0.15],
    ];
    for pt in &bottom_pts {
        vertices.push(*pt);
    }
    faces.push(vec![bottom_start + 1, bottom_start + 2, bottom_start + 3, bottom_start + 4]);
    body_faces.push(faces.len() - 1);
    
    // Generate front face
    let front_start = vertices.len();
    for j in 0..n_profile {
        let idx = j;
        vertices.push(vertices[idx]);
    }
    for j in 0..n_profile - 1 {
        faces.push(vec![front_start + j + 1, front_start + j + 2, 1]);
        body_faces.push(faces.len() - 1);
    }
    
    // Generate rear face
    let rear_start = vertices.len();
    for j in 0..n_profile {
        let idx = n_length * n_profile + j;
        vertices.push(vertices[idx]);
    }
    for j in 0..n_profile - 1 {
        faces.push(vec![rear_start + j + 1, rear_start + j + 2, rear_start + 1]);
        body_faces.push(faces.len() - 1);
    }
    
    // Generate wheels
    let wheel_positions = [
        (p.front_axle_x, p.track / 2.0),
        (p.front_axle_x, -p.track / 2.0),
        (p.front_axle_x + p.wheelbase, p.track / 2.0),
        (p.front_axle_x + p.wheelbase, -p.track / 2.0),
    ];
    
    for (wx, wy) in wheel_positions {
        let wheel_start = vertices.len();
        let n_wheel = 16;
        
        // Wheel center
        let wz = p.wheel_radius;
        
        // Outer rim
        for k in 0..n_wheel {
            let angle = k as f64 / n_wheel as f64 * 2.0 * PI;
            let y_off = wy + if wy > 0.0 { p.wheel_width / 2.0 } else { -p.wheel_width / 2.0 };
            vertices.push([
                wx + p.wheel_radius * angle.cos(),
                y_off,
                wz + p.wheel_radius * angle.sin(),
            ]);
        }
        
        // Inner rim
        for k in 0..n_wheel {
            let angle = k as f64 / n_wheel as f64 * 2.0 * PI;
            let y_off = wy + if wy > 0.0 { -p.wheel_width / 2.0 } else { p.wheel_width / 2.0 };
            vertices.push([
                wx + p.wheel_radius * angle.cos(),
                y_off,
                wz + p.wheel_radius * angle.sin(),
            ]);
        }
        
        // Tire faces
        for k in 0..n_wheel {
            let k1 = (k + 1) % n_wheel;
            let v0 = wheel_start + k + 1;
            let v1 = wheel_start + k1 + 1;
            let v2 = wheel_start + n_wheel + k1 + 1;
            let v3 = wheel_start + n_wheel + k + 1;
            faces.push(vec![v0, v1, v2, v3]);
            wheel_faces.push(faces.len() - 1);
        }
        
        // Hub caps (simplified)
        let hub_center = vertices.len();
        let y_off = wy + if wy > 0.0 { p.wheel_width / 2.0 + 0.01 } else { -p.wheel_width / 2.0 - 0.01 };
        vertices.push([wx, y_off, wz]);
        
        for k in 0..n_wheel {
            let k1 = (k + 1) % n_wheel;
            faces.push(vec![hub_center + 1, wheel_start + k + 1, wheel_start + k1 + 1]);
            wheel_faces.push(faces.len() - 1);
        }
    }
    
    // Windshield area (mark for glass material)
    // Find vertices in windshield region
    for i in 0..n_length {
        let t = i as f64 / n_length as f64;
        let x = t * p.length;
        
        if x >= p.cabin_start - 0.2 && x <= p.cabin_start + 0.5 {
            for j in (n_profile / 4)..(3 * n_profile / 4) {
                let face_idx = i * (n_profile - 1) + j.min(n_profile - 2);
                if face_idx < body_faces.len() {
                    glass_faces.push(face_idx);
                }
            }
        }
    }
    
    // Compute normals (simplified - flat shading)
    let normals = compute_normals(&vertices, &faces);
    
    Mesh {
        vertices,
        normals,
        faces,
        materials: vec![
            ("body".to_string(), body_faces),
            ("wheel".to_string(), wheel_faces),
            ("glass".to_string(), glass_faces),
        ],
    }
}

fn get_body_profile(x: f64, p: &CarParams) -> BodyProfile {
    // Determine height and width at position x
    let (height, z_base) = if x < p.hood_length {
        // Front bumper to hood
        let t = x / p.hood_length;
        let h = 0.4 + t * (p.hood_height - 0.4);
        (h, 0.2)
    } else if x < p.cabin_start {
        // Hood
        (p.hood_height, 0.2)
    } else if x < p.cabin_end {
        // Cabin
        let t = (x - p.cabin_start) / (p.cabin_end - p.cabin_start);
        let roof_t = smooth_step(t, 0.0, 0.3);
        let h = p.beltline_height + roof_t * (p.roof_height - p.beltline_height);
        (h, 0.2)
    } else if x < p.trunk_start {
        // Rear window transition
        let t = (x - p.cabin_end) / (p.trunk_start - p.cabin_end);
        let h = p.roof_height - t * (p.roof_height - p.beltline_height);
        (h, 0.2)
    } else {
        // Trunk
        let t = (x - p.trunk_start) / (p.length - p.trunk_start);
        let h = p.beltline_height - t * 0.3;
        (h.max(0.5), 0.2 + t * 0.1)
    };
    
    // Width variation
    let width = if x < 0.3 {
        p.width * 0.6 + (x / 0.3) * p.width * 0.4
    } else if x > p.length - 0.3 {
        let t = (x - (p.length - 0.3)) / 0.3;
        p.width * (1.0 - t * 0.3)
    } else {
        p.width
    };
    
    BodyProfile {
        width: width * 0.5,  // half width
        height,
        z_base,
        radius: p.body_radius,
    }
}

struct BodyProfile {
    width: f64,
    height: f64,
    z_base: f64,
    radius: f64,
}

fn apply_profile(angle: f64, profile: &BodyProfile) -> (f64, f64) {
    // Superellipse-like cross section
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    
    // Modified to create car-like profile
    let n = 2.5; // Roundness factor
    let y = profile.width * cos_a.abs().powf(2.0 / n) * cos_a.signum();
    let z = profile.z_base + (profile.height - profile.z_base) * sin_a.abs().powf(2.0 / n) * sin_a.signum().max(0.0);
    
    (y, z.max(profile.z_base))
}

fn smooth_step(t: f64, edge0: f64, edge1: f64) -> f64 {
    let x = ((t - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

fn compute_normals(vertices: &[[f64; 3]], faces: &[Vec<usize>]) -> Vec<[f64; 3]> {
    let mut normals = vec![[0.0, 0.0, 0.0]; vertices.len()];
    
    for face in faces {
        if face.len() >= 3 {
            let v0 = vertices[face[0] - 1];
            let v1 = vertices[face[1] - 1];
            let v2 = vertices[face[2] - 1];
            
            // Cross product
            let u = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let v = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];
            let n = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            
            // Add to vertex normals
            for &idx in face {
                let i = idx - 1;
                normals[i][0] += n[0];
                normals[i][1] += n[1];
                normals[i][2] += n[2];
            }
        }
    }
    
    // Normalize
    for n in &mut normals {
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        if len > 0.0 {
            n[0] /= len;
            n[1] /= len;
            n[2] /= len;
        }
    }
    
    normals
}

fn write_obj_file(mesh: &Mesh, obj_path: &str, mtl_path: &str) -> std::io::Result<()> {
    let mut file = File::create(obj_path)?;
    
    writeln!(file, "# Car 3D Model")?;
    writeln!(file, "# Generated by S-wind Car Designer")?;
    writeln!(file, "# Vertices: {}, Faces: {}", mesh.vertices.len(), mesh.faces.len())?;
    writeln!(file)?;
    writeln!(file, "mtllib {}", mtl_path)?;
    writeln!(file)?;
    
    // Write vertices
    for v in &mesh.vertices {
        writeln!(file, "v {:.6} {:.6} {:.6}", v[0], v[1], v[2])?;
    }
    writeln!(file)?;
    
    // Write normals
    for n in &mesh.normals {
        writeln!(file, "vn {:.6} {:.6} {:.6}", n[0], n[1], n[2])?;
    }
    writeln!(file)?;
    
    // Write faces with materials
    writeln!(file, "usemtl body")?;
    for (i, face) in mesh.faces.iter().enumerate() {
        // Check if this face should use different material
        for (mat_name, mat_faces) in &mesh.materials {
            if mat_faces.contains(&i) && mat_name != "body" {
                writeln!(file, "usemtl {}", mat_name)?;
                break;
            }
        }
        
        let face_str: Vec<String> = face.iter()
            .map(|&v| format!("{}//{}", v, v))
            .collect();
        writeln!(file, "f {}", face_str.join(" "))?;
    }
    
    Ok(())
}

fn write_mtl_file(mtl_path: &str) -> std::io::Result<()> {
    let mut file = File::create(mtl_path)?;
    
    writeln!(file, "# Car Materials")?;
    writeln!(file)?;
    
    // Body material (metallic blue)
    writeln!(file, "newmtl body")?;
    writeln!(file, "Ns 250.0")?;        // Shininess
    writeln!(file, "Ka 0.1 0.1 0.1")?;  // Ambient
    writeln!(file, "Kd 0.1 0.2 0.5")?;  // Diffuse (blue)
    writeln!(file, "Ks 0.8 0.8 0.8")?;  // Specular
    writeln!(file, "Ni 1.5")?;          // Optical density
    writeln!(file, "d 1.0")?;           // Dissolve (opacity)
    writeln!(file, "illum 2")?;         // Illumination model
    writeln!(file)?;
    
    // Wheel material (dark gray)
    writeln!(file, "newmtl wheel")?;
    writeln!(file, "Ns 100.0")?;
    writeln!(file, "Ka 0.05 0.05 0.05")?;
    writeln!(file, "Kd 0.15 0.15 0.15")?;
    writeln!(file, "Ks 0.3 0.3 0.3")?;
    writeln!(file, "d 1.0")?;
    writeln!(file, "illum 2")?;
    writeln!(file)?;
    
    // Glass material (transparent)
    writeln!(file, "newmtl glass")?;
    writeln!(file, "Ns 500.0")?;
    writeln!(file, "Ka 0.0 0.0 0.0")?;
    writeln!(file, "Kd 0.1 0.1 0.15")?;
    writeln!(file, "Ks 1.0 1.0 1.0")?;
    writeln!(file, "Ni 1.5")?;
    writeln!(file, "d 0.3")?;           // Transparent
    writeln!(file, "illum 2")?;
    
    Ok(())
}
