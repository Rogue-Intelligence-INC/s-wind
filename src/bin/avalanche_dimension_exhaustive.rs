//! S-wind Exhaustive Avalanche vs Dimension Experiment
//!
//! Question: What is the EXACT optimal dimension for maximum avalanche effect?
//! Tests dimensions from 1.0 to 4.0 in 0.1 increments

use s_wind::{EPR, Fate, KnowledgeBase};
use std::time::Instant;
use rayon::prelude::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║     S-wind Exhaustive Experiment: Optimal Dimension for Avalanche Effect                    ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════════════════════╣");
    println!("║  Question: At what EXACT dimension does MeshBrain achieve ideal 50% avalanche?              ║");
    println!("║  Method: Test dimensions 1.0 to 4.0 in 0.05 increments (61 data points)                     ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════════════════════╝");
    println!();

    let key = b"ExperimentKey!@#$%^&*()_+12345678";
    let test_data: Vec<u8> = (0..2048).map(|i| (i % 256) as u8).collect();
    
    // Test dimensions from 1.0 to 4.0 in 0.05 increments
    let dimensions: Vec<f64> = (20..=80).map(|i| i as f64 * 0.05).collect();
    
    println!("Testing {} dimension values from {:.2} to {:.2}...", 
        dimensions.len(), dimensions[0], dimensions[dimensions.len()-1]);
    println!();

    // Collect results
    let results: Vec<(f64, f64, f64, f64)> = dimensions.par_iter()
        .map(|&dim| {
            let (avalanche, key_sens, throughput) = test_dimension(dim, &test_data, key);
            (dim, avalanche, key_sens, throughput)
        })
        .collect();

    // Display results table
    println!("╔══════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              AVALANCHE vs DIMENSION RESULTS                                 ║");
    println!("╠═══════════╦════════════════╦════════════════╦════════════════╦═════════════════════════════╣");
    println!("║ Dimension ║   Avalanche    ║   Key Sens     ║   Throughput   ║          Graph              ║");
    println!("╠═══════════╬════════════════╬════════════════╬════════════════╬═════════════════════════════╣");

    let mut best_dim = 1.0;
    let mut best_avalanche = 0.0;
    let mut best_score = 0.0;

    for &(dim, av, ks, tp) in &results {
        // Score: how close to 50% avalanche
        let av_distance = (av - 0.5).abs();
        let score = 1.0 - av_distance * 2.0;
        
        if score > best_score {
            best_score = score;
            best_dim = dim;
            best_avalanche = av;
        }

        // Visual bar for avalanche (50% is center, ideal)
        let bar_pos = (av * 40.0) as usize;
        let ideal_pos = 20;
        let mut bar = vec![' '; 25];
        bar[ideal_pos] = '│';  // 50% marker
        if bar_pos < 25 {
            bar[bar_pos] = '█';
        }
        let bar_str: String = bar.iter().collect();

        let marker = if av_distance < 0.02 { "★" } else if av_distance < 0.05 { "◆" } else { " " };

        println!("║   {:>5.2}   ║     {:>6.2}%    ║     {:>6.2}%    ║   {:>8.1} MB/s║ {} {} ║",
            dim, av * 100.0, ks * 100.0, tp, bar_str, marker);
    }

    println!("╚═══════════╩════════════════╩════════════════╩════════════════╩═════════════════════════════╝");
    println!();

    // Find top 5 dimensions
    let mut sorted_results = results.clone();
    sorted_results.sort_by(|a, b| {
        let score_a = (1.0 - (a.1 - 0.5).abs() * 2.0);
        let score_b = (1.0 - (b.1 - 0.5).abs() * 2.0);
        score_b.partial_cmp(&score_a).unwrap()
    });

    println!("╔══════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              TOP 10 OPTIMAL DIMENSIONS                                      ║");
    println!("╠═══════╦═══════════╦═══════════════╦═══════════════╦═══════════════╦════════════════════════╣");
    println!("║ Rank  ║ Dimension ║   Avalanche   ║   Key Sens    ║   Throughput  ║   Distance from 50%    ║");
    println!("╠═══════╬═══════════╬═══════════════╬═══════════════╬═══════════════╬════════════════════════╣");

    for (i, &(dim, av, ks, tp)) in sorted_results.iter().take(10).enumerate() {
        let distance = (av - 0.5).abs() * 100.0;
        let medal = match i {
            0 => "🥇",
            1 => "🥈",
            2 => "🥉",
            _ => "  ",
        };
        println!("║  {} {}  ║   {:>5.2}   ║    {:>6.2}%    ║    {:>6.2}%    ║  {:>8.1} MB/s ║      {:>6.2}%           ║",
            medal, i + 1, dim, av * 100.0, ks * 100.0, tp, distance);
    }

    println!("╚═══════╩═══════════╩═══════════════╩═══════════════╩═══════════════╩════════════════════════╝");
    println!();

    // S-wind Fate Analysis
    println!("╔══════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                           S-WIND FATE PATTERN ANALYSIS                                      ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════════════════════╣");
    
    let mut fate = Fate::new();
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();

    // Convert avalanche data to EPR for analysis
    let avalanche_signal: Vec<EPR> = results.iter()
        .map(|&(_, av, _, _)| EPR::from_ratio((av * 10000.0) as i128, 10000))
        .collect();

    let dimension_signal: Vec<EPR> = results.iter()
        .map(|&(dim, _, _, _)| EPR::from_ratio((dim * 100.0) as i128, 100))
        .collect();

    println!("║                                                                                              ║");
    println!("║  Avalanche Pattern Analysis:                                                                 ║");
    let av_result = fate.explore(&avalanche_signal, &kb);
    if let Some(formula) = &av_result.formula {
        println!("║    Formula: {}  ║", formula);
    }
    if let Some(pattern) = &av_result.detected_pattern {
        println!("║    Pattern: {}                                                                        ║", pattern);
    }
    println!("║    Status: {:?}                                               ║", av_result.status);
    println!("║                                                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════════════════════╝");
    println!();

    // Final conclusion
    println!("╔══════════════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                                    CONCLUSION                                               ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════════════════════╣");
    println!("║                                                                                              ║");
    println!("║  OPTIMAL DIMENSION FOR AVALANCHE EFFECT:                                                     ║");
    println!("║                                                                                              ║");
    println!("║      ┌─────────────────────────────────────────────────────────────────────┐                 ║");
    println!("║      │                                                                     │                 ║");
    println!("║      │         DIMENSION = {:.2}                                            │                 ║", best_dim);
    println!("║      │         AVALANCHE = {:.2}%                                           │                 ║", best_avalanche * 100.0);
    println!("║      │         DISTANCE FROM IDEAL = {:.2}%                                 │                 ║", (best_avalanche - 0.5).abs() * 100.0);
    println!("║      │                                                                     │                 ║");
    println!("║      └─────────────────────────────────────────────────────────────────────┘                 ║");
    println!("║                                                                                              ║");

    // Additional insights
    let fractional_best: Vec<_> = sorted_results.iter()
        .filter(|&&(dim, _, _, _)| dim != dim.floor())
        .take(3)
        .collect();
    
    let integer_best: Vec<_> = sorted_results.iter()
        .filter(|&&(dim, _, _, _)| dim == dim.floor())
        .take(3)
        .collect();

    println!("║  Best FRACTIONAL dimensions: {:.2}, {:.2}, {:.2}                                            ║",
        fractional_best.get(0).map(|x| x.0).unwrap_or(0.0),
        fractional_best.get(1).map(|x| x.0).unwrap_or(0.0),
        fractional_best.get(2).map(|x| x.0).unwrap_or(0.0));
    
    println!("║  Best INTEGER dimensions:    {:.0}, {:.0}, {:.0}                                                ║",
        integer_best.get(0).map(|x| x.0).unwrap_or(0.0),
        integer_best.get(1).map(|x| x.0).unwrap_or(0.0),
        integer_best.get(2).map(|x| x.0).unwrap_or(0.0));
    
    println!("║                                                                                              ║");
    
    // Is fractional better than integer?
    let best_frac_score = fractional_best.get(0).map(|x| 1.0 - (x.1 - 0.5).abs() * 2.0).unwrap_or(0.0);
    let best_int_score = integer_best.get(0).map(|x| 1.0 - (x.1 - 0.5).abs() * 2.0).unwrap_or(0.0);
    
    if best_frac_score > best_int_score {
        println!("║  ★ FRACTIONAL dimensions achieve BETTER avalanche than INTEGER dimensions!                 ║");
    } else {
        println!("║  ○ INTEGER dimensions achieve similar or better avalanche than fractional                  ║");
    }
    
    println!("║                                                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════════════════════╝");
}

/// Test a specific dimension and return (avalanche, key_sensitivity, throughput)
fn test_dimension(dim: f64, test_data: &[u8], key: &[u8]) -> (f64, f64, f64) {
    let start = Instant::now();
    let iterations = 10;
    
    // Encrypt
    let mut cipher = Vec::new();
    for _ in 0..iterations {
        cipher = encrypt_nd(test_data, key, dim);
    }
    let elapsed = start.elapsed();
    let throughput = (test_data.len() as f64 * iterations as f64) / (elapsed.as_secs_f64() * 1024.0 * 1024.0);

    // Avalanche test
    let mut mod_data = test_data.to_vec();
    mod_data[0] ^= 0x01;
    let mod_cipher = encrypt_nd(&mod_data, key, dim);
    let avalanche = bit_diff(&cipher, &mod_cipher);

    // Key sensitivity test
    let mut mod_key = key.to_vec();
    mod_key[0] ^= 0x01;
    let key_cipher = encrypt_nd(test_data, &mod_key, dim);
    let key_sens = bit_diff(&cipher, &key_cipher);

    (avalanche, key_sens, throughput)
}

/// N-dimensional encryption with fractional dimension support
fn encrypt_nd(plaintext: &[u8], key: &[u8], dim: f64) -> Vec<u8> {
    if plaintext.is_empty() || dim <= 0.0 {
        return plaintext.to_vec();
    }

    let len = plaintext.len();
    let key0 = key[0];
    let key_len = key.len();

    // Fractional dimension: integer part + fractional influence
    let int_dim = dim.floor() as usize;
    let frac_dim = dim - dim.floor();
    
    // Calculate chunk size based on dimension
    let chunk_size = if int_dim == 0 {
        len
    } else {
        let root = (len as f64).powf(1.0 / dim).ceil() as usize;
        root.max(1)
    };
    
    let num_chunks = (len + chunk_size - 1) / chunk_size;
    let mut output = plaintext.to_vec();

    // Phase 1: Process chunks with dimension-based transformation
    for chunk_idx in 0..num_chunks {
        let start = chunk_idx * chunk_size;
        let end = (start + chunk_size).min(len);
        
        let chunk_seed = (chunk_idx & 0xFF) as u8;
        let dim_factor = ((dim * 100.0) as u8).wrapping_mul(17);
        let frac_factor = ((frac_dim * 256.0) as u8).wrapping_mul(23);
        
        let mut membrane = key0
            .wrapping_add(chunk_seed.wrapping_mul(37))
            .wrapping_add(dim_factor);

        for i in start..end {
            let byte = output[i];
            let local_idx = i - start;
            
            // Position with fractional dimension influence
            let pos_factor = ((local_idx.wrapping_mul(17).wrapping_add(chunk_idx.wrapping_mul(31))) & 0xFF) as u8;
            let frac_influence = ((local_idx as f64 * frac_dim) as u8).wrapping_mul(19);
            
            let key_idx = (local_idx.wrapping_add(chunk_idx.wrapping_mul(int_dim.max(1)))) % key_len;
            let pk = key[key_idx]
                .wrapping_add(pos_factor)
                .wrapping_add(frac_factor)
                .wrapping_add(frac_influence);

            // Neural state with dimension-modulated threshold
            let state = membrane.wrapping_add(byte).wrapping_add(frac_influence);
            let threshold = 127u8
                .wrapping_add(dim_factor & 0x1F)
                .wrapping_add(frac_factor & 0x0F);
            
            let activated = if state > threshold {
                state.wrapping_mul(pk).rotate_left(((dim as u32) % 4) + 1)
            } else {
                state.wrapping_add(pk).rotate_right(((dim as u32) % 3) + 1)
            };

            let enc = activated ^ pk ^ frac_influence;
            membrane = membrane.wrapping_add(enc).wrapping_add(frac_factor).rotate_left(1);
            output[i] = enc;
        }
    }

    // Phase 2: Cross-chunk cascade with fractional depth
    let depth_stride = ((len as f64 * frac_dim).ceil() as usize).max(1);
    let dim_factor = ((dim * 100.0) as u8).wrapping_mul(17);
    let mut cascade = key0.wrapping_add(dim_factor);
    
    for i in 0..len {
        // Fractional dimension creates "depth jumps"
        let depth_idx = if frac_dim > 0.0 {
            ((i as f64 * frac_dim) as usize) % len
        } else {
            (i + depth_stride) % len
        };
        
        let depth_influence = if depth_idx != i {
            output[depth_idx]
        } else {
            key[i % key_len]
        };
        
        let mixed = output[i]
            .wrapping_add(cascade)
            .wrapping_add(depth_influence.rotate_left(2));
        output[i] = mixed;
        cascade = cascade.wrapping_add(mixed).rotate_left(1);
    }

    output
}

fn bit_diff(a: &[u8], b: &[u8]) -> f64 {
    let n = a.len().min(b.len());
    if n == 0 { return 0.0; }
    let diff: u32 = a.iter().zip(b.iter()).take(n)
        .map(|(&x, &y)| (x ^ y).count_ones()).sum();
    diff as f64 / (n * 8) as f64
}
