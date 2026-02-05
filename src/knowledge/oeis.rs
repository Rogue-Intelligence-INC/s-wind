//! OEIS (Online Encyclopedia of Integer Sequences) Loader
//!
//! Built-in OEIS sequence database for pattern recognition

use super::{KnowledgeBase, KnownSequence};
use std::io::{BufRead, BufReader};

/// OEIS Sequence Data
#[derive(Debug, Clone)]
pub struct OeisSequence {
    pub id: String,
    pub name: String,
    pub values: Vec<i128>,
    pub formula: Option<String>,
    pub comments: Vec<String>,
}

/// OEIS Loader (network required)
pub struct OeisLoader {
    /// Cache downloaded sequences
    cache: std::collections::HashMap<String, OeisSequence>,
}

impl Default for OeisLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl OeisLoader {
    pub fn new() -> Self {
        OeisLoader {
            cache: std::collections::HashMap::new(),
        }
    }
    
    /// Load from OEIS b-file format (index value per line)
    pub fn load_bfile(&mut self, id: &str, path: &str) -> Result<usize, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Cannot read file {}: {}", path, e))?;
        
        let mut values = Vec::new();
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(val) = parts[1].parse::<i128>() {
                    values.push(val);
                }
            }
        }
        
        if values.is_empty() {
            return Err(format!("File {} has no valid data", path));
        }
        
        let count = values.len();
        self.cache.insert(id.to_string(), OeisSequence {
            id: id.to_string(),
            name: format!("OEIS {}", id),
            values,
            formula: None,
            comments: vec![],
        });
        
        Ok(count)
    }
    
    /// Load all b-files from directory
    pub fn load_bfiles_from_dir(&mut self, dir: &str) -> Result<usize, String> {
        let mut total = 0;
        let entries = std::fs::read_dir(dir)
            .map_err(|e| format!("Cannot read directory {}: {}", dir, e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with('b') && filename.ends_with(".txt") {
                    // Extract A000045 from b000045.txt
                    let id = format!("A{}", &filename[1..filename.len()-4]);
                    if self.load_bfile(&id, path.to_str().unwrap()).is_ok() {
                        total += 1;
                    }
                }
            }
        }
        
        Ok(total)
    }
    
    /// Load OEIS data from local file
    /// File format: each line "A000045 1,1,2,3,5,8,13,21,34 Fibonacci"
    pub fn load_from_file(&mut self, path: &str) -> Result<usize, String> {
        let file = std::fs::File::open(path)
            .map_err(|e| format!("Cannot open file: {}", e))?;
        let reader = BufReader::new(file);
        let mut count = 0;
        
        for line in reader.lines() {
            let line = line.map_err(|e| format!("读取错误: {}", e))?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            let parts: Vec<&str> = line.splitn(3, ' ').collect();
            if parts.len() >= 2 {
                let id = parts[0].to_string();
                let values: Vec<i128> = parts[1].split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();
                let name = if parts.len() >= 3 { parts[2].to_string() } else { id.clone() };
                
                self.cache.insert(id.clone(), OeisSequence {
                    id,
                    name,
                    values,
                    formula: None,
                    comments: vec![],
                });
                count += 1;
            }
        }
        
        Ok(count)
    }
    
    /// 从内置数据加载常用 OEIS 序列
    pub fn load_builtin(&mut self) -> usize {
        let builtin = get_builtin_oeis();
        let count = builtin.len();
        for seq in builtin {
            self.cache.insert(seq.id.clone(), seq);
        }
        count
    }
    
    /// 获取缓存的序列
    pub fn get(&self, id: &str) -> Option<&OeisSequence> {
        self.cache.get(id)
    }
    
    /// 将所有缓存的序列添加到知识库
    pub fn add_to_knowledge_base(&self, kb: &mut KnowledgeBase) -> usize {
        let mut count = 0;
        for (id, seq) in &self.cache {
            // 检查是否已存在
            if kb.known_sequences.iter().any(|s| s.id == *id) {
                continue;
            }
            
            // Determine if formula is closed form or recurrence
            let (closed_form, recurrence) = if let Some(ref f) = seq.formula {
                // If formula contains self-reference (a(n-1), F(n-1), etc.), it's recurrence
                if f.contains("(n-") || f.contains("a(n") || f.contains("F(n") {
                    (None, Some(f.clone()))
                } else {
                    // Otherwise it's a closed form
                    (Some(f.clone()), None)
                }
            } else {
                (None, None)
            };
            
            kb.known_sequences.push(KnownSequence {
                id: id.clone(),
                name: seq.name.clone(),
                first_terms: seq.values.iter().take(15).cloned().collect(),
                pattern_class: detect_pattern_class(&seq.values),
                recurrence,
                closed_form,
                generating_function: None,
                description: format!("OEIS {}", id),
            });
            count += 1;
        }
        count
    }
    
    /// 搜索序列 (在缓存中)
    pub fn search(&self, values: &[i128]) -> Vec<&OeisSequence> {
        if values.is_empty() {
            return self.cache.values().collect();
        }
        self.cache.values()
            .filter(|seq| {
                if seq.values.len() < values.len() {
                    return false;
                }
                values.iter().zip(seq.values.iter()).all(|(a, b)| a == b)
            })
            .collect()
    }
    
    /// 获取所有缓存的序列数量
    pub fn count(&self) -> usize {
        self.cache.len()
    }
    
    /// 获取所有序列 ID
    pub fn ids(&self) -> Vec<&String> {
        self.cache.keys().collect()
    }
}

/// 检测序列的模式类 (使用 saturating 运算避免溢出)
fn detect_pattern_class(values: &[i128]) -> Option<String> {
    if values.len() < 3 {
        return None;
    }
    
    // 检查常数
    if values.windows(2).all(|w| w[0] == w[1]) {
        return Some("constant".into());
    }
    
    // 检查线性 (一阶差分常数) - 使用 saturating_sub 避免溢出
    let diff1: Vec<i128> = values.windows(2)
        .map(|w| w[1].saturating_sub(w[0]))
        .collect();
    if diff1.windows(2).all(|w| w[0] == w[1]) {
        return Some("linear".into());
    }
    
    // 检查二次 (二阶差分常数)
    if diff1.len() >= 2 {
        let diff2: Vec<i128> = diff1.windows(2)
            .map(|w| w[1].saturating_sub(w[0]))
            .collect();
        if diff2.windows(2).all(|w| w[0] == w[1]) {
            return Some("quadratic".into());
        }
    }
    
    // 检查指数 (比值常数) - 避免除以零和大数溢出
    let valid_for_ratio = values.iter().all(|&v| v != 0 && v.abs() < i128::MAX / 1000);
    if valid_for_ratio {
        let ratios: Vec<f64> = values.windows(2)
            .filter_map(|w| {
                if w[0] != 0 {
                    Some(w[1] as f64 / w[0] as f64)
                } else {
                    None
                }
            })
            .collect();
        if ratios.len() >= 2 {
            let first = ratios[0];
            if first.is_finite() && ratios.iter().all(|&r| r.is_finite() && (r - first).abs() < 0.001) {
                return Some("exponential".into());
            }
        }
    }
    
    None
}

/// 内置的常用 OEIS 序列 (无需网络)
pub fn get_builtin_oeis() -> Vec<OeisSequence> {
    vec![
        // 基础序列
        OeisSequence {
            id: "A000001".into(),
            name: "Number of groups of order n".into(),
            values: vec![0, 1, 1, 1, 2, 1, 2, 1, 5, 2, 2, 1, 5, 1, 2, 1, 14, 1, 5, 1, 5],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000002".into(),
            name: "Kolakoski sequence".into(),
            values: vec![1, 2, 2, 1, 1, 2, 1, 2, 2, 1, 2, 2, 1, 1, 2, 1, 1, 2, 2, 1],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000004".into(),
            name: "The zero sequence".into(),
            values: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            formula: Some("a(n) = 0".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000005".into(),
            name: "d(n) (number of divisors of n)".into(),
            values: vec![1, 2, 2, 3, 2, 4, 2, 4, 3, 4, 2, 6, 2, 4, 4, 5, 2, 6, 2, 6],
            formula: Some("d(n) = sum of 1 for each divisor".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000007".into(),
            name: "The characteristic function of 0".into(),
            values: vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            formula: Some("a(n) = [n=0]".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000009".into(),
            name: "Expansion of Product_{m>=1} (1 + x^m)".into(),
            values: vec![1, 1, 1, 2, 2, 3, 4, 5, 6, 8, 10, 12, 15, 18, 22, 27],
            formula: None,
            comments: vec!["Number of partitions into distinct parts".into()],
        },
        OeisSequence {
            id: "A000010".into(),
            name: "Euler totient function phi(n)".into(),
            values: vec![1, 1, 2, 2, 4, 2, 6, 4, 6, 4, 10, 4, 12, 6, 8, 8, 16, 6, 18, 8],
            formula: Some("phi(n) = n * Product_{p|n} (1 - 1/p)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000012".into(),
            name: "The all 1's sequence".into(),
            values: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            formula: Some("a(n) = 1".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000027".into(),
            name: "The positive integers".into(),
            values: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            formula: Some("a(n) = n".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000035".into(),
            name: "Period 2: (0, 1)".into(),
            values: vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0],
            formula: Some("a(n) = n mod 2".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000037".into(),
            name: "Numbers that are not squares".into(),
            values: vec![2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000040".into(),
            name: "The prime numbers".into(),
            values: vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71],
            formula: None,
            comments: vec!["p(n) is the n-th prime".into()],
        },
        OeisSequence {
            id: "A000041".into(),
            name: "Number of partitions of n".into(),
            values: vec![1, 1, 2, 3, 5, 7, 11, 15, 22, 30, 42, 56, 77, 101, 135, 176, 231],
            formula: None,
            comments: vec!["Hardy-Ramanujan-Rademacher formula".into()],
        },
        OeisSequence {
            id: "A000043".into(),
            name: "Mersenne exponents".into(),
            values: vec![2, 3, 5, 7, 13, 17, 19, 31, 61, 89, 107, 127, 521, 607],
            formula: None,
            comments: vec!["p such that 2^p - 1 is prime".into()],
        },
        OeisSequence {
            id: "A000045".into(),
            name: "Fibonacci numbers".into(),
            values: vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987],
            formula: Some("F(n) = F(n-1) + F(n-2)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000058".into(),
            name: "Sylvester's sequence".into(),
            values: vec![2, 3, 7, 43, 1807, 3263443],
            formula: Some("a(n) = a(n-1)^2 - a(n-1) + 1".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000069".into(),
            name: "Odious numbers (odd number of 1's in binary)".into(),
            values: vec![1, 2, 4, 7, 8, 11, 13, 14, 16, 19, 21, 22, 25, 26, 28, 31],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000079".into(),
            name: "Powers of 2".into(),
            values: vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096],
            formula: Some("a(n) = 2^n".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000081".into(),
            name: "Number of rooted trees with n nodes".into(),
            values: vec![0, 1, 1, 2, 4, 9, 20, 48, 115, 286, 719, 1842, 4766, 12486],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000085".into(),
            name: "Number of self-inverse permutations on n elements".into(),
            values: vec![1, 1, 2, 4, 10, 26, 76, 232, 764, 2620, 9496, 35696, 140152],
            formula: Some("a(n) = a(n-1) + (n-1)*a(n-2)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000088".into(),
            name: "Number of graphs on n unlabeled nodes".into(),
            values: vec![1, 1, 2, 4, 11, 34, 156, 1044, 12346, 274668],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000105".into(),
            name: "Number of free polyominoes with n cells".into(),
            values: vec![1, 1, 1, 2, 5, 12, 35, 108, 369, 1285, 4655, 17073, 63600],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000108".into(),
            name: "Catalan numbers".into(),
            values: vec![1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796, 58786, 208012],
            formula: Some("C(n) = C(2n,n)/(n+1)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000110".into(),
            name: "Bell numbers".into(),
            values: vec![1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147, 115975, 678570],
            formula: Some("B(n+1) = sum_{k=0}^n C(n,k)*B(k)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000111".into(),
            name: "Euler zigzag numbers".into(),
            values: vec![1, 1, 1, 2, 5, 16, 61, 272, 1385, 7936, 50521, 353792],
            formula: None,
            comments: vec!["Alternating permutations".into()],
        },
        OeisSequence {
            id: "A000120".into(),
            name: "Binary weight of n".into(),
            values: vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3],
            formula: Some("Number of 1's in binary expansion".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000124".into(),
            name: "Central polygonal numbers (lazy caterer)".into(),
            values: vec![1, 2, 4, 7, 11, 16, 22, 29, 37, 46, 56, 67, 79, 92, 106, 121],
            formula: Some("a(n) = n*(n+1)/2 + 1".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000129".into(),
            name: "Pell numbers".into(),
            values: vec![0, 1, 2, 5, 12, 29, 70, 169, 408, 985, 2378, 5741, 13860],
            formula: Some("a(n) = 2*a(n-1) + a(n-2)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000142".into(),
            name: "Factorial numbers".into(),
            values: vec![1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800],
            formula: Some("n!".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000166".into(),
            name: "Subfactorial (derangements)".into(),
            values: vec![1, 0, 1, 2, 9, 44, 265, 1854, 14833, 133496, 1334961],
            formula: Some("D(n) = (n-1)*(D(n-1) + D(n-2))".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000169".into(),
            name: "Number of labeled rooted trees with n nodes".into(),
            values: vec![1, 1, 3, 16, 125, 1296, 16807, 262144, 4782969],
            formula: Some("n^(n-1)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000203".into(),
            name: "sigma(n) - sum of divisors".into(),
            values: vec![1, 3, 4, 7, 6, 12, 8, 15, 13, 18, 12, 28, 14, 24, 24, 31],
            formula: Some("sigma(n) = sum_{d|n} d".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000204".into(),
            name: "Lucas numbers".into(),
            values: vec![1, 3, 4, 7, 11, 18, 29, 47, 76, 123, 199, 322, 521, 843],
            formula: Some("L(n) = L(n-1) + L(n-2)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000217".into(),
            name: "Triangular numbers".into(),
            values: vec![0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, 66, 78, 91, 105, 120],
            formula: Some("T(n) = n*(n+1)/2".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000225".into(),
            name: "Mersenne numbers 2^n - 1".into(),
            values: vec![0, 1, 3, 7, 15, 31, 63, 127, 255, 511, 1023, 2047, 4095],
            formula: Some("a(n) = 2^n - 1".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000244".into(),
            name: "Powers of 3".into(),
            values: vec![1, 3, 9, 27, 81, 243, 729, 2187, 6561, 19683, 59049],
            formula: Some("a(n) = 3^n".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000262".into(),
            name: "Number of labeled rooted forests on n nodes".into(),
            values: vec![1, 1, 3, 13, 73, 501, 4051, 37633, 394353, 4596553],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000290".into(),
            name: "Square numbers".into(),
            values: vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81, 100, 121, 144, 169, 196],
            formula: Some("a(n) = n^2".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000292".into(),
            name: "Tetrahedral (pyramidal) numbers".into(),
            values: vec![0, 1, 4, 10, 20, 35, 56, 84, 120, 165, 220, 286, 364, 455],
            formula: Some("a(n) = n*(n+1)*(n+2)/6".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000302".into(),
            name: "Powers of 4".into(),
            values: vec![1, 4, 16, 64, 256, 1024, 4096, 16384, 65536, 262144],
            formula: Some("a(n) = 4^n".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000312".into(),
            name: "a(n) = n^n".into(),
            values: vec![1, 1, 4, 27, 256, 3125, 46656, 823543, 16777216],
            formula: Some("a(n) = n^n".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000326".into(),
            name: "Pentagonal numbers".into(),
            values: vec![0, 1, 5, 12, 22, 35, 51, 70, 92, 117, 145, 176, 210, 247],
            formula: Some("a(n) = n*(3*n-1)/2".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000330".into(),
            name: "Square pyramidal numbers".into(),
            values: vec![0, 1, 5, 14, 30, 55, 91, 140, 204, 285, 385, 506, 650, 819],
            formula: Some("a(n) = n*(n+1)*(2*n+1)/6".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000364".into(),
            name: "Euler (or secant) numbers".into(),
            values: vec![1, 1, 5, 61, 1385, 50521, 2702765, 199360981],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000396".into(),
            name: "Perfect numbers".into(),
            values: vec![6, 28, 496, 8128, 33550336],
            formula: Some("2^(p-1)*(2^p - 1) for Mersenne prime 2^p - 1".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000578".into(),
            name: "Cubes".into(),
            values: vec![0, 1, 8, 27, 64, 125, 216, 343, 512, 729, 1000, 1331, 1728],
            formula: Some("a(n) = n^3".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000583".into(),
            name: "Fourth powers".into(),
            values: vec![0, 1, 16, 81, 256, 625, 1296, 2401, 4096, 6561, 10000],
            formula: Some("a(n) = n^4".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000593".into(),
            name: "Sum of odd divisors of n".into(),
            values: vec![1, 1, 4, 1, 6, 4, 8, 1, 13, 6, 12, 4, 14, 8, 24, 1, 18, 13],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000670".into(),
            name: "Fubini numbers (ordered partitions)".into(),
            values: vec![1, 1, 3, 13, 75, 541, 4683, 47293, 545835, 7087261],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000688".into(),
            name: "Number of Abelian groups of order n".into(),
            values: vec![1, 1, 1, 2, 1, 1, 1, 3, 2, 1, 1, 2, 1, 1, 1, 5, 1, 2, 1, 2],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000720".into(),
            name: "pi(n), the prime counting function".into(),
            values: vec![0, 1, 2, 2, 3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 6, 6, 7, 7, 8, 8],
            formula: None,
            comments: vec!["Number of primes <= n".into()],
        },
        OeisSequence {
            id: "A000793".into(),
            name: "Landau's function g(n)".into(),
            values: vec![1, 1, 2, 3, 4, 6, 6, 12, 15, 20, 30, 30, 60, 60, 84, 105],
            formula: None,
            comments: vec!["Largest order of permutation of n elements".into()],
        },
        OeisSequence {
            id: "A000796".into(),
            name: "Decimal expansion of Pi".into(),
            values: vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3, 2, 3, 8, 4],
            formula: None,
            comments: vec!["π = 3.14159265358979...".into()],
        },
        OeisSequence {
            id: "A000961".into(),
            name: "Prime powers (including 1)".into(),
            values: vec![1, 2, 3, 4, 5, 7, 8, 9, 11, 13, 16, 17, 19, 23, 25, 27, 29, 31],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A000984".into(),
            name: "Central binomial coefficients C(2n,n)".into(),
            values: vec![1, 2, 6, 20, 70, 252, 924, 3432, 12870, 48620, 184756],
            formula: Some("C(2n, n)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A001006".into(),
            name: "Motzkin numbers".into(),
            values: vec![1, 1, 2, 4, 9, 21, 51, 127, 323, 835, 2188, 5798, 15511],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A001045".into(),
            name: "Jacobsthal sequence".into(),
            values: vec![0, 1, 1, 3, 5, 11, 21, 43, 85, 171, 341, 683, 1365, 2731],
            formula: Some("a(n) = a(n-1) + 2*a(n-2)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A001055".into(),
            name: "Number of factorizations of n".into(),
            values: vec![1, 1, 1, 2, 1, 2, 1, 3, 2, 2, 1, 4, 1, 2, 2, 5, 1, 4, 1, 4],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A001057".into(),
            name: "Canonical enumeration of integers".into(),
            values: vec![0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8],
            formula: Some("a(n) = (-1)^n * floor((n+1)/2)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A001065".into(),
            name: "Sum of proper divisors".into(),
            values: vec![0, 1, 1, 3, 1, 6, 1, 7, 4, 8, 1, 16, 1, 10, 9, 15, 1, 21],
            formula: Some("s(n) = sigma(n) - n".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A001109".into(),
            name: "a(n)^2 is a triangular number".into(),
            values: vec![0, 1, 6, 35, 204, 1189, 6930, 40391, 235416, 1372105],
            formula: Some("a(n) = 6*a(n-1) - a(n-2)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A001190".into(),
            name: "Wedderburn-Etherington numbers".into(),
            values: vec![0, 1, 1, 1, 2, 3, 6, 11, 23, 46, 98, 207, 451, 983, 2179],
            formula: None,
            comments: vec!["Unlabeled binary rooted trees".into()],
        },
        OeisSequence {
            id: "A001221".into(),
            name: "omega(n), number of distinct prime factors".into(),
            values: vec![0, 1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 2, 1, 2, 2, 1, 1, 2, 1, 2],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A001222".into(),
            name: "Omega(n), number of prime factors with multiplicity".into(),
            values: vec![0, 1, 1, 2, 1, 2, 1, 3, 2, 2, 1, 3, 1, 2, 2, 4, 1, 3, 1, 3],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A001227".into(),
            name: "Number of odd divisors of n".into(),
            values: vec![1, 1, 2, 1, 2, 2, 2, 1, 3, 2, 2, 2, 2, 2, 4, 1, 2, 3, 2, 2],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A001333".into(),
            name: "Numerators of continued fraction convergents to sqrt(2)".into(),
            values: vec![1, 1, 3, 7, 17, 41, 99, 239, 577, 1393, 3363, 8119, 19601],
            formula: Some("a(n) = 2*a(n-1) + a(n-2)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A001358".into(),
            name: "Semiprimes".into(),
            values: vec![4, 6, 9, 10, 14, 15, 21, 22, 25, 26, 33, 34, 35, 38, 39],
            formula: None,
            comments: vec!["Products of two primes".into()],
        },
        OeisSequence {
            id: "A001462".into(),
            name: "Golomb's sequence".into(),
            values: vec![1, 2, 2, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8],
            formula: Some("a(a(n)) = n".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A001477".into(),
            name: "The nonnegative integers".into(),
            values: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
            formula: Some("a(n) = n".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A001511".into(),
            name: "The ruler function".into(),
            values: vec![1, 2, 1, 3, 1, 2, 1, 4, 1, 2, 1, 3, 1, 2, 1, 5, 1, 2, 1, 3],
            formula: Some("a(n) = 2-adic valuation of 2n".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A001519".into(),
            name: "a(n) = F(2n-1), Fibonacci(odd index)".into(),
            values: vec![1, 1, 2, 5, 13, 34, 89, 233, 610, 1597, 4181, 10946, 28657],
            formula: Some("a(n) = 3*a(n-1) - a(n-2)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A001700".into(),
            name: "a(n) = C(2n+1, n+1)".into(),
            values: vec![1, 3, 10, 35, 126, 462, 1716, 6435, 24310, 92378, 352716],
            formula: Some("C(2n+1, n+1) = (2n+1)!/((n+1)!*n!)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A002024".into(),
            name: "n appears n times".into(),
            values: vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 6],
            formula: Some("a(n) = floor((1+sqrt(8n-7))/2)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A002110".into(),
            name: "Primorial numbers".into(),
            values: vec![1, 2, 6, 30, 210, 2310, 30030, 510510, 9699690],
            formula: Some("Product of first n primes".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A002113".into(),
            name: "Palindromic numbers".into(),
            values: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33, 44, 55, 66, 77, 88, 99, 101],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A002275".into(),
            name: "Repunits R_n = (10^n - 1)/9".into(),
            values: vec![0, 1, 11, 111, 1111, 11111, 111111, 1111111, 11111111],
            formula: Some("(10^n - 1)/9".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A002378".into(),
            name: "Pronic numbers n*(n+1)".into(),
            values: vec![0, 2, 6, 12, 20, 30, 42, 56, 72, 90, 110, 132, 156, 182],
            formula: Some("a(n) = n*(n+1)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A002620".into(),
            name: "Quarter-squares floor(n^2/4)".into(),
            values: vec![0, 0, 1, 2, 4, 6, 9, 12, 16, 20, 25, 30, 36, 42, 49, 56],
            formula: Some("floor(n^2/4)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A003418".into(),
            name: "LCM of {1, 2, ..., n}".into(),
            values: vec![1, 1, 2, 6, 12, 60, 60, 420, 840, 2520, 2520, 27720, 27720],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A005100".into(),
            name: "Deficient numbers".into(),
            values: vec![1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 19, 21, 22, 23],
            formula: None,
            comments: vec!["sigma(n) < 2n".into()],
        },
        OeisSequence {
            id: "A005101".into(),
            name: "Abundant numbers".into(),
            values: vec![12, 18, 20, 24, 30, 36, 40, 42, 48, 54, 56, 60, 66, 70, 72],
            formula: None,
            comments: vec!["sigma(n) > 2n".into()],
        },
        OeisSequence {
            id: "A005117".into(),
            name: "Squarefree numbers".into(),
            values: vec![1, 2, 3, 5, 6, 7, 10, 11, 13, 14, 15, 17, 19, 21, 22, 23, 26],
            formula: None,
            comments: vec!["Not divisible by any square > 1".into()],
        },
        OeisSequence {
            id: "A005132".into(),
            name: "Recaman's sequence".into(),
            values: vec![0, 1, 3, 6, 2, 7, 13, 20, 12, 21, 11, 22, 10, 23, 9, 24, 8, 25, 43],
            formula: Some("a(n) = a(n-1) - n if positive and new, else a(n-1) + n".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A005150".into(),
            name: "Look-and-say sequence".into(),
            values: vec![1, 11, 21, 1211, 111221, 312211, 13112221],
            formula: None,
            comments: vec!["Conway's constant".into()],
        },
        OeisSequence {
            id: "A005408".into(),
            name: "Odd numbers".into(),
            values: vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31],
            formula: Some("a(n) = 2n + 1".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A005843".into(),
            name: "Even numbers".into(),
            values: vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30],
            formula: Some("a(n) = 2n".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A006530".into(),
            name: "gpf(n): greatest prime factor of n".into(),
            values: vec![1, 2, 3, 2, 5, 3, 7, 2, 3, 5, 11, 3, 13, 7, 5, 2, 17, 3, 19, 5],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A006882".into(),
            name: "Double factorials n!!".into(),
            values: vec![1, 1, 2, 3, 8, 15, 48, 105, 384, 945, 3840, 10395, 46080],
            formula: Some("n!! = n*(n-2)*(n-4)*...".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A007318".into(),
            name: "Pascal's triangle read by rows".into(),
            values: vec![1, 1, 1, 1, 2, 1, 1, 3, 3, 1, 1, 4, 6, 4, 1, 1, 5, 10, 10, 5, 1],
            formula: Some("C(n, k)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A007504".into(),
            name: "Sum of first n primes".into(),
            values: vec![0, 2, 5, 10, 17, 28, 41, 58, 77, 100, 129, 160, 197, 238, 281],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A007947".into(),
            name: "Radical of n (squarefree kernel)".into(),
            values: vec![1, 2, 3, 2, 5, 6, 7, 2, 3, 10, 11, 6, 13, 14, 15, 2, 17, 6],
            formula: Some("Product of distinct prime factors".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A008292".into(),
            name: "Triangle of Eulerian numbers".into(),
            values: vec![1, 1, 1, 1, 4, 1, 1, 11, 11, 1, 1, 26, 66, 26, 1],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A014137".into(),
            name: "Partial sums of Catalan numbers".into(),
            values: vec![1, 2, 4, 9, 23, 65, 197, 626, 2056, 6918, 23714, 82500],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A018252".into(),
            name: "Composite numbers".into(),
            values: vec![4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27],
            formula: None,
            comments: vec!["Non-prime numbers > 1".into()],
        },
        OeisSequence {
            id: "A020639".into(),
            name: "lpf(n): least prime factor of n".into(),
            values: vec![1, 2, 3, 2, 5, 2, 7, 2, 3, 2, 11, 2, 13, 2, 3, 2, 17, 2, 19, 2],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A027641".into(),
            name: "Numerators of Bernoulli numbers".into(),
            values: vec![1, -1, 1, 0, -1, 0, 1, 0, -1, 0, 5, 0, -691, 0, 7, 0, -3617],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A027642".into(),
            name: "Denominators of Bernoulli numbers".into(),
            values: vec![1, 2, 6, 1, 30, 1, 42, 1, 30, 1, 66, 1, 2730, 1, 6, 1, 510],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A051959".into(),
            name: "Hofstadter Q-sequence".into(),
            values: vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 6, 6, 8, 8, 8, 10, 9, 10, 11, 11, 12],
            formula: Some("Q(n) = Q(n - Q(n-1)) + Q(n - Q(n-2))".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A055512".into(),
            name: "Lattice points in circle of radius n".into(),
            values: vec![1, 5, 13, 29, 49, 81, 113, 149, 197, 253, 317, 377, 441],
            formula: None,
            comments: vec![],
        },
        OeisSequence {
            id: "A070939".into(),
            name: "Length of binary representation of n".into(),
            values: vec![1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5],
            formula: Some("floor(log_2(n)) + 1".into()),
            comments: vec![],
        },
        // === ADDITIONAL FIGURATE NUMBERS ===
        OeisSequence {
            id: "A000384".into(),
            name: "Hexagonal numbers".into(),
            values: vec![0, 1, 6, 15, 28, 45, 66, 91, 120, 153, 190, 231, 276, 325],
            formula: Some("a(n) = n*(2*n-1)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000566".into(),
            name: "Heptagonal numbers".into(),
            values: vec![0, 1, 7, 18, 34, 55, 81, 112, 148, 189, 235, 286, 342, 403],
            formula: Some("a(n) = n*(5*n-3)/2".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000567".into(),
            name: "Octagonal numbers".into(),
            values: vec![0, 1, 8, 21, 40, 65, 96, 133, 176, 225, 280, 341, 408, 481],
            formula: Some("a(n) = n*(3*n-2)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A002061".into(),
            name: "Central polygonal numbers: n^2 - n + 1".into(),
            values: vec![1, 1, 3, 7, 13, 21, 31, 43, 57, 73, 91, 111, 133, 157, 183],
            formula: Some("a(n) = n^2 - n + 1".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A005563".into(),
            name: "n*(n+2) = (n+1)^2 - 1".into(),
            values: vec![0, 3, 8, 15, 24, 35, 48, 63, 80, 99, 120, 143, 168, 195],
            formula: Some("a(n) = n*(n+2)".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A002522".into(),
            name: "n^2 + 1".into(),
            values: vec![1, 2, 5, 10, 17, 26, 37, 50, 65, 82, 101, 122, 145, 170],
            formula: Some("a(n) = n^2 + 1".into()),
            comments: vec![],
        },
        OeisSequence {
            id: "A000096".into(),
            name: "n*(n+3)/2".into(),
            values: vec![0, 2, 5, 9, 14, 20, 27, 35, 44, 54, 65, 77, 90, 104, 119],
            formula: Some("a(n) = n*(n+3)/2".into()),
            comments: vec![],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_builtin_oeis() {
        let sequences = get_builtin_oeis();
        assert!(sequences.len() >= 90);
        
        // 验证 Fibonacci
        let fib = sequences.iter().find(|s| s.id == "A000045").unwrap();
        assert_eq!(fib.values[..8], [0, 1, 1, 2, 3, 5, 8, 13]);
        
        // 验证 Primes
        let primes = sequences.iter().find(|s| s.id == "A000040").unwrap();
        assert_eq!(primes.values[..5], [2, 3, 5, 7, 11]);
    }
    
    #[test]
    fn test_pattern_detection() {
        // 线性
        assert_eq!(detect_pattern_class(&[1, 2, 3, 4, 5]), Some("linear".into()));
        
        // 二次
        assert_eq!(detect_pattern_class(&[1, 4, 9, 16, 25]), Some("quadratic".into()));
        
        // 指数
        assert_eq!(detect_pattern_class(&[1, 2, 4, 8, 16]), Some("exponential".into()));
    }
    
    #[test]
    fn test_oeis_loader() {
        let mut loader = OeisLoader::new();
        let count = loader.load_builtin();
        assert!(count >= 90);
        
        // 搜索测试
        let results = loader.search(&[0, 1, 1, 2, 3, 5]);
        assert!(!results.is_empty());
        assert!(results.iter().any(|s| s.id == "A000045"));
    }
}
