//! MeshBrain Dimension Optimization Experiment
//!
//! Uses S-wind Fate system to find optimal dimensions for encryption.
//! Tests 1D through 8D and analyzes:
//! - Avalanche effect
//! - Throughput
//! - Entropy
//! - Key sensitivity

use s_wind::{EPR, Fate, KnowledgeBase};
use std::time::Instant;
use rayon::prelude::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════════════════════════╗");
    println!("║       S-wind MeshBrain Dimension Optimization Experiment                        ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════════╣");
    println!("║  Question: What is the optimal number of dimensions for MeshBrain encryption?   ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════════╝");
    println!();

    // Initialize S-wind Fate
    let mut fate = Fate::new();
    let mut kb = KnowledgeBase::new();
    kb.initialize_fundamentals();
    
    let key = b"ExperimentKey!@#$%^&*()_+1234567890";
    let test_sizes = [4096, 65536, 262144, 1048576]; // 4KB, 64KB, 256KB, 1MB
    
    // Test dimensions 1-8
    let dimensions: Vec<usize> = (1..=8).collect();
    
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("                        DIMENSION PERFORMANCE MATRIX");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();
    
    // Collect results for all dimensions
    let mut all_results: Vec<DimensionResult> = Vec::new();
    
    for &dim in &dimensions {
        println!("Testing {}D encryption...", dim);
        let result = test_dimension(dim, key, &test_sizes);
        all_results.push(result);
    }
    
    println!();
    
    // Display throughput matrix
    println!("╔══════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                          THROUGHPUT (MB/s) BY DIMENSION                         ║");
    println!("╠══════╦════════════╦════════════╦════════════╦════════════╦═════════════════════╣");
    println!("║ Dim  ║    4 KB    ║   64 KB    ║  256 KB    ║    1 MB    ║       Average       ║");
    println!("╠══════╬════════════╬════════════╬════════════╬════════════╬═════════════════════╣");
    
    let mut best_throughput_dim = 1;
    let mut best_throughput = 0.0;
    
    for result in &all_results {
        let avg = result.throughputs.iter().sum::<f64>() / result.throughputs.len() as f64;
        if avg > best_throughput {
            best_throughput = avg;
            best_throughput_dim = result.dimension;
        }
        
        println!("║  {}D  ║ {:>10.1} ║ {:>10.1} ║ {:>10.1} ║ {:>10.1} ║ {:>19.1} ║",
            result.dimension,
            result.throughputs[0],
            result.throughputs[1],
            result.throughputs[2],
            result.throughputs[3],
            avg
        );
    }
    println!("╚══════╩════════════╩════════════╩════════════╩════════════╩═════════════════════╝");
    println!();
    
    // Display security metrics
    println!("╔══════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                          SECURITY METRICS BY DIMENSION                          ║");
    println!("╠══════╦════════════╦════════════╦════════════╦════════════╦═════════════════════╣");
    println!("║ Dim  ║  Avalanche ║  Key Sens  ║   Entropy  ║ Uniformity ║    Security Score   ║");
    println!("╠══════╬════════════╬════════════╬════════════╬════════════╬═════════════════════╣");
    
    let mut best_security_dim = 1;
    let mut best_security_score = 0.0;
    
    for result in &all_results {
        // Security score: weighted combination
        // Avalanche: closer to 50% is better (max 40 points)
        // Key sensitivity: closer to 50% is better (max 30 points)
        // Entropy: closer to 8.0 is better (max 20 points)
        // Uniformity: higher is better (max 10 points)
        
        let av_score = 40.0 * (1.0 - (result.avalanche - 0.5).abs() * 2.0).max(0.0);
        let ks_score = 30.0 * (1.0 - (result.key_sensitivity - 0.5).abs() * 2.0).max(0.0);
        let ent_score = 20.0 * (result.entropy / 8.0).min(1.0);
        let uni_score = 10.0 * result.uniformity;
        
        let total_score = av_score + ks_score + ent_score + uni_score;
        
        if total_score > best_security_score {
            best_security_score = total_score;
            best_security_dim = result.dimension;
        }
        
        println!("║  {}D  ║ {:>9.2}% ║ {:>9.2}% ║ {:>10.4} ║ {:>9.2}% ║ {:>17.1}/100 ║",
            result.dimension,
            result.avalanche * 100.0,
            result.key_sensitivity * 100.0,
            result.entropy,
            result.uniformity * 100.0,
            total_score
        );
    }
    println!("╚══════╩════════════╩════════════╩════════════╩════════════╩═════════════════════╝");
    println!();
    
    // Combined score (throughput + security)
    println!("╔══════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                       COMBINED OPTIMIZATION SCORE                               ║");
    println!("╠══════╦════════════════╦════════════════╦════════════════╦══════════════════════╣");
    println!("║ Dim  ║  Speed Score   ║ Security Score ║  Combined 50/50║      Verdict         ║");
    println!("╠══════╬════════════════╬════════════════╬════════════════╬══════════════════════╣");
    
    // Normalize throughput scores
    let max_throughput = all_results.iter()
        .map(|r| r.throughputs.iter().sum::<f64>() / r.throughputs.len() as f64)
        .fold(0.0_f64, f64::max);
    
    let mut best_combined_dim = 1;
    let mut best_combined_score = 0.0;
    
    for result in &all_results {
        let avg_tp = result.throughputs.iter().sum::<f64>() / result.throughputs.len() as f64;
        let speed_score = 100.0 * (avg_tp / max_throughput);
        
        let av_score = 40.0 * (1.0 - (result.avalanche - 0.5).abs() * 2.0).max(0.0);
        let ks_score = 30.0 * (1.0 - (result.key_sensitivity - 0.5).abs() * 2.0).max(0.0);
        let ent_score = 20.0 * (result.entropy / 8.0).min(1.0);
        let uni_score = 10.0 * result.uniformity;
        let security_score = av_score + ks_score + ent_score + uni_score;
        
        let combined = (speed_score + security_score) / 2.0;
        
        if combined > best_combined_score {
            best_combined_score = combined;
            best_combined_dim = result.dimension;
        }
        
        let verdict = if result.dimension == best_combined_dim { "← OPTIMAL" } else { "" };
        
        println!("║  {}D  ║ {:>14.1} ║ {:>14.1} ║ {:>14.1} ║ {:>20} ║",
            result.dimension,
            speed_score,
            security_score,
            combined,
            verdict
        );
    }
    println!("╚══════╩════════════════╩════════════════╩════════════════╩══════════════════════╝");
    println!();
    
    // S-wind Fate analysis
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!("                        S-WIND FATE PATTERN ANALYSIS");
    println!("═══════════════════════════════════════════════════════════════════════════════════");
    println!();
    
    // Convert results to EPR for Fate analysis
    // Avalanche by dimension
    let avalanche_signal: Vec<EPR> = all_results.iter()
        .map(|r| EPR::from_ratio((r.avalanche * 10000.0) as i128, 10000))
        .collect();
    
    let throughput_signal: Vec<EPR> = all_results.iter()
        .map(|r| {
            let avg = r.throughputs.iter().sum::<f64>() / r.throughputs.len() as f64;
            EPR::from_ratio((avg * 100.0) as i128, 100)
        })
        .collect();
    
    // Analyze avalanche pattern
    println!("Avalanche Effect Pattern Analysis:");
    let av_result = fate.explore(&avalanche_signal, &kb);
    if let Some(formula) = &av_result.formula {
        println!("  Detected formula: {}", formula);
    }
    if let Some(pattern) = &av_result.detected_pattern {
        println!("  Pattern: {}", pattern);
    }
    if let Some(matched) = &av_result.matched_known {
        println!("  Matched known: {}", matched);
    }
    println!("  Status: {:?}", av_result.status);
    println!();
    
    // Analyze throughput pattern
    println!("Throughput Pattern Analysis:");
    let tp_result = fate.explore(&throughput_signal, &kb);
    if let Some(formula) = &tp_result.formula {
        println!("  Detected formula: {}", formula);
    }
    if let Some(pattern) = &tp_result.detected_pattern {
        println!("  Pattern: {}", pattern);
    }
    println!("  Status: {:?}", tp_result.status);
    println!();
    
    // Final recommendation
    println!("╔══════════════════════════════════════════════════════════════════════════════════╗");
    println!("║                              CONCLUSION                                         ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════════╣");
    println!("║                                                                                 ║");
    println!("║  Best for THROUGHPUT:     {}D                                                   ║", best_throughput_dim);
    println!("║  Best for SECURITY:       {}D                                                   ║", best_security_dim);
    println!("║  Best COMBINED (50/50):   {}D  ← RECOMMENDED                                    ║", best_combined_dim);
    println!("║                                                                                 ║");
    
    let best = &all_results[best_combined_dim - 1];
    let avg_tp = best.throughputs.iter().sum::<f64>() / best.throughputs.len() as f64;
    println!("║  {}D Performance:                                                               ║", best_combined_dim);
    println!("║    - Average Throughput: {:>8.1} MB/s                                          ║", avg_tp);
    println!("║    - Avalanche Effect:   {:>8.2}%                                              ║", best.avalanche * 100.0);
    println!("║    - Key Sensitivity:    {:>8.2}%                                              ║", best.key_sensitivity * 100.0);
    println!("║    - Entropy:            {:>8.4} bits                                          ║", best.entropy);
    println!("║                                                                                 ║");
    println!("╚══════════════════════════════════════════════════════════════════════════════════╝");
}

#[derive(Debug)]
struct DimensionResult {
    dimension: usize,
    throughputs: Vec<f64>,  // For different data sizes
    avalanche: f64,
    key_sensitivity: f64,
    entropy: f64,
    uniformity: f64,
}

fn test_dimension(dim: usize, key: &[u8], sizes: &[usize]) -> DimensionResult {
    let mut throughputs = Vec::new();
    
    // Test throughput for each size
    for &size in sizes {
        let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
        let iterations = if size >= 262144 { 10 } else { 30 };
        
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = encrypt_nd(&data, key, dim);
        }
        let elapsed = start.elapsed();
        let tp = (size as f64 * iterations as f64) / (elapsed.as_secs_f64() * 1024.0 * 1024.0);
        throughputs.push(tp);
    }
    
    // Test security metrics with fixed data
    let test_data: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();
    let cipher = encrypt_nd(&test_data, key, dim);
    
    // Avalanche
    let mut mod_data = test_data.clone();
    mod_data[0] ^= 0x01;
    let mod_cipher = encrypt_nd(&mod_data, key, dim);
    let avalanche = bit_diff(&cipher, &mod_cipher);
    
    // Key sensitivity
    let mut mod_key = key.to_vec();
    mod_key[0] ^= 0x01;
    let key_cipher = encrypt_nd(&test_data, &mod_key, dim);
    let key_sensitivity = bit_diff(&cipher, &key_cipher);
    
    // Entropy
    let entropy = calculate_entropy(&cipher);
    
    // Uniformity
    let uniformity = calculate_uniformity(&cipher);
    
    DimensionResult {
        dimension: dim,
        throughputs,
        avalanche,
        key_sensitivity,
        entropy,
        uniformity,
    }
}

/// N-dimensional MeshBrain encryption
fn encrypt_nd(plaintext: &[u8], key: &[u8], dim: usize) -> Vec<u8> {
    if plaintext.is_empty() || dim == 0 {
        return plaintext.to_vec();
    }
    
    let len = plaintext.len();
    
    // Calculate dimension sizes (balanced)
    let dims = calculate_dims_nd(len, dim);
    let layer_size: usize = dims[..dim.saturating_sub(1)].iter().product();
    let num_layers = *dims.last().unwrap_or(&1);
    
    // Pad data
    let total: usize = dims.iter().product();
    let mut padded = plaintext.to_vec();
    padded.resize(total, 0);
    
    let key0 = key[0];
    let key_len = key.len();
    
    // Process layers in parallel (last dimension)
    let encrypted_layers: Vec<Vec<u8>> = (0..num_layers)
        .into_par_iter()
        .map(|layer_idx| {
            let start = layer_idx * layer_size;
            let layer_data = &padded[start..start + layer_size];
            
            let mut result = Vec::with_capacity(layer_size);
            let layer_seed = (layer_idx & 0xFF) as u8;
            let mut membrane = key0.wrapping_add(layer_seed.wrapping_mul(31));
            
            for (idx, &byte) in layer_data.iter().enumerate() {
                // Calculate N-dimensional position
                let pos = index_to_nd_pos(idx, &dims[..dim-1]);
                let pos_hash = nd_pos_hash(&pos, layer_idx);
                
                // Key derivation based on position
                let key_idx = pos.iter().enumerate()
                    .fold(0usize, |acc, (i, &p)| acc.wrapping_add(p.wrapping_mul(primes(i))))
                    .wrapping_add(layer_idx.wrapping_mul(29)) % key_len;
                let pk = key[key_idx].wrapping_add(pos_hash);
                
                // Transform
                let enc = byte
                    .wrapping_add(membrane)
                    .wrapping_add(pk)
                    ^ pk.rotate_left(3);
                
                membrane = membrane.wrapping_add(enc);
                result.push(enc);
            }
            
            result
        })
        .collect();
    
    // Cross-layer cascade
    let mut output = Vec::with_capacity(total);
    let mut cascade = key0;
    
    for layer in &encrypted_layers {
        for &b in layer {
            let mixed = b.wrapping_add(cascade);
            output.push(mixed);
            cascade = cascade.wrapping_add(mixed).rotate_left(1);
        }
    }
    
    output
}

/// Calculate balanced dimensions for N-D
fn calculate_dims_nd(len: usize, dim: usize) -> Vec<usize> {
    if dim == 0 || len == 0 {
        return vec![1];
    }
    
    // N-th root approximation
    let root = (len as f64).powf(1.0 / dim as f64).ceil() as usize;
    let root = root.max(1);
    
    let mut dims = vec![root; dim];
    
    // Adjust to fit data
    let mut product: usize = dims.iter().product();
    while product < len {
        // Increase smallest dimension
        if let Some(min_idx) = dims.iter().enumerate().min_by_key(|(_, &v)| v).map(|(i, _)| i) {
            dims[min_idx] += 1;
        }
        product = dims.iter().product();
    }
    
    dims
}

/// Convert linear index to N-dimensional position
fn index_to_nd_pos(idx: usize, dims: &[usize]) -> Vec<usize> {
    let mut pos = Vec::with_capacity(dims.len());
    let mut remaining = idx;
    
    for (i, &d) in dims.iter().enumerate().rev() {
        if i == 0 {
            pos.push(remaining);
        } else {
            pos.push(remaining % d);
            remaining /= d;
        }
    }
    
    pos.reverse();
    pos
}

/// Hash N-dimensional position
fn nd_pos_hash(pos: &[usize], layer: usize) -> u8 {
    let mut hash = 0u8;
    for (i, &p) in pos.iter().enumerate() {
        hash = hash.wrapping_add(((p & 0xFF) as u8).wrapping_mul(primes(i) as u8));
    }
    hash.wrapping_add((layer & 0xFF) as u8)
}

/// Get prime by index (for position mixing)
fn primes(i: usize) -> usize {
    const PRIMES: [usize; 16] = [17, 37, 73, 97, 127, 151, 181, 211, 241, 271, 307, 337, 367, 397, 431, 461];
    PRIMES[i % PRIMES.len()]
}

fn bit_diff(a: &[u8], b: &[u8]) -> f64 {
    let n = a.len().min(b.len());
    if n == 0 { return 0.0; }
    let diff: u32 = a.iter().zip(b.iter()).take(n)
        .map(|(&x, &y)| (x ^ y).count_ones()).sum();
    diff as f64 / (n * 8) as f64
}

fn calculate_entropy(data: &[u8]) -> f64 {
    if data.is_empty() { return 0.0; }
    let mut c = [0u64; 256];
    for &b in data { c[b as usize] += 1; }
    let n = data.len() as f64;
    c.iter().filter(|&&x| x > 0).map(|&x| {
        let p = x as f64 / n;
        -p * p.log2()
    }).sum()
}

fn calculate_uniformity(data: &[u8]) -> f64 {
    if data.is_empty() { return 0.0; }
    let mut c = [0u64; 256];
    for &b in data { c[b as usize] += 1; }
    let expected = data.len() as f64 / 256.0;
    let dev: f64 = c.iter().map(|&x| (x as f64 - expected).abs()).sum();
    1.0 - (dev / (data.len() as f64 * 2.0))
}
