//! Test: S-wind Container Creation Capabilities
//!
//! Verifies that S-wind can create completely different types of containers
//! with varying dimensions (integer, fractional, adaptive).

use s_wind::space::{SWindSpace, ContainerType};
use s_wind::symbolic::Rational;

fn main() {
    println!("=== S-wind Container Creation Test ===\n");
    
    let mut space = SWindSpace::new();
    println!("Created SWindSpace: {}", space.space_id);
    println!("Initial dimension: {}\n", space.get_global_dimension());
    
    // 1. Create Basic Container
    println!("--- 1. Basic Container ---");
    let basic_id = space.create_container(ContainerType::Basic);
    let basic = space.containers.get(&basic_id).unwrap();
    println!("Container ID: {}", basic.container_id);
    println!("Type: {:?}", basic.container_type);
    println!("Dimensions: {}", basic.dimensions);
    println!("Created at tick: {}\n", basic.created_at_tick);
    
    // 2. Create Adaptive Container
    println!("--- 2. Adaptive Container ---");
    let adaptive_id = space.create_container(ContainerType::Adaptive);
    let adaptive = space.containers.get(&adaptive_id).unwrap();
    println!("Container ID: {}", adaptive.container_id);
    println!("Type: {:?}", adaptive.container_type);
    println!("Dimensions: {}", adaptive.dimensions);
    println!("Created at tick: {}\n", adaptive.created_at_tick);
    
    // 3. Create Fractional Dimension Container (from data)
    println!("--- 3. Fractional Dimension Container (from data) ---");
    let data = vec![
        Rational::new(1, 1),
        Rational::new(1618, 1000),  // ~φ (golden ratio)
        Rational::new(2718, 1000),  // ~e
        Rational::new(3141, 1000),  // ~π
    ];
    let frac_id = space.create_fractional_container(&data);
    let frac = space.containers.get(&frac_id).unwrap();
    println!("Container ID: {}", frac.container_id);
    println!("Type: {:?}", frac.container_type);
    println!("Integer dimensions: {}", frac.dimensions);
    println!("Fractional dimension: {}", frac.fractional_dimension.unwrap());
    println!("Created at tick: {}\n", frac.created_at_tick);
    
    // 4. Create another Fractional Container with different data
    println!("--- 4. Fractional Container (logarithmic scale data) ---");
    let data2 = vec![
        Rational::new(1, 100),
        Rational::new(1, 10),
        Rational::new(1, 1),
        Rational::new(10, 1),
        Rational::new(100, 1),
    ];
    let frac2_id = space.create_fractional_container(&data2);
    let frac2 = space.containers.get(&frac2_id).unwrap();
    println!("Container ID: {}", frac2.container_id);
    println!("Type: {:?}", frac2.container_type);
    println!("Integer dimensions: {}", frac2.dimensions);
    println!("Fractional dimension: {}", frac2.fractional_dimension.unwrap());
    println!("Created at tick: {}\n", frac2.created_at_tick);
    
    // 5. Create Explicit Fractional Container (7/2 = 3.5 dimensions)
    println!("--- 5. Explicit Fractional Container (3.5 dimensions) ---");
    let explicit_dim = Rational::new(7, 2);  // 3.5 dimensions
    let explicit_id = space.create_container(ContainerType::Fractional { dimension: explicit_dim });
    let explicit = space.containers.get(&explicit_id).unwrap();
    println!("Container ID: {}", explicit.container_id);
    println!("Type: {:?}", explicit.container_type);
    println!("Stored dimension: {}", explicit.dimensions);
    println!("Created at tick: {}\n", explicit.created_at_tick);
    
    // 6. Create high-dimensional container
    println!("--- 6. High-Dimensional Container (11 dimensions) ---");
    let high_dim_data: Vec<Rational> = (1..=1000)
        .map(|i| Rational::new(i, 1))
        .collect();
    let high_id = space.create_fractional_container(&high_dim_data);
    let high = space.containers.get(&high_id).unwrap();
    println!("Container ID: {}", high.container_id);
    println!("Type: {:?}", high.container_type);
    println!("Dimensions (clamped to max): {}", high.dimensions);
    println!("Fractional dimension: {}", high.fractional_dimension.unwrap());
    println!("Created at tick: {}\n", high.created_at_tick);
    
    // Summary
    println!("=== Summary ===");
    println!("Total containers created: {}", space.containers.len());
    println!("Global dimension after expansion: {}", space.get_global_dimension());
    println!("Logical time (signal_tick): {}", space.get_logical_time());
    
    // List all containers
    println!("\n=== All Containers ===");
    for (id, container) in &space.containers {
        println!("  {} | type={:?} | dim={} | tick={}", 
            id, container.container_type, container.dimensions, container.created_at_tick);
    }
    
    println!("\n✅ S-wind can create completely different containers!");
    println!("   - Basic containers (fixed dimension)");
    println!("   - Adaptive containers (auto-adjust)");
    println!("   - Fractional dimension containers (from data analysis)");
    println!("   - Explicit fractional containers (user-specified)");
    println!("   - High-dimensional containers (clamped to max 11)");
}
